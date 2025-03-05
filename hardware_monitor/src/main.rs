#![allow(unused_variables, unused_mut, dead_code)]

use hardware_monitor::codings;
use hardware_monitor::filter;
use hardware_monitor::system_alarm_handler;
use hardware_monitor::thermal_reader;

use hardware_monitor::filter::{Filter, FilterState};
use hardware_monitor::system_alarm_handler::SystemAlarmHandler;
use hardware_monitor::thermal_reader::ThermalReader;
use std::{thread, time::Duration};

// Constants for temperature thresholds
const MIN_TEMP_THRESHOLD: i32 = 0;   // 0°C - underheating threshold
const MAX_TEMP_THRESHOLD: i32 = 80;  // 80°C - overheating threshold
const VALID_CODING_MIN: i32 = 1000;  // Minimum valid coding value
const VALID_CODING_MAX: i32 = 9999;  // Maximum valid coding value
const TEMP_CHECK_INTERVAL_MS: u64 = 500; // Interval for temperature checks

#[derive(Debug)]
struct HardwareMonitor {
    version: String,
    thermal_reader: ThermalReader,
    system_alarm_handler: SystemAlarmHandler,
    coding: i32,
}

impl HardwareMonitor {
    fn new(coding: i32) -> Result<HardwareMonitor, String> {
        // Check if coding is plausible
        if coding < VALID_CODING_MIN || coding > VALID_CODING_MAX {
            return Err(format!("Invalid coding value: {}. Must be between {} and {}", 
                            coding, VALID_CODING_MIN, VALID_CODING_MAX));
        }

        Ok(HardwareMonitor {
            version: get_version(),
            thermal_reader: ThermalReader::new(),
            system_alarm_handler: SystemAlarmHandler::new(),
            coding,
        })
    }

    fn run(&mut self) {
        println!("Starting hardware monitoring with coding: {}", self.coding);
        
        loop {
            // Update temperature in filter (every 500ms)
            match self.thermal_reader.update_current_temp() {
                Ok(temp) => println!("Raw temperature reading: {}°C", temp),
                Err(e) => println!("Temperature reading error: {}", e),
            }
            
            // Read filtered temperature and check against thresholds
            match self.thermal_reader.read_filtered_temperature() {
                Ok(filtered_temp) => {
                    println!("Filtered temperature: {}°C", filtered_temp);
                    
                    // Check for overheating
                    if filtered_temp > MAX_TEMP_THRESHOLD {
                        println!("ALERT: System overheating detected!");
                        self.system_alarm_handler.report_overheating_alarm(filtered_temp, self.coding);
                    } 
                    // Check for underheating
                    else if filtered_temp < MIN_TEMP_THRESHOLD {
                        println!("ALERT: System underheating detected!");
                        self.system_alarm_handler.report_underheating_alarm(filtered_temp, self.coding);
                    }
                    else {
                        println!("Temperature within normal range.");
                    }
                },
                Err(e) => println!("Error reading filtered temperature: {}", e),
            }
            
            // Sleep for the specified interval
            thread::sleep(Duration::from_millis(TEMP_CHECK_INTERVAL_MS));
        }
    }
}

pub fn get_version() -> String {
    return "hardware_monitor:0.1.0".to_string();
}

fn main() {
    println!("Hardware Monitor System Starting...");
    println!("hardware_monitor version: {}", get_version());
    println!("filter version: {}", filter::get_version());
    println!("thermal_reader version: {}", thermal_reader::get_version());
    println!("system_alarm_handler version: {}", system_alarm_handler::get_version());
    println!("codings version: {}", codings::get_version());
    
    // Example coding value
    let coding = 1234;
    
    // Create and run the hardware monitor
    match HardwareMonitor::new(coding) {
        Ok(mut monitor) => {
            println!("Hardware Monitor initialized successfully.");
            monitor.run(); // This starts the infinite monitoring loop
        },
        Err(e) => {
            println!("Failed to initialize Hardware Monitor: {}", e);
        }
    }
}
