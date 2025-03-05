use hardware_monitor::codings;
use hardware_monitor::filter;
use hardware_monitor::system_alarm_handler;
use hardware_monitor::thermal_reader;


#[derive(Debug)]
struct HardwareMonitor {
    version: String,
}

impl HardwareMonitor {
    fn new() -> HardwareMonitor {
        HardwareMonitor {
            version: get_version(),
        }
    }

    fn read_temperature(&self) -> i32 {
        // Placeholder for actual temperature reading
        return 42;
    }

    fn check_system_alarms(&self) {
        // Placeholder for actual system alarm checks
        println!("Checking system alarms...");
    }
}

pub fn get_version() -> String {
	return "hardware_monitor:0.1.0".to_string();
}

fn main() {
    println!("Hello, world!");
    println!("hardware_monitor version: {}", get_version());
    println!("filter version: {}", filter::get_version());
    println!("thermal_reader version: {}", thermal_reader::get_version());
    println!("system_alarm_handler version: {}", system_alarm_handler::get_version());
    println!("codings version: {}", codings::get_version());
}
