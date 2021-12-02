use std::io::{self, BufRead};

fn main() {
	let mut x:i64 = 0;
	let mut y:i64 = 0;
	let mut a:i64 = 0;
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	for line in lines {
		let mut split = line.split(" ");
		let command = split.next().unwrap();
		let val = split.next().unwrap().parse::<i64>().unwrap();
		match command {
			"forward" => {
				y += a * val;
				x += val;
			}
			"up" => { a -= val; }
			"down" => { a += val; }
			_ => { panic!("{}", command); }
		}
	}
	println!("{}", x * y);
}
