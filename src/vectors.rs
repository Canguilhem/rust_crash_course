// Vectors - are resizable arrays

use std::mem;

pub fn run(){
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    // Re-assign value
    numbers[2]=-16;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single val
    println!("Single value: {}", numbers[0]);

    // Get vector length
    println!("vector length: {}", numbers.len());

    // Vector are stack allocated
    println!("vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice:&[i32] = &numbers[1..3];
    println!("slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x)
    }

    // Loop and mutate values
    for x in numbers.iter_mut(){
        *x *=2;
    }

    println!("Numbers in Vector: {:?}", numbers);
}