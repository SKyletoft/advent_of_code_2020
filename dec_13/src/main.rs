use rayon::prelude::*;

fn main() {
	let input = include_str!("input.txt");
	let sol1 = solve1(input);
	let sol2 = solve2(input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &str) -> usize {
	let linebreak = input.chars().position(|c| c == '\n').unwrap();
	let time_stamp = &input[..linebreak].parse().unwrap();
	let (bus, min_wait) = input[(linebreak + 1)..]
		.split(',')
		.filter_map(|s| s.parse::<usize>().ok())
		.map(|b| (b, b - time_stamp % b))
		.min_by_key(|(_, b)| *b)
		.unwrap();
	bus * min_wait
}

fn solve2(input: &str) -> isize {
	let linebreak = input.chars().position(|c| c == '\n').unwrap();
	let mut buses = input[(linebreak + 1)..]
		.split(',')
		.enumerate()
		.map(|(i, s)| (i, s.parse()))
		.filter(|(_, x)| x.is_ok())
		.map(|(a, b)| (a as isize, b.unwrap()))
		.collect::<Vec<(isize, isize)>>();
	buses.sort_unstable_by_key(|(_, a)| *a);
	dbg!(&buses);
	let first = (buses[0].1, buses[0].0);
	let res = buses
		.iter()
		.skip(1)
		.fold(first, |(x, r), &(a, b)| find_mod(x, r, b, (b - a).abs()))
		.1;

	for &(index, prim) in buses.iter() {
		assert_eq!(res % prim, index, "{} {}", index, prim);
	}

	res + buses[buses.len() - 1].0
}

fn find_mod(x1: isize, r1: isize, x2: isize, r2: isize) -> (isize, isize) {
	assert!(x2 != 0);
	let lcm = x1 * x2;
	assert!(lcm != 0);
	let r2 = (r2 + x2 * 16) % x2;
	let r1 = (r1 + x1 * 16) % x1;
	dbg!((x1, r1, x2, r2));
	(
		lcm,
		dbg!((r1..).step_by(x1 as usize).find(|&x| x % x2 == r2).unwrap()),
	)
}
