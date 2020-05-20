use backup_diff::{get_directory_map, get_diff, find_duplicates};

fn get_test_data_path(case: &str, target: &str) -> String {
    format!("./tests/test_data/case_{}/{}", case, target)
}

#[test]
fn same_files_different_filenames() {
    let dir_a = get_directory_map(&get_test_data_path("same_files_different_filenames","after"));
    let dir_b = get_directory_map(&get_test_data_path("same_files_different_filenames", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 0);
    assert_eq!(del.len(), 0);
}

#[test]
fn old_file_only() {
    let dir_a = get_directory_map(&get_test_data_path("old_file_only","after"));
    let dir_b = get_directory_map(&get_test_data_path("old_file_only", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 0);
    assert_eq!(del.len(), 1);
}

#[test]
fn new_file_only() {
    let dir_a = get_directory_map(&get_test_data_path("new_file_only","after"));
    let dir_b = get_directory_map(&get_test_data_path("new_file_only", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 1);
    assert_eq!(del.len(), 0);
}

#[test]
fn new_and_old() {
    let dir_a = get_directory_map(&get_test_data_path("new_and_old","after"));
    let dir_b = get_directory_map(&get_test_data_path("new_and_old", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 1);
    assert_eq!(del.len(), 1);
}

#[test]
fn empty_directories() {
    let dir_a = get_directory_map(&get_test_data_path("empty_directories","after"));
    let dir_b = get_directory_map(&get_test_data_path("empty_directories", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 0);
    assert_eq!(del.len(), 0);
    assert_eq!(dir_a.len(), 0);
    assert_eq!(dir_b.len(), 0);
}

#[test]
fn nesting_file_order() {
    let dir_a = get_directory_map(&get_test_data_path("nesting_file_order","after"));
    let dir_b = get_directory_map(&get_test_data_path("nesting_file_order", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 0);
    assert_eq!(del.len(), 0);
    assert_eq!(dir_a.len(), 3);
    assert_eq!(dir_b.len(), 3);
}

#[test]
fn duplicates_simple() {
    let dir_a = get_directory_map(&get_test_data_path("duplicates_simple","after"));
    let dir_b = get_directory_map(&get_test_data_path("duplicates_simple", "before"));

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
    let dir_a = get_directory_map(&get_test_data_path("duplicates_multiple","after"));
    let dir_b = get_directory_map(&get_test_data_path("duplicates_multiple", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert_eq!(new.len(), 1);
    assert_eq!(del.len(), 1);
    assert_eq!(dir_a.len(), 2);
    assert_eq!(dir_b.len(), 2);
    let dup_a = find_duplicates(&dir_a);
    let dup_b = find_duplicates(&dir_b);

    assert_eq!(dup_a.len(), 2);
    assert_eq!(dup_b.len(), 2);
    assert_eq!(dup_a.get(0).unwrap().len(), 2);
    assert_eq!(dup_b.get(0).unwrap().len(), 2);
    assert_eq!(dup_a.get(1).unwrap().len(), 2);
    assert_eq!(dup_b.get(1).unwrap().len(), 2);
}