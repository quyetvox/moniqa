use crate::models::SystemInfo;
use sysinfo::System;

pub fn collect_system_info(chip_override: Option<&str>) -> SystemInfo {
    SystemInfo {
        hostname: System::host_name().unwrap_or_else(|_| "MacBook".to_string()),
        os_name: System::name().unwrap_or_else(|_| "macOS".to_string()),
        os_version: System::os_version().unwrap_or_else(|_| "".to_string()),
        kernel_version: System::kernel_version().unwrap_or_else(|_| "".to_string()),
        uptime_secs: System::uptime().unwrap_or(0),
        chip_name: chip_override
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Apple Silicon".to_string()),
    }
}
