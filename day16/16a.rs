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
	let mut res = to_value(&b[..3]);
	let t = to_value(&b[3..6]);
	if t != 4 {
		if b[6] == 0 {
			let len = to_value(&b[7..22]) as usize;
			let mut p = 22;
			let end = p + len;
			while p < end {
				let r = sum_up(&b[p..]);
				res += r.1;
				p += r.0;
			}
			return (p, res);
		} else {
			let count = to_value(&b[7..18]) as usize;
			let mut p = 18;
			for _sub in 0..count {
				let r = sum_up(&b[p..]);
				res += r.1;
				p += r.0;
			}
			return (p, res);
		}
	} else {
		let mut p = 6;
		loop {
			let last = b[p] == 0;
			p += 5;
			if last {
				break;
			}
		}
		return (p, res);
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
