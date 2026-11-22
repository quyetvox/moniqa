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
    if (maxTemp() < 60) return "bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 border border-emerald-500/20";
    if (maxTemp() < 80) return "bg-amber-500/10 text-amber-600 dark:text-amber-400 border border-amber-500/20";
    return "bg-rose-500/10 text-rose-600 dark:text-rose-400 border border-rose-500/20";
  });
</script>

<Card title="Thermals & Cooling" badge="{formatTemp(maxTemp())}" badgeColor={tempBadgeColor()}>
  {#snippet headerAction()}
    <Thermometer class="w-4 h-4 text-orange-500" />
  {/snippet}

  <div class="space-y-3">
    <!-- Temperature readings -->
    <div class="grid grid-cols-2 gap-2">
      <div class="bg-slate-50/80 dark:bg-slate-800/40 rounded-xl p-2.5 border border-slate-100 dark:border-slate-800/50">
        <span class="text-[11px] font-medium text-slate-500 dark:text-slate-400">CPU Core</span>
        <div class="text-xl font-bold font-mono text-slate-800 dark:text-white mt-0.5">
          {formatTemp(thermal.cpu_temp_c)}
        </div>
      </div>

      <div class="bg-slate-50/80 dark:bg-slate-800/40 rounded-xl p-2.5 border border-slate-100 dark:border-slate-800/50">
        <span class="text-[11px] font-medium text-slate-500 dark:text-slate-400">GPU Core</span>
        <div class="text-xl font-bold font-mono text-slate-800 dark:text-white mt-0.5">
          {formatTemp(thermal.gpu_temp_c)}
        </div>
      </div>
    </div>

    <!-- Fans section -->
    {#if fans && fans.length > 0}
      <div class="pt-2 border-t border-slate-100 dark:border-slate-800/80">
        <div class="flex items-center gap-1.5 text-xs text-slate-500 dark:text-slate-400 mb-2">
          <Wind class="w-3.5 h-3.5 text-sky-400" />
          <span class="font-semibold uppercase tracking-wider text-[10px]">Cooling Fans</span>
        </div>
        <div class="space-y-1.5">
          {#each fans as fan}
            <div class="flex justify-between items-center text-xs bg-slate-50 dark:bg-slate-800/50 rounded-lg px-2.5 py-1">
              <span class="text-slate-600 dark:text-slate-300">{fan.name}</span>
              <span class="font-mono font-semibold text-slate-800 dark:text-white">{fan.rpm} RPM</span>
            </div>
          {/each}
        </div>
      </div>
    {:else}
      <div class="text-[11px] text-slate-400 dark:text-slate-500 italic text-center py-1">
        Fanless Passive Cooling (Air)
      </div>
    {/if}
  </div>
</Card>
