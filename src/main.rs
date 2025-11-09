use clap::Parser;
use deduplicator::{get_duplicated_files, get_shared_parents, list_files};
use std::path::PathBuf;
use std::{io, process};

/// Find all files duplicated inside of a path.
#[derive(Parser)]
struct Cli {
    /// The path to the folder to read
    path: PathBuf,
}

fn main() {
    let args = Cli::parse();

    if let Err(e) = run(args.path) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

fn run(path: PathBuf) -> io::Result<()> {
    let paths = list_files(path)?;
    let duplicated_files = get_duplicated_files(paths)?;
    let shared_parents = get_shared_parents(duplicated_files);

    let mut shared_parents_vec: Vec<_> = shared_parents.into_iter().collect();
    shared_parents_vec.sort_by(|a, b| b.1.0.len().cmp(&a.1.0.len()));

    for ((parent1, parent2), (files1, files2)) in shared_parents_vec {
        println!("In {parent1:?}:");
        for f in files1 {
            println!("  {:?}", f.file_name().unwrap())
        }

        println!("In {parent2:?}:");
        for f in files2 {
            println!("  {:?}", f.file_name().unwrap())
        }

        println!()
    }

    Ok(())
}
