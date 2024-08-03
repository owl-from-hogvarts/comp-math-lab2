use core::ops::{Add, Mul, Neg, Sub};

use crate::byte_serializable::{read_field, ByteSerializable};
use crate::{TNumber, T_NUMBER_SIZE_BYTES};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: TNumber,
    pub y: TNumber,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PointCoordinate {
    X,
    Y,
}

impl Point {
    pub const POINT_SIZE_BYTES: usize = 2 * T_NUMBER_SIZE_BYTES;
}

impl Point {
    pub fn new(x: TNumber, y: TNumber) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self { x: 0., y: 0. }
    }

    pub fn get_coordinate(&self, coordinate: PointCoordinate) -> TNumber {
        match coordinate {
            PointCoordinate::X => self.x,
            PointCoordinate::Y => self.y,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Mul<TNumber> for Point {
    type Output = Point;

    fn mul(self, rhs: TNumber) -> Self::Output {
        Point {
            x: rhs * self.x,
            y: rhs * self.y,
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<Point> for TNumber {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl ByteSerializable<{ Self::POINT_SIZE_BYTES }> for Point {
    fn to_bytes(&self) -> [u8; Self::POINT_SIZE_BYTES] {
        let mut bytes: [u8; Self::POINT_SIZE_BYTES] = [0; Self::POINT_SIZE_BYTES];
        bytes[0..T_NUMBER_SIZE_BYTES].copy_from_slice(&self.x.to_le_bytes());
        bytes[T_NUMBER_SIZE_BYTES..].copy_from_slice(&self.y.to_le_bytes());

        bytes
    }

    fn from_bytes(raw_bytes: &[u8; Self::POINT_SIZE_BYTES]) -> Self {
        Self {
            x: TNumber::from_le_bytes(read_field(raw_bytes, 0)),
            y: TNumber::from_le_bytes(read_field(raw_bytes, T_NUMBER_SIZE_BYTES)),
        }
    }
}

