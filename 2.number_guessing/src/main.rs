use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // program start
    println!("[!] Welcome to number guessing game!");
    let secret_number = rand::thread_rng().gen_range(1..=10);

    // main loop
    loop {
        println!("[*] Please enter your guess(between 1 to 10): ");
        // get user input
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("[!] Failed to read line.");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("[!] Invalid input: {guess}");
                continue;
            }   
            };
        
        // check the guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("    [*] too small!"),
            Ordering::Greater => println!("    [*]too big!"),
            Ordering::Equal => {
                println!("    [*] Bingo!");
                break;
                }
        }
    }
    // on quit or correct guess
    println!("[!] Hope you had a good game!");
}
