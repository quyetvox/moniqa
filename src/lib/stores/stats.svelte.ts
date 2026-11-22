import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { SystemSnapshot } from "../types";

class StatsStore {
  current = $state<SystemSnapshot | null>(null);
  connected = $state(false);
  cpuHistory = $state<number[]>([]);
  ramHistory = $state<number[]>([]);
  gpuHistory = $state<number[]>([]);
  private unlisten: UnlistenFn | null = null;

  async init() {
    if (this.unlisten) return;

    try {
      this.unlisten = await listen<SystemSnapshot>("system-stats", (event) => {
        this.current = event.payload;
        this.connected = true;

        const cpuVal = event.payload.cpu.global_usage;
        const ramVal = (event.payload.memory.used_bytes / (event.payload.memory.total_bytes || 1)) * 100;
        const gpuVal = event.payload.gpu.usage_percent;

        this.cpuHistory = [...this.cpuHistory.slice(-29), cpuVal];
        this.ramHistory = [...this.ramHistory.slice(-29), ramVal];
        this.gpuHistory = [...this.gpuHistory.slice(-29), gpuVal];
      });
    } catch (e) {
      console.warn("Tauri event listen unavailable (e.g. running in browser preview):", e);
    }
  }

  destroy() {
    if (this.unlisten) {
      this.unlisten();
      this.unlisten = null;
    }
  }
}

export const stats = new StatsStore();
