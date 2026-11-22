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

<Card
  tag="// DAEMON-PROC.07"
  title="Active Process Ledger"
  badge={`${filtered().length} TASKS`}
>
  {#snippet headerAction()}
    <div class="flex items-center gap-2">
      <!-- Search input -->
      <div class="relative">
        <Search class="w-3.5 h-3.5 absolute left-2 top-2 text-slate-400" />
        <input
          type="text"
          placeholder="Filter..."
          bind:value={searchQuery}
          class="pl-7 pr-2 py-1 text-xs font-mono bg-slate-100 dark:bg-slate-950 rounded border border-slate-200 dark:border-slate-800 focus:border-cyan-500 focus:outline-none w-32 text-slate-800 dark:text-slate-200"
        />
      </div>

      <!-- Sort Toggle -->
      <button
        onclick={() => (sortBy = sortBy === "cpu" ? "memory" : "cpu")}
        class="flex items-center gap-1 text-[10px] font-mono font-bold px-2.5 py-1 bg-slate-100 dark:bg-slate-950 hover:bg-slate-200 dark:hover:bg-slate-800 rounded border border-slate-200 dark:border-slate-800 text-slate-700 dark:text-slate-300 transition-colors"
      >
        <ArrowDownUp class="w-3 h-3 text-cyan-500" />
        <span>SORT: {sortBy.toUpperCase()}</span>
      </button>
    </div>
  {/snippet}

  <div class="overflow-x-auto -mx-4 -mb-4">
    <table class="w-full text-left text-xs font-mono">
      <thead class="bg-slate-100/70 dark:bg-slate-950/70 text-[10px] uppercase tracking-wider text-slate-400 border-b border-slate-200/80 dark:border-slate-800">
        <tr>
          <th class="py-2 px-4 w-20">PID</th>
          <th class="py-2 px-2">PROCESS IDENTIFIER</th>
          <th class="py-2 px-4 text-right w-28">CPU %</th>
          <th class="py-2 px-4 text-right w-32">RESIDENT RAM</th>
        </tr>
      </thead>
      <tbody class="divide-y divide-slate-100 dark:divide-slate-800/60 font-tabular">
        {#each filtered() as proc (proc.pid)}
          <tr class="hover:bg-slate-100/50 dark:hover:bg-slate-800/40 transition-colors">
            <td class="py-2 px-4 text-slate-400 text-[11px]">{proc.pid}</td>
            <td class="py-2 px-2 font-sans font-medium text-slate-800 dark:text-slate-200 truncate max-w-[200px]">
              {proc.name}
            </td>
            <td class="py-2 px-4 text-right font-bold {proc.cpu_usage > 20 ? 'text-amber-500' : 'text-slate-700 dark:text-slate-300'}">
              {proc.cpu_usage.toFixed(1)}%
            </td>
            <td class="py-2 px-4 text-right text-slate-600 dark:text-slate-400">
              {formatBytes(proc.memory_bytes)}
            </td>
          </tr>
        {/each}
        {#if filtered().length === 0}
          <tr>
            <td colspan="4" class="py-6 text-center text-slate-400 text-xs italic font-sans">
              No matching processes found
            </td>
          </tr>
        {/if}
      </tbody>
    </table>
  </div>
</Card>
