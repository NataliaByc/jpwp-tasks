use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        /*let guess: u32 = match guess.trim().parse() {
            //tutaj musimy uzupełnić matcha tak by w przypadku wartości Ok(liczba) przypisał tą liczbę do u32,
            //a w przypadku Err() przeszedł do kolejnej iteracji całej pętli, tj. zrobił continue.
        };*/

        println!("You guessed: {guess}");

        /*match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            //uzupełnij pozostałe matche, tak by w przypadku gdy gracz zgadnie liczbę, program się przerwał za pomocą break
            //a gdy liczba jest większa, to wypisuje "Too big!"
        }*/
    }
}
