use crate::{TNumber, PACKAGE_SIZE, T_NUMBER_SIZE_BYTES};
use crate::byte_serializable::{read_field, ByteSerializable};

pub enum EquationMode {
    Linear { method: Method, equation_number: u8 },
    NonLinear,
}

impl EquationMode {
    const LINEAR_MODE: u8 = 0;
    const NON_LINEAR_MODE: u8 = 1;

    const EQUATION_MODE_SIZE: usize = 3;

    const METHOD_OFFSET: usize = 1;
    const EQUATION_OFFSET: usize = 2;
}

impl ByteSerializable<{ Self::EQUATION_MODE_SIZE }> for EquationMode {
    fn to_bytes(&self) -> [u8; Self::EQUATION_MODE_SIZE] {
        let mut bytes = [0_u8; Self::EQUATION_MODE_SIZE];
        match self {
            EquationMode::Linear {
                method,
                equation_number,
            } => {
                bytes[0] = Self::LINEAR_MODE;
                bytes[Self::METHOD_OFFSET] = method.to_byte();
                bytes[Self::EQUATION_OFFSET] = *equation_number;
            }
            EquationMode::NonLinear => bytes[0] = Self::NON_LINEAR_MODE,
        }

        bytes
    }

    fn from_bytes(raw_bytes: [u8; Self::EQUATION_MODE_SIZE]) -> Self {
        match raw_bytes[0] {
            Self::LINEAR_MODE => Self::Linear {
                method: Method::from_byte(raw_bytes[Self::METHOD_OFFSET]),
                equation_number: raw_bytes[Self::EQUATION_OFFSET],
            },
            Self::NON_LINEAR_MODE => Self::NonLinear,
            _ => unreachable!(),
        }
    }
}

pub struct ComputeRootPayload {
    pub epsilon: TNumber,
    pub mode: EquationMode,
}

impl ComputeRootPayload {
    const MODE_OFFSET: usize = RequestPackage::REQUEST_PAYLOAD_OFFSET + T_NUMBER_SIZE_BYTES;
}

pub enum RequestPackage {
    FunctionPoints,
    InitialApproximations,
    ComputeRoot { payload: ComputeRootPayload },
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
            RequestPackage::FunctionPoints => RequestPackage::FUNCTION_POINTS_TYPE,
            RequestPackage::InitialApproximations => RequestPackage::INITIAL_APPROXIMATION_TYPE,
            RequestPackage::ComputeRoot { .. } => RequestPackage::COMPUTE_ROOT_TYPE,
        };

        let mut package: [u8; PACKAGE_SIZE] = [0; PACKAGE_SIZE];
        package[RequestPackage::REQUEST_TYPE_OFFSET] = request_type;
        match self {
            RequestPackage::FunctionPoints => (),
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

    fn from_bytes(raw_bytes: [u8; PACKAGE_SIZE]) -> RequestPackage {
        let request_type = raw_bytes[RequestPackage::REQUEST_TYPE_OFFSET];
        match request_type {
            RequestPackage::FUNCTION_POINTS_TYPE => RequestPackage::FunctionPoints,
            RequestPackage::INITIAL_APPROXIMATION_TYPE => RequestPackage::InitialApproximations,
            RequestPackage::COMPUTE_ROOT_TYPE => {
                let epsilon_bytes: [u8; T_NUMBER_SIZE_BYTES] =
                    read_field(&raw_bytes, Self::REQUEST_PAYLOAD_OFFSET);
                let mode_bytes: [u8; EquationMode::EQUATION_MODE_SIZE] =
                    read_field(&raw_bytes, ComputeRootPayload::MODE_OFFSET);

                RequestPackage::ComputeRoot {
                    payload: ComputeRootPayload {
                        epsilon: f64::from_le_bytes(epsilon_bytes),
                        mode: EquationMode::from_bytes(mode_bytes),
                    },
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
