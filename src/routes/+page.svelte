<script lang="ts">
  import { stats } from "$lib/stores/stats.svelte";
  import TrayPopup from "$lib/components/TrayPopup.svelte";
  import { Activity } from "@lucide/svelte";
</script>

<svelte:head>
  <title>MoniQA</title>
</svelte:head>

<main class="w-full">
  {#if stats.current}
    <TrayPopup snapshot={stats.current} />
  {:else}
    <div class="h-[460px] flex flex-col items-center justify-center p-6 text-center space-y-3">
      <div class="relative">
        <div class="w-10 h-10 rounded-xl bg-blue-600 flex items-center justify-center text-white font-bold text-lg shadow-md shadow-blue-500/20">
          M
        </div>
        <div class="absolute -bottom-1 -right-1 p-0.5 bg-white dark:bg-slate-900 rounded-full">
          <Activity class="w-3.5 h-3.5 text-emerald-500 animate-spin" />
        </div>
      </div>
      <div>
        <h2 class="text-xs font-bold text-slate-800 dark:text-white">Connecting...</h2>
        <p class="text-[10px] text-slate-400 mt-0.5">Reading macOS hardware metrics</p>
      </div>
    </div>
  {/if}
</main>
