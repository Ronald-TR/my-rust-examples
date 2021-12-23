/* This trait shows how to use pattern match and how to receive values from it.
First, we read the argument passed by the command line, returning a str for the respective match.
After, we try to convert it to i32, to use it in another match
that prints on screen instead of return something.

Pattern match evaulates all the structure of the object instead of just their values.
This is the reason we have a String == str as true in a if statement,
but String != str into a pattern match evaulation (String is a structure and str is a primitive type).
To use String to compare with str into a match expression, we need to access the String reference.

*/
use std::env;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let value = args[1].as_ref(); // Or args[1].as_str()

    let resp = match value {
        "22" | "11" => "Patos!",
        "42" => "Resposta para tudo mais",
        _ => "Resto",
    };
    println!("{}", resp);

    if let Ok(x) = &args[1].parse::<i32>() {
        match x {
            22 => println!("{}", x),
            _ => println!("Resto")
        }
    }
}
