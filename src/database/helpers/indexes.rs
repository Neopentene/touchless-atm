use std::time::Duration;

use mongodb::{bson::doc, options::IndexOptions, IndexModel};

use crate::models::token::TOKEN;

pub fn admin_indexes() -> IndexModel {
    let options = IndexOptions::builder().unique(true).build();
    IndexModel::builder()
        .keys(doc! {
            "username": 1,
        })
        .options(options)
        .build()
}

pub fn atm_indexes() -> IndexModel {
    let options = IndexOptions::builder().unique(true).build();
    IndexModel::builder()
        .keys(doc! {
            "name": 1,
        })
        .options(options)
        .build()
}

pub fn account_indexes() -> IndexModel {
    let options = IndexOptions::builder().unique(true).build();
    IndexModel::builder()
        .keys(doc! {
            "number": 1,
        })
        .options(options)
        .build()
}

pub fn token_indexes() -> IndexModel {
    let duration = Duration::from_millis(TOKEN::DEFAULT_EXPIRY as u64);
    let options = IndexOptions::builder().expire_after(duration).build();
    IndexModel::builder()
        .keys(doc! {
            "created": 1,
        })
        .options(options)
        .build()
}
