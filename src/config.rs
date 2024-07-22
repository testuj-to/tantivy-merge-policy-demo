use std::env;

use lazy_static::lazy_static;

fn get_env(name: &str) -> String {
    match env::var(name) {
        Ok(value) => value,
        Err(_) => {
            panic!("Missing env variable {}", name);
        },
    }
}

lazy_static! {
    pub static ref DATA_PEOPLE_PATH: String = get_env("DATA_PEOPLE_PATH");
    pub static ref INDEX_PEOPLE_PATH: String = get_env("INDEX_PEOPLE_PATH");
}
