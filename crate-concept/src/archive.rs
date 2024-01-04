pub mod arch {
    pub fn arch_file(name: &str){
        println!("Arhciving the file {}", name);
    }
}

// we can use the above module in our main.rs file using the concept of crate
// add the followinng line of code in the main.rs file
// use crate::archive::arch::arch_file;

// syntax is like this -> 'crate' keyword:: {rust_file_name} :: {module_name} :: {function_name}

// we can also alias using the as keyword

// use crate::archive::arch::arch_file as arch

// We can also use external crates which other developers have created for us:-
// 1) must be added to the toml file as follows:-
// [depedencies]
// rand = "0.7.3"