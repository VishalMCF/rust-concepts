// Arrays Basics

// 1. How to declare them?

// a. let primes = [1,2,3,4,5,6,7]
// b. let doubles:[f64; 4] = [2.0,4.0, 6.0, 8.0];
// c. To print them you need to use degub printer println!("{:?}",doubles)

// 2. Arrays cannot be resized
// 3. Elemets are indexed
// 4. modifying elemets are possible if the array is declared as mut
// 5. iterator can alsoe be used to loop through the array

pub mod arrays_concept {
    pub fn arrays_declaration(){
        let primes = [1,2,3,5,7,11,13,17,19,23,29,31];
        let doubles:[f64;4] = [2.0,4.0, 6.0, 8.0];  
        println!("{:?}", primes);
        println!("{:?}", doubles);
    }

    pub fn default_arrays(){
        let numbers = [0;15];
        const DEFAULT:i32 = 8;
        println!("{:?}", numbers);

        let numbers = [DEFAULT;10];
        println!("{:?}", numbers);
    }

    pub fn array_iterator(){
        let primes = [1,2,3,5,7,11,13,17,19,23,29,31];
        for prime in primes.iter() {
            println!("{}",prime+4);
        }
    }
}