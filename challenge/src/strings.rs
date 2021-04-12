// Primitive str = immutable fixed-length string somewhere in memory
// String = Growable, heap-alocated data structure. Use when you need to modify or own string data 

pub fn run() {
    let prim_hello = "Primitive String Hello";
    let mut hello = String::from("Hello ");

    //Get length
    println!("Length: {} and {}", hello.len(), prim_hello.len());

    // Push String
    hello.push('W');
    hello.push_str("orld!");

    //Capacity in bytes
    println!("Capacity:{}", hello.capacity());
    //Check if empty
    println!("IsEmpty:{}", hello.is_empty());
    //Contains
    println!("Contains 'world':{}", hello.contains("World"));
    //Replace
    println!("Replace 'Hello':{}", hello.replace("Hello", "Welcome"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{} and {}", prim_hello, hello);
}