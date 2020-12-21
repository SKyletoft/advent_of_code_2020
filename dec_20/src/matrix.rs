use std::{
	mem,
	ops::{Index, IndexMut},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Matrix<T> {
	h: usize,
	w: usize,
	val: Vec<T>,
}

impl<T> Index<(usize, usize)> for Matrix<T> {
	type Output = T;

	fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
		self.get(x, y).unwrap()
	}
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
	fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
		self.get_mut(x, y).unwrap()
	}
}

impl<T> Matrix<T> {
	pub fn get(&self, x: usize, y: usize) -> Option<&T> {
		if x >= self.w || y >= self.h {
			None
		} else {
			self.val.get(self.w * y + x)
		}
	}
	pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
		if x >= self.w || y >= self.h {
			None
		} else {
			self.val.get_mut(self.w * y + x)
		}
	}

	pub fn set(&mut self, x: usize, y: usize, val: T) {
		assert!(x < self.w);
		assert!(y < self.h);
		*self.get_mut(x, y).unwrap() = val;
	}

	pub fn mirror(&mut self) -> &mut Self {
		for section in 0..self.h {
			mirror_line(&mut self.val[section * self.w..(section + 1) * self.w]);
		}
		self
	}

	pub fn mirror_v(&mut self) -> &mut Self {
		//So crazily unoptimised
		mirror_line(&mut self.val);
		for section in 0..self.h {
			mirror_line(&mut self.val[section * self.w..(section + 1) * self.w]);
		}
		self
	}

	pub fn transpose(&mut self) -> &mut Self
	where
		T: Clone,
	{
		assert_eq!(self.w, self.h);
		for x in 0..self.h {
			for y in 0..x {
				let tmp = self[(x, y)].clone();
				self[(x, y)] = self[(y, x)].clone();
				self[(y, x)] = tmp;
			}
		}
		self
	}

	pub fn rotate(&mut self) -> &mut Self
	where
		T: Clone,
	{
		self.transpose().mirror()
	}

	pub fn rotate_by(&mut self, steps: usize) -> &mut Self
	where
		T: Clone,
	{
		for _ in 0..(steps % 4) {
			self.rotate();
		}
		self
	}

	pub fn rotate_and_mirror(&mut self, var: u8) -> &mut Self
	where
		T: Clone,
	{
		match var % 8 {
			0 => self,
			1 => self.mirror(),
			2 => self.rotate(),
			3 => self.transpose(),
			4 => self.rotate_by(2),
			5 => self.rotate_by(2).mirror(),
			6 => self.rotate_by(3),
			7 => self.rotate_by(3).mirror(),
			_ => unreachable!(),
		}
		.mirror_v()
	}

	pub fn raw(&self) -> &[T] {
		&self.val
	}

	pub fn width(&self) -> usize {
		self.w
	}

	pub fn height(&self) -> usize {
		self.h
	}

	pub fn new(h: usize, w: usize, val: Vec<T>) -> Self {
		assert_eq!(h * w, val.len());
		Matrix { h, w, val }
	}
}

fn mirror_line<T>(l: &mut [T]) {
	let (lhs, rhs) = l.split_at_mut(l.len() / 2);
	lhs.iter_mut()
		.zip(rhs.iter_mut().rev())
		.for_each(|(lhs, rhs)| mem::swap(lhs, rhs));
}
