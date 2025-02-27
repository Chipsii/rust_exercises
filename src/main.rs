fn main() {
    let x = 5;
    println!("Value is: {}", x);
    let x = { 5 };
    println!("Value IS: {}", x);

    let x = {
        let y = 5;
        let z = 10;
        y + z
    };
    println!("Value of x is: {}", x);
}
