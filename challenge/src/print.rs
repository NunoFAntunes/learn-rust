pub fn run() {
    //Print to console
    println!("Hello from the print.rs file!");

    //Basic Formatting
    println!("The {} Number is larger than {}", 2, 1);

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    //Named Arguments
    println!("{name} likes to play {activity}", 
    name="Nuno", 
    activity="football"
    );

    //Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10 , 10);

    //Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10+10);
}