use std::collections::HashMap;

macro_rules! create_map {
    // Match key-value pairs separated by commas
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        { // Use a block to create a scope
            let mut map = HashMap::new();
            $( // Repeat for each matched pair
                map.insert($key, $value);
            )*
            map // Return the map
        }
    };
}

fn main() {
    let my_map = create_map! {
        "one" => 1,
        "two" => 2,
        "three" => 3
    };
    println!("{:?}", my_map); // Output: {"one": 1, "two": 2, "three": 3} (order may vary)

    let my_map = create_map! {
        1 => "A",
        2 => "B",
        3 => "C"
    };
    println!("{:?}", my_map); 
}
