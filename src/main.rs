use crate::print_utils::{print_duplicates, print_errors, print_select_values};
use backup_diff::{find_duplicates, get_diff, get_directory_map, get_errors};
use clap::clap_app;
use std::fs;
use std::fs::File;
use std::io::Write;

mod print_utils;

fn main() {
    let matches = clap_app!("backup-diff" =>
        (version: "0.1")
        (author: "Maksim S. <modbrin@live.com>")
        (about: "Provides hash difference in filesets between two directories.")
        (@arg DIR_A: +required "First directory, e.g. newer version")
        (@arg DIR_B: +required "Second directory, e.g. older version")
        (@arg LINEAR: -l --linear "Disable concurrent processing")
    )
    .get_matches();

    // try getting directory paths from cmd arguments
    let enable_linear = matches.is_present("LINEAR");
    let map_a = get_directory_map(matches.value_of("DIR_A").unwrap(), enable_linear);
    let map_b = get_directory_map(matches.value_of("DIR_B").unwrap(), enable_linear);

    let (only_a, only_b) = get_diff(&map_a, &map_b);
    println!("\nNew files ({} items):", only_a.len());
    print_select_values(&map_a, &only_a);

    println!("\nRemoved files ({} items):", only_b.len());
    print_select_values(&map_b, &only_b);

    let (dup_a, dup_b) = (find_duplicates(&map_a), find_duplicates(&map_b));
    println!("\nDuplicates in `new` folder:");
    print_duplicates(&dup_a);

    println!("\nDuplicates in `old` folder:");
    print_duplicates(&dup_b);

    let errors = get_errors();

    println!("\nErrors:");
    print_errors(&errors);
}
