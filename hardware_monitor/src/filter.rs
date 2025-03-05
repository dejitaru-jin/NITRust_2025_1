#[derive(Debug)]
enum FilterState {
    Ready,
    NotReady,
}

#[derive(Debug)]
struct Filter {
    version: String,
    buffer: [i32; 9],      // Fixed-size buffer for 9 elements
    position: usize,       // Current position in the ring buffer
    initialized: NotReady,     // Whether all buffer elements have been filled
    count: usize,          // Number of elements added to the buffer
}

impl Filter {
    fn new() -> Filter {
        Filter {
            version: get_version(),
            buffer: [0; 9],
            position: 0,
            initialized: NotReady,
            count: 0,
        }
    }

    fn update_filter_data(&mut self, data: i32) {
        // Add the new data to the current position in the ring buffer
        self.buffer[self.position] = data;
        
        // Move position forward (wrap around to beginning if needed)
        self.position = (self.position + 1) % self.buffer.len();
        
        // Increase count and check if buffer is now fully initialized
        if self.initialized == NotReady {
            self.count += 1;
            if self.count >= self.buffer.len() {
                self.initialized = Ready;
            }
        }
    }

    fn filter_data(&self) -> Result<i32, FilterState> {
        if self.initialized == NotReady {
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

