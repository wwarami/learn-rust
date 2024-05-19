use std::slice::RChunksExactMut;

fn main() {
    // struct >> python dictionary

    // creating structs
    let mut user1 = create_user("bro".to_owned(), 
        "bro1234".to_owned(),
            18);

    user1.age = 17;

    // creating instances from other instances
    let updated_user1 = User {
        name: String::from("dude"),
        password: String::from("dude1234"),
        ..user1 // ! user1 values are moved to user2
    };

    // tuple structs
    let user_position = Position(0, 0);

    // structs example
    let rectangle: Rectangle = Rectangle {width: 4, 
        height: 3
    };

    let area: u32 = area(&rectangle);

    println!("Area of {:#?} is equal to {}", rectangle, area);
}


struct User {
    name: String,
    password: String,
    age: u8,
    is_active: bool
}

struct Position(i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

fn create_user(name: String,
        password: String,
        age: u8) -> User {
            User {
                name: name,
                password: password,
                age: age,
                is_active: true
            }
        }

fn area (rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
