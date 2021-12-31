// We can use this to implement the trait Debug, that allows us to print
// the entire struct using the format {:?} or {:#?} notation.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

// This add the method area() to our struct, everything inside the impl Rectangle {} block
// will be associated to the type.
// note that the first parameter are aways self, that represent our instance.
// note that self has &, that tells to area borrow the ownership of the instance.
// note, you can have many impl {} blocks that you want.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // We also can use the same name as a struct field in a method, this dont conflict.
    // we can see this as a getter.
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // this is an associated function, is a function that does not neet the instance (self)
    // to be executed, the syntax to call is using :: like Rectangle::square(10);
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main () {
    let scale = 2;
    let rect1 = Rectangle {
        // width: dbg!(30 * scale), this is usefull when you want specific debug info about one part of the code.
        width: 30,
        height: 50
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // A simple use of .width()
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
