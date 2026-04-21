import forge, { invoke } from '@forgedesk/api'

document.addEventListener('DOMContentLoaded', () => {
    const nameInput = document.getElementById('nameInput');
    const greetBtn = document.getElementById('greetBtn');
    const greetResult = document.getElementById('greetResult');
    
    const sysInfoBtn = document.getElementById('sysInfoBtn');
    const sysInfoResult = document.getElementById('sysInfoResult');

    // Greet logic
    greetBtn.addEventListener('click', async () => {
        const name = nameInput.value.trim() || 'Developer';
        greetBtn.innerText = 'Running...';
        greetBtn.style.opacity = '0.7';
        
        try {
            const result = await invoke('greet', { name });
            greetResult.innerText = result;
            greetResult.classList.add('show');
            greetResult.style.color = 'var(--success)';
            
            // Try clipboard
            try { await forge.clipboard.write(result); } catch (e) {}
        } catch (err) {
            greetResult.innerText = `Error: ${err.message || err}`;
            greetResult.classList.add('show');
            greetResult.style.color = '#ff5f56';
        } finally {
            greetBtn.innerText = 'Run';
            greetBtn.style.opacity = '1';
        }
    });

    // System info logic
    sysInfoBtn.addEventListener('click', async () => {
        sysInfoBtn.innerText = 'Fetching...';
        try {
            const info = await invoke('get_system_info');
            sysInfoResult.innerText = JSON.stringify(info, null, 2);
            sysInfoResult.classList.add('show');
            sysInfoResult.style.color = 'var(--accent)';
        } catch (err) {
            sysInfoResult.innerText = `Error: ${err.message || err}`;
            sysInfoResult.classList.add('show');
            sysInfoResult.style.color = '#ff5f56';
        } finally {
            sysInfoBtn.innerText = 'Fetch System Info';
        }
    });
});
