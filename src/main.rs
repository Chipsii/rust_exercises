fn main() {
    let x = 5;
    println!("Value of x: {}", x);
    let x = sauce(x);
    println!("Value of x: {}", x);
}

fn sauce(x: i32) -> i32 {
    let spider = x;
    let x = if spider != 5 {
        spider
    } else {
        spider * 2
    };

    println!("Value of x: {}", x);
    x + 1
}
