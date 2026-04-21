<template>
  <div id="noise"></div>
  <div class="glow-bg"></div>

  <div class="container">
    <header>
      <div class="logo-container">
        <div class="logo">⚡</div>
      </div>
      <h1>{{PROJECT_NAME}}</h1>
      <p class="tagline">Welcome to the future of Desktop Apps mapped with Python.</p>
    </header>

    <main>
      <section class="card glass">
        <div class="card-header">
          <div class="dot-group">
            <span class="dot red"></span>
            <span class="dot yellow"></span>
            <span class="dot green"></span>
          </div>
          <h2>Command Execution</h2>
        </div>
        <div class="card-body">
          <p class="desc">Enter a name to call the <code>greet</code> python command via IPC.</p>
          <div class="input-group">
            <input
              type="text"
              v-model="name"
              placeholder="Enter your name..."
              autocomplete="off"
            />
            <button @click="handleGreet" :disabled="loadingGreet">
              {{ loadingGreet ? 'Running...' : 'Run' }}
            </button>
          </div>
          <div v-if="greeting" class="output-box success show">{{ greeting }}</div>
          <div v-if="errorGreet" class="output-box error show">{{ errorGreet }}</div>
        </div>
      </section>

      <section class="card glass">
        <div class="card-header">
          <div class="dot-group">
            <span class="dot red"></span>
            <span class="dot yellow"></span>
            <span class="dot green"></span>
          </div>
          <h2>System Telemetry</h2>
        </div>
        <div class="card-body">
          <p class="desc">Fetching live diagnostics seamlessly from Python.</p>
          <button class="secondary-btn" @click="handleGetInfo" :disabled="loadingInfo">
            {{ loadingInfo ? 'Fetching...' : 'Fetch System Info' }}
          </button>
          <div v-if="systemInfo" class="output-box info show">
            {{ JSON.stringify(systemInfo, null, 2) }}
          </div>
          <div v-if="errorInfo" class="output-box error show">{{ errorInfo }}</div>
        </div>
      </section>
    </main>

    <footer>
      <p>Powered by <strong>Forge Framework</strong></p>
    </footer>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import forge, { invoke } from '@forgedesk/api'

const name = ref('')
const greeting = ref('')
const systemInfo = ref(null)
const loadingGreet = ref(false)
const loadingInfo = ref(false)
const errorGreet = ref(null)
const errorInfo = ref(null)

const handleGreet = async () => {
  loadingGreet.value = true
  errorGreet.value = null
  greeting.value = ''
  try {
    const result = await invoke('greet', { name: name.value || 'Developer' })
    greeting.value = result
    try { await forge.clipboard.write(result); } catch(e) {}
  } catch (err) {
    errorGreet.value = err.message || String(err)
  } finally {
    loadingGreet.value = false
  }
}

const handleGetInfo = async () => {
  loadingInfo.value = true
  errorInfo.value = null
  systemInfo.value = null
  try {
    const info = await invoke('get_system_info')
    systemInfo.value = info
  } catch (err) {
    errorInfo.value = err.message || String(err)
  } finally {
    loadingInfo.value = false
  }
}
</script>

<style>
/* Modern Forge Vue Template */
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');

:root {
  --bg-base: #09090b;
  --bg-overlay: rgba(24, 24, 27, 0.6);
  --text-main: #fafafa;
  --text-muted: #a1a1aa;
  --accent: #3b82f6;
  --accent-glow: rgba(59, 130, 246, 0.3);
  --accent-hover: #60a5fa;
  --border-color: rgba(255, 255, 255, 0.1);
  --card-shadow: 0 4px 30px rgba(0, 0, 0, 0.5);
  --font-sans: 'Inter', system-ui, -apple-system, sans-serif;
  --font-mono: 'ui-monospace', 'SFMono-Regular', 'Menlo', monospace;
  --success: #10b981;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: var(--font-sans);
  background-color: var(--bg-base);
  color: var(--text-main);
  min-height: 100vh;
  display: flex;
  justify-content: center;
  overflow-x: hidden;
  position: relative;
  line-height: 1.5;
}

#noise {
  position: fixed;
  top: 0; left: 0; width: 100vw; height: 100vh;
  pointer-events: none;
  z-index: 999;
  opacity: 0.04;
  background: url('data:image/svg+xml;utf8,%3Csvg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg"%3E%3Cfilter id="noiseFilter"%3E%3CfeTurbulence type="fractalNoise" baseFrequency="0.65" numOctaves="3" stitchTiles="stitch"/%3E%3C/filter%3E%3Crect width="100%25" height="100%25" filter="url(%23noiseFilter)"/%3E%3C/svg%3E');
}

