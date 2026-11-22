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
  import { Activity, Clock, Moon, RefreshCw, Sun } from "@lucide/svelte";

  interface Props {
    snapshot: SystemSnapshot;
  }

  let { snapshot }: Props = $props();
</script>

<div class="min-h-screen p-5 space-y-5 max-w-6xl mx-auto font-sans select-none pb-12">
  <!-- Top App Navigation Bar -->
  <header class="flex items-center justify-between pb-4 border-b border-slate-200/80 dark:border-slate-800/80">
    <div class="flex items-center gap-3">
      <div class="w-10 h-10 rounded-2xl bg-blue-600 flex items-center justify-center text-white font-black text-lg shadow-md shadow-blue-500/20">
        M
      </div>
      <div>
        <div class="flex items-center gap-2">
          <h1 class="text-xl font-black tracking-tight text-slate-800 dark:text-white">
            MoniQA
          </h1>
          <span class="text-xs px-2.5 py-0.5 rounded-full bg-blue-500/10 text-blue-600 dark:text-blue-400 font-semibold border border-blue-500/20">
            {snapshot.system_info.chip_name}
          </span>
        </div>
        <p class="text-xs text-slate-500 dark:text-slate-400 font-medium">
          macOS Telemetry & Hardware Monitor Dashboard
        </p>
      </div>
    </div>

    <div class="flex items-center gap-3">
      <!-- Live Indicator -->
      <div class="flex items-center gap-1.5 px-3 py-1.5 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-emerald-600 dark:text-emerald-400 text-xs font-semibold">
        <span class="w-2 h-2 rounded-full bg-emerald-500 animate-ping"></span>
        <span>Realtime 1s</span>
      </div>

      <!-- Uptime -->
      <div class="hidden sm:flex items-center gap-1.5 text-xs text-slate-500 dark:text-slate-400 font-mono bg-slate-100 dark:bg-slate-800/60 px-3 py-1.5 rounded-xl border border-slate-200/60 dark:border-slate-700/50">
        <Clock class="w-3.5 h-3.5 text-slate-400" />
        <span>Up: {formatUptime(snapshot.system_info.uptime_secs)}</span>
      </div>

      <!-- Theme Switcher -->
      <button
        onclick={() => theme.toggle()}
        class="p-2 rounded-xl text-slate-600 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-800 border border-slate-200/60 dark:border-slate-700/50 transition-colors"
        title="Toggle Theme"
      >
        {#if theme.isDark}
          <Sun class="w-4 h-4 text-amber-400" />
        {:else}
          <Moon class="w-4 h-4 text-slate-600" />
        {/if}
      </button>
    </div>
  </header>

  <!-- Core Gauges Grid -->
  <div class="grid grid-cols-1 lg:grid-cols-2 gap-4">
    <!-- CPU Usage -->
    <CpuCard cpu={snapshot.cpu} detailed={true} />

    <!-- Memory & Swap -->
    <MemoryCard memory={snapshot.memory} />
  </div>

  <!-- Secondary Metrics: GPU & Thermals -->
  <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
    <GpuCard gpu={snapshot.gpu} />
    <ThermalCard thermal={snapshot.thermal} fans={snapshot.fans} />
  </div>

  <!-- Storage, Network & Battery -->
  <div class="grid grid-cols-1 lg:grid-cols-3 gap-4">
    <div class="lg:col-span-2">
      <DiskNetworkCard disk={snapshot.disk} network={snapshot.network} />
    </div>
    <div class="lg:col-span-1">
      <BatteryCard battery={snapshot.battery} />
    </div>
  </div>

  <!-- Top Active Processes Table -->
  <ProcessTable processes={snapshot.top_processes} />

  <!-- System Specifications -->
  <SystemInfoCard info={snapshot.system_info} />
</div>
