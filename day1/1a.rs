use std::io::{self, BufRead};

fn main() {
	let mut res = 0;
	let mut prev = None;
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap().parse::<i32>().unwrap());
	for line in lines {
		if line > prev.unwrap_or(line) {
			res += 1;
		}
		prev = Some(line);
	}
	println!("{}", res);
}
