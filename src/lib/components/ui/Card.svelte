<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title?: string;
    subtitle?: string;
    badge?: string;
    badgeColor?: string;
    class?: string;
    children?: Snippet;
    headerAction?: Snippet;
  }

  let {
    title,
    subtitle,
    badge,
    badgeColor = "bg-blue-500/10 text-blue-600 dark:text-blue-400 border border-blue-500/20",
    class: customClass = "",
    children,
    headerAction,
  }: Props = $props();
</script>

<div
  class="bg-white/70 dark:bg-slate-900/60 backdrop-blur-md rounded-2xl p-4 border border-slate-200/70 dark:border-slate-800/80 shadow-sm transition-all duration-200 hover:shadow-md {customClass}"
>
  {#if title || headerAction}
    <div class="flex items-center justify-between mb-3">
      <div>
        {#if title}
          <div class="flex items-center gap-2">
            <h3 class="text-xs font-bold uppercase tracking-wider text-slate-500 dark:text-slate-400">
              {title}
            </h3>
            {#if badge}
              <span class="text-[10px] font-semibold px-2 py-0.5 rounded-full {badgeColor}">
                {badge}
              </span>
            {/if}
          </div>
        {/if}
        {#if subtitle}
          <p class="text-[11px] text-slate-400 dark:text-slate-500">{subtitle}</p>
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
