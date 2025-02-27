fn main() {
    let x = 5;

    let venom = carnage(x);
    println!("The value of venom is: {}", venom);
    {
        let x = vec![1, 2, 3].iter().map(|x| x + 1).fold(0, |x, y| x + y);
        println!("we are: {}", x);
    }
}

fn carnage(x: i32) -> i32 {
    println!("we will: {}", x);
    x
}
