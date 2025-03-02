use std::cmp::max;

fn main() {
    let cat = std::cmp::min(6, 9);
    println!("age of cat is {}", cat);

    let dog = max(6, 9);
    println!("age of dog is {}", dog);
}
