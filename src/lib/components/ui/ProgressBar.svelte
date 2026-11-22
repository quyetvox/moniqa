<script lang="ts">
  interface Props {
    value: number; // 0 to 100
    color?: string;
    segments?: number;
    showPercent?: boolean;
    height?: string;
  }

  let { value, color, segments = 24, showPercent = false, height = "h-2.5" }: Props = $props();

  let activeSegments = $derived(() => {
    const clamped = Math.min(Math.max(value, 0), 100);
    return Math.round((clamped / 100) * segments);
  });

  let segmentColor = $derived(() => {
    if (color) return color;
    if (value < 65) return "bg-cyan-500 dark:bg-cyan-400 shadow-[0_0_8px_rgba(6,182,212,0.6)]";
    if (value < 85) return "bg-amber-500 dark:bg-amber-400 shadow-[0_0_8px_rgba(245,158,11,0.6)]";
    return "bg-rose-500 dark:bg-rose-400 shadow-[0_0_8px_rgba(244,63,94,0.7)]";
  });
</script>

<div class="w-full">
  <!-- Segmented LED VU-Meter -->
  <div class="flex items-center gap-[3px] w-full p-1 bg-slate-200/80 dark:bg-slate-900/90 rounded border border-slate-300/80 dark:border-slate-800/90 {height}">
    {#each Array(segments) as _, i}
      {@const isActive = i < activeSegments()}
      <div
        class="flex-1 h-full rounded-[1px] transition-all duration-300 {isActive ? segmentColor() : 'bg-slate-300/60 dark:bg-slate-800/60'}"
      ></div>
    {/each}
  </div>

  {#if showPercent}
    <div class="text-right text-[11px] font-tabular font-bold text-slate-500 dark:text-slate-400 mt-1">
      {value.toFixed(1)}%
    </div>
  {/if}
</div>
