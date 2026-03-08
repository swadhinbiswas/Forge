use pyo3::prelude::*;
use serde::Deserialize;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder, EventLoopProxy},
    window::{Fullscreen, WindowBuilder, WindowId},
};
use wry::WebViewBuilder;

#[cfg(target_os = "linux")]
use gtk::prelude::*;
#[cfg(target_os = "linux")]
use std::rc::Rc;
#[cfg(target_os = "linux")]
use tao::platform::unix::WindowExtUnix;
#[cfg(target_os = "linux")]
use wry::WebViewBuilderExtUnix;

// ─── User Events for cross-thread communication ───
enum UserEvent {
    /// Evaluate JavaScript in the WebView
    Eval(String),
    /// Navigate to a URL in the WebView
    LoadUrl(String),
    /// Reload the current page
    Reload,
    /// Navigate backward in history
    GoBack,
    /// Navigate forward in history
    GoForward,
    /// Open webview devtools
    OpenDevtools,
    /// Close webview devtools
    CloseDevtools,
    /// Set window title from Python thread
    SetTitle(String),
    /// Resize window from Python thread
    Resize(f64, f64),
    /// Move the native window
    SetPosition(f64, f64),
    /// Toggle fullscreen from Python thread
    SetFullscreen(bool),
    /// Show or hide the native window
    SetVisible(bool),
    /// Minimize or restore the native window
    SetMinimized(bool),
    /// Maximize or restore the native window
    SetMaximized(bool),
    /// Toggle always-on-top
    SetAlwaysOnTop(bool),
    /// Replace the native application menu model
    SetMenu(String),
    /// Focus the native window
    Focus,
    /// Create an additional native window
    CreateWindow(WindowDescriptor),
    /// Close a managed native window by label
    CloseLabel(String),
    /// Request the event loop to exit
    Close,
    /// Request monitors info
    GetMonitors(crossbeam_channel::Sender<String>),
    /// Request primary monitor info
    GetPrimaryMonitor(crossbeam_channel::Sender<String>),
    /// Request cursor position
    GetCursorPosition(crossbeam_channel::Sender<String>),
    /// Register global shortcut
    RegisterShortcut(String, crossbeam_channel::Sender<bool>),
    /// Unregister global shortcut
    UnregisterShortcut(String, crossbeam_channel::Sender<bool>),
    /// Unregister all global shortcuts
    UnregisterAllShortcuts(crossbeam_channel::Sender<bool>),
    Print(String),
    SetProgressBar(f64),
    RequestUserAttention(Option<tao::window::UserAttentionType>),
    PowerGetBatteryInfo(crossbeam_channel::Sender<String>),
}

fn default_window_url() -> String {
    "forge://app/index.html".to_string()
}

fn default_window_visible() -> bool {
    true
}

fn default_window_focus() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
struct WindowDescriptor {
    label: String,
    title: String,
    #[serde(default = "default_window_url")]
    url: String,
    #[serde(default = "default_window_visible")]
    visible: bool,
    #[serde(default = "default_window_focus")]
    focus: bool,
    #[serde(default)]
    fullscreen: bool,
    #[serde(default = "default_menu_enabled")]
    resizable: bool,
    #[serde(default = "default_menu_enabled")]
    decorations: bool,
    #[serde(default)]
    transparent: bool,
    #[serde(default)]
    always_on_top: bool,
    #[serde(default = "default_window_min_width")]
    min_width: f64,
    #[serde(default = "default_window_min_height")]
    min_height: f64,
    x: Option<f64>,
    width: f64,
    height: f64,
}

fn default_window_min_width() -> f64 {
    320.0
}

fn default_window_min_height() -> f64 {
    240.0
}

fn default_menu_item_type() -> String {
    "item".to_string()
}

fn default_menu_enabled() -> bool {
    true
}

#[derive(Debug, Clone, Deserialize)]
struct NativeMenuItem {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    label: Option<String>,
    #[serde(default = "default_menu_item_type")]
    #[serde(rename = "type")]
    item_type: String,
    #[serde(default = "default_menu_enabled")]
    enabled: bool,
    #[serde(default)]
    checked: bool,
    #[serde(default)]
    checkable: bool,
    #[serde(default)]
    role: Option<String>,
    #[serde(default)]
    submenu: Vec<NativeMenuItem>,
}

#[cfg(target_os = "linux")]
type MenuEmitter = Rc<dyn Fn(String, Option<String>, Option<String>, Option<bool>)>;

#[cfg(target_os = "linux")]
fn clear_linux_menu(menu_bar: &gtk::MenuBar) {
    for child in menu_bar.children() {
        menu_bar.remove(&child);
    }
}

#[cfg(target_os = "linux")]
fn build_linux_menu_widget(item: &NativeMenuItem, emit: MenuEmitter) -> gtk::MenuItem {
    if item.item_type == "separator" {
        return gtk::SeparatorMenuItem::new().upcast::<gtk::MenuItem>();
    }

    if item.checkable {
        let menu_item = match &item.label {
            Some(label) => gtk::CheckMenuItem::with_label(label),
            None => gtk::CheckMenuItem::new(),
        };
        menu_item.set_sensitive(item.enabled);
        menu_item.set_active(item.checked);
        if let Some(item_id) = item.id.clone() {
            let label = item.label.clone();
            let role = item.role.clone();
            let emit_checked = emit.clone();
            menu_item.connect_toggled(move |entry| {
                emit_checked(
                    item_id.clone(),
                    label.clone(),
                    role.clone(),
                    Some(entry.is_active()),
                );
            });
        }
        return menu_item.upcast::<gtk::MenuItem>();
    }

    let menu_item = match &item.label {
        Some(label) => gtk::MenuItem::with_label(label),
        None => gtk::MenuItem::new(),
    };
    menu_item.set_sensitive(item.enabled);

    if !item.submenu.is_empty() {
        let submenu = gtk::Menu::new();
        for child in &item.submenu {
            let child_widget = build_linux_menu_widget(child, emit.clone());
            submenu.append(&child_widget);
        }
        menu_item.set_submenu(Some(&submenu));
    } else if let Some(item_id) = item.id.clone() {
        let label = item.label.clone();
        let role = item.role.clone();
        let emit_click = emit.clone();
        menu_item.connect_activate(move |_| {
            emit_click(item_id.clone(), label.clone(), role.clone(), None);
        });
    }

    menu_item
}

