<script lang="ts">
  import { formatTimeShort } from "../../utils/format";

  export interface ChartSeries {
    id: string;
    label: string;
    data: number[];
    color: string;
    formatValue?: (v: number) => string;
  }

  interface Props {
    series: ChartSeries[];
    timestamps?: number[];
    height?: number;
    yMax?: number; // 0 = auto scale
    yTicks?: number;
    timeLabel?: string;
    paused?: boolean;
    formatY?: (v: number) => string;
  }

  let {
    series,
    timestamps = [],
    height = 200,
    yMax = 100,
    yTicks = 4,
    timeLabel = "60s",
    paused = false,
    formatY = (v: number) => `${Math.round(v)}%`,
  }: Props = $props();

  let containerEl = $state<HTMLDivElement | null>(null);
  let width = $state(600);
  let hoverIdx = $state<number | null>(null);

  $effect(() => {
    if (!containerEl) return;
    const ro = new ResizeObserver((entries) => {
      for (const entry of entries) {
        if (entry.contentRect.width > 0) {
          width = Math.floor(entry.contentRect.width);
        }
      }
    });
    ro.observe(containerEl);
    return () => ro.disconnect();
  });

  const padLeft = 44;
  const padRight = 10;
  const padTop = 10;
  const padBottom = 22;

  let plotW = $derived(Math.max(width - padLeft - padRight, 40));
  let plotH = $derived(Math.max(height - padTop - padBottom, 40));

  let maxLen = $derived(
    Math.max(...series.map((s) => s.data.length), 0)
  );

  let effectiveYMax = $derived.by(() => {
    if (yMax > 0) return yMax;
    let peak = 0;
    for (const s of series) {
      for (const v of s.data) {
        if (v > peak) peak = v;
      }
    }
    return peak > 0 ? peak * 1.15 : 100;
  });

  function buildPoints(data: number[]): string {
    if (data.length === 0) return "";
    if (data.length === 1) {
      const y = padTop + plotH - (Math.min(data[0], effectiveYMax) / effectiveYMax) * plotH;
      return `${padLeft},${y.toFixed(1)} ${(padLeft + plotW).toFixed(1)},${y.toFixed(1)}`;
    }
    const stepX = plotW / (data.length - 1);
    return data
      .map((val, idx) => {
        const x = padLeft + idx * stepX;
        const clamped = Math.max(0, Math.min(val, effectiveYMax));
        const y = padTop + plotH - (clamped / effectiveYMax) * plotH;
        return `${x.toFixed(1)},${y.toFixed(1)}`;
      })
      .join(" ");
  }

  function buildArea(pts: string): string {
    if (!pts) return "";
    const bottom = (padTop + plotH).toFixed(1);
    return `M ${padLeft},${bottom} L ${pts} L ${(padLeft + plotW).toFixed(1)},${bottom} Z`;
  }

  function handleMouseMove(e: MouseEvent) {
    if (maxLen < 2 || !containerEl) return;
    const rect = containerEl.getBoundingClientRect();
    const relX = e.clientX - rect.left - padLeft;
    if (relX < 0 || relX > plotW) {
      hoverIdx = null;
      return;
    }
    const idx = Math.round((relX / plotW) * (maxLen - 1));
    hoverIdx = Math.max(0, Math.min(idx, maxLen - 1));
  }

  let hoverX = $derived.by(() => {
    if (hoverIdx === null || maxLen < 2) return 0;
    return padLeft + (hoverIdx / (maxLen - 1)) * plotW;
  });

  let yGridValues = $derived(
    Array.from({ length: yTicks + 1 }, (_, i) => (effectiveYMax / yTicks) * i)
  );
</script>

<div
  bind:this={containerEl}
  class="relative w-full select-none {paused ? 'opacity-75' : ''}"
  style="height: {height}px;"
  onmousemove={handleMouseMove}
  onmouseleave={() => (hoverIdx = null)}
  role="img"
  aria-label="Resource usage chart"
>
  <svg {width} {height} class="block">
    <!-- Horizontal gridlines & Y labels -->
    {#each yGridValues as val}
      {@const y = padTop + plotH - (val / effectiveYMax) * plotH}
      <line
        x1={padLeft}
        y1={y}
        x2={padLeft + plotW}
        y2={y}
        stroke="var(--line)"
        stroke-width="1"
      />
      <text
        x={padLeft - 6}
        y={y + 3}
        text-anchor="end"
        fill="var(--text-3)"
        class="font-num text-[10px]"
      >
        {formatY(val)}
      </text>
    {/each}

    <!-- Time axis baseline labels -->
    <text
      x={padLeft}
      y={height - 5}
      text-anchor="start"
      fill="var(--text-3)"
      class="font-num text-[10px]"
    >
      -{timeLabel}
    </text>
    <text
      x={padLeft + plotW}
      y={height - 5}
      text-anchor="end"
      fill="var(--text-3)"
      class="font-num text-[10px]"
    >
      now
    </text>

    <!-- Series paths -->
    {#each series as s (s.id)}
      {@const pts = buildPoints(s.data)}
      {@const area = buildArea(pts)}
      {#if area}
        <path d={area} fill={s.color} fill-opacity="0.11" />
      {/if}
      {#if pts}
        <polyline
          points={pts}
          fill="none"
          stroke={s.color}
          stroke-width="1.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      {/if}
    {/each}

    <!-- Hover crosshair & dots -->
    {#if hoverIdx !== null && maxLen >= 2}
      <line
        x1={hoverX}
        y1={padTop}
        x2={hoverX}
        y2={padTop + plotH}
        stroke="var(--text-3)"
        stroke-width="1"
        stroke-dasharray="3 2"
      />
      {#each series as s (s.id)}
        {#if hoverIdx < s.data.length}
          {@const val = Math.max(0, Math.min(s.data[hoverIdx], effectiveYMax))}
          {@const cy = padTop + plotH - (val / effectiveYMax) * plotH}
          <circle
            cx={hoverX}
            cy={cy}
            r="3"
            fill={s.color}
            stroke="var(--bg-1)"
            stroke-width="1.5"
          />
        {/if}
      {/each}
    {/if}
  </svg>

  <!-- Hover Tooltip -->
  {#if hoverIdx !== null && maxLen >= 2}
    <div
      class="pointer-events-none absolute top-2 z-10 px-2.5 py-1.5 text-[11px] font-num"
      style="
        left: {hoverX > width * 0.65 ? Math.max(8, hoverX - 150) : Math.min(width - 155, hoverX + 10)}px;
        background: var(--bg-1);
        border: 1px solid var(--line-strong);
        border-radius: var(--radius);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.18);
      "
    >
      {#if timestamps[hoverIdx]}
        <div class="text-[10px] mb-1" style="color: var(--text-3);">
          {formatTimeShort(timestamps[hoverIdx])}
        </div>
      {/if}
      <div class="space-y-0.5">
        {#each series as s (s.id)}
          {#if hoverIdx < s.data.length}
            <div class="flex items-center justify-between gap-3">
              <span class="flex items-center gap-1.5" style="color: var(--text-2);">
                <span class="w-2 h-2 rounded-full" style="background: {s.color};"></span>
                {s.label}
              </span>
              <span class="font-semibold" style="color: var(--text-1);">
                {s.formatValue ? s.formatValue(s.data[hoverIdx]) : `${s.data[hoverIdx].toFixed(1)}%`}
              </span>
            </div>
          {/if}
        {/each}
      </div>
    </div>
  {/if}
</div>
