import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { SensorBounds, SystemSnapshot } from "../types";
import { settings } from "./settings.svelte";

const MAX_HISTORY = 300;

class StatsStore {
  current = $state<SystemSnapshot | null>(null);
  connected = $state(false);
  isMockMode = $state(false);

  cpuHistory = $state<number[]>([]);
  ramHistory = $state<number[]>([]);
  gpuHistory = $state<number[]>([]);
  diskReadHistory = $state<number[]>([]);
  diskWriteHistory = $state<number[]>([]);
  netRxHistory = $state<number[]>([]);
  netTxHistory = $state<number[]>([]);
  timestamps = $state<number[]>([]);

  sensorBounds = $state<Record<string, SensorBounds>>({});

  private unlisten: UnlistenFn | null = null;
  private mockTimer: ReturnType<typeof setInterval> | null = null;

  async init() {
    if (this.unlisten || this.mockTimer) return;

    const isTauri = typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

    if (isTauri) {
      try {
        this.unlisten = await listen<SystemSnapshot>("system-stats", (event) => {
          this.ingest(event.payload);
        });
        return;
      } catch (e) {
        console.warn("Tauri event listen unavailable:", e);
      }
    }

    // Browser preview fallback when running outside Tauri
    this.isMockMode = true;
    this.ingest(createMockSnapshot(0));
    let tick = 1;
    this.mockTimer = setInterval(() => {
      this.ingest(createMockSnapshot(tick++));
    }, settings.intervalMs);
  }

  private ingest(snapshot: SystemSnapshot) {
    if (settings.paused) return;

    this.current = snapshot;
    this.connected = true;

    const cpuVal = snapshot.cpu.global_usage;
    const ramVal =
      snapshot.memory.total_bytes > 0
        ? (snapshot.memory.used_bytes / snapshot.memory.total_bytes) * 100
        : 0;
    const gpuVal = snapshot.gpu.usage_percent;

    this.cpuHistory = [...this.cpuHistory.slice(-(MAX_HISTORY - 1)), cpuVal];
    this.ramHistory = [...this.ramHistory.slice(-(MAX_HISTORY - 1)), ramVal];
    this.gpuHistory = [...this.gpuHistory.slice(-(MAX_HISTORY - 1)), gpuVal];
    this.diskReadHistory = [
      ...this.diskReadHistory.slice(-(MAX_HISTORY - 1)),
      snapshot.disk.read_bytes_per_sec,
    ];
    this.diskWriteHistory = [
      ...this.diskWriteHistory.slice(-(MAX_HISTORY - 1)),
      snapshot.disk.write_bytes_per_sec,
    ];
    this.netRxHistory = [
      ...this.netRxHistory.slice(-(MAX_HISTORY - 1)),
      snapshot.network.total_received_bytes_per_sec,
    ];
    this.netTxHistory = [
      ...this.netTxHistory.slice(-(MAX_HISTORY - 1)),
      snapshot.network.total_transmitted_bytes_per_sec,
    ];
    this.timestamps = [...this.timestamps.slice(-(MAX_HISTORY - 1)), Date.now()];

    this.updateBound("cpu_temp", snapshot.thermal.cpu_temp_c);
    this.updateBound("gpu_temp", snapshot.thermal.gpu_temp_c);
    if (snapshot.thermal.package_temp_c != null) {
      this.updateBound("pkg_temp", snapshot.thermal.package_temp_c);
    }
    if (snapshot.thermal.battery_temp_c != null) {
      this.updateBound("bat_temp", snapshot.thermal.battery_temp_c);
    }
    for (const fan of snapshot.fans) {
      this.updateBound(`fan_${fan.index}`, fan.rpm);
    }
  }

  private updateBound(key: string, val: number) {
    if (!val || val <= 0) return;
    const prev = this.sensorBounds[key];
    if (!prev) {
      this.sensorBounds[key] = { min: val, max: val };
    } else {
      if (val < prev.min || val > prev.max) {
        this.sensorBounds[key] = {
          min: Math.min(prev.min, val),
          max: Math.max(prev.max, val),
        };
      }
    }
  }

  destroy() {
    if (this.unlisten) {
      this.unlisten();
      this.unlisten = null;
    }
    if (this.mockTimer) {
      clearInterval(this.mockTimer);
      this.mockTimer = null;
    }
  }
}

