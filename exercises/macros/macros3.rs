// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// If you need help, open the corresponding README.md or run: rustlings hint macros3

// I AM NOT DONE

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
