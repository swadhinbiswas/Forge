/**
 * Forge API Explorer - Main JavaScript
 * 
 * Interactive exploration of all Forge Framework APIs.
 */

// Demo configurations
const demos = {
    // System APIs
    'system-info': {
        title: 'System Information',
        description: 'Get comprehensive system information including OS, Python version, and hardware details.',
        command: 'get_system_info',
        params: [],
    },
    'app-version': {
        title: 'App Version',
        description: 'Get the application version string.',
        command: 'get_app_version',
        params: [],
    },
    'timestamp': {
        title: 'Current Timestamp',
        description: 'Get current timestamp in various formats.',
        command: 'get_current_timestamp',
        params: [],
    },
    
    // Clipboard APIs
    'clipboard-read': {
        title: 'Read Clipboard',
        description: 'Read text content from the system clipboard.',
        command: 'clipboard_read',
        params: [],
    },
    'clipboard-write': {
        title: 'Write Clipboard',
        description: 'Write text to the system clipboard.',
        command: 'clipboard_write',
        params: [
            { name: 'text', type: 'text', label: 'Text to Copy', placeholder: 'Enter text to copy to clipboard' }
        ],
    },
    
    // File System APIs
    'fs-list': {
        title: 'List Directory',
        description: 'List contents of a directory.',
        command: 'fs_list',
        params: [
            { name: 'path', type: 'text', label: 'Path', placeholder: '.', default: '.' }
        ],
    },
    'fs-read': {
        title: 'Read File',
        description: 'Read contents of a file.',
        command: 'fs_read',
        params: [
            { name: 'path', type: 'text', label: 'File Path', placeholder: 'example.txt' }
        ],
    },
    'fs-write': {
        title: 'Write File',
        description: 'Write content to a file.',
        command: 'fs_write',
        params: [
            { name: 'path', type: 'text', label: 'File Path', placeholder: 'demo.txt' },
            { name: 'content', type: 'textarea', label: 'Content', placeholder: 'Enter content to write' }
        ],
    },
    'fs-exists': {
        title: 'Check Exists',
        description: 'Check if a file or directory exists.',
        command: 'fs_exists',
        params: [
            { name: 'path', type: 'text', label: 'Path', placeholder: 'example.txt' }
        ],
    },
    'fs-mkdir': {
        title: 'Create Directory',
        description: 'Create a new directory.',
        command: 'fs_mkdir',
        params: [
            { name: 'path', type: 'text', label: 'Directory Path', placeholder: 'new_folder' }
        ],
    },
    
    // Dialog APIs
    'dialog-open': {
        title: 'Open File Dialog',
        description: 'Show a native file open dialog and read the selected file.',
        command: 'dialog_open_file',
        params: [],
    },
    'dialog-save': {
        title: 'Save File Dialog',
        description: 'Show a native save dialog and write content to the selected file.',
        command: 'dialog_save_file',
        params: [
            { name: 'content', type: 'textarea', label: 'Content to Save', placeholder: 'Enter content to save' }
        ],
    },
    'dialog-message': {
        title: 'Message Dialog',
        description: 'Show a message dialog.',
        command: 'dialog_message',
        params: [
            { name: 'title', type: 'text', label: 'Title', placeholder: 'Hello' },
            { name: 'body', type: 'text', label: 'Message', placeholder: 'This is a message' },
            { name: 'type', type: 'select', label: 'Type', options: ['info', 'warning', 'error'], default: 'info' }
        ],
    },
    
    // Event APIs
    'counter': {
        title: 'Counter Demo',
        description: 'Start a counter that emits progress events to JavaScript.',
        command: 'start_counter',
        params: [],
        events: ['counter_update', 'counter_complete', 'counter_stop'],
    },
    'custom-event': {
        title: 'Custom Event',
        description: 'Emit a custom event from Python to JavaScript.',
        command: 'emit_custom_event',
        params: [
            { name: 'event_name', type: 'text', label: 'Event Name', placeholder: 'my_custom_event' },
            { name: 'data', type: 'textarea', label: 'Event Data (JSON)', placeholder: '{"key": "value"}' }
        ],
    },
    
    // Utility APIs
    'echo': {
        title: 'Echo',
        description: 'Echo back any data sent. Useful for testing.',
        command: 'echo',
        params: [
            { name: 'data', type: 'textarea', label: 'Data (JSON)', placeholder: '{"test": "hello"}' }
        ],
    },
    'text-process': {
        title: 'Text Processor',
        description: 'Process text with various operations.',
        command: 'process_text',
        params: [
            { name: 'text', type: 'textarea', label: 'Text', placeholder: 'Hello World' },
            { name: 'operation', type: 'select', label: 'Operation', 
              options: ['uppercase', 'lowercase', 'reverse', 'word_count', 'char_count', 'line_count', 'trim'] }
        ],
    },
    'api-list': {
        title: 'API List',
        description: 'Get a list of all available APIs organized by category.',
        command: 'get_api_list',
        params: [],
    },
};

