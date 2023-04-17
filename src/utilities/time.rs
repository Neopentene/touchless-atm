#![allow(dead_code)]
use chrono::prelude::Utc;

/// An enum to better represent utc time methods
pub enum Time {
    Seconds,
    Millis,
    Micros,
    Nanos,
}

impl Time {
    pub fn timestamp(&self) -> i64 {
        match self {
            Self::Seconds => Utc::now().timestamp(),
            Self::Millis => Utc::now().timestamp_millis(),
            Self::Micros => Utc::now().timestamp_micros(),
            Self::Nanos => Utc::now().timestamp_nanos(),
        }
    }
}
