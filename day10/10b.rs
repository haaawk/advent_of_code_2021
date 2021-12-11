use std::io::{self, BufRead};

fn main() {
	let mut res = Vec::<u64>::new();
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	for line in lines {
		let mut stack = Vec::<u8>::new();
		let mut corrupted = false;
		for c in line.as_bytes() {
			let elem = *c;
			match elem {
				b'(' => stack.push(b')'),
				b'[' => stack.push(b']'),
				b'{' => stack.push(b'}'),
				b'<' => stack.push(b'>'),
				b')' | b']' | b'}' | b'>' => {
					if let Some(top) = stack.pop() {
						if top != elem {
							corrupted = true;
							break;
						}
					} else {
						corrupted = true;
						break;
					}
				},
				_ => panic!("Unexpected character {}", elem),
			};
		}
		if !corrupted {
			let mut score = 0u64;
			while let Some(top) = stack.pop() {
				score *= 5;
				score += match top {
					b')' => 1,
					b']' => 2,
					b'}' => 3,
					b'>' => 4,
					_ => panic!("Unexpected character {}", top),
				};
			}
			if score > 0 {
				res.push(score);
			}
		}
		
	}
	res.sort();
	println!("{}", res[res.len() / 2]);
}
