use clap::clap_app;
use walkdir::WalkDir;
use std::fs::File;
use std::io;
use sha2::{Sha256, Digest};

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

    build_dir_tree(dir_a);

}

fn build_dir_tree(dir: &str) {
    for entry in WalkDir::new(dir).follow_links(false) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_file() {
                    let mut file = File::open(entry.path());
                    if file.is_err() {continue}
                    let mut sha256 = Sha256::new();
                    let copy_res = io::copy(&mut file.unwrap(), &mut sha256);
                    if copy_res.is_err() {continue}
                    let hash = sha256.result();
                    println!("{} {:x} {}", entry.path().display(), hash, copy_res.unwrap());
                }
            }
            Err(error) => {
                // do nothing
            }
        }
    }
}

