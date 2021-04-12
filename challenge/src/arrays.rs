// Arrays fixed list with same data types. Size must be exact!

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];
//    let numbers: [i32; 5] = [1,2,3,4];

    println!("{:?}", numbers);

    println!("Single value: {}", numbers[0]);
    numbers[2] = 20;
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice {:?}", slice);
}