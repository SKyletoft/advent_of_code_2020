type CustomErr = Box<dyn std::error::Error>;

fn main() -> Result<(), CustomErr> {
	let inputs = include_str!("input.txt")
		.lines()
		.map(|x| x.parse::<usize>().unwrap())
		.collect::<Vec<usize>>();

	for (index_1, &lhs) in inputs.iter().enumerate() {
		for (index_2, &rhs) in inputs.iter().enumerate().skip(index_1) {
			if lhs + rhs == 2020 {
				println!("{} * {} = {}", lhs, rhs, lhs * rhs);
			}
			for ths in inputs.iter().skip(index_2) {
				if lhs + rhs + ths == 2020 {
					println!("{} * {} * {} = {}", lhs, rhs, ths, ths * lhs * rhs);
				}
			}
		}
	}

	Ok(())
}
