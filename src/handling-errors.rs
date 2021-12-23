use std::fs::File;
use std::path::Path;

fn read_file(path: &Path) -> Result<String, io::Error> {
    let mut f = try!(File::open(path));
    let mut rv = String::new();
    try!(f.read_to_string(&mut rv));
    Ok(rv)
}

fn main() {
    let path = Path("this-file-does-not-exist.txt");
    let result = read_file(path);
    println!("{}", result);
}