use std::fs::File;
use criterion::Criterion;
use backup_diff::{hash_file_list, hash_file_list_parallel};
use std::io::Write;
use std::fs;
use criterion::{criterion_group, criterion_main};
use rand::random;
use std::time::Duration;

const POPULATE_FILE_NUM: usize = 500;
const POPULATE_FILE_SIZE: usize = 70_000;
const POPULATE_PATH: &str = "./tests/test_data/generated/";

fn generate_random_string(size: usize) -> String {
    (0..size).map(|_| random::<char>()).collect()
}

/// assuming tests won't fail during population and cleanup process
fn populate_data() -> Vec<String> {
    fs::create_dir_all(POPULATE_PATH);
    let mut paths = Vec::new();
    for num in 0..POPULATE_FILE_NUM {
        let path = format!("{}test_{}.data", POPULATE_PATH, num);
        let mut file = File::create(&path);
        match file {
            Ok(mut s_file) => {
                s_file.write_all(generate_random_string(POPULATE_FILE_SIZE).as_bytes());
            }
            Err(err) => {
                // file already created
                // do nothing
            }
        }
        paths.push(path);
    }
    paths
}

fn clean_data(paths: &Vec<String>) {
    println!("Cleaning data.");
    for path in paths.iter() {
        fs::remove_file(path);
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    // populate data
    println!("Generating data.");
    let paths = populate_data();
    println!("Generated {} test files.", paths.len());

    // configure
    let mut group = c.benchmark_group("Hashing");
    group.sample_size(10);
    group.measurement_time(Duration::from_secs(120));

    // run benchmarks
    group.bench_function("linear", |b| b.iter(|| hash_file_list(paths.clone(), Some(true))));
    group.bench_function("concurrent", |b|b.iter(|| hash_file_list_parallel(paths.clone(), Some(true))));

    // cleanup
    group.finish();
    clean_data(&paths);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);