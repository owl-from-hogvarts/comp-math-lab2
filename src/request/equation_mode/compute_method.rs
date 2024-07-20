use core::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Method {
    Chord,
    Secant,
    SimpleIterationSingle,
}

impl Method {
    const CHORD: u8 = 0;
    const SECANT: u8 = 1;
    const SIMPLE_ITERATION_SINGLE: u8 = 2;

    pub fn to_byte(&self) -> u8 {
        match self {
            Method::Chord => Method::CHORD,
            Method::Secant => Method::SECANT,
            Method::SimpleIterationSingle => Method::SIMPLE_ITERATION_SINGLE,
        }
    }

    pub fn from_byte(byte: u8) -> Method {
        match byte {
            Method::CHORD => Method::Chord,
            Method::SECANT => Method::Secant,
            Method::SIMPLE_ITERATION_SINGLE => Method::SimpleIterationSingle,
            _ => unreachable!(),
        }
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let string = match self {
            Method::Chord => "Chord",
            Method::Secant => "Secant",
            Method::SimpleIterationSingle => "Simple Iteration",
        };

        write!(f, "{}", string)
    }
}
