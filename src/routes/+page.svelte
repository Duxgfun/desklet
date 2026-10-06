<script>
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';

  // Sample widget data
  let widgets = [
    { id: 1, type: 'clock', x: 100, y: 100 },
    { id: 2, type: 'weather', x: 400, y: 100 },
    { id: 3, type: 'notes', x: 100, y: 300 },
    { id: 4, type: 'pomodoro', x: 400, y: 300 },
    { id: 5, type: 'monitor', x: 700, y: 100 },
  ];

  // Theme data (Styling Engine)
  let theme = {
    bgColor: '#000000',
    bgOpacity: 0.4,
    blur: 12, // px
    borderRadius: 12, // px
    borderOpacity: 0.1,
    textColor: '#ffffff',
    fontFamily: 'ui-sans-serif, system-ui, sans-serif'
  };

  let isStudioMode = false;
  let draggingWidget = null;
  let offsetX = 0;
  let offsetY = 0;

  // Real-time states
  let timeStr = "12:00 PM";
  let dateStr = "Loading...";
  let cpuUsage = 0;
  let ramUsage = 0;
  let interval;

  function onMouseDown(e, widget) {
    if (!isStudioMode) return;
    draggingWidget = widget;
    offsetX = e.clientX - widget.x;
    offsetY = e.clientY - widget.y;
  }

  function onMouseMove(e) {
    if (draggingWidget && isStudioMode) {
      let rawX = e.clientX - offsetX;
      let rawY = e.clientY - offsetY;
      draggingWidget.x = Math.round(rawX / 8) * 8;
      draggingWidget.y = Math.round(rawY / 8) * 8;
      widgets = [...widgets];
    }
  }

  function onMouseUp() {
    if (draggingWidget) {
      saveData();
      draggingWidget = null;
    }
  }

  function saveData() {
    localStorage.setItem('desklet_widgets', JSON.stringify(widgets));
    localStorage.setItem('desklet_theme', JSON.stringify(theme));
  }

  function loadData() {
    const savedWidgets = localStorage.getItem('desklet_widgets');
    if (savedWidgets) widgets = JSON.parse(savedWidgets);
    
    const savedTheme = localStorage.getItem('desklet_theme');
    if (savedTheme) theme = { ...theme, ...JSON.parse(savedTheme) };
  }

  // Convert hex to rgb for opacity mixing
  function hexToRgb(hex) {
    var result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
    return result ? `${parseInt(result[1], 16)}, ${parseInt(result[2], 16)}, ${parseInt(result[3], 16)}` : '0, 0, 0';
  }

  async function updateStats() {
    const now = new Date();
    timeStr = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    dateStr = now.toLocaleDateString([], { weekday: 'long', month: 'long', day: 'numeric' });

    try {
      const stats = await invoke('get_sys_stats');
      cpuUsage = Math.round(stats[0]);
      ramUsage = Math.round(stats[1]);
    } catch (e) {
      // Ignore when running outside Tauri
    }
  }

  onMount(() => {
    loadData();
    invoke('pin_to_desktop').catch(console.error);
    
    updateStats();
    interval = setInterval(updateStats, 1000);

    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
    
    window.addEventListener('keydown', (e) => {
      if (e.key.toLowerCase() === 's' && e.ctrlKey) {
        isStudioMode = !isStudioMode;
        if (!isStudioMode) saveData(); // Save when exiting Studio Mode
      }
    });

    return () => {
      clearInterval(interval);
      window.removeEventListener('mousemove', onMouseMove);
      window.removeEventListener('mouseup', onMouseUp);
    };
  });
</script>

