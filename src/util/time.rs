use std::time::{SystemTime, UNIX_EPOCH, Duration};
use chrono::prelude::DateTime;
use chrono::format::DelayedFormat;
use chrono::format::strftime::StrftimeItems;
use chrono::Local;

pub struct UTime;

impl UTime {
    pub fn now() -> u64 {
        let now = SystemTime::now();
        let delta = now.duration_since(UNIX_EPOCH).unwrap();
        delta.as_micros() as u64
    }

    pub fn format<'a>(timestamp: u64, format: &'a str) -> DelayedFormat<StrftimeItems<'a>> {
        let d = UNIX_EPOCH + Duration::from_micros(timestamp);
        let dt = DateTime::<Local>::from(d);
        dt.format(format)
    }

    pub fn datetime(timestamp: u64) -> DelayedFormat<StrftimeItems<'static>> {
        UTime::format(timestamp, "%Y-%m-%d %H:%M:%S.%6f")
    }
}
