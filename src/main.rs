fn main() {
    let shark = "Sammy".to_string();
    let mut fish: Vec<String> = Vec::new();
    fish.push(shark);
    println!("{:?}", fish);

    let mut fish: Vec<String> = std::vec::Vec::new();
    fish.push("Dory".to_string());
    println!("{:?}", fish);
}
