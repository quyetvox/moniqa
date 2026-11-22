use crate::models::{FanStats, GpuStats, ThermalStats};

pub struct AppleSiliconCollector {
    sampler: Option<macmon::Sampler>,
    chip_name: String,
}

impl AppleSiliconCollector {
    pub fn new() -> Self {
        match macmon::Sampler::new() {
            Ok(sampler) => {
                let chip_name = sampler.get_soc_info().chip_name.clone();
                Self {
                    sampler: Some(sampler),
                    chip_name,
                }
            }
            Err(e) => {
                eprintln!("[MoniQA] macmon Sampler init fallback: {}", e);
                Self {
                    sampler: None,
                    chip_name: "Apple Silicon / Mac".to_string(),
                }
            }
        }
    }

    pub fn get_chip_name(&self) -> &str {
        &self.chip_name
    }

    pub fn collect(&mut self, duration_ms: u32, total_ram: u64) -> (GpuStats, ThermalStats, Vec<FanStats>) {
        if let Some(sampler) = &mut self.sampler {
            match sampler.get_metrics(duration_ms) {
                Ok(metrics) => {
                    let gpu_usage = (metrics.gpu_active_ratio * 100.0).clamp(0.0, 100.0);
                    let gpu_power = metrics.gpu_power;

                    let gpu_stats = GpuStats {
                        usage_percent: gpu_usage,
                        power_watts: gpu_power,
                        frequency_mhz: 0,
                        vram_used_bytes: (total_ram as f32 * (gpu_usage / 100.0 * 0.5)) as u64,
                        vram_total_bytes: total_ram,
                    };

                    let cpu_temp = metrics.temp.cpu_temp_avg.unwrap_or(0.0);
                    let gpu_temp = metrics.temp.gpu_temp_avg.unwrap_or(0.0);

                    let thermal_stats = ThermalStats {
                        cpu_temp_c: cpu_temp,
                        gpu_temp_c: gpu_temp,
                        package_temp_c: Some((cpu_temp + gpu_temp) / 2.0),
                        battery_temp_c: None,
                    };

                    let fan_stats: Vec<FanStats> = metrics
                        .fans
                        .iter()
                        .enumerate()
                        .map(|(i, f)| FanStats {
                            index: i,
                            name: format!("Fan {}", i + 1),
                            rpm: f.rpm,
                        })
                        .collect();

                    return (gpu_stats, thermal_stats, fan_stats);
                }
                Err(e) => {
                    eprintln!("[MoniQA] Error collecting macmon metrics: {}", e);
                }
            }
        }

        // Fallback if macmon is unavailable or error
        (
            GpuStats {
                usage_percent: 0.0,
                power_watts: 0.0,
                frequency_mhz: 0,
                vram_used_bytes: 0,
                vram_total_bytes: total_ram,
            },
            ThermalStats {
                cpu_temp_c: 0.0,
                gpu_temp_c: 0.0,
                package_temp_c: None,
                battery_temp_c: None,
            },
            Vec::new(),
        )
    }
}
