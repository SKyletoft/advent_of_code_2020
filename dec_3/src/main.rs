fn main() {
	let inputs = include_str!("input.txt").lines().collect::<Vec<_>>();
	let sol1 = part_1(&inputs, 3);
	println!("{}", sol1);
	let sol2 = [
		part_2(&inputs, 1, 1),
		part_2(&inputs, 3, 1),
		part_2(&inputs, 5, 1),
		part_2(&inputs, 7, 1),
		part_2(&inputs, 1, 2),
	];
	println!("{:?}, {}", sol2, sol2.iter().product::<usize>());
}

fn part_1(inputs: &[&str], r_slope: usize) -> usize {
	inputs
		.iter()
		.enumerate()
		.filter(|(x, line)| {
			let line = line.as_bytes();
			line[(x * r_slope) % line.len()] == b'#'
		})
		.count()
}

fn part_2(inputs: &[&str], r_slope: usize, d_slope: usize) -> usize {
	inputs
		.iter()
		.enumerate()
		.step_by(d_slope)
		.filter(|(x, line)| {
			let line = line.as_bytes();
			line[(x * r_slope / d_slope) % line.len()] == b'#'
		})
		.count()
}
