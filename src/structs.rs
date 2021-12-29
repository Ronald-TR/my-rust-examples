struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    /* Note that has no return word here, because Rust also uses the "tail"
    return, so here, according to the tail patern
    the last line will be our return.

    Another tip: variables with the same name as the fields can be passed in a short way.
     */
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    /* Note that all the instance must be mutable.
     Rust does not allows us to mark only certain fields as mutable
     */
    let mut user1 = User{
        username: String::from("Jhon Doe"),
        email: String::from("jhon@doe.com"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("jhondoe@doe.com");
    
    // We alseo can use one old instance to build another using the struct update pattern
    // with the ..oldinstance form, this tell Rust to fill the remain fields using the old instance.
    let user2 = User {
        email: String::from("jhon@doe.com"),
        ..user1
    }; // IMPORTANT: after that we can not use user1 anymore, because their String values has passed to user2.
    // REMEMBER: primitive types implement the Copy trait (like active: bool and sign_in_count: u64 fields), if we dont pass
    // String fields from user1 to user2, we still can use user1 after the user2 creation.

    // Example of struct tuples: This short form can be used to create new types without naming fields, works like tuples but as unique type.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // We can access the elements of "struct tuples" by index using the dot notation, ex:
    println!("{} {}", black.1, origin.1);
}