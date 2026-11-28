<script lang="ts">
  import { onMount } from "svelte";
  import type { SystemSnapshot } from "../types";
  import { stats } from "../stores/stats.svelte";
  import { theme } from "../stores/theme.svelte";
  import { settings } from "../stores/settings.svelte";
  import { formatBytes, formatRate, formatTemp, formatUptime } from "../utils/format";
  import Sparkline from "./ui/Sparkline.svelte";
  import LineChart, { type ChartSeries } from "./ui/LineChart.svelte";
  import CapacityBar from "./ui/CapacityBar.svelte";
  import StatusBadge from "./ui/StatusBadge.svelte";
  import ProcessTable from "./ProcessTable.svelte";
  import SettingsView from "./views/SettingsView.svelte";
  import { Moon, Pause, Play, Settings, Sun, X } from "@lucide/svelte";

  interface Props {
    snapshot: SystemSnapshot;
  }

  let { snapshot }: Props = $props();

  let showCpu = $state(true);
  let showMem = $state(true);
  let showGpu = $state(true);
  let showSettings = $state(false);

  let sampleLimit = $derived(settings.timeWindow === "5m" ? 300 : 60);

  let avgFreq = $derived.by(() => {
    const freqs = snapshot.cpu.frequency_mhz;
    if (!freqs || freqs.length === 0) return 0;
    return Math.round(freqs.reduce((a, b) => a + b, 0) / freqs.length);
  });

  let ramPct = $derived(
    snapshot.memory.total_bytes > 0
      ? (snapshot.memory.used_bytes / snapshot.memory.total_bytes) * 100
      : 0
  );

  let swapPct = $derived(
    snapshot.memory.swap_total_bytes > 0
      ? (snapshot.memory.swap_used_bytes / snapshot.memory.swap_total_bytes) * 100
      : 0
  );

  let diskTotalRate = $derived(
    snapshot.disk.read_bytes_per_sec + snapshot.disk.write_bytes_per_sec
  );

  let diskSeries = $derived(
    stats.diskReadHistory
      .slice(-sampleLimit)
      .map((r, i) => r + (stats.diskWriteHistory.slice(-sampleLimit)[i] ?? 0))
  );

  let netSeries = $derived(
    stats.netRxHistory
      .slice(-sampleLimit)
      .map((r, i) => r + (stats.netTxHistory.slice(-sampleLimit)[i] ?? 0))
  );

  let chartSeries = $derived.by<ChartSeries[]>(() => {
    const out: ChartSeries[] = [];
    if (showCpu) {
      out.push({
        id: "cpu",
        label: "CPU",
        data: stats.cpuHistory.slice(-sampleLimit),
        color: "var(--s-cpu)",
      });
    }
    if (showMem) {
      out.push({
        id: "mem",
        label: "Memory",
        data: stats.ramHistory.slice(-sampleLimit),
        color: "var(--s-mem)",
      });
    }
    if (showGpu) {
      out.push({
        id: "gpu",
        label: "GPU",
        data: stats.gpuHistory.slice(-sampleLimit),
        color: "var(--s-gpu)",
      });
    }
    return out;
  });

  function boundText(key: string, unit = "°C"): { min: string; max: string } {
    const b = stats.sensorBounds[key];
    if (!b) return { min: "—", max: "—" };
    return {
      min: `${b.min.toFixed(0)}${unit}`,
      max: `${b.max.toFixed(0)}${unit}`,
    };
  }

  onMount(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      const target = e.target as HTMLElement | null;
      if (target && (target.tagName === "INPUT" || target.tagName === "TEXTAREA")) {
        return;
      }
      if (e.code === "Space") {
        e.preventDefault();
        settings.togglePause();
      } else if (e.key === "Escape" && showSettings) {
        showSettings = false;
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  });
</script>

