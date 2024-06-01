use std::io;
use rand::prelude::*;

fn main() {
    println!("Welcome to this number guessing game, your job is to guess a number between 1-100 (enter -1 if you want to quit)!");
    let mut rng = rand::thread_rng();
    let mut random_num = rng.gen_range(1..100);
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let number:i8 = input.trim().parse().unwrap();

        if number == -1 {
            break;
        }
        if number < random_num {
            println!("Your nubmer is too small!");
        }
        else if number > random_num {
            println!("Your nubmer is too big!");
        }
        else{
            println!("Correct, the number is {number}");
            println!("Generating a new number");
            random_num = rng.gen_range(1..100);
        }
    }
}
