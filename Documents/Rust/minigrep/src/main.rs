use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("query = {}", query);
    println!("filename = {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("unable to read file");

    println!("Text of tile:\n{}", contents);

    // create a parse command function for query and filename s
}
