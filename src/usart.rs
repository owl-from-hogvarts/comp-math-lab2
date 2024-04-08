use core::any::Any;

use avr_libc::UMSEL00;
use ruduino::cores::current::port;
use ruduino::cores::current::UCSR0A;
use ruduino::cores::current::UCSR0C;
use ruduino::cores::current::USART0;
use ruduino::modules::HardwareUsart;
use ruduino::Pin;
use ruduino::Register;
use ruduino::RegisterBits;

use ruduino::interrupt::without_interrupts;

use crate::blink;
use crate::lazy::Lazy;
use crate::mutex::Mutex;

const MAX_FRAM_SIZE: usize = 50;

// enum State {
//     Idle,
//     Buffer,
//     Transfer(&[u8]),
// }

pub struct UsartInner<T: HardwareUsart> {
    inner: T,
    read_buffer: [u8; MAX_FRAM_SIZE],
    write_buffer: [u8; MAX_FRAM_SIZE],
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

pub static USART: Lazy<Mutex<UsartInner<USART0>>> =
    Lazy::new(|| Mutex::new(UsartInner::configre(UsartConfig { baud_rate: 250_000 })));

impl UsartInner<USART0> {
    pub fn configre(UsartConfig { baud_rate }: UsartConfig) -> UsartInner<USART0> {
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
            <USART0 as HardwareUsart>::ControlRegisterB::write(0b0001_1000);
            // configuration register
            let config = <OperationMode as Into<u8>>::into(OperationMode::Async)
                | (1 << avr_libc::UCSZ01)
                | (1 << avr_libc::UCSZ00);
            <USART0 as HardwareUsart>::ControlRegisterC::write(config);
            UsartInner {
                inner: USART0,
                read_buffer: [0; MAX_FRAM_SIZE],
                write_buffer: [0; MAX_FRAM_SIZE],
            }
        })
    }

    unsafe fn write_byte(&mut self, byte: u8) {
        <USART0 as HardwareUsart>::DataRegister::write(byte);
    }

    unsafe fn is_available_write(&self) -> bool {
        <USART0 as HardwareUsart>::ControlRegisterA::is_set(UCSR0A::UDRE0)
    }

    // unsafe fn reset_available_write(&mut self) {
    //     <USART0 as HardwareUsart>::ControlRegisterA::unset(UCSR0A::TXC0)
    // }
}

impl Mutex<UsartInner<USART0>> {
    pub fn write_byte(&self, byte: u8) -> WriteStatus {
        without_interrupts(|| match self.lock() {
            Some(mut usart) => {
                unsafe { usart.write_byte(byte) };
                WriteStatus::Success
            }
            None => WriteStatus::Blocked,
        })
    }

    pub fn write_byte_blocking(&self, byte: u8) -> WriteStatus {
        without_interrupts(|| match self.lock() {
            Some(mut usart) => {
                while !(unsafe { usart.is_available_write() }) {}
                // unsafe { usart.reset_available_write() }
                unsafe { usart.write_byte(byte) }
                WriteStatus::Success
            }
            None => WriteStatus::Blocked,
        })
    }
}

// extern "avr-interrupt" fn __vector_21() {
// transmit complete flag is cleared when this interrupt is called
// }
