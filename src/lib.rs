#![no_std]

use point::Point;

mod point;
mod request;
mod response;
mod byte_serializable;

pub const PROTOCOL_SIGNATURE: u64 = 0x15_8d_c5_8c_30_4f_00_7b;
pub const POINT_AMOUNT: usize = 256;
pub const LONG_PACKAGE_SIZE: usize = Point::POINT_SIZE_BYTES * POINT_AMOUNT;
pub const PACKAGE_SIZE: usize = 16;

pub const T_NUMBER_SIZE_BYTES: usize = 8;
type TNumber = f64;
