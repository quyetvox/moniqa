<script lang="ts">
  import { onMount } from "svelte";
  import type { ProcessInfo } from "../types";
  import { settings } from "../stores/settings.svelte";
  import { formatBytes, middleEllipsis } from "../utils/format";
  import StatusBadge from "./ui/StatusBadge.svelte";
  import { ArrowDown, ArrowUp, Search, X } from "@lucide/svelte";

  interface Props {
    processes: ProcessInfo[];
    totalMemoryBytes?: number;
    limit?: number;
    title?: string;
    maxBodyHeight?: string;
    onShowAll?: () => void;
  }

  let {
    processes,
    totalMemoryBytes = 16 * 1024 ** 3,
    limit,
    title = "Processes",
    maxBodyHeight,
    onShowAll,
  }: Props = $props();

  type SortKey = "pid" | "name" | "cpu" | "memory";

  let searchQuery = $state("");
  let sortKey = $state<SortKey>("cpu");
  let sortDesc = $state(true);
  let selectedPid = $state<number | null>(null);
  let searchInputEl = $state<HTMLInputElement | null>(null);

  // Context menu state
  let ctxMenu = $state<{ x: number; y: number; proc: ProcessInfo } | null>(null);
  let copiedNotice = $state<string | null>(null);

  let filtered = $derived.by(() => {
    const q = searchQuery.trim().toLowerCase();
    let list = q
      ? processes.filter(
          (p) =>
            p.name.toLowerCase().includes(q) ||
            String(p.pid).includes(q) ||
            (p.user && p.user.toLowerCase().includes(q))
        )
      : [...processes];

    list.sort((a, b) => {
      let cmp = 0;
      if (sortKey === "pid") cmp = a.pid - b.pid;
      else if (sortKey === "name") cmp = a.name.localeCompare(b.name);
      else if (sortKey === "cpu") cmp = a.cpu_usage - b.cpu_usage;
      else cmp = a.memory_bytes - b.memory_bytes;
      return sortDesc ? -cmp : cmp;
    });

    if (limit && limit > 0 && !q) {
      return list.slice(0, limit);
    }
    return list;
  });

  function toggleSort(key: SortKey) {
    if (sortKey === key) {
      sortDesc = !sortDesc;
    } else {
      sortKey = key;
      sortDesc = key !== "name";
    }
  }

  function handleContextMenu(e: MouseEvent, proc: ProcessInfo) {
    e.preventDefault();
    selectedPid = proc.pid;
    ctxMenu = { x: e.clientX, y: e.clientY, proc };
  }

  async function copyText(text: string, label: string) {
    try {
      await navigator.clipboard.writeText(text);
      copiedNotice = `Copied ${label}`;
      setTimeout(() => (copiedNotice = null), 1500);
    } catch {
      // ignore clipboard errors
    }
    ctxMenu = null;
  }

  function cpuHeatBg(cpu: number): string {
    const alpha = Math.min(0.24, Math.max(0, (cpu / 100) * 0.35));
    if (alpha < 0.01) return "transparent";
    return `color-mix(in srgb, var(--s-cpu) ${Math.round(alpha * 100)}%, transparent)`;
  }

  function memHeatBg(bytes: number): string {
    const pct = totalMemoryBytes > 0 ? (bytes / totalMemoryBytes) * 100 : 0;
    const alpha = Math.min(0.24, Math.max(0, (pct / 40) * 0.24));
    if (alpha < 0.01) return "transparent";
    return `color-mix(in srgb, var(--s-mem) ${Math.round(alpha * 100)}%, transparent)`;
  }

  onMount(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === "f") {
        e.preventDefault();
        searchInputEl?.focus();
      } else if (e.key === "Escape") {
        ctxMenu = null;
        if (document.activeElement === searchInputEl) {
          searchQuery = "";
          searchInputEl?.blur();
        }
      }
    };
    const onClick = () => {
      ctxMenu = null;
    };
    window.addEventListener("keydown", onKeyDown);
    window.addEventListener("click", onClick);
    return () => {
      window.removeEventListener("keydown", onKeyDown);
      window.removeEventListener("click", onClick);
    };
  });
