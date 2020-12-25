const MOD: u64 = 20201227;

fn main() {
	let input = include_str!("input.txt")
		.lines()
		.map(|x| x.parse().unwrap())
		.collect::<Vec<u64>>();
	dbg!(&input);
	let sol1 = solve1(&input);
	println!("{}", sol1);
}

fn solve1(input: &[u64]) -> u64 {
	let door_public_key = input[0];
	let card_public_key = input[1];

	//let door_loop_size = solve_transform(7, door_public_key);
	let card_loop_size = solve_transform(7, card_public_key);

	let encryption_key_1 = transform(door_public_key, card_loop_size);
	//let encryption_key_2 = transform(card_public_key, door_loop_size);
	//assert_eq!(encryption_key_1, encryption_key_2);
	encryption_key_1
}

fn transform(sub_num: u64, loops: u64) -> u64 {
	pow2(sub_num, loops as u32, MOD)
}

pub const fn pow2(mut base: u64, mut exp: u32, m: u64) -> u64 {
	if exp == 0 {
		return 1;
	}
	let mut acc = 1;

	while exp > 1 {
		if (exp & 1) == 1 {
			acc = (acc * base) % m;
		}
		exp /= 2;
		base = base * base;
		base %= m;
	}
	(acc * base) % m
}

fn solve_transform(sub_num: u64, target: u64) -> u64 {
	for x in 0.. {
		let res = transform(sub_num, x);
		if res == target {
			return x;
		}
	}
	panic!();
}
