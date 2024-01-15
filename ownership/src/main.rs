// Main concept:- only one piece can own a piece of memory
// For primitive types:- copying data is cheap
// for complex types owership is transferred



fn main() {
    println!("Hello, Ownership!");

    // here i have declared two variables. 
    // in this statement j = i; the ownership is being transferred via copying
    let i = 5;
    let j = i;

    println!("{}",j);

    // this will work because the ownership was trasferred via copying. so i has still hold of its values
    println!("{}",i);

    //  lets deccalre complex types and see how ownership is transferred
    let v = vec![1,2,3,4,5,6,7];
    // let mut v = vec![1,2,3,4,5,6,7];
    let w = v;

    println!("{:?}",w);

    // the below line will not work here because the value of vector to which v was pointing has been borrowed by w
    // println!("{:?}",v);

    // if the variable was declared mut the it can get its ownership back because it can be reassigned twice.
    // v = w;
    // println!("{:?}",v);

    let foo = |v: Vec<i32>| -> Vec<i32> {
        println!("vector was passed to the function");
        v
    }

    let w = foo(w);
    println!("{:?}",w);

    // let foo = |v: Vec<i32>| -> (){
    //     println!("vector was passed to the function");
    // };
    // let w = foo(w);

}
