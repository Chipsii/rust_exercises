fn main() {
    let x = (20, 25);
    println!("first element is {}", x.0);
    let idea: String = "Hello sis".to_string();
    let idea2 = sleep(idea);
    println!("{}", idea2);
}

fn sleep(x: String) -> String {
    let idea2 = x.len().to_string();
    idea2
}
