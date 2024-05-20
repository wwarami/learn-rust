fn main() {
    // ip example
    let v4 = IPAddress {
        kind: IPAddrKind::V4,
        address: String::from("0.0.0.0")
    };

    let v6 = IPAddress {
        kind: IPAddrKind::V6,
        address: String::from("::1")
    };

    // shapes exaple
    let rectangle = Shape::Rectangle(10, 4);
    let circle = Shape::Circle(8);

    // Option & Some enums :: std library
    let some_number = Some(6);
    let absent_number: Option<i32> = None;


    // match
    let user =  User::Support;
    println!("{}", check_user(Some(user)));
    
}

fn check_user(user: Option<User>) -> String {
    match user {
        Some(User::Admin) => String::from("You are admin."),
        Some(User::Support) => String::from("You are a support"),
        Some(User::Client) => String::from("You are an end user."),
        _ =>String::from("what de hell man!")
    }
}

enum User {
    Admin,
    Support,
    Client
}

enum IPAddrKind {
    V4,
    V6
}
struct IPAddress {
    kind: IPAddrKind,
    address: String
}

enum Shape {
    Rectangle(u32, u32),
    Circle(u32)
}

impl Shape {
    fn info(&self) {
        // ...
    }
}