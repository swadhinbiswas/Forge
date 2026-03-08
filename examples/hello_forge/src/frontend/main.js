document.addEventListener('DOMContentLoaded', () => {
    const sysInfoContent = document.getElementById('sys-info-content');
    const runTestBtn = document.getElementById('run-test');
    const loader = document.getElementById('loader');
    const resultBox = document.getElementById('result-box');
    const execTime = document.getElementById('exec-time');
    const resultVal = document.getElementById('result-val');

    /**
     * Load initial system info using the native bridge
     */
    async function loadSystemInfo() {
        try {
            const info = await window.__forge__.invoke('get_system_info');
            sysInfoContent.innerHTML = `
                <ul class="info-list">
                    <li><strong>OS:</strong> ${info.os}</li>
                    <li><strong>Engine:</strong> ${info.engine}</li>
                    <li><strong>Bridge:</strong> ${info.bridge}</li>
                    <li><strong>Python:</strong> ${info.python_version}</li>
                </ul>
            `;
        } catch (err) {
            sysInfoContent.innerText = `Error: ${err}`;
        }
    }

    /**
     * Execute the performance-heavy Fibonacci test
     */
    async function runPerformanceTest() {
        // Toggle UI state
        runTestBtn.disabled = true;
        loader.classList.remove('hidden');
        resultBox.classList.add('hidden');

        try {
            const start = performance.now();
            
            // Invoke the heavy Python command
            const data = await window.__forge__.invoke('calculate_fibonacci', { n: 100000 });
            
            const totalRoundTrip = performance.now() - start;

            // Display results
            execTime.innerText = `${data.execution_time_ms.toFixed(2)}ms (Python) / ${totalRoundTrip.toFixed(2)}ms (Total)`;
            resultVal.innerText = data.result;
            resultBox.classList.remove('hidden');
        } catch (err) {
            alert(`Test failed: ${err}`);
        } finally {
            loader.classList.add('hidden');
            runTestBtn.disabled = false;
        }
    }

    runTestBtn.addEventListener('click', runPerformanceTest);
    
    // Initial load
    setTimeout(loadSystemInfo, 500);
});