#[cfg(target_os = "linux")]
fn apply_linux_menu(menu_bar: &gtk::MenuBar, menu_json: &str, emit: MenuEmitter) -> Result<usize, String> {
    let items: Vec<NativeMenuItem> =
        serde_json::from_str(menu_json).map_err(|e| format!("Invalid menu payload: {}", e))?;

    clear_linux_menu(menu_bar);

    if items.is_empty() {
        menu_bar.hide();
        return Ok(0);
    }

    for item in &items {
        let widget = build_linux_menu_widget(item, emit.clone());
        menu_bar.append(&widget);
    }

    menu_bar.show_all();
    Ok(items.len())
}

// ─── MIME type lookup (extended for web app assets) ───
fn mime_from_path(path: &str) -> &'static str {
    if path.ends_with(".html") || path.ends_with(".htm") {
        "text/html"
    } else if path.ends_with(".js") || path.ends_with(".mjs") {
        "application/javascript"
    } else if path.ends_with(".css") {
        "text/css"
    } else if path.ends_with(".json") {
        "application/json"
    } else if path.ends_with(".svg") {
        "image/svg+xml"
    } else if path.ends_with(".png") {
        "image/png"
    } else if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        "image/jpeg"
    } else if path.ends_with(".gif") {
        "image/gif"
    } else if path.ends_with(".webp") {
        "image/webp"
    } else if path.ends_with(".ico") {
        "image/x-icon"
    } else if path.ends_with(".woff") {
        "font/woff"
    } else if path.ends_with(".woff2") {
        "font/woff2"
    } else if path.ends_with(".ttf") {
        "font/ttf"
    } else if path.ends_with(".otf") {
        "font/otf"
    } else if path.ends_with(".wasm") {
        "application/wasm"
    } else if path.ends_with(".map") {
        "application/json"
    } else if path.ends_with(".xml") {
        "application/xml"
    } else if path.ends_with(".txt") {
        "text/plain"
    } else if path.ends_with(".mp4") {
        "video/mp4"
    } else if path.ends_with(".webm") {
        "video/webm"
    } else if path.ends_with(".mp3") {
        "audio/mpeg"
    } else if path.ends_with(".ogg") {
        "audio/ogg"
    } else {
        "application/octet-stream"
    }
}

struct RuntimeWindow {
    label: String,
    url: String,
    window: tao::window::Window,
    webview: wry::WebView,
    #[cfg(target_os = "linux")]
    menu_bar: gtk::MenuBar,
}

fn emit_window_event(
    callback: &Option<Py<PyAny>>,
    event_name: &str,
    label: &str,
    payload: serde_json::Value,
) {
    if let Some(cb) = callback {
        let payload_json = match payload {
            serde_json::Value::Object(mut object) => {
                object.insert("label".to_string(), serde_json::Value::String(label.to_string()));
                serde_json::Value::Object(object).to_string()
            }
            serde_json::Value::Null => serde_json::json!({ "label": label }).to_string(),
            other => serde_json::json!({ "label": label, "value": other }).to_string(),
        };
        Python::attach(|py| {
            if let Err(error) = cb.call1(py, (event_name, payload_json)) {
                eprintln!("[forge-core] window event callback error: {}", error);
            }
        });
    }
}

fn clone_py_callback(callback: &Option<Py<PyAny>>) -> Option<Py<PyAny>> {
    Python::attach(|py| callback.as_ref().map(|cb| cb.clone_ref(py)))
}

fn build_webview_for_window(
    window: &tao::window::Window,
    label: &str,
    url: &str,
    root_path: PathBuf,
    ipc_callback: Option<Py<PyAny>>,
    window_event_callback: Option<Py<PyAny>>,
    proxy_for_ipc: Py<WindowProxy>,
) -> Result<wry::WebView, String> {
    let mut webview_builder = WebViewBuilder::new();
    webview_builder = webview_builder.with_asynchronous_custom_protocol("forge".into(), move |_webview_id, request, responder| {
        let path = request.uri().path().to_string();
        let mut file_path = root_path.clone();

        std::thread::spawn(move || {
            let relative_path = if path == "/" { "index.html" } else { &path[1..] };
            file_path.push(relative_path);

            if let Ok(content) = fs::read(&file_path) {
                let mime = mime_from_path(&path);
                let response = wry::http::Response::builder()
                    .header("Content-Type", mime)
                    .header("Access-Control-Allow-Origin", "*")
                    .body(Cow::Owned(content))
                    .unwrap();
                responder.respond(response);
            } else {
                let response = wry::http::Response::builder()
                    .status(404)
                    .header("Access-Control-Allow-Origin", "*")
                    .body(Cow::Borrowed("File not found".as_bytes()))
                    .unwrap();
                responder.respond(response);
            }
        });
    });

    webview_builder = webview_builder.with_url(url);
    webview_builder = webview_builder.with_devtools(true);

    if let Some(cb) = clone_py_callback(&window_event_callback) {
        let navigation_label = label.to_string();
        webview_builder = webview_builder.with_navigation_handler(move |target_url| {
            let navigation_callback = Python::attach(|py| Some(cb.clone_ref(py)));
            emit_window_event(
                &navigation_callback,
                "navigated",
                &navigation_label,
                serde_json::json!({ "url": target_url }),
            );
            true
        });
    }

    if let Some(cb) = ipc_callback {
        webview_builder = webview_builder.with_ipc_handler(move |req| {
            let msg = req.into_body();
            Python::attach(|py| {
                if let Err(error) = cb.call1(py, (msg, proxy_for_ipc.clone_ref(py))) {
                    eprintln!("[forge-core] IPC callback error: {}", error);
                }
            });
        });
    }

    #[cfg(target_os = "linux")]
    {
        let vbox = window.default_vbox().expect(
            "tao window should have a default vbox; \
             did you disable it with with_default_vbox(false)?",
        );
        webview_builder.build_gtk(vbox).map_err(|error| error.to_string())
    }
    #[cfg(not(target_os = "linux"))]
    {
        webview_builder.build(window).map_err(|error| error.to_string())
    }
}

/// WindowProxy - A lightweight, thread-safe handle for sending commands
/// to the native window's event loop.
///
/// This is separated from NativeWindow to avoid PyO3 borrow conflicts:
/// NativeWindow.run() holds a mutable borrow for the event loop, so Python
/// code inside the IPC callback cannot call methods on NativeWindow directly.
/// WindowProxy holds only a clone of the EventLoopProxy, which is Send+Sync,
/// so it can safely be used from the IPC callback without touching NativeWindow.
#[pyclass(from_py_object)]
#[derive(Clone)]
struct WindowProxy {
    proxy: EventLoopProxy<UserEvent>,
}

