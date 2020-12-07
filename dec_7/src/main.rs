use std::collections::{HashMap, HashSet};

fn main() {
	let input = include_str!("input.txt").lines().collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[&str]) -> usize {
	let parsed = input
		.iter()
		.map(|l| parse_bag(l))
		.collect::<HashMap<_, _>>();

	let mut can_contain_white = HashSet::new();
	let mut can_contain_new = HashSet::new();

	can_contain_new.insert("shiny gold");

	while !can_contain_new.is_empty() {
		let target = *can_contain_new.iter().next().expect("loop requirements");
		can_contain_new.remove(target);
		can_contain_white.insert(target);
		for (&outer_bag, _) in parsed
			.iter()
			.filter(|(&k, v)| v.contains(&target) && !can_contain_white.contains(k))
		{
			can_contain_new.insert(outer_bag);
		}
	}
	can_contain_white.len() - 1
}

fn solve2(input: &[&str]) -> usize {
	let parsed = input
		.iter()
		.map(|l| parse_bag_with_amounts(l))
		.collect::<HashMap<_, _>>();
	let mut next = vec![(1, "shiny gold")];
	let mut needed = Vec::new();
	while !next.is_empty() {
		let (t_count, t_name) = next.remove(0);
		needed.push((t_count, t_name));
		if let Some(v) = parsed.get(t_name) {
			for (c, &n) in v.iter().map(|(c, n)| (c * t_count, n)) {
				next.push((c, n));
			}
		}
	}
	needed.iter().map(|&(c, _)| c).sum()
}

fn strip_bags(s: &str) -> &str {
	s.trim()
		.strip_suffix(" bags,")
		.or_else(|| s.trim().strip_suffix(" bags"))
		.or_else(|| s.trim().strip_suffix(" bag,"))
		.or_else(|| s.trim().strip_suffix(" bag"))
		.unwrap_or_else(|| s.trim())
}

fn parse_bag(mut bag: &str) -> (&str, Vec<&str>) {
	bag = bag.trim_end_matches('.');
	let (mut first, mut second) = bag.split_at(bag.find(" contain").unwrap());
	first = strip_bags(first);
	second = &second[8..];
	let matches = second
		.split(char::is_numeric)
		.skip(1)
		.map(strip_bags)
		.collect();
	(first, matches)
}

fn parse_bag_with_amounts(mut bag: &str) -> (&str, Vec<(usize, &str)>) {
	bag = bag.trim_end_matches('.');
	let (mut first, mut second) = bag.split_at(bag.find(" contain").unwrap());
	first = strip_bags(first);
	second = &second[8..];
	let matches = second
		.split(", ")
		.filter(|l| l != &" no other bags")
		.map(|s| {
			let st = s.trim();
			let w = st.find(char::is_whitespace).unwrap();
			(st[..w].parse().unwrap(), strip_bags(&st[w..]))
		})
		.collect();
	(first, matches)
}
