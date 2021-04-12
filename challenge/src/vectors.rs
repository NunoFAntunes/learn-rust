// Vectors Resizable arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4];

    println!("{:?}", numbers);

    println!("Single value: {}", numbers[0]);
    numbers[2] = 20;
    println!("Vector length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);

    // Add onto Vector
    numbers.push(5);
    numbers.push(6);
    println!("{:?}", numbers);
    numbers.pop();
    println!("{:?}", numbers);

    //loop through vector vals
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    //Loop and mutate
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}