// Current state
let currentDemo = null;
let eventListenersRegistered = {};

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    setupForgeEventListeners();
    logEvent('system', 'API Explorer initialized');
});

/**
 * Set up Forge event listeners.
 */
function setupForgeEventListeners() {
    if (!window.__forge__) {
        logEvent('error', 'Forge runtime not available');
        return;
    }
    
    // Register listeners for all possible events
    const events = [
        'counter_update', 'counter_complete', 'counter_stop',
        'progress_update', 'progress_complete', 'progress_stop'
    ];
    
    events.forEach(eventName => {
        window.__forge__.on(eventName, (data) => {
            handleEvent(eventName, data);
        });
        eventListenersRegistered[eventName] = true;
    });
    
    logEvent('system', `Registered ${events.length} event listeners`);
}

/**
 * Load a demo.
 */
function loadDemo(demoId) {
    const demo = demos[demoId];
    if (!demo) return;
    
    currentDemo = demo;
    
    // Update title and description
    document.getElementById('demoTitle').textContent = demo.title;
    document.getElementById('demoDescription').textContent = demo.description;
    
    // Update nav active state
    document.querySelectorAll('.nav-section li').forEach(li => {
        li.classList.remove('active');
        if (li.getAttribute('onclick')?.includes(demoId)) {
            li.classList.add('active');
        }
    });
    
    // Build request form
    buildRequestForm(demo);
    
    // Clear response
    clearResponse();
    
    logEvent('system', `Loaded demo: ${demoId}`);
}

/**
 * Build the request form for a demo.
 */
function buildRequestForm(demo) {
    const formContainer = document.getElementById('requestForm');
    const executeBtn = document.getElementById('executeBtn');
    
    if (demo.params.length === 0) {
        formContainer.innerHTML = '<p class="placeholder">No parameters required</p>';
        executeBtn.disabled = false;
        return;
    }
    
    let html = '';
    demo.params.forEach(param => {
        html += `<div class="form-group">`;
        html += `<label>${param.label}</label>`;
        
        if (param.type === 'select') {
            html += `<select id="param_${param.name}">`;
            param.options.forEach(opt => {
                const selected = param.default === opt ? 'selected' : '';
                html += `<option value="${opt}" ${selected}>${opt}</option>`;
            });
            html += `</select>`;
        } else if (param.type === 'textarea') {
            html += `<textarea id="param_${param.name}" placeholder="${param.placeholder || ''}">${param.default || ''}</textarea>`;
        } else {
            html += `<input type="text" id="param_${param.name}" placeholder="${param.placeholder || ''}" value="${param.default || ''}">`;
        }
        
        html += `</div>`;
    });
    
    formContainer.innerHTML = html;
    executeBtn.disabled = false;
}

/**
 * Execute the current demo.
 */
