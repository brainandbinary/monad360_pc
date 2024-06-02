use std::{env::current_dir, sync::Mutex};
use lazy_static::lazy_static;
use std::{fmt, io};

use rusqlite::Connection;

#[derive(Debug)]
pub enum DBError {
    PathError(String),
    ConnectionError(rusqlite::Error),
    IOError(io::Error),
}

impl fmt::Display for DBError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DBError::PathError(msg) => write!(f, "Path error: {}", msg),
            DBError::ConnectionError(err) => write!(f, "Connection error: {}", err),
            DBError::IOError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl From<rusqlite::Error> for DBError {
    fn from(err: rusqlite::Error) -> Self {
        DBError::ConnectionError(err)
    }
}

impl From<io::Error> for DBError {
    fn from(err: io::Error) -> Self {
        DBError::IOError(err)
    }
}


pub fn get_database_path(db_name: &str) -> Result<std::path::PathBuf, std::io::Error> {
    let current_dir: std::path::PathBuf = current_dir()?;
    let database_path: std::path::PathBuf = current_dir.join(db_name);
    Ok(database_path)
}



pub struct Database {
    pub connection: Connection,
}

impl Database {
    pub fn new(db_path: &str) -> Result<Self, DBError> {
        let path = get_database_path(db_path)?;
        let connection = Connection::open(path)?;
        Ok(Database { connection })
    }
}



lazy_static! {
    pub static ref DB: Mutex<Result<Database,DBError>> = Mutex::new(Database::new("/home/johnny/Desktop/db_1.db3"));
   
}


