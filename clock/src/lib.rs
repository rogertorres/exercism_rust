use std::fmt;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = hours;
        let mut m = minutes;

        // Convert excessive minutes into hours
        if !(0..60).contains(&minutes){
            m = minutes % 60;
            h += minutes / 60;

            if m < 0 {
                m += 60;
                h -= 1;
            };
        };

        // Convert hours to 24h pattern
        if !(0..24).contains(&h){
            h %= 24;
            if h < 0 { h += 24 };
        };

        Clock {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}
