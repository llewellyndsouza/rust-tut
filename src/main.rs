use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  // println!("Hello, world!");
  let secret_number = rand::thread_rng().gen_range(1..=100);
  println!("The secret number is {}", secret_number);
  println!("Input your guess");

  let mut guess = String::new();

  io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

  let guess: i16 = guess
    .trim()
    .parse()
    .expect("Please enter a number, next time");

  println!("You guessed {}", guess);

  match guess.cmp(&secret_number) {
    Ordering::Less => println!("Less"),
    Ordering::Greater => println!("More"),
    Ordering::Equal => println!("Equal"),
  }
}
