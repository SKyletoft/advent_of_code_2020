fn main() {
	let input = include_str!("input.txt");
	let sol1 = solve1(input);
	let sol2 = solve2(input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &str) -> usize {
	let linebreak = input.chars().position(|c| c == '\n').unwrap();
	let time_stamp = input[..linebreak].parse::<usize>().unwrap();
	let (bus, min_wait) = input[(linebreak + 1)..]
		.split(',')
		.filter_map(|s| s.parse::<usize>().ok())
		.map(|b| (b, b - time_stamp % b))
		.min_by_key(|(_, b)| *b)
		.unwrap();
	bus * min_wait
}

fn solve2(input: &str) -> usize {
	let linebreak = input.chars().position(|c| c == '\n').unwrap();
	let mut buses = input[(linebreak + 1)..]
		.split(',')
		.enumerate()
		.map(|(i, s)| (i, s.parse()))
		.filter(|(_, x)| x.is_ok())
		.map(|(a, b)| (a, b.unwrap()))
		.map(|(a, b)| ((16 * b - a) % b, b))
		.collect::<Vec<(usize, usize)>>();
	dbg!(&buses);
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

	dbg!(lcm);
	res
}

fn find_mod(x1: usize, r1: usize, x2: usize, r2: usize) -> (usize, usize) {
	let lcm = x1 * x2;
	let r2 = (r2 + x2 * 16) % x2;
	let r1 = (r1 + x1 * 16) % x1;
	let res = (r1..=lcm)
		.step_by(x1 as usize)
		.find(|&x| x % x2 == r2)
		.unwrap();
	(lcm, res)
}
