use rand::Rng;
// use std::io::{stdin,stdout,Write};
use std::process;
fn main() {
    loop {
        let mut input: String;
        loop {
            println!("this is a number guessing game, type \"exit\" to exit the game or \"play\" to start the game");
            input = get_input();
            match input.as_str() {
                "exit" => process::exit(1),
                "play" => break,
                _ => println!("no input"),
            }
            clear();
        }
        clear();
        let rand_num = get_random_number();
        let mut attempts_counter = 0;
        loop {
            println!("type a number in range from 0 to 100");
            input = get_input();
            if !input.parse::<i32>().is_ok() {
                clear();
                println!("{input} is wrong input");
                continue;
            }
            let input: i32 = input.parse::<i32>().unwrap();
            if input > 100 {
                clear();
                println!("{input} is grater then 100");
                continue;
            }
            if input < 0 {
                clear();
                println!("{input} is less then 0");
                continue;
            }
            attempts_counter += 1;
            if input > rand_num {
                clear();
                println!("{input} is grater then the random number");
                continue;
            }
            if input < rand_num {
                clear();
                println!("{input} is less then the random number");
                continue;
            }
            if input == rand_num {
                clear();
                println!("you guessed the number, it was {rand_num}, you used {attempts_counter} attempts to guess the number");
                break;
            }
        }
    }
}
fn clear() {
    print!("{esc}c", esc = 27 as char);
}

fn get_random_number() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=100)
}
fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error: unsupported input");
    input = input.trim().to_string();
    input
}
