<script lang="ts">
  import { stats } from "$lib/stores/stats.svelte";
  import Dashboard from "$lib/components/Dashboard.svelte";
  import { Activity } from "@lucide/svelte";
</script>

<svelte:head>
  <title>MoniQA — macOS Resource Monitor</title>
</svelte:head>

<main class="w-full">
  {#if stats.current}
    <Dashboard snapshot={stats.current} />
  {:else}
    <div class="min-h-screen flex flex-col items-center justify-center p-8 text-center space-y-4">
      <div class="relative">
        <div class="w-14 h-14 rounded-2xl bg-blue-600 flex items-center justify-center text-white font-black text-2xl shadow-lg shadow-blue-500/30">
          M
        </div>
        <div class="absolute -bottom-1 -right-1 p-1 bg-white dark:bg-slate-900 rounded-full">
          <Activity class="w-4 h-4 text-emerald-500 animate-spin" />
        </div>
      </div>
      <div>
        <h2 class="text-base font-bold text-slate-800 dark:text-white">Connecting to Telemetry...</h2>
        <p class="text-xs text-slate-400 mt-0.5">Initializing macOS hardware collectors & metrics engine</p>
      </div>
    </div>
  {/if}
</main>
