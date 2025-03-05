use std::io;
use rand::seq::SliceRandom;

fn main() {
    let words = vec!["maslo", "wafel", "iskra", "pazur", "obraz"];
    let secret_word = words.choose(&mut rand::thread_rng()).unwrap();

    println!("Welcome to Wordle!");
    println!("Guess the 5-letter word. You have 6 attempts.");

    let mut attempts = 0;

    loop {
        if attempts >= 6 {
            println!("You lose! The word was: {}", secret_word);
            break;
        }

        println!("Attempt {}/6", attempts + 1);
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = guess.trim();

        if guess.len() != 5 {
            println!("Your guess must be 5 letters long!");
            continue;
        }

        let mut result = String::new();
        for (i, c) in guess.chars().enumerate() {
            if secret_word.chars().nth(i) == Some(c) {
                result.push('🟩'); // Litera na swoim miejscu
            } else if secret_word.contains(c) {
                result.push('🟨'); // Litera jest w słowie, ale na złym miejscu
            } else {
                result.push('⬛'); // Litera nie występuje w słowie
            }
        }

        println!("{}", result);

        if guess == *secret_word {
            println!("You win! The word was: {}", secret_word);
            break;
        }

        attempts += 1;
    }
}