</script>

<div
  class="flex flex-col overflow-hidden"
  style="
    background: var(--bg-1);
    border: 1px solid var(--line);
    border-radius: var(--radius-panel);
  "
>
  <!-- Table Toolbar -->
  <div
    class="flex items-center justify-between gap-3 px-3 py-2"
    style="border-bottom: 1px solid var(--line);"
  >
    <div class="flex items-center gap-2">
      <h2 class="text-[12px] font-semibold tracking-tight">{title}</h2>
      <span class="text-[11px] font-num" style="color: var(--text-3);">
        {filtered.length}{limit && processes.length > filtered.length ? ` of ${processes.length}` : ""}
      </span>
      {#if copiedNotice}
        <span
          class="text-[10px] px-1.5 py-0.5"
          style="color: var(--ok); background: var(--accent-soft); border-radius: var(--radius);"
        >
          {copiedNotice}
        </span>
      {/if}
    </div>

    <div class="flex items-center gap-2">
      {#if onShowAll && limit}
        <button
          onclick={onShowAll}
          class="text-[11px] font-medium hover:underline"
          style="color: var(--accent);"
        >
          View all
        </button>
      {/if}
      <div class="relative flex items-center">
        <Search class="w-3.5 h-3.5 absolute left-2 pointer-events-none" style="color: var(--text-3);" />
        <input
          bind:this={searchInputEl}
          type="text"
          placeholder="Filter (⌘F)"
          bind:value={searchQuery}
          class="pl-7 pr-6 py-1 text-[12px] w-44 focus:outline-none transition-colors duration-100"
          style="
            background: var(--bg-0);
            color: var(--text-1);
            border: 1px solid var(--line);
            border-radius: var(--radius);
          "
        />
        {#if searchQuery}
          <button
            onclick={() => (searchQuery = "")}
            class="absolute right-1.5 p-0.5 rounded hover:bg-[var(--bg-2)]"
            style="color: var(--text-3);"
            title="Clear filter"
          >
            <X class="w-3 h-3" />
          </button>
        {/if}
      </div>
    </div>
  </div>

  <!-- Table -->
  <div
    class="overflow-x-auto overflow-y-auto"
    style={maxBodyHeight ? `max-height: ${maxBodyHeight};` : ""}
  >
    <table class="w-full border-collapse text-left text-[12px]">
      <thead class="sticky top-0 z-10">
        <tr
          class="text-[11px] font-medium select-none"
          style="background: var(--bg-0); color: var(--text-3); border-bottom: 1px solid var(--line);"
        >
          <th class="px-3 py-1.5 font-medium">
            <button onclick={() => toggleSort("name")} class="inline-flex items-center gap-1 hover:text-[var(--text-1)]">
              <span>Name</span>
              {#if sortKey === "name"}
                {#if sortDesc}<ArrowDown class="w-3 h-3" />{:else}<ArrowUp class="w-3 h-3" />{/if}
              {/if}
            </button>
          </th>
          <th class="px-3 py-1.5 text-right font-medium w-20">
            <button onclick={() => toggleSort("pid")} class="inline-flex items-center gap-1 ml-auto hover:text-[var(--text-1)]">
              <span>PID</span>
              {#if sortKey === "pid"}
                {#if sortDesc}<ArrowDown class="w-3 h-3" />{:else}<ArrowUp class="w-3 h-3" />{/if}
              {/if}
            </button>
          </th>
          <th class="px-3 py-1.5 text-right font-medium w-24">
            <button onclick={() => toggleSort("cpu")} class="inline-flex items-center gap-1 ml-auto hover:text-[var(--text-1)]">
              <span>CPU %</span>
              {#if sortKey === "cpu"}
                {#if sortDesc}<ArrowDown class="w-3 h-3" />{:else}<ArrowUp class="w-3 h-3" />{/if}
              {/if}
            </button>
          </th>
          <th class="px-3 py-1.5 text-right font-medium w-28">
            <button onclick={() => toggleSort("memory")} class="inline-flex items-center gap-1 ml-auto hover:text-[var(--text-1)]">
              <span>Memory</span>
              {#if sortKey === "memory"}
                {#if sortDesc}<ArrowDown class="w-3 h-3" />{:else}<ArrowUp class="w-3 h-3" />{/if}
              {/if}
            </button>
          </th>
          <th class="px-3 py-1.5 font-medium w-28 hidden sm:table-cell">User</th>
          <th class="px-3 py-1.5 text-right font-medium w-24 hidden md:table-cell">Status</th>
        </tr>
      </thead>
      <tbody class="divide-y" style="--tw-divide-opacity: 1; border-color: var(--line);">
        {#each filtered as proc (proc.pid)}
          {@const isSelected = selectedPid === proc.pid}
          <tr
            onclick={() => (selectedPid = proc.pid)}
            oncontextmenu={(e) => handleContextMenu(e, proc)}
            class="transition-colors duration-75 cursor-default hover:bg-[var(--bg-2)]"
            style="
              height: var(--row-h);
              background: {isSelected ? 'var(--accent-soft)' : ''};
              border-color: var(--line);
            "
          >
            <td class="px-3 font-medium truncate max-w-[260px]" title={proc.name}>
              {middleEllipsis(proc.name, 38)}
            </td>
            <td class="px-3 text-right font-num" style="color: var(--text-3);">
              {proc.pid}
            </td>
            <td
              class="px-3 text-right font-num font-medium"
              style="background: {cpuHeatBg(proc.cpu_usage)};"
            >
              {proc.cpu_usage.toFixed(1)}%
            </td>
            <td
              class="px-3 text-right font-num"
              style="background: {memHeatBg(proc.memory_bytes)}; color: var(--text-2);"
            >
              {formatBytes(proc.memory_bytes)}
            </td>
            <td class="px-3 truncate max-w-[110px] hidden sm:table-cell" style="color: var(--text-3);">
              {proc.user ?? "—"}
            </td>
            <td class="px-3 text-right hidden md:table-cell">
              <StatusBadge
                value={proc.cpu_usage}
                warn={settings.warnThreshold}
                crit={settings.critThreshold}
              />
            </td>
          </tr>
        {/each}
        {#if filtered.length === 0}
          <tr>
            <td colspan="6" class="px-3 py-6 text-center text-[12px]" style="color: var(--text-3);">
              No matching processes
            </td>
          </tr>
        {/if}
      </tbody>
    </table>
  </div>
</div>

<!-- Right-click Context Menu -->
{#if ctxMenu}
  <div
    class="fixed z-50 w-44 py-1 text-[12px]"
    style="
      left: {Math.min(ctxMenu.x, (typeof window !== 'undefined' ? window.innerWidth : 1000) - 185)}px;
      top: {Math.min(ctxMenu.y, (typeof window !== 'undefined' ? window.innerHeight : 700) - 100)}px;
      background: var(--bg-1);
      border: 1px solid var(--line-strong);
      border-radius: var(--radius-panel);
      box-shadow: 0 8px 20px rgba(0, 0, 0, 0.25);
    "
    onclick={(e) => e.stopPropagation()}
    onkeydown={(e) => { if (e.key === "Escape") ctxMenu = null; }}
    tabindex="-1"
    role="menu"
  >
    <div class="px-3 py-1 text-[11px] truncate" style="color: var(--text-3); border-bottom: 1px solid var(--line);">
      {ctxMenu.proc.name} ({ctxMenu.proc.pid})
    </div>
    <button
      onclick={() => ctxMenu && copyText(String(ctxMenu.proc.pid), "PID")}
      class="w-full text-left px-3 py-1.5 hover:bg-[var(--bg-2)] transition-colors"
    >
      Copy PID
    </button>
    <button
      onclick={() => ctxMenu && copyText(ctxMenu.proc.name, "process name")}
      class="w-full text-left px-3 py-1.5 hover:bg-[var(--bg-2)] transition-colors"
    >
      Copy Process Name
    </button>
  </div>
{/if}
