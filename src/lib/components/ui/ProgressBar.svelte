<script lang="ts">
  interface Props {
    value: number; // 0 to 100
    color?: string; // CSS color string e.g. "#06b6d4"
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

  // Return a plain CSS hex/rgb color — no Tailwind classes (they get purged when dynamic)
  let activeColorHex = $derived.by(() => {
    if (color) return color;
    if (clamped < 65) return "#06b6d4"; // cyan-500
    if (clamped < 85) return "#f59e0b"; // amber-500
    return "#f43f5e"; // rose-500
  });

  // Glow shadow string
  let glowShadow = $derived.by(() => {
    if (color) return `0 0 6px ${color}99`;
    if (clamped < 65) return "0 0 6px rgba(6,182,212,0.8)";
    if (clamped < 85) return "0 0 6px rgba(245,158,11,0.8)";
    return "0 0 6px rgba(244,63,94,0.9)";
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
        class="flex-1 h-full min-h-[4px] rounded-[1px] transition-all duration-200"
        style={isActive
          ? `background-color: ${activeColorHex}; box-shadow: ${glowShadow};`
          : "background-color: rgba(100,116,139,0.3);"}
      ></div>
    {/each}
  </div>

  {#if showPercent}
    <div class="text-right text-[11px] font-tabular font-bold text-slate-500 dark:text-slate-400 mt-1">
      {clamped.toFixed(1)}%
    </div>
  {/if}
</div>
