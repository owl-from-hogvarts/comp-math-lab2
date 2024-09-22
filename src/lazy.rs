use core::ops::Deref;

use crate::interrupts::without_interrupts;

enum LazyState<T, F> {
    Uninit(F),
    Init(T),
    Poisoned,
}

/// Unsafe notice!
/// Lazy is prooperly syncronized for atmega328:
/// Cricital sections are executed with interrupts disabled
unsafe impl<T, F> Sync for Lazy<T, F> {}

pub struct Lazy<T, F = fn() -> T> {
    state: LazyState<T, F>,
}

impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub const fn new(f: F) -> Lazy<T, F> {
        Lazy {
            state: LazyState::Uninit(f),
        }
    }

    fn init(&self) -> &T {
        let self_mut: *const Lazy<T, F> = self;
        let self_mut = self_mut.cast_mut();
        unsafe {
            let state = (&mut (*self_mut).state) as *mut LazyState<T, F>;
            let LazyState::Uninit(f) = state.replace(LazyState::Poisoned) else {
                unreachable!()
            };

            let data = f();
            *state = LazyState::Init(data);

            let LazyState::Init(data) = &self.state else {
                unreachable!()
            };

            data
        }
    }
}

impl<T, F: FnOnce() -> T> Deref for Lazy<T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        without_interrupts(|| {
            let data = match &self.state {
                LazyState::Uninit(_) => self.init(),
                LazyState::Init(data) => data,
                LazyState::Poisoned => {
                    panic!("LazyCell has previously been poisoned")
                }
            };
            data
        })

        // blink(100, 50);
        // if !<SREG as ruduino::Register>::is_set(SREG::I) {
        // }

        // data
    }
}
