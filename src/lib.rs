mod browser;
use browser::qutebrowser;
use browser::firefox;


#[derive(Debug, Clone)]
pub struct History {
    pub url: String,
    pub title: Option<String>,
}

pub fn get_history() -> Vec<History> {
    let mut history:Vec<History> = Vec::new();

    // qutebrowser
    let mut db = qutebrowser::locate_database().unwrap();
    qutebrowser::copy_database(&mut db);
    let qb_history = qutebrowser::get_history(db);

    qb_history.entries.iter().for_each(|entry| {
        history.push(History { url: entry.url.clone(), title: Some(entry.title.clone()) })
    });

    // firefox
    let dbs = firefox::locate_databases(firefox::Scope::CurrentUser).unwrap();
    for mut db in dbs {
        browser::firefox::copy_database(&mut db);
        browser::firefox::get_history(db).entries.iter().for_each(|entry| {
            history.push(History { url: entry.url.clone(), title: entry.title.clone() })
        });
    }

    history
}
