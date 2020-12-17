use std::mem;
use Cube::*;

const NEG_ONE: usize = (-1isize) as usize;
const NEIGHBOURING: [(usize, usize, usize); 26] = [
	(NEG_ONE, NEG_ONE, NEG_ONE),
	(NEG_ONE, NEG_ONE, 0),
	(NEG_ONE, NEG_ONE, 1),
	(NEG_ONE, 0, NEG_ONE),
	(NEG_ONE, 0, 0),
	(NEG_ONE, 0, 1),
	(NEG_ONE, 1, NEG_ONE),
	(NEG_ONE, 1, 0),
	(NEG_ONE, 1, 1),
	(0, NEG_ONE, NEG_ONE),
	(0, NEG_ONE, 0),
	(0, NEG_ONE, 1),
	(0, 0, NEG_ONE),
	(0, 0, 1),
	(0, 1, NEG_ONE),
	(0, 1, 0),
	(0, 1, 1),
	(1, NEG_ONE, NEG_ONE),
	(1, NEG_ONE, 0),
	(1, NEG_ONE, 1),
	(1, 0, NEG_ONE),
	(1, 0, 0),
	(1, 0, 1),
	(1, 1, NEG_ONE),
	(1, 1, 0),
	(1, 1, 1),
];

fn main() {
	let input = include_str!("input.txt");
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &str) -> usize {
	let height = input.lines().count() + 12;
	let width = input.lines().next().map(|l| l.len()).unwrap_or(0) + 12;
	let convert_index = |(x, y, z): (usize, usize, usize)| {
		if x < width && y < height && z < 14 {
			Some(x + y * width + z * width * height)
		} else {
			None
		}
	};
	let mut one = vec![Inactive; width * height * 13];
	let mut two = one.clone();

	for (x, line) in input.lines().enumerate() {
		for (y, c) in line.bytes().enumerate() {
			if c == b'#' {
				dbg!((x,y));
				one[convert_index((x + 6, y + 6, 6)).unwrap()] = Active;
			}
		}
	}

	for _ in 0..6 {
		for x in 0..width {
			for y in 0..height {
				for z in 0..14 {
					let idx = convert_index((x,y,z)).unwrap();
					let curr = *one.get(idx).unwrap();
					let neighbours = NEIGHBOURING
						.iter()
						.map(|&v| convert_index(v_add(v, (x, y, z))).and_then(|i| one.get(i)))
						.filter(|c| c == &Some(&Active))
						.count();
					let next = match (curr, neighbours) {
						(Active, 2) | (Active, 3) => Active,
						(Inactive, 3) => Active,
						_ => Inactive
					};
					*two.get_mut(idx).unwrap() = next;
				}
			}
		}
		mem::swap(&mut one, &mut two);
	}

	one.iter().filter(|c| c == &&Active).count()
}

fn solve2(input: &str) -> u64 {
	0
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cube {
	Active,
	Inactive,
}

fn v_add(rhs: (usize, usize, usize), lhs: (usize, usize, usize)) -> (usize, usize, usize) {
	(rhs.0.wrapping_add(lhs.0), rhs.1.wrapping_add(lhs.1), rhs.2.wrapping_add(lhs.2))
}
