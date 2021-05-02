// using_as.rs
//
// Type casting in Rust is done via the usage of the `as` operator.
// Please note that the `as` operator is not only used when type casting.
// It also helps with renaming imports.
//
// The goal is to make sure that the division does not fail to compile
// and returns the proper type.
//
// If you need help, open the corresponding README.md or run: rustlings hint using_as

// I AM NOT DONE

fn average(values: &[f64]) -> f64 {
    let total = values.iter().fold(0.0, |a, b| a + b);
    total / values.len()
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
