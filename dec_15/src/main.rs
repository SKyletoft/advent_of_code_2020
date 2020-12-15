use std::collections::HashMap;

fn main() {
	let input = include_str!("input.txt")
		.split(',')
		.map(|n| n.parse().unwrap())
		.collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[u64]) -> u64 {
	run(input, 2020)
}

fn solve2(input: &[u64]) -> u64 {
	run(input, 30000000)
}

fn run(input: &[u64], limit: u64) -> u64 {
	let mut last = input[..input.len()-1]
		.iter()
		.enumerate()
		.map(|(i, &v)| (v as u64, i as u64 + 1))
		.collect::<HashMap<_, _>>();
	let mut turn = input.len() as u64;
	let mut last_spoken = input[input.len() - 1];
	while turn < limit {
		if let Some(l) = last.get_mut(&last_spoken) {
			last_spoken = turn - *l;
			*l = turn;
		} else {
			last.insert(last_spoken, turn);
			last_spoken = 0;
		}
		turn += 1;
	}
	last_spoken
}
