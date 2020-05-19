#[macro_use]
extern crate log;
use log4rs;
use clap::clap_app;
use std::io;
use multimap::MultiMap;
use std::collections::HashSet;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Config, Appender, Root};
use log::{LevelFilter, SetLoggerError, Level};
use std::error::Error;
use chrono::Local;

use backup_diff::{get_directory_map, print_select_values};
use backup_diff::get_diff;
use backup_diff::find_duplicates;

// TODOS
// do erasing in progress
// add timestamps / erase log file
// add duplicates finding / printing
// add diff printing

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
        (about: "Provides hash difference in filesets between two directories.\nErrors are saved to problems.log file.")
        (@arg DIR_A: +required "First directory, e.g. newer version")
        (@arg DIR_B: +required "Second directory, e.g. older version")
    ).get_matches();

    // try getting directory paths from cmd arguments
    let map_a = get_directory_map(matches.value_of("DIR_A").unwrap());
    let map_b = get_directory_map(matches.value_of("DIR_B").unwrap());

    let (only_a, only_b) = get_diff(&map_a, &map_b);
    println!("New files ({} items):", only_a.len());
    print_select_values(&map_a, &only_a);

    println!("\nRemoved files ({} items):", only_b.len());
    print_select_values(&map_b, &only_b);

    let dup_a = find_duplicates(&map_a);
    let dup_b = find_duplicates(&map_b);
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

