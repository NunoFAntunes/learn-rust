/*
--Primitive types--
Integers u8, u16, i16, u32, i32, u64, i64, u128, i128
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

// Statically Typed Language.
// Compiler must know var type.
// Usually compiler can infer the type based on the value and how we use it

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 23478649462347896;

    // FInd max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i32: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater_than = 10 < 5;

    println!("{:?}", (x, y, z, is_active, is_greater_than));

    // Char
    let a1 = 'a';
    let face = '\u{1F600}';
    println!("{:?}", (a1, face));
}