function createMockSnapshot(tick: number): SystemSnapshot {
  const jitter = (base: number, amp: number, phase = 0) =>
    Math.max(0.2, Math.min(99.5, base + Math.sin((tick + phase) * 0.45) * amp + ((tick * 17 + phase * 31) % 7) * 0.4));

  const cpu = jitter(23.7, 8.5, 1);
  const ramUsed = 11.4 * 1024 ** 3 + Math.sin(tick * 0.15) * 0.25 * 1024 ** 3;
  const ramTotal = 31.9 * 1024 ** 3;
  const gpu = jitter(11.3, 6.2, 3);

  return {
    timestamp: Math.floor(Date.now() / 1000),
    cpu: {
      global_usage: cpu,
      per_core: Array.from({ length: 10 }, (_, i) => jitter(i < 4 ? 14 : 28, 12, i)),
      frequency_mhz: Array.from({ length: 10 }, (_, i) => (i < 4 ? 2064 : 3228)),
      core_count: 10,
      thread_count: 10,
      model_name: "Apple M3 Pro",
    },
    memory: {
      used_bytes: ramUsed,
      total_bytes: ramTotal,
      available_bytes: ramTotal - ramUsed,
      swap_used_bytes: 1.42 * 1024 ** 3,
      swap_total_bytes: 4.0 * 1024 ** 3,
    },
    gpu: {
      usage_percent: gpu,
      power_watts: 3.4 + (gpu / 100) * 14.2,
      frequency_mhz: 980,
      vram_used_bytes: 1.85 * 1024 ** 3,
      vram_total_bytes: 16 * 1024 ** 3,
    },
    thermal: {
      cpu_temp_c: 54.2 + Math.sin(tick * 0.2) * 3.1,
      gpu_temp_c: 48.6 + Math.sin(tick * 0.25) * 2.4,
      package_temp_c: 56.8,
      battery_temp_c: 31.4,
    },
    disk: {
      read_bytes_per_sec: Math.round(jitter(4.1, 3.2, 2) * 1024 * 1024),
      write_bytes_per_sec: Math.round(jitter(1.8, 1.4, 5) * 1024 * 1024),
      disks: [
        {
          name: "Macintosh HD",
          mount_point: "/",
          total_bytes: 926.4 * 1024 ** 3,
          used_bytes: 718.9 * 1024 ** 3,
          available_bytes: 207.5 * 1024 ** 3,
        },
        {
          name: "Data Volume",
          mount_point: "/System/Volumes/Data",
          total_bytes: 926.4 * 1024 ** 3,
          used_bytes: 412.3 * 1024 ** 3,
          available_bytes: 514.1 * 1024 ** 3,
        },
      ],
    },
    network: {
      total_received_bytes_per_sec: Math.round(jitter(2.4, 1.9, 4) * 1024 * 1024),
      total_transmitted_bytes_per_sec: Math.round(jitter(0.35, 0.28, 7) * 1024 * 1024),
      interfaces: [
        { name: "en0 (Wi-Fi)", rx_bytes_per_sec: 2_310_400, tx_bytes_per_sec: 342_100 },
        { name: "lo0", rx_bytes_per_sec: 48_200, tx_bytes_per_sec: 48_200 },
      ],
    },
    battery: {
      has_battery: true,
      charge_percent: 82,
      is_charging: false,
      health_percent: 94,
      cycle_count: 147,
      temperature_c: 31.4,
    },
    fans: [{ index: 0, name: "Left Exhaust", rpm: 1420 }],
    top_processes: [
      { pid: 412, name: "WindowServer", cpu_usage: jitter(9.8, 2.5, 1), memory_bytes: 842 * 1024 ** 2, user: "_windowserver" },
      { pid: 1842, name: "Code Helper (Renderer)", cpu_usage: jitter(7.4, 3.1, 2), memory_bytes: 1.48 * 1024 ** 3, user: "voquyt" },
      { pid: 904, name: "Google Chrome", cpu_usage: jitter(4.6, 2.0, 3), memory_bytes: 2.14 * 1024 ** 3, user: "voquyt" },
      { pid: 2219, name: "docker-desktop", cpu_usage: jitter(3.2, 1.1, 4), memory_bytes: 1.92 * 1024 ** 3, user: "voquyt" },
      { pid: 3104, name: "node", cpu_usage: jitter(2.1, 0.9, 5), memory_bytes: 384 * 1024 ** 2, user: "voquyt" },
      { pid: 781, name: "postgres", cpu_usage: 0.7, memory_bytes: 218 * 1024 ** 2, user: "postgres" },
      { pid: 112, name: "kernel_task", cpu_usage: 1.9, memory_bytes: 512 * 1024 ** 2, user: "root" },
      { pid: 551, name: "Finder", cpu_usage: 0.3, memory_bytes: 146 * 1024 ** 2, user: "voquyt" },
    ],
    system_info: {
      hostname: "macbook-pro.local",
      os_name: "macOS",
      os_version: "15.2",
      kernel_version: "24.2.0",
      uptime_secs: 3 * 86400 + 4 * 3600 + 18 * 60 + tick,
      chip_name: "Apple M3 Pro",
    },
  };
}

export const stats = new StatsStore();
