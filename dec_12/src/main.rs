use Instruction::*;

fn main() {
	let input = include_str!("input.txt")
		.lines()
		.map(parse)
		.collect::<Vec<_>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
	Vertical(i32),
	Horizontal(i32),
	Turn(i32),
	Forwards(i32),
}

fn parse(input: &str) -> Instruction {
	let val = input[1..].parse().unwrap();
	match input.as_bytes()[0] {
		b'N' => Vertical(val),
		b'S' => Vertical(-val),
		b'E' => Horizontal(val),
		b'W' => Horizontal(-val),
		b'L' => Turn(val),
		b'R' => Turn(-val),
		b'F' => Forwards(val),
		_ => unreachable!(),
	}
}

fn solve1(input: &[Instruction]) -> usize {
	let mut x: i32 = 0;
	let mut y: i32 = 0;
	let mut rot = 0.;

	for &ins in input.iter() {
		match ins {
			Vertical(v) => {
				y += v;
			}
			Horizontal(v) => {
				x += v;
			}
			Turn(v) => {
				rot += v as f32;
				rot %= 360.;
			}
			Forwards(v) => {
				let dx = rot.to_radians().cos() as i32 * v;
				let dy = rot.to_radians().sin() as i32 * v;
				x += dx;
				y += dy;
			}
		}
	}
	(x.abs() + y.abs()) as usize
}

fn solve2(input: &[Instruction]) -> usize {
	let mut x: i32 = 10;
	let mut y: i32 = 1;
	let mut sx: i32 = 0;
	let mut sy: i32 = 0;

	for &ins in input.iter() {
		match ins {
			Vertical(v) => {
				y += v;
			}
			Horizontal(v) => {
				x += v;
			}
			Turn(v) => {
				let t = (v as f32).to_radians();
				let x1 = t.cos() * x as f32;
				let x2 = -t.sin() * y as f32;
				let y1 = t.sin() * x as f32;
				let y2 = t.cos() * y as f32;
				x = (x1 + x2).round() as i32;
				y = (y1 + y2).round() as i32;
			}
			Forwards(v) => {
				sx += x * v;
				sy += y * v;
			}
		}
	}
	(sx.abs() + sy.abs()) as usize
}
