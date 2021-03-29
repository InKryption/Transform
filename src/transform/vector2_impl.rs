mod vector2_impl {
	#[allow(dead_code)]
	#[derive(Debug)]
	pub struct Vector2<XT = i32, YT = XT> {
		pub x: XT,
		pub y: YT,
	}

	pub mod ctrs {
		use super::Vector2;

		impl<XT, YT> Vector2<XT, YT> {
			/// Construct a vector, specifying the x and y axies.
			pub fn new(x: XT, y: YT) -> Self {
				Self { x, y }
			}
		}

		impl<T> Vector2<T, T> {
			/// Construct a vector both of whose axies are 'n'.
			pub fn from(n: T) -> Self
			where
				T: Clone,
			{
				Self { x: n.clone(), y: n }
			}
		}

		impl<XT, YT> std::default::Default for Vector2<XT, YT>
		where
			XT: std::default::Default,
			YT: std::default::Default,
		{
			fn default() -> Self {
				Self {
					x: Default::default(),
					y: Default::default(),
				}
			}
		}

		impl<XT, YT> Clone for Vector2<XT, YT>
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

		impl<XT, YT> Copy for super::Vector2<XT, YT>
		where
			XT: Copy,
			YT: Copy,
		{
		}
	}

	pub mod vec_ops {
		use super::Vector2;
		use std::ops::{Add, Mul};

		impl<XT, YT> Vector2<XT, YT>
		where
			XT: Mul,
			YT: Mul,
			<XT as Mul>::Output: Add<<YT as Mul>::Output>,
		{
			/// Obtain the dot product between this and another vector
			pub fn dot(
				&self,
				other: &Self,
			) -> <<XT as Mul>::Output as Add<<YT as Mul>::Output>>::Output {
				(self.x * other.x) + (self.y * other.y)
			}

			/// Obtain the magnitude of this vector squared, to avoid undoing the square root where only the scale of the scalar value is important
			pub fn mag2(&self) -> <<XT as Mul>::Output as Add<<YT as Mul>::Output>>::Output
			where
				Self: Clone,
			{
				self.clone().dot(self)
			}

			/// Obtain the magtnidue of this vector.
			pub fn mag(self) -> f64
			where
				<<XT as Mul>::Output as Add<<YT as Mul>::Output>>::Output: std::convert::Into<f64>,
				Self: Clone,
			{
				(self.mag2().into()).sqrt()
			}
			/**
			cardinal() const
            {
                    const Vector2   abs{    std::abs(x),
                                                            std::abs(y)                     };

                    Vector2                 out{    (x >= 0) - (x <= 0),
                                                            (y >= 0) - (y <= 0)     };

                    out.x *= (abs.x >= abs.y);
                    out.y *= (abs.x <= abs.y);

                    return out;
            }
			 */
			/// Obtain the vector aligned between this one and the closest vector whose angle is divisible by 45 in degrees
			pub fn cardinal(&self) -> Vector2<XT, YT>
			where
			{
				let abs = Self {
					x: self.x.abs(),
					y: self.y.abs(),
				};
				abs
			}
		}
	}

	pub mod addition {
		use super::Vector2;
		use std::ops::{Add, AddAssign};

		impl<XT, YT> Add for Vector2<XT, YT>
		where
			XT: Add,
			YT: Add,
		{
			type Output = super::Vector2<XT::Output, YT::Output>;
			fn add(self, rhs: Self) -> Self::Output {
				Self::Output {
					x: self.x + rhs.x,
					y: self.y + rhs.y,
				}
			}
		}

		impl<XT, YT> AddAssign for Vector2<XT, YT>
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

	pub mod subtraction {
		use super::Vector2;
		use std::ops::{Neg, Sub, SubAssign};

		impl<XT, YT> Neg for Vector2<XT, YT>
		where
			XT: Neg,
			YT: Neg,
		{
			type Output = Vector2<XT::Output, YT::Output>;
			fn neg(self) -> Self::Output {
				Self::Output {
					x: -self.x,
					y: -self.y,
				}
			}
		}

		impl<XT, YT> Sub for Vector2<XT, YT>
		where
			XT: Sub,
			YT: Sub,
		{
			type Output = Vector2<XT::Output, YT::Output>;
			fn sub(self, rhs: Self) -> Self::Output {
				Self::Output {
					x: self.x - rhs.x,
					y: self.y - rhs.y,
				}
			}
		}

		impl<XT, YT> SubAssign for Vector2<XT, YT>
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

	pub mod multiplication {
		use super::Vector2;
		use std::ops::{Mul, MulAssign};

		impl<XT, YT> Mul for Vector2<XT, YT>
		where
			XT: Mul,
			YT: Mul,
		{
			type Output = Vector2<XT::Output, YT::Output>;
			fn mul(self, rhs: Self) -> Self::Output {
				Self::Output {
					x: self.x * rhs.x,
					y: self.y * rhs.y,
				}
			}
		}

		impl<XT, YT> MulAssign for Vector2<XT, YT>
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

	pub mod division {
		use super::Vector2;
		use std::ops::{Div, DivAssign};

		impl<XT, YT> Div for Vector2<XT, YT>
		where
			XT: Div,
			YT: Div,
		{
			type Output = Vector2<XT::Output, YT::Output>;
			fn div(self, rhs: Self) -> Self::Output {
				Self::Output {
					x: self.x / rhs.x,
					y: self.y / rhs.y,
				}
			}
		}

		impl<XT, YT> DivAssign for Vector2<XT, YT>
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

	pub mod remainder {
		use super::Vector2;
		use std::ops::{Rem, RemAssign};

		impl<XT, YT> Rem for Vector2<XT, YT>
		where
			XT: Rem,
			YT: Rem,
		{
			type Output = Vector2<XT::Output, YT::Output>;
			fn rem(self, rhs: Self) -> Self::Output {
				Self::Output {
					x: self.x % rhs.x,
					y: self.y % rhs.y,
				}
			}
		}

		impl<XT, YT> RemAssign for Vector2<XT, YT>
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

	pub mod compare {
		use super::Vector2;
		use std::cmp::{Eq, Ord, PartialEq, PartialOrd};

		impl<XT, YT> PartialEq for Vector2<XT, YT>
		where
			XT: PartialEq,
			YT: PartialEq,
		{
			fn eq(&self, rhs: &Self) -> bool {
				(self.x == rhs.x) && (self.y == rhs.y)
			}
		}
		
		impl<XT, YT> Eq for Vector2<XT, YT>
		where
			XT: Eq,
			YT: Eq,
		{}
		
		impl<XT, YT> PartialOrd for Vector2<XT, YT>
		where
			XT: PartialOrd,
			YT: PartialOrd,
		{
			fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
				Option::<std::cmp::Ordering>::from( self.x.partial_cmp(&other.x) ).and(Option::from( self.y.partial_cmp(&other.y) ))
			}
		}
		
		impl<XT, YT> Ord for Vector2<XT, YT>
		where
			XT: Ord,
			YT: Ord,
		{
			fn cmp(&self, other: &Self) -> std::cmp::Ordering {
				use std::cmp::Ordering;
				let xcmp = match self.x.cmp(&other.x) {
					Ordering::Equal => Ordering::Equal,
					Ordering::Less => Ordering::Less,
					Ordering::Greater => Ordering::Greater,
				};
				let ycmp = match self.y.cmp(&other.y) {
					Ordering::Equal => Ordering::Equal,
					Ordering::Less => Ordering::Less,
					Ordering::Greater => Ordering::Greater,
				};
				
				xcmp.then(ycmp)
			}
		}
		
	}

	pub mod display_impl {
		use super::Vector2;
		use std::fmt;

		impl<XT, YT> fmt::Display for Vector2<XT, YT>
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

pub use vector2_impl::Vector2;
pub use vector2_impl::ctrs;
pub use vector2_impl::vec_ops;
pub use vector2_impl::display_impl;
