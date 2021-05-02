// clippy2.rs
//
// Make me compile! 
//
// If you need help, open the corresponding README.md or run: rustlings hint clippy2

// I AM NOT DONE

fn main() {
    let mut res = 42;
    let option = Some(12);
    for x in option {
        res += x;
    }
    println!("{}", res);
}
