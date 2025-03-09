/// Encode a number in a given radix.
/// # Arguments
/// * `num` - The number to encode.
/// * `radix` - The radix to encode the number in. Must be between 2 and 36.
/// # Returns
/// A string containing the number encoded in the given radix.
/// # Examples
/// ```
/// use misskey_id::radix::radix_encode;
///
/// let encoded_binary = radix_encode(10, 2).unwrap();
/// assert_eq!(encoded_binary, "1010");
///
/// let encoded_36 = radix_encode(1024, 36).unwrap();
/// assert_eq!(encoded_36, "sg");
///
/// let encoded_64 = radix_encode(1024, 64);
/// assert!(encoded_64.is_err());
///
/// ```
pub fn radix_encode(mut num: i64, radix: u32) -> Result<String, Box<dyn std::error::Error>> {
    if num == 0 {
        return Ok("0".to_string());
    }

    if radix < 2 || radix > 36 {
        return Err("Radix must be between 2 and 36".into());
    }

    let mut result = String::new();
    while num > 0 {
        let digit = (num % radix as i64) as u32;
        result.insert(0, std::char::from_digit(digit, radix).unwrap());
        num /= radix as i64;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radix_encode() {
        assert_eq!(radix_encode(0, 36).unwrap(), "0");
        assert_eq!(radix_encode(10, 2).unwrap(), "1010"); // 2進数
        assert_eq!(radix_encode(10, 8).unwrap(), "12"); // 8進数
        assert_eq!(radix_encode(10, 10).unwrap(), "10"); // 10進数
        assert_eq!(radix_encode(10, 16).unwrap(), "a"); // 16進数
        assert_eq!(radix_encode(1024, 36).unwrap(), "sg"); // 36進数
        assert_eq!(radix_encode(2000000, 36).unwrap(), "16v7k"); // 大きな数

        assert!(radix_encode(10, 1).is_err()); // 1進数はエラー
        assert!(radix_encode(10, 37).is_err()); // 37進数はエラー
    }

    #[test]
    fn test_radix_println() {
        println!("{}", radix_encode(1741519768780, 36).unwrap());
    }
}
