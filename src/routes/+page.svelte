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
      savePositions();
      draggingWidget = null;
    }
  }

  function savePositions() {
    localStorage.setItem('desklet_widgets', JSON.stringify(widgets));
  }

  function loadPositions() {
    const saved = localStorage.getItem('desklet_widgets');
    if (saved) {
      widgets = JSON.parse(saved);
    }
  }

  async function updateStats() {
    // Update Clock
    const now = new Date();
    timeStr = now.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
    dateStr = now.toLocaleDateString([], { weekday: 'long', month: 'long', day: 'numeric' });

    // Update CPU/RAM from Rust
    try {
      const stats = await invoke('get_sys_stats');
      cpuUsage = Math.round(stats[0]);
      ramUsage = Math.round(stats[1]);
    } catch (e) {
      console.error(e);
    }
  }

  onMount(() => {
    loadPositions();
    invoke('pin_to_desktop').catch(console.error);
    
    updateStats();
    interval = setInterval(updateStats, 1000);

    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);

    // Press 'S' to toggle Studio Mode
    window.addEventListener('keydown', (e) => {
      if (e.key.toLowerCase() === 's' && e.ctrlKey) {
        isStudioMode = !isStudioMode;
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
    <div class="absolute top-4 left-4 bg-white text-black px-4 py-2 rounded-full font-bold shadow-xl flex items-center gap-2">
      <div class="w-2 h-2 bg-red-500 rounded-full animate-pulse"></div>
      Studio Mode (Drag to move)
    </div>
  {/if}

  {#each widgets as widget (widget.id)}
    <div
      class="absolute p-4 rounded-xl shadow-lg backdrop-blur-md bg-black/40 text-white border border-white/10 {isStudioMode ? 'cursor-grab active:cursor-grabbing' : 'pointer-events-auto'}"
      style="left: {widget.x}px; top: {widget.y}px;"
      on:mousedown={(e) => onMouseDown(e, widget)}
    >
      {#if widget.type === 'clock'}
        <h2 class="text-3xl font-bold font-mono">{timeStr}</h2>
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
        <textarea class="w-48 h-32 bg-white/10 rounded p-2 text-sm resize-none outline-none focus:ring-1 focus:ring-white/50" placeholder="Write something..."></textarea>
      {:else if widget.type === 'pomodoro'}
        <h3 class="font-semibold text-center mb-1">Focus</h3>
        <div class="text-3xl font-mono text-center mb-2">25:00</div>
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
