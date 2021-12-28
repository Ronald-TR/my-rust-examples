/*
    Here we see two ways to represent strings in Rust, an immutable and a muttable form.
    The immutable type is the fast-efficient type, it's pushed into the stack memory layer,
    and will literally be into the final binary.

    The second one is a mutablle type, which means its size is unknown by the compiler,
    so it will be allocated into the heap memory layer.

    Both will be dropped after the scope ends.
*/
fn main () {
    let x = "Hello"; // immutable type, allocated in stack
    let mut s = String::from("Hello"); // muttable type, allocated in heap
    s.push_str(", World!");

    println!("{}", s);
    println!("{}", x);
}