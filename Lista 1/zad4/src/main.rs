/*
Jaka ilość zgadnięć gwarantuje wygraną?
Jeśli gracz będzie zgadywał liczby losowo to liczbą zganięć może być nawet 100.
Jeśli jednak gracz będzie wybierał połowę przedziału, np 100 -> less, 75 -> greater, 87, itd. to maksymalna ilość zgadnięć to 7.
*/

use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    let secret_number = ::rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {println!("You win!"); break;},
        }
    }
}

