use sysinfo::{System, SystemExt, CpuExt, ProcessExt};
use std::{thread, time::Duration};

fn main() {
    let mut system = System::new_all();

    loop {
        system.refresh_all();

        let total_memory = system.total_memory();
        let used_memory = system.used_memory();
        let total_swap = system.total_swap();
        let used_swap = system.used_swap();
        let cpu_usage = system.global_cpu_info().cpu_usage();

        println!("Total Memory: {} KB", total_memory);
        println!("Used Memory: {} KB", used_memory);
        println!("Total Swap: {} KB", total_swap);
        println!("Used Swap: {} KB", used_swap);
        println!("CPU Usage: {:.2}%", cpu_usage);

        let mut processes: Vec<_> = system.processes().iter().collect();
        processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap());

         for (pid, process) in processes.iter().take(5) {
            println!(
                "PID: {} | {} | Memory: {} KB",
                pid,
                process.name(),
                process.memory()
            );
        }

        println!("==============================\n");


        thread::sleep(Duration::from_secs(5));
    }
}