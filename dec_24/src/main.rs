use std::{collections::HashMap, mem, ops};

//Multiplied by 10 for safe rounding later
const EAST: Vector2 = Vector2 { x: 10., y: 0. };
const NORTH_EAST: Vector2 = Vector2 {
	x: 8.660254037844386,
	y: 5.,
}; //(3f64.sqrt() / 2f64, 1f64 / 2f64)
const N_ONE: usize = -1isize as usize;
const NEIGHBOURING: [(usize, usize); 6] = [
	(1, 0),
	(0, 1),
	(N_ONE, 1),
	(N_ONE, 0),
	(0, N_ONE),
	(1, N_ONE),
];
const SIZE: usize = 200;

fn main() {
	let input = include_str!("input.txt")
		.lines()
		.map(|l| l.as_bytes())
		.collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[&[u8]]) -> usize {
	let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
	for line in input.iter() {
		let mut line = *line;
		let mut vector = Vector2 { x: 0., y: 0. };
		while !line.is_empty() {
			let (l, u) = parse_line(line);
			line = l;
			vector = vector + u;
		}
		let x = vector.x.round() as i32;
		let y = vector.y.round() as i32;
		let rounded_vector = (x, y);
		let tile_ref = map.entry(rounded_vector).or_insert(White);
		let curr = *tile_ref;
		*tile_ref = if curr == Black { White } else { Black };
	}
	map.values().filter(|x| **x == Black).count()
}

fn solve2(input: &[&[u8]]) -> usize {
	let mut grid = [[White; SIZE]; SIZE];
	let mut other = [[White; SIZE]; SIZE];

	for line in input.iter() {
		let mut line = *line;
		let (mut x, mut y): (usize, usize) = (SIZE / 2, SIZE / 2);
		while !line.is_empty() {
			let (l, (dx, dy)) = parse_line_grid(line);
			line = l;
			x = x.wrapping_add(dx);
			y = y.wrapping_add(dy);
		}
		grid[x][y] = if grid[x][y] == Black { White } else { Black };
	}

	for _ in 0..100 {
		for y in 0..SIZE {
			for x in 0..SIZE {
				let hex = grid[x][y];
				let neighbours = NEIGHBOURING
					.iter()
					.map(|&(dx, dy)| {
						grid.get(dx.wrapping_add(x))
							.map(|l| l.get(dy.wrapping_add(y)))
					})
					.flatten()
					.filter(|&t| t == Some(&Black))
					.count() as u64;
				if hex == Black && (neighbours == 0 || neighbours > 2) {
					other[x][y] = White;
				} else if hex == White && neighbours == 2 {
					other[x][y] = Black;
				} else {
					other[x][y] = grid[x][y];
				}
			}
		}
		mem::swap(&mut grid, &mut other);
	}

	grid.iter()
		.flat_map(|l| l.iter())
		.filter(|&&t| t == Black)
		.count()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
	White,
	Black,
}
use Tile::*;

fn parse_line(line: &[u8]) -> (&[u8], Vector2) {
	match line {
		[b'e', ..] => (&line[1..], EAST),
		[b'w', ..] => (&line[1..], -EAST),
		[b'n', b'e', ..] => (&line[2..], NORTH_EAST),
		[b'n', b'w', ..] => (&line[2..], NORTH_EAST - EAST),
		[b's', b'e', ..] => (&line[2..], -NORTH_EAST + EAST),
		[b's', b'w', ..] => (&line[2..], -NORTH_EAST),
		_ => panic!(),
	}
}

fn parse_line_grid(line: &[u8]) -> (&[u8], (usize, usize)) {
	match line {
		[b'e', ..] => (&line[1..], NEIGHBOURING[0]),
		[b'w', ..] => (&line[1..], NEIGHBOURING[3]),
		[b'n', b'e', ..] => (&line[2..], NEIGHBOURING[1]),
		[b'n', b'w', ..] => (&line[2..], NEIGHBOURING[2]),
		[b's', b'e', ..] => (&line[2..], NEIGHBOURING[5]),
		[b's', b'w', ..] => (&line[2..], NEIGHBOURING[4]),
		_ => panic!(),
	}
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Vector2 {
	x: f64,
	y: f64,
}

impl ops::Add for Vector2 {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self {
			x: self.x + rhs.x,
			y: self.y + rhs.y,
		}
	}
}

impl ops::Mul<f64> for Vector2 {
	type Output = Self;

	fn mul(self, rhs: f64) -> Self::Output {
		Self {
			x: self.x * rhs,
			y: self.y * rhs,
		}
	}
}

impl ops::Neg for Vector2 {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self {
			x: -self.x,
			y: -self.y,
		}
	}
}

impl ops::Sub for Vector2 {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		self + (-rhs)
	}
}
