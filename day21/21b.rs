use std::cmp::max;
use std::collections::VecDeque;

fn main() {
	let mut q = VecDeque::<([u8; 2], [u8; 2], usize, u128)>::new();
	q.push_back(([1, 7], [0, 0], 0, 1));
	let mut count = [0u128, 0u128];
	let die: [(u8, u128); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
	while let Some((p, s, turn, num)) = q.pop_front() {
		let new_turn = (turn + 1) % 2;
		for (v, c) in die {
			let mut new_p = p;
			new_p[turn] = (new_p[turn] + v) % 10;
			let mut new_s = s;
			new_s[turn] += new_p[turn] + 1;
			if new_s[turn] >= 21 {
				count[turn] += num * c;
			} else {
				q.push_back((new_p, new_s, new_turn, num * c));
			}
		}
		
	}
	println!("{}", max(count[0], count[1]));
}
