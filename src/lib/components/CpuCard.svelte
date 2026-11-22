<script lang="ts">
  import type { CpuStats } from "../types";
  import Card from "./ui/Card.svelte";
  import ProgressBar from "./ui/ProgressBar.svelte";
  import Sparkline from "./ui/Sparkline.svelte";
  import { stats } from "../stores/stats.svelte";
  import { Cpu } from "@lucide/svelte";

  interface Props {
    cpu: CpuStats;
    detailed?: boolean;
  }

  let { cpu, detailed = true }: Props = $props();

  let avgFreq = $derived(() => {
    if (!cpu.frequency_mhz || cpu.frequency_mhz.length === 0) return 0;
    const sum = cpu.frequency_mhz.reduce((a, b) => a + b, 0);
    return Math.round(sum / cpu.frequency_mhz.length);
  });
</script>

<Card title="CPU Usage" badge="{cpu.global_usage.toFixed(1)}%">
  {#snippet headerAction()}
    <div class="flex items-center gap-2">
      <Cpu class="w-4 h-4 text-blue-500" />
      <span class="text-xs font-semibold text-slate-700 dark:text-slate-300">
        {cpu.core_count} Cores / {cpu.thread_count} Threads
      </span>
    </div>
  {/snippet}

  <div class="space-y-4">
    <!-- Main Meter & Sparkline -->
    <div class="flex items-center justify-between gap-4">
      <div class="flex-1">
        <div class="flex justify-between items-baseline mb-1">
          <span class="text-2xl font-black tracking-tight text-slate-800 dark:text-white">
            {cpu.global_usage.toFixed(1)}<span class="text-sm font-medium text-slate-400 ml-0.5">%</span>
          </span>
          {#if avgFreq() > 0}
            <span class="text-xs font-mono text-slate-500 dark:text-slate-400">
              ~{avgFreq()} MHz
            </span>
          {/if}
        </div>
        <ProgressBar value={cpu.global_usage} height="h-2.5" />
      </div>

      {#if stats.cpuHistory.length > 1}
        <div class="flex flex-col items-end">
          <span class="text-[10px] text-slate-400 font-mono mb-1">Last 30s</span>
          <Sparkline
            data={stats.cpuHistory}
            strokeColor="#3b82f6"
            fillColor="rgba(59, 130, 246, 0.15)"
            width={110}
            height={32}
          />
        </div>
      {/if}
    </div>

    <!-- Per Core Breakdown (if detailed) -->
    {#if detailed && cpu.per_core.length > 0}
      <div class="pt-2 border-t border-slate-100 dark:border-slate-800/80">
        <div class="flex justify-between items-center mb-2">
          <span class="text-[11px] font-semibold text-slate-500 dark:text-slate-400 uppercase tracking-wider">
            Per-Core Distribution
          </span>
          <span class="text-[11px] text-slate-400">
            {cpu.model_name}
          </span>
        </div>

        <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
          {#each cpu.per_core as coreUsage, i}
            <div class="bg-slate-50/80 dark:bg-slate-800/40 rounded-lg p-2 border border-slate-100 dark:border-slate-800/50">
              <div class="flex justify-between text-[11px] font-mono text-slate-600 dark:text-slate-300 mb-1">
                <span>Core {i + 1}</span>
                <span class="font-semibold">{coreUsage.toFixed(0)}%</span>
              </div>
              <ProgressBar value={coreUsage} height="h-1.5" />
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</Card>
