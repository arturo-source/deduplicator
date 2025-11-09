use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

/// Find all files duplicated inside of a path.
#[derive(Parser)]
struct Cli {
    /// The path to the folder to read
    path: PathBuf,
}

fn list_files(path: PathBuf) -> io::Result<Vec<PathBuf>> {
    if path.is_file() {
        return Ok(vec![path]);
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

fn get_duplicated_files_by_byte(paths: Vec<PathBuf>) -> Vec<Vec<PathBuf>> {
    let mut buf = [0; 1024];
    let mut files = Vec::new();
    let mut duplicated_files = Vec::new();
    let mut is_duplicated = vec![false; paths.len()];

    for path in &paths {
        let mut file = File::open(path).unwrap();
        file.read(&mut buf).unwrap();
        files.push(buf.clone());
    }

    for (i, f1) in files.iter().enumerate() {
        if is_duplicated[i] {
            continue;
        }

        let mut equal_files = vec![paths[i].clone()];
        for (j, f2) in files.iter().enumerate().skip(i + 1) {
            if f1 == f2 {
                is_duplicated[j] = true;
                equal_files.push(paths[j].clone());
            }
        }

        if equal_files.len() > 1 {
            is_duplicated[i] = true;
            duplicated_files.push(equal_files);
        }
    }

    duplicated_files
}

fn get_duplicated_files(paths: Vec<PathBuf>) -> io::Result<Vec<Vec<PathBuf>>> {
    let mut map_by_len: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    for path in paths {
        let len = path.metadata()?.len();
        map_by_len.entry(len).or_default().push(path);
    }

    let mut duplicated_files: Vec<Vec<PathBuf>> = Vec::new();
    for (_, paths) in map_by_len {
        if paths.len() <= 1 {
            continue;
        }

        duplicated_files.extend(get_duplicated_files_by_byte(paths));
    }

    Ok(duplicated_files)
}

fn main() {
    let args = Cli::parse();
    let paths = list_files(args.path);

    let duplicated_files = match paths {
        Ok(paths) => get_duplicated_files(paths),
        Err(err) => panic!("Error examining the folder: {err}"),
    };

    match duplicated_files {
        Ok(duplicated_files) => {
            for same_files in duplicated_files {
                println!("These are the same files: {same_files:?}")
            }
        }
        Err(err) => panic!("Error examining the files: {err}"),
    }
}
