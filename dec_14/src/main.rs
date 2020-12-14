use std::collections::HashMap;

fn main() {
	let input = include_str!("input.txt").lines().collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[&str]) -> usize {
	let mut mem: HashMap<usize, usize> = HashMap::new();
	let mut mask = Mask {
		one_mask: 0,
		zero_mask: usize::MAX,
	};
	for &line in input.iter() {
		match &line[..4] {
			"mask" => {
				mask = Mask::new(&line[7..]);
			}
			"mem[" => {
				let idx_end = line.char_indices().find(|(_, c)| c == &']').unwrap().0;
				let val_start = 2 + line.char_indices().find(|(_, c)| c == &'=').unwrap().0;
				let idx = line[4..idx_end].parse().unwrap();
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
	one_mask: usize,
	zero_mask: usize,
}

impl Mask {
	fn new(s: &str) -> Self {
		Mask {
			one_mask: s.bytes().fold(0, |acc, curr| {
				if curr == b'1' {
					(acc << 1) | 1
				} else {
					acc << 1
				}
			}),
			zero_mask: s.bytes().fold(0, |acc, curr| {
				if curr == b'0' {
					acc << 1
				} else {
					(acc << 1) | 1
				}
			}),
		}
	}
	fn mask(&self, num: usize) -> usize {
		(num & self.zero_mask) | self.one_mask
	}
}

fn solve2(input: &[&str]) -> usize {
	let mut mem: HashMap<usize, usize> = HashMap::new();
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
				let idx: usize = line[4..idx_end].parse().unwrap();
				let idx_mask1 = mask1 | idx;
				let val = line[val_start..].parse().unwrap();
				for adr in permutations(idx_mask1, mask2).into_iter() {
					mem.insert(adr, val);
				}
			}
			_ => unreachable!(),
		}
	}
	mem.values().sum()
}

fn parse_mask(num_str: &str) -> usize {
	num_str
		.bytes()
		.fold(0, |acc, curr| (acc << 1) | ((curr == b'1') as usize))
}

fn permutations(n: usize, mask: &str) -> Vec<usize> {
	let positions = mask
		.char_indices()
		.filter(|&(_, d)| d == 'X')
		.map(|(idx, _)| 35 - idx)
		.collect::<Vec<usize>>();
	(0..2usize.pow(positions.len() as u32))
		.rev()
		.map(|m| {
			positions.iter().enumerate().fold(n, |acc, (i, &curr)| {
				// set the curr:th bit (from the right) of acc to the i:th bit of m
				let to_set = (((1 << i) & m) << curr) >> i;
				let with_hole = (!(1 << curr)) & acc;
				with_hole | to_set
			})
		})
		.collect::<Vec<_>>()
}
