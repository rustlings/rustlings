// iterators5.rs

pub fn fibonacci(i: u64) -> u64 {
    // Complete this function to return the ith fibonacci number
    // Do not use:
    // - return
    // For extra fun don't use:
    // - imperative style loops (for, while)
    // - let
    // For the most fun don't use:
    // - recursion
    // Scroll down for hints.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_of_1() {
        assert_eq!(1, fibonacci(1));
    }
    #[test]
    fn fibonacci_of_2() {
        assert_eq!(2, fibonacci(2));
    }

    #[test]
    fn fibonacci_up_to_10() {
        assert_eq!(
            "[1, 1, 2, 3, 5, 8, 13, 21, 34, 55]",
            format!("{:?}", (0..10).map(fibonacci).collect::<Vec<u64>>())
        );
    }
}






















// In an imperative language you might write a for loop to iterate through
// add and keep track of the last two values into a mutable variables.
// Or you might write code more functionally with recursion and a match clause.
// But you can also use ranges and iterators to solve this in rust.
