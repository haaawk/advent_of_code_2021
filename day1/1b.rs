use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() {
	let mut res = 0;
	let mut sum = 0;
	let mut window = VecDeque::new();
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap().parse::<i32>().unwrap());
	for line in lines {
		window.push_back(line);
		sum += line;
		if window.len() == 4 {
			if sum - window.back().unwrap() < sum - window.front().unwrap() {
				res += 1;
			}
			window.pop_front();
		}
	}
	println!("{}", res);
}
