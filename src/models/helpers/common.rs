use crate::utilities::time::Time;

#[macro_export]
macro_rules! pwd {
    ($($type:ty),+) => {
        use crate::utilities::crypto::hash_password_default;
        use rocket::http::Status;

        $(impl $type {
            pub fn hash_password(&mut self) -> Result<&mut Self, Status> {
                self.password = match hash_password_default(self.password.to_owned()) {
                    Ok(hash) => hash,
                    Err(error) => {
                        println!("Error while hashing: {error}");
                        return Err(Status::InternalServerError);
                    }
                };
                Ok(self)
            }
        })+
    };
}

pub fn timestamp() -> i64 {
    Time::Seconds.timestamp()
}

pub fn timestamp_millis() -> i64 {
    Time::Millis.timestamp()
}

pub fn copy_from_slice(slice: &mut [u8], other: &[u8]) {
    let limit = slice.len().min(other.len());
    for index in 0..limit {
        slice[index] = other[index].to_owned();
    }
}
