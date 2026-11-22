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

  let ramPercent = $derived(
    snapshot.memory.total_bytes > 0
      ? (snapshot.memory.used_bytes / snapshot.memory.total_bytes) * 100
      : 0
  );

  async function openDashboard() {
    try {
      await invoke("open_dashboard_window");
    } catch (e) {
      console.error("Failed to open dashboard:", e);
    }
  }
</script>

<div class="w-full max-w-[340px] mx-auto p-4 select-none font-sans bg-slate-50/90 dark:bg-slate-900/95 backdrop-blur-xl border border-slate-200/80 dark:border-slate-800/80 rounded-2xl shadow-xl">
  <!-- Telemetry Header -->
  <div class="flex items-center justify-between pb-3 mb-3 border-b border-slate-200/80 dark:border-slate-800/80">
    <div class="flex items-center gap-2.5">
      <div class="relative w-7 h-7 rounded bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center text-white font-black text-xs shadow-md shadow-cyan-500/20">
        M
        <span class="absolute -bottom-0.5 -right-0.5 w-2 h-2 rounded-full bg-emerald-400 ring-2 ring-white dark:ring-slate-900"></span>
      </div>
      <div>
        <div class="flex items-center gap-1.5">
          <h1 class="text-xs font-black tracking-widest text-slate-900 dark:text-white uppercase">
            MoniQA
          </h1>
          <span class="text-[9px] font-mono px-1 py-0.2 rounded bg-cyan-500/10 text-cyan-600 dark:text-cyan-400 border border-cyan-500/20 font-bold">
            3s
          </span>
        </div>
        <p class="text-[9px] font-mono text-slate-400">TELEMETRY TRAY DECK</p>
      </div>
    </div>

    <!-- Controls (Tactile Theme Switch + Expand Dashboard) -->
    <div class="flex items-center gap-1.5">
      <!-- Tactile Physical-style Theme Switch -->
      <button
        onclick={() => theme.toggle()}
        class="flex items-center gap-1 px-2 py-1 rounded bg-slate-200/80 dark:bg-slate-800 hover:bg-slate-300 dark:hover:bg-slate-700/80 border border-slate-300/80 dark:border-slate-700/80 transition-all duration-150 active:scale-95 text-slate-700 dark:text-slate-200"
        title="Toggle Light / Dark Mode"
      >
        {#if theme.isDark}
          <Sun class="w-3.5 h-3.5 text-amber-400" />
          <span class="w-1.5 h-1.5 rounded-full bg-amber-400 shadow-[0_0_6px_rgba(251,191,36,0.8)]"></span>
        {:else}
          <Moon class="w-3.5 h-3.5 text-blue-600" />
          <span class="w-1.5 h-1.5 rounded-full bg-blue-600 shadow-[0_0_6px_rgba(37,99,235,0.8)]"></span>
        {/if}
      </button>

      <!-- Expand to Dashboard -->
      <button
        onclick={openDashboard}
        class="p-1.5 rounded bg-slate-200/80 dark:bg-slate-800 hover:bg-slate-300 dark:hover:bg-slate-700/80 border border-slate-300/80 dark:border-slate-700/80 text-slate-600 hover:text-cyan-600 dark:text-slate-300 dark:hover:text-cyan-400 transition-all duration-150 active:scale-95"
        title="Open Full Deck"
      >
        <ExternalLink class="w-3.5 h-3.5" />
      </button>
    </div>
  </div>

  <!-- Metric Gauges Array -->
  <div class="space-y-2.5">
    <!-- CPU -->
    <div class="bg-white/80 dark:bg-slate-950/60 rounded p-2.5 border border-slate-200/70 dark:border-slate-800/80">
      <div class="flex justify-between items-baseline mb-1">
        <span class="text-[10px] font-mono font-bold text-slate-600 dark:text-slate-300 flex items-center gap-1.5 uppercase">
          <Activity class="w-3 h-3 text-cyan-500" />
          CPU LOAD
        </span>
        <div class="text-xs font-mono font-tabular">
          <span class="font-black text-slate-900 dark:text-white">
            {snapshot.cpu.global_usage.toFixed(1)}%
          </span>
          {#if snapshot.thermal.cpu_temp_c > 0}
            <span class="text-slate-400 text-[10px] ml-1">
              [{formatTemp(snapshot.thermal.cpu_temp_c)}]
            </span>
          {/if}
        </div>
      </div>
      <ProgressBar value={snapshot.cpu.global_usage} height="h-2.5" segments={20} />
    </div>

    <!-- RAM -->
    <div class="bg-white/80 dark:bg-slate-950/60 rounded p-2.5 border border-slate-200/70 dark:border-slate-800/80">
      <div class="flex justify-between items-baseline mb-1">
        <span class="text-[10px] font-mono font-bold text-slate-600 dark:text-slate-300 flex items-center gap-1.5 uppercase">
          <HardDrive class="w-3 h-3 text-purple-500" />
          RAM ACTIVE
        </span>
        <span class="text-[11px] font-mono font-tabular font-bold text-slate-900 dark:text-white">
          {formatBytes(snapshot.memory.used_bytes)} / {formatBytes(snapshot.memory.total_bytes)}
        </span>
      </div>
      <ProgressBar
        value={ramPercent}
        height="h-2.5"
        segments={20}
        color="bg-purple-500 dark:bg-purple-400 shadow-[0_0_6px_rgba(168,85,247,0.6)]"
      />
    </div>

    <!-- GPU -->
    <div class="bg-white/80 dark:bg-slate-950/60 rounded p-2.5 border border-slate-200/70 dark:border-slate-800/80">
      <div class="flex justify-between items-baseline mb-1">
        <span class="text-[10px] font-mono font-bold text-slate-600 dark:text-slate-300 flex items-center gap-1.5 uppercase">
          <Zap class="w-3 h-3 text-amber-500" />
          GPU COMPUTE
        </span>
        <div class="text-xs font-mono font-tabular">
          <span class="font-black text-slate-900 dark:text-white">
            {snapshot.gpu.usage_percent.toFixed(1)}%
          </span>
          <span class="text-slate-400 text-[10px] ml-1">
            [{snapshot.gpu.power_watts.toFixed(1)} W]
          </span>
        </div>
      </div>
      <ProgressBar
        value={snapshot.gpu.usage_percent}
        height="h-2.5"
        segments={20}
        color="bg-amber-500 dark:bg-amber-400 shadow-[0_0_6px_rgba(245,158,11,0.6)]"
      />
    </div>

    <!-- Quick Stats Grid (Network & Battery) -->
    <div class="grid grid-cols-2 gap-2 text-xs">
      <!-- Network Throughput -->
      <div class="bg-white/80 dark:bg-slate-950/60 rounded p-2 border border-slate-200/70 dark:border-slate-800/80">
        <div class="text-[9px] font-mono font-bold text-slate-400 uppercase mb-1">BUS I/O</div>
        <div class="space-y-0.5 font-mono font-tabular text-[10px]">
          <div class="flex items-center justify-between text-emerald-600 dark:text-emerald-400 font-bold">
            <span class="flex items-center gap-0.5"><ArrowDown class="w-2.5 h-2.5" /> DL</span>
            <span>{formatRate(snapshot.network.total_received_bytes_per_sec)}</span>
          </div>
          <div class="flex items-center justify-between text-sky-600 dark:text-sky-400 font-bold">
            <span class="flex items-center gap-0.5"><ArrowUp class="w-2.5 h-2.5" /> UL</span>
            <span>{formatRate(snapshot.network.total_transmitted_bytes_per_sec)}</span>
          </div>
        </div>
      </div>

      <!-- Battery -->
      <div class="bg-white/80 dark:bg-slate-950/60 rounded p-2 border border-slate-200/70 dark:border-slate-800/80 flex flex-col justify-between">
        <div class="flex justify-between items-center text-[9px] font-mono font-bold text-slate-400 uppercase">
          <span>CELL PWR</span>
          {#if snapshot.battery.is_charging}
            <span class="text-emerald-500 font-bold flex items-center gap-0.5">
              <BatteryCharging class="w-2.5 h-2.5 animate-pulse" /> CHG
            </span>
          {/if}
        </div>
        <div class="text-sm font-black font-mono font-tabular text-slate-900 dark:text-white my-0.5">
          {snapshot.battery.charge_percent.toFixed(0)}%
        </div>
        <ProgressBar
          value={snapshot.battery.charge_percent}
          height="h-2"
          segments={12}
          color={snapshot.battery.is_charging ? "bg-emerald-500" : undefined}
        />
      </div>
    </div>
  </div>

  <!-- Open Full Dashboard Button -->
  <button
    onclick={openDashboard}
    class="w-full mt-3 py-2 px-3 bg-gradient-to-r from-blue-600 to-cyan-600 hover:from-blue-500 hover:to-cyan-500 active:scale-[0.98] text-white rounded font-mono font-bold text-xs flex items-center justify-center gap-2 shadow-md shadow-blue-500/20 transition-all duration-150 uppercase tracking-wider"
  >
    <span>Open Telemetry Deck</span>
    <ExternalLink class="w-3.5 h-3.5" />
  </button>
</div>