async function executeDemo() {
    if (!currentDemo) return;
    
    const executeBtn = document.getElementById('executeBtn');
    const responseOutput = document.getElementById('responseOutput');
    
    // Build parameters
    const params = {};
    currentDemo.params.forEach(param => {
        const value = document.getElementById(`param_${param.name}`).value;
        
        // Try to parse JSON for textarea fields
        if (param.type === 'textarea' && (param.name.includes('data') || param.name.includes('json'))) {
            try {
                params[param.name] = JSON.parse(value);
            } catch (e) {
                params[param.name] = value;
            }
        } else {
            params[param.name] = value;
        }
    });
    
    // Execute
    executeBtn.disabled = true;
    executeBtn.textContent = '⏳ Executing...';
    responseOutput.innerHTML = '<span class="placeholder">Calling Python...</span>';
    
    logEvent('system', `Executing: ${currentDemo.command}`, params);
    
    try {
        const startTime = Date.now();
        const result = await window.__forge__.invoke(currentDemo.command, params);
        const duration = Date.now() - startTime;
        
        // Display result
        responseOutput.innerHTML = `<span class="success">${syntaxHighlight(JSON.stringify(result, null, 2))}</span>`;
        logEvent('event', `Response received in ${duration}ms`, result);
        
    } catch (error) {
        responseOutput.innerHTML = `<span class="error">Error: ${error.message}</span>`;
        logEvent('error', `Execution failed: ${error.message}`);
    } finally {
        executeBtn.disabled = false;
        executeBtn.textContent = '▶ Execute';
    }
}

/**
 * Handle events from Python.
 */
function handleEvent(eventName, data) {
    logEvent('event', `Received event: ${eventName}`, data);
    
    // Special handling for counter demo
    if (eventName === 'counter_update') {
        const responseOutput = document.getElementById('responseOutput');
        responseOutput.innerHTML = `<span class="success">Counter: ${data.count}%</span>`;
    }
    
    if (eventName === 'counter_complete') {
        const responseOutput = document.getElementById('responseOutput');
        responseOutput.innerHTML += `\n<span class="success">${data.message}</span>`;
        
        const executeBtn = document.getElementById('executeBtn');
        executeBtn.disabled = false;
        executeBtn.textContent = '▶ Execute';
    }
}

/**
 * Log an event.
 */
function logEvent(type, message, data = null) {
    const eventLog = document.getElementById('eventLog');
    const timestamp = new Date().toLocaleTimeString();
    
    let html = `<div class="log-entry ${type}">`;
    html += `<span class="log-timestamp">[${timestamp}]</span>`;
    html += `${message}`;
    
    if (data) {
        html += ` → ${JSON.stringify(data)}`;
    }
    
    html += `</div>`;
    eventLog.innerHTML += html;
    eventLog.scrollTop = eventLog.scrollHeight;
}

/**
 * Clear event log.
 */
function clearEventLog() {
    document.getElementById('eventLog').innerHTML = '<p class="log-entry system">Event log cleared</p>';
}

/**
 * Copy response to clipboard.
 */
async function copyResponse() {
    const responseOutput = document.getElementById('responseOutput');
    const text = responseOutput.textContent;
    
    if (text && text !== 'Response will appear here') {
        try {
            await window.__forge__.clipboard.write(text);
            logEvent('system', 'Response copied to clipboard');
        } catch (error) {
            logEvent('error', 'Failed to copy: ' + error.message);
        }
    }
}

/**
 * Clear response.
 */
function clearResponse() {
    document.getElementById('responseOutput').innerHTML = '<p class="placeholder">Response will appear here</p>';
}

/**
 * Syntax highlight JSON.
 */
function syntaxHighlight(json) {
    if (!json) return '';
    
    json = json.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
    return json.replace(/("(\\u[a-zA-Z0-9]{4}|\\[^u]|[^\\"])*"(\s*:)?|\b(true|false|null)\b|-?\d+(?:\.\d*)?(?:[eE][+\-]?\d+)?)/g, (match) => {
        let cls = 'number';
        if (/^"/.test(match)) {
            if (/:$/.test(match)) {
                cls = 'key';
            } else {
                cls = 'string';
            }
        } else if (/true|false/.test(match)) {
            cls = 'boolean';
        } else if (/null/.test(match)) {
            cls = 'null';
        }
        return `<span class="${cls}">${match}</span>`;
    });
}
