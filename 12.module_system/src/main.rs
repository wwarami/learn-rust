use module_system::send_updates;


fn main() {
    let username: String = String::from("root");
    let password: String = String::from("123456");
    let data: String = String::from("Hi server.");

    send_updates(username, password, data);
}