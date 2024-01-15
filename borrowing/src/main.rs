// main concept :- borrowing is the concept in rust which states that the ownership of any data can be transferred temporarily
// to another variable and when that variable is done using the data to which it was holding on to will be return to the variable from where it was borrowed
// initially

// only one variable can own a piece of memory
// variables can borrow ownership to other pieces of memory
// once the variable who has borrowed the ownership is destroyed the ownership will return back to the initial variable

fn main() {
    println!("Hello, borrowing concept in rust!");

    // let a = 6;
    // let b = &a;

    // the below combinnation of two lines will work. first we are using *b to access to value to which the reference is pointing to
    // then when we are using a, b will be destroyed ad it will not be borrowing any value
    // println!("correct combination 1:- {}",*b);
    // println!("correct combination 2:- {}",a);

    // this will also work. because b borrowed the value from a and then we are using a again it but because a is immutable rust will allow
    // println!("correct combination 1:- {}",a);
    // println!("correct combination 2:- {}",*b);

    
    
    // let mut a = 6;
    // let b = &mut a;

    // the below two cobinations will work. b borrowed the value and after sing we are again using a and b will be removed. because b was decalred mut reference
    // println!("{}",*b);
    // println!("{}",a);

    // this will not work because after b borrowed the vaue from a and we print the a. the a snatched the owweship back from b.
    // println!("{}",a);
    // println!("{}",*b);


    // here we have declared b in its own scope ad when the scope is destroyed the owership will return back to a
    // let mut a = 6;
    // {
    //     let b = &mut a;
    //     *b +=2;
    //     println!("{}",*b);
    // }
    // println!("{}",a);

    let mut v = vec![1,2,3,4,5,6,6,7,8];
    // here in the for loop the ownership of v has been borrowed in immutable way. So inside the for loop we cannnot perform any mutable operations.
    for i in &v {
        println!("{}",i);
        // this will not work
        // v.push(8);
    }

    // this is fine
    v.push(9);

}
