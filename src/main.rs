use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

/// Find all files duplicated inside of a path.
#[derive(Parser)]
struct Cli {
    /// The path to the folder to read
    path: PathBuf,
}

fn list_files(path: &Path) -> std::io::Result<Vec<PathBuf>> {
    if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }

    let mut files = Vec::new();
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry_path = entry?.path();
        let nested_files = list_files(&entry_path)?;
        files.extend(nested_files);
    }

    Ok(files)
}

fn print_all(paths: Vec<PathBuf>) {
    for path in paths {
        let len = path.metadata().unwrap().len();
        println!("{} {}", len, path.display())
    }
}

fn main() {
    let args = Cli::parse();
    let paths = list_files(&args.path);

    match paths {
        Ok(paths) => print_all(paths),
        Err(err) => println!("Error examining the folder: {err}"),
    }
}
