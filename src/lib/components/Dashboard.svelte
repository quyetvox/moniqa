<script lang="ts">
  import type { SystemSnapshot } from "../types";
  import CpuCard from "./CpuCard.svelte";
  import MemoryCard from "./MemoryCard.svelte";
  import GpuCard from "./GpuCard.svelte";
  import ThermalCard from "./ThermalCard.svelte";
  import DiskNetworkCard from "./DiskNetworkCard.svelte";
  import BatteryCard from "./BatteryCard.svelte";
  import ProcessTable from "./ProcessTable.svelte";
  import SystemInfoCard from "./SystemInfoCard.svelte";
  import { theme } from "../stores/theme.svelte";
  import { formatUptime } from "../utils/format";
  import { Activity, Clock, Moon, Sun, Terminal } from "@lucide/svelte";

  interface Props {
    snapshot: SystemSnapshot;
  }

  let { snapshot }: Props = $props();
</script>

<div class="min-h-screen p-6 space-y-6 max-w-7xl mx-auto font-sans select-none pb-16 bg-slate-50 dark:bg-[#070b14] text-slate-900 dark:text-slate-100 transition-colors duration-200">
  <!-- Top Cockpit Avionics Navigation Header -->
  <header class="flex flex-col sm:flex-row items-start sm:items-center justify-between pb-4 border-b border-slate-200 dark:border-slate-800/80 gap-4">
    <!-- Brand & Platform -->
    <div class="flex items-center gap-3">
      <div class="relative w-11 h-11 rounded-lg bg-gradient-to-br from-cyan-500 via-blue-600 to-indigo-700 flex items-center justify-center text-white font-black text-xl shadow-lg shadow-cyan-500/20 border border-white/20">
        M
        <span class="absolute -top-1 -right-1 w-2.5 h-2.5 rounded-full bg-cyan-400 ring-2 ring-white dark:ring-slate-900 animate-pulse"></span>
      </div>
      <div>
        <div class="flex items-center gap-2">
          <h1 class="text-xl font-black font-mono tracking-widest text-slate-900 dark:text-white uppercase">
            MoniQA
          </h1>
          <span class="text-[10px] font-mono px-2 py-0.5 rounded bg-cyan-500/10 text-cyan-600 dark:text-cyan-400 font-bold border border-cyan-500/30">
            {snapshot.system_info.chip_name}
          </span>
          <span class="hidden md:inline text-[9px] font-mono text-slate-400 uppercase tracking-widest">
            // TELEMETRY COCKPIT
          </span>
        </div>
        <p class="text-xs font-mono text-slate-500 dark:text-slate-400 mt-0.5">
          macOS Silicon Hardware & Process Telemetry Deck
        </p>
      </div>
    </div>

    <!-- Telemetry Status & Tactile Theme Switch -->
    <div class="flex items-center gap-3 flex-wrap">
      <!-- 3-Second Realtime Polling Badge -->
      <div class="flex items-center gap-2 px-3 py-1.5 rounded bg-emerald-500/10 border border-emerald-500/30 text-emerald-600 dark:text-emerald-400 text-xs font-mono font-bold">
        <span class="relative flex h-2 w-2">
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-75"></span>
          <span class="relative inline-flex rounded-full h-2 w-2 bg-emerald-500"></span>
        </span>
        <span>INTERVAL 3.0s</span>
      </div>

      <!-- Uptime Counter -->
      <div class="hidden sm:flex items-center gap-1.5 text-xs text-slate-600 dark:text-slate-300 font-mono bg-slate-100 dark:bg-slate-900 px-3 py-1.5 rounded border border-slate-200 dark:border-slate-800">
        <Clock class="w-3.5 h-3.5 text-slate-400" />
        <span>UP: {formatUptime(snapshot.system_info.uptime_secs)}</span>
      </div>

      <!-- Tactile Industrial Mode Switcher -->
      <button
        onclick={() => theme.toggle()}
        class="flex items-center gap-2 px-3 py-1.5 rounded font-mono text-xs font-bold bg-slate-200/90 dark:bg-slate-900 hover:bg-slate-300 dark:hover:bg-slate-800 border border-slate-300 dark:border-slate-700/80 transition-all duration-150 active:scale-95 shadow-sm"
        title="Toggle Light / Dark Mode"
      >
        {#if theme.isDark}
          <Sun class="w-4 h-4 text-amber-400" />
          <span class="text-slate-200 text-[11px]">DARK // ON</span>
          <span class="w-2 h-2 rounded-full bg-amber-400 shadow-[0_0_8px_rgba(251,191,36,0.9)]"></span>
        {:else}
          <Moon class="w-4 h-4 text-blue-600" />
          <span class="text-slate-800 text-[11px]">LIGHT // ON</span>
          <span class="w-2 h-2 rounded-full bg-blue-600 shadow-[0_0_8px_rgba(37,99,235,0.9)]"></span>
        {/if}
      </button>
    </div>
  </header>

  <!-- Primary Processing Tier: CPU & RAM -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-5">
    <CpuCard cpu={snapshot.cpu} detailed={true} />
    <MemoryCard memory={snapshot.memory} />
  </div>

  <!-- Accelerator & Thermal Array: GPU & Thermals -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-5">
    <GpuCard gpu={snapshot.gpu} />
    <ThermalCard thermal={snapshot.thermal} fans={snapshot.fans} />
  </div>

  <!-- Peripheral Bus & Power: Storage, Network & Battery -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-5">
    <div class="lg:col-span-2">
      <DiskNetworkCard disk={snapshot.disk} network={snapshot.network} />
    </div>
    <div class="lg:col-span-1">
      <BatteryCard battery={snapshot.battery} />
    </div>
  </div>

  <!-- Active Processes Ledger -->
  <ProcessTable processes={snapshot.top_processes} />

  <!-- Kernel & Hardware Specifications -->
  <SystemInfoCard info={snapshot.system_info} />
</div>
