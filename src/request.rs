use crate::byte_serializable::{read_field, ByteSerializable};
use crate::{PACKAGE_SIZE, T_NUMBER_SIZE_BYTES};

mod equation_mode;
pub mod payloads;
pub use equation_mode::*;
use payloads::{ComputeRootPayload, FunctionPointsPayload};

pub enum RequestPackage {
    /// In case of system of equations await two
    /// [`FunctionPointsResponse`](crate::response::FunctionPointsResponse)'s.
    /// That is response per function within system.
    FunctionPoints {
        payload: FunctionPointsPayload,
    },
    InitialApproximations,
    ComputeRoot {
        payload: ComputeRootPayload,
    },
}

impl RequestPackage {
    const FUNCTION_POINTS_TYPE: u8 = 0;
    const INITIAL_APPROXIMATION_TYPE: u8 = 1;
    const COMPUTE_ROOT_TYPE: u8 = 2;

    const REQUEST_TYPE_OFFSET: usize = 0;
    const REQUEST_PAYLOAD_OFFSET: usize = 1;
}

impl ByteSerializable<PACKAGE_SIZE> for RequestPackage {
    fn to_bytes(&self) -> [u8; PACKAGE_SIZE] {
        let request_type = match self {
            RequestPackage::FunctionPoints { .. } => RequestPackage::FUNCTION_POINTS_TYPE,
            RequestPackage::InitialApproximations => RequestPackage::INITIAL_APPROXIMATION_TYPE,
            RequestPackage::ComputeRoot { .. } => RequestPackage::COMPUTE_ROOT_TYPE,
        };

        let mut package: [u8; PACKAGE_SIZE] = [0; PACKAGE_SIZE];
        package[RequestPackage::REQUEST_TYPE_OFFSET] = request_type;
        match self {
            RequestPackage::FunctionPoints { payload } => package
                [RequestPackage::REQUEST_PAYLOAD_OFFSET
                    ..(RequestPackage::REQUEST_PAYLOAD_OFFSET
                        + FunctionPointsPayload::FUNCTION_POINTS_PAYLOAD_SIZE)]
                .copy_from_slice(&payload.to_bytes()),
            RequestPackage::InitialApproximations => (),
            RequestPackage::ComputeRoot { payload } => {
                package[RequestPackage::REQUEST_PAYLOAD_OFFSET
                    ..(RequestPackage::REQUEST_PAYLOAD_OFFSET + T_NUMBER_SIZE_BYTES)]
                    .copy_from_slice(&payload.epsilon.to_le_bytes());

                package[ComputeRootPayload::MODE_OFFSET..(EquationMode::EQUATION_MODE_SIZE)]
                    .copy_from_slice(&payload.mode.to_bytes());
            }
        };

        package
    }

    fn from_bytes(raw_bytes: &[u8; PACKAGE_SIZE]) -> RequestPackage {
        let request_type = raw_bytes[RequestPackage::REQUEST_TYPE_OFFSET];
        match request_type {
            RequestPackage::FUNCTION_POINTS_TYPE => RequestPackage::FunctionPoints {
                payload: FunctionPointsPayload::from_bytes(&read_field(
                    raw_bytes,
                    Self::REQUEST_PAYLOAD_OFFSET,
                )),
            },
            RequestPackage::INITIAL_APPROXIMATION_TYPE => RequestPackage::InitialApproximations,
            RequestPackage::COMPUTE_ROOT_TYPE => {
                let epsilon_bytes: [u8; T_NUMBER_SIZE_BYTES] =
                    read_field(raw_bytes, Self::REQUEST_PAYLOAD_OFFSET);
                let mode_bytes: [u8; EquationMode::EQUATION_MODE_SIZE] =
                    read_field(raw_bytes, ComputeRootPayload::MODE_OFFSET);

                RequestPackage::ComputeRoot {
                    payload: ComputeRootPayload {
                        epsilon: f64::from_le_bytes(epsilon_bytes),
                        mode: EquationMode::from_bytes(&mode_bytes),
                    },
                }
            }
            _ => unreachable!(),
        }
    }
}
