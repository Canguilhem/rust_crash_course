// Arg[0] is always the relative path to executable

use std::env;

pub fn run(){
    let args: Vec<String> = env::args().collect();
    let command= args[1].clone();
    let name="Anthony";
    let status = "100%";

    println!("Args {:?}", args);

    // println!("Command {}", command);

    if command == "hello"{
        println!("Hi {} how are you?", name)
    } else if command == "status"{
        println!("Status is {}", status)
    } else {
        println!("No a valid command")
    }
}