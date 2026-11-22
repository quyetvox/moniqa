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

<Card
  tag="// BUS-IO.05"
  title="Storage & Network Pipeline"
>
  <div class="space-y-4">
    <!-- Network Speed Telemetry -->
    <div>
      <div class="flex items-center gap-1.5 text-[10px] font-mono font-bold text-slate-500 dark:text-slate-400 uppercase tracking-widest mb-2">
        <Wifi class="w-3.5 h-3.5 text-blue-500" />
        <span>NETWORK BUS THROUGHPUT</span>
      </div>

      <div class="grid grid-cols-2 gap-2">
        <!-- Download Ingress -->
        <div class="flex items-center gap-3 bg-emerald-500/5 dark:bg-emerald-950/20 border border-emerald-500/20 dark:border-emerald-800/40 rounded p-2.5">
          <div class="p-2 bg-emerald-500/10 text-emerald-500 rounded">
            <ArrowDown class="w-4 h-4" />
          </div>
          <div>
            <div class="text-[9px] font-mono font-bold text-slate-400 dark:text-slate-500 uppercase tracking-widest">INGRESS (DL)</div>
            <div class="text-base font-black font-tabular text-emerald-600 dark:text-emerald-400">
              {formatRate(network.total_received_bytes_per_sec)}
            </div>
          </div>
        </div>

        <!-- Upload Egress -->
        <div class="flex items-center gap-3 bg-sky-500/5 dark:bg-sky-950/20 border border-sky-500/20 dark:border-sky-800/40 rounded p-2.5">
          <div class="p-2 bg-sky-500/10 text-sky-400 rounded">
            <ArrowUp class="w-4 h-4" />
          </div>
          <div>
            <div class="text-[9px] font-mono font-bold text-slate-400 dark:text-slate-500 uppercase tracking-widest">EGRESS (UL)</div>
            <div class="text-base font-black font-tabular text-sky-600 dark:text-sky-400">
              {formatRate(network.total_transmitted_bytes_per_sec)}
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Disk Partitions Matrix -->
    {#if disk.disks && disk.disks.length > 0}
      <div class="pt-3 border-t border-slate-100 dark:border-slate-800/80">
        <div class="flex items-center gap-1.5 text-[10px] font-mono font-bold text-slate-500 dark:text-slate-400 uppercase tracking-widest mb-2">
          <HardDrive class="w-3.5 h-3.5 text-indigo-500" />
          <span>STORAGE VOLUMES [{disk.disks.length}]</span>
        </div>

        <div class="space-y-3">
          {#each disk.disks as partition}
            {@const usedPct = partition.total_bytes > 0 ? (partition.used_bytes / partition.total_bytes) * 100 : 0}
            <div class="space-y-1 bg-slate-100/50 dark:bg-slate-950/40 rounded p-2 border border-slate-200/50 dark:border-slate-800/60">
              <div class="flex justify-between items-baseline text-xs font-mono">
                <span class="font-bold text-slate-700 dark:text-slate-300">
                  {partition.mount_point}
                  <span class="text-[10px] text-slate-400 font-normal ml-1">[{partition.name || "Macintosh HD"}]</span>
                </span>
                <span class="font-tabular text-slate-600 dark:text-slate-400 font-medium">
                  {formatBytes(partition.used_bytes)} / {formatBytes(partition.total_bytes)}
                </span>
              </div>
              <ProgressBar
                value={usedPct}
                height="h-2"
                segments={24}
                color="bg-indigo-500 dark:bg-indigo-400 shadow-[0_0_8px_rgba(99,102,241,0.6)]"
              />
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
</Card>