.glow-bg {
  position: fixed;
  top: -20%;
  left: 20%;
  width: 60vw;
  height: 60vh;
  background: radial-gradient(circle, var(--accent-glow) 0%, transparent 60%);
  filter: blur(100px);
  z-index: 0;
  pointer-events: none;
}

.container {
  width: 100%;
  max-width: 800px;
  padding: 3rem 1.5rem;
  position: relative;
  z-index: 1;
}

header {
  text-align: center;
  margin-bottom: 4rem;
  animation: fadeDown 0.8s ease-out;
}

.logo-container {
  width: 64px;
  height: 64px;
  margin: 0 auto 1.5rem;
  border-radius: 20px;
  background: linear-gradient(135deg, rgba(255,255,255,0.1), rgba(255,255,255,0));
  border: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
  backdrop-filter: blur(10px);
}

.logo {
  font-size: 2rem;
  filter: drop-shadow(0 0 8px var(--accent-glow));
}

header h1 {
  font-size: 3rem;
  font-weight: 700;
  letter-spacing: -0.04em;
  background: linear-gradient(135deg, #fff 0%, #a1a1aa 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  margin-bottom: 0.75rem;
}

.tagline {
  color: var(--text-muted);
  font-size: 1.1rem;
  font-weight: 400;
}

main {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.card.glass {
  background: var(--bg-overlay);
  border: 1px solid var(--border-color);
  border-radius: 16px;
  overflow: hidden;
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  box-shadow: var(--card-shadow);
  transition: transform 0.3s ease, border-color 0.3s ease;
  animation: fadeUp 0.8s ease-out backwards;
}

.card.glass:nth-child(2) {
  animation-delay: 0.15s;
}

.card.glass:hover {
  border-color: rgba(255, 255, 255, 0.2);
  transform: translateY(-2px);
}

.card-header {
  background: rgba(0, 0, 0, 0.2);
  padding: 1rem 1.5rem;
  display: flex;
  align-items: center;
  border-bottom: 1px solid var(--border-color);
}

.dot-group {
  display: flex;
  gap: 6px;
  margin-right: 12px;
}

.dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
}
.dot.red { background-color: #ff5f56; }
.dot.yellow { background-color: #ffbd2e; }
.dot.green { background-color: #27c93f; }

.card-header h2 {
  font-size: 0.95rem;
  font-weight: 500;
  color: #e4e4e7;
  letter-spacing: 0.02em;
}

.card-body {
  padding: 1.5rem;
}

.desc {
  color: var(--text-muted);
  font-size: 0.95rem;
  margin-bottom: 1.5rem;
}

.input-group {
  display: flex;
  gap: 0.75rem;
  margin-bottom: 1.5rem;
}

input[type="text"] {
  flex: 1;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--border-color);
  color: var(--text-main);
  padding: 0.75rem 1rem;
  border-radius: 8px;
  font-family: var(--font-sans);
  font-size: 0.95rem;
  outline: none;
  transition: all 0.2s ease;
}

input[type="text"]:focus {
  border-color: var(--accent);
  box-shadow: 0 0 0 3px var(--accent-glow);
}

button {
  background: var(--accent);
  color: #fff;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  font-weight: 600;
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.2s ease;
  font-family: var(--font-sans);
  box-shadow: 0 4px 12px var(--accent-glow);
}

button:hover:not(:disabled) {
  background: var(--accent-hover);
  transform: translateY(-1px);
}

button:active:not(:disabled) {
  transform: translateY(1px);
}

button:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.secondary-btn {
  background: rgba(255, 255, 255, 0.1);
  box-shadow: none;
  border: 1px solid var(--border-color);
  margin-bottom: 1.5rem;
}

.secondary-btn:hover:not(:disabled) {
  background: rgba(255, 255, 255, 0.15);
}

.output-box {
  background: rgba(0, 0, 0, 0.4);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 1rem;
  font-family: var(--font-mono);
  font-size: 0.85rem;
  min-height: 3.5rem;
  white-space: pre-wrap;
  word-break: break-word;
  display: none;
}

.output-box.show {
  display: block;
  animation: fadeIn 0.3s ease;
}

.output-box.success { color: var(--success); }
.output-box.error { color: #ff5f56; }
.output-box.info { color: var(--accent); }

footer {
  margin-top: 4rem;
  text-align: center;
  color: var(--text-muted);
  font-size: 0.9rem;
  padding-bottom: 2rem;
}

footer strong {
  color: var(--text-main);
  font-weight: 500;
}

@keyframes fadeDown {
  from { opacity: 0; transform: translateY(-20px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes fadeUp {
  from { opacity: 0; transform: translateY(20px); }
  to { opacity: 1; transform: translateY(0); }
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}
</style>
