use error_chain::error_chain;
use walkdir::WalkDir;

error_chain! {
    foreign_links {
        WalkDir(walkdir::Error);
        Io(std::io::Error);
        SytemTime(std::time::SystemTimeError);
    }
}

pub fn recursively_find_all_files_with_given_predicate() -> Result<()> {
    println!("Recursively find all files");
    for entry in WalkDir::new(".")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        let sec = entry.metadata()?.modified()?;

        if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 86400 {
            println!("{}", f_name);
        }
    }

    Ok(())
}
