use crate::database::helpers::indexes::{
    account_indexes, admin_indexes, atm_indexes, token_indexes,
};
use crate::models::{atm::ATM, keys::KEY, token::JWT, transaction::TRANSACTION, user::ACCOUNT};
use crate::resolve_result;
use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::{sync::Client, sync::Collection};
use std::collections::HashMap;
use std::env;

use crate::models::{admin::ADMIN, logs::LOG};

pub struct Repository {
    pub admin: Collection<ADMIN>,
    pub atm: Collection<ATM>,
    pub account: Collection<ACCOUNT>,
    pub txn: Collection<TRANSACTION>,
    pub token: Collection<JWT>,
    pub logs: Collection<LOG>,
    pub keys: KEY,
}

impl Repository {
    pub async fn init() -> Result<Repository, String> {
        dotenv().ok();
        let mongo_uri = resolve_result!(name, _ -> env::var("LOCAL"); {name} | {
            return Err("Couldn't Load Variable URI".to_string())
        });

        let database = resolve_result!(name, _ -> env::var("DB_NAME"); {name} | {
            return Err("Couldn't Load Variable DB_NAME".to_string())
        });
        println!("Accessing the following URI: {mongo_uri}");

        let client = Client::with_uri_str(mongo_uri).unwrap();
        let database = client.database(&database);

        resolve_result!(_, _ ->  database.run_command(doc! {"ping": 1}, None); {println!("Server Connected");} | {
            return Err("Failed to Connect to Server".to_string());
        });

        let admin = database.collection("admin");
        let atm = database.collection("atm");
        let account = database.collection("account");
        let txn = database.collection("transaction");
        let logs = database.collection("logs");
        let token = database.collection("token");
        let keys = KEY::retrive_keys();

        let mut index_results = HashMap::new();
        index_results.insert("ADMIN", admin.create_index(admin_indexes(), None));
        index_results.insert("ACCOUNT", account.create_index(account_indexes(), None));
        index_results.insert("ATM", atm.create_index(atm_indexes(), None));
        index_results.insert("TOKEN", token.create_index(token_indexes(), None));

        for (name, result) in index_results {
            println!("Collection Name: {name}");
            println!("Result: {result:#?}");
        }

        Ok(Self {
            admin,
            atm,
            account,
            txn,
            logs,
            token,
            keys,
        })
    }
}
