use crate::models::MemoryStats;
use sysinfo::System;

pub fn collect_memory(sys: &System) -> MemoryStats {
    MemoryStats {
        used_bytes: sys.used_memory(),
        total_bytes: sys.total_memory(),
        available_bytes: sys.available_memory(),
        swap_used_bytes: sys.used_swap(),
        swap_total_bytes: sys.total_swap(),
    }
}
