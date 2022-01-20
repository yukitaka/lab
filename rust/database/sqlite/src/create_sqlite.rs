use rusqlite::{Connection, Result};

pub fn create_a_sqlite_database() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    conn.execute(
        "create table if not exists cat_colors (\
            id integer primary key,
            name text not null unique
        )",
        [],
    )?;

    Ok(())
}
