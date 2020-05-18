use clap::clap_app;
use walkdir::WalkDir;
use std::fs::File;
use std::io;
use sha2::{Sha256, Digest};
use std::path::Path;
use std::io::Error;
use multimap::MultiMap;
use std::collections::HashSet;

fn main() {
    let matches = clap_app!("backup-diff" =>
        (version: "0.1")
        (author: "Maksim S. <modbrin@live.com>")
        (about: "Provides hash difference in filesets between two directories")
        (@arg DIR_A: +required "First directory, e.g. newer version")
        (@arg DIR_B: +required "Second directory, e.g. older version")
    ).get_matches();

    let dir_a = matches.value_of("DIR_A").unwrap();
    let dir_b = matches.value_of("DIR_B").unwrap();

    let tree_a = build_dir_tree(dir_a);
    let tree_b = build_dir_tree(dir_b);
    let set_a: HashSet<&str> = tree_a.iter().map(|(k, v)|k.as_str()).collect();
    let set_b: HashSet<&str> = tree_b.iter().map(|(k, v)|k.as_str()).collect();
    let only_a: HashSet<&str> = set_a.difference(&set_b).cloned().collect(); // new items
    let only_b: HashSet<&str> = set_b.difference(&set_a).cloned().collect(); // deleted items

    println!("New items: {}\nDeleted items: {}", only_a.len(), only_b.len());
}

fn get_file_hash(path: &Path) -> Result<String, Error> {
    let mut file = File::open(path)?;
    let mut sha256 = Sha256::new();
    let copy_res = io::copy(&mut file, &mut sha256)?;
    Ok(format!("{:x}", sha256.result()))
}

fn build_dir_tree(dir: &str) -> MultiMap<String, String> {
    let mut store = MultiMap::new();
    for entry in WalkDir::new(dir).follow_links(false) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let hash = get_file_hash(entry.path());
                    if hash.is_err() {continue}
                    let uwr_hash = hash.unwrap();
                    store.insert(uwr_hash.clone(), entry.path().to_str().unwrap().to_owned());
                    println!("{} {}", entry.path().display(), uwr_hash);
                }
            }
            Err(error) => {
                // do nothing
            }
        }
    }
    store
}