#[pymethods]
impl WindowProxy {
    /// Send JavaScript to the WebView for evaluation (thread-safe, non-blocking).
    fn evaluate_script(&self, script: String) -> PyResult<()> {
        self.proxy.send_event(UserEvent::Eval(script)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send script to event loop")
        })
    }

    /// Navigate the live webview to a URL.
    fn load_url(&self, url: String) -> PyResult<()> {
        self.proxy.send_event(UserEvent::LoadUrl(url)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send navigation event")
        })
    }

    /// Reload the active webview page.
    fn reload(&self) -> PyResult<()> {
        self.proxy.send_event(UserEvent::Reload).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send reload event")
        })
    }

    /// Navigate backward in browser history.
    fn go_back(&self) -> PyResult<()> {
        self.proxy.send_event(UserEvent::GoBack).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send go-back event")
        })
    }

    /// Navigate forward in browser history.
    fn go_forward(&self) -> PyResult<()> {
        self.proxy.send_event(UserEvent::GoForward).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send go-forward event")
        })
    }

    /// Open native webview devtools.
    fn open_devtools(&self) -> PyResult<()> {
        self.proxy.send_event(UserEvent::OpenDevtools).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send open-devtools event")
        })
    }

    /// Close native webview devtools.
    fn close_devtools(&self) -> PyResult<()> {
        self.proxy.send_event(UserEvent::CloseDevtools).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send close-devtools event")
        })
    }

    /// Set the window title at runtime (thread-safe).
    fn set_title(&self, title: String) -> PyResult<()> {
        self.proxy
            .send_event(UserEvent::SetTitle(title))
            .map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send title update")
            })
    }

    /// Resize the window at runtime (thread-safe).
    fn set_size(&self, width: f64, height: f64) -> PyResult<()> {
        self.proxy
            .send_event(UserEvent::Resize(width, height))
            .map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send resize event")
            })
    }

    /// Move the window at runtime.
    fn set_position(&self, x: f64, y: f64) -> PyResult<()> {
        self.proxy
            .send_event(UserEvent::SetPosition(x, y))
            .map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to send position event",
                )
            })
    }

    /// Toggle fullscreen mode at runtime.
    fn set_fullscreen(&self, enabled: bool) -> PyResult<()> {
        self.proxy
            .send_event(UserEvent::SetFullscreen(enabled))
            .map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to send fullscreen event",
                )
            })
    }

    /// Show or hide the native window.
    fn set_visible(&self, visible: bool) -> PyResult<()> {
        self.proxy
            .send_event(UserEvent::SetVisible(visible))
            .map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to send visibility event",
                )
            })
    }

    /// Focus the native window.
    fn focus(&self) -> PyResult<()> {
        self.proxy.send_event(UserEvent::Focus).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send focus event")
        })
    }

    /// Minimize or restore the native window.
    fn set_minimized(&self, minimized: bool) -> PyResult<()> {
        self.proxy
            .send_event(UserEvent::SetMinimized(minimized))
            .map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to send minimized event",
                )
            })
    }

    /// Maximize or restore the native window.
    fn set_maximized(&self, maximized: bool) -> PyResult<()> {
        self.proxy
            .send_event(UserEvent::SetMaximized(maximized))
            .map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to send maximized event",
                )
            })
    }

    /// Toggle always-on-top at runtime.
    fn set_always_on_top(&self, always_on_top: bool) -> PyResult<()> {
        self.proxy
            .send_event(UserEvent::SetAlwaysOnTop(always_on_top))
            .map_err(|_| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Failed to send always-on-top event",
                )
            })
    }

    /// Replace the native application menu model.
    fn set_menu(&self, menu_json: String) -> PyResult<()> {
        self.proxy.send_event(UserEvent::SetMenu(menu_json)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send menu update")
        })
    }

    /// Create a managed native child window.
    fn create_window(&self, descriptor_json: String) -> PyResult<()> {
        let descriptor: WindowDescriptor = serde_json::from_str(&descriptor_json).map_err(|error| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid window descriptor: {}", error))
        })?;
        self.proxy.send_event(UserEvent::CreateWindow(descriptor)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send create-window event")
        })
    }

    /// Close a managed native child window by label.
    fn close_window_label(&self, label: String) -> PyResult<()> {
        self.proxy.send_event(UserEvent::CloseLabel(label)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send close-window event")
        })
    }

    /// Close the native window.
    fn close(&self) -> PyResult<()> {
        self.proxy.send_event(UserEvent::Close).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send close event")
        })
    }

    /// Get all monitors
    fn get_monitors(&self) -> PyResult<String> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.proxy.send_event(UserEvent::GetMonitors(tx)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to request monitors")
        })?;
        rx.recv().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to receive monitors")
        })
    }

    /// Get primary monitor
    fn get_primary_monitor(&self) -> PyResult<String> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.proxy.send_event(UserEvent::GetPrimaryMonitor(tx)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to request primary monitor")
        })?;
        rx.recv().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to receive primary monitor")
        })
    }

    /// Get global cursor position
    fn get_cursor_position(&self) -> PyResult<String> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.proxy.send_event(UserEvent::GetCursorPosition(tx)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to request cursor position")
        })?;
        rx.recv().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to receive cursor position")
        })
    }

    /// Register a global shortcut
    fn register_shortcut(&self, accelerator: String) -> PyResult<bool> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.proxy.send_event(UserEvent::RegisterShortcut(accelerator, tx)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send register shortcut event")
        })?;
        rx.recv().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to receive shortcut registration status")
        })
    }

    /// Unregister a global shortcut
    fn unregister_shortcut(&self, accelerator: String) -> PyResult<bool> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.proxy.send_event(UserEvent::UnregisterShortcut(accelerator, tx)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send unregister shortcut event")
        })?;
        rx.recv().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to receive shortcut unregistration status")
        })
    }

    /// Unregister all global shortcuts
    fn unregister_all_shortcuts(&self) -> PyResult<bool> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.proxy.send_event(UserEvent::UnregisterAllShortcuts(tx)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send unregister all shortcuts event")
        })?;
        rx.recv().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to receive shortcut unregistration status")
        })
    }

    fn print(&self, label: String) -> PyResult<()> {
        self.proxy.send_event(UserEvent::Print(label)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send print event")
        })?;
        Ok(())
    }

    fn os_set_progress_bar(&self, progress: f64) -> PyResult<bool> {
        self.proxy.send_event(UserEvent::SetProgressBar(progress)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send progress bar event")
        })?;
        Ok(true)
    }

    fn os_request_user_attention(&self, type_str: String) -> PyResult<bool> {
        let attention_type = match type_str.as_str() {
            "critical" => Some(tao::window::UserAttentionType::Critical),
            "informational" => Some(tao::window::UserAttentionType::Informational),
            _ => None,
        };
        self.proxy.send_event(UserEvent::RequestUserAttention(attention_type)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send user attention event")
        })?;
        Ok(true)
    }

    fn power_get_battery_info(&self) -> PyResult<String> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.proxy.send_event(UserEvent::PowerGetBatteryInfo(tx)).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to send power get battery info event")
        })?;
        rx.recv().map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Failed to receive battery info")
        })
    }
}

