#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}
pub fn convert_time(hours: i32, minutes: i32) -> (i32, i32) {
    let new_hours = (hours + minutes.div_euclid(60)).rem_euclid(24);
    let new_minutes = minutes.rem_euclid(60);
    (
        match new_hours {
            ..=-1 => 24 + new_hours,
            _ => new_hours,
        },
        match new_minutes {
            ..=-1 => 60 + new_minutes,
            _ => new_minutes,
        },
    )
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_time = convert_time(hours, minutes);
        Clock {
            hours: new_time.0,
            minutes: new_time.1,
        }
    }
    pub fn to_string(&self) -> String {
        let str_hours = match self.hours {
            0..=9 => format!("0{}", self.hours),
            _ => format!("{}", self.hours),
        };
        let str_minutes = match self.minutes {
            0..=9 => format!("0{}", self.minutes),
            _ => format!("{}", self.minutes),
        };
        format!("{}:{}", str_hours, str_minutes)
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let new_time = convert_time(self.hours, self.minutes + minutes);
        Clock {
            hours: new_time.0,
            minutes: new_time.1,
        }
    }
}
