<script lang="ts">
  import type { BatteryStats } from "../types";
  import Card from "./ui/Card.svelte";
  import ProgressBar from "./ui/ProgressBar.svelte";
  import { Battery, BatteryCharging, Heart, RotateCcw } from "@lucide/svelte";

  interface Props {
    battery: BatteryStats;
  }

  let { battery }: Props = $props();

  let battColor = $derived(() => {
    if (battery.is_charging) return "bg-emerald-500";
    if (battery.charge_percent < 20) return "bg-rose-500";
    if (battery.charge_percent < 50) return "bg-amber-500";
    return "bg-emerald-500";
  });
</script>

<Card title="Battery & Power" badge="{battery.charge_percent.toFixed(0)}%">
  {#snippet headerAction()}
    <div class="flex items-center gap-1.5 text-xs font-semibold {battery.is_charging ? 'text-emerald-500' : 'text-slate-600 dark:text-slate-400'}">
      {#if battery.is_charging}
        <BatteryCharging class="w-4 h-4 animate-pulse" />
        <span>Charging</span>
      {:else}
        <Battery class="w-4 h-4" />
        <span>On Battery</span>
      {/if}
    </div>
  {/snippet}

  <div class="space-y-4">
    <!-- Level Bar -->
    <div>
      <div class="flex justify-between items-baseline mb-1">
        <span class="text-2xl font-black tracking-tight text-slate-800 dark:text-white">
          {battery.charge_percent.toFixed(0)}<span class="text-sm font-medium text-slate-400 ml-0.5">%</span>
        </span>
        {#if battery.temperature_c}
          <span class="text-xs text-slate-500 font-mono">
            {battery.temperature_c.toFixed(1)}°C
          </span>
        {/if}
      </div>
      <ProgressBar value={battery.charge_percent} height="h-2.5" color={battColor()} />
    </div>

    <!-- Health & Cycles -->
    <div class="grid grid-cols-2 gap-2 pt-2 border-t border-slate-100 dark:border-slate-800/80">
      <div class="flex items-center gap-2 bg-slate-50 dark:bg-slate-800/40 rounded-xl p-2.5">
        <Heart class="w-4 h-4 text-rose-500" />
        <div>
          <div class="text-[10px] text-slate-400 uppercase tracking-wider">Health</div>
          <div class="text-sm font-bold font-mono text-slate-700 dark:text-slate-200">
            {battery.health_percent ? `${battery.health_percent}%` : "Normal"}
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2 bg-slate-50 dark:bg-slate-800/40 rounded-xl p-2.5">
        <RotateCcw class="w-4 h-4 text-blue-500" />
        <div>
          <div class="text-[10px] text-slate-400 uppercase tracking-wider">Cycle Count</div>
          <div class="text-sm font-bold font-mono text-slate-700 dark:text-slate-200">
            {battery.cycle_count ? `${battery.cycle_count}` : "--"}
          </div>
        </div>
      </div>
    </div>
  </div>
</Card>
