use config::Config;
use record::get_new_records;
use std::{env, process};

mod config;
mod db;
mod err;
mod record;

fn main() {
    Config::new(env::args())
        .and_then(get_new_records)
        .unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        })
}
