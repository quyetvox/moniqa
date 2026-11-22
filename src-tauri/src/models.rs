use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemSnapshot {
    pub timestamp: u64,
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub gpu: GpuStats,
    pub thermal: ThermalStats,
    pub disk: DiskStats,
    pub network: NetworkStats,
    pub battery: BatteryStats,
    pub fans: Vec<FanStats>,
    pub top_processes: Vec<ProcessInfo>,
    pub system_info: SystemInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStats {
    pub global_usage: f32,
    pub per_core: Vec<f32>,
    pub frequency_mhz: Vec<u64>,
    pub core_count: usize,
    pub thread_count: usize,
    pub model_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuStats {
    pub usage_percent: f32,
    pub power_watts: f32,
    pub frequency_mhz: u32,
    pub vram_used_bytes: u64,
    pub vram_total_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalStats {
    pub cpu_temp_c: f32,
    pub gpu_temp_c: f32,
    pub package_temp_c: Option<f32>,
    pub battery_temp_c: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskStats {
    pub read_bytes_per_sec: u64,
    pub write_bytes_per_sec: u64,
    pub disks: Vec<DiskPartitionInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskPartitionInfo {
    pub name: String,
    pub mount_point: String,
    pub total_bytes: u64,
    pub available_bytes: u64,
    pub used_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkStats {
    pub total_received_bytes_per_sec: u64,
    pub total_transmitted_bytes_per_sec: u64,
    pub interfaces: Vec<NetworkInterfaceInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceInfo {
    pub name: String,
    pub rx_bytes_per_sec: u64,
    pub tx_bytes_per_sec: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryStats {
    pub has_battery: bool,
    pub charge_percent: f32,
    pub is_charging: bool,
    pub health_percent: Option<f32>,
    pub cycle_count: Option<u32>,
    pub temperature_c: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanStats {
    pub index: usize,
    pub name: String,
    pub rpm: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_bytes: u64,
    pub user: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub uptime_secs: u64,
    pub chip_name: String,
}