<main class="w-screen h-screen overflow-hidden bg-transparent {isStudioMode ? 'pointer-events-auto bg-black/20' : 'pointer-events-none'}">
  {#if isStudioMode}
    <!-- Studio Top Bar -->
    <div class="absolute top-4 left-4 bg-white text-black px-4 py-2 rounded-full font-bold shadow-xl flex items-center gap-2 z-50">
      <div class="w-2 h-2 bg-red-500 rounded-full animate-pulse"></div>
      Studio Mode (Drag to move, Ctrl+S to save/exit)
    </div>

    <!-- Styling Sidebar -->
    <div class="absolute right-4 top-4 w-80 bg-white text-black p-6 rounded-2xl shadow-2xl z-50 overflow-y-auto max-h-[90vh]">
      <h3 class="font-bold text-xl mb-4 border-b pb-2">Styling Engine</h3>
      
      <div class="space-y-4">
        <div>
          <label class="block text-sm font-semibold mb-1">Background Color</label>
          <div class="flex gap-2 items-center">
            <input type="color" bind:value={theme.bgColor} on:change={saveData} class="w-10 h-10 rounded cursor-pointer border-0 p-0" />
            <span class="text-sm font-mono">{theme.bgColor}</span>
          </div>
        </div>
        
        <div>
          <label class="block text-sm font-semibold mb-1">Background Opacity: {Math.round(theme.bgOpacity * 100)}%</label>
          <input type="range" bind:value={theme.bgOpacity} on:change={saveData} min="0" max="1" step="0.05" class="w-full accent-black" />
        </div>

        <div>
          <label class="block text-sm font-semibold mb-1">Glassmorphism Blur: {theme.blur}px</label>
          <input type="range" bind:value={theme.blur} on:change={saveData} min="0" max="40" step="1" class="w-full accent-black" />
        </div>

        <div>
          <label class="block text-sm font-semibold mb-1">Border Radius: {theme.borderRadius}px</label>
          <input type="range" bind:value={theme.borderRadius} on:change={saveData} min="0" max="100" step="1" class="w-full accent-black" />
        </div>

        <div>
          <label class="block text-sm font-semibold mb-1">Border Opacity: {Math.round(theme.borderOpacity * 100)}%</label>
          <input type="range" bind:value={theme.borderOpacity} on:change={saveData} min="0" max="1" step="0.05" class="w-full accent-black" />
        </div>

        <div>
          <label class="block text-sm font-semibold mb-1">Text Color</label>
          <div class="flex gap-2 items-center">
            <input type="color" bind:value={theme.textColor} on:change={saveData} class="w-10 h-10 rounded cursor-pointer border-0 p-0" />
          </div>
        </div>

        <div>
          <label class="block text-sm font-semibold mb-1">Font Family</label>
          <select bind:value={theme.fontFamily} on:change={saveData} class="w-full p-2 border rounded text-sm">
            <option value="ui-sans-serif, system-ui, sans-serif">System Sans</option>
            <option value="ui-serif, Georgia, serif">System Serif</option>
            <option value="ui-monospace, SFMono-Regular, monospace">Monospace</option>
            <option value="Arial, sans-serif">Arial</option>
            <option value="Inter, sans-serif">Inter</option>
          </select>
        </div>
      </div>
    </div>
  {/if}

  {#each widgets as widget (widget.id)}
    <div
      class="absolute p-4 shadow-lg {isStudioMode ? 'cursor-grab active:cursor-grabbing' : 'pointer-events-auto'}"
      style="
        left: {widget.x}px; 
        top: {widget.y}px;
        background-color: rgba({hexToRgb(theme.bgColor)}, {theme.bgOpacity});
        backdrop-filter: blur({theme.blur}px);
        -webkit-backdrop-filter: blur({theme.blur}px);
        border-radius: {theme.borderRadius}px;
        border: 1px solid rgba(255,255,255, {theme.borderOpacity});
        color: {theme.textColor};
        font-family: {theme.fontFamily};
      "
      on:mousedown={(e) => onMouseDown(e, widget)}
    >
      {#if widget.type === 'clock'}
        <h2 class="text-3xl font-bold font-mono" style="font-family: inherit;">{timeStr}</h2>
        <p class="text-sm opacity-70">{dateStr}</p>
      {:else if widget.type === 'weather'}
        <div class="flex items-center gap-2">
          <span class="text-2xl">⛅</span>
          <div>
            <h2 class="text-xl font-semibold">24°C</h2>
            <p class="text-xs opacity-70">Hanoi, AQI: 50</p>
          </div>
        </div>
      {:else if widget.type === 'notes'}
        <h3 class="font-semibold mb-2">Quick Notes</h3>
        <textarea 
          class="w-48 h-32 rounded p-2 text-sm resize-none outline-none focus:ring-1 focus:ring-white/50" 
          style="background-color: rgba(255,255,255,0.1); color: inherit;"
          placeholder="Write something..."></textarea>
      {:else if widget.type === 'pomodoro'}
        <h3 class="font-semibold text-center mb-1">Focus</h3>
        <div class="text-3xl font-mono text-center mb-2" style="font-family: inherit;">25:00</div>
        <div class="flex gap-2 justify-center">
          <button class="bg-white/20 hover:bg-white/30 px-3 py-1 rounded text-xs transition-colors">Start</button>
          <button class="bg-white/20 hover:bg-white/30 px-3 py-1 rounded text-xs transition-colors">Reset</button>
        </div>
      {:else if widget.type === 'monitor'}
        <h3 class="font-semibold mb-2">System</h3>
        <div class="text-xs space-y-1 w-32">
          <div class="flex justify-between gap-4"><span>CPU</span> <span>{cpuUsage}%</span></div>
          <div class="w-full bg-white/20 h-1.5 rounded overflow-hidden"><div class="bg-blue-400 h-full rounded transition-all duration-1000" style="width: {cpuUsage}%"></div></div>
          
          <div class="flex justify-between gap-4 mt-2"><span>RAM</span> <span>{ramUsage}%</span></div>
          <div class="w-full bg-white/20 h-1.5 rounded overflow-hidden"><div class="bg-green-400 h-full rounded transition-all duration-1000" style="width: {ramUsage}%"></div></div>
        </div>
      {/if}
    </div>
  {/each}
</main>
