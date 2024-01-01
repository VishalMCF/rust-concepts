fn main() {
    let x = 6;
    println!("x = {}", x);

    // this would not work
    // x = 7

    let mut y = 9;
    println!("y = {}", y);

    // this will work
    y = 10;
    println!("updated value of y = {}", y);

    // const cannot be declared with mut keyword
    const COSTANT: i8 = 98;
    println!("constant value = {}", COSTANT);

    // this is not allowed
    // const mut CONSTAT: i32 = 345
}
