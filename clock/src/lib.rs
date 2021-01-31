use std::fmt;

#[derive(Debug, PartialEq)]
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
    // New solution
    pub fn new(hours: i32, minutes: i32) -> Self {
        const DAY: i32 = 24 * 60; //1440
        const HOUR: i32 = 60;

        let temp = (((hours * HOUR + minutes) % DAY) + DAY) % DAY;
        //        Here to deal with negative values <---|______|     

        Clock{
            hours: temp / HOUR,
            minutes: temp % HOUR,
        }
    }

    // First solution
    pub fn old_new(hours: i32, minutes: i32) -> Self {
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
