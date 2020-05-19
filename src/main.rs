extern crate chrono;
#[macro_use]
extern crate log;
extern crate log4rs;

use clap::clap_app;
use walkdir::WalkDir;
use std::fs::File;
use std::io;
use sha2::{Sha256, Digest};
use std::path::Path;
use multimap::MultiMap;
use std::collections::HashSet;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Config, Appender, Root};
use log::{LevelFilter, SetLoggerError, Level};
use std::error::Error;
use chrono::Local;

// TODOS
// do erasing in progress
// add timestamps / erase log file
// add duplicates finding / printing
// add diff printing

struct ProgressTracker {
    counter: usize,
    min_val: usize,
    max_val: usize,
}

impl ProgressTracker {
    fn new() -> Self {
        ProgressTracker {
            counter: 0,
            min_val: 0,
            max_val: 100,
        }
    }

    fn init(&mut self, min_val: usize, max_val: usize) {
        self.counter = min_val;
        self.min_val = min_val;
        self.max_val = max_val;
    }

    fn update(&mut self, new_v: usize) {
        if new_v >= self.min_val && new_v <= self.max_val {
            self.counter = new_v;
        }
    }

    // value of [0.0, 1.0]
    fn get_percentage(&self) -> f32 {
        (self.counter - self.min_val) as f32 / (self.max_val - self.min_val) as f32
    }

    fn show(&self) {
        println!("{:2}%", (self.get_percentage() * 100.0).floor()) //TODO: do erasing
    }
}

fn main() {
    let logging_res = setup_logging();
    if logging_res.is_err() {
        println!("Error setting up logger.");
        return;
    }

    info!("New Session");

    let matches = clap_app!("backup-diff" =>
        (version: "0.1")
        (author: "Maksim S. <modbrin@live.com>")
        (about: "Provides hash difference in filesets between two directories")
        (@arg DIR_A: +required "First directory, e.g. newer version")
        (@arg DIR_B: +required "Second directory, e.g. older version")
    ).get_matches();

    // try getting directory paths from cmd arguments
    let map_a = get_directory_map(matches.value_of("DIR_A").unwrap());
    let map_b = get_directory_map(matches.value_of("DIR_B").unwrap());

    get_diff(&map_a, &map_b);
}

fn setup_logging() -> Result<(), Box<dyn Error>> {
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("problems.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    Ok(())
}

fn get_directory_map(dir: &str) -> MultiMap<String, String> {
    // get list of filepaths in given directories
    let list = recursive_dir_list(dir);

    // connect filepaths and file hashes
    hash_file_list(&list, &mut ProgressTracker::new())
}

fn get_diff(map_a: &MultiMap<String, String>, map_b: &MultiMap<String, String>) {
    // get filehash sets
    let set_a: HashSet<&str> = map_a.iter().map(|(k, _)| k.as_str()).collect();
    let set_b: HashSet<&str> = map_b.iter().map(|(k, _)| k.as_str()).collect();

    // find difference in filehash sets
    let only_a: HashSet<&str> = set_a.difference(&set_b).cloned().collect(); // new items
    let only_b: HashSet<&str> = set_b.difference(&set_a).cloned().collect(); // deleted items

    println!("New items: {}\nDeleted items: {}", only_a.len(), only_b.len());
}

fn find_duplicates() {}

fn get_file_hash(path: &str) -> io::Result<String> {
    let mut file = File::open(Path::new(path))?;
    let mut sha256 = Sha256::new();
    io::copy(&mut file, &mut sha256)?;
    Ok(format!("{:x}", sha256.result()))
}

fn recursive_dir_list(dir: &str) -> Vec<String> {
    println!("Listing directory in progress");

    let mut list = Vec::new();

    for entry in WalkDir::new(dir).follow_links(false) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    list.push(entry.path().to_str().unwrap().to_owned());
                } else {
                    //info!("Entry is not a file: {}", entry.path().display())
                }
            }
            Err(error) => {
                info!("Building dir tree: {}", error);
            }
        }
    }
    println!("Done");
    list
}

// moves out strings from input into resulting map
fn hash_file_list(filepaths: &Vec<String>, tracker: &mut ProgressTracker) -> MultiMap<String, String> {
    let mut store = MultiMap::new();

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
    store
}