#[pyclass]
struct SingleInstanceGuard {
    _instance: single_instance::SingleInstance,
    is_single: bool,
}

#[pymethods]
impl SingleInstanceGuard {
    #[new]
    fn new(name: &str) -> PyResult<Self> {
        let instance = single_instance::SingleInstance::new(name).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Single instance error: {:?}", e))
        })?;
        let is_single = instance.is_single();
        Ok(SingleInstanceGuard {
            _instance: instance,
            is_single,
        })
    }

    fn is_single(&self) -> bool {
        self.is_single
    }
}

/// NativeWindow - The Rust-backed window for Forge Framework.
///
/// In Python 3.14+ free-threaded mode, the IPC callback can be invoked
/// without acquiring the GIL, enabling true parallel command execution.
#[pyclass]
struct NativeWindow {
    title: String,
    base_path: PathBuf,
    width: f64,
    height: f64,
    fullscreen: bool,
    resizable: bool,
    decorations: bool,
    transparent: bool,
    always_on_top: bool,
    min_width: f64,
    min_height: f64,
    x: Option<f64>,
    y: Option<f64>,
    vibrancy: Option<String>,
    ipc_callback: Option<Py<PyAny>>,
    ready_callback: Option<Py<PyAny>>,
    window_event_callback: Option<Py<PyAny>>,
}

