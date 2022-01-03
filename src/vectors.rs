// Vectors are a kind of re-sizable array, but all elements must be
// in the same type.

// REMEMBER: If you want to change the vector values, you need to specify the variable
// as mutable: let mut v = vec![];

fn main () {    

    // create a empty vector using the vec! macro
    let mut _v0: Vec<i32> = vec![];
    // create a empty vector using the new() macro
    let mut _v1: Vec<i32> = Vec::new();

    // REMEMBER Capacity: A empty vector has capacity 0
    // Every time when you push a item that exceeds the capacity
    // all the vector items will be realocated, wich is potencially slow.
    // Whenever possible, set the vector capacity, wich in range
    // of 0-capacity, makes .push() essentially a free instruction.
    // Yes, performance!
    let mut v2 = Vec::with_capacity(10);
    for i in 0..10 {
        v2.push(i); // essentially a free instruction to your program.
    }

    println!("{:?}", v2);

    v2.push(11); // This is also possible, but may make the vector reallocate
}