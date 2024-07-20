use crate::{
    byte_serializable::{read_field, ByteSerializable},
    point::Point,
    TNumber, LONG_PACKAGE_SIZE, PACKAGE_SIZE, POINT_AMOUNT, T_NUMBER_SIZE_BYTES,
};

#[derive(Debug, Clone, Copy)]
pub enum ResponsePackage {
    InitialApproximations(InitialApproximationsResponse),
    ComputeRoot(ComputeRootResponse),
    FunctionPoints(FunctionPointsResponse),
}

impl From<InitialApproximationsResponse> for ResponsePackage {
    fn from(value: InitialApproximationsResponse) -> Self {
        Self::InitialApproximations(value)
    }
}

impl From<ComputeRootResponse> for ResponsePackage {
    fn from(value: ComputeRootResponse) -> Self {
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

#[derive(Debug, Clone, Copy)]
pub struct InitialApproximationsResponse {
    pub left: TNumber,
    pub right: TNumber,
}
impl InitialApproximationsResponse {
    const LEFT_BYTES_OFFSET: usize = ResponsePackage::PAYLOAD_OFFSET;
    const RIGHT_BYTES_OFFSET: usize = Self::LEFT_BYTES_OFFSET + T_NUMBER_SIZE_BYTES;
}

#[derive(Debug, Clone, Copy)]
pub struct ComputeRootResponse {
    pub root: Point,
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

    fn from_bytes(raw_bytes: &[u8; PACKAGE_SIZE]) -> Self {
        let left_bytes: [u8; T_NUMBER_SIZE_BYTES] = read_field(raw_bytes, Self::LEFT_BYTES_OFFSET);
        let left = f64::from_le_bytes(left_bytes);

        let right_bytes: [u8; 8] = raw_bytes
            [Self::RIGHT_BYTES_OFFSET..(Self::RIGHT_BYTES_OFFSET + T_NUMBER_SIZE_BYTES)]
            .try_into()
            .unwrap();
        let right = f64::from_le_bytes(right_bytes);

        Self { left, right }
    }
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
