pub fn run() {
    // Primitive Array
    let arr1 = [1,2,3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, arr2));

    //Non primitives: If you assign another var to a piece of data, the first var will no longer hold that value.
    // You need to use a reference (&) to point to the resource

    // Vector
    let vec1 = vec![1,2,3];
    let vec2 = &vec1;

    println!("Values: {:?}", (&vec1, vec2));
}