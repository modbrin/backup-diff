use multimap::MultiMap;
use std::collections::HashSet;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::path::Path;
use std::io;
use std::error::Error;
use lazy_static::lazy_static;
use std::sync::Mutex;

mod tracker;

lazy_static! {
    static ref ERRORS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn get_errors() -> Vec<String> {
    ERRORS.lock().unwrap().clone()
}

pub fn push_error(message: &str) {
    ERRORS.lock().unwrap().push(message.to_owned());
}

pub fn get_directory_map(dir: &str) -> MultiMap<String, String> {
    // get list of filepaths in given directories
    let list = recursive_dir_list(dir);

    // connect filepaths and file hashes
    hash_file_list(&list, &mut tracker::ProgressTracker::new())
}

pub fn get_diff(map_a: &MultiMap<String, String>, map_b: &MultiMap<String, String>) -> (Vec<String>, Vec<String>) {
    // get filehash sets
    let set_a: HashSet<&str> = map_a.iter().map(|(k, _)| k.as_str()).collect();
    let set_b: HashSet<&str> = map_b.iter().map(|(k, _)| k.as_str()).collect();

    // find difference in filehash sets
    let only_a = set_a.difference(&set_b).cloned().map(|s|s.to_owned()).collect(); // new items
    let only_b = set_b.difference(&set_a).cloned().map(|s|s.to_owned()).collect(); // deleted items

    (only_a, only_b)
}

pub fn print_select_values(map: &MultiMap<String, String>, keys: &Vec<String>) {
    for key in keys.iter() {
        println!("  {}", map.get(key).unwrap());
    }
    if keys.is_empty() {
        println!("  <Empty>")
    }
}

pub fn find_duplicates(map: &MultiMap<String, String>) -> Vec<Vec<String>> {
    let mut result = Vec::new();
    for key in map.keys() {
        let vals = map.get_vec(key).unwrap();
        if vals.len() > 1 {
            result.push(vals.clone());
        }
    }
    result
}

pub fn print_duplicates(duplicates: &Vec<Vec<String>>) {
    for dup in duplicates.iter() {
        for fp in dup.iter().enumerate() {
            match fp {
                (0, str) => {
                    println!("  [{}]", str)
                }
                (_, str) => {
                    println!("    {} (duplicate)", str)
                }
            }
        }
    }
    if duplicates.is_empty() {
        println!("  <Empty>");
    }
}

pub fn print_errors(errors: &Vec<String>) {
    for err in errors {
        println!("ERR: {}", err);
    }
    if errors.is_empty() {
        println!("  <Empty>");
    }
}

fn get_file_hash(path: &str) -> io::Result<String> {
    let mut file = File::open(Path::new(path))?;
    let mut sha256 = Sha256::new();
    io::copy(&mut file, &mut sha256)?;
    Ok(format!("{:x}", sha256.result()))
}

fn recursive_dir_list(dir: &str) -> Vec<String> {
    print!("Listing directory [{}] in progress...", dir);

    let mut list = Vec::new();

    for entry in WalkDir::new(dir).follow_links(false) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    list.push(entry.path().to_str().unwrap().to_owned());
                } else {
                    //info!("Entry is not a file: {}", entry.path().display())
                    // folders and symlinks are ignored, as they do not hold data
                }
            }
            Err(error) => {
                push_error(error.description());
            }
        }
    }
    println!("Done");
    list
}

// moves out strings from input into resulting map
fn hash_file_list(filepaths: &Vec<String>, tracker: &mut tracker::ProgressTracker) -> MultiMap<String, String> {
    let mut store = MultiMap::new();
    println!("Hashing {} file(s), this may take long time.", filepaths.len());

    let mut counter = 0usize;
    tracker.init(0, filepaths.len());

    for fp in filepaths.iter() {
        let hash = get_file_hash(fp);
        if hash.is_err() { continue; }
        let uwr_hash = hash.unwrap();
        store.insert(uwr_hash, fp.to_owned());
        tracker.update(counter);
        counter += 1;
        tracker.show();
    }
    println!("\rDone");
    store
}

