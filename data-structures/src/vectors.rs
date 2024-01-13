// Vector Basics

// 1. Vectors are arrays of variable size. They are implemented using structs and are not built in types

// 2. How to declare them ? 2 ways one using constructor and the other one using macros
// a. let mut primes: Vec<i32> = Vec:: new();
// b. let mut primes = vec![2,3,5,7,11,13]; 

// 3. to push any new elements in the vector we must declare them mut
// a. Adding elements -> use push method
// b. Removing elements -> use remove(index_value) method

// 4. Just like arrays in, we can also create vectors using default values
// a. let mut numbers = vec![3: 10];
// b. const DEFAULT: i32  = 6;
// c. let mut numbers = [DEFAULT; 8];

// 5. to update any element in the vector we must make the vector first mutable and then using indexing oeprator we can modify it


pub mod vector_concept {
    
}