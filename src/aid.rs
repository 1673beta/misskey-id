/// # AID
/// AID is a 10 character long unique identifier.
/// The first 8 characters represent the time in milliseconds since the Unix epoch (2000-01-01T00:00:00Z).
/// The last 2 characters are a counter that increments with each new AID generated.
/// (c) mei23
/// https://misskey.m544.net/notes/71899acdcc9859ec5708ac24
use std::{
    num::ParseIntError,
    time::{SystemTime, UNIX_EPOCH},
};

use lazy_static::lazy_static;

use crate::radix::radix_encode;

const TIME2000: u64 = 946684800000;
static COUNTER: std::sync::atomic::AtomicU16 = std::sync::atomic::AtomicU16::new(0);

lazy_static! {
    pub static ref AID_REGEX: regex::Regex = regex::Regex::new(r"^[0-9a-z]{10}$").unwrap();
}

fn get_time(time: u64) -> String {
    let timestamp = std::cmp::max(0, time - TIME2000);
    format!("{:0>8}", radix_encode(timestamp as i64, 36).unwrap())
}

fn get_noise() -> String {
    let counter_val = COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    format!("{:0>2}", radix_encode(counter_val as i64, 36).unwrap())
}

pub fn gen(time: u64) -> Result<String, &'static str> {
    Ok(format!("{}{}", get_time(time), get_noise()))
}

pub fn parse(id: &str) -> Result<SystemTime, ParseIntError> {
    let time_part = &id[0..8];
    let time = u64::from_str_radix(time_part, 36)? + TIME2000;
    Ok(UNIX_EPOCH + std::time::Duration::from_millis(time))
}

pub fn parse_into_local_time(id: &str) -> Result<String, ParseIntError> {
    let systime = parse(id).unwrap();
    let datetime: chrono::DateTime<chrono::Local> = systime.into();
    Ok(datetime.to_rfc3339())
}

pub fn parse_into_utc(id: &str) -> Result<String, ParseIntError> {
    let systime = parse(id).unwrap();
    let datetime: chrono::DateTime<chrono::Utc> = systime.into();
    Ok(datetime.to_rfc3339())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let generated = gen(1741519768780).unwrap();
        println!("{}", generated);
        assert_eq!(generated.len(), 10);
        assert!(AID_REGEX.is_match(&generated));
    }

    #[test]
    fn test_parse() {
        let generated = gen(1741519768780).unwrap();
        let parsed = parse(&generated).unwrap();
        assert_eq!(
            parsed.duration_since(UNIX_EPOCH).unwrap().as_millis(),
            1741519768780
        );
    }

    #[test]
    fn test_parse_into_utc() {
        let generated = gen(1741519768780).unwrap();
        let parsed = parse_into_utc(&generated).unwrap();
        println!("{}", parsed);
    }
}
