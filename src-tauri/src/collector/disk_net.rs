use crate::models::{DiskPartitionInfo, DiskStats, NetworkInterfaceInfo, NetworkStats};
use sysinfo::{Disks, Networks};

pub struct DiskNetCollector {
    networks: Networks,
    disks: Disks,
}

impl DiskNetCollector {
    pub fn new() -> Self {
        Self {
            networks: Networks::new_with_refreshed_list()
                .or_else(|_| Networks::new())
                .unwrap(),
            disks: Disks::new_with_refreshed_list()
                .or_else(|_| Disks::new())
                .unwrap(),
        }
    }

    pub fn collect(&mut self) -> (DiskStats, NetworkStats) {
        // Refresh networks
        self.networks.refresh(true);

        let mut total_rx = 0u64;
        let mut total_tx = 0u64;
        let mut iface_list = Vec::new();

        for (name, data) in &self.networks {
            let name_str = name.to_string_lossy().to_string();
            let rx = data.received();
            let tx = data.transmitted();
            total_rx += rx;
            total_tx += tx;

            if rx > 0 || tx > 0 || name_str.starts_with("en") || name_str.starts_with("eth") {
                iface_list.push(NetworkInterfaceInfo {
                    name: name_str,
                    rx_bytes_per_sec: rx,
                    tx_bytes_per_sec: tx,
                });
            }
        }

        // Refresh disks
        self.disks.refresh(true);
        let mut disk_list = Vec::new();
        for disk in &self.disks {
            let total = disk.total_space();
            let available = disk.available_space();
            disk_list.push(DiskPartitionInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total_bytes: total,
                available_bytes: available,
                used_bytes: total.saturating_sub(available),
            });
        }

        let disk_stats = DiskStats {
            read_bytes_per_sec: 0,
            write_bytes_per_sec: 0,
            disks: disk_list,
        };

        let net_stats = NetworkStats {
            total_received_bytes_per_sec: total_rx,
            total_transmitted_bytes_per_sec: total_tx,
            interfaces: iface_list,
        };

        (disk_stats, net_stats)
    }
}
