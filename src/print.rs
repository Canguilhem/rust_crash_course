pub fn run(){
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("Number: {}",1);
    println!("{} is from {}", "Anthony","France");

    // Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Anthony","France", "code");

    // Named Arguments
    println!("{name}, likes to play {sport}", name="Julien", sport="Guitar");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10 ,10,10);

    // Placeholder for debug trait
    println!("{:?}", (12,true, "hello"));

    // Basic math
    println!("10+10={}",10+10)
}