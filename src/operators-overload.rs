/* 
In this crate we see two things:
1 - Operators overload
2 - Immutable Borrowing
You can see we create a struct MyType and implement
both Add and Sub traits using the form &MyType instead of MyType,
this tell that our method Add|Sub does not will own the values, but will only access their references.
this is helpfull to us because we need to use variables of MyType to do in operations many times.
In case we dont need that, we just transfer the ownership to Add|Sub traits
and they will clean the values after the end of their scopes.
*/

use std::ops::{Add, Sub};

struct MyType {
    value: i32,
}

impl MyType {
    fn new(value: i32) -> MyType {
        MyType { value: value }
    }
}

impl Add for &MyType {
    type Output = MyType; // MyType here also can be Self (in case of no borrowing the self)

    fn add(self, other: &MyType) -> MyType {
        MyType { value: self.value + other.value }
    }
}

impl Sub for &MyType {
    type Output = MyType; 

    fn sub(self, other: &MyType) -> MyType {
        MyType { value: self.value - other.value }
    }
}

fn main() {
    let a1 = MyType::new(1);
    let a2 = MyType::new(2);
    let suma = &a1 + &a2;
    let suba = &a1 - &a2;

    println!("Sum of {} + {} = {}", a1.value, a2.value, suma.value);
    println!("Sub of {} + {} = {}", a1.value, a2.value, suba.value);
}