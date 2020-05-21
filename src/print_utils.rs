use multimap::MultiMap;

/// print first value in given map by keys, panics if key does not exist
pub fn print_select_values(map: &MultiMap<String, String>, keys: &Vec<String>) {
    for key in keys.iter() {
        println!("  {}", map.get(key).unwrap());
    }
    if keys.is_empty() {
        println!("  <Empty>")
    }
}

/// print duplicate entries
pub fn print_duplicates(duplicates: &Vec<Vec<String>>) {
    for dup in duplicates.iter() {
        for fp in dup.iter().enumerate() {
            match fp {
                (0, str) => {
                    println!("  [{}]", str)
                }
                (_, str) => {
                    println!("    {} (duplicate)", str)
                }
            }
        }
    }
    if duplicates.is_empty() {
        println!("  <Empty>");
    }
}

/// print given errors
pub fn print_errors(errors: &Vec<String>) {
    for err in errors {
        println!("ERR: {}", err);
    }
    if errors.is_empty() {
        println!("  <Empty>");
    }
}