use crate::models::admin::{Role, ADMIN};
use std::{
    cmp::Ordering::{Equal, Greater, Less},
    str::FromStr,
};

impl FromStr for Role {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "STAFF" => Ok(Role::STAFF),
            "COORIDNATOR" => Ok(Role::COORIDNATOR),
            "SUPERVISOR" => Ok(Role::SUPERVISOR),
            "MANAGER" => Ok(Role::MANAGER),
            "EXECUTIVE" => Ok(Role::EXECUTIVE),
            "MANAGEMENT" => Ok(Role::MANAGEMENT),
            _ => Err("Invalid Value".to_string()),
        }
    }
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::STAFF => "STAFF".to_string(),
            Role::COORIDNATOR => "COORIDNATOR".to_string(),
            Role::SUPERVISOR => "SUPERVISOR".to_string(),
            Role::MANAGER => "MANAGER".to_string(),
            Role::EXECUTIVE => "EXECUTIVE".to_string(),
            Role::MANAGEMENT => "MANAGEMENT".to_string(),
        }
    }
}

impl PartialOrd for Role {
    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Less | Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Greater | Equal))
    }

    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.value() == other.value() {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.value() > other.value() {
            return Some(std::cmp::Ordering::Greater);
        }
        return Some(std::cmp::Ordering::Less);
    }
}

impl Ord for Role {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.value() == other.value() {
            return std::cmp::Ordering::Equal;
        }
        if self.value() > other.value() {
            return std::cmp::Ordering::Greater;
        }
        return std::cmp::Ordering::Less;
    }
}

impl ADMIN {
    pub fn validate(&self) -> Result<&Self, String> {
        if self.username.len() < 8 {
            return Err("Username too short".to_string());
        }
        if self.password.len() < 8 {
            return Err("Password too short".to_string());
        }
        if self.role.is_none() {
            return Err("Role has to be provided".to_string());
        }
        Ok(self)
    }
}

impl Clone for ADMIN {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            username: self.username.to_owned(),
            password: self.password.to_owned(),
            role: self.role,
            token: self.token.to_owned(),
        }
    }
}
