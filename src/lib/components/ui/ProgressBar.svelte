<script lang="ts">
  interface Props {
    value: number; // 0 to 100
    color?: string;
    showPercent?: boolean;
    height?: string;
  }

  let { value, color, showPercent = false, height = "h-2" }: Props = $props();

  let autoColor = $derived(() => {
    if (color) return color;
    if (value < 60) return "bg-emerald-500";
    if (value < 85) return "bg-amber-500";
    return "bg-rose-500";
  });
</script>

<div class="w-full">
  <div class="w-full bg-slate-200/60 dark:bg-slate-700/50 rounded-full overflow-hidden {height} backdrop-blur-sm">
    <div
      class="{autoColor()} h-full rounded-full transition-all duration-500 ease-out"
      style="width: {Math.min(Math.max(value, 0), 100)}%"
    ></div>
  </div>
  {#if showPercent}
    <div class="text-right text-xs font-medium text-slate-500 dark:text-slate-400 mt-1">
      {value.toFixed(1)}%
    </div>
  {/if}
</div>
