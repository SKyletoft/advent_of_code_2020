fn main() {
	let input = include_str!("input.txt")
		.lines()
		.map(|x| x.parse::<u64>().unwrap())
		.collect::<Vec<u64>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[u64]) -> u64 {
	for (index_1, &lhs) in input.iter().enumerate() {
		for &rhs in input.iter().skip(index_1) {
			if lhs + rhs == 2020 {
				return lhs * rhs;
			}
		}
	}
	0
}

fn solve2(input: &[u64]) -> u64 {
	for (index_1, &lhs) in input.iter().enumerate() {
		for (index_2, &rhs) in input.iter().enumerate().skip(index_1) {
			for ths in input.iter().skip(index_2) {
				if lhs + rhs + ths == 2020 {
					return lhs * rhs * ths;
				}
			}
		}
	}
	0
}
