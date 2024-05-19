fn main() {
    let username = String::from("username");
    let password = String::from("1");
    let user: User = User::create(username, password);
    user.show();
}

#[derive(Debug)]
struct User {
    username: String,
    password: String
}

impl User {
    fn create(username: String, password: String) -> Self {
        let mut secure_password: String = String::new();

        for _i in password.into_bytes().iter() {
            secure_password.push('*');
        }

        let user: User = Self {username, password: secure_password};
        user
    }

    fn show(&self) {
        println!("User(username=@{}, password={})", 
        self.username, 
        self.password);
    }

}
