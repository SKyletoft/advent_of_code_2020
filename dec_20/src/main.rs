use std::{collections::HashMap, num::ParseIntError, str::FromStr, unreachable};

pub mod matrix;
use matrix::Matrix;

fn main() {
	let input = include_str!("input.txt");
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &str) -> u64 {
	let map = solve_map(input);

	let x_max = map.keys().map(|&(x, _)| x).max().unwrap();
	let x_min = map.keys().map(|&(x, _)| x).min().unwrap();
	let y_max = map.keys().map(|&(_, y)| y).max().unwrap();
	let y_min = map.keys().map(|&(_, y)| y).min().unwrap();

	let a = map[&(x_max, y_max)].id as u64;
	let b = map[&(x_min, y_max)].id as u64;
	let c = map[&(x_max, y_min)].id as u64;
	let d = map[&(x_min, y_min)].id as u64;
	a * b * c * d
}
fn solve2(input: &str) -> i32 {
	let map = solve_map(input);
	let x_max = map.keys().map(|&(x, _)| x).max().unwrap();
	let x_min = map.keys().map(|&(x, _)| x).min().unwrap();
	let y_max = map.keys().map(|&(_, y)| y).max().unwrap();
	let y_min = map.keys().map(|&(_, y)| y).min().unwrap();

	let raw = input.lines().collect::<Vec<_>>();

	let height = (y_max - y_min + 1) as usize * 8;
	let width = (x_max - x_min + 1) as usize * 8;
	let mut image = Matrix::new(height, width, vec![false; height * width]);
	for x in x_min..=x_max {
		for y in y_min..=y_max {
			let id = map[&(x, y)].id;
			let var = map[&(x, y)].variation;
			let base_x = (x - x_min) as usize * 8;
			let base_y = (y - y_min) as usize * 8;
			let mut tile = Matrix::new(
				8,
				8,
				raw.iter()
					.skip_while(|l| l.get(5..9).map(|x| x.parse()) != Some(Ok(id)))
					.skip(2)
					.take(8)
					.flat_map(|l| l[1..9].bytes().map(|c| c == b'#'))
					.collect::<Vec<_>>(),
			);
			tile.rotate_and_mirror(var);

			let width = tile.width();
			let height = tile.height();
			for (t_y, t_x, v) in tile
				.raw()
				.iter()
				.enumerate()
				.map(|(i, v)| (i / width, i % height, v))
			{
				image[(base_x + t_x, base_y + t_y)] = *v;
			}
		}
	}

	let pattern: [[bool; 20]; 3] = [
		[
			false, false, false, false, false, false, false, false, false, false, false, false,
			false, false, false, false, false, false, true, false,
		],
		[
			true, false, false, false, false, true, true, false, false, false, false, true, true,
			false, false, false, false, true, true, true,
		],
		[
			false, true, false, false, true, false, false, true, false, false, true, false, false,
			true, false, false, true, false, false, false,
		],
	];
	let sum = (0..8)
		.map(|v| {
			let mut clone = image.clone();
			clone.rotate_and_mirror(v);
			(0..clone.width() - 3)
				.map(|y| {
					(0..clone.height() - 20)
						.filter(|&x| {
							(0..3).all(|o| {
								let offset = clone.width() * (y + o) + x;
								clone.raw()[offset..offset + 20]
									.iter()
									.zip(pattern[o].iter())
									.all(|(&a, &b)| a || !b)
							})
						})
						.count() as i32
				})
				.sum::<i32>()
		})
		.sum::<i32>();

	let blocks = image.raw().iter().filter(|&&b| b).count() as i32;

	blocks - 15 * sum
}

fn solve_map(input: &str) -> HashMap<(i32, i32), Tile> {
	let mut all_tiles = input
		.split("\n\n")
		.filter(|l| !l.is_empty())
		.map(|l| l.parse::<Tile>().unwrap())
		.map(|t| (0..8).map(move |v| t.variation(v)))
		.flatten()
		.collect::<Vec<_>>();
	let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
	let to_insert = all_tiles.swap_remove(0);
	all_tiles.retain(|t| t.id != to_insert.id);
	map.insert((0, 0), to_insert);

	while !all_tiles.is_empty() {
		for space in map
			.keys()
			.flat_map(|c| surrounding(*c).into_iter())
			.filter(|c| !map.contains_key(c))
		{
			if let [right, left, above, below] = &surrounding(space)[..4] {
				let r_req = map.get(right).map(|t| t.left);
				let l_req = map.get(left).map(|t| t.right);
				let t_req = map.get(above).map(|t| t.bottom);
				let b_req = map.get(below).map(|t| t.top);
				let mut i = all_tiles.iter().filter(|t| {
					r_req.map(|r| r == t.right) != Some(false)
						&& l_req.map(|r| r == t.left) != Some(false)
						&& t_req.map(|r| r == t.top) != Some(false)
						&& b_req.map(|r| r == t.bottom) != Some(false)
				});
				if let (Some(t), None) = (i.next(), i.next()) {
					let next_tile = *t;
					all_tiles.retain(|t| t.id != next_tile.id);
					map.insert(space, next_tile);
					break;
				}
			} else {
				unreachable!();
			}
		}
	}
	map
}

fn surrounding((x, y): (i32, i32)) -> Vec<(i32, i32)> {
	vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Tile {
	id: u16,
	top: u16,
	bottom: u16,
	left: u16,
	right: u16,
	variation: u8,
}

impl Tile {
	fn variation(self, v: usize) -> Self {
		let mut new = [
			self,
			self.mirror(),
			self.rotate_by(1),
			self.rotate_by(1).mirror(),
			self.rotate_by(2),
			self.rotate_by(2).mirror(),
			self.rotate_by(3),
			self.rotate_by(3).mirror(),
		][v % 8];
		new.variation = (v % 8) as u8;
		new
	}
	fn rotate_by(self, step: u8) -> Self {
		(0..step).fold(self, |acc, _| acc.rotate())
	}
	fn rotate(self) -> Self {
		Tile {
			id: self.id,
			top: self.left.reverse_bits() >> 6,
			right: self.top,
			bottom: self.right.reverse_bits() >> 6,
			left: self.bottom,
			variation: self.variation,
		}
	}
	fn mirror(self) -> Self {
		Tile {
			id: self.id,
			top: self.top.reverse_bits() >> 6,
			right: self.left,
			bottom: self.bottom.reverse_bits() >> 6,
			left: self.right,
			variation: self.variation,
		}
	}
}

impl FromStr for Tile {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let rows = s.lines().collect::<Vec<_>>();
		let id = rows[0][5..9].parse()?;
		let top = rows[1]
			.bytes()
			.fold(0, |acc, curr| (acc << 1) + (curr == b'#') as u16);
		let bottom = rows[10]
			.bytes()
			.fold(0, |acc, curr| (acc << 1) + (curr == b'#') as u16);
		let left = rows
			.iter()
			.skip(1)
			.map(|l| l.as_bytes()[0])
			.fold(0, |acc, curr| (acc << 1) + (curr == b'#') as u16);
		let right = rows
			.iter()
			.skip(1)
			.map(|l| l.as_bytes()[9])
			.fold(0, |acc, curr| (acc << 1) + (curr == b'#') as u16);
		Ok(Tile {
			id,
			top,
			bottom,
			left,
			right,
			variation: 0,
		})
	}
}
