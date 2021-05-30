// tests3.rs
// The tests aren't testing our function -- make it do that in such a way that
// the tests pass, but just replace the ???
// Execute `rustlings hint tests3` for hints :)

// I AM NOT DONE

// Don't modify this function
pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        assert!(???);
    }

    #[test]
    fn is_false_when_odd() {
        assert_eq!(false, ???);
    }

    #[test]
    fn is_false_when_odd2() {
        assert_ne!(true, ???);
    }
}
