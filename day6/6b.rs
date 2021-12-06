use std::io;

fn main() {
	let mut line = String::new();
	io::stdin().read_line(&mut line).unwrap();
	let mut stats = [0u128; 9];
	for n in line.trim().split(",").map(|x| x.parse::<usize>().unwrap()) {
		stats[n] += 1;
	}
	for _day in 0..256 {
		let mut new_stats = [0u128; 9];
		new_stats[6] += stats[0];
		new_stats[8] += stats[0]; 
		for idx in 0..8 {
			new_stats[idx] += stats[idx + 1];
		}
		stats = new_stats;
	}
	println!("{}", stats.iter().sum::<u128>());
}
