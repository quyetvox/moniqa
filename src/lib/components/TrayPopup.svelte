<script lang="ts">
  import type { SystemSnapshot } from "../types";
  import Sparkline from "./ui/Sparkline.svelte";
  import { stats } from "../stores/stats.svelte";
  import { theme } from "../stores/theme.svelte";
  import { settings } from "../stores/settings.svelte";
  import { formatBytes, formatRate, formatTemp } from "../utils/format";
  import { ArrowUpRight, Moon, Pause, Play, Sun } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";

  interface Props {
    snapshot: SystemSnapshot;
  }

  let { snapshot }: Props = $props();

  let topProc = $derived(snapshot.top_processes[0] ?? null);
  let ramGb = $derived(formatBytes(snapshot.memory.used_bytes));
  let ramTotalGb = $derived(formatBytes(snapshot.memory.total_bytes));
  let diskTotalRate = $derived(snapshot.disk.read_bytes_per_sec + snapshot.disk.write_bytes_per_sec);
  let diskSeries = $derived(
    stats.diskReadHistory.map((r, i) => r + (stats.diskWriteHistory[i] ?? 0))
  );
  let netSeries = $derived(
    stats.netRxHistory.map((r, i) => r + (stats.netTxHistory[i] ?? 0))
  );

  async function openDashboard() {
    try {
      await invoke("open_dashboard_window");
    } catch {
      // Browser preview fallback
      if (typeof window !== "undefined") {
        window.location.href = "/dashboard";
      }
    }
  }
</script>

<div
  class="w-[320px] mx-auto select-none text-[12px]"
  style="
    background: var(--bg-1);
    color: var(--text-1);
    border: 1px solid var(--line-strong);
    border-radius: var(--radius-panel);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.22);
  "
