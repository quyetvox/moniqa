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

<Card title="GPU & Graphics" badge="{gpu.usage_percent.toFixed(1)}%">
  {#snippet headerAction()}
    <div class="flex items-center gap-1.5 text-xs text-amber-500 font-semibold">
      <Zap class="w-4 h-4" />
      <span>{gpu.power_watts.toFixed(1)} W</span>
    </div>
  {/snippet}

  <div class="space-y-4">
    <div class="flex items-center justify-between gap-4">
      <div class="flex-1">
        <div class="flex justify-between items-baseline mb-1">
          <span class="text-2xl font-black tracking-tight text-slate-800 dark:text-white">
            {gpu.usage_percent.toFixed(1)}<span class="text-sm font-medium text-slate-400 ml-0.5">%</span>
          </span>
          <span class="text-xs text-slate-500 dark:text-slate-400 font-mono">
            Power: {gpu.power_watts.toFixed(2)} W
          </span>
        </div>
        <ProgressBar value={gpu.usage_percent} height="h-2.5" color="bg-amber-500" />
      </div>

      {#if stats.gpuHistory.length > 1}
        <div class="flex flex-col items-end">
          <span class="text-[10px] text-slate-400 font-mono mb-1">GPU Activity</span>
          <Sparkline
            data={stats.gpuHistory}
            strokeColor="#f59e0b"
            fillColor="rgba(245, 158, 11, 0.15)"
            width={110}
            height={32}
          />
        </div>
      {/if}
    </div>

    <!-- Unified Memory allocated to GPU -->
    {#if gpu.vram_used_bytes > 0}
      <div class="pt-2 border-t border-slate-100 dark:border-slate-800/80">
        <div class="flex justify-between text-xs text-slate-500 dark:text-slate-400 mb-1">
          <span>Unified Memory Allocation</span>
          <span class="font-mono">~{formatBytes(gpu.vram_used_bytes)}</span>
        </div>
      </div>
    {/if}
  </div>
</Card>
