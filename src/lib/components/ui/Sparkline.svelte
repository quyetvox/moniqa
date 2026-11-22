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
    strokeColor = "#3b82f6",
    fillColor = "rgba(59, 130, 246, 0.15)",
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

<div class="relative overflow-hidden rounded">
  <svg {width} {height} class="overflow-visible block">
    {#if fillPath()}
      <path d={fillPath()} fill={fillColor} />
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