<div class="h-screen flex flex-col overflow-hidden select-none" style="background: var(--bg-0); color: var(--text-1);">
  <!-- Slim Unified Header -->
  <header
    class="h-9 px-3 flex items-center justify-between shrink-0"
    style="background: var(--bg-1); border-bottom: 1px solid var(--line);"
  >
    <div class="flex items-center gap-2 min-w-0">
      <img src="/logo.svg" alt="MoniQA" class="w-5 h-5 rounded shrink-0" />
      <span class="font-semibold text-[13px] tracking-tight">MoniQA</span>
      <span class="text-[11px] font-num truncate" style="color: var(--text-3);">
        {snapshot.system_info.chip_name}
      </span>
      {#if settings.paused}
        <span
          class="px-1.5 py-0.5 text-[10px] font-medium shrink-0"
          style="color: var(--warn); background: var(--warn-bg); border-radius: var(--radius);"
        >
          Paused
        </span>
      {/if}
      {#if stats.isMockMode}
        <span
          class="px-1.5 py-0.5 text-[10px] shrink-0"
          style="color: var(--text-3); background: var(--bg-2); border-radius: var(--radius);"
        >
          Preview Data
        </span>
      {/if}
    </div>

    <div class="flex items-center gap-1.5 shrink-0">
      <!-- Time Window Toggle -->
      <div
        class="inline-flex text-[11px] font-num overflow-hidden"
        style="border: 1px solid var(--line); border-radius: var(--radius);"
      >
        <button
          onclick={() => settings.setTimeWindow("60s")}
          class="px-2 py-0.5 transition-colors"
          style="background: {settings.timeWindow === '60s' ? 'var(--accent-soft)' : 'transparent'}; color: {settings.timeWindow === '60s' ? 'var(--accent)' : 'var(--text-2)'};"
        >
          60s
        </button>
        <button
          onclick={() => settings.setTimeWindow("5m")}
          class="px-2 py-0.5 transition-colors"
          style="border-left: 1px solid var(--line); background: {settings.timeWindow === '5m' ? 'var(--accent-soft)' : 'transparent'}; color: {settings.timeWindow === '5m' ? 'var(--accent)' : 'var(--text-2)'};"
        >
          5m
        </button>
      </div>

      <!-- Pause / Resume -->
      <button
        onclick={() => settings.togglePause()}
        class="inline-flex items-center gap-1 px-2 py-0.5 text-[11px] font-medium hover:bg-[var(--bg-2)] transition-colors"
        style="border: 1px solid var(--line); border-radius: var(--radius); color: var(--text-2);"
        title="Pause / Resume (Space)"
      >
        {#if settings.paused}
          <Play class="w-3 h-3" />
          <span>Resume</span>
        {:else}
          <Pause class="w-3 h-3" />
          <span>Pause</span>
        {/if}
      </button>

      <!-- Theme Toggle -->
      <button
        onclick={() => theme.toggle()}
        class="p-1 hover:bg-[var(--bg-2)] transition-colors"
        style="border: 1px solid var(--line); border-radius: var(--radius); color: var(--text-2);"
        title="Toggle Light / Dark"
      >
        {#if theme.isDark}
          <Sun class="w-3.5 h-3.5" />
        {:else}
          <Moon class="w-3.5 h-3.5" />
        {/if}
      </button>

      <!-- Settings Popover Trigger -->
      <button
        onclick={() => (showSettings = !showSettings)}
        class="p-1 transition-colors"
        style="
          border: 1px solid var(--line);
          border-radius: var(--radius);
          background: {showSettings ? 'var(--accent-soft)' : 'transparent'};
          color: {showSettings ? 'var(--accent)' : 'var(--text-2)'};
        "
        title="Settings"
      >
        <Settings class="w-3.5 h-3.5" />
      </button>
    </div>
  </header>

  <!-- Single-View All-In-One Workspace -->
  <main class="flex-1 overflow-y-auto p-2.5 space-y-2.5">
    <!-- 1. Compact 5-Metric Summary Strip -->
    <div
      class="grid grid-cols-2 sm:grid-cols-5 overflow-hidden"
      style="background: var(--bg-1); border: 1px solid var(--line); border-radius: var(--radius-panel);"
    >
      <!-- CPU -->
      <div class="flex items-center justify-between gap-2 px-2.5 py-2" style="border-right: 1px solid var(--line);">
        <div class="min-w-0">
          <div class="flex items-center gap-1.5 text-[10px]" style="color: var(--text-2);">
            <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: var(--s-cpu);"></span>
            <span>CPU</span>
            <StatusBadge value={snapshot.cpu.global_usage} warn={settings.warnThreshold} crit={settings.critThreshold} />
          </div>
          <div class="font-num text-[15px] font-semibold leading-tight mt-0.5">
            {snapshot.cpu.global_usage.toFixed(1)}<span class="text-[10px] ml-0.5 font-normal" style="color: var(--text-3);">%</span>
          </div>
          <div class="font-num text-[10px] truncate" style="color: var(--text-3);">
            {snapshot.cpu.core_count}C/{snapshot.cpu.thread_count}T · {avgFreq > 0 ? `${avgFreq} MHz` : "—"}
          </div>
        </div>
        <Sparkline data={stats.cpuHistory.slice(-30)} width={48} height={22} color="var(--s-cpu)" />
      </div>

      <!-- Memory -->
      <div class="flex items-center justify-between gap-2 px-2.5 py-2" style="border-right: 1px solid var(--line);">
        <div class="min-w-0">
          <div class="flex items-center gap-1.5 text-[10px]" style="color: var(--text-2);">
            <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: var(--s-mem);"></span>
            <span>Memory</span>
            <StatusBadge value={ramPct} warn={settings.warnThreshold} crit={settings.critThreshold} />
          </div>
          <div class="font-num text-[13px] font-semibold leading-tight mt-0.5 truncate">
            {formatBytes(snapshot.memory.used_bytes)}
            <span class="text-[10px] font-normal" style="color: var(--text-3);">/ {formatBytes(snapshot.memory.total_bytes)}</span>
          </div>
          <div class="font-num text-[10px] truncate" style="color: var(--text-3);">
            Avail {formatBytes(snapshot.memory.available_bytes)} · Swap {formatBytes(snapshot.memory.swap_used_bytes)}
          </div>
        </div>
        <Sparkline data={stats.ramHistory.slice(-30)} width={44} height={22} color="var(--s-mem)" />
      </div>

      <!-- GPU -->
      <div class="flex items-center justify-between gap-2 px-2.5 py-2" style="border-right: 1px solid var(--line);">
        <div class="min-w-0">
          <div class="flex items-center gap-1.5 text-[10px]" style="color: var(--text-2);">
            <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: var(--s-gpu);"></span>
            <span>GPU</span>
          </div>
          <div class="font-num text-[15px] font-semibold leading-tight mt-0.5">
            {snapshot.gpu.usage_percent.toFixed(1)}<span class="text-[10px] ml-0.5 font-normal" style="color: var(--text-3);">%</span>
          </div>
          <div class="font-num text-[10px] truncate" style="color: var(--text-3);">
            {snapshot.gpu.power_watts.toFixed(1)}W · {snapshot.gpu.frequency_mhz > 0 ? `${snapshot.gpu.frequency_mhz}MHz` : "—"}
          </div>
        </div>
        <Sparkline data={stats.gpuHistory.slice(-30)} width={48} height={22} color="var(--s-gpu)" />
      </div>

      <!-- Disk I/O -->
      <div class="flex items-center justify-between gap-2 px-2.5 py-2" style="border-right: 1px solid var(--line);">
        <div class="min-w-0">
          <div class="flex items-center gap-1.5 text-[10px]" style="color: var(--text-2);">
            <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: var(--s-disk);"></span>
            <span>Disk I/O</span>
          </div>
          <div class="font-num text-[13px] font-semibold leading-tight mt-0.5 truncate">
            {formatRate(diskTotalRate)}
          </div>
          <div class="font-num text-[10px] truncate" style="color: var(--text-3);">
            R {formatRate(snapshot.disk.read_bytes_per_sec)} · W {formatRate(snapshot.disk.write_bytes_per_sec)}
          </div>
        </div>
        <Sparkline data={diskSeries.slice(-30)} width={44} height={22} color="var(--s-disk)" max={0} />
      </div>

      <!-- Network -->
      <div class="flex items-center justify-between gap-2 px-2.5 py-2">
        <div class="min-w-0">
          <div class="flex items-center gap-1.5 text-[10px]" style="color: var(--text-2);">
            <span class="w-1.5 h-1.5 rounded-full shrink-0" style="background: var(--s-net);"></span>
            <span>Network</span>
          </div>
          <div class="font-num text-[13px] font-semibold leading-tight mt-0.5 truncate">
            ↓ {formatRate(snapshot.network.total_received_bytes_per_sec, settings.netUnit)}
          </div>
          <div class="font-num text-[10px] truncate" style="color: var(--text-3);">
            ↑ {formatRate(snapshot.network.total_transmitted_bytes_per_sec, settings.netUnit)}
          </div>
        </div>
        <Sparkline data={netSeries.slice(-30)} width={44} height={22} color="var(--s-net)" max={0} />
      </div>
    </div>

    <!-- 2. Charts Row: System Utilization + Disk Throughput + Network Throughput -->
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-2.5">
      <!-- System Utilization -->
      <div
        class="lg:col-span-6 p-2.5"
        style="background: var(--bg-1); border: 1px solid var(--line); border-radius: var(--radius-panel);"
      >
        <div class="flex items-center justify-between gap-2 mb-1.5">
          <span class="text-[11px] font-semibold">System Utilization</span>
          <div class="flex items-center gap-1.5 text-[10px]">
            <button
              onclick={() => (showCpu = !showCpu)}
              class="inline-flex items-center gap-1 px-1 py-0.5 transition-opacity"
              style="opacity: {showCpu ? 1 : 0.4}; color: var(--text-2);"
            >
              <span class="w-1.5 h-1.5 rounded-full" style="background: var(--s-cpu);"></span>
              CPU
            </button>
            <button
              onclick={() => (showMem = !showMem)}
              class="inline-flex items-center gap-1 px-1 py-0.5 transition-opacity"
              style="opacity: {showMem ? 1 : 0.4}; color: var(--text-2);"
            >
              <span class="w-1.5 h-1.5 rounded-full" style="background: var(--s-mem);"></span>
              Mem
            </button>
            <button
              onclick={() => (showGpu = !showGpu)}
              class="inline-flex items-center gap-1 px-1 py-0.5 transition-opacity"
              style="opacity: {showGpu ? 1 : 0.4}; color: var(--text-2);"
            >
              <span class="w-1.5 h-1.5 rounded-full" style="background: var(--s-gpu);"></span>
              GPU
            </button>
          </div>
        </div>
        <LineChart
          series={chartSeries}
          timestamps={stats.timestamps.slice(-sampleLimit)}
          height={128}
          timeLabel={settings.timeWindow}
          paused={settings.paused}
        />
      </div>

      <!-- Disk Throughput -->
      <div
        class="lg:col-span-3 p-2.5"
        style="background: var(--bg-1); border: 1px solid var(--line); border-radius: var(--radius-panel);"
      >
        <div class="flex items-center justify-between gap-2 mb-1.5">
          <div class="flex items-center gap-1.5">
            <span class="w-1.5 h-1.5 rounded-full" style="background: var(--s-disk);"></span>
            <span class="text-[11px] font-semibold">Disk I/O</span>
          </div>
          <span class="font-num text-[10px]" style="color: var(--text-3);">
            R {formatRate(snapshot.disk.read_bytes_per_sec)}
          </span>
        </div>
        <LineChart
          series={[
            {
              id: "read",
              label: "Read",
              data: stats.diskReadHistory.slice(-sampleLimit),
              color: "var(--s-disk)",
              formatValue: (v) => formatRate(v),
            },
            {
              id: "write",
              label: "Write",
              data: stats.diskWriteHistory.slice(-sampleLimit),
              color: "#a8691c",
              formatValue: (v) => formatRate(v),
            },
          ]}
          timestamps={stats.timestamps.slice(-sampleLimit)}
          height={128}
          yMax={0}
          formatY={(v) => formatRate(v)}
          timeLabel={settings.timeWindow}
          paused={settings.paused}
        />
      </div>

      <!-- Network Throughput -->
      <div
        class="lg:col-span-3 p-2.5"
        style="background: var(--bg-1); border: 1px solid var(--line); border-radius: var(--radius-panel);"
      >
        <div class="flex items-center justify-between gap-2 mb-1.5">
          <div class="flex items-center gap-1.5">
            <span class="w-1.5 h-1.5 rounded-full" style="background: var(--s-net);"></span>
            <span class="text-[11px] font-semibold">Network</span>
          </div>
          <span class="font-num text-[10px]" style="color: var(--text-3);">
            ↓ {formatRate(snapshot.network.total_received_bytes_per_sec, settings.netUnit)}
          </span>
        </div>
        <LineChart
          series={[
            {
              id: "rx",
              label: "Receive",
              data: stats.netRxHistory.slice(-sampleLimit),
              color: "var(--s-net)",
              formatValue: (v) => formatRate(v, settings.netUnit),
            },
            {
              id: "tx",
              label: "Transmit",
              data: stats.netTxHistory.slice(-sampleLimit),
              color: "#2b6cb0",
              formatValue: (v) => formatRate(v, settings.netUnit),
            },
          ]}
          timestamps={stats.timestamps.slice(-sampleLimit)}
          height={128}
          yMax={0}
          formatY={(v) => formatRate(v, settings.netUnit)}
          timeLabel={settings.timeWindow}
          paused={settings.paused}
        />
      </div>
    </div>

    <!-- 3. Hardware Details Strip: Cores & Memory | Volumes & Network | Sensors & Battery -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-2.5">
      <!-- Panel A: Logical Cores, Memory & GPU Specs -->
      <div
        class="p-2.5 space-y-2"
        style="background: var(--bg-1); border: 1px solid var(--line); border-radius: var(--radius-panel);"
      >
        <div class="flex items-center justify-between">
          <span class="text-[11px] font-semibold">Compute & Memory</span>
          <span class="text-[10px] font-num truncate max-w-[160px]" style="color: var(--text-3);" title={snapshot.cpu.model_name}>
            {snapshot.cpu.model_name}
          </span>
        </div>

        {#if snapshot.cpu.per_core.length > 0}
          <div class="grid grid-cols-4 gap-x-2 gap-y-1">
            {#each snapshot.cpu.per_core as coreUsage, i}
              <div>
                <div class="flex justify-between text-[10px] font-num leading-tight">
                  <span style="color: var(--text-3);">C{String(i + 1).padStart(2, "0")}</span>
                  <span style="color: {coreUsage >= settings.critThreshold ? 'var(--crit)' : coreUsage >= settings.warnThreshold ? 'var(--warn)' : 'var(--text-1)'};">
                    {coreUsage.toFixed(0)}%
                  </span>
                </div>
                <CapacityBar value={coreUsage} color="var(--s-cpu)" height={4} />
              </div>
            {/each}
          </div>
        {/if}

        <div class="space-y-1.5 pt-1.5" style="border-top: 1px solid var(--line);">
          <div>
            <div class="flex justify-between text-[10px] mb-0.5">
              <span style="color: var(--text-2);">RAM ({ramPct.toFixed(1)}%)</span>
              <span class="font-num" style="color: var(--text-3);">
                {formatBytes(snapshot.memory.used_bytes)} / {formatBytes(snapshot.memory.total_bytes)}
              </span>
            </div>
            <CapacityBar value={ramPct} color="var(--s-mem)" height={5} />
          </div>
          <div>
            <div class="flex justify-between text-[10px] mb-0.5">
              <span style="color: var(--text-2);">Swap ({swapPct.toFixed(1)}%)</span>
              <span class="font-num" style="color: var(--text-3);">
                {formatBytes(snapshot.memory.swap_used_bytes)} / {formatBytes(snapshot.memory.swap_total_bytes)}
              </span>
            </div>
            <CapacityBar value={swapPct} color="var(--s-mem)" height={4} />
          </div>
          <div class="flex justify-between text-[10px] pt-0.5 font-num" style="color: var(--text-3);">
            <span>GPU VRAM</span>
            <span>{snapshot.gpu.vram_used_bytes > 0 ? formatBytes(snapshot.gpu.vram_used_bytes) : "Shared / Unified"}</span>
          </div>
        </div>
      </div>

      <!-- Panel B: Mounted Volumes & Network Interfaces -->
      <div
        class="p-2.5 space-y-2"
        style="background: var(--bg-1); border: 1px solid var(--line); border-radius: var(--radius-panel);"
      >
        <div class="text-[11px] font-semibold">Volumes & Interfaces</div>

        {#if snapshot.disk.disks.length > 0}
          <div class="space-y-1.5">
            {#each snapshot.disk.disks as part}
              {@const usedPct = part.total_bytes > 0 ? (part.used_bytes / part.total_bytes) * 100 : 0}
              <div>
                <div class="flex items-center justify-between text-[10px] mb-0.5">
                  <div class="flex items-center gap-1.5 min-w-0">
                    <span class="font-medium truncate">{part.name || "Volume"}</span>
                    <span class="font-num truncate" style="color: var(--text-3);">{part.mount_point}</span>
                    <StatusBadge value={usedPct} warn={80} crit={92} />
                  </div>
                  <span class="font-num shrink-0" style="color: var(--text-2);">
                    {formatBytes(part.used_bytes)}/{formatBytes(part.total_bytes)}
                  </span>
                </div>
                <CapacityBar value={usedPct} color="var(--s-disk)" height={4} />
              </div>
            {/each}
          </div>
        {/if}

        {#if snapshot.network.interfaces.length > 0}
          <div class="pt-1.5" style="border-top: 1px solid var(--line);">
            <table class="w-full text-[10px]">
              <thead>
                <tr style="color: var(--text-3);">
                  <th class="text-left pb-0.5 font-medium">Interface</th>
                  <th class="text-right pb-0.5 font-medium">Receive</th>
                  <th class="text-right pb-0.5 font-medium">Transmit</th>
                </tr>
              </thead>
              <tbody class="divide-y" style="border-color: var(--line);">
                {#each snapshot.network.interfaces as iface}
                  <tr class="h-5">
                    <td class="font-medium">{iface.name}</td>
                    <td class="text-right font-num">{formatRate(iface.rx_bytes_per_sec, settings.netUnit)}</td>
                    <td class="text-right font-num" style="color: var(--text-2);">{formatRate(iface.tx_bytes_per_sec, settings.netUnit)}</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        {/if}
      </div>

      <!-- Panel C: Thermal, Fans, Battery & System Info -->
      <div
        class="p-2.5 space-y-2"
        style="background: var(--bg-1); border: 1px solid var(--line); border-radius: var(--radius-panel);"
      >
        <div class="text-[11px] font-semibold">Sensors, Power & System</div>

        <table class="w-full text-[10px]">
          <thead>
            <tr style="color: var(--text-3); border-bottom: 1px solid var(--line);">
              <th class="text-left pb-0.5 font-medium">Sensor</th>
              <th class="text-right pb-0.5 font-medium">Cur</th>
              <th class="text-right pb-0.5 font-medium">Min / Max</th>
            </tr>
          </thead>
          <tbody class="divide-y" style="border-color: var(--line);">
            {#each [
              { key: "cpu_temp", name: "CPU Die", val: snapshot.thermal.cpu_temp_c },
              { key: "gpu_temp", name: "GPU Die", val: snapshot.thermal.gpu_temp_c },
              { key: "pkg_temp", name: "SoC Package", val: snapshot.thermal.package_temp_c },
              { key: "bat_temp", name: "Battery Cell", val: snapshot.thermal.battery_temp_c },
            ] as row}
              {@const b = boundText(row.key)}
              <tr class="h-5">
                <td style="color: var(--text-2);">{row.name}</td>
                <td
                  class="text-right font-num font-medium"
                  style="color: {row.val && row.val >= 85 ? 'var(--crit)' : row.val && row.val >= 72 ? 'var(--warn)' : 'var(--text-1)'};"
                >
                  {formatTemp(row.val)}
                </td>
                <td class="text-right font-num" style="color: var(--text-3);">{b.min} / {b.max}</td>
              </tr>
            {/each}
            {#if snapshot.fans.length > 0}
              {#each snapshot.fans as fan}
                {@const fb = boundText(`fan_${fan.index}`, "")}
                <tr class="h-5">
                  <td style="color: var(--text-2);">{fan.name}</td>
                  <td class="text-right font-num font-medium">{fan.rpm} RPM</td>
                  <td class="text-right font-num" style="color: var(--text-3);">{fb.min} / {fb.max}</td>
                </tr>
              {/each}
            {:else}
              <tr class="h-5">
                <td style="color: var(--text-3);">Cooling Fans</td>
                <td colspan="2" class="text-right" style="color: var(--text-3);">Passive / No fan</td>
              </tr>
            {/if}
          </tbody>
        </table>

        <div class="grid grid-cols-2 gap-x-3 gap-y-1 pt-1.5 text-[10px]" style="border-top: 1px solid var(--line);">
          <div>
            <span style="color: var(--text-3);">Battery: </span>
            <span class="font-num font-medium">
              {snapshot.battery.has_battery
                ? `${snapshot.battery.charge_percent.toFixed(0)}%${snapshot.battery.is_charging ? " (AC)" : ""}`
                : "AC Only"}
            </span>
          </div>
          <div class="text-right">
            <span style="color: var(--text-3);">Health/Cyc: </span>
            <span class="font-num font-medium">
              {snapshot.battery.health_percent ? `${snapshot.battery.health_percent}%` : "—"}/{snapshot.battery.cycle_count ?? "—"}
            </span>
          </div>
          <div class="col-span-2 truncate font-num" style="color: var(--text-3);" title="{snapshot.system_info.os_name} {snapshot.system_info.os_version} ({snapshot.system_info.kernel_version})">
            {snapshot.system_info.os_name} {snapshot.system_info.os_version} · Kernel {snapshot.system_info.kernel_version}
          </div>
        </div>
      </div>
    </div>

    <!-- 4. Active Processes Table -->
    <ProcessTable
      processes={snapshot.top_processes}
      totalMemoryBytes={snapshot.memory.total_bytes}
      title="Processes"
      maxBodyHeight="230px"
    />
  </main>

  <!-- Compact Settings Modal/Popover -->
  {#if showSettings}
    <div
      class="fixed inset-0 z-50 flex items-start justify-end p-3 pt-11"
      style="background: rgba(0, 0, 0, 0.35);"
      onclick={() => (showSettings = false)}
      role="presentation"
    >
      <div
        class="w-full max-w-md overflow-hidden"
        style="
          background: var(--bg-0);
          border: 1px solid var(--line-strong);
          border-radius: var(--radius-panel);
          box-shadow: 0 12px 32px rgba(0, 0, 0, 0.35);
        "
        onclick={(e) => e.stopPropagation()}
        onkeydown={(e) => { if (e.key === "Escape") showSettings = false; }}
        tabindex="-1"
        role="dialog"
        aria-label="Settings"
      >
        <div
          class="flex items-center justify-between px-3 py-2"
          style="background: var(--bg-1); border-bottom: 1px solid var(--line);"
        >
          <span class="text-[12px] font-semibold">Settings</span>
          <button
            onclick={() => (showSettings = false)}
            class="p-0.5 rounded hover:bg-[var(--bg-2)]"
            style="color: var(--text-3);"
          >
            <X class="w-4 h-4" />
          </button>
        </div>
        <div class="p-3">
          <SettingsView />
        </div>
      </div>
    </div>
  {/if}

  <!-- Bottom Status Bar -->
  <footer
    class="h-6 px-3 flex items-center justify-between text-[10px] font-num shrink-0"
    style="background: var(--bg-1); border-top: 1px solid var(--line); color: var(--text-3);"
  >
    <div class="flex items-center gap-2.5">
      <span class="inline-flex items-center gap-1.5">
        <span
          class="w-1.5 h-1.5 rounded-full"
          style="background: {settings.paused ? 'var(--warn)' : 'var(--ok)'};"
        ></span>
        {settings.paused ? "Paused (Space)" : "Live"}
      </span>
      <span>·</span>
      <span>Samples: {stats.cpuHistory.length}</span>
    </div>

    <div class="flex items-center gap-2.5 truncate">
      <span>Uptime {formatUptime(snapshot.system_info.uptime_secs)}</span>
      <span class="hidden sm:inline">·</span>
      <span class="hidden sm:inline">{snapshot.system_info.hostname}</span>
      <span class="hidden md:inline">·</span>
      <span class="hidden md:inline">{snapshot.system_info.os_name} {snapshot.system_info.os_version}</span>
    </div>
  </footer>
</div>
