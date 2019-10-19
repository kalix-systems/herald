use serde::*;
use std::{
    convert::TryInto,
    ops::Deref,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub struct Time(pub i64);

fn u64_as_i64(u: u64) -> i64 {
    match u.try_into() {
        Ok(i) => i,
        Err(_) => i64::max_value(),
    }
}

impl Time {
    pub fn now() -> Self {
        let now = SystemTime::now();

        let secs = match now.duration_since(UNIX_EPOCH) {
            Ok(d) => u64_as_i64(d.as_secs()),
            Err(d) => -u64_as_i64(d.duration().as_secs()),
        };

        Time(secs)
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
