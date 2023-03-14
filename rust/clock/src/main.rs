#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: hours,
            minutes: minutes,
        }
    }
    pub fn to_string(&self) -> String {
        let mut hours = self.hours + self.minutes / 60;
        let mut minutes = self.minutes % 60;
        hours = match self.hours {
            24.. => hours % 24,
            _ => hours,
        };
        minutes = match self.hours {
            60.. => hours % 60,
            _ => minutes,
        };
        let str_hours = match hours {
            0..=9 => format!("0{}", hours),
            24 => "00".to_owned(),
            _ => format!("{}", hours),
        };
        let str_minutes = match minutes {
            0..=9 => format!("0{}", minutes),
            60 => "00".to_owned(),
            _ => format!("{}", minutes),
        };
        format!("{}:{}", str_hours, str_minutes)
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {minutes} minutes to existing Clock time");
    }
}

fn main() {
    print!("{}", Clock::new(10, 37).to_string())
}
