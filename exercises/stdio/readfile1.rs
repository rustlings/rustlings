//readfile1.rs

// Make me compile! Execute `rustlings hint readfile1` for hints

use std::fs::File;

fn main() {
    let path = "exercises/stdio/dummy.txt";
    let file = File::open(path).expect("No file dummy.txt");
    let mut s = String::new();
    file.read_to_string(&mut s);
    println!("{}", s);
}
