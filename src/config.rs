use crate::err::{Error, Res};
use chrono::Duration;
use macaddr::MacAddr6;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub db_path: String,
    pub mac_addresses: Vec<MacAddr6>,
    pub min_duration: Duration,
}

impl Config {
    pub fn new(mut args: env::Args) -> Res<Config> {
        let progname = args.next().ok_or("arguments missing")?;
        let db_path = match args.next().as_deref() {
            Some("--db") => args.next().ok_or::<Error>(get_usage(&progname).into()),
            _ => Err(get_usage(&progname).into()),
        }?;
        let mac_addresses = args.map(|s| Ok(s.parse()?)).collect::<Res<_>>()?;
        let min_duration = match env::var("MIN_MINUTES") {
            Ok(s) => Duration::minutes(s.parse().map_err(Error::from)?),
            _ => Duration::minutes(5),
        };
        Ok(Self {
            db_path,
            mac_addresses,
            min_duration,
        })
    }
}

fn get_usage(program_name: &str) -> String {
    format!(
        "usage: {} --db /path/to/the/database.db mac1 mac2...",
        program_name
    )
}
