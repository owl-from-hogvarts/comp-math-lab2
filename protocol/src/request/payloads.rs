use crate::{TNumber, T_NUMBER_SIZE_BYTES};

use super::{EquationMode, RequestPackage};

#[derive(Copy, Clone, Debug)]
pub struct ComputeRootPayload {
    pub epsilon: TNumber,
    pub mode: EquationMode,
}

impl ComputeRootPayload {
    pub const MODE_OFFSET: usize = RequestPackage::REQUEST_PAYLOAD_OFFSET + T_NUMBER_SIZE_BYTES;
}
