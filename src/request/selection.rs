use crate::byte_serializable::ByteSerializable;

use super::{EquationMode, EquationModeRaw};

#[derive(Copy, Clone, Debug)]
pub struct Selection {
    pub mode: EquationModeRaw,
    pub index: u8,
}

impl Selection {
    pub const FUNCTION_POINTS_PAYLOAD_SIZE: usize = 2;
}

impl ByteSerializable<{ Self::FUNCTION_POINTS_PAYLOAD_SIZE }> for Selection {
    fn to_bytes(&self) -> [u8; Self::FUNCTION_POINTS_PAYLOAD_SIZE] {
        let mode_byte = match self.mode {
            EquationModeRaw::SingleEquation => EquationModeRaw::SINGLE_EQUATION_MODE,
            EquationModeRaw::SystemOfEquations => EquationModeRaw::SYSTEM_OF_EQUATIONS_MODE,
        };

        [mode_byte, self.index]
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
            index: equation_number,
        }
    }
}

impl From<EquationMode> for Selection {
    fn from(value: EquationMode) -> Self {
        match value {
            EquationMode::Single(single) => Selection {
                mode: EquationModeRaw::SingleEquation,
                index: single.equation_number,
            },
            EquationMode::SystemOfEquations { system_number } => Selection {
                mode: EquationModeRaw::SystemOfEquations,
                index: system_number,
            },
        }
    }
}
