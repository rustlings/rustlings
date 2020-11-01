use glob::glob;
use std::fs;
use std::path::PathBuf;

#[test]
fn all_exercises_require_confirmation() {
    for path in all_exercises() {
        let source = fs::read_to_string(&path).unwrap();
        source.matches("// I AM NOT DONE").next().expect(&format!(
            "There should be an `I AM NOT DONE` annotation in {:?}",
            path
        ));
    }
}

fn all_exercises() -> impl Iterator<Item = PathBuf> {
    glob("exercises/**/*.rs")
        .unwrap()
        .map(|result| result.expect("Unable to traverse exercises folder"))
}
