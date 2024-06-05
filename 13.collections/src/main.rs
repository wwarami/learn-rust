fn main() {
    // tuple -> fixed length
    let family: (&str, &str) = ("father", "sister");
    // array -> Same type & fixed length
    let users: [u8; 2] = [1, 1];


    // Vectors -> Same type
    let v: Vec<i32> = Vec::new();
    let v: Vec<&str> = vec!["bro", "dude"];

    // add to vector
    let mut users: Vec<&str> = vec![];
    users.push("bro");
    users.push("dude");
    users.push("kabra");

    // read from vector
    let user_1: &&str = &users[0];
    // safe
    let user_2: Option<&&str>= users.get(1);
    match user_2 {
        Some(user_2) =>  println!("Second user is {user_2}"),
        None => println!("No second user!")
    }


    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    for n in &mut numbers {
        *n += 1
    }

    // using multiple types in the vector
    let users: Vec<UserID> = vec![UserID::Int(123456), 
                        UserID::Str(String::from("123456"))];
}


enum UserID {
    Int(u32),
    Str(String)
}