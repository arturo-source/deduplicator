use clap::Parser;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Find all files duplicated inside of a path.
#[derive(Parser)]
struct Cli {
    /// The path to the folder to read
    path: PathBuf,
}

struct FileInfo {
    path: PathBuf,
    len: u64,
}

impl FileInfo {
    fn try_new(path: PathBuf) -> io::Result<FileInfo> {
        let len = path.metadata()?.len();
        Ok(FileInfo { path, len })
    }
}

fn list_files(path: PathBuf) -> io::Result<Vec<FileInfo>> {
    if path.is_file() {
        let fi = FileInfo::try_new(path)?;
        return Ok(vec![fi]);
    }

    let mut files = Vec::new();
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry_path = entry?.path();
        let nested_files = list_files(entry_path)?;
        files.extend(nested_files);
    }

    Ok(files)
}

fn print_all(files: Vec<FileInfo>) {
    for f in files {
        println!("{} {}", f.len, f.path.display())
    }
}

fn main() {
    let args = Cli::parse();
    let paths = list_files(args.path);

    match paths {
        Ok(paths) => print_all(paths),
        Err(err) => println!("Error examining the folder: {err}"),
    }
}
