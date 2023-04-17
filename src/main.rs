// #![allow(unused_imports)] // TODO -> Clear out unused imports - Almost complete
mod database;
mod errors;
mod models;
mod routes;
mod utilities;

#[macro_use]
extern crate rocket;
extern crate dotenv;

use database::repository::Repository;
use dotenv::dotenv;
use errors::catchers::*;
use rocket::Config;
use routes::{create::*, details::*, login::*, transaction::*};
use std::{env, net::Ipv4Addr, str::FromStr};
use utilities::cors::*;

// TODO -> Use resolve_result macro in place of match clauses

#[launch]
async fn rocket() -> _ {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE", "full");
    let repository = match Repository::init().await {
        Ok(repository) => repository,
        Err(error) => panic!("Failed to initialize repository: {}", error),
    };

    let url = resolve_result!(name, _ -> env::var("URL"); {name} | {
        panic!("Couldn't Load Variable URL")
    });

    let config = Config {
        port: 8000,
        address: Ipv4Addr::from_str(&url).unwrap().into(),
        ..Config::debug_default()
    };

    rocket::build()
        .manage(repository)
        .configure(config)
        .attach(CORS)
        .register(
            "/",
            catchers![
                bad_request,
                conflict,
                internal_error,
                not_found,
                unauthorized,
                not_acceptable,
                forbidden
            ],
        )
        .mount("/", routes![route_options])
        .mount("/", routes![login_admin, login_atm, login_account])
        .mount("/", routes![create_admin, create_atm, create_account])
        .mount(
            "/",
            routes![get_atm, get_account, get_atm_admin, get_account_admin],
        )
        .mount("/", routes![create_txn_account, create_txn_atm])
        .mount("/", routes![confirm_txn_account, confirm_txn_atm])
        .mount(
            "/",
            routes![get_otp_account, get_otp_atm, get_txn_status_atm],
        )
}

// Testing
// fn main() {
//     env::set_var("RUST_BACKTRACE", "full");
//     let time = utilities::time::Time::Nanos.timestamp();

//     // let mut temp = [0u8; 16];

//     // let gseed = Generator::generate_seed();
//     // let gbytes = Generator::generate_random_bytes();
//     // let gsecret = Generator::generate_nonce(&mut temp);
//     // let rate = 123;
//     // let gseed = Generator::generate_seed();
//     // let gbytes = Generator::generate_random_bytes();
//     // let gsecret = Generator::generate_nonce(&mut temp);
//     // let rate = 123;
//     // let gseed = Generator::generate_seed();
//     // let gbytes = Generator::generate_random_bytes();
//     // let gsecret = Generator::generate_nonce(&mut temp);
//     // let rate = 123;

//     let seed = from_hex("92591f12a8832c5789f72d47".to_string()).unwrap();
//     let secret = from_hex("7e7cbe01c15af4e898c6648076c1a802".to_string()).unwrap();
//     let seed = seed.as_slice();
//     let secret = secret.as_slice();
//     let bytes = Generator::generate_random_bytes();
//     let rate = 167;

//     // let test = Response::<Vec<u8>>::new()
//     //     .message("It works".to_string())
//     //     .error("No Error".to_string())
//     //     .build();
//     // println!("Response: {:#?}", test);

//     // let token = TOKEN::new("Gary".to_string(), "Admin".to_string()).unwrap();
//     // println!("Token: {:#?}", token);

//     // let jwt = "Bearer ".to_string() + &token.to_jwt(bytes, secret, seed, rate).unwrap();

//     // println!(
//     //     "Token: {:#?}",
//     //     TOKEN::from_jwt(jwt, bytes, secret, seed, rate).unwrap()
//     // );

//     // let limit = 12usize % (12usize + 1);
//     // let limit = if limit == 0usize { 1usize } else { limit };

//     // println!("{:#?}", (0..limit).map(|x| x as i32).collect::<Vec<i32>>());

//     // let gseed = Generator::generate_seed();
//     // let gbytes = Generator::generate_random_bytes();
//     // let gsecret = Generator::generate_nonce(&mut temp);
//     // let rate = 123;

//     // println!("Dummy data: {gseed:?}, {gbytes:?}, {gsecret:?}, {rate}");

//     // println!(
//     //     "Total time taken: {}",
//     //     utilities::time::Time::Nanos.timestamp() - time
//     // );

//     // let file = OpenOptions::new()
//     //     .create(true)
//     //     .write(true)
//     //     .read(true)
//     //     .truncate(true)
//     //     .open("Test.txt");

//     // let mut file = match file {
//     //     Ok(file) => {
//     //         println!("File Created");
//     //         Ok(file)
//     //     }
//     //     Err(error) => {
//     //         println!("File Error: {:#?}", error);
//     //         Err(error)
//     //     }
//     // };

