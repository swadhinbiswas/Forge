/**
 * Forge Notes - Main JavaScript
 * 
 * A note-taking app demonstrating Forge's file system API.
 */

// State
let currentNote = null;
let notes = [];
let hasUnsavedChanges = false;
let isSearching = false;
let searchResults = null;

// Initialize
document.addEventListener('DOMContentLoaded', () => {
    loadNotes();
    setupEditor();
    setupKeyboardShortcuts();
    updateStats();
});

/**
 * Set up the editor event listeners.
 */
function setupEditor() {
    const editor = document.getElementById('noteEditor');
    
    editor.addEventListener('input', () => {
        hasUnsavedChanges = true;
        updateUnsavedIndicator();
        updateCounts();
    });
}

/**
 * Set up keyboard shortcuts.
 */
function setupKeyboardShortcuts() {
    document.addEventListener('keydown', (e) => {
        // Ctrl+S / Cmd+S to save
        if ((e.ctrlKey || e.metaKey) && e.key === 's') {
            e.preventDefault();
            saveCurrentNote();
        }
        
        // Ctrl+N / Cmd+N for new note
        if ((e.ctrlKey || e.metaKey) && e.key === 'n') {
            e.preventDefault();
            createNewNote();
        }
    });
}

/**
 * Load and display all notes.
 */
async function loadNotes() {
    try {
        notes = await window.__forge__.invoke('list_notes');
        isSearching = false;
        searchResults = null;
        renderNotesList();
        updateStats();
    } catch (error) {
        console.error('[Forge Notes] Failed to load notes:', error);
        showToast('Failed to load notes', 'error');
    }
}

/**
 * Render the notes list in the sidebar.
 */
function renderNotesList() {
    const listEl = document.getElementById('notesList');
    
    const itemsToRender = isSearching && searchResults ? searchResults : notes;
    
    if (itemsToRender.length === 0) {
        if (isSearching) {
            listEl.innerHTML = `
                <div class="empty-search">
                    <p>No notes found matching "${document.getElementById('searchInput').value}"</p>
                </div>
            `;
        } else {
            listEl.innerHTML = `
                <div class="empty-notes">
                    <p>No notes yet</p>
                    <button onclick="createNewNote()">Create your first note</button>
                </div>
            `;
        }
        return;
    }
    
    listEl.innerHTML = itemsToRender.map(note => `
        <div class="note-item ${currentNote === note.name ? 'active' : ''}" 
             onclick="selectNote('${note.name.replace(/'/g, "\\'")}')">
            <div class="note-item-name">${escapeHtml(note.name)}</div>
            <div class="note-item-meta">
                <span>${note.modified_formatted}</span>
                <span>${formatFileSize(note.size)}</span>
            </div>
            ${note.preview ? `<div class="note-item-preview">...${escapeHtml(note.preview)}...</div>` : ''}
        </div>
    `).join('');
}

/**
 * Select and load a note.
 */
async function selectNote(name) {
    // Check for unsaved changes
    if (hasUnsavedChanges) {
        if (!confirm('You have unsaved changes. Discard them?')) {
            return;
        }
    }
    
    try {
        const content = await window.__forge__.invoke('read_note', { name });
        
        currentNote = name;
        document.getElementById('noteTitle').value = name;
        document.getElementById('noteEditor').value = content;
        
        hasUnsavedChanges = false;
        updateUnsavedIndicator();
        updateLastSaved('Loaded from disk');
        updateCounts();
        renderNotesList();
        showEmptyState(false);
        
    } catch (error) {
        console.error('[Forge Notes] Failed to load note:', error);
        showToast('Failed to load note: ' + error.message, 'error');
    }
}

/**
 * Create a new note.
 */
function createNewNote() {
    if (hasUnsavedChanges) {
        if (!confirm('You have unsaved changes. Discard them?')) {
            return;
        }
    }
    
    const name = prompt('Enter note name:', 'Untitled');
    if (!name) return;
    
    currentNote = name;
    document.getElementById('noteTitle').value = name;
    document.getElementById('noteEditor').value = '';
    
    hasUnsavedChanges = false;
    updateUnsavedIndicator();
    updateLastSaved('New note');
    updateCounts();
    renderNotesList();
    showEmptyState(false);
    
    document.getElementById('noteEditor').focus();
}

/**
 * Save the current note.
 */
async function saveCurrentNote() {
    const title = document.getElementById('noteTitle').value.trim();
    const content = document.getElementById('noteEditor').value;
    
    if (!title) {
        showToast('Please enter a note title', 'error');
        return;
    }
    
    if (!content.trim()) {
        showToast('Note is empty', 'error');
        return;
    }
    
    try {
        const result = await window.__forge__.invoke('save_note', { name: title, content });
        
        currentNote = title;
        hasUnsavedChanges = false;
        updateUnsavedIndicator();
        updateLastSaved('Saved just now');
        
        // Refresh notes list
        await loadNotes();
        selectNote(title);
        
        showToast('Note saved successfully', 'success');
        
    } catch (error) {
        console.error('[Forge Notes] Failed to save note:', error);
        showToast('Failed to save note: ' + error.message, 'error');
    }
}

/**
 * Save note using dialog.
 */
