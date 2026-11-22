<script lang="ts">
  import type { FanStats, ThermalStats } from "../types";
  import Card from "./ui/Card.svelte";
  import { formatTemp } from "../utils/format";
  import { Thermometer, Wind } from "@lucide/svelte";

  interface Props {
    thermal: ThermalStats;
    fans: FanStats[];
  }

  let { thermal, fans }: Props = $props();

  let maxTemp = $derived(() => {
    return Math.max(thermal.cpu_temp_c, thermal.gpu_temp_c);
  });

  let tempBadgeColor = $derived(() => {
    if (maxTemp() < 60) return "text-emerald-500 bg-emerald-500/10 border-emerald-500/20";
    if (maxTemp() < 80) return "text-amber-500 bg-amber-500/10 border-amber-500/20";
    return "text-rose-500 bg-rose-500/10 border-rose-500/20";
  });
</script>

<Card
  tag="// THERMAL-MATRIX.04"
  title="Thermals & Cooling Array"
  badge={formatTemp(maxTemp())}
  badgeColor={tempBadgeColor()}
>
  {#snippet headerAction()}
    <div class="flex items-center gap-1.5 text-xs font-mono text-orange-500">
      <Thermometer class="w-3.5 h-3.5" />
      <span>{maxTemp() >= 80 ? "HOT" : "NOMINAL"}</span>
    </div>
  {/snippet}

  <div class="space-y-3">
    <!-- Temperature readings -->
    <div class="grid grid-cols-2 gap-2">
      <div class="bg-slate-100/60 dark:bg-slate-950/60 rounded p-3 border border-slate-200/60 dark:border-slate-800/80">
        <div class="flex items-center justify-between text-[10px] font-mono text-slate-400 dark:text-slate-500 uppercase tracking-wider mb-1">
          <span>ZONE CPU</span>
          <span class="w-1.5 h-1.5 rounded-full {thermal.cpu_temp_c > 75 ? 'bg-rose-500 animate-ping' : 'bg-emerald-500'}"></span>
        </div>
        <div class="text-2xl font-black font-tabular text-slate-900 dark:text-white">
          {formatTemp(thermal.cpu_temp_c)}
        </div>
      </div>

      <div class="bg-slate-100/60 dark:bg-slate-950/60 rounded p-3 border border-slate-200/60 dark:border-slate-800/80">
        <div class="flex items-center justify-between text-[10px] font-mono text-slate-400 dark:text-slate-500 uppercase tracking-wider mb-1">
          <span>ZONE GPU</span>
          <span class="w-1.5 h-1.5 rounded-full {thermal.gpu_temp_c > 75 ? 'bg-rose-500 animate-ping' : 'bg-emerald-500'}"></span>
        </div>
        <div class="text-2xl font-black font-tabular text-slate-900 dark:text-white">
          {formatTemp(thermal.gpu_temp_c)}
        </div>
      </div>
    </div>

    <!-- Fans section -->
    {#if fans && fans.length > 0}
      <div class="pt-3 border-t border-slate-100 dark:border-slate-800/80">
        <div class="flex items-center gap-1.5 text-[10px] font-mono font-bold text-slate-500 dark:text-slate-400 uppercase tracking-widest mb-2">
          <Wind class="w-3.5 h-3.5 text-cyan-400" />
          <span>TACHOMETER FANS [{fans.length}]</span>
        </div>
        <div class="space-y-1.5">
          {#each fans as fan}
            <div class="flex justify-between items-center text-xs font-mono bg-slate-100/60 dark:bg-slate-950/60 rounded px-3 py-1.5 border border-slate-200/40 dark:border-slate-800/60">
              <span class="text-slate-600 dark:text-slate-300 font-medium">{fan.name}</span>
              <span class="font-tabular font-bold text-cyan-600 dark:text-cyan-400">{fan.rpm} RPM</span>
            </div>
          {/each}
        </div>
      </div>
    {:else}
      <div class="text-[10px] font-mono text-slate-400 dark:text-slate-500 text-center py-2 bg-slate-100/40 dark:bg-slate-950/40 rounded border border-dashed border-slate-200 dark:border-slate-800">
        // PASSIVE CONVECTION RADIATOR (FANLESS)
      </div>
    {/if}
  </div>
</Card>
