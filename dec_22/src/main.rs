use std::collections::{HashSet, VecDeque};

fn main() {
	let input = include_str!("input.txt").lines().collect::<Vec<_>>();
	let (player1, player2) = parse_input(&input);
	let sol1 = solve1(player1.clone(), player2.clone());
	let sol2 = solve2(player1, player2);
	println!("{} {}", sol1, sol2);
}

fn solve1(mut player1: VecDeque<u8>, mut player2: VecDeque<u8>) -> u64 {
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

fn solve2(mut player1: VecDeque<u8>, mut player2: VecDeque<u8>) -> u64 {
	let res = recursive_combat(&mut player1, &mut player2);
	match res {
		true => &mut player2.clear(),
		false => &mut player1.clear(),
	};
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
	let p1 = player1
		.iter()
		.rev()
		.enumerate()
		.map(|(i, &v)| (i as u64 + 1) * v as u64)
		.sum::<u64>();
	let p2 = player2
		.iter()
		.rev()
		.enumerate()
		.map(|(i, &v)| (i as u64 + 1) * v as u64)
		.sum::<u64>();
	p1.max(p2)
}

fn recursive_combat(player1: &mut VecDeque<u8>, player2: &mut VecDeque<u8>) -> bool {
	let mut history = HashSet::new();
	while !player1.is_empty() && !player2.is_empty() {
		let history_entry = GameState::from_iter(player1.iter().copied());
		if history.contains(&history_entry) {
			return true;
		} else {
			history.insert(history_entry);
		}
		let p1 = player1.pop_front().unwrap();
		let p2 = player2.pop_front().unwrap();
		let winner = if player1.len() as u8 >= p1 && player2.len() as u8 >= p2 {
			let mut c1 = player1.iter().take(p1 as usize).copied().collect();
			let mut c2 = player2.iter().take(p2 as usize).copied().collect();
			recursive_combat(&mut c1, &mut c2)
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
	player2.is_empty()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct GameState {
	a: u32, //The more bits, the lower chance of collisions.
	        //6 bits per number up to a total of 6 * 51 for 100% accuracy
}

impl GameState {
	fn from_iter(i: impl Iterator<Item = u8> + Clone) -> Self {
		let mut gs = GameState { a: 0 };
		for n in i.map(|n| n as u32).take(32 / 6) {
			gs.a = (gs.a << 6) | n;
		}
		gs
	}
}
