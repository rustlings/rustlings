//readfile1.rs
// How would you read a file without a &mut File.
// Find a suitable type that implements Read and Write trait to read a File.
// Make me compile! Execute `rustlings hint readfile1` for hints

use std::fs::File;

fn main() {
    let path = "exercises/stdio/dummy.txt";
    let file = File::open(path).expect("No file dummy.txt"); // Don't Change this line.
    let mut buf = ???;
    file.read_to_string(&mut but);
    println!("{}", s);
}
