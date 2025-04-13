use rust_macros1::AutoDisplay;

#[derive(AutoDisplay)]
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

    // Person { id: 1, name: HeisenBerg, age: 20, hobby: programming, }
    println!("{}", p);
}
