<script lang="ts">
  import type { MemoryStats } from "../types";
  import Card from "./ui/Card.svelte";
  import ProgressBar from "./ui/ProgressBar.svelte";
  import Sparkline from "./ui/Sparkline.svelte";
  import { stats } from "../stores/stats.svelte";
  import { formatBytes } from "../utils/format";
  import { HardDrive } from "@lucide/svelte";

  interface Props {
    memory: MemoryStats;
  }

  let { memory }: Props = $props();

  let percentUsed = $derived(() => {
    if (!memory.total_bytes || memory.total_bytes === 0) return 0;
    return (memory.used_bytes / memory.total_bytes) * 100;
  });

  let swapPercent = $derived(() => {
    if (!memory.swap_total_bytes || memory.swap_total_bytes === 0) return 0;
    return (memory.swap_used_bytes / memory.swap_total_bytes) * 100;
  });
</script>

<Card title="Memory & Swap" badge="{percentUsed().toFixed(1)}%">
  {#snippet headerAction()}
    <div class="flex items-center gap-1.5 text-xs text-purple-600 dark:text-purple-400 font-semibold">
      <HardDrive class="w-4 h-4" />
      <span>{formatBytes(memory.used_bytes)} / {formatBytes(memory.total_bytes)}</span>
    </div>
  {/snippet}

  <div class="space-y-4">
    <!-- Physical RAM -->
    <div class="flex items-center justify-between gap-4">
      <div class="flex-1">
        <div class="flex justify-between items-baseline mb-1">
          <span class="text-2xl font-black tracking-tight text-slate-800 dark:text-white">
            {formatBytes(memory.used_bytes)}
            <span class="text-xs font-normal text-slate-400 ml-1">used</span>
          </span>
          <span class="text-xs text-slate-500 dark:text-slate-400 font-mono">
            Avail: {formatBytes(memory.available_bytes)}
          </span>
        </div>
        <ProgressBar value={percentUsed()} height="h-2.5" color="bg-purple-500" />
      </div>

      {#if stats.ramHistory.length > 1}
        <div class="flex flex-col items-end">
          <span class="text-[10px] text-slate-400 font-mono mb-1">RAM Trend</span>
          <Sparkline
            data={stats.ramHistory}
            strokeColor="#a855f7"
            fillColor="rgba(168, 85, 247, 0.15)"
            width={110}
            height={32}
          />
        </div>
      {/if}
    </div>

    <!-- Swap Memory -->
    <div class="pt-2 border-t border-slate-100 dark:border-slate-800/80">
      <div class="flex justify-between items-center text-xs mb-1">
        <span class="font-medium text-slate-600 dark:text-slate-400">
          Virtual Swap Memory
        </span>
        <span class="font-mono text-slate-500 dark:text-slate-400">
          {formatBytes(memory.swap_used_bytes)} / {formatBytes(memory.swap_total_bytes)}
        </span>
      </div>
      <ProgressBar value={swapPercent()} height="h-1.5" color="bg-indigo-400" />
    </div>
  </div>
</Card>
