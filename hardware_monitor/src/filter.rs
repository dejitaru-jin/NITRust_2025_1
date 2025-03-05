#![allow(unused_variables, unused_mut, dead_code)]

#[derive(Debug, PartialEq)]
pub enum FilterState {
    Ready,
    NotReady,
}

#[derive(Debug)]
pub struct Filter {
    version: String,
    buffer: [i32; 9],      // Fixed-size buffer for 9 elements
    position: usize,       // Current position in the ring buffer
    initialized: FilterState,  // Whether all buffer elements have been filled
    count: usize,          // Number of elements added to the buffer
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            version: get_version(),
            buffer: [0; 9],
            position: 0,
            initialized: FilterState::NotReady,
            count: 0,
        }
    }

    pub fn update_filter_data(&mut self, data: i32) {
        // Add the new data to the current position in the ring buffer
        self.buffer[self.position] = data;
        
        // Move position forward (wrap around to beginning if needed)
        self.position = (self.position + 1) % self.buffer.len();
        
        // Increase count and check if buffer is now fully initialized
        if self.initialized == FilterState::NotReady {
            self.count += 1;
            if self.count >= self.buffer.len() {
                self.initialized = FilterState::Ready;
            }
        }
    }

    pub fn filter_data(&self) -> Result<i32, FilterState> {
        if self.initialized == FilterState::NotReady {
            // Return error if buffer isn't fully initialized
            return Err(FilterState::NotReady);
        }
        
        // Create a copy of the buffer to sort
        let mut sorted_buffer = self.buffer.clone();
        sorted_buffer.sort();
        
        // Return the median value (middle element in sorted array)
        Ok(sorted_buffer[sorted_buffer.len() / 2])
    }
}

pub fn get_version() -> String {
    return "filter:0.1.0".to_string();
}
