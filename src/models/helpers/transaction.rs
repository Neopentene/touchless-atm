use super::common::timestamp;
use crate::models::transaction::{TxnStatus, TxnType, TRANSACTION};
use std::str::FromStr;

impl TxnType {
    pub fn value(&self) -> String {
        match self {
            TxnType::DEBIT => "DEBIT".to_string(),
            TxnType::CREDIT => "CREDIT".to_string(),
        }
    }

    pub fn cmp(&self, value: &String) -> bool {
        match Self::from_str(value) {
            Ok(value) => {
                if value.value() == self.value() {
                    return true;
                }
                false
            }
            Err(_) => false,
        }
    }
}

impl FromStr for TxnType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "DEBIT" => Ok(TxnType::DEBIT),
            "CREDIT" => Ok(TxnType::CREDIT),
            _ => Err("Invalid Value".to_string()),
        }
    }
}

impl ToString for TxnType {
    fn to_string(&self) -> String {
        match self {
            TxnType::DEBIT => "DEBIT".to_string(),
            TxnType::CREDIT => "CREDIT".to_string(),
        }
    }
}

impl PartialOrd for TxnType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match &self {
            TxnType::DEBIT => Some(std::cmp::Ordering::Equal),
            TxnType::CREDIT => Some(std::cmp::Ordering::Equal),
        }
    }

    fn lt(&self, other: &Self) -> bool {
        false
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        false
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Equal))
    }
}

impl TxnStatus {
    pub fn value(&self) -> String {
        match self {
            TxnStatus::PENDING => "PENDING".to_string(),
            TxnStatus::EXPIRED => "EXPIRED".to_string(),
            TxnStatus::COMPLETE => "COMPLETE".to_string(),
            TxnStatus::REJECTED => "REJECTED".to_string(),
        }
    }

    pub fn cmp(&self, value: &String) -> bool {
        match Self::from_str(value) {
            Ok(value) => {
                if value.value() == self.value() {
                    return true;
                }
                false
            }
            Err(_) => false,
        }
    }
}

impl FromStr for TxnStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "PENDING" => Ok(TxnStatus::PENDING),
            "EXPIRED" => Ok(TxnStatus::EXPIRED),
            "COMPLETE" => Ok(TxnStatus::COMPLETE),
            "REJECTED" => Ok(TxnStatus::REJECTED),
            _ => Err("Invalid Value".to_string()),
        }
    }
}

impl ToString for TxnStatus {
    fn to_string(&self) -> String {
        match self {
            TxnStatus::PENDING => "PENDING".to_string(),
            TxnStatus::EXPIRED => "EXPIRED".to_string(),
            TxnStatus::COMPLETE => "COMPLETE".to_string(),
            TxnStatus::REJECTED => "REJECTED".to_string(),
        }
    }
}

impl TRANSACTION {
    pub fn is_valid(&mut self, balance: i64) -> bool {
        if self.created.timestamp_millis() + Self::DEFAULT_EXPIRY < timestamp() {
            self.status = TxnStatus::EXPIRED;
            return false;
        }
        if self.txn_type == TxnType::DEBIT && balance < self.amount {
            self.status = TxnStatus::REJECTED;
            return false;
        }
        true
    }
}
