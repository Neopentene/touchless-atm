#[macro_export]
macro_rules! status_ok_action {
    ($result:expr, $action:literal, $status:expr) => {
        match $result {
            Ok(result) => {
                println!("{} succeeded", $action);
                Ok(result)
            }
            Err(_) => {
                println!("{} failed", $action);
                Err($status)
            }
        }
    };
}

#[macro_export]
macro_rules! status_ok {
    ($result:expr, $status:expr) => {
        match $result {
            Ok(result) => Ok(result),
            Err(_) => Err($status),
        }
    };
}

#[macro_export]
macro_rules! status_if {
    ($expression:expr, $status:expr) => {
        if $expression {
            return Err($status);
        }
    };
}

#[macro_export]
macro_rules! check_ok_401 {
    ($result:expr) => {
        crate::status_ok!($result, rocket::http::Status::Unauthorized)
    };

    ($result:expr, $action:literal) => {
        crate::status_ok_action!($result, $action, rocket::http::Status::Unauthorized)
    };
}

#[macro_export]
macro_rules! check_ok_404 {
    ($result:expr) => {
        crate::status_ok!($result, rocket::http::Status::NotFound)
    };

    ($result:expr, $action:literal) => {
        crate::status_ok_action!($result, $action, rocket::http::Status::NotFound)
    };
}

#[macro_export]
macro_rules! check_ok_406 {
    ($result:expr) => {
        crate::status_ok!($result, rocket::http::Status::NotAcceptable)
    };

    ($result:expr, $action:literal) => {
        crate::status_ok_action!($result, $action, rocket::http::Status::NotAcceptable)
    };
}

#[macro_export]
macro_rules! check_ok_500 {
    ($result:expr) => {
        crate::status_ok!($result, rocket::http::Status::InternalServerError)
    };

    ($result:expr, $action:literal) => {
        crate::status_ok_action!($result, $action, rocket::http::Status::InternalServerError)
    };
}

#[macro_export]
macro_rules! check_if_401 {
    ($expression:expr) => {
        crate::status_if!($expression, rocket::http::Status::Unauthorized)
    };
}

#[macro_export]
macro_rules! check_if_400 {
    ($expression:expr) => {
        crate::status_if!($expression, rocket::http::Status::BadRequest)
    };
}

#[macro_export]
macro_rules! check_if_406 {
    ($expression:expr) => {
        crate::status_if!($expression, rocket::http::Status::NotAcceptable)
    };
}

#[macro_export]
macro_rules! seal_details {
    ($object:expr) => {
        $object.id = None;
        $object.password = "$SEALED$".to_string();
    };
}
