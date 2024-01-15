// Lifetime
// Main cocept:- the indication of how long the object will live
// Rust prevents parts of obejct outliving the object
// Lifetime ellision:- compiler builds lifetime for us hen evident

#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Dog<'l> {
    name: String,
    owner: &'l Person
}

fn main() {
    println!("{}",get_str());
    let p1 = Person{name: String::from("John")};
    let d1 = Dog{name:String::from("Spike"), owner: &p1};
    println!("{:?}", d1);
}

// So what we are doing below is that we are defining the lifetime of the return type. So &'static lifetime meas that as long as the program lives
// the value of "hello Lifetime!" will stay in the memory
fn get_str() -> &'static str {
    "hello Lifetime!"
}
