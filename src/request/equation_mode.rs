use compute_method::Method;

use crate::byte_serializable::ByteSerializable;

pub mod compute_method;

pub enum EquationMode {
    Single { method: Method, equation_number: u8 },
    SystemOfEquations { system_number: u8 },
}

impl EquationMode {
    pub const EQUATION_MODE_SIZE: usize = 3;

    const METHOD_OFFSET: usize = 1;
    const EQUATION_NUMBER_OFFSET: usize = 2;
    const SYSTEM_NUMBER_OFFSET: usize = 1;
}

impl ByteSerializable<{ Self::EQUATION_MODE_SIZE }> for EquationMode {
    fn to_bytes(&self) -> [u8; Self::EQUATION_MODE_SIZE] {
        let mut bytes = [0_u8; Self::EQUATION_MODE_SIZE];
        match self {
            EquationMode::Single {
                method,
                equation_number,
            } => {
                bytes[0] = EquationModeRaw::SINGLE_EQUATION_MODE;
                bytes[Self::METHOD_OFFSET] = method.to_byte();
                bytes[Self::EQUATION_NUMBER_OFFSET] = *equation_number;
            }
            &EquationMode::SystemOfEquations { system_number } => {
                bytes[0] = EquationModeRaw::SYSTEM_OF_EQUATIONS_MODE;
                bytes[Self::SYSTEM_NUMBER_OFFSET] = system_number;
            }
        }

        bytes
    }

    fn from_bytes(raw_bytes: &[u8; Self::EQUATION_MODE_SIZE]) -> Self {
        match raw_bytes[0] {
            EquationModeRaw::SINGLE_EQUATION_MODE => Self::Single {
                method: Method::from_byte(raw_bytes[Self::METHOD_OFFSET]),
                equation_number: raw_bytes[Self::EQUATION_NUMBER_OFFSET],
            },
            EquationModeRaw::SYSTEM_OF_EQUATIONS_MODE => Self::SystemOfEquations {
                system_number: raw_bytes[Self::SYSTEM_NUMBER_OFFSET],
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquationModeRaw {
    SingleEquation,
    SystemOfEquations,
}

impl EquationModeRaw {
    pub(super) const SINGLE_EQUATION_MODE: u8 = 0;
    pub(super) const SYSTEM_OF_EQUATIONS_MODE: u8 = 1;
}

impl From<&EquationMode> for EquationModeRaw {
    fn from(value: &EquationMode) -> Self {
        match value {
            EquationMode::Single { .. } => Self::SingleEquation,
            EquationMode::SystemOfEquations { .. } => Self::SystemOfEquations,
        }
    }
}
