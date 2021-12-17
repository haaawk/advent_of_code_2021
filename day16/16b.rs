use std::io;
use std::collections::HashMap;

fn to_value(b: &[u8]) -> u64 {
	let mut mul = 1;
	let mut res = 0u64;
	for idx in 0..b.len() {
		res += mul * (b[b.len() - idx - 1] as u64);
		mul *= 2;
	}
	res
}

fn sum_up(b: &[u8]) -> (usize, u64) {
	let t = to_value(&b[3..6]);
	if t != 4 {
		let mut params = Vec::<u64>::new();
		let mut p;
		if b[6] == 0 {
			let len = to_value(&b[7..22]) as usize;
			p = 22;
			let end = p + len;
			while p < end {
				let r = sum_up(&b[p..]);
				params.push(r.1);
				p += r.0;
			}
		} else {
			let count = to_value(&b[7..18]) as usize;
			p = 18;
			for _sub in 0..count {
				let r = sum_up(&b[p..]);
				params.push(r.1);
				p += r.0;
			}
		}
		let res = match t {
			0 => params.iter().sum(),
			1 => {
				let mut r = 1;
				for e in params {
					r *= e;
				}
				r
			},
			2 => *params.iter().min().unwrap(),
			3 => *params.iter().max().unwrap(),
			5 => if params[0] > params[1] { 1 } else { 0 },
			6 => if params[0] < params[1] { 1 } else { 0 },
			7 => if params[0] == params[1] { 1 } else { 0 },
			_ => panic!("Unexpected type {}", t),
		};
		return (p, res);
	} else {
		let mut p = 6;
		let mut bits = Vec::<u8>::new();
		loop {
			let last = b[p] == 0;
			for idx in 1..5 {
				bits.push(b[p + idx]);
			}
			p += 5;
			if last {
				break;
			}
		}
		return (p, to_value(&bits));
	}
}

fn main() {
	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	let mut bits = HashMap::<u8, [u8; 4]>::new();
	bits.insert(b'0', [0, 0, 0, 0]);
	bits.insert(b'1', [0, 0, 0, 1]);
	bits.insert(b'2', [0, 0, 1, 0]);
	bits.insert(b'3', [0, 0, 1, 1]);
	bits.insert(b'4', [0, 1, 0, 0]);
	bits.insert(b'5', [0, 1, 0, 1]);
	bits.insert(b'6', [0, 1, 1, 0]);
	bits.insert(b'7', [0, 1, 1, 1]);
	bits.insert(b'8', [1, 0, 0, 0]);
	bits.insert(b'9', [1, 0, 0, 1]);
	bits.insert(b'A', [1, 0, 1, 0]);
	bits.insert(b'B', [1, 0, 1, 1]);
	bits.insert(b'C', [1, 1, 0, 0]);
	bits.insert(b'D', [1, 1, 0, 1]);
	bits.insert(b'E', [1, 1, 1, 0]);
	bits.insert(b'F', [1, 1, 1, 1]);
	let mut b = Vec::<u8>::with_capacity(line.len() * 4);
	for c in line.trim().as_bytes() {
		let bb = bits.get(c).unwrap();
		for p in 0..4 {
			b.push(bb[p]);
		}
	}
	println!("{}", sum_up(&b).1);
}
