#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let new_hours = (hours + minutes / 60) % 24;
        let new_minutes = minutes % 60;
        let result_hours = match new_hours {
            ..=-1 => 24 + new_hours,
            _ => new_hours,
        };

        let result_minutes = match new_minutes {
            ..=-1 => 60 + new_minutes,
            _ => new_minutes,
        };
        Clock {
            hours: match result_hours {
                _ => result_hours,
            },
            minutes: match result_minutes {
                _ => result_minutes,
            },
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
        unimplemented!("Add {minutes} minutes to existing Clock time");
    }
}

fn main() {
    print!("{}", Clock::new(-12, -268).to_string())
}
