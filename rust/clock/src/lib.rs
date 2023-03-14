#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let positive_minutes = match minutes {
            ..=-1 => 60 + minutes % 60,
            _ => minutes,
        };
        let positive_hours = match hours {
            ..=-1 => 24 + hours % 24,
            _ => hours,
        };
        let result_hours = (positive_hours + positive_minutes / 60) % 24;
        let result_minutes = positive_minutes % 60;
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
