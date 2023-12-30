CREATE TABLE IF NOT EXISTS record (
    id           INTEGER PRIMARY KEY,
    mac          BLOB NOT NULL,
    datetime     DATETIME NOT NULL,
    temperature  FLOAT NOT NULL,
    humidity     FLOAT NOT NULL,
    air_pressure FLOAT NOT NULL
);

CREATE UNIQUE INDEX record_mac_datetime ON record(mac, datetime);
