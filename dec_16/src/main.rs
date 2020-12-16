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
		.flat_map(|l| {
			let mut i = l.char_indices();
			let mut flip = true;
			let mut find_next = || {
				flip = !flip;
				i.find(|&(_, c)| flip ^ c.is_ascii_digit())
					.map(|(i, _)| i)
					.unwrap()
			};
			let start_of_n1 = find_next();
			let end_of_n1 = find_next();
			let start_of_n2 = find_next();
			let end_of_n2 = find_next();
			let start_of_n3 = find_next();
			let end_of_n3 = find_next();
			let start_of_n4 = find_next();
			let n1 = l[start_of_n1..end_of_n1].parse::<u64>().unwrap();
			let n2 = l[start_of_n2..end_of_n2].parse::<u64>().unwrap();
			let n3 = l[start_of_n3..end_of_n3].parse::<u64>().unwrap();
			let n4 = l[start_of_n4..].parse::<u64>().unwrap();
			(n1..n2).chain(n3..n4)
		})
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
		.map(|l| {
			let len = l.len();
			let mut i = l.char_indices();
			let mut flip = true;
			let mut find_next = || {
				flip = !flip;
				i.find(|&(_, c)| flip ^ c.is_ascii_digit())
					.map(|(i, _)| i)
					.unwrap_or(len)
			};
			let mut next_section = || {
				l[find_next()..find_next()].parse::<u64>().unwrap()
					..l[find_next()..find_next()].parse::<u64>().unwrap()
			};
			(next_section(), next_section())
		})
		.collect::<Vec<_>>();
	let my_ticket = parse_line(&input[22]);
	let valid_tickets = input
		.iter()
		.skip(25)
		.map(parse_line)
		.filter(|v| {
			v.iter().all(|&n| {
				valid_ranges
					.iter()
					.any(|(r1, r2)| (n >= r1.start && n < r1.end) || (n >= r2.start && n < r2.end))
			})
		})
		.collect::<Vec<_>>();
	valid_tickets.iter().for_each(|v| assert!(v.len() == 20));
	let mut possible = (0..20)
		.map(|target| {
			valid_tickets.iter().fold([true; 20], |mut acc, curr| {
				acc.iter_mut().zip(curr.iter()).for_each(|(a, &t)| {
					let (r1, r2) = &valid_ranges[target];
					*a = *a && ((r1.start <= t && t <= r1.end) || (r2.start <= t && t <= r2.end))
				});
				acc
			})
		})
		.collect::<Vec<_>>();
	let mut i = 0;
	loop {
		i = (i + 1) % possible.len();
		let sum = possible[i].iter().map(|&v| v as u8).sum::<u8>();
		if sum == 1 {
			let index = possible[i].iter().position(|&x| x).unwrap();
			for (_, line) in possible.iter_mut().enumerate().filter(|&(j, _)| j != i) {
				line[index] = false;
			}
			if possible
				.iter()
				.map(|l| l.iter().map(|&v| if v { 1 } else { 0 }).sum::<u8>())
				.all(|x| x == 1)
			{
				break;
			}
		}
	}
	possible
		.iter()
		.take(6)
		.map(|v| {
			let i = v.iter().position(|&x| x).unwrap();
			my_ticket[i]
		})
		.product()
}

fn parse_line(l: &&str) -> Vec<u64> {
	l.split(',')
		.map(|n| n.parse::<u64>().unwrap())
		.collect::<Vec<_>>()
}
