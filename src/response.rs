use core::fmt::Display;

use crate::{
    byte_serializable::{read_field, ByteSerializable},
    point::Point,
    TNumber, LONG_PACKAGE_SIZE, PACKAGE_SIZE, POINT_AMOUNT, T_NUMBER_SIZE_BYTES,
};

#[derive(Debug, Clone, Copy)]
pub enum MethodError {
    NoRootInRange,
    MoreThanOneRootInRange,
    Diverges,
}

impl MethodError {
    const NO_ROOT_IN_RANGE: u8 = 0;
    const MORE_THAN_ONE_ROOT_IN_RANGE: u8 = 1;
    const DIVERGES: u8 = 2;
}

impl Display for MethodError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            MethodError::NoRootInRange => write!(f, "No roots found withing range"),
            MethodError::MoreThanOneRootInRange => write!(f, "More than one root withing range"),
            MethodError::Diverges => write!(f, "Method diverges"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ResponsePackage {
    InitialApproximations(InitialApproximationsResponse),
    ComputeRoot(Result<ComputeRootResponse, MethodError>),
    FunctionPoints(FunctionPointsResponse),
    FunctionPointsSecond(FunctionPointsResponse),
}

impl ByteSerializable<PACKAGE_SIZE> for Result<ComputeRootResponse, MethodError> {
    fn to_bytes(&self) -> [u8; PACKAGE_SIZE] {
        match self {
            Ok(response) => response.to_bytes(),
            Err(error) => {
                const ROOT_OFFSET: usize = ComputeRootResponse::ROOT_OFFSET;
                let mut bytes = [0; PACKAGE_SIZE];

                bytes[ROOT_OFFSET..ROOT_OFFSET + T_NUMBER_SIZE_BYTES]
                    .copy_from_slice(&TNumber::to_le_bytes(TNumber::NAN));

                let error_status = match error {
                    MethodError::NoRootInRange => MethodError::NO_ROOT_IN_RANGE,
                    MethodError::MoreThanOneRootInRange => MethodError::MORE_THAN_ONE_ROOT_IN_RANGE,
                    MethodError::Diverges => MethodError::DIVERGES,
                };

                bytes[ComputeRootResponse::STATUS_OFFSET] = error_status;

                bytes
            }
        }
    }

    fn from_bytes(raw_bytes: &[u8; PACKAGE_SIZE]) -> Self {
        let marker_bytes: [u8; T_NUMBER_SIZE_BYTES] =
            read_field(raw_bytes, ComputeRootResponse::ROOT_OFFSET);
        let marker = TNumber::from_le_bytes(marker_bytes);
        if !marker.is_nan() {
            return Ok(ComputeRootResponse::from_bytes(raw_bytes));
        }

        let error = match raw_bytes[ComputeRootResponse::STATUS_OFFSET] {
            MethodError::NO_ROOT_IN_RANGE => MethodError::NoRootInRange,
            MethodError::MORE_THAN_ONE_ROOT_IN_RANGE => MethodError::MoreThanOneRootInRange,
            MethodError::DIVERGES => MethodError::Diverges,
            _ => unreachable!(),
        };

        Err(error)
    }
}

impl From<InitialApproximationsResponse> for ResponsePackage {
    fn from(value: InitialApproximationsResponse) -> Self {
        Self::InitialApproximations(value)
    }
}

impl From<Result<ComputeRootResponse, MethodError>> for ResponsePackage {
    fn from(value: Result<ComputeRootResponse, MethodError>) -> Self {
        Self::ComputeRoot(value)
    }
}

impl From<FunctionPointsResponse> for ResponsePackage {
    fn from(value: FunctionPointsResponse) -> Self {
        Self::FunctionPoints(value)
    }
}

impl ResponsePackage {
    const PAYLOAD_OFFSET: usize = 0;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InitialApproximationsResponse {
    pub left: TNumber,
    pub right: TNumber,
}
impl InitialApproximationsResponse {
    const LEFT_BYTES_OFFSET: usize = ResponsePackage::PAYLOAD_OFFSET;
    const RIGHT_BYTES_OFFSET: usize = Self::LEFT_BYTES_OFFSET + T_NUMBER_SIZE_BYTES;
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

    fn from_bytes(raw_bytes: &[u8; PACKAGE_SIZE]) -> Self {
        let left_bytes: [u8; T_NUMBER_SIZE_BYTES] = read_field(raw_bytes, Self::LEFT_BYTES_OFFSET);
        let left = TNumber::from_le_bytes(left_bytes);

        let right_bytes: [u8; T_NUMBER_SIZE_BYTES] = raw_bytes
            [Self::RIGHT_BYTES_OFFSET..(Self::RIGHT_BYTES_OFFSET + T_NUMBER_SIZE_BYTES)]
            .try_into()
            .unwrap();
        let right = TNumber::from_le_bytes(right_bytes);

        Self { left, right }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ComputeRootResponse {
    pub root: Point,
}

impl ComputeRootResponse {
    const ROOT_OFFSET: usize = ResponsePackage::PAYLOAD_OFFSET;
    const STATUS_OFFSET: usize = ResponsePackage::PAYLOAD_OFFSET + T_NUMBER_SIZE_BYTES;
}

impl ByteSerializable<PACKAGE_SIZE> for ComputeRootResponse {
    fn to_bytes(&self) -> [u8; PACKAGE_SIZE] {
        let mut package: [u8; PACKAGE_SIZE] = [0; PACKAGE_SIZE];
        package[Self::ROOT_OFFSET..(Self::ROOT_OFFSET + Point::POINT_SIZE_BYTES)]
            .copy_from_slice(&self.root.to_bytes());

        package
    }

    fn from_bytes(raw_bytes: &[u8; PACKAGE_SIZE]) -> Self {
        let point_bytes: [u8; Point::POINT_SIZE_BYTES] = read_field(raw_bytes, Self::ROOT_OFFSET);
        ComputeRootResponse {
            root: Point::from_bytes(&point_bytes),
        }
    }
}

/// struct is too big to fit into arduino's memory
/// points should be send one after another, sorted by `x` field
#[derive(Debug, Clone, Copy)]
pub struct FunctionPointsResponse(pub [Point; POINT_AMOUNT]);

impl From<&[u8; LONG_PACKAGE_SIZE]> for FunctionPointsResponse {
    fn from(value: &[u8; LONG_PACKAGE_SIZE]) -> Self {
        let mut points: [Point; POINT_AMOUNT] = [Point::zero(); POINT_AMOUNT];
        for (index, point) in points.iter_mut().enumerate() {
            *point = Point::from_bytes(&read_field(value, index * Point::POINT_SIZE_BYTES))
        }

        FunctionPointsResponse(points)
    }
}
