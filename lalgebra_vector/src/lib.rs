use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

use std::ops::Add;


impl<T: Scalar<Item = T> + Clone> Add for Vector<T> {
	type Output = Option<Vector<T>>;
	fn add(self, rhs: Self) -> Self::Output {
		if self.0.len() == 0 || self.0.len() != rhs.0.len() {
			return None;
		}
		let mut result = Vector::new();
		self.0.iter().enumerate().for_each(|(i, v)| {
			result.0.push(v.to_owned() + rhs.0[i].to_owned())
		});
		Some(
			result
		)
	}
}

impl<T: Scalar<Item = T> + Clone>  Vector<T> {
	pub fn new() -> Self {
		Self(vec![])
	}

	pub fn dot(&self, other: &Self) -> Option<T> {
		if self.0.len() == 0 || self.0.len() != other.0.len() {
			return None;
		}
		let mut result = Vector::new();
		self.0.iter().enumerate().for_each(|(i, v)| {
			result.0.push(v.to_owned() * other.0[i].to_owned())
		});
		Some(
			result.0.into_iter().fold(T::zero(), |acc, v| {
				acc + v
			})
		)
	}
}