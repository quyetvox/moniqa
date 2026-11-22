<script lang="ts">
  import type { ProcessInfo } from "../types";
  import Card from "./ui/Card.svelte";
  import { formatBytes } from "../utils/format";
  import { ArrowDownUp, Search } from "@lucide/svelte";

  interface Props {
    processes: ProcessInfo[];
  }

  let { processes }: Props = $props();

  let searchQuery = $state("");
  let sortBy = $state<"cpu" | "memory">("cpu");

  let filtered = $derived(() => {
    let list = processes.filter((p) =>
      p.name.toLowerCase().includes(searchQuery.toLowerCase())
    );

    if (sortBy === "cpu") {
      list.sort((a, b) => b.cpu_usage - a.cpu_usage);
    } else {
      list.sort((a, b) => b.memory_bytes - a.memory_bytes);
    }

    return list;
  });
</script>

<Card title="Top Processes">
  {#snippet headerAction()}
    <div class="flex items-center gap-2">
      <!-- Search bar -->
      <div class="relative">
        <Search class="w-3.5 h-3.5 absolute left-2 top-2 text-slate-400" />
        <input
          type="text"
          placeholder="Filter..."
          bind:value={searchQuery}
          class="pl-7 pr-2 py-1 text-xs bg-slate-100 dark:bg-slate-800 rounded-lg border border-transparent focus:border-blue-500 focus:outline-none w-28 text-slate-800 dark:text-slate-200"
        />
      </div>

      <!-- Sort Toggle -->
      <button
        onclick={() => (sortBy = sortBy === "cpu" ? "memory" : "cpu")}
        class="flex items-center gap-1 text-[11px] font-medium px-2 py-1 bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 rounded-lg text-slate-600 dark:text-slate-300 transition-colors"
      >
        <ArrowDownUp class="w-3 h-3 text-slate-400" />
        <span>Sort: {sortBy.toUpperCase()}</span>
      </button>
    </div>
  {/snippet}

  <div class="overflow-x-auto -mx-4 -mb-4">
    <table class="w-full text-left text-xs">
      <thead class="bg-slate-50 dark:bg-slate-800/50 text-[10px] uppercase tracking-wider text-slate-400 border-b border-slate-100 dark:border-slate-800">
        <tr>
          <th class="py-2 px-4">PID</th>
          <th class="py-2 px-2">Process Name</th>
          <th class="py-2 px-4 text-right">CPU</th>
          <th class="py-2 px-4 text-right">Memory</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-100 dark:divide-slate-800/50">
        {#each filtered() as proc (proc.pid)}
          <tr class="hover:bg-slate-50/50 dark:hover:bg-slate-800/30 transition-colors">
            <td class="py-2 px-4 font-mono text-slate-400 text-[11px]">{proc.pid}</td>
            <td class="py-2 px-2 font-medium text-slate-800 dark:text-slate-200 truncate max-w-[160px]">
              {proc.name}
            </td>
            <td class="py-2 px-4 text-right font-mono font-semibold {proc.cpu_usage > 20 ? 'text-amber-500' : 'text-slate-700 dark:text-slate-300'}">
              {proc.cpu_usage.toFixed(1)}%
            </td>
            <td class="py-2 px-4 text-right font-mono text-slate-600 dark:text-slate-400">
              {formatBytes(proc.memory_bytes)}
            </td>
          </tr>
        {/each}
        {#if filtered().length === 0}
          <tr>
            <td colspan="4" class="py-4 text-center text-slate-400 text-xs italic">
              No matching processes
            </td>
          </tr>
        {/if}
      </tbody>
    </table>
  </div>
</Card>
