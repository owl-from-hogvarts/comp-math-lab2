/// If state of a button stays the same for at least
/// debounce delay, callback is triggered

/// Pin Change interrupt resets timer for a toggled button,
/// if new pin's state is HIGH set time to actual state change.
/// Timer is configured with that much milliseconds.
/// Upon timer expiration, timer finds a button with least time to state change.
/// Timer reschedules itself with the time obtained on the previouse step.
/// Timer executes callback.

/// Timer reset routine should be shared accross Pin Change Interrupt and
/// Timer Expiration Interrupt. This ensures consistency in logic.
use core::cell::UnsafeCell;

use ruduino::{
    cores::current::{OCR0A, PCICR, PCMSK0, PCMSK1, PINB, TCCR0A, TCCR0B, TCNT0, TIMSK0},
    interrupt::without_interrupts,
    Register,
};

use crate::{blink, lazy::Lazy};

const BUTTONS_AMOUNT: usize = 4;

struct DebouncedButtonsContextInner<const BUTTONS_COUNT: usize> {
    buttons: [Button; BUTTONS_COUNT],
    timer: Timer,
}

pub struct DebouncedButtonsContext<const BUTTONS_COUNT: usize> {
    inner: UnsafeCell<DebouncedButtonsContextInner<{ BUTTONS_COUNT }>>,
}

impl<const BUTTONS_COUNT: usize> DebouncedButtonsContext<{ BUTTONS_COUNT }> {
    fn get_context_mut(&self) -> &mut DebouncedButtonsContextInner<{ BUTTONS_COUNT }> {
        unsafe { &mut *self.inner.get() }
    }

    pub fn set_callback(&self, callback: fn() -> (), button_id: usize) {
        without_interrupts(|| {
            self.get_context_mut().buttons[button_id].callback = Some(callback);
        })
    }
}

pub static DEBOUNCED_BUTTONS_CONTEXT: Lazy<DebouncedButtonsContext<{ BUTTONS_AMOUNT }>> =
    Lazy::new(|| {
        PCMSK0::write(0b1111);
        PCICR::set(PCICR::PCIE0);

        DebouncedButtonsContext {
            inner: UnsafeCell::new(DebouncedButtonsContextInner {
                buttons: core::array::from_fn(|_| Button::default()),
                timer: Timer::default(),
            }),
        }
    });

/// ***Tick is `1.024 ms`***
///
/// Maximum tick amount is 16!
///
/// Tick is not exactly equal to 1 ms!
/// This is limitation of hardware.
/// Won't fix, since such error is acceptable.
/// Could account for error, but application domain (i.e. debounce of buttons)
/// does not require such precision.
///
/// 16 ticks - 16 ms = 0.384 ms
#[derive(Clone, Copy)]
struct Tick(u8);

impl Tick {
    const fn new<const TICK_AMOUNT: u8>() -> Self {
        assert!(
            TICK_AMOUNT <= 16,
            "TICK_AMOUNT should be lower or equal to 16!"
        );

        Self(TICK_AMOUNT)
    }

    fn from_output_compare_value(value: u8) -> Self {
        // as value is u8, it can't be higher than 255
        // thus
        let (value, overflow) = value.overflowing_add(1);
        let overflow = overflow as u8;
        Tick(value >> 4 | overflow << 4)
    }

    fn has_expired(&self) -> bool {
        const TIMER_EXPIRATION_TRESHOLD: u8 = 16;
        self.0 <= TIMER_EXPIRATION_TRESHOLD
    }

    fn to_output_comapre_value(&self) -> u8 {
        // -1 to prevent overflow:
        // 16 * 16 = 256 -- value exceeds u8 range.
        // Therefore norimilizing the value
        (self.0 * 16).wrapping_sub(1)
    }
}

#[derive(Default)]
struct Button {
    /// ***!!! CALLBACK MUST NOT ENABLE INTERRUPTS !!!***
    ///
    /// Enabling interrupts within callback breaks some pre-conditions.
    /// This results into multiple mutable references to buttons array.
    callback: Option<fn() -> ()>,
    state: ButtonState,
}

#[derive(Default, Clone, Copy)]
enum ButtonState {
    /// Button has been pressed down for less than debounce time.
    Down { time_remaining: Tick },
    /// Button is pressed longer than debounce time. Callback has been executed
    Pressed,
    /// Button is not pressed.
    #[default]
    Up,
}

impl ButtonState {
    /// Returns `true` if the button state is [`Down`].
    ///
    /// [`Down`]: ButtonState::Down
    #[must_use]
    fn is_down(&self) -> bool {
        matches!(self, Self::Down { .. })
    }

    /// Returns `true` if the button state is [`Pressed`].
    ///
    /// [`Pressed`]: ButtonState::Pressed
    #[must_use]
    fn is_pressed(&self) -> bool {
        matches!(self, Self::Pressed)
    }

    /// Returns `true` if the button state is [`Up`].
    ///
    /// [`Up`]: ButtonState::Up
    #[must_use]
    fn is_up(&self) -> bool {
        matches!(self, Self::Up)
    }
}

const DEBOUNCE_DELAY: Tick = Tick::new::<16>();

impl ButtonState {
    fn update_changed(&mut self, is_physically_pressed: bool) -> &mut Self {
        if (self.is_down() || self.is_pressed()) != is_physically_pressed {
            match is_physically_pressed {
                true => {
                    *self = ButtonState::Down {
                        time_remaining: DEBOUNCE_DELAY,
                    }
                }
                false => *self = ButtonState::Up,
            }
        }

        self
    }
}

