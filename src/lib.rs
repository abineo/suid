//! Sortable (timestamp-first) unique identifier
//!
//! ## Why?
//!
//! Random identifiers can be bad for performance when inserting into [_balanced_ b-tree](https://en.wikipedia.org/wiki/B-tree) indexes.
//!
//! ## Variants
//!
//! | variant        | timestamp bits                           | random bits |
//! | -------------- | -----------------------------------------| ----------- |
//! | `u32`, `i32`   | **16** _(hours since unix epoch)_        | **16**      |
//! | `u64`, `i64`   | **40** _(seconds since unix epoch)_      | **24**      |
//! | `u128`, `i128` | **64** _(milliseconds since unix epoch)_ | **64**      |
//!
//! ## Usage
//! 
//! ```
//! use suid::Suid;
//!
//! println!("{}", u32::suid());
//! println!("{}", i32::suid());
//! println!("{}", u64::suid());
//! println!("{}", i64::suid());
//! println!("{}", u128::suid());
//! println!("{}", i128::suid());
//! ```
//! 
//! ## License
//! 
//! [☕ Coffee License 2.0](https://coffee-license.org/v2.0)

use std::time::{SystemTime, UNIX_EPOCH};

use rand::random;

pub trait Suid {
    fn suid() -> Self;
}

impl Suid for u32 {
    /// 16 bit timestamp (hours since `1970-01-01 00:00:00`) + 16 random bits
    fn suid() -> u32 {
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("UNIX_EPOCH is before now")
            .as_secs();
        let hours = (ts / (60 * 60 * 24)) as u32;

        let rando = random::<u32>();

        (hours << 16) | (rando << 16 >> 16)
    }
}

impl Suid for i32 {
    /// 16 bit timestamp (hours since `1970-01-01 00:00:00`) + 16 random bits
    ///
    /// literally just `u32::suid() as i32`
    fn suid() -> i32 {
        u32::suid() as i32
    }
}

impl Suid for u64 {
    /// 40 bit unix timestamp (seconds since `1970-01-01 00:00:00`) + 24 random bits
    fn suid() -> u64 {
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("UNIX_EPOCH is before now")
            .as_secs();

        let rando = random::<u64>();

        (secs << 24) | (rando << 40 >> 40)
    }
}

impl Suid for i64 {
    /// 40 bit unix timestamp (seconds since `1970-01-01 00:00:00`) + 24 random bits
    ///
    /// literally just `u64::suid() as i64`
    fn suid() -> i64 {
        u64::suid() as i64
    }
}

impl Suid for u128 {
    /// 64 bit timestamp (milliseconds since `1970-01-01 00:00:00`) + 64 random bits
    fn suid() -> u128 {
        let millis = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("UNIX_EPOCH is before now")
            .as_millis();

        let rando = random::<u128>();

        (millis << 64) | (rando << 64 >> 64)
    }
}

impl Suid for i128 {
    /// 64 bit timestamp (milliseconds since `1970-01-01 00:00:00`) + 64 random bits
    ///
    /// literally just `u128::suid() as i128`
    fn suid() -> i128 {
        u128::suid() as i128
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_u64() {
        let first = u64::suid();
        sleep(Duration::from_secs(1));
        let second = u64::suid();
        assert!(first < second);
    }

    #[test]
    fn test_i64() {
        let first = i64::suid();
        sleep(Duration::from_secs(1));
        let second = i64::suid();
        assert!(first < second);
    }

    #[test]
    fn test_u128() {
        let first = u128::suid();
        sleep(Duration::from_millis(1));
        let second = u128::suid();
        assert!(first < second);
    }

    #[test]
    fn test_i128() {
        let first = i128::suid();
        sleep(Duration::from_millis(1));
        let second = i128::suid();
        assert!(first < second);
    }
}
