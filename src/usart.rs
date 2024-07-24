use core::arch::asm;
use core::cell::UnsafeCell;

use ruduino::cores::current::SREG;
use ruduino::cores::current::UCSR0A;
use ruduino::cores::current::UCSR0B;
use ruduino::cores::current::UCSR0C;
use ruduino::cores::current::USART0;
use ruduino::modules::HardwareUsart;
use ruduino::Register;

use ruduino::interrupt::without_interrupts;
use ruduino::RegisterBits;

use crate::lazy::Lazy;
use crate::ring_buffer;
use crate::ring_buffer::RingBuffer;

const MAX_FRAME_SIZE: usize = 64;

enum State {
    Idle,
    Buffer,
    // idea with transfers is discarded
    // transfers are redundant here:
    // they complicate logic alot, while theirs functions
    // can be emulated easily with blocking read and writes
    // to arrays
}

unsafe impl Sync for Usart<USART0> {}

pub struct UsartInner<T: HardwareUsart> {
    state: State,
    hardware: T,
    read_buffer: RingBuffer<u8, MAX_FRAME_SIZE>,
    write_buffer: RingBuffer<u8, MAX_FRAME_SIZE>,
}

pub struct Usart<T: HardwareUsart> {
    pub inner: UnsafeCell<UsartInner<T>>,
}

enum OperationMode {
    Async,
    Sync,
    MasterSPI,
}

impl From<OperationMode> for RegisterBits<UCSR0C> {
    fn from(value: OperationMode) -> Self {
        // constants are defined in atmega328p reference
        match value {
            OperationMode::Async => RegisterBits::zero(),
            OperationMode::Sync => UCSR0C::UMSEL00, // 1
            OperationMode::MasterSPI => UCSR0C::UMSEL0, // 3
        }
    }
}

struct UsartConfig {
    pub baud_rate: u32,
}

#[derive(Debug)]
pub enum UsartError {
    Blocked,
}

// mutex is not necessary, because we are in single thread environment
pub static USART: Lazy<Usart<USART0>> =
    Lazy::new(|| Usart::configre(UsartConfig { baud_rate: 250_000 }));

impl Usart<USART0> {
    fn configre(UsartConfig { baud_rate }: UsartConfig) -> Usart<USART0> {
        without_interrupts(|| {
            // set maximum baud rate
            <USART0 as HardwareUsart>::BaudRateRegister::write(
                ((ruduino::config::CPU_FREQUENCY_HZ / 16 / baud_rate) - 1) as u16,
                // 3_u16,
            );
            // operation register
            // <USART0 as HardwareUsart>::ControlRegisterA::set(UCSR0A::U2X0);
            <USART0 as HardwareUsart>::ControlRegisterA::write(0);
            // configuration register
            
            let config = UCSR0C::UCSZ0 | OperationMode::Async.into();
            <USART0 as HardwareUsart>::ControlRegisterC::write(config);
            // enable usart
            <USART0 as HardwareUsart>::ControlRegisterB::set(
                UCSR0B::TXEN0 | UCSR0B::UDRIE0 | UCSR0B::RXCIE0 | UCSR0B::RXEN0,
            );
            Usart {
                inner: UnsafeCell::new(UsartInner {
                    read_buffer: RingBuffer::new(),
                    write_buffer: RingBuffer::new(),
                    state: State::Idle,
                    hardware: USART0,
                }),
            }
        })
    }

    pub fn write_byte(&self, byte: u8) -> Result<(), UsartError> {
        without_interrupts(|| {
            let inner = unsafe { self.get_inner_mut() };
            match inner.state {
                State::Idle => {
                    // when status is idle, it's guaranteed that write buffer is empty
                    inner.write_buffer.push_back(byte);
                    unsafe { inner.set_state(State::Buffer) };
                    Ok(())
                }
                State::Buffer => return inner.write_buffer.push_back(byte).into(),
            }
        })
    }

    pub fn write_blocking(&self, data: &[u8]) {
        for &byte in data {
            self.write_byte_blocking(byte);
        }
    }

