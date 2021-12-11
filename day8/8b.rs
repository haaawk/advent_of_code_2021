use std::io::{self, BufRead};
use std::collections::HashMap;


fn main() {
	let mut digits = HashMap::<[bool; 7], u8>::new();
	digits.insert([true, true, true, false, true, true, true], 0);
	digits.insert([false, false, true, false, false, true, false], 1);
	digits.insert([true, false, true, true, true, false, true], 2);
	digits.insert([true, false, true, true, false, true, true], 3);
	digits.insert([false, true, true, true, false, true, false], 4);
	digits.insert([true, true, false, true, false, true, true], 5);
	digits.insert([true, true, false, true, true, true, true], 6);
	digits.insert([true, false, true, false, false, true, false], 7);
	digits.insert([true, true, true, true, true, true, true], 8);
	digits.insert([true, true, true, true, false, true, true], 9);
	let mut res:u64 = 0;
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	for line in lines {
		let mut split = line.split("|");
		let mut count = [0; 7];
		let mut one = [false; 7];
		let mut four = [false; 7];
		let mut seven = [false; 7];

		for d in split.next().unwrap().trim().split(" ") {
			for letter in d.as_bytes() {
				count[(letter - b'a') as usize] += 1;	
			}
			match d.len() {
				2 => {
					for letter in d.as_bytes() {
						one[(letter - b'a') as usize] = true;
					}
				},
				4 => {
					for letter in d.as_bytes() {
						four[(letter - b'a') as usize] = true;
					}
				},
				3 => {
					for letter in d.as_bytes() {
						seven[(letter - b'a') as usize] = true;
					}
				},
				_ => {},
			};
		}
		let mut mapping = [10usize; 7];
		let mut a_idx = 0;
		for idx in 0..7 {
			if seven[idx] && !one[idx] {
				mapping[idx] = 0;
				a_idx = idx;
				break;
			}
		}
		for idx in 0..7 {
			match count[idx] {
				4 => mapping[idx] = 4,
				6 => mapping[idx] = 1,
				7 => mapping[idx] = if four[idx] { 3 } else { 6 },
				8 => {
					if a_idx != idx {
						mapping[idx] = 2;
					}
				},
				9 => mapping[idx] = 5,
				_ => panic!("Unexpected count {}", count[idx]),
			}
		}
		let mut base = 1000;
		for d in split.next().unwrap().trim().split(" ") {
			let mut segments = [false; 7];
			for digit in d.as_bytes() {
				segments[mapping[(digit - b'a') as usize]] = true;
			}
			res += base * (*digits.get(&segments).unwrap() as u64);
			base /= 10;
		}
		
	}
	println!("{}", res);
}
