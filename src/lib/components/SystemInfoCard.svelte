<script lang="ts">
  import type { SystemInfo } from "../types";
  import Card from "./ui/Card.svelte";
  import { formatUptime } from "../utils/format";
  import { Apple, Clock, Laptop, Shield } from "@lucide/svelte";

  interface Props {
    info: SystemInfo;
  }

  let { info }: Props = $props();
</script>

<Card title="System Information">
  <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
    <!-- Chip -->
    <div class="bg-slate-50 dark:bg-slate-800/40 rounded-xl p-3 border border-slate-100 dark:border-slate-800/50">
      <div class="flex items-center gap-1.5 text-slate-400 mb-1">
        <Apple class="w-3.5 h-3.5" />
        <span class="text-[10px] font-semibold uppercase tracking-wider">Processor</span>
      </div>
      <div class="text-xs font-bold text-slate-800 dark:text-slate-100 truncate" title={info.chip_name}>
        {info.chip_name}
      </div>
    </div>

    <!-- OS -->
    <div class="bg-slate-50 dark:bg-slate-800/40 rounded-xl p-3 border border-slate-100 dark:border-slate-800/50">
      <div class="flex items-center gap-1.5 text-slate-400 mb-1">
        <Shield class="w-3.5 h-3.5" />
        <span class="text-[10px] font-semibold uppercase tracking-wider">macOS</span>
      </div>
      <div class="text-xs font-bold text-slate-800 dark:text-slate-100">
        {info.os_name} {info.os_version}
      </div>
    </div>

    <!-- Hostname -->
    <div class="bg-slate-50 dark:bg-slate-800/40 rounded-xl p-3 border border-slate-100 dark:border-slate-800/50">
      <div class="flex items-center gap-1.5 text-slate-400 mb-1">
        <Laptop class="w-3.5 h-3.5" />
        <span class="text-[10px] font-semibold uppercase tracking-wider">Device</span>
      </div>
      <div class="text-xs font-bold text-slate-800 dark:text-slate-100 truncate" title={info.hostname}>
        {info.hostname}
      </div>
    </div>

    <!-- Uptime -->
    <div class="bg-slate-50 dark:bg-slate-800/40 rounded-xl p-3 border border-slate-100 dark:border-slate-800/50">
      <div class="flex items-center gap-1.5 text-slate-400 mb-1">
        <Clock class="w-3.5 h-3.5" />
        <span class="text-[10px] font-semibold uppercase tracking-wider">Uptime</span>
      </div>
      <div class="text-xs font-bold font-mono text-emerald-600 dark:text-emerald-400">
        {formatUptime(info.uptime_secs)}
      </div>
    </div>
  </div>
</Card>
