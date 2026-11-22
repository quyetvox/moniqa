pub mod apple_silicon;
pub mod battery;
pub mod cpu;
pub mod disk_net;
pub mod memory;
pub mod processes;
pub mod system_info;

use crate::models::SystemSnapshot;
use std::time::{SystemTime, UNIX_EPOCH};
use sysinfo::{ProcessesToUpdate, System};

pub struct CollectorHub {
    sys: System,
    disk_net: disk_net::DiskNetCollector,
    apple_silicon: apple_silicon::AppleSiliconCollector,
}

impl CollectorHub {
    pub fn new() -> Self {
        let mut sys = System::new_all().or_else(|_| System::new()).unwrap();
        sys.refresh_all();
        let disk_net = disk_net::DiskNetCollector::new();
        let apple_silicon = apple_silicon::AppleSiliconCollector::new();

        Self {
            sys,
            disk_net,
            apple_silicon,
        }
    }

    pub fn collect_snapshot(&mut self) -> SystemSnapshot {
        // Refresh sysinfo core
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();
        self.sys.refresh_processes(ProcessesToUpdate::All, true);

        let cpu = cpu::collect_cpu(&self.sys);
        let memory = memory::collect_memory(&self.sys);
        let (disk, network) = self.disk_net.collect();
        let top_processes = processes::collect_top_processes(&self.sys, 15);
        let battery = battery::collect_battery();
        let (gpu, thermal, fans) = self.apple_silicon.collect(3000, memory.total_bytes);
        let system_info = system_info::collect_system_info(Some(self.apple_silicon.get_chip_name()));

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        SystemSnapshot {
            timestamp,
            cpu,
            memory,
            gpu,
            thermal,
            disk,
            network,
            battery,
            fans,
            top_processes,
            system_info,
        }
    }
}
