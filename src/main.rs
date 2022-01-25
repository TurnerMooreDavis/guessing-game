use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
	let num = rand::thread_rng().gen_range(1..100); 
	println!("Guess the number!");
  println!("Please input your guess.");

	loop {
		let mut guess = String::new();
		io::stdin().read_line(&mut guess)
				.expect("Failed to read line");
		let guess: i32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => {
				println!("please enter a number between 1 and 100");
				continue;
			},
		};
		println!("You guessed: {}", guess);
		match guess.cmp(&num) {
			Ordering::Less => {println!("Too small!"); println!("\n")},
			Ordering::Greater => {println!("Too big!"); println!("\n")},
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
		println!("Please input your guess.");
	}
}
