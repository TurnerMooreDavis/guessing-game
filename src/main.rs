use std::io;
use std::cmp::Ordering;

fn main() {
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
		println!("{}", num);
		println!("Please input your guess.");
		match guess.cmp(&num) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
