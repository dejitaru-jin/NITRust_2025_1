use crate::filter::{Filter, FilterState};
use std::fmt;

// Custom error enum for ThermalReader
#[derive(Debug)]
enum ThermalReaderError {
    HardwareReadError,
    ConsecutiveErrors,
    FilterError(FilterState),
}

impl fmt::Display for ThermalReaderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ThermalReaderError::HardwareReadError => write!(f, "Failed to read from hardware"),
            ThermalReaderError::ConsecutiveErrors => write!(f, "Too many consecutive read errors"),
            ThermalReaderError::FilterError(state) => write!(f, "Filter error: {:?}", state),
        }
    }
}

#[derive(Debug)]
struct ThermalReader {
    version: String,
    filter: Filter,
    consecutive_errors: u8,
    last_valid_temp: i32,
    // Simulated temperature readings
    simulated_readings: Vec<i32>,
    current_reading_index: usize,
}

impl ThermalReader {
    fn new() -> ThermalReader {
        // Initialize with some simulated temperature readings
        let simulated_readings = vec![
            25, 26, 27, 28, 29, 30, 31, 32, 33,  // Valid readings to fill buffer
            35, 40, 45, 50, 55, 60,              // More valid readings
            150, 200,                            // Invalid readings (too high)
            50, 55, 60, 65,                      // Back to valid readings
            -50, -100,                           // Invalid readings (too low)
            30, 32, 35, 38, 40                   // More valid readings
        ];
        
        ThermalReader {
            version: get_version(),
            filter: Filter::new(),
            consecutive_errors: 0,
            last_valid_temp: 0,
            simulated_readings,
            current_reading_index: 0,
        }
    }

    fn update_current_temp(&mut self) -> Result<i32, ThermalReaderError> {
        // Get the next simulated reading
        let temp = self.get_next_reading();
        
        // Check if temperature is within valid range [-20, 120]
        if temp < -20 || temp > 120 {
            self.consecutive_errors += 1;
            return Err(ThermalReaderError::HardwareReadError);
        }
        
        // If we got here, we have a valid reading
        self.consecutive_errors = 0;
        
        // Update the filter with the new temperature
        self.filter.update_filter_data(temp);
        
        Ok(temp)
    }
    
    fn read_filtered_temperature(&mut self) -> Result<i32, ThermalReaderError> {
        // Check if we have too many consecutive errors
        if self.consecutive_errors >= 3 {
            return Err(ThermalReaderError::ConsecutiveErrors);
        }
        
        // Try to get filtered data
        match self.filter.filter_data() {
            Ok(filtered_temp) => {
                self.last_valid_temp = filtered_temp;
                Ok(filtered_temp)
            },
            Err(FilterState::NotReady) => {
                // If filter is not ready, this is not considered an error
                // Just return the last valid temperature or 0 if none
                Ok(self.last_valid_temp)
            }
        }
    }
    
    // Helper method to get the next simulated reading
    fn get_next_reading(&mut self) -> i32 {
        let reading = self.simulated_readings[self.current_reading_index];
        self.current_reading_index = (self.current_reading_index + 1) % self.simulated_readings.len();
        reading
    }
}

pub fn get_version() -> String {
    return "thermal_reader:0.1.0".to_string();
}
