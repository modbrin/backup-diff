use multimap::MultiMap;
use std::collections::HashSet;
use walkdir::WalkDir;
use sha2::{Sha256, Digest};
use std::fs::File;
use std::path::Path;
use std::{io, thread};
use std::error::Error;
use lazy_static::lazy_static;
use std::sync::{Mutex, Arc};
use crate::tracker::ProgressTracker;
use std::sync::mpsc::channel;
use std::ops::DerefMut;
use core::mem;
use std::borrow::Borrow;

mod tracker;

lazy_static! {
    static ref ERRORS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

/// get list of all reported errors
pub fn get_errors() -> Vec<String> {
    ERRORS.lock().unwrap().clone()
}

/// report error
pub fn push_error(message: &str) {
    ERRORS.lock().unwrap().push(message.to_owned());
}

/// get Map of Filehash -> List of Filepaths
pub fn get_directory_map(dir: &str, force_linear: bool) -> MultiMap<String, String> {
    // get list of filepaths in given directories
    let list = recursive_dir_list(dir);

    // connect filepaths and file hashes
    if force_linear {
        println!("Concurrent Processing Disabled");
        hash_file_list(list)
    } else {
        println!("Concurrent Processing Enabled");
        hash_file_list_parallel(list)
    }
}

/// get symmetric difference of keys in given maps split in two parts
pub fn get_diff(map_a: &MultiMap<String, String>, map_b: &MultiMap<String, String>) -> (Vec<String>, Vec<String>) {
    // get filehash sets
    let set_a: HashSet<&str> = map_a.iter().map(|(k, _)| k.as_str()).collect();
    let set_b: HashSet<&str> = map_b.iter().map(|(k, _)| k.as_str()).collect();

    // find difference in filehash sets
    let only_a = set_a.difference(&set_b).cloned().map(|s| s.to_owned()).collect(); // new items
    let only_b = set_b.difference(&set_a).cloned().map(|s| s.to_owned()).collect(); // deleted items

    (only_a, only_b)
}


/// get duplicate entries referring to same key
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
pub fn hash_file_list(filepaths: Vec<String>) -> MultiMap<String, String> {
    let mut store = MultiMap::new();
    println!("Hashing {} file(s), this may take long time.", (&filepaths).len());

    let mut tracker = ProgressTracker::new(0, (&filepaths).len());

    for fp in filepaths.iter() { // TODO: concurrent processing
        tracker.increment();
        tracker.show();
        let hash = get_file_hash(fp);
        if hash.is_err() { continue; }
        let uwr_hash = hash.unwrap();
        store.insert(uwr_hash, fp.to_owned());
    }
    println!("\rDone");
    store
}

pub fn hash_file_list_parallel(filepaths: Vec<String>) -> MultiMap<String, String> {
    println!("Hashing {} file(s), this may take long time.", filepaths.len());

    let mut paths = Arc::new(filepaths);
    let mut store = Arc::new(Mutex::new(MultiMap::new()));
    let mut tracker = Arc::new(Mutex::new(ProgressTracker::new(0, paths.len())));

    let (tx, rx) = channel();
    for fp_i in 0..paths.len() {
        let (store, tracker, paths, tx) = (Arc::clone(&store), Arc::clone(&tracker), Arc::clone(&paths), tx.clone());
        thread::spawn(move || {
            let fp = paths.get(fp_i).unwrap();
            let hash = get_file_hash(fp);
            match hash {
                Ok(hash_string) => {
                    let mut store = store.lock().unwrap().insert(hash_string, fp.clone());
                }
                Err(err) => {
                    push_error(err.description());
                }
            }
            let mut tr = tracker.lock().unwrap();
            tr.increment();
            tr.show();
            if tr.is_done() {
                tx.send(()).unwrap();
            }
        });
    }
    rx.recv().unwrap();
    let mut store_cont = MultiMap::new();
    mem::swap(store.lock().unwrap().deref_mut(), &mut store_cont);
    store_cont
}