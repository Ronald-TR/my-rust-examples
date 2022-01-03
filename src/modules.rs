fn main() {
    grettings::hello();
    foo::bar::hello();
    phrases::greet();
    phrases::greetings::hello();
}

mod grettings {
    // By default, everything inside a module is private
    pub fn hello() { // So functions has to be public to access from outside
        println!("Hello, world!");
    }
}

// modules can also be nested
mod foo {
    pub mod bar {
        pub fn hello() {
            println!("Hello, nested modules!");
        }
    }
}
fn root_hello() {
    println!("Hello, root module!");
}
// REMEMBER: The "self" keyword is used to refer the same module,
// while the "super" keyword is used to refer parent module. Also, 
// the "super" keyword can be used to access root functions from inside a module.
mod phrases {
    pub fn greet() {
        // private functions can be called from the same
        // module or from a child module.
        private_hello();
        // note that "root_hello" is in the root scope, that is the
        // parent of our "phrases" module. 
        super::root_hello();
    }

    fn private_hello() {
        println!("Hello, private world!");
    }

    pub mod greetings {
        pub fn hello() {
            // This super is necessary when calling private
            // functions of the parent module.
            super::private_hello();
        }
    }
}


// How to test with modules:

fn greet() -> String {
    "Hello, world!".to_string()
}

#[cfg(test)] // Only compiles when running tests
mod tests {
    use super::greet; // Import root greet function

    #[test]
    fn test_greet() {
        assert_eq!("Hello, world!", greet());
    }
}