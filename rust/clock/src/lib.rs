use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time_vec: (i32, i32) = Clock::get_reduced_hrs_min(hours, minutes);
        Clock {hours: time_vec.0, minutes: time_vec.1}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time_vec: (i32, i32) = Clock::get_reduced_hrs_min(self.hours, self.minutes + minutes);
        Clock {hours: time_vec.0, minutes: time_vec.1}
    }

    pub fn get_reduced_hrs_min(hours: i32, minutes: i32) -> (i32, i32){
        let tminutes = (hours * 60 ) + (minutes);
        let minutes: i32 = ( (tminutes % 60) + 60) % 60;
        let hours: i32 = ( ( ( ( (tminutes - minutes) / 60 ) % 24) + 24) % 24);
        (hours , minutes)
    }
}
impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
