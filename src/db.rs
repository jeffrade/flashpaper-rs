use rusqlite::{params, Connection};

pub struct Secret {
    pub id: String,
    pub iv: String,
    pub hash: String,
    pub secret: String,
}

pub fn init() {
    println!("Initializing SQLite3...");
    let conn: Connection = get_connection().unwrap();

    match conn.execute(
        "CREATE TABLE IF NOT EXISTS secrets (
            id TEXT PRIMARY KEY,
            iv TEXT,
            hash TEXT,
            secret TEXT
        )",
        params![],
    ) {
        Ok(_) => (),
        Err(_) => {
            println!("Ignore since table might already exist.");
        }
    };
}

pub fn store(id: &str, iv: &str, hash: &str, secret: &str) -> Result<usize, rusqlite::Error> {
    let conn: Connection = get_connection().unwrap();
    conn.execute(
        "INSERT INTO secrets (id, iv, hash, secret) VALUES (?1, ?2, ?3, ?4)",
        params![id, iv, hash, secret],
    )
}

pub fn get(id: &str) -> Result<Secret, rusqlite::Error> {
    let conn: Connection = get_connection().unwrap();
    let mut stmt = conn.prepare("SELECT id, iv, hash, secret FROM secrets WHERE id = ?")?;
    let mut rows = stmt.query([&id])?;

    let secret = match rows.next()? {
        Some(row) => Secret {
            id: row.get(0)?,
            iv: row.get(1)?,
            hash: row.get(2)?,
            secret: row.get(3)?,
        },
        _ => Secret {
            id: "".to_string(),
            iv: "".to_string(),
            hash: "".to_string(),
            secret: "".to_string(),
        },
    };

    Ok(secret)
}

pub fn delete(id: &str) -> Result<usize, rusqlite::Error> {
    let conn: Connection = get_connection().unwrap();
    conn.execute("DELETE FROM secrets WHERE id = ?", params![id])
}

fn get_connection() -> rusqlite::Result<Connection> {
    let path: String = "./flashpaper-rs.db3".to_string();
    let conn = Connection::open(&path)?;
    println!("is autocommit? {}", conn.is_autocommit());
    Ok(conn)
}
