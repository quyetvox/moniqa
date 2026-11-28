export interface SystemSnapshot {
  timestamp: number;
  cpu: CpuStats;
  memory: MemoryStats;
  gpu: GpuStats;
  thermal: ThermalStats;
  disk: DiskStats;
  network: NetworkStats;
  battery: BatteryStats;
  fans: FanStats[];
  top_processes: ProcessInfo[];
  system_info: SystemInfo;
}

export interface CpuStats {
  global_usage: number;
  per_core: number[];
  frequency_mhz: number[];
  core_count: number;
  thread_count: number;
  model_name: string;
}

export interface MemoryStats {
  used_bytes: number;
  total_bytes: number;
  available_bytes: number;
  swap_used_bytes: number;
  swap_total_bytes: number;
}

export interface GpuStats {
  usage_percent: number;
  power_watts: number;
  frequency_mhz: number;
  vram_used_bytes: number;
  vram_total_bytes: number;
}

export interface ThermalStats {
  cpu_temp_c: number;
  gpu_temp_c: number;
  package_temp_c: number | null;
  battery_temp_c: number | null;
}

export interface DiskStats {
  read_bytes_per_sec: number;
  write_bytes_per_sec: number;
  disks: DiskPartitionInfo[];
}

export interface DiskPartitionInfo {
  name: string;
  mount_point: string;
  total_bytes: number;
  available_bytes: number;
  used_bytes: number;
}

export interface NetworkStats {
  total_received_bytes_per_sec: number;
  total_transmitted_bytes_per_sec: number;
  interfaces: NetworkInterfaceInfo[];
}

export interface NetworkInterfaceInfo {
  name: string;
  rx_bytes_per_sec: number;
  tx_bytes_per_sec: number;
}

export interface BatteryStats {
  has_battery: boolean;
  charge_percent: number;
  is_charging: boolean;
  health_percent: number | null;
  cycle_count: number | null;
  temperature_c: number | null;
}

export interface FanStats {
  index: number;
  name: string;
  rpm: number;
}

export interface ProcessInfo {
  pid: number;
  name: string;
  cpu_usage: number;
  memory_bytes: number;
  user: string | null;
}

export interface SystemInfo {
  hostname: string;
  os_name: string;
  os_version: string;
  kernel_version: string;
  uptime_secs: number;
  chip_name: string;
}

export type DensityMode = "compact" | "comfortable";
export type NetworkUnit = "bytes" | "bits";
export type TimeWindow = "60s" | "5m";

export interface SensorBounds {
  min: number;
  max: number;
}
