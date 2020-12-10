fn main() {
	let input = include_str!("input.txt")
		.lines()
		.map(|l| l.parse().unwrap())
		.collect::<Vec<u64>>();
	let sol1 = solve1(&input);
	let sol2 = solve2(&input);
	println!("{} {}", sol1, sol2);
}

fn solve1(input: &[u64]) -> u64 {
	let max = *input.iter().max().unwrap() + 3;
	let mut owned = input.to_vec();
	owned.push(max);
	owned.sort_unstable();
	let ((one, three), _) = owned
		.iter()
		.fold(((0, 0), 0), |((ones, threes), prev), &curr| {
			match curr - prev {
				2 => panic!(),
				0 => ((ones, threes), curr),
				1 => ((ones + 1, threes), curr),
				3 => ((ones, threes + 1), curr),
				_ => panic!(),
			}
		});
	one * three
}

fn solve2(input: &[u64]) -> u64 {
	let mut input = input.to_vec();
	let max = *input.iter().max().unwrap();
	input.insert(0, 0);
	input.push(max + 3);
	input.sort_unstable();
	let base_matrix = base_matrix(&input);
	let mut wip = base_matrix.clone();
	let mut sum = 0;
	for _ in 0..input.len() {
		let clone = base_matrix.clone();
		wip = wip * clone;
		sum += wip.get(0, wip.h - 1);
	}
	sum
}

#[derive(Debug, Clone, PartialEq)]
struct Matrix {
	h: usize,
	w: usize,
	val: Vec<u64>,
}

impl Matrix {
	fn get(&self, x: usize, y: usize) -> u64 {
		assert!(x < self.w);
		assert!(y < self.h);
		self.val[self.w * y + x]
	}
	fn set(&mut self, x: usize, y: usize, val: u64) {
		assert!(x < self.w);
		assert!(y < self.h);
		self.val[self.w * y + x] = val;
	}
}

impl std::ops::Mul for Matrix {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self {
		let lhs = self;
		assert!(lhs.w == rhs.h);
		let mut new = Matrix {
			h: lhs.h,
			w: rhs.w,
			val: Vec::with_capacity(lhs.h * rhs.w),
		};
		for y in 0..new.h {
			for x in 0..new.w {
				new.val.push(
					lhs.val
						.iter()
						.skip(y * lhs.w)
						.take(lhs.w)
						.zip(rhs.val.iter().skip(x).step_by(rhs.w))
						.map(|(a, b)| a * b)
						.sum::<u64>(),
				);
			}
		}
		new
	}
}

fn base_matrix(input: &[u64]) -> Matrix {
	let mut out: Matrix = Matrix {
		h: input.len(),
		w: input.len(),
		val: Vec::with_capacity(input.len().pow(2)),
	};
	unsafe {
		out.val.set_len(input.len().pow(2));
	}
	for i in 0..input.len() {
		for j in (1 + i)..input.len() {
			if input[j] <= input[i] + 3 {
				out.set(i, j, 1);
			} else {
				out.set(i, j, 0);
			}
		}
	}
	out
}

#[allow(dead_code)]
fn print_matrix(m: &Matrix) {
	for line in 0..m.h {
		println!("{:?}", &m.val[m.w * line..m.w * (line + 1)])
	}
}
