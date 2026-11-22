<script lang="ts">
  import type { GpuStats } from "../types";
  import Card from "./ui/Card.svelte";
  import ProgressBar from "./ui/ProgressBar.svelte";
  import Sparkline from "./ui/Sparkline.svelte";
  import { stats } from "../stores/stats.svelte";
  import { formatBytes } from "../utils/format";
  import { Zap } from "@lucide/svelte";

  interface Props {
    gpu: GpuStats;
  }

  let { gpu }: Props = $props();
</script>

<Card
  tag="// METAL-GPU.03"
  title="Metal Graphics & Compute"
  badge={`${gpu.usage_percent.toFixed(1)}%`}
  badgeColor="text-amber-600 dark:text-amber-400 bg-amber-500/10 border-amber-500/20"
>
  {#snippet headerAction()}
    <div class="flex items-center gap-1.5 text-xs font-mono text-amber-500 font-bold">
      <Zap class="w-3.5 h-3.5" />
      <span>{gpu.power_watts.toFixed(2)} W</span>
    </div>
  {/snippet}

  <div class="space-y-4">
    <div class="flex items-center justify-between gap-4">
      <div class="flex-1">
        <div class="flex justify-between items-baseline mb-1.5">
          <div class="flex items-baseline gap-1">
            <span class="text-3xl font-black font-tabular tracking-tight text-slate-900 dark:text-white">
              {gpu.usage_percent.toFixed(1)}
            </span>
            <span class="text-xs font-mono font-bold text-amber-500">%</span>
          </div>
          <span class="text-[11px] font-mono px-1.5 py-0.5 rounded bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 border border-slate-200 dark:border-slate-700/60">
            PWR: {gpu.power_watts.toFixed(1)} W
          </span>
        </div>
        <ProgressBar
          value={gpu.usage_percent}
          height="h-3"
          segments={28}
          color="bg-amber-500 dark:bg-amber-400 shadow-[0_0_8px_rgba(245,158,11,0.6)]"
        />
      </div>

      {#if stats.gpuHistory.length > 1}
        <div class="flex flex-col items-end shrink-0">
          <span class="text-[9px] text-slate-400 dark:text-slate-500 font-mono tracking-widest uppercase mb-1">
            TRACE // 30s
          </span>
          <Sparkline
            data={stats.gpuHistory}
            strokeColor="#f59e0b"
            fillColor="rgba(245, 158, 11, 0.15)"
            width={120}
            height={34}
          />
        </div>
      {/if}
    </div>

    <!-- Unified Memory allocated to GPU -->
    {#if gpu.vram_used_bytes > 0}
      <div class="pt-3 border-t border-slate-100 dark:border-slate-800/80">
        <div class="flex justify-between text-xs text-slate-500 dark:text-slate-400 mb-1">
          <span class="text-[10px] font-mono font-bold uppercase tracking-widest">
            UNIFIED MEMORY ALLOCATION
          </span>
          <span class="font-mono font-semibold text-slate-700 dark:text-slate-300">
            ~{formatBytes(gpu.vram_used_bytes)}
          </span>
        </div>
      </div>
    {/if}
  </div>
</Card>
