use std::mem::swap;

use Seat::*;

const NEIGHBOURING: [(usize, usize); 8] = [
	(usize::MAX, usize::MAX),
	(usize::MAX, 0),
	(usize::MAX, 1),
	(0, usize::MAX),
	(0, 1),
	(1, usize::MAX),
	(1, 0),
	(1, 1),
];

fn main() {
	let input = include_str!("input.txt");
	let starting_layout = interpret(&input);
	let sol1 = solve1(&starting_layout);
	let sol2 = solve2(&starting_layout);
	println!("{} {}", sol1, sol2);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Seat {
	Floor,
	Empty,
	Occupied,
}

#[derive(Clone, PartialEq)]
struct Matrix {
	h: usize,
	w: usize,
	val: Vec<Seat>,
}

impl Matrix {
	fn get(&self, x: usize, y: usize) -> Option<Seat> {
		if x >= self.w || y >= self.h {
			return None;
		}
		let index = self.w.wrapping_mul(y).wrapping_add(x);
		self.val.get(index).copied()
	}
	fn set(&mut self, x: usize, y: usize, val: Seat) {
		assert!(x < self.w);
		assert!(y < self.h);
		self.val[self.w * y + x] = val;
	}
}

impl std::fmt::Debug for Matrix {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		writeln!(f)?;
		for line in 0..self.h {
			let from = line * self.w;
			let to = (line + 1) * self.w;
			for &seat in &self.val[from..to] {
				let print = match seat {
					Occupied => '#',
					Empty => 'L',
					Floor => '.',
				};
				write!(f, "{}", print)?;
			}
			writeln!(f)?;
		}
		write!(f, "")
	}
}

fn step(from: &Matrix, to: &mut Matrix) {
	for height in 0..from.h {
		for width in 0..from.w {
			let seat = from.get(width, height).unwrap();
			let neighbours = NEIGHBOURING
				.iter()
				.map(|&(x, y)| from.get(x.wrapping_add(width), y.wrapping_add(height)))
				.filter(|&x| x == Some(Occupied))
				.count();
			to.set(
				width,
				height,
				match (neighbours, seat) {
					(0, Empty) => Occupied,
					(n, Occupied) if n >= 4 => Empty,
					_ => seat,
				},
			);
		}
	}
}

fn step_diagonal(from: &Matrix, to: &mut Matrix) {
	for height in 0..from.h {
		for width in 0..from.w {
			let seat = from.get(width, height).unwrap();
			let neighbours = NEIGHBOURING
				.iter()
				.map(|&(del_x, del_y)| {
					(1..)
						.map(|n| from.get(width + n * del_x, height + n * del_y))
						.find(|&x| x != Some(Floor))
						.flatten()
				})
				.filter(|&x| x == Some(Occupied))
				.count();
			to.set(
				width,
				height,
				match (neighbours, seat) {
					(0, Empty) => Occupied,
					(n, Occupied) if n >= 5 => Empty,
					_ => seat,
				},
			);
		}
	}
}

fn interpret(input: &str) -> Matrix {
	let width = input.lines().next().unwrap().len();
	let height = input.lines().count();
	Matrix {
		w: width,
		h: height,
		val: input
			.bytes()
			.filter(|&x| x == b'#' || x == b'.' || x == b'L')
			.map(|x| match x {
				b'#' => Occupied,
				b'.' => Floor,
				b'L' => Empty,
				_ => unreachable!(),
			})
			.collect(),
	}
}

fn solve1(m: &Matrix) -> usize {
	let mut one = m.clone();
	let mut two = m.clone();
	two.set(0, 0, Occupied);
	while one != two {
		step(&one, &mut two);
		swap(&mut one, &mut two);
	}
	one.val.iter().filter(|&&s| s == Occupied).count()
}

fn solve2(m: &Matrix) -> usize {
	let mut one = m.clone();
	let mut two = m.clone();
	two.set(0, 0, Occupied);
	while one != two {
		step_diagonal(&one, &mut two);
		swap(&mut one, &mut two);
	}
	one.val.iter().filter(|&&s| s == Occupied).count()
}
