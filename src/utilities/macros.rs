#[macro_export]
macro_rules! check_result {
    ($result:expr, $action:literal) => {{
        match $result {
            Ok(_) => println!("{} succeeded", $action),
            Err(_) => println!("{} failed", $action),
            #[allow(unreachable_patterns)]
            _ => println!("{} has different match arms", $action),
        };
    }};
}

#[macro_export]
macro_rules! resolve_result {
    ($name:ident, $err:ident -> $result:expr; $action:block | $error:block) => {
        match $result {
            Ok($name) => $action,
            Err($err) => $error,
        }
    };

    ($name:ident, _ -> $result:expr; $action:block | $error:block) => {
        match $result {
            Ok($name) => $action,
            Err(_) => $error,
        }
    };

    (_, _ -> $result:expr; $action:block | $error:block) => {
        match $result {
            Ok(_) => $action,
            Err(_) => $error,
        }
    };
}

#[macro_export]
macro_rules! option {
    ($name:ident -> $option:expr; $action:block | $error:block) => {
        match $option {
            Some($name) => $action,
            None => $error,
        }
    };
}
