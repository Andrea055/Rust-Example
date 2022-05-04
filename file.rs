use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Error during open of file.");
}
