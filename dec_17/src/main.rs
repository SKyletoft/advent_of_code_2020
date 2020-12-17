use std::mem;
use Cube::*;

const TIME_STEPS: usize = 6;
const NEG_ONE: usize = (-1isize) as usize;
const NEIGHBOURING: [usize; 3] = [NEG_ONE, 0, 1];
fn main() {
	let input = include_str!("input.txt");
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &str) -> usize {
	let height = input.lines().count() + TIME_STEPS * 2;
	let width = input.lines().next().map(|l| l.len()).unwrap_or(0) + TIME_STEPS * 2;
	let depth = 2 * TIME_STEPS + 1;
	let convert_index = |(x, y, z): (usize, usize, usize)| {
		if x < width && y < height && z < depth {
			Some(x + y * width + z * width * height)
		} else {
			None
		}
	};
	let v_add = |rhs: (usize, usize, usize), lhs: (usize, usize, usize)| {
		(
			rhs.0.wrapping_add(lhs.0),
			rhs.1.wrapping_add(lhs.1),
			rhs.2.wrapping_add(lhs.2),
		)
	};
	let mut one = vec![Inactive; width * height * depth];
	let mut two = one.clone();

	for (x, line) in input.lines().enumerate() {
		for (y, c) in line.bytes().enumerate() {
			if c == b'#' {
				one[convert_index((x + TIME_STEPS, y + TIME_STEPS, TIME_STEPS)).unwrap()] = Active;
			}
		}
	}

	for _ in 0..TIME_STEPS {
		for (z, y, x) in iterator_3d((depth, height, width)) {
			let idx = convert_index((x, y, z)).unwrap();
			let curr = *one.get(idx).unwrap();
			let neighbours = iterator_3d((3, 3, 3))
				.map(|(a, b, c)| (NEIGHBOURING[a], NEIGHBOURING[b], NEIGHBOURING[c]))
				.filter(|&v| v != (0, 0, 0))
				.map(|v| convert_index(v_add(v, (x, y, z))).and_then(|idx| one.get(idx)))
				.filter(|v| v == &Some(&Active))
				.count();
			let next = match (curr, neighbours) {
				(Active, 2) | (Active, 3) => Active,
				(Inactive, 3) => Active,
				_ => Inactive,
			};
			*two.get_mut(idx).unwrap() = next;
		}
		mem::swap(&mut one, &mut two);
	}

	one.iter().filter(|c| c == &&Active).count()
}

fn solve2(input: &str) -> usize {
	let height = input.lines().count() + 2 * TIME_STEPS;
	let width = input.lines().next().map(|l| l.len()).unwrap_or(0) + 2 * TIME_STEPS;
	let depth = 2 * TIME_STEPS + 1;
	let convert_index = |(x, y, z, w): (usize, usize, usize, usize)| {
		if x < width && y < height && z < depth && w < depth {
			Some(x + y * width + z * width * height + w * width * height * depth)
		} else {
			None
		}
	};
	let v_add = |rhs: (usize, usize, usize, usize), lhs: (usize, usize, usize, usize)| {
		(
			rhs.0.wrapping_add(lhs.0),
			rhs.1.wrapping_add(lhs.1),
			rhs.2.wrapping_add(lhs.2),
			rhs.3.wrapping_add(lhs.3),
		)
	};
	let mut one = vec![Inactive; width * height * depth * depth];
	let mut two = one.clone();

	for (x, line) in input.lines().enumerate() {
		for (y, c) in line.bytes().enumerate() {
			if c == b'#' {
				one[convert_index((x + 6, y + 6, 6, 6)).unwrap()] = Active;
			}
		}
	}

	for _ in 0..TIME_STEPS {
		for (w, z, y, x) in iterator_4d((depth, depth, height, width)) {
			let idx = convert_index((x, y, z, w)).unwrap();
			let curr = *one.get(idx).unwrap();

			let neighbours = iterator_4d((3, 3, 3, 3))
				.map(|(a, b, c, d)| {
					(
						NEIGHBOURING[a],
						NEIGHBOURING[b],
						NEIGHBOURING[c],
						NEIGHBOURING[d],
					)
				})
				.filter(|&v| v != (0, 0, 0, 0))
				.map(|v| convert_index(v_add(v, (x, y, z, w))).and_then(|idx| one.get(idx)))
				.filter(|v| v == &Some(&Active))
				.count();
			let next = match (curr, neighbours) {
				(Active, 2) | (Active, 3) => Active,
				(Inactive, 3) => Active,
				_ => Inactive,
			};
			*two.get_mut(idx).unwrap() = next;
		}
		mem::swap(&mut one, &mut two);
	}

	one.iter().filter(|c| c == &&Active).count()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cube {
	Active,
	Inactive,
}

fn iterator_3d((x, y, z): (usize, usize, usize)) -> impl Iterator<Item = (usize, usize, usize)> {
	(0..x)
		.map(move |a| (0..y).map(move |b| (0..z).map(move |c| (a, b, c))))
		.flatten()
		.flatten()
}

fn iterator_4d(
	(x, y, z, w): (usize, usize, usize, usize),
) -> impl Iterator<Item = (usize, usize, usize, usize)> {
	(0..x)
		.map(move |a| (0..y).map(move |b| (0..z).map(move |c| (0..w).map(move |d| (a, b, c, d)))))
		.flatten()
		.flatten()
		.flatten()
}
