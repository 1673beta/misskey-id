use std::time::{Duration, SystemTime, UNIX_EPOCH};
use ulid::{DecodeError, Generator, Ulid};

use chrono::{DateTime, Local};

pub fn parse(id: &str) -> Result<SystemTime, DecodeError> {
    let ulid = Ulid::from_string(id)?;
    let timestamp = ulid.timestamp_ms();
    Ok(UNIX_EPOCH + Duration::from_millis(timestamp))
}

pub fn formatted_time(id: &str) -> String {
    let systime = parse(id).unwrap();
    let datetime: DateTime<Local> = systime.into();
    datetime.to_rfc3339()
}

pub fn gen_ulid(time: u64) -> String {
    let mut gen = Generator::new();
    let now = SystemTime::UNIX_EPOCH + Duration::from_millis(time);
    let ulid = gen.generate_from_datetime(now).unwrap().to_string();
    ulid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let generated = gen_ulid(1741525010000);
        println!("{}", generated);
        assert_eq!(generated.len(), 26);
    }

    #[test]
    fn test_parse() {
        let generated = gen_ulid(1741535238126);
        let parsed = parse(&generated).unwrap();
        assert_eq!(parsed, UNIX_EPOCH + Duration::from_millis(1741535238126));
    }
}