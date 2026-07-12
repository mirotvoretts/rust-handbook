//! 03 (1x) - кольцевой буфер на const generic. Эталонное решение.
pub struct RingBuffer<T, const N: usize> {
    data: [Option<T>; N],
    head: usize,
    len: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    pub fn new() -> Self {
        RingBuffer {
            data: std::array::from_fn(|_| None),
            head: 0,
            len: 0,
        }
    }
    pub fn capacity(&self) -> usize {
        N
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn push(&mut self, value: T) {
        let tail = (self.head + self.len) % N;
        self.data[tail] = Some(value);
        if self.len == N {
            self.head = (self.head + 1) % N;
        } else {
            self.len += 1;
        }
    }
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let value = self.data[self.head].take();
        self.head = (self.head + 1) % N;
        self.len -= 1;
        value
    }
}

impl<T, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
