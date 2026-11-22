<script lang="ts">
  import type { DiskStats, NetworkStats } from "../types";
  import Card from "./ui/Card.svelte";
  import ProgressBar from "./ui/ProgressBar.svelte";
  import { formatBytes, formatRate } from "../utils/format";
  import { ArrowDown, ArrowUp, HardDrive, Wifi } from "@lucide/svelte";

  interface Props {
    disk: DiskStats;
    network: NetworkStats;
  }

  let { disk, network }: Props = $props();
</script>

<Card title="Storage & Network">
  <div class="space-y-4">
    <!-- Network Speed Badges -->
    <div>
      <div class="flex items-center gap-1.5 text-xs text-slate-500 dark:text-slate-400 mb-2">
        <Wifi class="w-3.5 h-3.5 text-blue-500" />
        <span class="font-semibold uppercase tracking-wider text-[10px]">Network Throughput</span>
      </div>

      <div class="grid grid-cols-2 gap-2">
        <div class="flex items-center gap-2 bg-emerald-50/60 dark:bg-emerald-950/20 border border-emerald-200/50 dark:border-emerald-800/30 rounded-xl p-2.5">
          <div class="p-1.5 bg-emerald-500/10 text-emerald-600 dark:text-emerald-400 rounded-lg">
            <ArrowDown class="w-4 h-4" />
          </div>
          <div>
            <div class="text-[10px] text-slate-400 font-medium uppercase tracking-wider">Download</div>
            <div class="text-sm font-bold font-mono text-emerald-600 dark:text-emerald-400">
              {formatRate(network.total_received_bytes_per_sec)}
            </div>
          </div>
        </div>

        <div class="flex items-center gap-2 bg-sky-50/60 dark:bg-sky-950/20 border border-sky-200/50 dark:border-sky-800/30 rounded-xl p-2.5">
          <div class="p-1.5 bg-sky-500/10 text-sky-600 dark:text-sky-400 rounded-lg">
            <ArrowUp class="w-4 h-4" />
          </div>
          <div>
            <div class="text-[10px] text-slate-400 font-medium uppercase tracking-wider">Upload</div>
            <div class="text-sm font-bold font-mono text-sky-600 dark:text-sky-400">
              {formatRate(network.total_transmitted_bytes_per_sec)}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Disk Partitions -->
    {#if disk.disks && disk.disks.length > 0}
      <div class="pt-2 border-t border-slate-100 dark:border-slate-800/80">
        <div class="flex items-center gap-1.5 text-xs text-slate-500 dark:text-slate-400 mb-2">
          <HardDrive class="w-3.5 h-3.5 text-indigo-500" />
          <span class="font-semibold uppercase tracking-wider text-[10px]">Disk Partitions</span>
        </div>

        <div class="space-y-3">
          {#each disk.disks as partition}
            {@const usedPct = partition.total_bytes > 0 ? (partition.used_bytes / partition.total_bytes) * 100 : 0}
            <div class="space-y-1">
              <div class="flex justify-between items-baseline text-xs">
                <span class="font-medium text-slate-700 dark:text-slate-300">
                  {partition.mount_point}
                  <span class="text-[10px] text-slate-400 ml-1">({partition.name || "Macintosh HD"})</span>
                </span>
                <span class="font-mono text-slate-500 dark:text-slate-400">
                  {formatBytes(partition.used_bytes)} / {formatBytes(partition.total_bytes)}
                </span>
              </div>
              <ProgressBar value={usedPct} height="h-2" color="bg-indigo-500" />
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</Card>
