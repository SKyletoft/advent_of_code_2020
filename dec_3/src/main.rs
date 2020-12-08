fn main() {
	let inputs = include_str!("input.txt").lines().collect::<Vec<_>>();
	let sol1 = part_1(&inputs, 3, 1);
	let sol2 = part_2(&inputs);
	println!("{} {}", sol1, sol2);
}

fn part_1(inputs: &[&str], r_slope: usize, d_slope: usize) -> usize {
	inputs
		.iter()
		.step_by(d_slope)
		.enumerate()
		.filter(|(x, line)| line.as_bytes().get((x * r_slope) % line.len()) == Some(&b'#'))
		.count()
}

fn part_2(inputs: &[&str]) -> usize {
	let part_1_sols = [
		part_1(inputs, 1, 1),
		part_1(inputs, 3, 1),
		part_1(inputs, 5, 1),
		part_1(inputs, 7, 1),
		part_1(inputs, 1, 2),
	];
	part_1_sols.iter().product()
}
