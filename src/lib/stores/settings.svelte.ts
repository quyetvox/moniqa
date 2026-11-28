import type { DensityMode, NetworkUnit, TimeWindow } from "../types";

interface PersistedSettings {
  density: DensityMode;
  intervalMs: number;
  netUnit: NetworkUnit;
  warnThreshold: number;
  critThreshold: number;
  timeWindow: TimeWindow;
  sidebarCollapsed: boolean;
}

const STORAGE_KEY = "moniqa_settings_v1";

class SettingsStore {
  density = $state<DensityMode>("compact");
  intervalMs = $state<number>(1000);
  paused = $state<boolean>(false);
  netUnit = $state<NetworkUnit>("bytes");
  warnThreshold = $state<number>(70);
  critThreshold = $state<number>(90);
  timeWindow = $state<TimeWindow>("60s");
  sidebarCollapsed = $state<boolean>(false);

  constructor() {
    if (typeof window !== "undefined") {
      try {
        const raw = localStorage.getItem(STORAGE_KEY);
        if (raw) {
          const parsed = JSON.parse(raw) as Partial<PersistedSettings>;
          if (parsed.density === "compact" || parsed.density === "comfortable") {
            this.density = parsed.density;
          }
          if (typeof parsed.intervalMs === "number" && [500, 1000, 2000, 5000].includes(parsed.intervalMs)) {
            this.intervalMs = parsed.intervalMs;
          }
          if (parsed.netUnit === "bytes" || parsed.netUnit === "bits") {
            this.netUnit = parsed.netUnit;
          }
          if (typeof parsed.warnThreshold === "number") {
            this.warnThreshold = parsed.warnThreshold;
          }
          if (typeof parsed.critThreshold === "number") {
            this.critThreshold = parsed.critThreshold;
          }
          if (parsed.timeWindow === "60s" || parsed.timeWindow === "5m") {
            this.timeWindow = parsed.timeWindow;
          }
          if (typeof parsed.sidebarCollapsed === "boolean") {
            this.sidebarCollapsed = parsed.sidebarCollapsed;
          }
        }
      } catch {
        // Ignore malformed localStorage
      }
    }
  }

  private persist() {
    if (typeof window === "undefined") return;
    const payload: PersistedSettings = {
      density: this.density,
      intervalMs: this.intervalMs,
      netUnit: this.netUnit,
      warnThreshold: this.warnThreshold,
      critThreshold: this.critThreshold,
      timeWindow: this.timeWindow,
      sidebarCollapsed: this.sidebarCollapsed,
    };
    localStorage.setItem(STORAGE_KEY, JSON.stringify(payload));
  }

  setDensity(density: DensityMode) {
    this.density = density;
    this.persist();
  }

  setIntervalMs(ms: number) {
    this.intervalMs = ms;
    this.persist();
  }

  togglePause() {
    this.paused = !this.paused;
  }

  setNetUnit(unit: NetworkUnit) {
    this.netUnit = unit;
    this.persist();
  }

  setThresholds(warn: number, crit: number) {
    this.warnThreshold = Math.max(10, Math.min(warn, crit - 5));
    this.critThreshold = Math.min(99, Math.max(crit, this.warnThreshold + 5));
    this.persist();
  }

  setTimeWindow(win: TimeWindow) {
    this.timeWindow = win;
    this.persist();
  }

  toggleSidebar() {
    this.sidebarCollapsed = !this.sidebarCollapsed;
    this.persist();
  }
}

export const settings = new SettingsStore();
