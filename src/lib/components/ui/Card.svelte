<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title?: string;
    tag?: string;
    badge?: string;
    badgeColor?: string;
    class?: string;
    children?: Snippet;
    headerAction?: Snippet;
  }

  let {
    title,
    tag,
    badge,
    badgeColor = "text-cyan-600 dark:text-cyan-400 bg-cyan-500/10 border-cyan-500/20",
    class: customClass = "",
    children,
    headerAction,
  }: Props = $props();
</script>

<div
  class="relative bg-white/80 dark:bg-slate-900/70 backdrop-blur-md rounded-xl p-4 border border-slate-200/90 dark:border-slate-800/90 shadow-sm transition-all duration-200 hover:border-slate-300 dark:hover:border-slate-700/80 {customClass}"
>
  <!-- Technical corner highlight markings -->
  <div class="absolute top-0 left-0 w-2 h-2 border-t border-l border-slate-400/40 dark:border-slate-600/40 rounded-tl-sm pointer-events-none"></div>
  <div class="absolute top-0 right-0 w-2 h-2 border-t border-r border-slate-400/40 dark:border-slate-600/40 rounded-tr-sm pointer-events-none"></div>
  <div class="absolute bottom-0 left-0 w-2 h-2 border-b border-l border-slate-400/40 dark:border-slate-600/40 rounded-bl-sm pointer-events-none"></div>
  <div class="absolute bottom-0 right-0 w-2 h-2 border-b border-r border-slate-400/40 dark:border-slate-600/40 rounded-br-sm pointer-events-none"></div>

  {#if title || headerAction}
    <div class="flex items-center justify-between mb-3.5 pb-2 border-b border-slate-100 dark:border-slate-800/80">
      <div class="flex items-center gap-2">
        {#if tag}
          <span class="text-[9px] font-mono tracking-wider font-bold text-slate-400 dark:text-slate-500 uppercase px-1 py-0.5 bg-slate-100 dark:bg-slate-800 rounded">
            {tag}
          </span>
        {/if}
        {#if title}
          <h3 class="text-xs font-bold uppercase tracking-wider text-slate-700 dark:text-slate-300">
            {title}
          </h3>
        {/if}
        {#if badge}
          <span class="text-[10px] font-tabular font-bold px-2 py-0.5 rounded border {badgeColor}">
            {badge}
          </span>
        {/if}
      </div>
      {#if headerAction}
        {@render headerAction()}
      {/if}
    </div>
  {/if}

  {#if children}
    {@render children()}
  {/if}
</div>
