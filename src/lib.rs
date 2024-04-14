#![no_std]

use core::mem::size_of;

pub const PROTOCOL_SIGNATURE: u64 = 0x15_8d_c5_8c_30_4f_00_7b;
pub const POINT_AMOUNT: usize = 256;
pub const LONG_PACKAGE_SIZE: usize = Point::POINT_SIZE_BYTES * POINT_AMOUNT;
pub const PACKAGE_SIZE: usize = 16;

pub const T_NUMBER_SIZE_BYTES: usize = 8;

type TNumber = f64;

pub enum RequestPackage {
    FunctionPoints,
    InitialApproximations,
    SelectMethod(Method),
    ComputeRoot { epsilon: TNumber },
}

pub trait ByteSerializable<const SIZE: usize> {
    fn to_bytes(&self) -> [u8; SIZE];
    fn from_bytes(raw_bytes: [u8; SIZE]) -> Self;
}

// uses custom to/from methods
// While impl TryFrom<RequestPackage> for [u8; PACKAGE_SIZE] is possible
// package.into() is not straitforward enough
// I'd expect person to look for `to_bytes` method

impl RequestPackage {
    const FUNCTION_POINTS_TYPE: u8 = 0;
    const INITAL_APPROXIMATION_TYPE: u8 = 1;
    const SELECT_METHOD_TYPE: u8 = 2;
    const COMPUTE_ROOT_TYPE: u8 = 3;

    const REQUEST_TYPE_OFFSET: usize = 0;
    const REQUEST_PAYLOAD_OFFSET: usize = 1;
}

impl ByteSerializable<PACKAGE_SIZE> for RequestPackage {
    fn to_bytes(&self) -> [u8; PACKAGE_SIZE] {
        let request_type = match self {
            RequestPackage::FunctionPoints => RequestPackage::FUNCTION_POINTS_TYPE,
            RequestPackage::InitialApproximations => RequestPackage::INITAL_APPROXIMATION_TYPE,
            RequestPackage::SelectMethod(_) => RequestPackage::SELECT_METHOD_TYPE,
            RequestPackage::ComputeRoot { .. } => RequestPackage::COMPUTE_ROOT_TYPE,
        };

        let mut package: [u8; PACKAGE_SIZE] = [0; PACKAGE_SIZE];
        package[RequestPackage::REQUEST_TYPE_OFFSET] = request_type;
        match self {
            RequestPackage::FunctionPoints => (),
            RequestPackage::InitialApproximations => (),
            RequestPackage::SelectMethod(method) => {
                package[RequestPackage::REQUEST_PAYLOAD_OFFSET] = method.to_byte();
            }
            RequestPackage::ComputeRoot { epsilon } => {
                package[RequestPackage::REQUEST_PAYLOAD_OFFSET
                    ..(RequestPackage::REQUEST_PAYLOAD_OFFSET + T_NUMBER_SIZE_BYTES)]
                    .copy_from_slice(&epsilon.to_le_bytes());
            }
        };

        package
    }

    fn from_bytes(raw_bytes: [u8; PACKAGE_SIZE]) -> RequestPackage {
        let request_type = raw_bytes[RequestPackage::REQUEST_TYPE_OFFSET];
        match request_type {
            RequestPackage::FUNCTION_POINTS_TYPE => RequestPackage::FunctionPoints,
            RequestPackage::INITAL_APPROXIMATION_TYPE => RequestPackage::InitialApproximations,
            RequestPackage::SELECT_METHOD_TYPE => RequestPackage::SelectMethod(Method::from_byte(
                raw_bytes[RequestPackage::REQUEST_PAYLOAD_OFFSET],
            )),
            RequestPackage::COMPUTE_ROOT_TYPE => {
                let epsilon_bytes: [u8; T_NUMBER_SIZE_BYTES] =
                    read_field(&raw_bytes, Self::REQUEST_PAYLOAD_OFFSET);
                RequestPackage::ComputeRoot {
                    epsilon: TNumber::from_le_bytes(epsilon_bytes),
                }
            }
            _ => unreachable!(),
        }
    }
}

pub enum Method {
    Chord,
    Secant,
    SimpleIterationSingle,
    SimpleIteration,
}

impl Method {
    const CHORD: u8 = 0;
    const SECANT: u8 = 1;
    const SIMPLEITERATIONSINGLE: u8 = 2;
    const SIMPLEITERATION: u8 = 3;

    pub fn to_byte(&self) -> u8 {
        match self {
            Method::Chord => Method::CHORD,
            Method::Secant => Method::SECANT,
            Method::SimpleIterationSingle => Method::SIMPLEITERATIONSINGLE,
            Method::SimpleIteration => Method::SIMPLEITERATION,
        }
    }

    pub fn from_byte(byte: u8) -> Method {
        match byte {
            Method::CHORD => Method::Chord,
            Method::SECANT => Method::Secant,
            Method::SIMPLEITERATIONSINGLE => Method::SimpleIterationSingle,
            Method::SIMPLEITERATION => Method::SimpleIteration,
            _ => unreachable!(),
        }
    }
}

pub struct ResponsePackage();

impl ResponsePackage {
    const PAYLOAD_OFFSET: usize = 0;
}

struct InitialApproximationsResponse {
    left: TNumber,
    right: TNumber,
}
impl InitialApproximationsResponse {
    const LEFT_BYTES_OFFSET: usize = ResponsePackage::PAYLOAD_OFFSET;
    const RIGHT_BYTES_OFFSET: usize = Self::LEFT_BYTES_OFFSET + T_NUMBER_SIZE_BYTES;
}

