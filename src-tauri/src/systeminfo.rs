use serde::Serialize;
use sysinfo::{CpuExt, NetworkExt, System, SystemExt};
use tauri::Manager;

pub fn system_info_loop(app_handle: tauri::AppHandle) {
    tauri::async_runtime::spawn(async move {
        let one_sec = tokio::time::Duration::from_secs(1);
        let duration = one_sec;
        let mut sys = System::new_all();
        loop {
            let info = Info::refresh(&mut sys);
            app_handle.emit_all("system-info", info.json()).unwrap();
            tokio::time::sleep(duration).await;
        }
    });
}

#[derive(Default, Serialize)]
struct Info {
    upload: String,
    download: String,
    cpu: String,
    memory: String,
}

impl Info {
    pub fn refresh(sys: &mut System) -> Info {
        let mut info: Info = Default::default();

        sys.refresh_all();
        let networks = sys.networks();
        for (_, network) in networks {
            info.upload
                .push_str(&net_speed_to_string(network.transmitted()));
            info.download
                .push_str(&net_speed_to_string(network.received()));
        }

        let mut cpu_usage = 0.0;
        for cpu in sys.cpus() {
            cpu_usage += cpu.cpu_usage();
        }
        info.cpu = format!("{:.0}%", cpu_usage / sys.cpus().len() as f32);

        info.memory = format!(
            "{:.0}%",
            (sys.used_memory() as f32 / sys.total_memory() as f32) * 100.0
        );

        info
    }

    pub fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

fn net_speed_to_string(speed: u64) -> String {
    let one_k = 1024_f64;
    let mut speed = speed as f64;
    let mut unit = "KB/s";
    speed /= one_k;
    if speed > one_k {
        speed /= one_k;
        unit = "MB/s";
    }
    if speed > one_k {
        speed /= one_k;
        unit = "GB/s";
    }
    format!("{:.2} {}", speed, unit)
}
