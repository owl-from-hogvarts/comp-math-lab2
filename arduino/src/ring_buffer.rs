use core::ops::{Add, Index, IndexMut};

use crate::interrupts::without_interrupts;

#[derive(Clone, Copy)]
struct RingIndex<const N: usize> {
    index: usize,
}

impl<const N: usize> Add<usize> for RingIndex<N> {
    type Output = RingIndex<N>;

    fn add(self, rhs: usize) -> Self::Output {
        RingIndex {
            index: (self.index + rhs) % N,
        }
    }
}

impl<const N: usize> PartialEq<RingIndex<N>> for RingIndex<N> {
    fn eq(&self, other: &RingIndex<N>) -> bool {
        self.index == other.index
    }
}

impl<const N: usize> RingIndex<N> {
    #[inline]
    fn next(&mut self) {
        self.index = (self.index + 1) % N;
    }
}

impl<T: Default, const N: usize> Index<RingIndex<N>> for RingBuffer<T, N> {
    type Output = Option<T>;

    fn index(&self, index: RingIndex<N>) -> &Self::Output {
        &self.buffer[index.index]
    }
}

impl<T: Default, const N: usize> IndexMut<RingIndex<N>> for RingBuffer<T, N> {
    fn index_mut(&mut self, index: RingIndex<N>) -> &mut Self::Output {
        &mut self.buffer[index.index]
    }
}

pub struct RingBuffer<T, const N: usize> {
    front: RingIndex<N>,
    back: RingIndex<N>,
    buffer: [Option<T>; N],
}

pub enum Status {
    Success,
    BufferFull,
}

impl<T: Default, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        RingBuffer {
            front: RingIndex { index: 0 },
            back: RingIndex { index: 1 },
            buffer: core::array::from_fn(|_| None),
        }
    }

    pub fn push_back(&mut self, value: T) -> Status {
        without_interrupts(|| {
            if self.is_full() {
                return Status::BufferFull;
            }
            let back = self.back;
            self[back] = Some(value);
            self.back.next();

            Status::Success
        })
    }

    /// returns amount of elements currently stored in
    /// the ring buffer
    pub fn size(&self) -> usize {
        let (back, front) = without_interrupts(|| (self.back.index, self.front.index));
        if back > front {
            return back - front;
        }

        (back - front) + N
    }

    pub fn pop_front(&mut self) -> Option<T> {
        without_interrupts(|| {
            if self.is_empty() {
                return None;
            }

            self.front.next();
            let front = self.front;
            self[front].take()
        })
    }
    fn is_empty(&self) -> bool {
        self.front + 1 == self.back
    }

    fn is_full(&self) -> bool {
        self.front == self.back
    }
}
