// USed to check condition of sth and act

pub fn run() {
    let age = 20;
    let check_id: bool = false;
    let knows_person_of_age = true;
    
    // If else
    if age >= 18 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?")
    } else if age < 18 && check_id {
        println!("Bartender: Sorry, you have to leave.")
    } else {
        println!("Bartender: Check ID.")
    }

    // Shorthand if
    let is_of_age = if age >= 18 { true } else { false };
    println!("Is Of age: {}", is_of_age);
}