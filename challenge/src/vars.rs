// variables hold primitive data or references to data
// Immutable
// Block-scoped language

pub fn run() {
    let name = "Nuno";
    let mut age = 37; // define mut for mutable variables
    println!("My name is {} and I am {}", name, age);
    age = 24;
    println!("My name is {} and I am {}", name, age);

    // Define constants
    const ID: i32 = 00123;
    println!("ID: {}", ID);

    //Assign multiple variables
    let ( my_name, my_age ) = ("Nuno", 24);
    println!("{} is {}", my_name, my_age);
}