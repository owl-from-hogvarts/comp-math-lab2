use core::arch::asm;
use core::cell::UnsafeCell;

use avr_libc::UMSEL00;
use ruduino::cores::current::SREG;
use ruduino::cores::current::UCSR0A;
use ruduino::cores::current::UCSR0B;
use ruduino::cores::current::USART0;
use ruduino::delay::delay_ms;
use ruduino::modules::HardwareUsart;
use ruduino::Register;

use ruduino::interrupt::without_interrupts;

use crate::blink;
use crate::lazy::Lazy;
use crate::ring_buffer;
use crate::ring_buffer::RingBuffer;

const MAX_FRAME_SIZE: usize = 50;

enum State {
    Idle,
    Buffer,
    // Transfer(&'a [u8]),
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

impl From<OperationMode> for u8 {
    fn from(value: OperationMode) -> Self {
        // constants are defined in atmega328p reference
        let bits = match value {
            OperationMode::Async => 0,
            OperationMode::Sync => 1,
            OperationMode::MasterSPI => 3,
        };

        bits << UMSEL00
    }
}

struct UsartConfig {
    pub baud_rate: u32,
}

pub enum WriteStatus {
    Success,
    Blocked,
}

// mutex is not necessary, because we are in single thread environment
pub static USART: Lazy<Usart<USART0>> =
    Lazy::new(|| Usart::configre(UsartConfig { baud_rate: 250_000 }));

impl Usart<USART0> {
    pub fn configre(UsartConfig { baud_rate }: UsartConfig) -> Usart<USART0> {
        without_interrupts(|| {
            // set maximum baud rate
            <USART0 as HardwareUsart>::BaudRateRegister::write(
                ((ruduino::config::CPU_FREQUENCY_HZ / 16 / baud_rate) - 1) as u16,
                // 3_u16,
            );
            // operation register
            // <USART0 as HardwareUsart>::ControlRegisterA::set(UCSR0A::U2X0);
            <USART0 as HardwareUsart>::ControlRegisterA::write(0);
            // enable usart
            <USART0 as HardwareUsart>::ControlRegisterB::set(UCSR0B::TXEN0 | UCSR0B::TXCIE0);
            // configuration register
            let config = <OperationMode as Into<u8>>::into(OperationMode::Async)
                | (1 << avr_libc::UCSZ01)
                | (1 << avr_libc::UCSZ00);
            <USART0 as HardwareUsart>::ControlRegisterC::write(config);
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

    pub fn write_byte(&self, byte: u8) -> WriteStatus {
        without_interrupts(|| {
            let inner = unsafe { self.get_inner_mut() };
            match inner.state {
                State::Idle => {
                    // inner.state = State::Buffer;
                    // when status is idle, it's guaranteed that write buffer is empty
                    inner.write_buffer.push_back(byte);
                    // unsafe { inner.write_byte_actual() };
                    inner.set_state(State::Buffer);
                    WriteStatus::Success
                }
                State::Buffer => return inner.write_buffer.push_back(byte).into(),
                // State::Transfer(_) => todo!(),
            }
        })
    }

    pub fn read_byte(&self) {}

    unsafe fn get_inner_mut(&self) -> &mut UsartInner<USART0> {
        &mut *self.inner.get()
    }

    pub fn write_byte_blocking(&self, byte: u8) {
        without_interrupts(|| while let WriteStatus::Blocked = self.write_byte(byte) {});
    }
}

impl From<ring_buffer::Status> for WriteStatus {
    fn from(value: ring_buffer::Status) -> Self {
        match value {
            ring_buffer::Status::Success => WriteStatus::Success,
            ring_buffer::Status::BufferFull => WriteStatus::Blocked,
        }
    }
}

impl UsartInner<USART0> {
    fn set_state(&mut self, state: State) {
        match state {
            State::Idle => {
                unsafe { self.disable_output() };
            }
            State::Buffer => unsafe { self.enable_output() },
        }

        self.state = state;
    }

    unsafe fn read_byte_actual(&mut self) {
        let byte = <USART0 as HardwareUsart>::DataRegister::read();
        self.read_buffer.push_back(byte);
    }

    unsafe fn write_byte_actual(&mut self) {
        let byte = self.write_buffer.pop_front();
        match byte {
            Some(byte) => unsafe {
                <USART0 as HardwareUsart>::DataRegister::write(byte);
            },
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

#[inline(always)]
pub fn without_interrupts_blink<F, T>(f: F) -> T
where
    F: FnOnce() -> T,
{
    let _disabled = InterruptsStatus::disable_safe();
    f()
}

impl InterruptsStatus {
    #[inline(always)]
    pub fn disable_safe() -> InterruptsStatus {
        let status: InterruptsStatus = SREG::is_set(SREG::I).into();
        unsafe { asm!("CLI") };
        status
    }
}

impl Drop for InterruptsStatus {
    #[inline(always)]
    fn drop(&mut self) {
        if let InterruptsStatus::Enabled = self {
            unsafe { asm!("SEI") };
            // blink(2, 50);
        }
    }
}
