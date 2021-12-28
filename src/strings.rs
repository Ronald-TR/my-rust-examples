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
    let mut s = String::from(x); // muttable type, allocated in heap
    s.push_str(", World!");

    println!("{}", s);

    let word = first_word(&s);
    // let word = first_word(&s[..]); // this form are also valid, because a &String also works as slice.
    println!("{}", word);
}

// This function returns a slice of the first word into the string,
// if no empty space found, we assume that the entire string is the first word.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..]
}