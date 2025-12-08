use crate::err::Res;
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{ConnectOptions, SqliteConnection, migrate};

pub async fn get_connection(db_path: &str) -> Res<SqliteConnection> {
    let opt = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);
    let mut con = opt.connect().await?;

    migrate!().run(&mut con).await?;
    Ok(con)
}
