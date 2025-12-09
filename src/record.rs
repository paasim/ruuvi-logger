use crate::{config::Config, db::get_connection, err::Res};
use chrono::{DateTime, NaiveDateTime, TimeDelta, Utc};
use macaddr::MacAddr6;
use ruuvi::{Record, get_log};
use sqlx::{SqliteConnection, query};

#[tokio::main(flavor = "current_thread")]
pub async fn get_new_records(config: Config) -> Res<()> {
    let mut con = get_connection(&config.db_path).await?;
    for m in config.mac_addresses {
        if let Err(e) = get_new_records_for_mac(&mut con, m, config.min_delta).await {
            eprintln!("{}", e);
        }
    }
    Ok(())
}

async fn get_new_records_for_mac(
    con: &mut SqliteConnection,
    mac: MacAddr6,
    min_delta: TimeDelta,
) -> Res<()> {
    println!("obtaining records for {}", mac);
    let start = match log_end(con, &mac)
        .await
        .map(|o| valid_start(o, min_delta))?
    {
        Some(s) => s,
        None => return Ok(()),
    };
    for r in get_log(mac, start).await? {
        insert_rec(con, &mac, r).await?;
    }
    Ok(())
}

fn valid_start(opt_start: Option<DateTime<Utc>>, min_delta: TimeDelta) -> Option<DateTime<Utc>> {
    let now = Utc::now();
    match opt_start {
        Some(s) if now - s < min_delta => {
            println!(" skipping because latest record was at {}", s);
            None
        }
        Some(s) if now - s > TimeDelta::hours(240) => {
            println!(" for a maximum duration at {}", s);
            Some(now - TimeDelta::hours(240))
        }
        Some(s) => {
            println!(" after latest record at {}", s);
            Some(s)
        }
        None => {
            let s = now - TimeDelta::hours(240);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_start_missing_opt_start_works() {
        let start = valid_start(None, TimeDelta::minutes(123)).unwrap();
        let n = Utc::now();
        assert!((n - start).num_seconds() <= 240 * 60 * 60);
        assert!((n - start).num_seconds() >= 240 * 60 * 60 - 1);

        // does not depend on min_delta
        let start1 = valid_start(None, TimeDelta::minutes(456)).unwrap();

        // within 1 second
        assert!(start.timestamp() <= start1.timestamp());
        assert!(start.timestamp() + 1 >= start1.timestamp());
    }

    #[test]
    fn valid_start_returns_none_when_out_of_bounds() {
        let n = Utc::now();
        let min_delta = TimeDelta::minutes(5);

        // valid duration
        let start1 = Some(n - TimeDelta::minutes(6));
        assert_eq!(valid_start(start1, min_delta), start1);
        let start2 = Some(n - TimeDelta::hours(239) - TimeDelta::minutes(59));
        assert_eq!(valid_start(start2, min_delta), start2);

        // too short => none
        assert!(valid_start(Some(n - TimeDelta::minutes(4)), min_delta).is_none());

        // to long => clip to 240 hours
        let start_clipped = valid_start(Some(n - TimeDelta::hours(241)), min_delta).unwrap();
        assert!((n - start_clipped).num_seconds() <= 240 * 60 * 60);
        assert!((n - start_clipped).num_seconds() >= 240 * 60 * 60 - 1);
    }
}
