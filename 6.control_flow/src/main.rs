use std::io;

fn main() {
    // if base usage
    let name: &str = "bro";
    let is_man: bool = true;

    println!("[*] Hello {name}");
    if is_man {
        println!("[*] You are a man");
    } else if name == "Bro" {
        println!("[*] But your name is for a bro.");
    } else {
        println!("[*] You are a woman")
    }
    // if in variables
    const ID: u8 = 1;
    let is_bro: bool = if ID == 1 { true } else {false};

    if is_bro {
        println!("[*] You are a bro.");
    } else {
        println!("[*] You are not a bro.")
    }

    // loops
    // loop basic usage
    const PASSWORD: &str = "admin";
    println!("[*] Please enter the password: ");
    // loops can have names or not!
    'main_loop: loop {
        let mut text: String = String::new();
        io::stdin()
            .read_line(&mut text)
            .expect("[!] Failed to read line.");

        if text.trim() == PASSWORD {
            println!("[*] Correct password.");
            break 'main_loop;
        } else if text.trim() == "q"{
            break 'main_loop;
        } else {
            println!("[*] Not correct password!\nTry again(or q): ");
        }
    }

    // while loops
    let mut counter: u8 = 10;

    while counter != 0 {
        println!("[*] {counter}");
        counter -= 1;
    }
    println!("[*] BOOOOOOM!");

    // for loops
    let users: [&str; 3] = ["bro", "dude", "man"];
    
    for user in users {
        println!("[*] {user}");
    }
    // ranges
    for i in 0..=2 {
        let counter = i + 1;
        let user = users[i];
        println!("[{counter}] {user}")
    }

}
