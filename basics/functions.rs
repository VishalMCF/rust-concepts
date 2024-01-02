fn main() {
    // for i in 1..8 {
    //     say_hi();
    // }
    let mut name: &str = "Billa";
    let greeting: String = get_greeting(&mut name);
    println!("{}", greeting)
    //say_hi_with_name(&mut name);
}

// fn say_hi() {
//     println!("Hello Rust functions");
// }

// pass by value
// fn say_hi_with_name(name: &str) {
//     println!("Hello {}", name);
// }

// in rust when we pass the value to another function, the function is not copying the value into its parameters
// instead it is borrowing the value from the calling function

// pass by reference
fn say_hi_with_name(name: &mut &str) {
    *name = "Alex";
    println!("Hello {}", name);
}

fn get_greeting(name: &mut &str) -> String {
    let greeting: String = format!("Hello {}", name);
    return greeting;
}
