use crate::models::user::ACCOUNT;

impl ACCOUNT {
    pub fn validate(&self) -> Result<&Self, String> {
        if self.password.len() < 8 {
            return Err("Password too short".to_string());
        }
        Ok(self)
    }
}

impl Clone for ACCOUNT {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.to_owned(),
            password: self.password.to_owned(),
            number: self.number.to_owned(),
            token: self.token.to_owned(),
            balance: self.balance,
            pin: self.pin,
        }
    }
}
