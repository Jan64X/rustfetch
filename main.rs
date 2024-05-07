// Import necessary libraries
extern crate sysinfo;
use sysinfo::{System, SystemExt, DiskExt};
use crate::sysinfo::CpuExt;
use std::collections::HashMap;

fn main() {
    // Create a new system object
    let mut system = System::new_all();

    // Refresh system information
    system.refresh_all();

    // indent
    println!(" ");

    // Display ASCII art
    println!("\x1b[38;2;23;147;209m           #                  \x1b[0m");
    println!("\x1b[38;2;23;147;209m          ###                 \x1b[0m");
    println!("\x1b[38;2;23;147;209m         #####                \x1b[0m");
    println!("\x1b[38;2;23;147;209m        #######               \x1b[0m");
    println!("\x1b[38;2;23;147;209m       #########              \x1b[0m");
    println!("\x1b[38;2;23;147;209m      ###########             \x1b[0m");
    println!("\x1b[38;2;23;147;209m     #############            \x1b[0m");
    println!("\x1b[38;2;23;147;209m    #######  ######           \x1b[0m");
    println!("\x1b[38;2;23;147;209m   #######    ######          \x1b[0m");
    println!("\x1b[38;2;23;147;209m  ########    #######         \x1b[0m");
    println!("\x1b[38;2;23;147;209m #####           #####        \x1b[0m");
    println!("\x1b[38;2;23;147;209m###                 ###       \x1b[0m");

    // indent
    println!(" ");

    // Display system information
    let os = uname::uname().unwrap().sysname;
    println!("OS: {}", os);

    let uptime_seconds = system.uptime();
    let uptime_hours = uptime_seconds / 3600;
    let uptime_minutes = (uptime_seconds % 3600) / 60;
    let uptime_seconds_remainder = uptime_seconds % 60;
    println!("Kernel: {}", system.kernel_version().unwrap_or_else(|| "Unknown".to_string()));
    println!("Uptime: {}h {}m {}s", uptime_hours, uptime_minutes, uptime_seconds_remainder);

    // Display CPU information
    let cpu = system.global_cpu_info();
    println!("CPU: {}", cpu.brand());
    println!("CPU Speed: {:.2} GHz", cpu.frequency() as f32 / 1000.0);

    // Display memory information
    let total_memory = system.total_memory();
    let used_memory = system.used_memory();
    let memory_percentage = (used_memory as f64 / total_memory as f64) * 100.0;
    println!("Memory Total: {:.2} GB", total_memory as f64 / 1024.0 / 1024.0);
    println!("Memory Used: {:.2} GB ({:.2}%)", used_memory as f64 / 1024.0 / 1024.0, memory_percentage);

    // Display disk information
    let mut disk_totals: HashMap<String, (f64, f64)> = HashMap::new();

    let partitions = system.disks();
    for partition in partitions {
        let disk_name = partition.name().to_string_lossy().split_at(8).0.to_string();
        let total_space = partition.total_space() as f64 / 1024.0 / 1024.0 / 1024.0; // Convert bytes to gigabytes
        let used_space = (partition.total_space() - partition.available_space()) as f64 / 1024.0 / 1024.0 / 1024.0; // Convert bytes to gigabytes
        
        let (total, used) = disk_totals.entry(disk_name.clone()).or_insert((0.0, 0.0));
        *total += total_space;
        *used += used_space;
    }

    for (disk_name, (total, used)) in disk_totals.iter() {
        let used_percentage = (used / total) * 100.0;
        println!("Disk: {} - Total: {:.2} GB, Used: {:.2} GB ({:.2}%)", disk_name, *total, *used, used_percentage);
    }



    // Add more system information as needed
}
