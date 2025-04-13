use rust_macros1::AutoDisplay;

#[derive(Debug, AutoDisplay)]
struct Person {
    id: i64,
    name: String,
    age: u8,
    hobby: String,
}

fn main() {
    let p = Person {
        id: 1,
        name: "HeisenBerg".to_string(),
        age: 20,
        hobby: "programming".to_string(),
    };

    println!("{}", p);
}
