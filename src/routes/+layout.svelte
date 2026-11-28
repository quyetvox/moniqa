<script lang="ts">
  import "../app.css";
  import { onMount } from "svelte";
  import { stats } from "$lib/stores/stats.svelte";
  import { theme } from "$lib/stores/theme.svelte";
  import { settings } from "$lib/stores/settings.svelte";

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

<div
  class="{theme.isDark ? 'dark' : ''} {settings.density === 'comfortable' ? 'density-comfortable' : 'density-compact'} min-h-screen"
  style="background: var(--bg-0); color: var(--text-1);"
>
  {@render children?.()}
</div>
