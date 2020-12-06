// features1.rs
// Conditional compilation example:
// `cargo +stable test --features "english" --release`
// `cargo +stable test --features "korean" --release`
// `cargo +stable test --features "russian" --release`
// https://doc.rust-lang.org/cargo/reference/features.html
// https://doc.rust-lang.org/reference/conditional-compilation.html
// Also, try `cargo expand` https://github.com/dtolnay/cargo-expand
// `cargo expand --features "russian"`
// Execute `rustlings hint features1` for hints!

// I AM NOT DONE

#[cfg(feature = "korean")]
pub fn hello() -> String {
    String::from("안녕하세요!")
}

#[cfg(feature = "russian")]
pub fn hello() -> String {
    String::from("Привет мир!")
}

#[cfg(feature = "english")]
pub fn hello() -> String {
    String::from("Hello World!")
}

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn say_hello() {
        #[cfg(feature = "korean")]
        let r = "안녕하세요!".to_string();

        #[cfg(feature = "english")]
        let r = "Hello World!".to_string();

        #[cfg(feature = "russian")]
        let r = "Привет мир!".to_string();

        let a = hello();

        assert_eq!(r, a);
    }
}
