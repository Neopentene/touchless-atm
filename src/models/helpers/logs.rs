use super::super::logs::Type;
use std::str::FromStr;

impl Type {
    pub fn value(&self) -> String {
        match self {
            Type::ADMIN => "ADMIN".to_string(),
            Type::ACCOUNT => "ACCOUNT".to_string(),
            Type::ATM => "ATM".to_string(),
            Type::TRANSACTION => "TRANSACTION".to_string(),
            Type::TOKEN => "TOKEN".to_string(),
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

impl FromStr for Type {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "ADMIN" => Ok(Type::ADMIN),
            "ACCOUNT" => Ok(Type::ACCOUNT),
            "USER" => Ok(Type::ACCOUNT),
            "ATM" => Ok(Type::ATM),
            "TRANSACTION" => Ok(Type::TRANSACTION),
            "TOKEN" => Ok(Type::TOKEN),
            _ => Err("Invalid Value".to_string()),
        }
    }
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::ADMIN => "ADMIN".to_string(),
            Type::ACCOUNT => "ACCOUNT".to_string(),
            Type::ATM => "ATM".to_string(),
            Type::TRANSACTION => "TRANSACTION".to_string(),
            Type::TOKEN => "TOKEN".to_string(),
        }
    }
}
