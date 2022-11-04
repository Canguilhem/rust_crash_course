// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - use when you need to modify or own string data

pub fn run(){
    // Immutable
    let hello = "Hello";

    // Mutable
    let mut hello_2 = String::from("Hello ");

    // Get Length
    println!("Length : {}", hello.len());

    // push a single char
    hello_2.push('W');

    // push a more than a char
    hello_2.push_str("orld");

    // Capacity in bytes
    println!("Capacity: {}", hello_2.capacity());

    // Empty ?
    println!("Is empty: {}", "".is_empty());

    // Contains ?
    println!("Contains world: {}", hello_2.contains("World"));

    // Replace
    println!("Replace: {}", hello_2.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello_2.split_whitespace(){
        println!("{}", word)
    }

    // Create string with capacity
    let mut s= String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing -> Nothing happens if assertion pass 
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}