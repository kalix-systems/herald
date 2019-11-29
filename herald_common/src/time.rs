use kson::*;
use std::{
    convert::TryInto,
    ops::Deref,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Ser, De)]
pub struct Time(i64);

fn u128_as_i64(u: u128) -> i64 {
    match u.try_into() {
        Ok(i) => i,
        Err(_) => i64::max_value(),
    }
}

impl Time {
    pub fn now() -> Self {
        let now = SystemTime::now();

        let secs = match now.duration_since(UNIX_EPOCH) {
            Ok(d) => u128_as_i64(d.as_millis()),
            Err(d) => -u128_as_i64(d.duration().as_millis()),
        };

        Time(secs)
    }

    pub fn within(
        self,
        fuzz: i64,
        other: Time,
    ) -> bool {
        (self.0 - other.0).abs() <= fuzz
    }

    pub fn as_i64(&self) -> &i64 {
        &self.0
    }
}

impl Deref for Time {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl From<i64> for Time {
    fn from(i: i64) -> Time {
        Time(i)
    }
}

impl Into<i64> for Time {
    fn into(self) -> i64 {
        self.0
    }
}

impl std::ops::Add for Time {
    type Output = Time;

    fn add(
        self,
        rhs: Time,
    ) -> Time {
        Time(self.0 + rhs.0)
    }
}

impl std::ops::Sub for Time {
    type Output = Time;

    fn sub(
        self,
        rhs: Time,
    ) -> Time {
        Time(self.0.saturating_sub(rhs.0))
    }
}
