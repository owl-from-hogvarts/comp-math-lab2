#![no_std]

use point::Point;

pub mod byte_serializable;
pub mod point;
pub mod request;
pub mod response;

pub type TProtocolSignature = u64;
pub const PROTOCOL_SIGNATURE_SIZE: usize = size_of::<TProtocolSignature>();
pub const PROTOCOL_SIGNATURE: TProtocolSignature = 0x15_8d_c5_8c_30_4f_00_7b;
pub const POINT_AMOUNT: usize = 256;
pub const LONG_PACKAGE_SIZE: usize = Point::POINT_SIZE_BYTES * POINT_AMOUNT;
pub const PACKAGE_SIZE: usize = 16;

pub const T_NUMBER_SIZE_BYTES: usize = 8;
pub type TNumber = f64;

pub fn is_signature_valid(bytes: &[u8]) -> bool {
    const SIGNATURE_BYTES: [u8; PROTOCOL_SIGNATURE_SIZE] = PROTOCOL_SIGNATURE.to_le_bytes();
    if bytes.len() < PROTOCOL_SIGNATURE_SIZE {
        return false;
    }

    bytes
        .windows(PROTOCOL_SIGNATURE_SIZE)
        .any(|slice| slice == SIGNATURE_BYTES)
}
