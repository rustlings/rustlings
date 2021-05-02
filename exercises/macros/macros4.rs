// macros4.rs
//
// Make me compile!
//
// If you need help, open the corresponding README.md or run: rustlings hint macros4

// I AM NOT DONE

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    }
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
