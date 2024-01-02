fn main() {
    println!("Hello String!!");

    // how to declar strings in rust
    let cat: &str = "Meow G";
    println!("cat -> {}", cat);

    // string slices are immutable
    let _another_cat: &'static str = "Fluffy";
    println!("another_cat -> {}", cat);

    // declaring string using new method()
    let dog = String::new();

    // instantiate from a string slice
    let mut dog = String::from("Max");

    println!("another_dog -> {} ", dog);

    // rememeber the concept of shadowing in Rust

    // How does format macro works?
    let owner = format!("Hi I am {} owner of dog {} ", "Mark", "Bruno");
    println!("{}", owner);

    // printing the length of the string
    println!("{}", owner.len());

    // push and push_str functions
    // remember that these do not work on slices only on string

    // push -> push one character
    // push_str -> push string
    dog.push(' ');
    dog.push_str("the dog");
    println!("{}", dog); // Max the dog

    // replace function()
    let dog = dog.replace("the", " is my ");
    println!("{}", dog); // Max is my dog
}
