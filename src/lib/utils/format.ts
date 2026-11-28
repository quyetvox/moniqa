import type { NetworkUnit } from "../types";

export function formatBytes(bytes: number, decimals = 1): string {
  if (!bytes || bytes <= 0) return "0 B";
  const k = 1024;
  const dm = decimals < 0 ? 0 : decimals;
  const sizes = ["B", "KB", "MB", "GB", "TB", "PB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const safeI = Math.min(i, sizes.length - 1);
  return `${parseFloat((bytes / Math.pow(k, safeI)).toFixed(dm))} ${sizes[safeI]}`;
}

export function formatRate(bytesPerSec: number, unit: NetworkUnit = "bytes"): string {
  if (!bytesPerSec || bytesPerSec <= 0) {
    return unit === "bits" ? "0 bps" : "0 B/s";
  }
  if (unit === "bits") {
    const bits = bytesPerSec * 8;
    const k = 1000;
    const sizes = ["bps", "Kbps", "Mbps", "Gbps"];
    const i = Math.min(Math.floor(Math.log(bits) / Math.log(k)), sizes.length - 1);
    return `${(bits / Math.pow(k, i)).toFixed(1)} ${sizes[i]}`;
  }
  return `${formatBytes(bytesPerSec)}/s`;
}

export function formatTemp(celsius: number | null | undefined): string {
  if (celsius === null || celsius === undefined || celsius <= 0) return "—";
  return `${celsius.toFixed(0)}°C`;
}

export function formatPercent(percent: number | null | undefined, decimals = 1): string {
  if (percent === null || percent === undefined || Number.isNaN(percent)) return "—";
  return `${percent.toFixed(decimals)}%`;
}

export function formatUptime(seconds: number): string {
  const days = Math.floor(seconds / (3600 * 24));
  const hours = Math.floor((seconds % (3600 * 24)) / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);

  if (days > 0) return `${days}d ${hours}h ${minutes}m`;
  if (hours > 0) return `${hours}h ${minutes}m`;
  return `${minutes}m`;
}

export function formatTimeShort(tsMs: number): string {
  const d = new Date(tsMs);
  return d.toLocaleTimeString([], { hour12: false, hour: "2-digit", minute: "2-digit", second: "2-digit" });
}

export function middleEllipsis(text: string, maxLen = 36): string {
  if (!text || text.length <= maxLen) return text;
  const keep = Math.floor((maxLen - 1) / 2);
  return `${text.slice(0, keep)}…${text.slice(text.length - keep)}`;
}
