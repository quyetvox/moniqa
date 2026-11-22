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

<Card
  tag="// SYS-KERNEL.08"
  title="Hardware & Kernel Profile"
>
  <div class="grid grid-cols-2 sm:grid-cols-4 gap-3">
    <!-- Chip -->
    <div class="bg-slate-100/60 dark:bg-slate-950/60 rounded p-3 border border-slate-200/60 dark:border-slate-800/80">
      <div class="flex items-center gap-1.5 text-slate-400 mb-1">
        <Apple class="w-3.5 h-3.5 text-slate-500 dark:text-slate-300" />
        <span class="text-[9px] font-mono font-bold uppercase tracking-widest">ARCHITECTURE</span>
      </div>
      <div class="text-xs font-bold font-mono text-slate-900 dark:text-white truncate" title={info.chip_name}>
        {info.chip_name}
      </div>
    </div>

    <!-- OS -->
    <div class="bg-slate-100/60 dark:bg-slate-950/60 rounded p-3 border border-slate-200/60 dark:border-slate-800/80">
      <div class="flex items-center gap-1.5 text-slate-400 mb-1">
        <Shield class="w-3.5 h-3.5 text-blue-500" />
        <span class="text-[9px] font-mono font-bold uppercase tracking-widest">PLATFORM</span>
      </div>
      <div class="text-xs font-bold font-mono text-slate-900 dark:text-white">
        {info.os_name} {info.os_version}
      </div>
    </div>

    <!-- Hostname -->
    <div class="bg-slate-100/60 dark:bg-slate-950/60 rounded p-3 border border-slate-200/60 dark:border-slate-800/80">
      <div class="flex items-center gap-1.5 text-slate-400 mb-1">
        <Laptop class="w-3.5 h-3.5 text-cyan-500" />
        <span class="text-[9px] font-mono font-bold uppercase tracking-widest">HOSTNAME</span>
      </div>
      <div class="text-xs font-bold font-mono text-slate-900 dark:text-white truncate" title={info.hostname}>
        {info.hostname}
      </div>
    </div>

    <!-- Uptime -->
    <div class="bg-slate-100/60 dark:bg-slate-950/60 rounded p-3 border border-slate-200/60 dark:border-slate-800/80">
      <div class="flex items-center gap-1.5 text-slate-400 mb-1">
        <Clock class="w-3.5 h-3.5 text-emerald-500" />
        <span class="text-[9px] font-mono font-bold uppercase tracking-widest">UPTIME</span>
      </div>
      <div class="text-xs font-bold font-mono text-emerald-600 dark:text-emerald-400">
        {formatUptime(info.uptime_secs)}
      </div>
    </div>
  </div>
</Card>
