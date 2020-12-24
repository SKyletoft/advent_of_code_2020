use std::collections::{LinkedList, VecDeque};

fn main() {
	let input = include_str!("input.txt");
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &str) -> u64 {
	let mut cupsl = input
		.bytes()
		.map(|b| (b - b'0') as u32)
		.collect::<LinkedList<_>>();
	let mut cupsvd = input
		.bytes()
		.map(|b| (b - b'0') as u32)
		.collect::<VecDeque<_>>();
	run_ll(&mut cupsl, 100);
	run(&mut cupsvd, 100);
	assert!(cupsl.iter().zip(cupsvd.iter()).all(|(a, b)| a == b));
	while cupsl.front() != Some(&1) {
		let next = cupsl.pop_front().unwrap();
		cupsl.push_back(next);
	}
	cupsl.pop_front();
	cupsl.iter().fold(0, |acc, &curr| acc * 10 + curr as u64)
}

fn solve2(input: &str) -> u64 {
	let mut cups = input
		.bytes()
		.map(|b| (b - b'0') as u32)
		.chain(10..)
		.take(1_000_000)
		.collect::<VecDeque<_>>();

	run(&mut cups, 10_000_000);
	let mut iter = cups.iter().skip_while(|&&i| i != 1);
	if let (Some(1), Some(&a), Some(&b)) = (iter.next(), iter.next(), iter.next()) {
		dbg!((a, b));
		a as u64 * b as u64
	} else {
		panic!();
	}
}

fn run(cups: &mut VecDeque<u32>, moves: usize) {
	let max = *cups.iter().max().unwrap();
	for _ in 0..moves {
		let current_cup = cups.pop_front().unwrap();
		let r1 = cups.pop_front().unwrap();
		let r2 = cups.pop_front().unwrap();
		let r3 = cups.pop_front().unwrap();
		let destination_cup = {
			let mut target = current_cup;
			loop {
				target -= 1;
				if target == 0 {
					target = max;
				}
				if target != r1 && target != r2 && target != r3 && target != current_cup {
					break;
				}
			}
			let pos = cups.iter().position(|c| *c == target).unwrap();
			pos
		};
		cups.insert(destination_cup + 1, r1);
		cups.insert(destination_cup + 2, r2);
		cups.insert(destination_cup + 3, r3);
		cups.push_back(current_cup);
	}
}

fn run_ll(cups: &mut LinkedList<u32>, moves: usize) {
	let max = *cups.iter().max().unwrap();
	for _ in 0..moves {
		let current_cup = cups.pop_front().unwrap();
		let r1 = cups.pop_front().unwrap();
		let r2 = cups.pop_front().unwrap();
		let r3 = cups.pop_front().unwrap();
		let destination_cup = {
			let mut target = current_cup;
			loop {
				target -= 1;
				if target == 0 {
					target = max;
				}
				if target != r1 && target != r2 && target != r3 && target != current_cup {
					break;
				}
			}
			let pos = cups.iter().position(|c| *c == target).unwrap();
			pos
		};
		let mut rest = cups.split_off(destination_cup + 1);
		rest.push_front(r3);
		rest.push_front(r2);
		rest.push_front(r1);
		cups.append(&mut rest);
		cups.push_back(current_cup);
	}
}
