use postgres::{Client, Error, NoTls};

struct Nation {
    nationality: String,
    count: i64,
}

pub fn aggregate_data() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;

    for row in client.query(
        "SELECT nationality, COUNT(nationality) AS count FROM artists GROUP BY nationality ORDER BY count DESC",
        &[],
    )? {
        let (nationality, count): (Option<String>, Option<i64>) = (row.get(0), row.get(1));

        if nationality.is_some() && count.is_some() {
            let nation = Nation {
                nationality: nationality.unwrap(),
                count: count.unwrap(),
            };
            println!("{} {}", nation.nationality, nation.count);
        }
    }

    Ok(())
}
