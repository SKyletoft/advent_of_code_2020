fn main() {
	let input = include_str!("input.txt").lines().collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(ids: &[&str]) -> u16 {
	ids.iter().map(parse_id).max().unwrap_or(0)
}

fn parse_id(id: &&str) -> u16 {
	id.chars().fold(0, |acc, c| {
		if c == 'B' || c == 'R' {
			(acc << 1) + 1
		} else {
			acc << 1
		}
	})
}

fn solve2(ids: &[&str]) -> u16 {
	let ids = ids.iter().map(parse_id).collect::<Vec<_>>();
	let min = *ids
		.iter()
		.min()
		.expect("There should be at least one element in the list");
	(min..)
		.find(|n| ids.contains(n) && ids.contains(&(*n + 2)) && !ids.contains(&(*n + 1)))
		.expect("There should be an id with both neigbours")
		+ 1
}
