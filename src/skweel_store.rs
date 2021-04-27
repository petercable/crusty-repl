use crate::enums::HandleResult;
use crate::store::Store;

use rusqlite::{params, Connection, Result, Error};

pub struct SkweelStore {
    conn: Connection,
    depth: i32,
}

impl SkweelStore {
    pub fn new() -> Result<SkweelStore> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE store ( key TEXT PRIMARY KEY, value TEXT NOT NULL)",
            [],
        )?;
        Ok(SkweelStore { conn, depth: 0 })
    }

    fn tx_name(&self) -> String {
        format!("tx_{}", self.depth)
    }
}

impl Store for SkweelStore {
    fn read(&self, key: String) -> HandleResult {
        let mut stmt = self.conn.prepare(
            "SELECT value FROM store WHERE key = ?1").unwrap();

        match stmt.query_row(params![key], |row| row.get(0)) {
            Ok(s) => HandleResult::Result(s),
            Err(_) => HandleResult::Failure(format!("Key not found: {}", key))
        }
    }

    fn write(&mut self, key: String, value: String) -> HandleResult {
        match self.conn.execute(
            "INSERT INTO store (key, value) VALUES (?1, ?2)",
            params![key, value],
        ) {
            Ok(_) => HandleResult::Success,
            Err(e) => HandleResult::Failure(e.to_string())
        }
    }

    fn delete(&mut self, key: String) -> HandleResult {
        match self.conn.execute(
            "DELETE FROM store WHERE key = ?1",
            params![key],
        ) {
            Ok(_) => HandleResult::Success,
            Err(e) => HandleResult::Failure(e.to_string())
        }
    }

    fn start(&mut self) -> HandleResult {
        self.depth += 1;
        self.conn.execute(format!("SAVEPOINT {}", self.tx_name()).as_str(), []).unwrap();
        HandleResult::Success
    }

    fn abort(&mut self) -> HandleResult {
        if self.depth == 0 {
            return HandleResult::Failure("no transaction to abort".to_string())
        }
        self.conn.execute(format!("ROLLBACK TO {}", self.tx_name()).as_str(), []).unwrap();
        self.depth -= 1;
        HandleResult::Success
    }

    fn commit(&mut self) -> HandleResult {
        if self.depth == 0 {
            return HandleResult::Failure("no transaction to commit".to_string())
        }
        self.conn.execute(format!("RELEASE {}", self.tx_name()).as_str(), []).unwrap();
        self.depth -= 1;
        HandleResult::Success
    }
}