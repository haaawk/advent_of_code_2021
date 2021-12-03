use std::io::{self, BufRead};

fn main() {
	let mut count = [0; 12];
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	for line in lines {
		for idx in 0..line.len() {
			count[idx] += if line.as_bytes()[idx] == b'0' { -1 } else { 1 };
		}
	}
	let mut gamma:u64 = 0;
	let mut epsilon:u64 = 0;
	let mut power = 1;
	for idx in 0..12 {
		gamma += power * (if count[11 - idx] > 0 { 1 } else { 0 });
		epsilon += power * (if count[11 - idx] > 0 { 0 } else { 1 });
		power *= 2;
	}
	println!("{}", gamma * epsilon);
}
