// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run(){
    let name="Anthony";
    let mut age =31;
    println!("My name is {} and i am {}", name, age);
    
    age+=1;
    println!("My name is {} and i am {}", name, age);

    // Define constant
    const ID:i32=001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age)= ("Anthony", 31);
    println!("{} is {}", my_name, my_age);
}