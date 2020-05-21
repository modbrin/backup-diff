
use clap::clap_app;
use backup_diff::{get_diff, find_duplicates, get_directory_map, get_errors};
use crate::print_utils::{print_select_values, print_duplicates, print_errors};
mod print_utils;

// TODOS
// optimize for memory usage
// use rayon for concurrent processing / handle concurrency with shared state
// filter out warnings

fn main() {

    let matches = clap_app!("backup-diff" =>
        (version: "0.1")
        (author: "Maksim S. <modbrin@live.com>")
        (about: "Provides hash difference in filesets between two directories.")
        (@arg DIR_A: +required "First directory, e.g. newer version")
        (@arg DIR_B: +required "Second directory, e.g. older version")
        (@arg LINEAR: -l --linear "Disable concurrent processing")
    ).get_matches();

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