#[derive(Default)]
struct Timer {
    state: TimerState,
}

#[derive(Default)]
enum TimerState {
    #[default]
    Disabled,
    Running,
    Expired,
}

impl Timer {
    // clk_IO / 1024 -- prescaled clock
    const CLOCK_DISABLE: u8 = 0b111;
    const CLOCK_SELECT: u8 = 0b101;
    // Clear Timer register on compare match
    const WAVEFORM_SELECT: u8 = 0b10;
    const BOTTOM: u8 = 0;

    // fn update(&mut self, buttons: &mut [Button]) {
    //     let any_down = buttons.iter().any(|b| b.state.is_down());

    //     match (&self.state, any_down) {
    //         (TimerState::Disabled, false) | (TimerState::Running, true) => (), // do nothing
    //         (TimerState::Disabled, true) => todo!("schedule"),
    //         (TimerState::Expired, _) => {
    //             // find lowest time above treshold
    //             // schedule timer with the time from previous step
    //             // if there is no such time -> disable timer
    //             // pick buttons with time below treshold
    //             // run their callbacks
    //             todo!("schedule");
    //             todo!("update buttons' state to [Pressed]");
    //             // run callbacks for buttons with remaining time below treshold
    //             for button in buttons
    //                 .iter_mut()
    //                 .filter(|button| button.state.is_pressed())
    //             {
    //                 (button.callback)();
    //                 button.state = ButtonState::Pressed;
    //             }
    //         }
    //         (TimerState::Running, false) => todo!("disable timer"),
    //     }
    // }

    fn schedule(&mut self, time: Tick) {
        without_interrupts(|| {
            self.state = TimerState::Running;
            OCR0A::write(time.to_output_comapre_value());
            // counting from BOTTOM to MAX
            TCNT0::write(Timer::BOTTOM);
            // enable interrupt
            TIMSK0::set(TIMSK0::OCIE0A0);
            TCCR0A::write(Timer::WAVEFORM_SELECT);
            // connect clock to the timer
            TCCR0B::write(Timer::CLOCK_SELECT);
        })
    }

    /// Stop and reset timer
    fn disable(&mut self) {
        without_interrupts(|| {
            self.state = TimerState::Disabled;
            // disconnect clock from the timer.
            // This effectivily stops timer
            TCCR0B::unset_mask_raw(Timer::CLOCK_DISABLE);
            TIMSK0::unset(TIMSK0::OCIE0A);
            TCNT0::write(0);
        })
    }
}

// Pin Change Interrupt triggered.
// If pin for Button is HIGH, set ButtonState to Down with debounce time.
// If pin for Button is LOW, set ButtonState to Up.
// In both cases update timer:
// // - if all buttons are Up -> disable timer
// next thing can be handled by timer routine
// - if Some buttons are Down -> get timer state.
//                               if timer is Disabled -> configure timer
//                               else -> do nothing. Timer is already scheduled. It will
//                                                                                   fire soon
#[no_mangle]
extern "avr-interrupt" fn __vector_3() {
    let buttons: &mut [Button] = &mut DEBOUNCED_BUTTONS_CONTEXT.get_context_mut().buttons;

    // Theoretically, more than one bit can change at a time.
    let new_state_raw: u8 = PINB::read();

    let mut mask = 0b1;
    for button in &mut *buttons {
        let new_state = new_state_raw & mask != 0;
        mask <<= 1;
        button.state.update_changed(new_state);
    }

    let timer: &mut Timer = &mut DEBOUNCED_BUTTONS_CONTEXT.get_context_mut().timer;

    let all_up = buttons.iter().all(|b| !b.state.is_down());
    match all_up {
        true => timer.disable(),
        false => match timer.state {
            // the use of constant may be replaced with lowest time from
            // actual buttons' states. Also, the whole match statement
            // could be merged with one from Timer Interrupt,
            // hard to get right though. May require new state for
            // buttons.
            TimerState::Disabled => timer.schedule(DEBOUNCE_DELAY),
            TimerState::Running | TimerState::Expired => (),
        },
    }
}

// Timer Interrupt triggered.
// Subtract elapsed time from each Down button.
// Find button with lowest remaning time
// Schedule timer with the time from previous step.
// If some button have time less than elapsed,
// Set state to Pressed and call it's callback.
#[no_mangle]
extern "avr-interrupt" fn __vector_14() {
    let DebouncedButtonsContextInner { buttons, timer } =
        DEBOUNCED_BUTTONS_CONTEXT.get_context_mut();

    let elapsed_time: Tick = Tick::from_output_compare_value(TCNT0::read());
    let mut lowest_time: Option<Tick> = None;
    for button in &mut *buttons {
        if let ButtonState::Down { time_remaining } = &mut button.state {
            // update remaining time
            time_remaining.0 = time_remaining.0.saturating_sub(elapsed_time.0);

            // timer has *almost* expired
            // This should avoid frequent timer reschedules
            // Do not call callbacks here to reschedule timer as soon as possible
            if time_remaining.has_expired() {
                continue;
            }

            // find lowest time
            match lowest_time {
                Some(lowest) if time_remaining.0 < lowest.0 => lowest_time = Some(*time_remaining),
                None => lowest_time = Some(*time_remaining),
                _ => (),
            }
        }
    }

    match lowest_time {
        Some(next_time) => timer.schedule(next_time),
        None => timer.disable(),
    }

    for button in buttons {
        if let ButtonState::Down { time_remaining } = button.state {
            if time_remaining.has_expired() {
                if let Some(callback) = button.callback {
                    (callback)();
                }
                button.state = ButtonState::Pressed;
            }
        }
    }
}
