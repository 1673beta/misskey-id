//! 24 characters unique identifier for documents in database.
//! 
//! This is for compatibility with legacy version of misskey.
//! The first 4 bytes are a timestamp value representing the ObjectId's creation, specifically the number of seconds since the Unix epoch.
//! The last 8 bytes are a random value.
use std::{num::ParseIntError, time::SystemTime};

use chrono::{DateTime, TimeZone, Utc};
use lazy_static::lazy_static;
use rand::Rng;
use regex::Regex;

const CHARS: &str = "0123456789abcdef";
lazy_static! {
    static ref OBJECT_ID_REGEX: Regex = Regex::new(r"^[0-9a-f]{24}$").unwrap();
}

fn get_time(time: u64) -> String {
    let time_sec = time / 1000;
    format!("{:08x}", time_sec)
}

fn get_random() -> String {
    let mut rng = rand::rng();
    // 正確に16文字を生成するように制限
    (0..16)
        .map(|_| {
            let idx = rng.random_range(0..CHARS.len());
            CHARS.chars().nth(idx).unwrap()
        })
        .collect::<String>()
}

// TODO: Resultにする
pub fn gen_object_id(t: u64) -> String {
    format!("{}{}", get_time(t), get_random())
}

pub fn parse_object_id(id: &str) -> Result<SystemTime, ParseIntError> {
    let timestamp = u64::from_str_radix(&id[0..8], 16).unwrap();
    let time = SystemTime::UNIX_EPOCH + std::time::Duration::from_millis(timestamp * 1000);
    Ok(time)
}

pub fn parse_object_id_with_format(id: &str) -> DateTime<Utc> {
    let time = parse_object_id(id).unwrap();

    let duration = time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
    Utc.timestamp_millis_opt(duration.as_millis() as i64)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let generated = gen_object_id(1741525010000);
        println!("{}", generated);
        assert_eq!(generated.len(), 24);
        assert!(OBJECT_ID_REGEX.is_match(&generated));
    }

    #[test]
    fn test_parse() {}
}
