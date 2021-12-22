use std::cmp::min;

fn main() {
	let mut p = [1, 7];
	let mut s = [0, 0];
	let size = 10;
	let mut die = 0;
	let die_max = 100;
	let mut count = 0;
	let mut turn = 0;
	loop {
		count += 3;
		p[turn] = p[turn] + die + 1;
		die = (die + 1) % die_max;
		p[turn] = p[turn] + die + 1;
		die = (die + 1) % die_max;
		p[turn] = p[turn] + die + 1;
		die = (die + 1) % die_max;
		p[turn] %= size;
		s[turn] += p[turn] + 1;
		if s[turn] >= 1000 {
			break;
		}
		turn = (turn + 1) % 2;
	}
	println!("{} {} {}", count, s[0], s[1]);
	println!("{}", count * min(s[0], s[1]));
}