//     // let keys = KEY::retrive_keys();
//     // println!("Keys: {:#?}", keys);
//     // let seed_len = keys.seed.len();
//     // let secret_len = keys.secret.len();
//     // let bytes_len = keys.bytes.len();

//     // println!("Lengths: \n\t{seed_len}\n\t{secret_len}\n\t{bytes_len}");

//     // let token = TOKEN::new("adminNeo".to_string(), "Admin".to_string()).unwrap();
//     // println!("Token: {:#?}", token);
//     // let jwt = token.to_jwt(keys.bytes, &keys.secret, &keys.seed, keys.rate);
//     // println!("JWT: {:#?}", jwt);

//     // fn function_test<T: Sized + Sync + Send + Unpin>() {
//     //     println!("Yes it works");
//     // }

//     // function_test::<TOKEN>();
//     // function_test::<ADMIN>();
//     // function_test::<ATM>();
//     // function_test::<Repository>();
//     // function_test::<KEY>();

//     // let location = Location::new(89.21, 144.23).unwrap();
//     // println!("Location: {:#?}", location);
//     // println!("Latitude: {}", location.0.to_decimal());
//     // println!("Longitude: {}", location.1.to_decimal());

//     // let rt = runtime::Builder::new_current_thread()
//     //     .enable_all()
//     //     .build()
//     //     .unwrap();

//     // let repo = rt.block_on(async { Repository::init().await }).unwrap();
//     // let admin = rt
//     //     .block_on(async { repo.get_admin("adminNeo".to_string()).await })
//     //     .unwrap();

//     // let location = Location::new(78.12314, 101.45).unwrap();

//     // println!("{location:#?}");
//     // println!("{}, {}", location.0.to_decimal(), location.1.to_decimal());

//     // let atm = ATM::new(
//     //     None,
//     //     "Test".to_string(),
//     //     "Test".to_string(),
//     //     "Test".to_string(),
//     //     location,
//     // );

//     // let result = rt.block_on(async { repo.get_atm("Test".to_string()).await });
//     // println!("{result:#?}");

//     // let rt = runtime::Builder::new_current_thread()
//     //     .enable_all()
//     //     .build()
//     //     .unwrap();

//     // let repo = rt.block_on(async { Repository::init().await }).unwrap();
//     // let admin = rt
//     //     .block_on(async { repo.get_admin("adminNeo".to_string()).await })
//     //     .unwrap();
//     // let admin = rt
//     //     .block_on(async { repo.get_admin("adminNeo".to_string()).await })
//     //     .unwrap();
//     // let collection = &repo.atm;
//     // let atm = find_one!(collection, None, ("name", "Test"));

//     // let login = rt.block_on(async { repo.login_admin("adminNeo".to_string()).await });

//     // println!("ADMIN: {admin:#?}, \nATM: {atm:#?}");
//     // println!("LOGIN: {login:#?}");

//     // let mut new_admin = ADMIN::new(
//     //     None,
//     //     "testingNeo".to_string(),
//     //     "password".to_string(),
//     //     Some(Role::STAFF),
//     // );

//     // new_admin
//     //     .hash_password()
//     //     .expect("Ohh! Hashing failed I see");

//     // match rt.block_on(async { repo.create_admin(admin.id, new_admin).await }) {
//     //     Ok(_) => println!("New Admin is here baby"),
//     //     Err(error) => println!("Crap it failed => {error:#?}"),
//     // };
//     let details = "6431286488ee17304b073a33".to_string() + "64312677c24244f68af56d94";
//     let ecrypted_details = encrypt(bytes, details.to_owned(), secret, seed, rate);

//     let decrypted_details = decrypt(
//         bytes,
//         ecrypted_details.to_owned().unwrap(),
//         secret,
//         seed,
//         rate,
//     );

//     let utfs = String::from_utf8(from_hex(decrypted_details.to_owned().unwrap()).unwrap()).unwrap();

//     println!("Details: {details}");
//     println!("Encrypted: {ecrypted_details:#?}");
//     println!("Decrypted: {decrypted_details:#?}");
//     println!("String: {utfs}");
// }

// fn main() {
//     let rt = runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//         .unwrap();

//     let repo = rt.block_on(async { Repository::init().await }).unwrap();
//     let atm = rt
//         .block_on(async { repo.get_atm_from_id("6431461f63c6a92f65459f82").await })
//         .unwrap();

//     let txn = rt
//         .block_on(async { repo.get_pending_txn("atm", &atm.name).await })
//         .unwrap();

//     let response = Response::<String>::new().data(txn.status.value()).clone();

//     println!("status: {response:#?}");
// }
