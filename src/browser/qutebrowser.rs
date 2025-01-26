use log::info;
use rusqlite::Connection;
use uuid::Uuid;

pub struct History {
    pub entries: Vec<HistoryEntry>
}

pub struct HistoryEntry {
    pub url: String,
    pub title: String,
    pub access_time: u32,
    pub redirect: bool,
}

pub struct Database {
    location: PathBuf,
    tmp_location: Option<PathBuf>,
}

use std::path::{Path, PathBuf};

// check if QUTE_DATA_DIR is set
// for example: QUTE_DATA_DIR=/home/john/.local/share/qutebrowser
// if not use default path constructed from home directory
// /home/USERNAME/.local/share/qutebrowser/history.sqlite
pub fn locate_database() -> Option<Database> {
    if let Ok(qute_data_dir) = std::env::var("QUTE_DATA_DIR") {
        let basepath = Path::new(&qute_data_dir);

        let path = basepath
            .join("history.sqlite");
        info!("checking path: {}", &path.as_os_str().to_str().unwrap());
        if path.exists() {
            Some(Database { location: path, tmp_location: None })
        } else {
            None
        }
    } else {
        let home = std::env::var("HOME").unwrap();
	let basepath = Path::new(&home).join(".local/share/qutebrowser");
	let path = basepath
	    .join("history.sqlite");
	info!("checking path: {}", &path.as_os_str().to_str().unwrap());
	if path.exists() {
	    Some(Database { location: path, tmp_location: None })
	} else {
	    None
	}
    }
}

pub fn copy_database(database: &mut Database) {
    let id = Uuid::new_v4();
    let filepath = format!("/tmp/browser-history-qutebrowser-{}.db", id);
    let cache_file_path = Path::new(&filepath);
    let file_path = database.location.as_os_str();
    std::fs::copy(file_path, cache_file_path).unwrap();
    database.tmp_location = Some(cache_file_path.to_path_buf());
}

pub fn get_history(database: Database) -> History {
    let db_path = database.tmp_location.unwrap();
    info!("{}", &db_path.as_os_str().to_str().unwrap());
    let conn = Connection::open(db_path).unwrap();

    let mut history_query = conn.prepare("SELECT title, url, atime, redirect FROM History").unwrap();

    let history_rows = history_query.query_map([], |row| {
        Ok(HistoryEntry {
            title: row.get(0).unwrap(),
            url: row.get(1).unwrap(),
            access_time: row.get(2).unwrap(),
            redirect: row.get(3).unwrap(),
        })
    }).unwrap();

    let mut history = Vec::new();

    history_rows.into_iter().for_each(|a| {
        history.push(a.unwrap())
    });

    History { entries: history }
}