#[pymethods]
impl NativeWindow {
    #[new]
    #[pyo3(signature = (
        title,
        base_path,
        width = 800.0,
        height = 600.0,
        fullscreen = false,
        resizable = true,
        decorations = true,
        transparent = false,
        always_on_top = false,
        min_width = 400.0,
        min_height = 300.0,
        x = None,
        y = None,
        vibrancy = None,
    ))]
    fn new(
        title: String,
        base_path: String,
        width: f64,
        height: f64,
        fullscreen: bool,
        resizable: bool,
        decorations: bool,
        transparent: bool,
        always_on_top: bool,
        min_width: f64,
        min_height: f64,
        x: Option<f64>,
        y: Option<f64>,
        vibrancy: Option<String>,
    ) -> Self {
        NativeWindow {
            title,
            base_path: PathBuf::from(base_path),
            width,
            height,
            fullscreen,
            resizable,
            decorations,
            transparent,
            always_on_top,
            min_width,
            min_height,
            x,
            y,
            vibrancy,
            ipc_callback: None,
            ready_callback: None,
            window_event_callback: None,
        }
    }

    /// Register the Python IPC callback.
    ///
    /// The callback receives two arguments: (message: str, proxy: WindowProxy).
    /// The proxy can be used to send JS back to the WebView without touching
    /// NativeWindow (avoiding the PyO3 borrow conflict).
    fn set_ipc_callback(&mut self, callback: Py<PyAny>) {
        self.ipc_callback = Some(callback);
    }

    /// Register a callback that fires once the window is ready.
    ///
    /// The callback receives one argument: (proxy: WindowProxy).
    /// This allows Python code to store the proxy for later use (e.g. emitting
    /// events to JS from background threads).
    fn set_ready_callback(&mut self, callback: Py<PyAny>) {
        self.ready_callback = Some(callback);
    }

    /// Register a callback for native window lifecycle/state events.
    ///
    /// The callback receives two arguments: (event_name: str, payload_json: str).
    fn set_window_event_callback(&mut self, callback: Py<PyAny>) {
        self.window_event_callback = Some(callback);
    }

    /// Launch the native window and block until closed.
    ///
    /// The IPC handler uses Python::attach which, under free-threaded Python 3.14+,
    /// does NOT serialize execution -- multiple IPC calls run truly in parallel.
    ///
    /// On launch, a WindowProxy is created and passed to:
    ///   1. The IPC callback (as the second argument on each call)
    ///   2. The ready callback (once, immediately after window creation)
    fn run(slf: PyRefMut<'_, Self>) -> PyResult<()> {
        let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
        let proxy = event_loop.create_proxy();


        let mut builder = WindowBuilder::new()
            .with_title(&slf.title)
            .with_inner_size(tao::dpi::LogicalSize::new(slf.width, slf.height))
            .with_min_inner_size(tao::dpi::LogicalSize::new(slf.min_width, slf.min_height))
            .with_fullscreen(if slf.fullscreen {
                Some(Fullscreen::Borderless(None))
            } else {
                None
            })
            .with_resizable(slf.resizable)
            .with_decorations(slf.decorations)
            .with_transparent(slf.transparent)
            .with_always_on_top(slf.always_on_top);

        if let (Some(x), Some(y)) = (slf.x, slf.y) {
            builder = builder.with_position(tao::dpi::LogicalPosition::new(x, y));
        }

        let main_window = builder
            .build(&event_loop)

            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                    "Failed to build window: {}",
                    e
                ))
            })?;


        #[cfg(target_os = "linux")]
        {
            let _ = &slf.vibrancy;
        }


        #[cfg(target_os = "linux")]
        {
            let _ = &slf.vibrancy;
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(v) = &slf.vibrancy {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
                let material = match v.as_str() {
                    "appearance_based" => Some(NSVisualEffectMaterial::AppearanceBased),
                    "light" => Some(NSVisualEffectMaterial::Light),
                    "dark" => Some(NSVisualEffectMaterial::Dark),
                    "titlebar" => Some(NSVisualEffectMaterial::Titlebar),
                    "selection" => Some(NSVisualEffectMaterial::Selection),
                    "menu" => Some(NSVisualEffectMaterial::Menu),
                    "popover" => Some(NSVisualEffectMaterial::Popover),
                    "sidebar" => Some(NSVisualEffectMaterial::Sidebar),
                    "header_view" => Some(NSVisualEffectMaterial::HeaderView),
                    "sheet" => Some(NSVisualEffectMaterial::Sheet),
                    "window_background" => Some(NSVisualEffectMaterial::WindowBackground),
                    "hud_window" => Some(NSVisualEffectMaterial::HudWindow),
                    "full_screen_ui" => Some(NSVisualEffectMaterial::FullScreenUI),
                    "tooltip" => Some(NSVisualEffectMaterial::Tooltip),
                    "content_background" => Some(NSVisualEffectMaterial::ContentBackground),
                    "under_window_background" => Some(NSVisualEffectMaterial::UnderWindowBackground),
                    "under_page_background" => Some(NSVisualEffectMaterial::UnderPageBackground),
                    _ => None,
                };
                if let Some(mat) = material {
                    let _ = apply_vibrancy(&main_window, mat, None, None);
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(v) = &slf.vibrancy {
                use window_vibrancy::{apply_mica, apply_acrylic, apply_blur};
                match v.as_str() {
                    "mica" => { let _ = apply_mica(&main_window, None); }
                    "acrylic" => { let _ = apply_acrylic(&main_window, None); }
                    "blur" => { let _ = apply_blur(&main_window, None); }
                    _ => {}
                }
            }
        }


        #[cfg(target_os = "linux")]
        {
            let _ = &slf.vibrancy;
        }


        #[cfg(target_os = "linux")]
        {
            let _ = &slf.vibrancy;
        }

        #[cfg(target_os = "macos")]
        {
            if let Some(v) = &slf.vibrancy {
                use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
                let material = match v.as_str() {
                    "appearance_based" => Some(NSVisualEffectMaterial::AppearanceBased),
                    "light" => Some(NSVisualEffectMaterial::Light),
                    "dark" => Some(NSVisualEffectMaterial::Dark),
                    "titlebar" => Some(NSVisualEffectMaterial::Titlebar),
                    "selection" => Some(NSVisualEffectMaterial::Selection),
                    "menu" => Some(NSVisualEffectMaterial::Menu),
                    "popover" => Some(NSVisualEffectMaterial::Popover),
                    "sidebar" => Some(NSVisualEffectMaterial::Sidebar),
                    "header_view" => Some(NSVisualEffectMaterial::HeaderView),
                    "sheet" => Some(NSVisualEffectMaterial::Sheet),
                    "window_background" => Some(NSVisualEffectMaterial::WindowBackground),
                    "hud_window" => Some(NSVisualEffectMaterial::HudWindow),
                    "full_screen_ui" => Some(NSVisualEffectMaterial::FullScreenUI),
                    "tooltip" => Some(NSVisualEffectMaterial::Tooltip),
                    "content_background" => Some(NSVisualEffectMaterial::ContentBackground),
                    "under_window_background" => Some(NSVisualEffectMaterial::UnderWindowBackground),
                    "under_page_background" => Some(NSVisualEffectMaterial::UnderPageBackground),
                    _ => None,
                };
                if let Some(mat) = material {
                    let _ = apply_vibrancy(&main_window, mat, None, None);
                }
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(v) = &slf.vibrancy {
                use window_vibrancy::{apply_mica, apply_acrylic, apply_blur};
                match v.as_str() {
                    "mica" => { let _ = apply_mica(&main_window, None); }
                    "acrylic" => { let _ = apply_acrylic(&main_window, None); }
                    "blur" => { let _ = apply_blur(&main_window, None); }
                    _ => {}
                }
            }
        }

        // ─── LINUX: Add GtkHeaderBar for proper window decorations ───
        // GTK on Linux may use client-side decorations (CSD) depending on the
        // desktop environment and theme. When CSD is active, the title bar is
        // drawn by GTK as part of the window content. Without a GtkHeaderBar,
        // the WebView fills the entire window and the close/minimize/maximize
        // buttons are missing. Adding a HeaderBar ensures these controls are
        // always visible in both CSD and SSD environments.
        #[cfg(target_os = "linux")]
        if slf.decorations {
            let gtk_window = main_window.gtk_window();
            let header_bar = gtk::HeaderBar::new();
            header_bar.set_show_close_button(true);
            header_bar.set_title(Some(&slf.title));
            gtk_window.set_titlebar(Some(&header_bar));
            header_bar.show_all();
        }

        #[cfg(target_os = "linux")]
        let menu_bar = {
            let vbox = main_window.default_vbox().expect(
                "tao window should have a default vbox; \
                 did you disable it with with_default_vbox(false)?",
            );
            let menu_bar = gtk::MenuBar::new();
            menu_bar.hide();
            vbox.pack_start(&menu_bar, false, false, 0);
            vbox.reorder_child(&menu_bar, 0);
            menu_bar
        };

        // Create the Python-visible WindowProxy (holds only the EventLoopProxy)
        let py = slf.py();
        let window_proxy = WindowProxy {
            proxy: proxy.clone(),
        };
        let window_proxy_py = Py::new(py, window_proxy.clone())?;

        // Clone callbacks out before dropping the PyRefMut borrow
        let ipc_cb = slf.ipc_callback.as_ref().map(|cb| cb.clone_ref(py));
        let ready_cb = slf.ready_callback.as_ref().map(|cb| cb.clone_ref(py));
        let window_event_cb = slf.window_event_callback.as_ref().map(|cb| cb.clone_ref(py));
        let root_path = slf.base_path.clone();
        let main_title = slf.title.clone();
        let main_width = slf.width;
        let main_height = slf.height;
        let main_fullscreen = slf.fullscreen;
        let main_resizable = slf.resizable;
        let main_decorations = slf.decorations;
        let main_always_on_top = slf.always_on_top;
        let main_min_width = slf.min_width;
        let main_min_height = slf.min_height;

        // Drop the mutable borrow on NativeWindow before entering the event loop.
        // From here on, all communication goes through WindowProxy / EventLoopProxy.
        drop(slf);

        let main_webview = build_webview_for_window(
            &main_window,
            "main",
            "forge://app/index.html",
            root_path.clone(),
            clone_py_callback(&ipc_cb),
            clone_py_callback(&window_event_cb),
            window_proxy_py.clone_ref(py),
        )
        .map_err(|error| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to build WebView: {}",
                error
            ))
        })?;

        let main_window_id = main_window.id();
        let mut windows_by_id: HashMap<WindowId, RuntimeWindow> = HashMap::new();
        let mut labels_to_id: HashMap<String, WindowId> = HashMap::new();
        windows_by_id.insert(
            main_window_id,
            RuntimeWindow {
                label: "main".to_string(),
                url: "forge://app/index.html".to_string(),
                window: main_window,
                webview: main_webview,
                #[cfg(target_os = "linux")]
                menu_bar,
            },
        );
        labels_to_id.insert("main".to_string(), main_window_id);

        if let Some(cb) = ready_cb {
            Python::attach(|py| {
                if let Err(error) = cb.call1(py, (window_proxy_py.clone_ref(py),)) {
                    eprintln!("[forge-core] ready callback error: {}", error);
                }
            });
        }

        emit_window_event(
            &window_event_cb,
            "ready",
            "main",
            serde_json::json!({
                "title": main_title,
                "url": "forge://app/index.html",
                "width": main_width,
                "height": main_height,
                "fullscreen": main_fullscreen,
                "resizable": main_resizable,
                "decorations": main_decorations,
                "always_on_top": main_always_on_top,
                "visible": true,
                "min_width": main_min_width,
                "min_height": main_min_height,
            }),
        );

        #[cfg(target_os = "linux")]
        let emit_menu_selection: MenuEmitter = {
            let cb = clone_py_callback(&window_event_cb);
            Rc::new(
                move |item_id: String,
                      label: Option<String>,
                      role: Option<String>,
                      checked: Option<bool>| {
                    emit_window_event(
                        &cb,
                        "menu_selected",
                        "main",
                        serde_json::json!({
                            "id": item_id,
                            "label": label,
                            "role": role,
                            "checked": checked,
                        }),
                    );
                },
            )
        };

        // ─── GLOBAL HOTKEYS ───
        let hotkey_manager = global_hotkey::GlobalHotKeyManager::new().expect("Failed to initialize hotkey manager");
        let hotkey_channel = global_hotkey::GlobalHotKeyEvent::receiver();
        let mut registered_hotkeys: std::collections::HashMap<String, global_hotkey::hotkey::HotKey> = std::collections::HashMap::new();
        let mut hotkey_id_to_string: std::collections::HashMap<u32, String> = std::collections::HashMap::new();

        // ─── EVENT LOOP ───
        event_loop.run(move |event, target, control_flow| {
            *control_flow = ControlFlow::Wait;

            // Check global hotkeys
            if let Ok(hotkey_event) = hotkey_channel.try_recv() {
                if hotkey_event.state == global_hotkey::HotKeyState::Released {
                    if let Some(accelerator) = hotkey_id_to_string.get(&hotkey_event.id) {
                        emit_window_event(&window_event_cb, "global_shortcut", "main", serde_json::json!({
                            "accelerator": accelerator
                        }));
                    }
                }
            }

            match event {
                Event::UserEvent(UserEvent::Eval(script)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            let _ = runtime_window.webview.evaluate_script(&script);
                        }
                    }
                }
                Event::UserEvent(UserEvent::LoadUrl(url)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get_mut(main_id) {
                            runtime_window.url = url.clone();
                            let _ = runtime_window.webview.load_url(&url);
                        }
                    }
                }
                Event::UserEvent(UserEvent::Reload) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            let _ = runtime_window.webview.reload();
                        }
                    }
                    emit_window_event(&window_event_cb, "reloaded", "main", serde_json::Value::Null);
                }
                Event::UserEvent(UserEvent::GoBack) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            let _ = runtime_window.webview.evaluate_script("window.history.back();");
                        }
                    }
                    emit_window_event(&window_event_cb, "history_back", "main", serde_json::Value::Null);
                }
                Event::UserEvent(UserEvent::GoForward) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            let _ = runtime_window.webview.evaluate_script("window.history.forward();");
                        }
                    }
                    emit_window_event(&window_event_cb, "history_forward", "main", serde_json::Value::Null);
                }
                Event::UserEvent(UserEvent::OpenDevtools) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.webview.open_devtools();
                        }
                    }
                    emit_window_event(&window_event_cb, "devtools", "main", serde_json::json!({ "open": true }));
                }
                Event::UserEvent(UserEvent::CloseDevtools) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.webview.close_devtools();
                        }
                    }
                    emit_window_event(&window_event_cb, "devtools", "main", serde_json::json!({ "open": false }));
                }
                Event::UserEvent(UserEvent::SetTitle(title)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.window.set_title(&title);
                        }
                    }
                }
                Event::UserEvent(UserEvent::Resize(w, h)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.window.set_inner_size(tao::dpi::LogicalSize::new(w, h));
                        }
                    }
                }
                Event::UserEvent(UserEvent::SetPosition(x, y)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.window.set_outer_position(tao::dpi::LogicalPosition::new(x, y));
                        }
                    }
                }
                Event::UserEvent(UserEvent::SetFullscreen(enabled)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.window.set_fullscreen(if enabled {
                                Some(Fullscreen::Borderless(None))
                            } else {
                                None
                            });
                        }
                    }
                }
                Event::UserEvent(UserEvent::SetVisible(visible)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.window.set_visible(visible);
                        }
                    }
                }
                Event::UserEvent(UserEvent::SetMinimized(minimized)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.window.set_minimized(minimized);
                        }
                    }
                }
                Event::UserEvent(UserEvent::SetMaximized(maximized)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.window.set_maximized(maximized);
                        }
                    }
                }
                Event::UserEvent(UserEvent::SetAlwaysOnTop(always_on_top)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.window.set_always_on_top(always_on_top);
                        }
                    }
                }
                Event::UserEvent(UserEvent::SetMenu(menu_json)) => {
                    #[cfg(target_os = "linux")]
                    {
                        if let Some(main_id) = labels_to_id.get("main") {
                            if let Some(runtime_window) = windows_by_id.get(main_id) {
                                if let Err(error) = apply_linux_menu(&runtime_window.menu_bar, &menu_json, emit_menu_selection.clone()) {
                                    emit_window_event(&window_event_cb, "menu_error", "main", serde_json::json!({ "error": error }));
                                }
                            }
                        }
                    }
                    #[cfg(not(target_os = "linux"))]
                    {
                        emit_window_event(&window_event_cb, "menu_unsupported", "main", serde_json::json!({ "platform": std::env::consts::OS }));
                    }
                }
                Event::UserEvent(UserEvent::Focus) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.window.set_focus();
                        }
                    }
                }
                Event::UserEvent(UserEvent::CreateWindow(descriptor)) => {
                    let label = descriptor.label.trim().to_lowercase();
                    if label.is_empty() {
                        emit_window_event(&window_event_cb, "window_error", "main", serde_json::json!({ "error": "Window label is required" }));
                    } else if labels_to_id.contains_key(&label) {
                        emit_window_event(&window_event_cb, "window_error", &label, serde_json::json!({ "error": "Window already exists" }));
                    } else if let Ok(child_window) = WindowBuilder::new()
                        .with_title(&descriptor.title)
                        .with_inner_size(tao::dpi::LogicalSize::new(descriptor.width, descriptor.height))
                        .with_min_inner_size(tao::dpi::LogicalSize::new(descriptor.min_width, descriptor.min_height))
                        .with_fullscreen(if descriptor.fullscreen {
                            Some(Fullscreen::Borderless(None))
                        } else {
                            None
                        })
                        .with_resizable(descriptor.resizable)
                        .with_decorations(descriptor.decorations)
                        .with_transparent(descriptor.transparent)
                        .with_always_on_top(descriptor.always_on_top)
                        .build(target)
                    {
                        #[cfg(target_os = "linux")]
                        if descriptor.decorations {
                            let gtk_window = child_window.gtk_window();
                            let header_bar = gtk::HeaderBar::new();
                            header_bar.set_show_close_button(true);
                            header_bar.set_title(Some(&descriptor.title));
                            gtk_window.set_titlebar(Some(&header_bar));
                            header_bar.show_all();
                        }

                        #[cfg(target_os = "linux")]
                        let child_menu_bar = {
                            let vbox = child_window.default_vbox().expect(
                                "tao window should have a default vbox; \
                                 did you disable it with with_default_vbox(false)?",
                            );
                            let menu_bar = gtk::MenuBar::new();
                            menu_bar.hide();
                            vbox.pack_start(&menu_bar, false, false, 0);
                            vbox.reorder_child(&menu_bar, 0);
                            menu_bar
                        };

                        if let Ok(child_webview) = build_webview_for_window(
                            &child_window,
                            &label,
                            &descriptor.url,
                            root_path.clone(),
                            clone_py_callback(&ipc_cb),
                            clone_py_callback(&window_event_cb),
                            Python::attach(|py| window_proxy_py.clone_ref(py)),
                        ) {
                            if !descriptor.visible {
                                child_window.set_visible(false);
                            }
                            if descriptor.focus {
                                child_window.set_focus();
                            }

                            let child_window_id = child_window.id();
                            windows_by_id.insert(
                                child_window_id,
                                RuntimeWindow {
                                    label: label.clone(),
                                    url: descriptor.url.clone(),
                                    window: child_window,
                                    webview: child_webview,
                                    #[cfg(target_os = "linux")]
                                    menu_bar: child_menu_bar,
                                },
                            );
                            labels_to_id.insert(label.clone(), child_window_id);
                            emit_window_event(
                                &window_event_cb,
                                "created",
                                &label,
                                serde_json::json!({
                                    "title": descriptor.title,
                                    "url": descriptor.url,
                                    "width": descriptor.width,
                                    "height": descriptor.height,
                                    "fullscreen": descriptor.fullscreen,
                                    "resizable": descriptor.resizable,
                                    "decorations": descriptor.decorations,
                                    "always_on_top": descriptor.always_on_top,
                                    "visible": descriptor.visible,
                                    "focused": descriptor.focus,
                                }),
                            );
                        } else {
                            emit_window_event(&window_event_cb, "window_error", &label, serde_json::json!({ "error": "Failed to build WebView" }));
                        }
                    } else {
                        emit_window_event(&window_event_cb, "window_error", &label, serde_json::json!({ "error": "Failed to build window" }));
                    }
                }
                Event::UserEvent(UserEvent::CloseLabel(label)) => {
                    let normalized = label.trim().to_lowercase();
                    if normalized == "main" {
                        emit_window_event(&window_event_cb, "close_requested", "main", serde_json::Value::Null);
                        *control_flow = ControlFlow::Exit;
                    } else if let Some(window_id) = labels_to_id.remove(&normalized) {
                        windows_by_id.remove(&window_id);
                        emit_window_event(&window_event_cb, "destroyed", &normalized, serde_json::Value::Null);
                    }
                }
                Event::UserEvent(UserEvent::Close) => {
                    emit_window_event(&window_event_cb, "close_requested", "main", serde_json::Value::Null);
                    *control_flow = ControlFlow::Exit;
                }
                Event::UserEvent(UserEvent::GetMonitors(tx)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            let mut monitors = Vec::new();
                            for m in runtime_window.window.available_monitors() {
                                let start = m.position();
                                let size = m.size();
                                let is_primary = runtime_window.window.primary_monitor().map_or(false, |pm| pm.name() == m.name());
                                let mon_json = serde_json::json!({
                                    "name": m.name(),
                                    "position": { "x": start.x, "y": start.y },
                                    "size": { "width": size.width, "height": size.height },
                                    "scale_factor": m.scale_factor(),
                                    "is_primary": is_primary
                                });
                                monitors.push(mon_json);
                            }
                            let _ = tx.send(serde_json::to_string(&monitors).unwrap_or_else(|_| "[]".to_string()));
                        } else {
                            let _ = tx.send("[]".into());
                        }
                    } else {
                        let _ = tx.send("[]".into());
                    }
                }
                Event::UserEvent(UserEvent::GetPrimaryMonitor(tx)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            if let Some(m) = runtime_window.window.primary_monitor() {
                                let start = m.position();
                                let size = m.size();
                                let mon_json = serde_json::json!({
                                    "name": m.name(),
                                    "position": { "x": start.x, "y": start.y },
                                    "size": { "width": size.width, "height": size.height },
                                    "scale_factor": m.scale_factor(),
                                    "is_primary": true
                                });
                                let _ = tx.send(serde_json::to_string(&mon_json).unwrap_or_else(|_| "null".into()));
                            } else {
                                let _ = tx.send("null".into());
                            }
                        } else {
                            let _ = tx.send("null".into());
                        }
                    } else {
                        let _ = tx.send("null".into());
                    }
                }
                Event::UserEvent(UserEvent::GetCursorPosition(tx)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            if let Ok(pos) = runtime_window.window.cursor_position() {
                                let pos_json = serde_json::json!({
                                    "x": pos.x as i32,
                                    "y": pos.y as i32
                                });
                                let _ = tx.send(serde_json::to_string(&pos_json).unwrap_or_else(|_| "{\"x\":0,\"y\":0}".into()));
                            } else {
                                let _ = tx.send("{\"x\":0,\"y\":0}".into());
                            }
                        } else {
                            let _ = tx.send("{\"x\":0,\"y\":0}".into());
                        }
                    } else {
                        let _ = tx.send("{\"x\":0,\"y\":0}".into());
                    }
                }
                Event::UserEvent(UserEvent::RegisterShortcut(accelerator, tx)) => {
                    use std::str::FromStr;
                    match global_hotkey::hotkey::HotKey::from_str(&accelerator) {
                        Ok(hotkey) => {
                            if hotkey_manager.register(hotkey).is_ok() {
                                registered_hotkeys.insert(accelerator.clone(), hotkey);
                                hotkey_id_to_string.insert(hotkey.id(), accelerator.clone());
                                let _ = tx.send(true);
                            } else {
                                let _ = tx.send(false);
                            }
                        }
                        Err(_) => {
                            let _ = tx.send(false);
                        }
                    }
                }
                Event::UserEvent(UserEvent::UnregisterShortcut(accelerator, tx)) => {
                    if let Some(hotkey) = registered_hotkeys.remove(&accelerator) {
                        hotkey_id_to_string.remove(&hotkey.id());
                        let _ = hotkey_manager.unregister(hotkey);
                        let _ = tx.send(true);
                    } else {
                        let _ = tx.send(false);
                    }
                }
                Event::UserEvent(UserEvent::UnregisterAllShortcuts(tx)) => {
                    for (_, hotkey) in registered_hotkeys.drain() {
                        let _ = hotkey_manager.unregister(hotkey);
                    }
                    hotkey_id_to_string.clear();
                    let _ = tx.send(true);
                }
                Event::UserEvent(UserEvent::Print(label)) => {
                    if let Some(window_id) = labels_to_id.get(&label) {
                        if let Some(runtime_window) = windows_by_id.get(window_id) {
                            let _ = runtime_window.webview.print();
                        }
                    }
                }
                Event::UserEvent(UserEvent::SetProgressBar(progress)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            let state = if progress < 0.0 {
                                tao::window::ProgressBarState {
                                    progress: None,
                                    state: None,
                                    desktop_filename: None,
                                }
                            } else {
                                tao::window::ProgressBarState {
                                    progress: Some((progress * 100.0) as u64),
                                    state: Some(tao::window::ProgressState::Normal),
                                    desktop_filename: None,
                                }
                            };
                            runtime_window.window.set_progress_bar(state);
                        }
                    }
                }
                Event::UserEvent(UserEvent::RequestUserAttention(attention_type)) => {
                    if let Some(main_id) = labels_to_id.get("main") {
                        if let Some(runtime_window) = windows_by_id.get(main_id) {
                            runtime_window.window.request_user_attention(attention_type);
                        }
                    }
                }
                Event::UserEvent(UserEvent::PowerGetBatteryInfo(tx)) => {
                    let mut battery_info = "{}".to_string();
                    if let Ok(manager) = starship_battery::Manager::new() {
                        if let Ok(mut batteries) = manager.batteries() {
                            if let Some(Ok(battery)) = batteries.next() {
                                let state = match battery.state() {
                                    starship_battery::State::Charging => "charging",
                                    starship_battery::State::Discharging => "discharging",
                                    starship_battery::State::Empty => "empty",
                                    starship_battery::State::Full => "full",
                                    _ => "unknown",
                                };
                                let charge = battery.state_of_charge().value;
                                battery_info = format!(r#"{{"state": "{}", "charge": {}}}"#, state, charge);
                            }
                        }
                    }
                    let _ = tx.send(battery_info);
                }
                Event::WindowEvent { event, window_id, .. } => {
                    if let Some(runtime_window) = windows_by_id.get(&window_id) {
                        let label = runtime_window.label.clone();

                        match event {
                            WindowEvent::Resized(size) => {
                                emit_window_event(&window_event_cb, "resized", &label, serde_json::json!({
                                    "width": size.width,
                                    "height": size.height,
                                }));
                            }
                            WindowEvent::Moved(position) => {
                                emit_window_event(&window_event_cb, "moved", &label, serde_json::json!({
                                    "x": position.x,
                                    "y": position.y,
                                }));
                            }
                            WindowEvent::Focused(focused) => {
                                emit_window_event(&window_event_cb, "focused", &label, serde_json::json!({ "focused": focused }));
                            }
                            WindowEvent::CloseRequested => {
                                emit_window_event(&window_event_cb, "close_requested", &label, serde_json::Value::Null);
                                if label == "main" {
                                    *control_flow = ControlFlow::Exit;
                                } else {
                                    labels_to_id.remove(&label);
                                    windows_by_id.remove(&window_id);
                                    emit_window_event(&window_event_cb, "destroyed", &label, serde_json::Value::Null);
                                }
                            }
                            WindowEvent::Destroyed => {
                                labels_to_id.remove(&label);
                                windows_by_id.remove(&window_id);
                                emit_window_event(&window_event_cb, "destroyed", &label, serde_json::Value::Null);
                                if windows_by_id.is_empty() {
                                    *control_flow = ControlFlow::Exit;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Event::Suspended => {
                    emit_window_event(&window_event_cb, "power:suspended", "main", serde_json::Value::Null);
                }
                Event::Resumed => {
                    emit_window_event(&window_event_cb, "power:resumed", "main", serde_json::Value::Null);
                }
                _ => (),
            }
        });
    }
}

#[pyclass]
struct AutoLaunchManager {
    inner: auto_launch::AutoLaunch,
}

#[pymethods]
impl AutoLaunchManager {
    #[new]
    fn new(app_name: &str) -> PyResult<Self> {
        let app_path = std::env::current_exe().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get executable path: {}", e))
        })?;
        let path_str = app_path.to_str().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Invalid executable path")
        })?;
        let auto_launch = auto_launch::AutoLaunchBuilder::new()
            .set_app_name(app_name)
            .set_app_path(path_str)
            .build()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to build auto_launch: {}", e))
            })?;
        Ok(AutoLaunchManager { inner: auto_launch })
    }

    fn enable(&self) -> PyResult<bool> {
        self.inner.enable().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to enable autostart: {}", e))
        })?;
        Ok(true)
    }

    fn disable(&self) -> PyResult<bool> {
        self.inner.disable().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to disable autostart: {}", e))
        })?;
        Ok(true)
    }

    fn is_enabled(&self) -> PyResult<bool> {
        self.inner.is_enabled().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to check autostart: {}", e))
        })
    }
}
#[pyclass]
struct KeychainManager {
    service: String,
}

