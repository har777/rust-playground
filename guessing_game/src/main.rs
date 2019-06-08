use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Welcome to the guessing game!");

  let min_rand_value = 1;
  let max_rand_value = 20;
  println!("Your guess should be a value between {} and {}", min_rand_value, max_rand_value);

  // Generates a random number between min_rand_value and min_rand_value
  let secret_number = rand::thread_rng().gen_range(min_rand_value, max_rand_value + 1);

  let mut guesses_remaining = 5;

  loop {
    if guesses_remaining <= 0 {
      println!("You have no guesses remaining. Game Over :(");
      println!("Correct guess was: {}", secret_number);
      break;
    }

    println!("You have {} guesses remaining.", guesses_remaining);
    println!("Please input your guess: ");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    // Parsed to u32
    let guess: u32 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please enter a valid number");
        continue;
      }
    };

    // Decrement guesses remaining if valid guess
    guesses_remaining = guesses_remaining - 1;

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("You guessed: {}, your guess is too small. Try again.", guess),
      Ordering::Greater => println!("You guessed: {}, your guess is too large. Try again.", guess),
      Ordering::Equal => {
        println!("You guess: {} is correct. Hurrah you win!!!", guess);
        break;
      }
    }
  }
}
