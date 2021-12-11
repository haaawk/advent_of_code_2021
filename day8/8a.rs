use std::io::{self, BufRead};

fn main() {
	let mut res:i64 = 0;
	let stdin = io::stdin();
	let lines = stdin.lock().lines().map(|l| l.unwrap());
	for line in lines {
		let mut split = line.split("|");
		split.next();
		let mut split2 = split.next().unwrap().trim().split(" ");
		let len1 = split2.next().unwrap().len();
		let len2 = split2.next().unwrap().len();
		let len3 = split2.next().unwrap().len();
		let len4 = split2.next().unwrap().len();
		res += if len1 == 2 || len1 == 3 || len1 == 4 || len1 == 7 { 1 } else { 0 };
		res += if len2 == 2 || len2 == 3 || len2 == 4 || len2 == 7 { 1 } else { 0 };
		res += if len3 == 2 || len3 == 3 || len3 == 4 || len3 == 7 { 1 } else { 0 };
		res += if len4 == 2 || len4 == 3 || len4 == 4 || len4 == 7 { 1 } else { 0 };
	}
	println!("{}", res);
}