#[pymethods]
impl KeychainManager {
    #[new]
    fn new(service: &str) -> PyResult<Self> {
        Ok(KeychainManager {
            service: service.to_string(),
        })
    }

    fn set_password(&self, user: &str, password: &str) -> PyResult<()> {
        let entry = keyring::Entry::new(&self.service, user).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get keyring entry: {}", e))
        })?;
        entry.set_password(password).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to set password: {}", e))
        })?;
        Ok(())
    }

    fn get_password(&self, user: &str) -> PyResult<String> {
        let entry = keyring::Entry::new(&self.service, user).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get keyring entry: {}", e))
        })?;
        let password = entry.get_password().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get password: {}", e))
        })?;
        Ok(password)
    }

    fn delete_password(&self, user: &str) -> PyResult<()> {
        let entry = keyring::Entry::new(&self.service, user).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get keyring entry: {}", e))
        })?;
        entry.delete_credential().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to delete password: {}", e))
        })?;
        Ok(())
    }
}
#[pymodule]
fn forge_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<NativeWindow>()?;
    m.add_class::<WindowProxy>()?;
    m.add_class::<SingleInstanceGuard>()?;
    m.add_class::<AutoLaunchManager>()?;
    m.add_class::<KeychainManager>()?;
    Ok(())
}
