use std::collections::{BTreeSet, HashSet, VecDeque};

fn main() {
	let input = include_str!("input.txt").lines().collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[&str]) -> u64 {
	let (mut player1, mut player2) = parse_input(input);
	while !player1.is_empty() && !player2.is_empty() {
		let p1 = player1.pop_front().unwrap();
		let p2 = player2.pop_front().unwrap();
		if p1 > p2 {
			player1.push_back(p1);
			player1.push_back(p2);
		} else {
			player2.push_back(p2);
			player2.push_back(p1);
		}
	}
	calculate_score(&player1, &player2)
}

fn solve2(input: &[&str]) -> u64 {
	let (mut player1, mut player2) = parse_input(input);
	let mut history = HashSet::new();
	let res = recursive_combat(&mut player1, &mut player2, &mut history);
	match res {
		Some(true) | None => player2.clear(),
		Some(false) => player1.clear(),
	}
	dbg!(&player1);
	dbg!(&player2);
	calculate_score(&player1, &player2)
}

fn parse_input(input: &[&str]) -> (VecDeque<u8>, VecDeque<u8>) {
	let player1 = input
		.iter()
		.skip(1)
		.take_while(|l| !l.is_empty())
		.map(|l| l.parse::<u8>().unwrap())
		.collect::<VecDeque<_>>();
	let player2 = input
		.iter()
		.skip(player1.len() + 3)
		.take_while(|l| !l.is_empty())
		.map(|l| l.parse::<u8>().unwrap())
		.collect::<VecDeque<_>>();
	(player1, player2)
}

fn calculate_score(player1: &VecDeque<u8>, player2: &VecDeque<u8>) -> u64 {
	player1
		.iter()
		.rev()
		.enumerate()
		.map(|(i, &v)| (i as u64 + 1) * v as u64)
		.sum::<u64>()
		+ player2
			.iter()
			.rev()
			.enumerate()
			.map(|(i, &v)| (i as u64 + 1) * v as u64)
			.sum::<u64>()
}

fn recursive_combat(
	player1: &mut VecDeque<u8>,
	player2: &mut VecDeque<u8>,
	history: &mut HashSet<GameState>,
) -> Option<bool> {
	while !player1.is_empty() && !player2.is_empty() {
		let history_entry = GameState::from_iter(
			player1
				.iter()
				.chain([0u8].iter())
				.chain(player2.iter())
				.copied(),
		);
		if history.contains(&history_entry) {
			return None;
		} else {
			history.insert(history_entry);
		}
		let p1 = player1.pop_front().unwrap();
		let p2 = player2.pop_front().unwrap();
		let winner = if player1.len() as u8 >= p1 && player2.len() as u8 >= p2 {
			let mut c1 = player1.clone();
			let mut c2 = player2.clone();
			let mut h = HashSet::new();
			recursive_combat(&mut c1, &mut c2, &mut h)?
		} else {
			p1 > p2
		};
		if winner {
			player1.push_back(p1);
			player1.push_back(p2);
		} else {
			player2.push_back(p2);
			player2.push_back(p1);
		}
	}
	Some(player2.is_empty())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GameState {
	a: u128,
	b: u128,
	c: u128,
}

impl GameState {
	fn from_iter(i: impl Iterator<Item = u8> + Clone) -> Self {
		let iter = i.map(|n| n as u128);
		let mut gs = GameState { a: 0, b: 0, c: 0 };
		for n in iter.clone().take(21) {
			gs.a = (gs.a << 6) + n;
		}
		for n in iter.clone().skip(21 * 2).take(21) {
			gs.b = (gs.b << 6) + n;
		}
		for n in iter.skip(21 * 3).take(21) {
			gs.c = (gs.c << 6) + n;
		}
		gs
	}
}
