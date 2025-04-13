// Define the declarative macro `string_vec`
macro_rules! string_vec {
    // Match rule:
    // $() -> Indicates repetition
    // $element:expr -> Matches any Rust expression and binds it to the name $element
    // , -> Separator for the repetition (comma)
    // * -> Repeat zero or more times
    // $(,)? -> Optionally match a trailing comma
    ( $( $element:expr ),* $(,)? ) => {
        // Expansion:
        // This code replaces the macro invocation.
        // It uses the standard `vec!` macro internally.
        vec![
            // $() -> Indicates repetition corresponding to the match rule
            $(
                // For each matched $element expression:
                // Call .to_string() on the result of the expression
                // to ensure we get a String.
                $element.to_string()
            ),* // Repeat zero or more times, separated by commas
        ]
    };
}

fn main() {
    let vec1: Vec<String> = string_vec!["hello", "world", "rust"];
    println!("vec1: {:?}", vec1);

    let middle_word: &str = "is";
    let vec3: Vec<String> = string_vec!["Rust", middle_word, "awesome",]; // Note the trailing comma
    println!("vec3: {:?}", vec3);
}
