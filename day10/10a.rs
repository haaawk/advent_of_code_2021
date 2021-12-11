use std::io::{self, BufRead};

fn main() {
	let mut res:u64 = 0;
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	for line in lines {
		let mut stack = Vec::<u8>::new();
		for c in line.as_bytes() {
			let elem = *c;
			match elem {
				b'{' => stack.push(elem),
				b'(' => stack.push(elem),
				b'[' => stack.push(elem),
				b'<' => stack.push(elem),
				b'}' => {
					if let Some(top) = stack.pop() {
						if top != b'{' {
							res += 1197;
							break;
						}
					} else {
						res += 1197;
						break;
					}
				},
				b')' => {
					if let Some(top) = stack.pop() {
						if top != b'(' {
							res += 3;
							break;
						}
					} else {
						res += 3;
						break;
					}
				},
				b']' => {
					if let Some(top) = stack.pop() {
						if top != b'[' {
							res += 57;
							break;
						}
					} else {
						res += 57;
						break;
					}
				},
				b'>' => {
					if let Some(top) = stack.pop() {
						if top != b'<' {
							res += 25137;
							break;
						}
					} else {
						res += 25137;
						break;
					}
				},
				_ => panic!("Unexpected character {}", elem),
			};
		}
		
	}
	println!("{}", res);
}
