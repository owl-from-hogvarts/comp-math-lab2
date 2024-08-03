use crate::{byte_serializable::ByteSerializable, TNumber, T_NUMBER_SIZE_BYTES};

use super::{EquationMode, EquationModeRaw, RequestPackage};

#[derive(Copy, Clone, Debug)]
pub struct ComputeRootPayload {
  pub epsilon: TNumber,
  pub mode: EquationMode,
}

impl ComputeRootPayload {
  pub const MODE_OFFSET: usize = RequestPackage::REQUEST_PAYLOAD_OFFSET + T_NUMBER_SIZE_BYTES;
}

pub struct FunctionPointsPayload {
  pub mode: EquationModeRaw,
  pub equation_number: u8,
}

impl FunctionPointsPayload {
  pub const FUNCTION_POINTS_PAYLOAD_SIZE: usize = 2;
}

impl ByteSerializable<{ Self::FUNCTION_POINTS_PAYLOAD_SIZE }> for FunctionPointsPayload {
  fn to_bytes(&self) -> [u8; Self::FUNCTION_POINTS_PAYLOAD_SIZE] {
      let mode_byte = match self.mode {
          EquationModeRaw::SingleEquation => EquationModeRaw::SINGLE_EQUATION_MODE,
          EquationModeRaw::SystemOfEquations => EquationModeRaw::SYSTEM_OF_EQUATIONS_MODE,
      };

      [mode_byte, self.equation_number]
  }

  fn from_bytes(raw_bytes: &[u8; Self::FUNCTION_POINTS_PAYLOAD_SIZE]) -> Self {
      let mode_byte = raw_bytes[0];
      let equation_number = raw_bytes[1];

      let mode = match mode_byte {
          EquationModeRaw::SINGLE_EQUATION_MODE => EquationModeRaw::SingleEquation,
          EquationModeRaw::SYSTEM_OF_EQUATIONS_MODE => EquationModeRaw::SystemOfEquations,
          _ => unreachable!(),
      };

      Self {
          mode,
          equation_number,
      }
  }
}