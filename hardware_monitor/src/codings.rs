
#[derive(Debug)]
pub struct Codings {
    version: String,
    min_threshold: i32,
    max_threshold: i32,
}

// Error types for invalid thresholds
#[derive(Debug)]
pub enum ThresholdError {
    OutOfRange,
    MinExceedsMax,
}

impl Codings {
    // Constructor with default values
    pub fn new() -> Codings {
        Codings {
            version: get_version(),
            min_threshold: 0,    // Default min threshold
            max_threshold: 80,   // Default max threshold
        }
    }
    
    // Constructor with custom thresholds
    pub fn with_thresholds(min: i32, max: i32) -> Result<Codings, ThresholdError> {
        // Check if thresholds are in plausible range
        if !Codings::is_temperature_plausible(min) || !Codings::is_temperature_plausible(max) {
            return Err(ThresholdError::OutOfRange);
        }
        
        // Check if min is less than max
        if min >= max {
            return Err(ThresholdError::MinExceedsMax);
        }
        
        Ok(Codings {
            version: get_version(),
            min_threshold: min,
            max_threshold: max,
        })
    }
    
    // Get the minimum threshold for underheating alarms
    pub fn get_min_threshold(&self) -> i32 {
        self.min_threshold
    }
    
    // Get the maximum threshold for overheating alarms
    pub fn get_max_threshold(&self) -> i32 {
        self.max_threshold
    }
    
    // Update thresholds with validation
    pub fn update_thresholds(&mut self, min: i32, max: i32) -> Result<(), ThresholdError> {
        // Check if thresholds are in plausible range
        if !Codings::is_temperature_plausible(min) || !Codings::is_temperature_plausible(max) {
            return Err(ThresholdError::OutOfRange);
        }
        
        // Check if min is less than max
        if min >= max {
            return Err(ThresholdError::MinExceedsMax);
        }
        
        // If validation passes, update the thresholds
        self.min_threshold = min;
        self.max_threshold = max;
        
        Ok(())
    }
    
    // Check if a temperature value is in the plausible range [-20, 120]
    pub fn is_temperature_plausible(temp: i32) -> bool {
        temp >= -20 && temp <= 120
    }
}

pub fn get_version() -> String {
    return "codings:0.1.0".to_string();
}
