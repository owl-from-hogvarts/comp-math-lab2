use core::ops::{Deref, DerefMut};

use ruduino::interrupt::without_interrupts;

use crate::blink;

enum LazyState<T, F> {
    Uninit(F),
    Init(T),
    Poisoned,
}

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
        without_interrupts(|| match &self.state {
            LazyState::Uninit(_) => self.init(),
            LazyState::Init(data) => data,
            LazyState::Poisoned => panic!("LazyCell has previously been poisoned"),
        })
    }
}
