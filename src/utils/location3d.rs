use crate::utils::Parsable;
use crate::utils::location::Distance;
use nom::Parser;
use nom::error::Error;
use num::integer::Roots;
use num::traits::Euclid;
use num::{Bounded, Num, Signed, Zero, one, zero};
use std::fmt::{Display, Formatter};
use std::iter::{Sum, successors};
use std::ops::{Add, Div, Mul, Neg, RangeInclusive, Rem, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Location3<T: Num> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Display + Num> Display for Location3<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl<T: Num> Location3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn map<U: Num, F: Fn(T) -> U>(self, f: F) -> Location3<U> {
        Location3::new(f(self.x), f(self.y), f(self.z))
    }

    pub fn try_map<U: Num, E, F: Fn(T) -> Result<U, E>>(self, f: F) -> Result<Location3<U>, E> {
        Ok(Location3::new(f(self.x)?, f(self.y)?, f(self.z)?))
    }
}

pub fn location3<'a, T, Sep>(
    mut sep: Sep,
) -> impl Parser<&'a str, Output = Location3<T>, Error = Error<&'a str>>
where
    T: Num + Parsable<'a>,
    Sep: Parser<&'a str, Error = Error<&'a str>>,
{
    move |i: &'a str| {
        let (i, x) = T::parse(i)?;
        let (i, _) = sep.parse(i)?;
        let (i, y) = T::parse(i)?;
        let (i, _) = sep.parse(i)?;
        let (i, z) = T::parse(i)?;

        Ok((i, Location3::new(x, y, z)))
    }
}

impl<T: Num + Copy + Signed> Location3<T> {
    pub fn manhattan_distance(self, other: Self) -> T {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl<T: Num + Copy + PartialOrd> Location3<T> {
    pub fn iter_ray(self, direction: Location3<T>) -> impl Iterator<Item = Location3<T>> {
        successors(Some(self), move |&current| Some(current + direction))
    }

    pub fn square_range(self, end: Location3<T>) -> RangeInclusive<Location3<T>> {
        self..=(end - Location3::new(one(), one(), one()))
    }
}

impl<T: Num> Zero for Location3<T> {
    fn zero() -> Self {
        Location3::new(zero(), zero(), zero())
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}

impl<T: Num> Add<Self> for Location3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Location3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<T: Num> Sub<Self> for Location3<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Location3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: Num + Copy> Mul<T> for Location3<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Location3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<T: Num + Copy> Div<T> for Location3<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Location3::new(self.x / rhs, self.y / rhs, self.z / rhs)
    }
}

impl<T: Num + Copy> Div<Self> for Location3<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Location3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl<T: Num + Copy> Rem<Self> for Location3<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Location3::new(self.x % rhs.x, self.y % rhs.y, self.z % rhs.z)
    }
}

impl<T: Num + Copy + Euclid> Euclid for Location3<T> {
    fn div_euclid(&self, rhs: &Self) -> Self {
        Location3::new(
            self.x.div_euclid(&rhs.x),
            self.y.div_euclid(&rhs.y),
            self.z.div_euclid(&rhs.z),
        )
    }

    fn rem_euclid(&self, rhs: &Self) -> Self {
        Location3::new(
            self.x.rem_euclid(&rhs.x),
            self.y.rem_euclid(&rhs.y),
            self.z.rem_euclid(&rhs.z),
        )
    }
}

impl<T: Num + Bounded> Bounded for Location3<T> {
    fn min_value() -> Self {
        Location3::new(T::min_value(), T::min_value(), T::min_value())
    }

    fn max_value() -> Self {
        Location3::new(T::max_value(), T::max_value(), T::max_value())
    }
}

impl<T: Num + Copy + Signed> Neg for Location3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Location3::new(-self.x, -self.y, -self.z)
    }
}

impl<T: Num + Copy> Sum for Location3<T> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Location3::zero(), Add::add)
    }
}

impl<T: Num + Copy + Into<U>, U: Num + Copy + Roots> Distance<U> for Location3<T> {
    fn distance(self: &Self, other: &Self) -> U {
        let diff_x = (self.x - other.x).into();
        let diff_y = (self.y - other.y).into();
        let diff_z = (self.z - other.z).into();

        (diff_x * diff_x + diff_y * diff_y + diff_z * diff_z).sqrt()
    }
}

pub mod direction {
    use crate::utils::location3d::Location3;

    pub const ZERO: Location3<i32> = Location3::new(0, 0, 0);
    pub const LEFT: Location3<i32> = Location3::new(-1, 0, 0);
    pub const RIGHT: Location3<i32> = Location3::new(1, 0, 0);
    pub const UP: Location3<i32> = Location3::new(0, -1, 0);
    pub const DOWN: Location3<i32> = Location3::new(0, 1, 0);
    pub const FORWARD: Location3<i32> = Location3::new(0, 0, -1);
    pub const BACKWARD: Location3<i32> = Location3::new(0, 0, 1);
}
