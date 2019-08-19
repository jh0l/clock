use std::fmt;

fn rem_euclid(a: f32, rhs: f32) -> f32 {
    let r = a % rhs;
    if r < 0. {
        if rhs < 0. {
            r - rhs
        } else {
            r + rhs
        }
    } else {
        r
    }
}

#[derive(Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    const MIN: f32 = 60.;
    const HRS: f32 = 24.;
    fn get_hrs_min_res(hours: i32, minutes: i32) -> (i32, i32) {
        let t = (minutes as f32 + hours as f32 * Self::MIN) / Self::MIN;
        let h = rem_euclid(t, Self::HRS);
        let m = (h - h as i32 as f32) * Self::MIN;
        (h as i32, m.round() as i32)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Self::get_hrs_min_res(hours, minutes);
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = Self::get_hrs_min_res(self.hours, self.minutes + minutes);
        Self { hours, minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Clock) -> bool {
        self.minutes == other.minutes && self.hours == other.hours
    }
}
