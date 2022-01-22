use postgres::{Client, Error, NoTls};

pub fn create_tables_in_a_postgres_database() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;

    client.batch_execute(
        "\
        CREATE TABLE IF NOT EXISTS author (
            id      SERIAL PRIMARY KEY,
            name    VARCHAR NOT NULL,
            country VARCHAR NOT NULL
        )
    ",
    )?;

    Ok(())
}
