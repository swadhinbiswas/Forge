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
        self.proxy.send_event(UserEvent::RegisterShortcut(accelerator, tx)).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to send register shortcut event: {}", e))
        })?;
        Ok(rx.recv().unwrap_or(false))
    }

    fn unregister_shortcut(&self, accelerator: String) -> PyResult<bool> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.proxy.send_event(UserEvent::UnregisterShortcut(accelerator, tx)).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to send unregister shortcut event: {}", e))
        })?;
        Ok(rx.recv().unwrap_or(false))
    }

    fn unregister_all(&self) -> PyResult<bool> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        self.proxy.send_event(UserEvent::UnregisterAllShortcuts(tx)).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to send unregister all shortcuts event: {}", e))
        })?;
        Ok(rx.recv().unwrap_or(false))
    }

    fn print(&self, label: String) -> PyResult<()> {
        self.proxy.send_event(UserEvent::Print(label)).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to send print event: {}", e))
        })?;
        Ok(())
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
                            WindowEvent::DroppedFile(path) => {
                                emit_window_event(&window_event_cb, "file_drop", &label, serde_json::json!({ "paths": [path] }));
                            }
                            WindowEvent::HoveredFile(path) => {
                                emit_window_event(&window_event_cb, "file_drop_hover", &label, serde_json::json!({ "paths": [path] }));
                            }
                            WindowEvent::HoveredFileCancelled => {
                                emit_window_event(&window_event_cb, "file_drop_cancelled", &label, serde_json::Value::Null);
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
