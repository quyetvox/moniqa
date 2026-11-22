<script lang="ts">
  interface Props {
    value: number; // 0 to 100
    color?: string;
    segments?: number;
    showPercent?: boolean;
    height?: string;
  }

  let {
    value = 0,
    color,
    segments = 20,
    showPercent = false,
    height = "h-2",
  }: Props = $props();

  let clamped = $derived(Math.min(Math.max(Number(value) || 0, 0), 100));

  let activeCount = $derived(
    segments > 0
      ? clamped > 0
        ? Math.max(1, Math.round((clamped / 100) * segments))
        : 0
      : 0
  );

  let segmentList = $derived(Array.from({ length: Math.max(segments, 1) }, (_, i) => i));

  let activeColor = $derived.by(() => {
    if (color) return color;
    if (clamped < 65) return "bg-cyan-500 dark:bg-cyan-400 shadow-[0_0_6px_rgba(6,182,212,0.8)]";
    if (clamped < 85) return "bg-amber-500 dark:bg-amber-400 shadow-[0_0_6px_rgba(245,158,11,0.8)]";
    return "bg-rose-500 dark:bg-rose-400 shadow-[0_0_6px_rgba(244,63,94,0.9)]";
  });
</script>

<div class="w-full">
  <!-- LED VU-Meter Segment Array -->
  <div
    class="flex items-center gap-[2px] w-full p-[2px] bg-slate-200/90 dark:bg-slate-950/90 rounded border border-slate-300/80 dark:border-slate-800/90 {height} min-h-[8px] box-border"
  >
    {#each segmentList as i (i)}
      {@const isActive = i < activeCount}
      <div
        class="flex-1 h-full min-h-[4px] rounded-[1px] transition-all duration-200 {isActive
          ? activeColor
          : 'bg-slate-300/80 dark:bg-slate-800/70'}"
      ></div>
    {/each}
  </div>

  {#if showPercent}
    <div class="text-right text-[11px] font-tabular font-bold text-slate-500 dark:text-slate-400 mt-1">
      {clamped.toFixed(1)}%
    </div>
  {/if}
</div>
