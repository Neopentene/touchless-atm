#[macro_export]
macro_rules! find_one {
    ($collection:expr, $options:ident, $(($key:expr, $value:expr)),*) => {{
        let mut filter = mongodb::bson::Document::new();
        $(filter.insert($key.to_owned(), $value.to_owned());)*
        match $collection.find_one(filter, $options) {
            Ok(result) => match result {
                Some(val) => Ok(val),
                None => Err(rocket::http::Status::NotFound),
            },
            Err(_) => Err(rocket::http::Status::InternalServerError),
        }
    }};

    ($collection:expr, $options:ident, $filter:ident) => {{
        match $collection.find_one($filter, $options) {
            Ok(result) => match result {
                Some(val) => Ok(val),
                None => Err(rocket::http::Status::NotFound),
            },
            Err(_) => Err(rocket::http::Status::InternalServerError),
        }
    }};

    ($collection:expr, $options:ident, _id => $id:expr) => {{
        match mongodb::bson::oid::ObjectId::parse_str($id) {
            Ok(id) => {
                let mut filter = mongodb::bson::Document::new();
                filter.insert("_id", id);
                match $collection.find_one(filter, $options) {
                    Ok(result) => match result {
                        Some(val) => Ok(val),
                        None => Err(rocket::http::Status::NotFound),
                    },
                    Err(_) => Err(rocket::http::Status::InternalServerError),
                }
            },
            Err(_) => Err(rocket::http::Status::NotFound),
        }
    }};
}

#[macro_export]
macro_rules! find_one_and_update {
    ($collection:expr, $options:ident, $update:ident, $(($key:expr, $value:expr)),*) => {{
        let mut filter = mongodb::bson::Document::new();
        $(filter.insert($key.to_owned(), $value.to_owned());)*

        match $collection.find_one_and_update(filter, $update, $options) {
            Ok(result) => match result {
                Some(val) => Ok(val),
                None => Err(rocket::http::Status::NotFound),
            },
            Err(_) => Err(rocket::http::Status::InternalServerError),
        }
    }};

    ($collection:expr, $options:ident, $update:ident, $filter:ident) => {{
        match $collection.find_one_and_update($filter, $update, $options) {
            Ok(result) => match result {
                Some(val) => Ok(val),
                None => Err(rocket::http::Status::NotFound),
            },
            Err(_) => Err(rocket::http::Status::InternalServerError),
        }
    }};
}

#[macro_export]
macro_rules! update_one {
    ($collection:expr, $update:ident, $options:ident, $(($key:expr, $value:expr)),*) => {{
        let mut query = mongodb::bson::Document::new();
        $(query.insert($key.to_owned(), $value.to_owned());)*

        match $collection.update_one(query, $update, $options) {
            Ok(result) => Ok(result),
            Err(_) => Err(rocket::http::Status::InternalServerError),
        }
    }};

    ($collection:expr, $update:ident, $options:ident, $query:ident) => {{
        match $collection.update_one($query, $update, $options) {
            Ok(result) => Ok(result),
            Err(_) => Err(rocket::http::Status::InternalServerError),
        }
    }};
}

#[macro_export]
macro_rules! update_many {
    ($collection:expr, $update:ident, $options:ident, $(($key:expr, $value:expr)),*) => {{
        let mut query = mongodb::bson::Document::new();
        $(query.insert($key.to_owned(), $value.to_owned());)*

        match $collection.update_many(query, $update, $options) {
            Ok(result) => Ok(result),
            Err(_) => Err(rocket::http::Status::InternalServerError),
        }
    }};

    ($collection:expr, $update:ident, $options:ident, $query:ident) => {{
        match $collection.update_many($query, $update, $options) {
            Ok(result) => Ok(result),
            Err(_) => Err(rocket::http::Status::InternalServerError),
        }
    }};
}

#[macro_export]
macro_rules! log_action {
    ($collection:expr, $creator:ident, $timestamp:expr, $subject:expr, $role:expr, $affected_id:expr) => {{
        let log = crate::models::logs::LOG::new()
            .creator($creator)
            .timestamp($timestamp)
            .role($role)
            .creation($affected_id, $subject)
            .build();
        crate::check_result!(insert_one!($collection, log, None), "Logging");
    }};

    ($collection:expr, $creator:ident, $timestamp:expr, $role:expr, $affected_id:expr) => {{
        let log = crate::models::logs::LOG::new()
            .creator($creator)
            .timestamp($timestamp)
            .role($role)
            .generation($affected_id)
            .build();
        crate::check_result!(crate::insert_one!($collection, log, None), "Logging");
    }};
}

#[macro_export]
macro_rules! create_one {
    ($collection:expr, $logs_collection:expr, $data:ident, $options:ident, $creator:ident, $timestamp:expr, $subject:expr, $role:expr) => {{
        let affected_id = crate::insert_one!($collection, $data, $options)?
            .inserted_id
            .as_object_id();
        match affected_id {
            Some(affected_id) => {
                crate::log_action!(
                    $logs_collection,
                    $creator,
                    $timestamp,
                    $subject,
                    $role,
                    affected_id
                );
                Ok(())
            }
            None => {
                println!("Affected Id is None");
                Err(rocket::http::Status::BadRequest)
            }
        }
    }};
}

#[macro_export]
macro_rules! generate_one {
    ($collection:expr, $logs_collection:expr, $data:ident, $options:ident, $creator:ident, $timestamp:expr, $role:expr) => {{
        let affected_id = crate::insert_one!($collection, $data, $options)?
            .inserted_id
            .as_object_id();
        match affected_id {
            Some(affected_id) => {
                crate::log_action!($logs_collection, $creator, $timestamp, $role, affected_id);
                Ok(())
            }
            None => {
                println!("Affected Id is None");
                Err(rocket::http::Status::BadRequest)
            }
        }
    }};
}

#[macro_export]
macro_rules! insert_one {
    ($collection:expr, $data:ident, $options:ident) => {{
        match $collection.insert_one($data, $options) {
            Ok(result) => Ok(result),
            Err(error) => match *error.kind {
                mongodb::error::ErrorKind::Write(_) => Err(rocket::http::Status::Conflict),
                _ => Err(rocket::http::Status::InternalServerError),
            },
        }
    }};
}