    pub fn read_byte(&self) -> Result<u8, UsartError> {
        without_interrupts(|| {
            let inner = unsafe { self.get_inner_mut() };
            inner.read_buffer.pop_front().ok_or(UsartError::Blocked)
        })
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<(), UsartError> {
        without_interrupts(|| {
            let inner: &UsartInner<USART0> = unsafe { self.get_inner_mut() };
            if inner.read_buffer.size() < buffer.len() {
                return Err(UsartError::Blocked);
            }

            for byte in buffer {
                *byte = self
                    .read_byte()
                    .expect("buffer contains enough data for us");
            }

            Ok(())
        })
    }
    pub fn read_blocking(&self, buffer: &mut [u8]) {
        for byte in buffer {
            *byte = self.read_byte_blocking();
        }
    }

    pub fn read_byte_blocking(&self) -> u8 {
        loop {
            match self.read_byte() {
                Ok(byte) => break byte,
                Err(UsartError::Blocked) => continue,
            }
        }
    }

    unsafe fn get_inner_mut(&self) -> &mut UsartInner<USART0> {
        &mut *self.inner.get()
    }

    pub fn write_byte_blocking(&self, byte: u8) {
        while let Err(UsartError::Blocked) = self.write_byte(byte) {}
    }
}

impl From<ring_buffer::Status> for Result<(), UsartError> {
    fn from(value: ring_buffer::Status) -> Self {
        match value {
            ring_buffer::Status::Success => Ok(()),
            ring_buffer::Status::BufferFull => Err(UsartError::Blocked),
        }
    }
}

impl UsartInner<USART0> {
    unsafe fn set_state(&mut self, state: State) {
        match state {
            State::Idle => {
                unsafe { self.disable_output() };
            }
            State::Buffer => unsafe { self.enable_output() },
        }

        self.state = state;
    }

    /// in case of buffer overflow excessive bytes are silently discarded
    unsafe fn read_byte_actual(&mut self) {
        let byte = <USART0 as HardwareUsart>::DataRegister::read();
        self.read_buffer.push_back(byte);
    }

    unsafe fn write_byte_actual(&mut self) {
        let byte = self.write_buffer.pop_front();
        match byte {
            Some(byte) => <USART0 as HardwareUsart>::DataRegister::write(byte),
            None => self.set_state(State::Idle),
        }
    }

    unsafe fn is_available_write(&self) -> bool {
        <USART0 as HardwareUsart>::ControlRegisterA::is_set(UCSR0A::UDRE0)
    }

    unsafe fn enable_output(&mut self) {
        <USART0 as HardwareUsart>::ControlRegisterB::set(UCSR0B::UDRIE0);
    }

    unsafe fn disable_output(&mut self) {
        <USART0 as HardwareUsart>::ControlRegisterB::unset(UCSR0B::UDRIE0);
    }
}

// USART DATA REGISTER EMPTY
#[no_mangle]
extern "avr-interrupt" fn __vector_19() {
    let inner = unsafe { USART.get_inner_mut() };
    unsafe { inner.write_byte_actual() }
}

// USART READ
#[no_mangle]
extern "avr-interrupt" fn __vector_18() {
    let inner = unsafe { USART.get_inner_mut() };
    unsafe { inner.read_byte_actual() }
}

enum InterruptsStatus {
    Enabled,
    Disabled,
}

impl From<bool> for InterruptsStatus {
    fn from(value: bool) -> Self {
        match value {
            true => InterruptsStatus::Enabled,
            false => InterruptsStatus::Disabled,
        }
    }
}

#[inline]
pub fn without_interrupts_blink<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let _disabled = InterruptsStatus::disable_safe();
    f()
}

impl InterruptsStatus {
    #[inline]
    pub fn disable_safe() -> InterruptsStatus {
        let status: InterruptsStatus = SREG::is_set(SREG::I).into();
        unsafe { asm!("CLI") };
        status
    }
}

impl Drop for InterruptsStatus {
    #[inline]
    fn drop(&mut self) {
        if let InterruptsStatus::Enabled = self {
            unsafe { asm!("SEI") };
            // blink(2, 50);
        }
    }
}
