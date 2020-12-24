fn main() {
	let input = include_str!("input.txt");
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &str) -> usize {
	let mut cups = vec![0; input.len() + 1];

	for (idx, to) in input
		.bytes()
		.map(|b| (b - b'0') as usize)
		.zip(input.bytes().map(|b| (b - b'0') as usize).cycle().skip(1))
	{
		cups[idx] = to;
	}
	cups[0] = (input.as_bytes()[0] - b'0') as usize;

	run(&mut cups, 100);

	let mut idx = cups[cups.iter().position(|c| c == &1).unwrap()];
	let mut sum = 0;
	for _ in 0..(cups.len() - 2) {
		sum *= 10;
		idx = cups[idx];
		sum += idx;
	}
	sum
}

fn solve2(input: &str) -> usize {
	let mut cups = vec![0; input.len() + 1];
	cups.reserve(1_000_000);

	for (idx, to) in input
		.bytes()
		.map(|b| (b - b'0') as usize)
		.zip(input.bytes().map(|b| (b - b'0') as usize).skip(1))
	{
		cups[idx] = to;
	}
	cups[0] = (input.as_bytes()[0] - b'0') as usize;
	let last = cups.iter().position(|c| c == &0).unwrap();
	cups[last] = 10;
	for i in 11..=1_000_000 {
		cups.push(i);
	}
	cups.push(cups[0]);

	run(&mut cups, 10_000_000);

	let a = cups[1];
	let b = cups[a];
	a * b
}

fn run(list: &mut [usize], rounds: usize) {
	let max = *list.iter().max().unwrap();
	let mut idx = list[0];
	for _ in 0..rounds {
		//list: index is the label, value is the following cup
		let r1 = list[idx];
		let r2 = list[r1];
		let r3 = list[r2];
		let after = list[r3];
		let to_add_after = {
			let mut suggestion = idx;
			loop {
				suggestion -= 1;
				if suggestion == 0 {
					suggestion = max;
				}
				if ![r1, r2, r3].contains(&suggestion) {
					break;
				}
			}
			suggestion
		};
		list[idx] = list[r3];
		list[r3] = list[to_add_after];
		list[to_add_after] = r1;
		idx = after;
	}
}
