use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use rusqlite::Connection;

pub type Database = Arc<Mutex<Connection>>;

pub fn create_db(path: String) -> Result<Database, Box<dyn Error>> {
    let conn = Connection::open(path);
    match conn {
        Ok(c) => Ok(Arc::new(Mutex::new(c))),
        Err(_) => Err("Failed loading database")?,
    }
}

pub fn convert_type(_type: String) -> Option<String> {
    Some("a".to_string())
}
