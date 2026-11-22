use crate::models::ProcessInfo;
use sysinfo::System;

pub fn collect_top_processes(sys: &System, limit: usize) -> Vec<ProcessInfo> {
    let mut procs: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, proc_info)| ProcessInfo {
            pid: pid.as_u32(),
            name: proc_info.name().to_string_lossy().to_string(),
            cpu_usage: proc_info.cpu_usage(),
            memory_bytes: proc_info.memory(),
            user: proc_info.user_id().map(|u| u.to_string()),
        })
        .collect();

    // Sort descending by CPU usage primarily, then by Memory
    procs.sort_by(|a, b| {
        b.cpu_usage
            .partial_cmp(&a.cpu_usage)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| b.memory_bytes.cmp(&a.memory_bytes))
    });

    procs.truncate(limit);
    procs
}
