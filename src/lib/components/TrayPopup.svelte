<script lang="ts">
  import type { SystemSnapshot } from "../types";
  import ProgressBar from "./ui/ProgressBar.svelte";
  import { theme } from "../stores/theme.svelte";
  import { formatBytes, formatRate, formatTemp } from "../utils/format";
  import {
    Activity,
    ArrowDown,
    ArrowUp,
    BatteryCharging,
    ExternalLink,
    HardDrive,
    Moon,
    Sun,
    Zap,
  } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    snapshot: SystemSnapshot;
  }

  let { snapshot }: Props = $props();

  let ramPercent = $derived(() => {
    if (!snapshot.memory.total_bytes) return 0;
    return (snapshot.memory.used_bytes / snapshot.memory.total_bytes) * 100;
  });

  async function openDashboard() {
    try {
      await invoke("open_dashboard_window");
    } catch (e) {
      console.error("Failed to open dashboard:", e);
    }
  }
</script>

<div class="w-full max-w-[340px] mx-auto p-4 select-none font-sans">
  <!-- Header -->
  <div class="flex items-center justify-between pb-3 mb-3 border-b border-slate-200/80 dark:border-slate-800/80">
    <div class="flex items-center gap-2">
      <div class="w-6 h-6 rounded-lg bg-blue-600 flex items-center justify-center text-white font-bold text-xs shadow-sm">
        M
      </div>
      <div>
        <h1 class="text-sm font-bold tracking-tight text-slate-800 dark:text-slate-100">
          MoniQA
        </h1>
        <p class="text-[10px] text-slate-400 font-medium">macOS Resource Pulse</p>
      </div>
    </div>

    <div class="flex items-center gap-1">
      <!-- Theme toggle -->
      <button
        onclick={() => theme.toggle()}
        class="p-1.5 rounded-lg text-slate-500 hover:text-slate-800 dark:hover:text-slate-200 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
        title="Toggle Dark/Light Mode"
      >
        {#if theme.isDark}
          <Sun class="w-4 h-4 text-amber-400" />
        {:else}
          <Moon class="w-4 h-4 text-slate-600" />
        {/if}
      </button>

      <!-- Expand to Dashboard -->
      <button
        onclick={openDashboard}
        class="p-1.5 rounded-lg text-slate-500 hover:text-blue-600 dark:hover:text-blue-400 hover:bg-slate-100 dark:hover:bg-slate-800 transition-colors"
        title="Open Full Dashboard"
      >
        <ExternalLink class="w-4 h-4" />
      </button>
    </div>
  </div>

  <!-- Metric Gauges -->
  <div class="space-y-3">
    <!-- CPU -->
    <div class="bg-white/60 dark:bg-slate-900/50 rounded-xl p-3 border border-slate-200/60 dark:border-slate-800/60">
      <div class="flex justify-between items-baseline mb-1">
        <span class="text-xs font-semibold text-slate-700 dark:text-slate-300 flex items-center gap-1.5">
          <Activity class="w-3.5 h-3.5 text-blue-500" />
          CPU Load
        </span>
        <div class="text-xs font-mono">
          <span class="font-bold text-slate-800 dark:text-white">
            {snapshot.cpu.global_usage.toFixed(1)}%
          </span>
          {#if snapshot.thermal.cpu_temp_c > 0}
            <span class="text-slate-400 text-[11px] ml-1">
              ({formatTemp(snapshot.thermal.cpu_temp_c)})
            </span>
          {/if}
        </div>
      </div>
      <ProgressBar value={snapshot.cpu.global_usage} height="h-2" />
    </div>

    <!-- RAM -->
    <div class="bg-white/60 dark:bg-slate-900/50 rounded-xl p-3 border border-slate-200/60 dark:border-slate-800/60">
      <div class="flex justify-between items-baseline mb-1">
        <span class="text-xs font-semibold text-slate-700 dark:text-slate-300 flex items-center gap-1.5">
          <HardDrive class="w-3.5 h-3.5 text-purple-500" />
          Memory
        </span>
        <span class="text-xs font-mono font-bold text-slate-800 dark:text-white">
          {formatBytes(snapshot.memory.used_bytes)} / {formatBytes(snapshot.memory.total_bytes)}
        </span>
      </div>
      <ProgressBar value={ramPercent()} height="h-2" color="bg-purple-500" />
    </div>

    <!-- GPU -->
    <div class="bg-white/60 dark:bg-slate-900/50 rounded-xl p-3 border border-slate-200/60 dark:border-slate-800/60">
      <div class="flex justify-between items-baseline mb-1">
        <span class="text-xs font-semibold text-slate-700 dark:text-slate-300 flex items-center gap-1.5">
          <Zap class="w-3.5 h-3.5 text-amber-500" />
          GPU
        </span>
        <div class="text-xs font-mono">
          <span class="font-bold text-slate-800 dark:text-white">
            {snapshot.gpu.usage_percent.toFixed(1)}%
          </span>
          <span class="text-slate-400 text-[11px] ml-1">
            ({snapshot.gpu.power_watts.toFixed(1)} W)
          </span>
        </div>
      </div>
      <ProgressBar value={snapshot.gpu.usage_percent} height="h-2" color="bg-amber-500" />
    </div>

    <!-- Quick Stats Grid (Network & Battery) -->
    <div class="grid grid-cols-2 gap-2 text-xs">
      <!-- Network throughput -->
      <div class="bg-slate-50/80 dark:bg-slate-800/40 rounded-xl p-2.5 border border-slate-100 dark:border-slate-800/50">
        <div class="flex items-center gap-1 text-[10px] text-slate-400 uppercase font-semibold mb-1">
          <span>Traffic</span>
        </div>
        <div class="space-y-1 font-mono text-[11px]">
          <div class="flex items-center justify-between text-emerald-600 dark:text-emerald-400 font-medium">
            <span class="flex items-center gap-0.5"><ArrowDown class="w-3 h-3" /> In</span>
            <span>{formatRate(snapshot.network.total_received_bytes_per_sec)}</span>
          </div>
          <div class="flex items-center justify-between text-sky-600 dark:text-sky-400 font-medium">
            <span class="flex items-center gap-0.5"><ArrowUp class="w-3 h-3" /> Out</span>
            <span>{formatRate(snapshot.network.total_transmitted_bytes_per_sec)}</span>
          </div>
        </div>
      </div>

      <!-- Battery -->
      <div class="bg-slate-50/80 dark:bg-slate-800/40 rounded-xl p-2.5 border border-slate-100 dark:border-slate-800/50 flex flex-col justify-between">
        <div class="text-[10px] text-slate-400 uppercase font-semibold">Battery</div>
        <div class="flex items-baseline justify-between mt-1">
          <span class="text-base font-bold font-mono text-slate-800 dark:text-white">
            {snapshot.battery.charge_percent.toFixed(0)}%
          </span>
          {#if snapshot.battery.is_charging}
            <span class="text-[10px] text-emerald-500 font-semibold flex items-center gap-0.5">
              <BatteryCharging class="w-3 h-3 animate-pulse" /> Sạc
            </span>
          {/if}
        </div>
        <ProgressBar value={snapshot.battery.charge_percent} height="h-1.5" />
      </div>
    </div>
  </div>

  <!-- Open Full Dashboard Button -->
  <button
    onclick={openDashboard}
    class="w-full mt-4 py-2 px-3 bg-blue-600 hover:bg-blue-700 active:bg-blue-800 text-white rounded-xl text-xs font-semibold flex items-center justify-center gap-1.5 shadow-sm transition-all duration-150"
  >
    <span>Open Full Dashboard</span>
    <ExternalLink class="w-3.5 h-3.5" />
  </button>
</div>
