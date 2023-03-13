use time::{OffsetDateTime, PrimitiveDateTime as DateTime};
// Returns a DateTime one billion seconds after start.
fn dt(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> DateTime {
    use time::{Date, Time};

    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}
fn main() {
    let start_date = dt(2011, 4, 25, 0, 0, 0);
    println!("{:?}",start_date);
    let start_timestamp = &start_date.assume_utc().unix_timestamp();
    let end_time = match OffsetDateTime::from_unix_timestamp(start_timestamp + 1_000_000_000) {
        Ok(dt) => dt,
        _ => OffsetDateTime::UNIX_EPOCH,
    };
    println!("{:?}", end_time.date())
}