struct SelectMethodResponse {
    method: Method,
}

impl SelectMethodResponse {
    const METHOD_OFFSET: usize = ResponsePackage::PAYLOAD_OFFSET;
}

struct ComputeRootResponse {
    root: Point,
}

impl ComputeRootResponse {
    const ROOT_OFFSET: usize = ResponsePackage::PAYLOAD_OFFSET;
}

impl ByteSerializable<PACKAGE_SIZE> for InitialApproximationsResponse {
    fn to_bytes(&self) -> [u8; PACKAGE_SIZE] {
        let mut package: [u8; PACKAGE_SIZE] = [0; PACKAGE_SIZE];
        let left_bytes = self.left.to_le_bytes();
        let right_bytes = self.right.to_le_bytes();

        package[Self::LEFT_BYTES_OFFSET..Self::RIGHT_BYTES_OFFSET].copy_from_slice(&left_bytes);
        package[Self::RIGHT_BYTES_OFFSET..(Self::RIGHT_BYTES_OFFSET + T_NUMBER_SIZE_BYTES)]
            .copy_from_slice(&right_bytes);
        package
    }

    fn from_bytes(raw_bytes: [u8; PACKAGE_SIZE]) -> Self {
        let left_bytes: [u8; T_NUMBER_SIZE_BYTES] = read_field(&raw_bytes, Self::LEFT_BYTES_OFFSET);
        let left = f64::from_le_bytes(left_bytes);

        let right_bytes: [u8; 8] = raw_bytes
            [Self::RIGHT_BYTES_OFFSET..(Self::RIGHT_BYTES_OFFSET + T_NUMBER_SIZE_BYTES)]
            .try_into()
            .unwrap();
        let right = f64::from_le_bytes(right_bytes);

        Self { left, right }
    }
}

impl ByteSerializable<PACKAGE_SIZE> for SelectMethodResponse {
    fn to_bytes(&self) -> [u8; PACKAGE_SIZE] {
        let mut package: [u8; PACKAGE_SIZE] = [0; PACKAGE_SIZE];
        package[Self::METHOD_OFFSET] = self.method.to_byte();

        package
    }

    fn from_bytes(raw_bytes: [u8; PACKAGE_SIZE]) -> Self {
        Self {
            method: Method::from_byte(raw_bytes[Self::METHOD_OFFSET]),
        }
    }
}
impl ByteSerializable<PACKAGE_SIZE> for ComputeRootResponse {
    fn to_bytes(&self) -> [u8; PACKAGE_SIZE] {
        let mut package: [u8; PACKAGE_SIZE] = [0; PACKAGE_SIZE];
        package[Self::ROOT_OFFSET..(Self::ROOT_OFFSET + Point::POINT_SIZE_BYTES)]
            .copy_from_slice(&self.root.to_bytes());

        package
    }

    fn from_bytes(raw_bytes: [u8; PACKAGE_SIZE]) -> Self {
        let point_bytes: [u8; Point::POINT_SIZE_BYTES] = read_field(&raw_bytes, Self::ROOT_OFFSET);
        ComputeRootResponse {
            root: Point::from_bytes(point_bytes),
        }
    }
}

/// struct is too big to fit into arduino's memory
/// points should be send one after another, sorted by `x` field
pub struct FunctionPointsResponse(pub [Point; POINT_AMOUNT]);

impl From<[u8; LONG_PACKAGE_SIZE]> for FunctionPointsResponse {
    fn from(value: [u8; LONG_PACKAGE_SIZE]) -> Self {
        let mut points: [Point; POINT_AMOUNT] = [Point::zero(); POINT_AMOUNT];
        for (index, point) in points.iter_mut().enumerate() {
            *point = Point::from_bytes(read_field(&value, index * Point::POINT_SIZE_BYTES))
        }

        FunctionPointsResponse(points)
    }
}

#[derive(Clone, Copy)]
pub struct Point {
    x: TNumber,
    y: TNumber,
}

impl Point {
    fn zero() -> Self {
        Point { x: 0., y: 0. }
    }
}

impl ByteSerializable<{ Self::POINT_SIZE_BYTES }> for Point {
    fn to_bytes(&self) -> [u8; Self::POINT_SIZE_BYTES] {
        let mut bytes: [u8; Self::POINT_SIZE_BYTES] = [0; Self::POINT_SIZE_BYTES];
        bytes[0..T_NUMBER_SIZE_BYTES].copy_from_slice(&self.x.to_le_bytes());
        bytes[T_NUMBER_SIZE_BYTES..].copy_from_slice(&self.y.to_le_bytes());

        bytes
    }

    fn from_bytes(raw_bytes: [u8; Self::POINT_SIZE_BYTES]) -> Self {
        Self {
            x: f64::from_be_bytes(read_field(&raw_bytes, 0)),
            y: f64::from_le_bytes(read_field(&raw_bytes, T_NUMBER_SIZE_BYTES)),
        }
    }
}

impl Point {
    pub const POINT_SIZE_BYTES: usize = 2 * 8;
}

fn read_field<const LENGTH: usize>(raw_bytes: &[u8], offset: usize) -> [u8; LENGTH] {
    raw_bytes[offset..(offset + LENGTH)]
        .try_into()
        .expect("lengthes always match")
}
