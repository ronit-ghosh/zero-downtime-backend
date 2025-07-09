use crate::config::Config;
use diesel::{Connection, ConnectionError, PgConnection};

pub struct Store {
    pub conn: PgConnection,
}

impl Store {
    pub fn default() -> Result<Self, ConnectionError> {
        let config = Config::default();
        let conn = PgConnection::establish(&config.database_url)?;

        Ok(Self { conn })
    }
}
