<script lang="ts">
  import { theme } from "../../stores/theme.svelte";
  import { settings } from "../../stores/settings.svelte";
</script>

<div class="space-y-4 max-w-2xl">
  <!-- Appearance -->
  <div
    class="p-3 space-y-3"
    style="background: var(--bg-1); border: 1px solid var(--line); border-radius: var(--radius-panel);"
  >
    <div class="text-[12px] font-semibold">Appearance</div>

    <div class="flex items-center justify-between py-1">
      <div>
        <div class="text-[12px]">Theme</div>
        <div class="text-[11px]" style="color: var(--text-3);">Instrument Dark or Native Quiet Light</div>
      </div>
      <div class="inline-flex text-[12px] overflow-hidden" style="border: 1px solid var(--line); border-radius: var(--radius);">
        <button
          onclick={() => { if (theme.isDark) theme.toggle(); }}
          class="px-3 py-1"
          style="background: {!theme.isDark ? 'var(--accent-soft)' : 'transparent'}; color: {!theme.isDark ? 'var(--accent)' : 'var(--text-2)'};"
        >
          Light
        </button>
        <button
          onclick={() => { if (!theme.isDark) theme.toggle(); }}
          class="px-3 py-1"
          style="border-left: 1px solid var(--line); background: {theme.isDark ? 'var(--accent-soft)' : 'transparent'}; color: {theme.isDark ? 'var(--accent)' : 'var(--text-2)'};"
        >
          Dark
        </button>
      </div>
    </div>

    <div class="flex items-center justify-between py-1" style="border-top: 1px solid var(--line);">
      <div>
        <div class="text-[12px]">Table Density</div>
        <div class="text-[11px]" style="color: var(--text-3);">Row height in process and sensor tables (24px / 28px)</div>
      </div>
      <div class="inline-flex text-[12px] overflow-hidden" style="border: 1px solid var(--line); border-radius: var(--radius);">
        <button
          onclick={() => settings.setDensity("compact")}
          class="px-3 py-1"
          style="background: {settings.density === 'compact' ? 'var(--accent-soft)' : 'transparent'}; color: {settings.density === 'compact' ? 'var(--accent)' : 'var(--text-2)'};"
        >
          Compact
        </button>
        <button
          onclick={() => settings.setDensity("comfortable")}
          class="px-3 py-1"
          style="border-left: 1px solid var(--line); background: {settings.density === 'comfortable' ? 'var(--accent-soft)' : 'transparent'}; color: {settings.density === 'comfortable' ? 'var(--accent)' : 'var(--text-2)'};"
        >
          Comfortable
        </button>
      </div>
    </div>
  </div>

  <!-- Monitoring -->
  <div
    class="p-3 space-y-3"
    style="background: var(--bg-1); border: 1px solid var(--line); border-radius: var(--radius-panel);"
  >
    <div class="text-[12px] font-semibold">Monitoring & Units</div>

    <div class="flex items-center justify-between py-1">
      <div>
        <div class="text-[12px]">Network Throughput Unit</div>
        <div class="text-[11px]" style="color: var(--text-3);">Display rates in bytes/sec (MB/s) or bits/sec (Mbps)</div>
      </div>
      <div class="inline-flex text-[12px] overflow-hidden" style="border: 1px solid var(--line); border-radius: var(--radius);">
        <button
          onclick={() => settings.setNetUnit("bytes")}
          class="px-3 py-1 font-num"
          style="background: {settings.netUnit === 'bytes' ? 'var(--accent-soft)' : 'transparent'}; color: {settings.netUnit === 'bytes' ? 'var(--accent)' : 'var(--text-2)'};"
        >
          MB/s
        </button>
        <button
          onclick={() => settings.setNetUnit("bits")}
          class="px-3 py-1 font-num"
          style="border-left: 1px solid var(--line); background: {settings.netUnit === 'bits' ? 'var(--accent-soft)' : 'transparent'}; color: {settings.netUnit === 'bits' ? 'var(--accent)' : 'var(--text-2)'};"
        >
          Mbps
        </button>
      </div>
    </div>

    <div class="flex items-center justify-between py-1" style="border-top: 1px solid var(--line);">
      <div>
        <div class="text-[12px]">Alert Thresholds</div>
        <div class="text-[11px]" style="color: var(--text-3);">Warning and Critical percentage levels for CPU & Memory</div>
      </div>
      <div class="flex items-center gap-2 font-num text-[12px]">
        <label class="flex items-center gap-1">
          <span style="color: var(--warn);">Warn</span>
          <input
            type="number"
            min="20"
            max="90"
            value={settings.warnThreshold}
            onchange={(e) => settings.setThresholds(Number(e.currentTarget.value), settings.critThreshold)}
            class="w-14 px-1.5 py-0.5 text-right"
            style="background: var(--bg-0); border: 1px solid var(--line); border-radius: var(--radius);"
          />%
        </label>
        <label class="flex items-center gap-1">
          <span style="color: var(--crit);">Crit</span>
          <input
            type="number"
            min="30"
            max="99"
            value={settings.critThreshold}
            onchange={(e) => settings.setThresholds(settings.warnThreshold, Number(e.currentTarget.value))}
            class="w-14 px-1.5 py-0.5 text-right"
            style="background: var(--bg-0); border: 1px solid var(--line); border-radius: var(--radius);"
          />%
        </label>
      </div>
    </div>
  </div>
</div>
