use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let rolled_minutes = total_minutes.rem_euclid(24 * 60);
        
        Self {
            hours: rolled_minutes / 60,
            minutes: rolled_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}

// --- Add this block to fix the compilation error ---
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // {:02} pads integers with a leading zero if they are a single digit
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}