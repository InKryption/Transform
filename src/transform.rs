use std::fmt;
use std::ops;

#[derive(Debug)]
pub struct Vec2<XT = i32, YT = XT> {
    pub x: XT,
    pub y: YT,
}

impl<XT, YT> Vec2<XT, YT> {
    pub fn new(x: XT, y: YT) -> Vec2<XT, YT> {
        Vec2 { x, y }
    }
}

impl<T> Vec2<T, T> {
    pub fn iso(n: T) -> Vec2<T,T>
    where
        T: Copy
    {
        Vec2 { x: n, y: n }
    }
}

impl<XT, YT> std::default::Default for Vec2<XT, YT>
where
    XT: std::convert::From<u8>,
    YT: std::convert::From<u8>,
{
    fn default() -> Vec2<XT, YT> {
        Vec2 {
            x: std::convert::From::from(0),
            y: std::convert::From::from(0),
        }
    }
}

impl<XT, YT> Clone for Vec2<XT, YT>
where
    XT: Clone,
    YT: Clone
{
    fn clone(&self) -> Vec2<XT, YT> {
        Vec2 { x: self.x.clone(), y: self.y.clone() }
    }
}

impl<XT, YT> Copy for Vec2<XT, YT>
where XT: Copy, YT: Copy {}

impl<XT, YT> ops::Add for Vec2<XT, YT>
where
    XT: ops::Add,
    YT: ops::Add,
{
    type Output = Vec2<XT::Output, YT::Output>;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> ops::Add<T> for Vec2<T,T>
where
    T: ops::Add + Clone
{
    type Output = Vec2<T::Output, T::Output>;
    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x + rhs.clone(),
            y: self.y + rhs,
        }
    }
}

impl<XT, YT> ops::AddAssign for Vec2<XT, YT>
where
    XT: ops::AddAssign,
    YT: ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> ops::AddAssign for Vec2<T,T>
where
    T: ops::AddAssign + Clone
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs.clone();
        self.y += rhs;
    }
}

impl<XT, YT> ops::Neg for Vec2<XT,YT>
where
    XT: ops::Neg,
    YT: ops::Neg
{
    type Output = Vec2<XT::Output, YT::Output>;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y
        }
    }
}

impl<XT, YT> ops::Sub for Vec2<XT, YT>
where
    XT: ops::Sub,
    YT: ops::Sub,
{
    type Output = Vec2<XT::Output, YT::Output>;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> ops::Sub for Vec2<T, T>
where
    T: ops::Sub + Clone
{
    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x - rhs.clone(),
            y: self.x - rhs
        }
    }
}

impl<T> ops::SubAssign for Vec2<T, T>
where
    T: ops::SubAssign + Clone
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs.clone();
        self.y -= rhs;
    }
}

impl<XT, YT> ops::SubAssign for Vec2<XT, YT>
where
    XT: ops::SubAssign,
    YT: ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<XT, YT> ops::Mul for Vec2<XT, YT>
where
    XT: ops::Mul,
    YT: ops::Mul,
{
    type Output = Vec2<XT::Output, YT::Output>;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<XT, YT> ops::MulAssign for Vec2<XT, YT>
where
    XT: ops::MulAssign,
    YT: ops::MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<XT, YT> ops::Div for Vec2<XT, YT>
where
    XT: ops::Div,
    YT: ops::Div,
{
    type Output = Vec2<XT::Output, YT::Output>;
    fn div(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<XT, YT> ops::DivAssign for Vec2<XT, YT>
where
    XT: ops::DivAssign,
    YT: ops::DivAssign,
{
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<XT, YT> ops::Rem for Vec2<XT, YT>
where
    XT: ops::Rem,
    YT: ops::Rem
{
    type Output = Vec2<XT::Output, YT::Output>;
    fn rem(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x % rhs.x,
            y: self.y % rhs.y
        }
    }
}

impl<XT, YT> ops::RemAssign for Vec2<XT, YT>
where
    XT: ops::RemAssign,
    YT: ops::RemAssign
{
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

impl<XT, YT> fmt::Display for Vec2<XT, YT>
where
    XT: fmt::Display,
    YT: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}
