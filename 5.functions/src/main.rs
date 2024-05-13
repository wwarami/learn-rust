use std::io;

fn main() {
    let mut name = String::new();

    println!("[*] Please enter your name: ");
    io::stdin()
        .read_line(&mut name)
        .expect("[!] Failed to read line.");

    let parsed_name = parse_name(&name);
    println!("[*] You are {parsed_name}.")
}

fn parse_name(name: &str) -> &str {
    // the last expression is returned
    name.trim()
    // return name.trim();
}
