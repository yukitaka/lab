use error_chain::error_chain;
use glob::glob;

error_chain! {
    foreign_links {
        Glob(glob::GlobError);
        Pattern(glob::PatternError);
    }
}

pub fn find_all_png_files_recursively() -> Result<()> {
    for entry in glob("**/*.png")? {
        println!("{}", entry?.display());
    }

    Ok(())
}
