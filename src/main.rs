use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;

/// Find all files duplicated inside of a path.
#[derive(Parser)]
struct Cli {
    /// The path to the folder to read
    path: PathBuf,
}

#[derive(Debug)]
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

fn find_exact_same_size(files: Vec<FileInfo>) {
    let mut join_by_size: HashMap<u64, Vec<FileInfo>> = HashMap::new();
    for f in files {
        join_by_size.entry(f.len).or_default().push(f);
    }

    println!("The next files are potentially the same:");
    for (len, files) in join_by_size {
        if files.len() > 1 {
            println!("{len}: {files:?}")
        }
    }
}

fn main() {
    let args = Cli::parse();
    let paths = list_files(args.path);

    match paths {
        Ok(paths) => find_exact_same_size(paths),
        Err(err) => println!("Error examining the folder: {err}"),
    }
}
