pub fn run() {
    greeting("Hello", "Nuno", 24);
    
    // Bind fn vals to vars
    let get_sum = add(10,15);
    println!("Sum:{}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3; //Allows use of vars outside of fn scope, like n3.
    println!("C Sum:{}", add_nums(3,6));
}

fn greeting(greet: &str, name: &str, age: u8) {
    println!("{}, my name is {} and I am {}", greet, name, age);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}