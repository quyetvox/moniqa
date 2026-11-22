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

  let percentUsed = $derived(
    memory.total_bytes > 0 ? (memory.used_bytes / memory.total_bytes) * 100 : 0
  );

  let swapPercent = $derived(
    memory.swap_total_bytes > 0 ? (memory.swap_used_bytes / memory.swap_total_bytes) * 100 : 0
  );
</script>

<Card
  tag="// MEM-RAM.02"
  title="Physical & Swap Memory"
  badge={`${percentUsed.toFixed(1)}%`}
  badgeColor="text-purple-600 dark:text-purple-400 bg-purple-500/10 border-purple-500/20"
>
  {#snippet headerAction()}
    <div class="flex items-center gap-1.5 text-xs font-mono text-purple-600 dark:text-purple-400 font-semibold">
      <HardDrive class="w-3.5 h-3.5" />
      <span>{formatBytes(memory.used_bytes)} / {formatBytes(memory.total_bytes)}</span>
    </div>
  {/snippet}

  <div class="space-y-4">
    <!-- Physical RAM Meter & Sparkline -->
    <div class="flex items-center justify-between gap-4">
      <div class="flex-1">
        <div class="flex justify-between items-baseline mb-1.5">
          <div class="flex items-baseline gap-1.5">
            <span class="text-3xl font-black font-tabular tracking-tight text-slate-900 dark:text-white">
              {formatBytes(memory.used_bytes)}
            </span>
            <span class="text-xs font-mono font-bold text-slate-400">ACTIVE</span>
          </div>
          <span class="text-[11px] font-mono px-1.5 py-0.5 rounded bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 border border-slate-200 dark:border-slate-700/60">
            AVAIL: {formatBytes(memory.available_bytes)}
          </span>
        </div>
        <ProgressBar
          value={percentUsed}
          height="h-3"
          segments={28}
          color="bg-purple-500 dark:bg-purple-400 shadow-[0_0_8px_rgba(168,85,247,0.6)]"
        />
      </div>

      {#if stats.ramHistory.length > 1}
        <div class="flex flex-col items-end shrink-0">
          <span class="text-[9px] text-slate-400 dark:text-slate-500 font-mono tracking-widest uppercase mb-1">
            TRACE // 30s
          </span>
          <Sparkline
            data={stats.ramHistory}
            strokeColor="#a855f7"
            fillColor="rgba(168, 85, 247, 0.15)"
            width={120}
            height={34}
          />
        </div>
      {/if}
    </div>

    <!-- Virtual Swap Partition -->
    <div class="pt-3 border-t border-slate-100 dark:border-slate-800/80">
      <div class="flex justify-between items-center text-xs mb-1.5">
        <span class="text-[10px] font-mono font-bold text-slate-500 dark:text-slate-400 uppercase tracking-widest">
          VIRTUAL SWAP POOL
        </span>
        <span class="font-mono text-xs text-slate-600 dark:text-slate-300 font-tabular font-semibold">
          {formatBytes(memory.swap_used_bytes)} / {formatBytes(memory.swap_total_bytes)}
        </span>
      </div>
      <ProgressBar
        value={swapPercent}
        height="h-2.5"
        segments={24}
        color="bg-indigo-500 dark:bg-indigo-400 shadow-[0_0_8px_rgba(99,102,241,0.6)]"
      />
    </div>
  </div>
</Card>
