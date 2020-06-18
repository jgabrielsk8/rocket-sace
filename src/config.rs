use rocket::config::{Config, Environment, Value};
use std::env;
use std::collections::HashMap;

/// js toISOString() in test suit can't handle chrono's default precision
// pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";


// pub const TOKEN_PREFIX: &'static str = "Token ";

// pub struct AppState {
//     pub secret: Vec<u8>,
// }

// impl AppState {
//     pub fn manage() -> AdHoc {
//         AdHoc::on_attach("Manage config", |rocket| {
//             // Rocket doesn't expose it's own secret_key, so we use our own here.
//             let secret = env::var("ROCKET_SECRET_KEY").unwrap_or_else(|err| {
//               panic!("No SECRET_KEY environment variable found: {:?}", err)
//             });

//             Ok(rocket.manage(AppState{secret: secret.into_bytes()}))
//         })
//     }
// }


/// Create rocket config from environment variables
pub fn from_env() -> Config {
    let environment = Environment::active().expect("No environment found");

    let address = env::var("ROCKET_ADDRESS")
        .unwrap_or_else(|_| "localhost".to_string())
        .parse::<String>()
        .expect("ADDRESS environment variable should parse to a string");

    let port = env::var("ROCKET_PORT")
        .unwrap_or_else(|_| "8000".to_string())
        .parse::<u16>()
        .expect("PORT environment variable should parse to an integer");

    let mut database_config = HashMap::new();
    let mut databases = HashMap::new();
    let database_url =
        env::var("ROCKET_DATABASE_URL").expect("No DATABASE_URL environment variable found");
    database_config.insert("url", Value::from(database_url));
    databases.insert("sace_rocket", Value::from(database_config));

    Config::build(environment)
        .address(address)
        .port(port)
        .extra("databases", databases)
        .finalize()
        .unwrap()
}
