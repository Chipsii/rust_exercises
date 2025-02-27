fn main() {
    let x = 5;
    let y = 10;
    let z = x + y;
    let unreal = engine(z);
    println!("Hello {}", unreal);

    let x = vec![2, 4, 6].iter().map(|x| x * 5).fold(0, |x, y| x + y);
    println!("Hello {}", x);
}

fn engine(z: i32) -> i32 {
    println!("value is {}", z);
    z
}
