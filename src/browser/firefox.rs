use std::path::{Path, PathBuf};

use rusqlite::Connection;
use uuid::Uuid;

pub struct History {
    pub entries: Vec<HistoryEntry>
}

use chrono::{DateTime, NaiveDateTime, Utc};

// places.sqlite::moz_places
pub struct HistoryEntry {
    pub url: String,
    pub title: Option<String>,
    pub last_visit_date: Option<DateTime<Utc>>,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct Database {
    location: PathBuf,
    tmp_location: Option<PathBuf>,
}

pub enum Scope {
    CurrentUser,
    User(String),
    System
}

fn parse_profile_ini(path: PathBuf) -> Vec<String> {
    let mut profiles = Vec::new();
    use ini::Ini;
    let ini = Ini::load_from_file(path).unwrap();
    for (section, props) in ini.iter() {
        if let Some(section) = section {
            if section.starts_with("Install") {
                let default = props.get::<String>("Default".into()).unwrap();
                profiles.push(default.to_string())
            }
        }
    }
    profiles
}

pub fn locate_databases(scope: Scope) -> Option<Vec<Database>> {
    match scope {
        Scope::CurrentUser => {
            if let Ok(home_dir) = std::env::var("HOME") {

                let mut databases = Vec::new();

                let user_profile_ini_path = Path::new(&home_dir)
                    .join(".mozilla")
                    .join("firefox")
                    .join("profiles.ini");

                if user_profile_ini_path.exists() {
                    let profiles = parse_profile_ini(user_profile_ini_path);

                    profiles.iter().for_each(|p| {
                        let db_path = Path::new(&home_dir)
                            .join(".mozilla")
                            .join("firefox")
                            .join(p)
                            .join("places.sqlite");
                        databases.push(Database {
                            location: db_path,
                            tmp_location: None
                        });
                    });

                    Some(databases)

                } else {
                    None
                }
            } else {
                None
            }

        },
        Scope::User(_) => todo!(),
        Scope::System => todo!(),
    }
}

pub fn copy_database(database: &mut Database) {
    let file_path = database.location.as_os_str();
    let id = Uuid::new_v4();
    let filepath = format!("/tmp/browser-history-firefox-{}.db", id);
    let cache_file_path = Path::new(&filepath);
    std::fs::copy(file_path, cache_file_path).unwrap();
    database.tmp_location = Some(cache_file_path.to_path_buf());
}

pub fn get_history(database: Database) -> History {
    let db_path = database.tmp_location.unwrap();
    let conn = Connection::open(db_path).unwrap();
    let mut history_query = conn.prepare("SELECT url, title, last_visit_date, description FROM moz_places").unwrap();

    let history_rows = history_query.query_map([], |row| {
        Ok(HistoryEntry {
            url: row.get(0).unwrap(),
            title: row.get(1).unwrap(),

            last_visit_date: {
                let last_visit_date: Option<i64> = row.get(2).unwrap();
                if last_visit_date.is_some() {
                    let date_timestamp = last_visit_date.unwrap() / 1000; // convert from microseconds to milliseconds
                    let naive_date_time = NaiveDateTime::from_timestamp_millis(date_timestamp).unwrap();
                    let time = naive_date_time.format("%d/%m/%Y %H:%M:%S");
                    let date_time = DateTime::<Utc>::from_utc(naive_date_time, Utc);
                    Some(date_time)
                } else {
                    None
                }
            },
            description: row.get(3).unwrap(),
        })
    }).unwrap();

    let mut history = Vec::new();

    history_rows.into_iter().for_each(|h| {
        history.push(h.unwrap())
    });

    History { entries: history }
}
