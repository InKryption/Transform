mod vec2_impl {
	
	#[derive(Debug)]
	pub struct Vec2<XT = i32, YT = XT> {
		pub x: XT,
		pub y: YT,
	}

	mod ctrs {
		impl<XT, YT> super::Vec2<XT, YT> {
			pub fn new(x: XT, y: YT) -> Self {
				Self { x, y }
			}
		}

		impl<T> super::Vec2<T, T> {
			pub fn iso(n: T) -> Self
			where
				T: Copy,
			{
				Self { x: n, y: n }
			}
		}

		impl<XT, YT> std::default::Default for super::Vec2<XT, YT>
		where
			XT: std::convert::From<u8>,
			YT: std::convert::From<u8>,
		{
			fn default() -> Self {
				Self {
					x: std::convert::From::from(0),
					y: std::convert::From::from(0),
				}
			}
		}

		impl<XT, YT> Clone for super::Vec2<XT, YT>
		where
			XT: Clone,
			YT: Clone,
		{
			fn clone(&self) -> Self {
				Self {
					x: self.x.clone(),
					y: self.y.clone(),
				}
			}
		}

		impl<XT, YT> Copy for super::Vec2<XT, YT>
		where
			XT: Copy,
			YT: Copy,
		{
		}
	}

	mod vec_ops {
		use super::Vec2;
		use std::ops::{Add, Mul};

		impl<XT, YT> Vec2<XT, YT>
		where
			XT: Mul,
			YT: Mul,
			<XT as Mul>::Output: Add<<YT as Mul>::Output>,
		{
			pub fn dot(
				self,
				other: Self,
			) -> <<XT as Mul>::Output as Add<<YT as Mul>::Output>>::Output {
				(self.x * other.x) + (self.y * other.y)
			}

			pub fn mag2(self) -> <<XT as Mul>::Output as Add<<YT as Mul>::Output>>::Output
			where
				Self: Clone,
			{
				self.clone().dot(self)
			}

			pub fn mag(self) -> f64
			where
				<<XT as Mul>::Output as Add<<YT as Mul>::Output>>::Output: std::convert::Into<f64>,
				Self: Clone,
			{
				(self.mag2().into()).sqrt()
			}
		}
	}

	mod addition {
		use super::Vec2;
		use std::ops::{Add, AddAssign};

		impl<XT, YT> Add for Vec2<XT, YT>
		where
			XT: Add,
			YT: Add,
		{
			type Output = super::Vec2<XT::Output, YT::Output>;
			fn add(self, rhs: Self) -> Self::Output {
				Self::Output {
					x: self.x + rhs.x,
					y: self.y + rhs.y,
				}
			}
		}

		impl<XT, YT> AddAssign for Vec2<XT, YT>
		where
			XT: AddAssign,
			YT: AddAssign,
		{
			fn add_assign(&mut self, rhs: Self) {
				self.x += rhs.x;
				self.y += rhs.y;
			}
		}
	}

	mod subtraction {
		use super::Vec2;
		use std::ops::{Neg, Sub, SubAssign};

		impl<XT, YT> Neg for Vec2<XT, YT>
		where
			XT: Neg,
			YT: Neg,
		{
			type Output = Vec2<XT::Output, YT::Output>;
			fn neg(self) -> Self::Output {
				Self::Output {
					x: -self.x,
					y: -self.y,
				}
			}
		}

		impl<XT, YT> Sub for Vec2<XT, YT>
		where
			XT: Sub,
			YT: Sub,
		{
			type Output = Vec2<XT::Output, YT::Output>;
			fn sub(self, rhs: Self) -> Self::Output {
				Self::Output {
					x: self.x - rhs.x,
					y: self.y - rhs.y,
				}
			}
		}

		impl<XT, YT> SubAssign for Vec2<XT, YT>
		where
			XT: SubAssign,
			YT: SubAssign,
		{
			fn sub_assign(&mut self, rhs: Self) {
				self.x -= rhs.x;
				self.y -= rhs.y;
			}
		}
	}

	mod multiplication {
		use super::Vec2;
		use std::ops::{Mul, MulAssign};

		impl<XT, YT> Mul for Vec2<XT, YT>
		where
			XT: Mul,
			YT: Mul,
		{
			type Output = Vec2<XT::Output, YT::Output>;
			fn mul(self, rhs: Self) -> Self::Output {
				Self::Output {
					x: self.x * rhs.x,
					y: self.y * rhs.y,
				}
			}
		}

		impl<XT, YT> MulAssign for Vec2<XT, YT>
		where
			XT: MulAssign,
			YT: MulAssign,
		{
			fn mul_assign(&mut self, rhs: Self) {
				self.x *= rhs.x;
				self.y *= rhs.y;
			}
		}
	}

	mod division {
		use super::Vec2;
		use std::ops::{Div, DivAssign};

		impl<XT, YT> Div for Vec2<XT, YT>
		where
			XT: Div,
			YT: Div,
		{
			type Output = Vec2<XT::Output, YT::Output>;
			fn div(self, rhs: Self) -> Self::Output {
				Self::Output {
					x: self.x / rhs.x,
					y: self.y / rhs.y,
				}
			}
		}

		impl<XT, YT> DivAssign for Vec2<XT, YT>
		where
			XT: DivAssign,
			YT: DivAssign,
		{
			fn div_assign(&mut self, rhs: Self) {
				self.x /= rhs.x;
				self.y /= rhs.y;
			}
		}
	}

	mod remainder {
		use super::Vec2;
		use std::ops::{Rem, RemAssign};

		impl<XT, YT> Rem for Vec2<XT, YT>
		where
			XT: Rem,
			YT: Rem,
		{
			type Output = Vec2<XT::Output, YT::Output>;
			fn rem(self, rhs: Self) -> Self::Output {
				Self::Output {
					x: self.x % rhs.x,
					y: self.y % rhs.y,
				}
			}
		}

		impl<XT, YT> RemAssign for Vec2<XT, YT>
		where
			XT: RemAssign,
			YT: RemAssign,
		{
			fn rem_assign(&mut self, rhs: Self) {
				self.x %= rhs.x;
				self.y %= rhs.y;
			}
		}
	}

	mod display_impl {
		use super::Vec2;
		use std::fmt;
		
		impl<XT, YT> fmt::Display for Vec2<XT, YT>
		where
			XT: fmt::Display,
			YT: fmt::Display,
		{
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				write!(f, "{} {}", self.x, self.y)
			}
		}
	}
}

pub use vec2_impl::Vec2;
