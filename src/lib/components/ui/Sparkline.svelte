<script lang="ts">
  interface Props {
    data: number[];
    width?: number;
    height?: number;
    strokeColor?: string;
    fillColor?: string;
  }

  let {
    data,
    width = 120,
    height = 36,
    strokeColor = "#00f2fe",
    fillColor = "rgba(0, 242, 254, 0.12)",
  }: Props = $props();

  let points = $derived(() => {
    if (!data || data.length === 0) return "";
    const min = 0;
    const max = 100;
    const stepX = width / Math.max(data.length - 1, 1);

    return data
      .map((val, idx) => {
        const x = idx * stepX;
        const normalized = (val - min) / (max - min || 1);
        const y = height - normalized * (height - 4) - 2;
        return `${x.toFixed(1)},${y.toFixed(1)}`;
      })
      .join(" ");
  });

  let fillPath = $derived(() => {
    if (!points()) return "";
    return `M 0,${height} L ${points()} L ${width},${height} Z`;
  });
</script>

<div class="relative overflow-hidden rounded bg-slate-100/70 dark:bg-slate-950/70 border border-slate-200/80 dark:border-slate-800/80 p-0.5">
  <svg {width} {height} class="overflow-visible block">
    <defs>
      <linearGradient id="grid-grad" x1="0" y1="0" x2="0" y2="1">
        <stop offset="0%" stop-color={strokeColor} stop-opacity="0.25" />
        <stop offset="100%" stop-color={strokeColor} stop-opacity="0.0" />
      </linearGradient>
    </defs>

    <!-- Oscilloscope background baseline -->
    <line x1="0" y1={height / 2} x2={width} y2={height / 2} stroke="currentColor" stroke-opacity="0.08" stroke-dasharray="2 2" />

    {#if fillPath()}
      <path d={fillPath()} fill="url(#grid-grad)" />
    {/if}
    {#if points()}
      <polyline
        points={points()}
        fill="none"
        stroke={strokeColor}
        stroke-width="1.75"
        stroke-linecap="round"
        stroke-linejoin="round"
      />
    {/if}
  </svg>
</div>
