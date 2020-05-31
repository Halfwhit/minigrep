use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    // $arg[0] contains the program's relative path and name
    let query = &args[1];
    let filename = &args[2];

    println!("Searching for '{}' in the file named '{}'.\n", query, filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("Complete text:\n{}\n", contents)
}
