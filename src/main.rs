fn main() {
    let result1 = explicit_return(5);
    let result2 = implicit_return(5);

    println!("Explicit return: {}", result1);
    println!("Implicit return: {}", result2);
}

fn explicit_return(x: i32) -> i32 {
    return x * 2;
}

fn implicit_return(x: i32) -> i32 {
    x * 2
}
