#[derive(Debug)]
struct Data {
    id: String,
}

// Define the declarative macro `string_vec`
macro_rules! build_data_list {
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
                Data {id: $element.to_string() }
            ),* // Repeat zero or more times, separated by commas
        ]
    };
}

fn main() {
    let data_list: Vec<Data> = build_data_list!["one", "two", "three"];
    println!("{:?}", data_list);

    data_list.into_iter().for_each(|x| println!("{}", x.id));
}
