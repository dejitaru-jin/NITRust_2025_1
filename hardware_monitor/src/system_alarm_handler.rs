use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

// Enum to represent different types of alarms
#[derive(Debug)]
pub enum AlarmType {
    Overheating,
    Underheating,
}

impl fmt::Display for AlarmType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AlarmType::Overheating => write!(f, "OVERHEATING"),
            AlarmType::Underheating => write!(f, "UNDERHEATING"),
        }
    }
}

// Struct to store alarm information
#[derive(Debug)]
pub struct AlarmEvent {
    alarm_type: AlarmType,
    temperature: i32,
    coding: i32,
    timestamp: u64,
}

#[derive(Debug)]
pub struct SystemAlarmHandler {
    version: String,
    alarm_count: usize,
}

impl SystemAlarmHandler {
    pub fn new() -> SystemAlarmHandler {
        SystemAlarmHandler {
            version: get_version(),
            alarm_count: 0,
        }
    }
    
    pub fn report_overheating_alarm(&mut self, temperature: i32, coding: i32) {
        self.alarm_count += 1;
        
        // Create alarm event
        let alarm = AlarmEvent {
            alarm_type: AlarmType::Overheating,
            temperature,
            coding,
            timestamp: SystemAlarmHandler::get_current_timestamp(),
        };
        
        self.log_alarm(&alarm);
    }
    
    pub fn report_underheating_alarm(&mut self, temperature: i32, coding: i32) {
        self.alarm_count += 1;
        
        // Create alarm event
        let alarm = AlarmEvent {
            alarm_type: AlarmType::Underheating,
            temperature,
            coding,
            timestamp: SystemAlarmHandler::get_current_timestamp(),
        };
        
        self.log_alarm(&alarm);
    }
    
    // Helper method to log alarms to console
    fn log_alarm(&self, alarm: &AlarmEvent) {
        println!("╔════════════════════════════════════════════════════════╗");
        println!("║  ALARM #{:<6} TYPE: {:<20}          ║", 
                self.alarm_count, alarm.alarm_type);
        println!("╠════════════════════════════════════════════════════════╣");
        println!("║  Temperature: {:<3}°C                                  ║", 
                alarm.temperature);
        println!("║  Coding:      {:<4}                                    ║", 
                alarm.coding);
        println!("║  Timestamp:   {:<10}                              ║", 
                alarm.timestamp);
        println!("╚════════════════════════════════════════════════════════╝");
    }
    
    fn get_current_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
    
    pub fn get_alarm_count(&self) -> usize {
        self.alarm_count
    }
}

pub fn get_version() -> String {
    return "system_alarm_handler:0.1.0".to_string();
}
