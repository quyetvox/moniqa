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
    if (battery.is_charging) return "bg-emerald-500 dark:bg-emerald-400 shadow-[0_0_8px_rgba(16,185,129,0.7)]";
    if (battery.charge_percent < 20) return "bg-rose-500 dark:bg-rose-400 shadow-[0_0_8px_rgba(244,63,94,0.7)]";
    if (battery.charge_percent < 50) return "bg-amber-500 dark:bg-amber-400 shadow-[0_0_8px_rgba(245,158,11,0.7)]";
    return "bg-emerald-500 dark:bg-emerald-400 shadow-[0_0_8px_rgba(16,185,129,0.7)]";
  });
</script>

<Card
  tag="// CELL-PWR.06"
  title="Power & Cell Status"
  badge={`${battery.charge_percent.toFixed(0)}%`}
  badgeColor={battery.is_charging ? "text-emerald-500 bg-emerald-500/10 border-emerald-500/20" : "text-slate-500 bg-slate-500/10 border-slate-500/20"}
>
  {#snippet headerAction()}
    <div class="flex items-center gap-1.5 text-xs font-mono font-bold {battery.is_charging ? 'text-emerald-500' : 'text-slate-500 dark:text-slate-400'}">
      {#if battery.is_charging}
        <BatteryCharging class="w-4 h-4 animate-pulse" />
        <span>AC POWER</span>
      {:else}
        <Battery class="w-4 h-4" />
        <span>BATTERY</span>
      {/if}
    </div>
  {/snippet}

  <div class="space-y-4">
    <!-- Level Bar -->
    <div>
      <div class="flex justify-between items-baseline mb-1.5">
        <div class="flex items-baseline gap-1">
          <span class="text-3xl font-black font-tabular tracking-tight text-slate-900 dark:text-white">
            {battery.charge_percent.toFixed(0)}
          </span>
          <span class="text-xs font-mono font-bold text-slate-400">%</span>
        </div>
        {#if battery.temperature_c}
          <span class="text-[11px] font-mono px-1.5 py-0.5 rounded bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300 border border-slate-200 dark:border-slate-700/60">
            {battery.temperature_c.toFixed(1)}°C
          </span>
        {/if}
      </div>
      <ProgressBar value={battery.charge_percent} height="h-3" segments={24} color={battColor()} />
    </div>

    <!-- Health & Cycles -->
    <div class="grid grid-cols-2 gap-2 pt-3 border-t border-slate-100 dark:border-slate-800/80">
      <div class="flex items-center gap-2.5 bg-slate-100/60 dark:bg-slate-950/60 rounded p-2.5 border border-slate-200/60 dark:border-slate-800/80">
        <Heart class="w-4 h-4 text-rose-500 shrink-0" />
        <div>
          <div class="text-[9px] font-mono font-bold text-slate-400 uppercase tracking-widest">HEALTH</div>
          <div class="text-sm font-black font-tabular text-slate-800 dark:text-slate-200">
            {battery.health_percent ? `${battery.health_percent}%` : "100% OK"}
          </div>
        </div>
      </div>

      <div class="flex items-center gap-2.5 bg-slate-100/60 dark:bg-slate-950/60 rounded p-2.5 border border-slate-200/60 dark:border-slate-800/80">
        <RotateCcw class="w-4 h-4 text-cyan-500 shrink-0" />
        <div>
          <div class="text-[9px] font-mono font-bold text-slate-400 uppercase tracking-widest">CYCLES</div>
          <div class="text-sm font-black font-tabular text-slate-800 dark:text-slate-200">
            {battery.cycle_count ? `${battery.cycle_count}` : "--"}
          </div>
        </div>
      </div>
    </div>
  </div>
</Card>
