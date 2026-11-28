<script lang="ts">
  import { stats } from "$lib/stores/stats.svelte";
  import Dashboard from "$lib/components/Dashboard.svelte";
</script>

<svelte:head>
  <title>MoniQA — System Monitor</title>
</svelte:head>

<main class="w-full">
  {#if stats.current}
    <Dashboard snapshot={stats.current} />
  {:else}
    <!-- Skeleton matching Sidebar + Content layout -->
    <div class="h-screen flex" style="background: var(--bg-0);">
      <div class="w-[176px] p-3 space-y-2" style="background: var(--bg-1); border-right: 1px solid var(--line);">
        <div class="h-5 w-20 rounded mb-4" style="background: var(--bg-2);"></div>
        {#each [1, 2, 3, 4, 5, 6] as _}
          <div class="h-7 w-full rounded" style="background: var(--bg-2);"></div>
        {/each}
      </div>
      <div class="flex-1 p-4 space-y-4">
        <div class="h-16 w-full rounded" style="background: var(--bg-1); border: 1px solid var(--line);"></div>
        <div class="h-52 w-full rounded" style="background: var(--bg-1); border: 1px solid var(--line);"></div>
        <div class="h-64 w-full rounded" style="background: var(--bg-1); border: 1px solid var(--line);"></div>
      </div>
    </div>
  {/if}
</main>
