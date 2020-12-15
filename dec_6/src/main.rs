use itertools::Itertools;

fn main() {
	let input = include_str!("input.txt");
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &str) -> u64 {
	input
		.split("\n\n")
		.map(|group| {
			group
				.chars()
				.filter(char::is_ascii_alphabetic)
				.unique()
				.count()
		})
		.sum::<usize>() as u64
}

fn solve2(input: &str) -> u64 {
	input
		.split("\n\n")
		.map(|group| {
			let members = group.lines().collect::<Vec<_>>();
			group
				.chars()
				.unique()
				.filter(|&c| c.is_ascii_alphabetic() && members.iter().all(|m| m.contains(c)))
				.count()
		})
		.sum::<usize>() as u64
}
