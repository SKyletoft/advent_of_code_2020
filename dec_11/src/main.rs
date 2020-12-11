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
		let index = self.w.wrapping_mul(y).wrapping_add(x);
		self.val.get(index).map(|&x| x)
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
					Floor => '.'
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
				.map(|&(x, y)| (x.wrapping_add(width), y.wrapping_add(height)))
				.filter(|&(x, y)| x < from.w && y < from.h)
				.map(|(x, y)| from.get(x,y))
				.filter(|&x| x == Some(Occupied))
				.count();
			to.set(
				width,
				height,
				match neighbours {
					0 if seat == Empty => Occupied,
					n if n >= 4 && seat == Occupied => Empty,
					_ => seat,
				},
			);
		}
	}
}

fn interpret(input: &str) -> Matrix {
	let width = input.lines().next().unwrap().len();
	let height = input.lines().count();
	let mut m = Matrix {
		w: width,
		h: height,
		val: Vec::with_capacity(width * height),
	};
	input
		.bytes()
		.filter(|&x| x == b'#' || x == b'.' || x == b'L')
		.map(|x| match x {
			b'#' => Occupied,
			b'.' => Floor,
			b'L' => Empty,
			_ => unreachable!(),
		})
		.for_each(|s| m.val.push(s));
	m
}

fn solve1(m: &Matrix) -> usize {
	let mut one = m.clone();
	let mut two = m.clone();
	dbg!(&one);
	two.set(0, 0, Occupied);
	while one != two {
		step(&one, &mut two);
		swap(&mut one, &mut two);
		//dbg!(&one);
	}
	one.val.iter().filter(|&&s| s == Occupied).count()
}

fn solve2(m: &Matrix) -> usize {
	let mut one = m.clone();
	let mut two = m.clone();
	//dbg!(&one);
	two.set(0, 0, Occupied);
	while one != two {
		step_diagonal(&one, &mut two);
		swap(&mut one, &mut two);
		//dbg!(&one);
	}
	one.val.iter().filter(|&&s| s == Occupied).count()
}

fn step_diagonal(from: &Matrix, to: &mut Matrix) {
	for height in 0..from.h {
		for width in 0..from.w {
			let seat = from.get(width, height).unwrap();
			let mut neighbours = 0;
			for &(del_x, del_y) in NEIGHBOURING.iter() {
				let mut x = width.wrapping_add(del_x);
				let mut y = height.wrapping_add(del_y);
				while x < from.w && y < from.h {
					let vis_seat = from.get(x, y).unwrap();
					if vis_seat == Occupied {
						neighbours += 1;
						break;
					}
					if vis_seat == Empty {
						break;
					}
					x = x.wrapping_add(del_x);
					y = y.wrapping_add(del_y);
				}

			}
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