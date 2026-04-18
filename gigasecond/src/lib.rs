use time::{PrimitiveDateTime as DateTime, UtcDateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let giga = 1_000_000_000;
    let utc = UtcDateTime::new(start.date(), start.time());
    let epoch = utc.unix_timestamp();
    let target = epoch + giga;

    match UtcDateTime::from_unix_timestamp(target) {
        Ok(result) => DateTime::new(result.date(), result.time()),
        Err(_) => start,
    }

    // assume date and time 1 gigasecond after a certain date

    // todo!("What time is a gigasecond later than {start}");
}
