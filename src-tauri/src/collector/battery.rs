use crate::models::BatteryStats;
use std::process::Command;

pub fn collect_battery() -> BatteryStats {
    let output = Command::new("ioreg")
        .args(["-r", "-c", "AppleSmartBattery", "-d", "1"])
        .output();

    if let Ok(out) = output {
        let text = String::from_utf8_lossy(&out.stdout);
        if text.trim().is_empty() {
            return BatteryStats {
                has_battery: false,
                charge_percent: 100.0,
                is_charging: false,
                health_percent: None,
                cycle_count: None,
                temperature_c: None,
            };
        }

        let mut current_capacity: Option<f32> = None;
        let mut max_capacity: Option<f32> = None;
        let mut is_charging = false;
        let mut cycle_count: Option<u32> = None;
        let mut temperature_c: Option<f32> = None;

        for line in text.lines() {
            let line = line.trim();
            if line.starts_with("\"CurrentCapacity\" =") {
                if let Some(val) = line.split('=').nth(1) {
                    current_capacity = val.trim().parse::<f32>().ok();
                }
            } else if line.starts_with("\"MaxCapacity\" =") {
                if let Some(val) = line.split('=').nth(1) {
                    max_capacity = val.trim().parse::<f32>().ok();
                }
            } else if line.starts_with("\"IsCharging\" =") {
                is_charging = line.contains("Yes");
            } else if line.starts_with("\"CycleCount\" =") {
                if let Some(val) = line.split('=').nth(1) {
                    cycle_count = val.trim().parse::<u32>().ok();
                }
            } else if line.starts_with("\"Temperature\" =") {
                if let Some(val) = line.split('=').nth(1) {
                    if let Ok(raw_temp) = val.trim().parse::<f32>() {
                        temperature_c = Some(raw_temp / 100.0);
                    }
                }
            }
        }

        let charge_percent = current_capacity.unwrap_or(0.0);
        let health_percent = max_capacity;

        return BatteryStats {
            has_battery: true,
            charge_percent,
            is_charging,
            health_percent,
            cycle_count,
            temperature_c,
        };
    }

    BatteryStats {
        has_battery: false,
        charge_percent: 100.0,
        is_charging: false,
        health_percent: None,
        cycle_count: None,
        temperature_c: None,
    }
}
