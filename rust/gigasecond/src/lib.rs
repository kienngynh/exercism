use time::{Date, OffsetDateTime, PrimitiveDateTime as DateTime, Time};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let end_time = match OffsetDateTime::from_unix_timestamp(
        start.assume_utc().unix_timestamp() + 1_000_000_000,
    ) {
        Ok(dt) => dt,
        _ => OffsetDateTime::UNIX_EPOCH,
    };
    let (year, month, day) = end_time.to_calendar_date();
    let (hour, minute, second) = end_time.to_hms();
    DateTime::new(
        Date::from_calendar_date(year, month.try_into().unwrap(), day).unwrap(),
        Time::from_hms(hour, minute, second).unwrap(),
    )
}
