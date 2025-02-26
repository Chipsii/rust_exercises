fn main() {
    let x = 5;
    let y = 7;
    let z = x + y;
    let alpha = value_x(z);
    println!("The value of alpha is: {}", alpha);

    let x = vec![2, 4, 6].iter().map(|x| x * 2).fold(0, |x, y| x + y);
    println!("The value of x is: {}", x);
}

fn value_x(z: i32) -> i32 {
    println!("The value of x is: {}", z);
    z
}
