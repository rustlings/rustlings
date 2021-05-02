// modules2.rs
//
// Make me compile!
//
// If you need help, open the corresponding README.md or run: rustlings hint modules2

// I AM NOT DONE

mod delicious_snacks {
    use self::fruits::PEAR as fruit;
    use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