>
  <!-- Header -->
  <div
    class="flex items-center justify-between px-3 py-2"
    style="border-bottom: 1px solid var(--line);"
  >
    <div class="flex items-center gap-2">
      <img src="/logo.svg" alt="MoniQA" class="w-4 h-4 rounded shrink-0" />
      <span class="font-semibold tracking-tight text-[13px]">MoniQA</span>
      {#if settings.paused}
        <span
          class="px-1.5 py-0.5 text-[10px] font-medium"
          style="color: var(--warn); background: var(--warn-bg); border-radius: var(--radius);"
        >
          Paused
        </span>
      {:else}
        <span class="text-[11px] font-num" style="color: var(--text-3);">
          {snapshot.system_info.chip_name}
        </span>
      {/if}
    </div>

    <div class="flex items-center gap-1">
      <button
        onclick={() => settings.togglePause()}
        class="p-1 rounded transition-colors duration-100 hover:bg-[var(--bg-2)]"
        style="color: var(--text-2);"
        title={settings.paused ? "Resume updates" : "Pause updates"}
      >
        {#if settings.paused}
          <Play class="w-3.5 h-3.5" />
        {:else}
          <Pause class="w-3.5 h-3.5" />
        {/if}
      </button>
      <button
        onclick={() => theme.toggle()}
        class="p-1 rounded transition-colors duration-100 hover:bg-[var(--bg-2)]"
        style="color: var(--text-2);"
        title="Toggle theme"
      >
        {#if theme.isDark}
          <Sun class="w-3.5 h-3.5" />
        {:else}
          <Moon class="w-3.5 h-3.5" />
        {/if}
      </button>
    </div>
  </div>

  <!-- 5 Core Metric Rows -->
  <div class="px-3 py-2 space-y-1.5">
    <!-- CPU -->
    <div class="flex items-center justify-between gap-2 h-7">
      <div class="flex items-center gap-2 w-16 shrink-0">
        <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: var(--s-cpu);"></span>
        <span style="color: var(--text-2);">CPU</span>
      </div>
      <Sparkline data={stats.cpuHistory.slice(-30)} width={104} height={20} color="var(--s-cpu)" />
      <div class="w-20 text-right font-num font-medium">
        {snapshot.cpu.global_usage.toFixed(1)}<span class="text-[10px] ml-0.5" style="color: var(--text-3);">%</span>
      </div>
    </div>

    <!-- Memory -->
    <div class="flex items-center justify-between gap-2 h-7">
      <div class="flex items-center gap-2 w-16 shrink-0">
        <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: var(--s-mem);"></span>
        <span style="color: var(--text-2);">Memory</span>
      </div>
      <Sparkline data={stats.ramHistory.slice(-30)} width={104} height={20} color="var(--s-mem)" />
      <div class="w-20 text-right font-num font-medium truncate" title="{ramGb} / {ramTotalGb}">
        {ramGb}
      </div>
    </div>

    <!-- GPU -->
    <div class="flex items-center justify-between gap-2 h-7">
      <div class="flex items-center gap-2 w-16 shrink-0">
        <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: var(--s-gpu);"></span>
        <span style="color: var(--text-2);">GPU</span>
      </div>
      <Sparkline data={stats.gpuHistory.slice(-30)} width={104} height={20} color="var(--s-gpu)" />
      <div class="w-20 text-right font-num font-medium">
        {snapshot.gpu.usage_percent.toFixed(1)}<span class="text-[10px] ml-0.5" style="color: var(--text-3);">%</span>
      </div>
    </div>

    <!-- Disk -->
    <div class="flex items-center justify-between gap-2 h-7">
      <div class="flex items-center gap-2 w-16 shrink-0">
        <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: var(--s-disk);"></span>
        <span style="color: var(--text-2);">Disk</span>
      </div>
      <Sparkline data={diskSeries.slice(-30)} width={104} height={20} color="var(--s-disk)" max={0} />
      <div class="w-20 text-right font-num font-medium text-[11px]">
        {formatRate(diskTotalRate)}
      </div>
    </div>

    <!-- Network -->
    <div class="flex items-center justify-between gap-2 h-7">
      <div class="flex items-center gap-2 w-16 shrink-0">
        <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: var(--s-net);"></span>
        <span style="color: var(--text-2);">Net</span>
      </div>
      <Sparkline data={netSeries.slice(-30)} width={104} height={20} color="var(--s-net)" max={0} />
      <div class="w-20 text-right font-num font-medium text-[11px]">
        ↓ {formatRate(snapshot.network.total_received_bytes_per_sec, settings.netUnit)}
      </div>
    </div>
  </div>

  <!-- Secondary Context: Top Process, CPU Temp, Battery -->
  <div
    class="px-3 py-2 space-y-1 text-[11px]"
    style="border-top: 1px solid var(--line); background: var(--bg-0);"
  >
    {#if topProc}
      <div class="flex items-center justify-between gap-2">
        <span style="color: var(--text-3);">Top</span>
        <span class="truncate flex-1" style="color: var(--text-2);">{topProc.name}</span>
        <span class="font-num font-medium">{topProc.cpu_usage.toFixed(1)}%</span>
      </div>
    {/if}
    <div class="flex items-center justify-between gap-2">
      <span style="color: var(--text-3);">CPU temp</span>
      <span class="font-num" style="color: var(--text-2);">
        {formatTemp(snapshot.thermal.cpu_temp_c)}
        {#if snapshot.battery.has_battery}
          <span class="mx-1" style="color: var(--text-3);">·</span>
          Batt {snapshot.battery.charge_percent.toFixed(0)}%{snapshot.battery.is_charging ? " (AC)" : ""}
        {/if}
      </span>
    </div>
  </div>

  <!-- Footer -->
  <div
    class="flex items-center justify-between px-3 py-2"
    style="border-top: 1px solid var(--line);"
  >
    <button
      onclick={openDashboard}
      class="flex items-center gap-1 text-[12px] font-medium transition-colors duration-100 hover:underline"
      style="color: var(--accent);"
    >
      <span>Open Monitor</span>
      <ArrowUpRight class="w-3.5 h-3.5" />
    </button>
    <span class="text-[10px] font-num" style="color: var(--text-3);">
      ↑ {formatRate(snapshot.network.total_transmitted_bytes_per_sec, settings.netUnit)}
    </span>
  </div>
</div>
