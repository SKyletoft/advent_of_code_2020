use std::ops::RangeInclusive;

fn main() {
	let input = include_str!("input.txt").lines().collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[&str]) -> u64 {
	let mut valid_ranges = input
		.iter()
		.take(20)
		.map(parse_limit_line)
		.flat_map(|(a, b)| a.chain(b))
		.collect::<Vec<_>>();
	valid_ranges.sort_unstable();
	valid_ranges.dedup();
	input
		.iter()
		.skip(25)
		.flat_map(parse_line)
		.filter(|x| !valid_ranges.contains(x))
		.sum()
}

fn solve2(input: &[&str]) -> u64 {
	let valid_ranges = input
		.iter()
		.take(20)
		.map(parse_limit_line)
		.collect::<Vec<_>>();
	let my_ticket = parse_line(&input[22]);
	let valid_tickets = input
		.iter()
		.skip(25)
		.map(parse_line)
		.filter(|v| {
			v.iter().all(|n| {
				valid_ranges
					.iter()
					.any(|(r1, r2)| r1.contains(n) || r2.contains(n))
			})
		})
		.collect::<Vec<_>>();
	assert!(valid_tickets.iter().all(|v| v.len() == 20));

	let mut possible = (0..20)
		.map(|target| {
			let (r1, r2) = &valid_ranges[target];
			let mut acc = [true; 20];
			valid_tickets.iter().for_each(|curr| {
				acc.iter_mut().zip(curr.iter()).for_each(|(a, c)| {
					*a &= r1.contains(c) || r2.contains(c);
				});
			});
			acc
		})
		.collect::<Vec<_>>();

	for line_idx in (0..possible.len()).cycle() {
		if possible[line_idx].iter().map(|&v| v as u8).sum::<u8>() != 1 {
			continue;
		}
		let column_idx = possible[line_idx].iter().position(|&x| x).unwrap();
		for line in possible.iter_mut() {
			line[column_idx] = false;
		}
		possible[line_idx][column_idx] = true;
		if possible
			.iter()
			.map(|l| l.iter().map(|&v| v as u8).sum::<u8>())
			.all(|x| x == 1)
		{
			break;
		}
	}

	possible
		.iter()
		.take(6)
		.map(|v| {
			let idx = v.iter().position(|&x| x).unwrap();
			my_ticket[idx]
		})
		.product()
}

fn parse_line(line: &&str) -> Vec<u64> {
	line.split(',')
		.map(|n| n.parse::<u64>().unwrap())
		.collect::<Vec<_>>()
}

fn parse_limit_line(line: &&str) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
	let len = line.len();
	let mut idx = line.char_indices();
	let mut flip = true;
	let mut find_next = || {
		flip = !flip;
		idx.find(|&(_, c)| flip ^ c.is_ascii_digit())
			.map(|(idx, _)| idx)
			.unwrap_or(len)
	};
	let mut next_section = || {
		line[find_next()..find_next()].parse::<u64>().unwrap()
			..=line[find_next()..find_next()].parse::<u64>().unwrap()
	};
	(next_section(), next_section())
}
