//! A few basic types shared between crates.
//! Shared types related to API or other specific parts are defined in their corresponding module and not in here.

/// The default ID Type for referencing objects across the Paddlers services.
pub type PadlId = i64;

#[derive(Default, Debug, Copy, Clone, PartialEq, PartialOrd)]
/// Micro second precision
pub struct Timestamp(i64);
#[allow(dead_code)]
impl Timestamp {
    #[inline(always)]
    pub fn from_us(us: i64) -> Self {
        Timestamp(us)
    }
    #[inline(always)]
    pub fn from_millis(ms: i64) -> Self {
        Timestamp(ms * 1000)
    }
    #[inline(always)]
    pub fn from_seconds(s: i64) -> Self {
        Timestamp(s * 1_000_000)
    }
    #[inline(always)]
    pub fn micros(&self) -> i64 {
        self.0
    }
    #[inline(always)]
    pub fn millis(&self) -> i64 {
        self.0 / 1000
    }
    #[inline(always)]
    pub fn seconds(&self) -> i64 {
        self.0 / 1000_000
    }
}

impl std::ops::Add for Timestamp {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl std::ops::Sub for Timestamp {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

use chrono::Duration;
impl std::ops::Add<Duration> for Timestamp {
    type Output = Self;

    fn add(self, other: Duration) -> Self {
        Self(self.0 + other.num_microseconds().unwrap())
    }
}

impl std::ops::Sub<Duration> for Timestamp {
    type Output = Self;

    fn sub(self, other: Duration) -> Self {
        Self(self.0 - other.num_microseconds().unwrap())
    }
}
