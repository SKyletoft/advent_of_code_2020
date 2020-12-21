use std::{collections::HashMap, num::ParseIntError, str::FromStr};

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

	let mut image =
		vec![vec![false; (y_max - y_min + 1) as usize * 8]; (x_max - x_min + 1) as usize * 8];
	for x in x_min..=x_max {
		for y in y_min..=y_max {
			let id = map[&(x, y)].id;
			let var = map[&(x, y)].variation;
			let base_x = (x - x_min) as usize * 8;
			let base_y = (y - y_min) as usize * 8;
			let tile = raw
				.iter()
				.skip_while(|l| l.get(5..9).map(|x| x.parse()) != Some(Ok(id)))
				.skip(2)
				.take(8)
				.map(|l| l[1..9].bytes().map(|c| c == b'#').collect::<Vec<bool>>())
				.collect::<Vec<_>>();
			let rotated = rotate_and_mirror(&tile, var);

			for (t_x, t_y, v) in rotated
				.iter()
				.enumerate()
				.flat_map(|(x, l)| l.iter().enumerate().map(move |(y, v)| (y, x, v)))
			{
				image[base_y + t_y][base_x + t_x] = *v;
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
		.map(|v| rotate_and_mirror(&image, v))
		.map(|image| {
			(0..image.len() - 3)
				.map(|y| {
					(0..image[y].len() - 20)
						.filter(|&x| {
							(0..3).all(|o| {
								image[y + o][x..x + 20]
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

	let blocks = image
		.iter()
		.map(|v| v.iter().filter(|&&b| b).count() as i32)
		.sum::<i32>();

	blocks - 15 * sum
}

fn rotate_and_mirror(tile: &[Vec<bool>], var: u8) -> Vec<Vec<bool>> {
	match var {
		0 => mirrorv(tile),
		1 => mirrorv(&mirrorh(tile)),
		2 => mirrorv(&rotate(tile)),
		3 => mirrorv(&mirrorh(&rotate(tile))),
		4 => mirrorv(&rotate(&rotate(tile))),
		5 => mirrorv(&mirrorh(&rotate(&rotate(tile)))),
		6 => mirrorv(&rotate(&rotate(&rotate(tile)))),
		7 => mirrorv(&mirrorh(&rotate(&rotate(&rotate(tile))))),
		_ => unreachable!(),
	}
}

fn rotate(v: &[Vec<bool>]) -> Vec<Vec<bool>> {
	let mut u = vec![vec![false; v.len()]; v[0].len()];
	for (x, y, b) in v
		.iter()
		.enumerate()
		.flat_map(|(x, slice)| slice.iter().enumerate().map(move |(y, &b)| (x, y, b)))
	{
		u[y][v.len() - x - 1] = b;
	}
	u
}
fn mirrorh(v: &[Vec<bool>]) -> Vec<Vec<bool>> {
	let mut u = vec![vec![false; v.len()]; v[0].len()];
	for (x, y, b) in v
		.iter()
		.enumerate()
		.flat_map(|(x, slice)| slice.iter().enumerate().map(move |(y, &b)| (x, y, b)))
	{
		u[x][v.len() - y - 1] = b;
	}
	u
}
fn mirrorv(v: &[Vec<bool>]) -> Vec<Vec<bool>> {
	let mut u = vec![vec![false; v.len()]; v[0].len()];
	for (x, y, b) in v
		.iter()
		.enumerate()
		.flat_map(|(x, slice)| slice.iter().enumerate().map(move |(y, &b)| (x, y, b)))
	{
		u[v.len() - x - 1][y] = b;
	}
	u
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
		let spaces = map
			.keys()
			.flat_map(|c| surrounding(*c).into_iter())
			.filter(|c| !map.contains_key(c))
			.collect::<Vec<_>>();

		for &space in spaces.iter() {
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
