use rusqlite::{Connection, Result as SqlResult};
use crate::migrations::ALL_MIGRATIONS;

pub struct MigrationRunner;

impl MigrationRunner {
    pub fn run(conn: &Connection) -> SqlResult<Vec<u32>> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS _migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL
            );"
        )?;

        let applied: Vec<u32> = {
            let mut stmt = conn.prepare("SELECT version FROM _migrations ORDER BY version")?;
            let rows = stmt.query_map([], |row| row.get::<_, u32>(0))?;
            rows.filter_map(|r| r.ok()).collect()
        };

        let mut ran: Vec<u32> = Vec::new();

        for migration in ALL_MIGRATIONS {
            if applied.contains(&migration.version) {
                continue;
            }

            conn.execute_batch(migration.sql)?;
            conn.execute(
                "INSERT INTO _migrations (version, name, applied_at) VALUES (?1, ?2, ?3)",
                rusqlite::params![
                    migration.version,
                    migration.name,
                    chrono::Utc::now().to_rfc3339()
                ],
            )?;

            ran.push(migration.version);
        }

        Ok(ran)
    }

    pub fn pending(conn: &Connection) -> SqlResult<Vec<u32>> {
        let applied: Vec<u32> = {
            let mut stmt = conn.prepare("SELECT version FROM _migrations ORDER BY version")?;
            let rows = stmt.query_map([], |row| row.get::<_, u32>(0))?;
            rows.filter_map(|r| r.ok()).collect()
        };

        Ok(ALL_MIGRATIONS
            .iter()
            .filter(|m| !applied.contains(&m.version))
            .map(|m| m.version)
            .collect())
    }

    pub fn applied_versions(conn: &Connection) -> SqlResult<Vec<(u32, String)>> {
        let mut stmt = conn.prepare("SELECT version, name FROM _migrations ORDER BY version")?;
        let rows = stmt.query_map([], |row| Ok((row.get::<_, u32>(0)?, row.get::<_, String>(1)?)))?;
        rows.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::connection;

    #[test]
    fn test_migrations_run_cleanly() {
        let conn = connection::open_in_memory().unwrap();
        let ran = MigrationRunner::run(&conn).unwrap();
        assert!(!ran.is_empty(), "expected at least one migration to run");
        assert!(ran.contains(&1), "expected migration v1");
    }

    #[test]
    fn test_migrations_idempotent() {
        let conn = connection::open_in_memory().unwrap();
        MigrationRunner::run(&conn).unwrap();
        let ran_again = MigrationRunner::run(&conn).unwrap();
        assert!(ran_again.is_empty(), "second run should not apply any migrations");
    }

    #[test]
    fn test_applied_versions() {
        let conn = connection::open_in_memory().unwrap();
        MigrationRunner::run(&conn).unwrap();
        let applied = MigrationRunner::applied_versions(&conn).unwrap();
        assert_eq!(applied.len(), 1);
        assert_eq!(applied[0].0, 1);
    }

    #[test]
    fn test_pending_empty_after_run() {
        let conn = connection::open_in_memory().unwrap();
        MigrationRunner::run(&conn).unwrap();
        let pending = MigrationRunner::pending(&conn).unwrap();
        assert!(pending.is_empty());
    }
}