async function saveNoteDialog() {
    const content = document.getElementById('noteEditor').value;
    const defaultName = document.getElementById('noteTitle').value.trim() || 'note';
    
    if (!content.trim()) {
        showToast('Note is empty', 'error');
        return;
    }
    
    try {
        const path = await window.__forge__.invoke('save_note_dialog', { content, default_name: defaultName });
        
        if (path) {
            showToast('Saved to: ' + path, 'success');
        }
        
    } catch (error) {
        console.error('[Forge Notes] Failed to save via dialog:', error);
        showToast('Failed to save: ' + error.message, 'error');
    }
}

/**
 * Delete the current note.
 */
async function deleteCurrentNote() {
    if (!currentNote) {
        showToast('No note selected', 'error');
        return;
    }
    
    if (!confirm(`Delete "${currentNote}"? This cannot be undone.`)) {
        return;
    }
    
    try {
        await window.__forge__.invoke('delete_note', { name: currentNote });
        
        currentNote = null;
        document.getElementById('noteTitle').value = '';
        document.getElementById('noteEditor').value = '';
        hasUnsavedChanges = false;
        
        updateUnsavedIndicator();
        updateLastSaved('Note deleted');
        updateCounts();
        showEmptyState(true);
        
        await loadNotes();
        showToast('Note deleted', 'success');
        
    } catch (error) {
        console.error('[Forge Notes] Failed to delete note:', error);
        showToast('Failed to delete note: ' + error.message, 'error');
    }
}

/**
 * Open a note using native file dialog.
 */
async function openNoteDialog() {
    if (hasUnsavedChanges) {
        if (!confirm('You have unsaved changes. Discard them?')) {
            return;
        }
    }
    
    try {
        const result = await window.__forge__.invoke('open_note_dialog');
        
        if (result) {
            currentNote = result.name;
            document.getElementById('noteTitle').value = result.name;
            document.getElementById('noteEditor').value = result.content;
            hasUnsavedChanges = true;
            updateUnsavedIndicator();
            updateLastSaved('Imported from file');
            updateCounts();
            showEmptyState(false);
            renderNotesList();
            
            showToast('File opened: ' + result.path, 'success');
        }
        
    } catch (error) {
        console.error('[Forge Notes] Failed to open file:', error);
        showToast('Failed to open file: ' + error.message, 'error');
    }
}

/**
 * Refresh the notes list.
 */
async function refreshNotes() {
    await loadNotes();
    showToast('Notes refreshed', 'success');
}

/**
 * Search notes.
 */
async function searchNotes() {
    const query = document.getElementById('searchInput').value.trim();
    
    if (!query) {
        // Clear search
        isSearching = false;
        searchResults = null;
        renderNotesList();
        return;
    }
    
    try {
        searchResults = await window.__forge__.invoke('search_notes', { query });
        isSearching = true;
        renderNotesList();
    } catch (error) {
        console.error('[Forge Notes] Search failed:', error);
        showToast('Search failed: ' + error.message, 'error');
    }
}

/**
 * Handle search input.
 */
function handleSearch(event) {
    if (event.key === 'Enter') {
        searchNotes();
    } else if (event.target.value.trim() === '') {
        // Clear search when input is empty
        isSearching = false;
        searchResults = null;
        renderNotesList();
    }
}

/**
 * Handle title change.
 */
function onTitleChange() {
    hasUnsavedChanges = true;
    updateUnsavedIndicator();
    updateLastSaved('Title changed');
}

/**
 * Handle content change.
 */
function onContentChange() {
    hasUnsavedChanges = true;
    updateUnsavedIndicator();
    updateLastSaved('Unsaved changes');
}

/**
 * Update unsaved changes indicator.
 */
function updateUnsavedIndicator() {
    const indicator = document.getElementById('unsavedIndicator');
    if (hasUnsavedChanges) {
        indicator.classList.add('visible');
    } else {
        indicator.classList.remove('visible');
    }
}

/**
 * Update last saved status.
 */
function updateLastSaved(message) {
    document.getElementById('lastSaved').textContent = message;
}

/**
 * Update character, word, and line counts.
 */
function updateCounts() {
    const content = document.getElementById('noteEditor').value;
    
    const charCount = content.length;
    const wordCount = content.trim() ? content.trim().split(/\s+/).length : 0;
    const lineCount = content ? content.split('\n').length : 0;
    
    document.getElementById('charCount').textContent = `${charCount} chars`;
    document.getElementById('wordCount').textContent = `${wordCount} words`;
    document.getElementById('lineCount').textContent = `${lineCount} lines`;
}

/**
 * Update stats in sidebar.
 */
async function updateStats() {
    try {
        const stats = await window.__forge__.invoke('get_note_stats');
        document.getElementById('noteCount').textContent = `${stats.count} note${stats.count !== 1 ? 's' : ''}`;
        document.getElementById('totalSize').textContent = stats.total_size_formatted;
    } catch (error) {
        console.error('[Forge Notes] Failed to get stats:', error);
    }
}

/**
 * Show/hide empty state.
 */
function showEmptyState(show) {
    const emptyState = document.getElementById('emptyState');
    if (show) {
        emptyState.classList.add('visible');
    } else {
        emptyState.classList.remove('visible');
    }
}

/**
 * Show toast notification.
 */
function showToast(message, type = 'info') {
    const toast = document.getElementById('toast');
    toast.textContent = message;
    toast.className = 'toast ' + type + ' visible';
    
    setTimeout(() => {
        toast.classList.remove('visible');
    }, 3000);
}

/**
 * Escape HTML to prevent XSS.
 */
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

/**
 * Format file size for display.
 */
function formatFileSize(bytes) {
    if (bytes < 1024) return bytes + ' B';
    if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(1) + ' KB';
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB';
}
