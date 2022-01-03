// This is a simple enum definition, with no value associated.
enum IpAddrKind {
    V4,
    V6
}
// This is a enum with associated data, we can define the enum and store some data.
enum IpAddrKindWithAssociation {
    V4(u8, u8, u8, u8),
    V6(String)
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// A enum with a variety of kinds
enum Message {
    Quit, // just a simple enum with no data associated with it at all.
    Move { x: i32, y: i32 }, // with named fields, like a struct does.
    Write(String), // includes a single String
    ChangeColor(i32, i32, i32), // includes three i32 values.
}
// Enum with generics, like Option<i32>::Some(42)
enum Option<T> {
    None,
    Some(T),
}

fn route(ip_kind: IpAddrKind) {

}

fn main() {
   let four = IpAddrKind::V4; // we define a enum with double colon ::
   let six = IpAddrKind::V6;

   // This is the form using enums and structs to store the associated enum data
   let home = IpAddr {
       kind: four,
       address: String::from("127.0.0.1"),
   };

   let loopback = IpAddr {
       kind: six,
       address: String::from("::1"),
   };

   // But we can solve it in a simpler way using associated types with enums.
   let _home = IpAddrKindWithAssociation::V4(127, 0, 0, 1);
   let _loopback = IpAddrKindWithAssociation::V6("::1");

}
