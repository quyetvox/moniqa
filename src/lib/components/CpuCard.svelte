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

  let avgFreq = $derived.by(() => {
    if (!cpu.frequency_mhz || cpu.frequency_mhz.length === 0) return 0;
    const sum = cpu.frequency_mhz.reduce((a, b) => a + b, 0);
    return Math.round(sum / cpu.frequency_mhz.length);
  });
</script>

<Card
  tag="// CPU-NODE.01"
  title="Processor Core"
  badge={`${cpu.global_usage.toFixed(1)}%`}
>
  {#snippet headerAction()}
    <div class="flex items-center gap-1.5 text-xs font-mono font-medium text-slate-500 dark:text-slate-400">
      <Cpu class="w-3.5 h-3.5 text-cyan-500" />
      <span>{cpu.core_count}C / {cpu.thread_count}T</span>
    </div>
  {/snippet}

  <div class="space-y-4">
    <!-- Main Meter & Oscilloscope -->
    <div class="flex items-center justify-between gap-4">
      <div class="flex-1">
        <div class="flex justify-between items-baseline mb-1.5">
          <div class="flex items-baseline gap-1">
            <span class="text-3xl font-black font-tabular tracking-tight text-slate-900 dark:text-white">
              {cpu.global_usage.toFixed(1)}
            </span>
            <span class="text-xs font-mono font-bold text-cyan-600 dark:text-cyan-400">%</span>
          </div>
          {#if avgFreq > 0}
            <span class="text-[11px] font-mono px-1.5 py-0.5 rounded bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 border border-slate-200 dark:border-slate-700/60">
              {avgFreq} MHz
            </span>
          {/if}
        </div>
        <ProgressBar value={cpu.global_usage} height="h-3" segments={28} />
      </div>

      {#if stats.cpuHistory.length > 1}
        <div class="flex flex-col items-end shrink-0">
          <span class="text-[9px] text-slate-400 dark:text-slate-500 font-mono tracking-widest uppercase mb-1">
            TRACE // 30s
          </span>
          <Sparkline
            data={stats.cpuHistory}
            strokeColor="#06b6d4"
            fillColor="rgba(6, 182, 212, 0.15)"
            width={120}
            height={34}
          />
        </div>
      {/if}
    </div>

    <!-- Per Core Matrix Breakdown -->
    {#if detailed && cpu.per_core.length > 0}
      <div class="pt-3 border-t border-slate-100 dark:border-slate-800/80">
        <div class="flex justify-between items-center mb-2">
          <span class="text-[10px] font-mono font-bold text-slate-500 dark:text-slate-400 uppercase tracking-widest">
            CORE APERTURE [{cpu.per_core.length}]
          </span>
          <span class="text-[10px] font-mono text-slate-400 truncate max-w-[200px]">
            {cpu.model_name}
          </span>
        </div>

        <div class="grid grid-cols-2 sm:grid-cols-4 gap-2">
          {#each cpu.per_core as coreUsage, i}
            <div class="bg-slate-100/60 dark:bg-slate-950/60 rounded p-2 border border-slate-200/60 dark:border-slate-800/80">
              <div class="flex justify-between text-[10px] font-mono text-slate-600 dark:text-slate-300 mb-1">
                <span class="text-slate-400 font-bold">C{String(i + 1).padStart(2, '0')}</span>
                <span class="font-tabular font-bold {coreUsage > 75 ? 'text-rose-500' : 'text-slate-700 dark:text-slate-200'}">
                  {coreUsage.toFixed(0)}%
                </span>
              </div>
              <ProgressBar value={coreUsage} height="h-2" segments={12} />
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</Card>
