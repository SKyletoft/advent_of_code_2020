fn main() {
	let input = include_str!("input.txt");
	let sol1 = solve1(input);
	let sol2 = solve2(input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &str) -> u64 {
	let linebreak = input.chars().position(|c| c == '\n').unwrap();
	let time_stamp = input[..linebreak].parse::<u64>().unwrap();
	let (bus, min_wait) = input[(linebreak + 1)..]
		.trim()
		.split(',')
		.filter_map(|s| s.parse::<u64>().ok())
		.map(|b| (b, b - time_stamp % b))
		.min_by_key(|(_, b)| *b)
		.unwrap();
	bus * min_wait
}

fn solve2(input: &str) -> u64 {
	let linebreak = input.chars().position(|c| c == '\n').unwrap();
	let mut buses = input[(linebreak + 1)..]
		.trim()
		.split(',')
		.enumerate()
		.map(|(i, s)| (i, s.parse()))
		.filter(|(_, x)| x.is_ok())
		.map(|(a, b)| (a as u64, b.unwrap()))
		.map(|(a, b)| ((16 * b - a) % b, b))
		.collect::<Vec<(u64, u64)>>();
	buses.sort_unstable_by_key(|(_, a)| *a);
	let (lcm, res) = buses
		.iter()
		.skip(1)
		.fold((buses[0].1, buses[0].0), |(x, r), &(a, b)| {
			find_mod(x, r, b, a)
		});

	assert_eq!(lcm, buses.iter().map(|(_, p)| p).product());
	for &(index, prim) in buses.iter() {
		assert_eq!(res % prim, index, "{} {}", index, prim);
	}

	res
}

fn find_mod(x1: u64, r1: u64, x2: u64, r2: u64) -> (u64, u64) {
	let lcm = x1 * x2;
	let r2 = (r2 + x2 * 16) % x2;
	let r1 = (r1 + x1 * 16) % x1;
	let res = (r1..=lcm)
		.step_by(x1 as usize)
		.find(|&x| x % x2 == r2)
		.unwrap();
	(lcm, res)
}
