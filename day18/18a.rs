use std::io::{self, BufRead};

fn is_digit(x: u8) -> bool {
	x >= b'0' && x <= b'9'
}

fn reduce(mut x: String) -> String {
	loop {
		let mut depth = 1u64;
		let mut start = 0;
		for idx in 1..x.len() - 1 {
			match x.as_bytes()[idx] {
				b'[' => depth += 1,
				b']' => depth -= 1,
				_ => depth = depth,
			}
			if depth == 5 {
				start = idx;
				break;	
			}
		}
		if start != 0 {
			let mut a = 100000;
			let mut end = start + 1;
			let mut div = 100000;
			while x.as_bytes()[end] != b']' {
				if x.as_bytes()[end] == b',' {
					a = x[start + 1..end].parse::<u64>().unwrap();
					div = end;
				}
				end += 1;
			}
			let b = x[div + 1..end].parse::<u64>().unwrap();
			let mut x_prefix = x[..start].to_string();
			let mut x_suffix = x[end + 1..].to_string();
			for idx3 in 1..x_prefix.len() - 1 {
				let idx = x_prefix.len() - idx3;
				if is_digit(x_prefix.as_bytes()[idx]) {
					let mut idx2 = idx - 1;
					while is_digit(x_prefix.as_bytes()[idx2]) {
						idx2 -= 1;
					}
					let val = x_prefix[idx2 + 1..idx + 1].parse::<u64>().unwrap() + a;
					x_prefix = x_prefix[..idx2 + 1].to_string() + &val.to_string() + &x_prefix[idx + 1..];
					break;
				}
			}
			for idx in 0..x_suffix.len() - 1 {
				if is_digit(x_suffix.as_bytes()[idx]) {
					let mut idx2 = idx + 1;
					while is_digit(x_suffix.as_bytes()[idx2]) {
						idx2 += 1;
					}
					let val = x_suffix[idx..idx2].parse::<u64>().unwrap() + b;
					x_suffix = x_suffix[..idx].to_string() + &val.to_string() + &x_suffix[idx2..];
					break;
				}
			}
			x = x_prefix + "0" + &x_suffix;
			continue;
		} else {
			let mut found = false;
			for idx in 2..x.len() - 1 {
				if is_digit(x.as_bytes()[idx - 1]) && is_digit(x.as_bytes()[idx]) {
					found = true;
					let val = (x.as_bytes()[idx - 1] - b'0') * 10 + x.as_bytes()[idx] - b'0';
					x = x[..idx-1].to_string() + "[" + &(val / 2).to_string() + "," + &((val + 1) / 2).to_string() + "]" + &x[idx + 1..];
					break;
				}
			}
			if found {
				continue;
			}
		}
		break;
	}
	x
}

fn score(x: &str) -> u64 {
	if x.len() == 1 {
		return x.parse::<u64>().unwrap();
	}
	let mut depth = 1u64;
	let mut div = 0;
	for idx in 1..x.len() - 1 {
		match x.as_bytes()[idx] {
			b'[' => depth += 1,
			b']' => depth -= 1,
			_ => depth = depth,
		}
		if depth == 1 && x.as_bytes()[idx] == b',' {
			div = idx;
			break;
		}
	}
	return 3 * score(&x[1..div]) + 2 * score(&x[(div + 1)..x.len() - 1]);
}

fn main() {
	let stdin = io::stdin();
	let mut first = true;
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	let mut res = String::new();
	for line in lines {
		if first {
			first = false;
			res = line;
		} else {
			res = "[".to_owned() + &res + "," + &line + "]";
			res = reduce(res);
		}
	}
	println!("{}", score(&res));
}
