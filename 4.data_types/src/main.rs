fn main() {
    // converting data types
    let a = "1";
    let b: i32 = a.parse()
        .expect("Not a number!");
    println!("B is {b}");

    // scalar types: 
    // A scalar type represents a single value
    
    // integers
    // signed numbers: can be negative
    // unsigned numbers: only positive
    let age: u8 = 1; // unsigned
    let c: i8 = -2; // signed

    // floating-point
    // all are signed
    let d: f32 = 1.001;

    // Booleans
    let is_running: bool = false; 
    
    // characters
    // only a one character
    const SIGN: char = 'a';
    const LOVE: char = '💖';


    // compound types
    // Compound types can group multiple values into one type.
    
    // tuples > can have different types
    const ADMINS: (u8, u8) = (12, 123);
    let users = ("bro", "dude");

    let (admin1, admin2) = users;
    println!("Admin1  is {admin1}, Admin2 is {admin2}");
    let admin_1_id = ADMINS.0; // indexes
    println!("Admin1 id is {admin_1_id}");

    // arrays > fixed length and same type
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    const HEARTS: [char; 2] = ['💖', '💖'];
    let a = [3; 2]; // [3, 3]
    let b = a[1]; // indexes

    println!("Hello, world!");
}
