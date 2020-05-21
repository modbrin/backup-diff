use backup_diff::{get_directory_map, get_diff, find_duplicates, hash_file_list_parallel, hash_file_list};
use defer;
use std::fs::File;
use std::fs;
use std::io::Write;
//use test::Bencher;

fn get_test_data_path(case: &str, target: &str) -> String {
    format!("./tests/test_data/case_{}/{}", case, target)
}

#[test]
fn same_files_different_filenames() {
    let dir_a = get_directory_map(&get_test_data_path("same_files_different_filenames", "after"), false);
    let dir_b = get_directory_map(&get_test_data_path("same_files_different_filenames", "before"), false);

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 0);
    assert_eq!(del.len(), 0);
}

#[test]
fn old_file_only() {
    let dir_a = get_directory_map(&get_test_data_path("old_file_only", "after"), false);
    let dir_b = get_directory_map(&get_test_data_path("old_file_only", "before"), false);

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 0);
    assert_eq!(del.len(), 1);
}

#[test]
fn new_file_only() {
    let dir_a = get_directory_map(&get_test_data_path("new_file_only", "after"), false);
    let dir_b = get_directory_map(&get_test_data_path("new_file_only", "before"), false);

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 1);
    assert_eq!(del.len(), 0);
}

#[test]
fn new_and_old() {
    let dir_a = get_directory_map(&get_test_data_path("new_and_old", "after"), false);
    let dir_b = get_directory_map(&get_test_data_path("new_and_old", "before"), false);

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 1);
    assert_eq!(del.len(), 1);
}

#[test]
fn empty_directories() {
    let dir_a = get_directory_map(&get_test_data_path("empty_directories", "after"), false);
    let dir_b = get_directory_map(&get_test_data_path("empty_directories", "before"), false);

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 0);
    assert_eq!(del.len(), 0);
    assert_eq!(dir_a.len(), 0);
    assert_eq!(dir_b.len(), 0);
}

#[test]
fn nesting_file_order() {
    let dir_a = get_directory_map(&get_test_data_path("nesting_file_order", "after"), false);
    let dir_b = get_directory_map(&get_test_data_path("nesting_file_order", "before"), false);

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 0);
    assert_eq!(del.len(), 0);
    assert_eq!(dir_a.len(), 3);
    assert_eq!(dir_b.len(), 3);
}

#[test]
fn duplicates_simple() {
    let dir_a = get_directory_map(&get_test_data_path("duplicates_simple", "after"), false);
    let dir_b = get_directory_map(&get_test_data_path("duplicates_simple", "before"), false);

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 1);
    assert_eq!(del.len(), 1);
    assert_eq!(dir_a.len(), 1);
    assert_eq!(dir_b.len(), 1);
    let dup_a = find_duplicates(&dir_a);
    let dup_b = find_duplicates(&dir_b);

    assert_eq!(dup_a.len(), 1);
    assert_eq!(dup_b.len(), 1);
    assert_eq!(dup_a.get(0).unwrap().len(), 2);
    assert_eq!(dup_b.get(0).unwrap().len(), 2);
}

#[test]
fn duplicates_multiple() {
    let dir_a = get_directory_map(&get_test_data_path("duplicates_multiple", "after"), false);
    let dir_b = get_directory_map(&get_test_data_path("duplicates_multiple", "before"), false);

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 1);
    assert_eq!(del.len(), 1);
    assert_eq!(dir_a.len(), 2);
    assert_eq!(dir_b.len(), 2);
    let dup_a = find_duplicates(&dir_a);
    let dup_b = find_duplicates(&dir_b);

    assert_eq!(dup_a.len(), 2);
    assert_eq!(dup_b.len(), 2);
    assert_eq!(dup_a.get(0).unwrap().len(), 3);
    assert_eq!(dup_b.get(0).unwrap().len(), 3);
    assert_eq!(dup_a.get(1).unwrap().len(), 3);
    assert_eq!(dup_b.get(1).unwrap().len(), 3);
}

const POPULATE_FILE_NUM: usize = 1_000;
const POPULATE_FILE_SIZE: usize = 10_000;
const POPULATE_PATH: &str = "./tests/test_data/generated/";

fn generate_random_string(size: usize) -> String {
    unimplemented!()
}

/// assuming tests won't fail during population and cleanup process
fn populate_data() -> Vec<String> {
    let mut paths = Vec::new();
    (0..POPULATE_FILE_NUM).map(|num| {
        let path = format!("{}test_{}.data", POPULATE_PATH, num);
        let mut file = File::create(&path).unwrap();
        paths.push(path);
        file.write_all(generate_random_string(POPULATE_FILE_SIZE).as_bytes());
    });
    paths
}

fn clean_data(paths: &Vec<String>) {
    paths.iter().map(|fp| fs::remove_file(fp));
}

// #[bench]
// fn parallel_hashing(b: &mut Bencher) {
//     let paths = generate_data();
//     defer!(clean_data(&paths));
//     b.iter(||{
//         hash_file_list_parallel(paths.clone());
//     })
// }
//
// #[bench]
// fn linear_hashing(b: &mut Bencher) {
//     let paths = generate_data();
//     defer!(clean_data(&paths));
//     b.iter(||{
//         hash_file_list(paths.clone());
//     })
// }