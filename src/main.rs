use std::cmp::max;

fn main() {
    let cat = std::cmp::min(6, 17);
    println!("age of cat is {}", cat);

    let dog = max(06, 17);
    println!("age of dog is {}", dog);

    let name = "Rust".to_string();
    let age = str::len(&name);
    println!("{} is {} years old", name, age);
}
