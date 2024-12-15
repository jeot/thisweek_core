use chrono::{DateTime, Local};

pub fn get_unix_day_from_local_datetime(datetime: DateTime<Local>) -> i32 {
    // get the unix timestamp, add the local timezone offset, then calculate the day index
    let utc_epoch = datetime.to_utc().timestamp(); // Seconds since Unix epoch
    let offset_seconds = datetime.offset().local_minus_utc(); // offset seconds based on timezone
    let epoch_with_local_offset = utc_epoch + offset_seconds as i64;
    (epoch_with_local_offset / 3600 / 24) as i32
}

pub fn get_local_datetime_form_unix_day(day: i32) -> DateTime<Local> {
    // the reverse operation of get_unix_day()
    let offset_seconds = Local::now().offset().local_minus_utc();
    let sec: i64 = day as i64 * 3600 * 24 - offset_seconds as i64;
    let nano: u32 = 0;
    let datetime = DateTime::from_timestamp(sec, nano).expect("this should never happen!!");
    let datetime: DateTime<Local> = datetime.into();
    // println!("constructed datetime: {}", datetime);
    datetime
}
