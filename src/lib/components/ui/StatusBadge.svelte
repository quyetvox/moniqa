<script lang="ts">
  interface Props {
    value: number;
    warn?: number;
    crit?: number;
    showOk?: boolean;
  }

  let {
    value,
    warn = 70,
    crit = 90,
    showOk = false,
  }: Props = $props();

  let level = $derived.by(() => {
    if (value >= crit) return "crit";
    if (value >= warn) return "warn";
    return "ok";
  });
</script>

{#if level === "crit"}
  <span
    class="inline-flex items-center gap-1 px-1.5 py-0.5 text-[10px] font-medium"
    style="color: var(--crit); background: var(--crit-bg); border-radius: var(--radius);"
  >
    <span class="w-1.5 h-1.5 rounded-full" style="background: var(--crit);"></span>
    Critical
  </span>
{:else if level === "warn"}
  <span
    class="inline-flex items-center gap-1 px-1.5 py-0.5 text-[10px] font-medium"
    style="color: var(--warn); background: var(--warn-bg); border-radius: var(--radius);"
  >
    <span class="w-1.5 h-1.5 rounded-full" style="background: var(--warn);"></span>
    High
  </span>
{:else if showOk}
  <span class="inline-flex items-center gap-1 text-[10px]" style="color: var(--text-3);">
    Normal
  </span>
{/if}
