use std::collections::HashMap;

fn main() {
	let input = include_str!("input.txt").lines().collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[&str]) -> u64 {
	let mut mem = HashMap::new();
	let mut mask = Mask::new();
	for &line in input.iter() {
		match &line[..4] {
			"mask" => {
				mask = Mask::from(&line[7..]);
			}
			"mem[" => {
				let idx_end = line.char_indices().find(|(_, c)| c == &']').unwrap().0;
				let val_start = 2 + line.char_indices().find(|(_, c)| c == &'=').unwrap().0;
				let idx = line[4..idx_end].parse::<u64>().unwrap();
				let val = line[val_start..].parse().unwrap();
				mem.insert(idx, mask.mask(val));
			}
			_ => unreachable!(),
		}
	}
	mem.values().sum()
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Mask {
	one_mask: u64,
	zero_mask: u64,
}

impl Mask {
	fn new() -> Self {
		Mask {
			one_mask: 0,
			zero_mask: u64::MAX,
		}
	}

	fn from(s: &str) -> Self {
		Mask {
			one_mask: s
				.bytes()
				.fold(0, |acc, curr| (acc << 1) | (curr == b'1') as u64),
			zero_mask: s
				.bytes()
				.fold(0, |acc, curr| (acc << 1) | (curr != b'0') as u64),
		}
	}

	fn mask(&self, num: u64) -> u64 {
		(num & self.zero_mask) | self.one_mask
	}
}

fn solve2(input: &[&str]) -> u64 {
	let mut mem = HashMap::new();
	let mut mask1 = 0;
	let mut mask2 = "";
	for &line in input.iter() {
		match &line[..4] {
			"mask" => {
				mask2 = &line[7..];
				mask1 = parse_mask(mask2);
			}
			"mem[" => {
				let idx_end = line.char_indices().find(|(_, c)| c == &']').unwrap().0;
				let val_start = 2 + line.char_indices().find(|(_, c)| c == &'=').unwrap().0;
				let idx = line[4..idx_end].parse::<u64>().unwrap();
				let idx_mask1 = mask1 | idx;
				let val = line[val_start..].parse().unwrap();
				for adr in variations(idx_mask1, mask2).into_iter() {
					mem.insert(adr, val);
				}
			}
			_ => unreachable!(),
		}
	}
	mem.values().sum()
}

fn parse_mask(num_str: &str) -> u64 {
	num_str
		.bytes()
		.fold(0, |acc, curr| (acc << 1) | ((curr == b'1') as u64))
}

fn variations(n: u64, mask: &str) -> Vec<u64> {
	let positions = mask
		.char_indices()
		.filter(|&(_, d)| d == 'X')
		.map(|(idx, _)| 35 - idx as u64)
		.collect::<Vec<u64>>();
	(0..2u64.pow(positions.len() as u32))
		.rev()
		.map(|m| {
			positions.iter().enumerate().fold(n, |acc, (i, &curr)| {
				// set the curr:th bit (from the right) of acc to the i:th bit of m
				let to_set = (((1 << i) & m) << curr) >> i;
				let with_hole = (!(1 << curr)) & acc;
				with_hole | to_set
			})
		})
		.collect()
}
