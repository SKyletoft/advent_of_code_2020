use std::mem::swap;
use Seat::*;

const NEG_ONE: usize = (-1isize) as usize;
const NEIGHBOURING: [(usize, usize); 8] = [
	(NEG_ONE, NEG_ONE),
	(NEG_ONE, 0),
	(NEG_ONE, 1),
	(0, NEG_ONE),
	(0, 1),
	(1, NEG_ONE),
	(1, 0),
	(1, 1),
];

fn main() {
	let input = interpret(include_str!("input.txt"));
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Seat {
	Floor,
	Empty,
	Occupied,
}

#[derive(Clone, PartialEq)]
struct Board {
	h: usize,
	w: usize,
	val: Vec<Seat>,
}

impl Board {
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

impl std::fmt::Debug for Board {
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

fn count_neighbouring(from: &Board, x: usize, y: usize) -> u64 {
	NEIGHBOURING
		.iter()
		.map(|&(dx, dy)| from.get(dx.wrapping_add(x), dy.wrapping_add(y)))
		.filter(|&t| t == Some(Occupied))
		.count() as u64
}

fn count_line_of_sight(from: &Board, x: usize, y: usize) -> u64 {
	NEIGHBOURING
		.iter()
		.flat_map(|&(dx, dy)| {
			(1..)
				.map(|n| from.get(x + n * dx, y + n * dy))
				.find(|&x| x != Some(Floor))
		})
		.filter(|&x| x == Some(Occupied))
		.count() as u64
}

fn step(from: &Board, to: &mut Board, how: fn(from: &Board, x: usize, y: usize) -> u64, req: u64) {
	for y in 0..from.h {
		for x in 0..from.w {
			let seat = from.get(x, y).unwrap();
			if seat == Floor {
				continue;
			}
			let neighbours = how(from, x, y);
			to.set(
				x,
				y,
				match (neighbours, seat) {
					(0, Empty) => Occupied,
					(n, Occupied) if n >= req => Empty,
					_ => seat,
				},
			);
		}
	}
}

fn interpret(input: &str) -> Board {
	let x = input.lines().next().unwrap().len();
	let y = input.lines().count();
	Board {
		w: x,
		h: y,
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

fn solve1(m: &Board) -> u64 {
	let mut one = m.clone();
	let mut two = m.clone();
	two.set(0, 0, Occupied);
	while one != two {
		step(&one, &mut two, count_neighbouring, 4);
		swap(&mut one, &mut two);
	}
	one.val.iter().filter(|&&s| s == Occupied).count() as u64
}

fn solve2(m: &Board) -> u64 {
	let mut one = m.clone();
	let mut two = m.clone();
	two.set(0, 0, Occupied);
	while one != two {
		step(&one, &mut two, count_line_of_sight, 5);
		swap(&mut one, &mut two);
	}
	one.val.iter().filter(|&&s| s == Occupied).count() as u64
}
