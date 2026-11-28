<script lang="ts">
  import { stats } from "$lib/stores/stats.svelte";
  import TrayPopup from "$lib/components/TrayPopup.svelte";
</script>

<svelte:head>
  <title>MoniQA</title>
</svelte:head>

<main class="w-full p-1">
  {#if stats.current}
    <TrayPopup snapshot={stats.current} />
  {:else}
    <!-- Skeleton matching TrayPopup structure -->
    <div
      class="w-[320px] mx-auto p-3 space-y-2.5"
      style="background: var(--bg-1); border: 1px solid var(--line-strong); border-radius: var(--radius-panel);"
    >
      <div class="h-4 w-24 rounded" style="background: var(--bg-2);"></div>
      <div class="space-y-2 pt-1">
        {#each [1, 2, 3, 4, 5] as _}
          <div class="flex items-center justify-between gap-2 h-6">
            <div class="h-3 w-12 rounded" style="background: var(--bg-2);"></div>
            <div class="h-4 w-24 rounded" style="background: var(--bg-2);"></div>
            <div class="h-3 w-14 rounded" style="background: var(--bg-2);"></div>
          </div>
        {/each}
      </div>
      <div class="h-7 w-full rounded" style="background: var(--bg-2);"></div>
    </div>
  {/if}
</main>
