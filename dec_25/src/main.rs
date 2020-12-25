use rayon::prelude::*;

const MOD: u64 = 20201227;

fn main() {
	let input = include_str!("input.txt")
		.lines()
		.map(|x| x.parse().unwrap())
		.collect::<Vec<u64>>();
	dbg!(&input);
	let example = vec![5764801, 17807724];
	let sol1 = solve1(&input);
	println!("{}", sol1);
}

fn solve1(input: &[u64]) -> u64 {
	let door_public_key = input[0];
	let card_public_key = input[1];

	let card_loop_size = solve_transform(7, card_public_key);
	dbg!(card_loop_size);
	let door_loop_size = solve_transform(7, door_public_key);
	dbg!(door_loop_size);

	let encryption_key_1 = transform(door_public_key, card_loop_size);
	let encryption_key_2 = transform(card_public_key, door_loop_size);
	assert_eq!(encryption_key_1, encryption_key_2);
	encryption_key_1
}

fn transform(sub_num: u64, loops: u64) -> u64 {
	let mut val = 1;
	for _ in 0..loops {
		val *= sub_num;
		while val >= MOD {
			val -= MOD;
		}
		//val %= 20201227;
	}
	val
}

fn solve_transform(sub_num: u64, target: u64) -> u64 {
	let offset = ((MOD / 7) + 1) * 7;
	for which in 0.. {
		let res = ((target + offset * which) as f64).log(sub_num as f64) as u32;
		if 7u64.pow(res) == target {
			return res as u64;
		}
	}
	0
}
