// multiple modules are grouped into a crate
// two types

// 1) binary crates :-> 
// 2) library crates :-> 

// cargo is used to manage crates

// use crate::archive::arch::arch_file

// How to generate random numbers in rust?
// use rand::Rng;
// let mut rng = rand::thead_rng();
// rng.gen()
mod archive;
use crate::archive::arch::arch_file as arch;
use rand::Rng;
use rand::distributions::Alphanumeric;

fn main() {
    arch("somefile.txt");

    // single randpm number
    let mut rnd_gen = rand::thread_rng();
    let a:i32 = rnd_gen.gen();
    println!("single random number {}",a);

    // radom ints in a range
    println!("bounded int: {}",rnd_gen.gen_range(0,100));
    println!("bounded float: {}",rnd_gen.gen_range(0.0,100.0));

    // random flotas in a range
    let rand_string: String = rand::thread_rng().sample_iter(&Alphanumeric).take(30).collect();

    println!("Gen string: {}", rand_string);
}
