<script lang="ts">
  interface Props {
    data: number[];
    width?: number;
    height?: number;
    color?: string;
    max?: number;
  }

  let {
    data,
    width = 96,
    height = 24,
    color = "var(--s-cpu)",
    max = 100,
  }: Props = $props();

  let effectiveMax = $derived.by(() => {
    if (max > 0) return max;
    const peak = Math.max(...data, 1);
    return peak * 1.15;
  });

  let points = $derived.by(() => {
    if (!data || data.length === 0) return "";
    if (data.length === 1) {
      const y = height - (Math.min(data[0], effectiveMax) / effectiveMax) * (height - 4) - 2;
      return `0,${y.toFixed(1)} ${width},${y.toFixed(1)}`;
    }
    const stepX = width / (data.length - 1);
    return data
      .map((val, idx) => {
        const x = idx * stepX;
        const clamped = Math.max(0, Math.min(val, effectiveMax));
        const y = height - (clamped / effectiveMax) * (height - 4) - 2;
        return `${x.toFixed(1)},${y.toFixed(1)}`;
      })
      .join(" ");
  });

  let fillPath = $derived.by(() => {
    if (!points) return "";
    return `M 0,${height} L ${points} L ${width},${height} Z`;
  });
</script>

<svg
  {width}
  {height}
  viewBox="0 0 {width} {height}"
  class="block shrink-0 overflow-hidden"
  aria-hidden="true"
>
  <line
    x1="0"
    y1={height / 2}
    x2={width}
    y2={height / 2}
    stroke="var(--line)"
    stroke-width="1"
    stroke-dasharray="2 2"
  />
  {#if fillPath}
    <path d={fillPath} fill={color} fill-opacity="0.14" />
  {/if}
  {#if points}
    <polyline
      {points}
      fill="none"
      stroke={color}
      stroke-width="1.25"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  {/if}
</svg>
