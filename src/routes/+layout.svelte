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

  $effect(() => {
    theme.apply();
  });
</script>

<div class="{theme.isDark ? 'dark' : ''} min-h-screen telemetry-grid transition-colors duration-300">
  {@render children?.()}
</div>
