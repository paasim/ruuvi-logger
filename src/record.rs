use crate::{config::Config, db::get_connection, err::Res};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use macaddr::MacAddr6;
use ruuvi::{Record, get_log};
use sqlx::{SqliteConnection, query};

#[tokio::main(flavor = "current_thread")]
pub async fn get_new_records(config: Config) -> Res<()> {
    let mut con = get_connection(&config.db_path).await?;
    for m in config.mac_addresses {
        if let Err(e) = get_new_records_for_mac(&mut con, m).await {
            eprintln!("{}", e);
        }
    }
    Ok(())
}

async fn get_new_records_for_mac(con: &mut SqliteConnection, mac: MacAddr6) -> Res<()> {
    println!("obtaining records for {}", mac);
    let start = match log_end(con, &mac).await.map(valid_start)? {
        Some(s) => s,
        None => return Ok(()),
    };
    for r in get_log(mac, start).await? {
        insert_rec(con, &mac, r).await?;
    }
    Ok(())
}

fn valid_start(opt_start: Option<DateTime<Utc>>) -> Option<DateTime<Utc>> {
    let now = Utc::now();
    match opt_start {
        Some(s) if s > now - Duration::minutes(5) => {
            println!(" skipping because latest record was at {}", s);
            None
        }
        Some(s) => {
            println!(" after latest record at {}", s);
            Some(s)
        }
        None => {
            let s = now - Duration::hours(240);
            println!(" after {} since there are no previous records", s);
            Some(s)
        }
    }
}

async fn insert_rec(con: &mut SqliteConnection, mac: &MacAddr6, r: Record) -> Res<u32> {
    let mac = mac.as_bytes();
    let id_row = query!(
        r#"
        INSERT INTO record (mac, datetime, temperature, humidity, air_pressure)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id AS "id: u32"
        "#,
        mac,
        r.datetime,
        r.temperature,
        r.humidity,
        r.air_pressure
    )
    .fetch_one(con)
    .await?;
    Ok(id_row.id)
}

async fn log_end(con: &mut SqliteConnection, mac: &MacAddr6) -> Res<Option<DateTime<Utc>>> {
    let mac = mac.as_bytes();
    let max_row = query!(
        r#"SELECT MAX(datetime) AS "max_datetime: NaiveDateTime" FROM record WHERE mac = ?"#,
        mac
    )
    .fetch_optional(con)
    .await?;
    Ok(max_row.and_then(|r| r.max_datetime).map(|dt| dt.and_utc()))
}
