// functions3.rs
//
// Make me compile!
//
// If you need help, open the corresponding README.md or run: rustlings hint functions3

// I AM NOT DONE

fn main() {
    call_me();
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
