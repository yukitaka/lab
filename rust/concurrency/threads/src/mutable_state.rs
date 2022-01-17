use error_chain::error_chain;
use lazy_static::lazy_static;
use std::sync::Mutex;

error_chain! {}

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn maintain_global_mutable_state() -> Result<()> {
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
        db.iter()
            .enumerate()
            .for_each(|(i, item)| println!("{}: {}", i, item));
    }

    Ok(())
}

fn insert(fruit: &str) -> Result<()> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
}
