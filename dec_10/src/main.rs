fn main() {
	let input = include_str!("example.txt")
		.lines()
		.map(|l| l.parse().unwrap())
		.collect::<Vec<u32>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[u32]) -> u32 {
	let max = *input.iter().max().unwrap() + 3;
	let mut owned = input.to_vec();
	owned.push(max);
	owned.sort_unstable();
	let ((one, three), _) = owned
		.iter()
		.fold(((0, 0), 0), |((ones, threes), prev), &curr| {
			match curr - prev {
				0 | 2 => ((ones, threes), curr),
				1 => ((ones + 1, threes), curr),
				3 => ((ones, threes + 1), curr),
				_ => panic!(),
			}
		});
	one * three
}

fn solve2(input: &[u32]) -> u32 {
	let mut clone = input.to_vec();
	clone.sort();
	helper(clone, 0)
}

fn helper(from: Vec<u32>, prev: u32) -> u32 {
	let options = from
		.iter()
		.filter(|&&x| x >= prev && x <= prev + 3)
		.map(|&x| x)
		.collect::<Vec<_>>();
	if options.is_empty() {
		return 1;
	}
	options
		.iter()
		.map(|&item| {
			let index = from.binary_search(&item).unwrap();
			let clone = from.iter().skip(index).map(|&x| x).collect::<Vec<_>>();
			helper(clone, item)
		})
		.sum()
}
