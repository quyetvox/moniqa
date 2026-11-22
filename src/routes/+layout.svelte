<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { stats } from "$lib/stores/stats.svelte";
  import { theme } from "$lib/stores/theme.svelte";

  let { children } = $props();

  onMount(() => {
    theme.apply();
    stats.init();

    return () => {
      stats.destroy();
    };
  });
</script>

<div class="min-h-screen bg-slate-50 dark:bg-[#0b0f19] text-slate-800 dark:text-slate-100 transition-colors duration-200">
  {@render children?.()}
</div>
