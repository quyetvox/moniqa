use crate::models::CpuStats;
use sysinfo::System;

pub fn collect_cpu(sys: &System) -> CpuStats {
    let global_usage = sys.global_cpu_usage();
    let cpus = sys.cpus();
    let per_core = cpus.iter().map(|c| c.usage()).collect();
    let frequency_mhz = cpus.iter().map(|c| c.frequency()).collect();
    let core_count = System::physical_core_count().unwrap_or(cpus.len());
    let thread_count = cpus.len();
    let model_name = cpus
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Apple Silicon".to_string());

    CpuStats {
        global_usage,
        per_core,
        frequency_mhz,
        core_count,
        thread_count,
        model_name,
    }
}
