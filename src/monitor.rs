use sysinfo::{System, SystemExt, ProcessorExt};

pub fn print_system_stats() {
    let mut system = System::new_all();
    system.refresh_all();

    // CPU usage
    for processor in system.processors() {
        println!("Processor: {} - Usage: {:.2}%", processor.name(), processor.cpu_usage());
    }

    // Memory usage
    println!("Total memory: {} KB", system.total_memory());
    println!("Used memory: {} KB", system.used_memory());
}
