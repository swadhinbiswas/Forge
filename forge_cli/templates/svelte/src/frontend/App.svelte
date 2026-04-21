<script>
  import { onMount } from 'svelte';
  import forge, { invoke } from '@forgedesk/api';
  import './style.css';

  let name = '';
  let greeting = '';
  let systemInfo = null;
  let loadingGreet = false;
  let loadingInfo = false;
  let errorGreet = null;
  let errorInfo = null;

  async function handleGreet() {
    loadingGreet = true;
    errorGreet = null;
    greeting = '';
    try {
      const result = await invoke('greet', { name: name || 'Developer' });
      greeting = result;
      try { await forge.clipboard.write(result); } catch(e) {}
    } catch (err) {
      errorGreet = err.message || String(err);
    } finally {
      loadingGreet = false;
    }
  }

  async function handleGetInfo() {
    loadingInfo = true;
    errorInfo = null;
    systemInfo = null;
    try {
      const info = await invoke('get_system_info');
      systemInfo = info;
    } catch (err) {
      errorInfo = err.message || String(err);
    } finally {
      loadingInfo = false;
    }
  }
</script>

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
            bind:value={name}
            placeholder="Enter your name..."
            autocomplete="off"
          />
          <button on:click={handleGreet} disabled={loadingGreet}>
            {#if loadingGreet}Running...{:else}Run{/if}
          </button>
        </div>
        {#if greeting}
          <div class="output-box success show">{greeting}</div>
        {/if}
        {#if errorGreet}
          <div class="output-box error show">{errorGreet}</div>
        {/if}
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
        <button class="secondary-btn" on:click={handleGetInfo} disabled={loadingInfo}>
          {#if loadingInfo}Fetching...{:else}Fetch System Info{/if}
        </button>
        {#if systemInfo}
          <div class="output-box info show">
            {JSON.stringify(systemInfo, null, 2)}
          </div>
        {/if}
        {#if errorInfo}
          <div class="output-box error show">{errorInfo}</div>
        {/if}
      </div>
    </section>
  </main>

  <footer>
    <p>Powered by <strong>Forge Framework</strong></p>
  </footer>
</div>
