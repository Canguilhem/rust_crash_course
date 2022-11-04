// Used to check condition of something and act

pub fn run(){
    let age: u8 = 18;
    let check_id: bool = true;
    let knows_person_of_age= true;

    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: What would you like to drink?")
    }else if age< 21 && check_id{
        println!("Bartender: You shouldn't be here")
    }else{
        println!("Bartender: I'll need to see your ID")
    }

    // shorthand if
    let is_of_age = if age >= 21 {true} else {false};
    println!("is of age: {}", is_of_age);
}