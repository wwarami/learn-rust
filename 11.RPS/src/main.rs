/*
Project started at 05.30.24 at 20:55 (Tehran).
This will be a simple rock paper scissor game.
 
Project ended at 05.30.24 at 23:10 (Tehran).
*/
use rand::Rng;
use std::io;


fn main() {
    // stuff may seem stupid, I'm trying to use any thing I learned and that's all I know
    
    // declare game constants and variables
    const START: &str = "\x1B[0;34m\x1B[1m---- RPC GAME IN RUST ----\x1B[0;0m\x1B[0m";
    const ENTER_INPUT: &str = "\x1B[1m[*] Please enter your choice(r, p or s): \x1B[0m";
    const NEW_GAME_STATED: &str = "[*] New game started.";
    const INVALID_INPUT: &str = "\x1B[0;31m[!] Please enter a valid character.\x1B[0;0m";
    const ERROR: &str = "\x1B[0;31m[!] Sorry, Something went wrong.\x1B[0;0m";
    const KEEP_PLAYING: &str = "\x1B[1m[*] Do you wanna keep playing?(y or n): \x1B[0m";

    let mut score: u32 = 0;

    const ROCK: GameChoice = GameChoice {
        char: 'r',
        master_char: 'p',
        slave_char: 's'
    };
    const PAPER: GameChoice = GameChoice {
        char: 'p',
        master_char: 's',
        slave_char: 'r'
    };
    const SCISSOR: GameChoice = GameChoice {
        char: 's',
        master_char: 'r',
        slave_char: 'p'
    };

    let choices: [GameChoice; 3]  = [ROCK, PAPER, SCISSOR];

    // program start
    println!("    {}    ", START);

    // main program loop
    loop {
        println!("{}", NEW_GAME_STATED);
        println!("[*] Your score: {}", score);
        let game_choice: GameChoice = get_game_choice(&choices);

        // get user input and parse it
        println!("{}", ENTER_INPUT);
        let mut user_input: String = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect(ERROR);

        let user_char: char = match user_input.trim().parse() {
                Ok(c) => c,
                Err(_) => {
                    println!("{}", INVALID_INPUT);
                    continue;
                }
            };
    

        // check for the provided char   
        let (result, text) = check_char(game_choice, user_char);
        if result {
            score += 10;
        }
        println!("{}", text);
        println!("[*] Your score: {}", score);

        // check to keep playing
        println!("{}", KEEP_PLAYING);
        let mut keep_playing: String = String::new();
        match io::stdin().read_line(&mut keep_playing) {
            Err(_) => continue,
            Ok(_) => ()
        }

        if keep_playing.trim().to_lowercase() == "n" {
            break;
        }
    }

    // game ends
    println!("\x1B[0;34m\x1B[1m[!] Hope you had a good game. Final score: {}\x1B[0;0m\x1B[0m", score)
}


fn check_char(game_choice: GameChoice, user_char: char) -> (bool, String){
    match user_char {
        c if c == game_choice.char => (false, String::from("[*] Tie!")),
        c if c == game_choice.master_char => (true, String::from("\x1B[0;32m[*] You won!\x1B[0;0m")),
        c if c == game_choice.slave_char => (false, String::from("[*] You lost!")),
        _ => (false, String::from("\x1B[0;31m[!] Please enter a valid character.\x1B[0;0m"))
    }
}


fn get_game_choice(choices: &[GameChoice; 3]) -> GameChoice{
    let index: usize = rand::thread_rng().gen_range(0..=2);
    choices[index]
}


#[derive(Debug, Clone, Copy)]
struct GameChoice {
    char: char,
    master_char: char,
    slave_char: char
}
