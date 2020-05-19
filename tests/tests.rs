use backup_diff::{get_directory_map, get_diff};

fn get_test_data_path(case: &str, target: &str) -> String {
    format!("./tests/test_data/case_{}/{}", case, target)
}

#[test]
fn same_files_different_filenames() {
    let dir_a = get_directory_map(&get_test_data_path("same_files_different_filenames","after"));
    let dir_b = get_directory_map(&get_test_data_path("same_files_different_filenames", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert!(new.len() == 0);
    assert!(del.len() == 0);
}

#[test]
fn old_file_only() {
    let dir_a = get_directory_map(&get_test_data_path("old_file_only","after"));
    let dir_b = get_directory_map(&get_test_data_path("old_file_only", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert!(new.len() == 0);
    assert!(del.len() == 1);
}

#[test]
fn new_file_only() {
    let dir_a = get_directory_map(&get_test_data_path("new_file_only","after"));
    let dir_b = get_directory_map(&get_test_data_path("new_file_only", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert!(new.len() == 1);
    assert!(del.len() == 0);
}

#[test]
fn new_and_old() {
    let dir_a = get_directory_map(&get_test_data_path("new_and_old","after"));
    let dir_b = get_directory_map(&get_test_data_path("new_and_old", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert!(new.len() == 1);
    assert!(del.len() == 1);
}

#[test]
fn empty_directories() {
    let dir_a = get_directory_map(&get_test_data_path("empty_directories","after"));
    let dir_b = get_directory_map(&get_test_data_path("empty_directories", "before"));

    let (new, del) = get_diff(&dir_a, &dir_b);
    assert!(new.len() == 0);
    assert!(del.len() == 0);
    assert!(dir_a.len() == 0);
    assert!(dir_b.len() == 0);
}