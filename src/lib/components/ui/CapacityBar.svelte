<script lang="ts">
  interface Props {
    value: number; // 0 to 100
    color?: string;
    height?: number;
  }

  let {
    value = 0,
    color = "var(--s-mem)",
    height = 6,
  }: Props = $props();

  let clamped = $derived(Math.min(Math.max(Number(value) || 0, 0), 100));
</script>

<div
  class="w-full overflow-hidden"
  style="height: {height}px; background: var(--bg-2); border: 1px solid var(--line); border-radius: 2px;"
>
  <div
    class="h-full transition-[width] duration-150 ease-out"
    style="width: {clamped}%; background: {color};"
  ></div>
</div>
