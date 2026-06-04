/// A fixed-capacity circular buffer that overwrites the oldest element when full.
pub struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl<T: Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Capacity must be positive");
        RingBuffer {
            buffer: vec![None; capacity],
            head: 0,
            tail: 0,
            size: 0,
            capacity,
        }
    }

    /// Insert a value. If full, overwrites the oldest element.
    pub fn push(&mut self, value: T) {
        self.buffer[self.tail] = Some(value);
        if self.size == self.capacity {
            self.head = (self.head + 1) % self.capacity;
        } else {
            self.size += 1;
        }
        self.tail = (self.tail + 1) % self.capacity;
    }

    /// Access element at logical index (0 = oldest).
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.size {
            return None;
        }
        let actual = (self.head + index) % self.capacity;
        self.buffer[actual].as_ref()
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }

        /// Iterate from oldest to newest.
    pub fn iter(&self) -> RingBufferIter<T> {
        RingBufferIter {
            buffer: self,
            index: 0,
        }
    }
}

pub struct RingBufferIter<'a, T> {
    buffer: &'a RingBuffer<T>,
    index: usize,
}

impl<'a, T: Clone> Iterator for RingBufferIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.buffer.size {
            return None;
        }
        let actual = (self.buffer.head + self.index) % self.buffer.capacity;
        self.index += 1;
        self.buffer.buffer[actual].as_ref()
    }
}

// --- stdlib equivalent ---
// std::collections::VecDeque is a growable ring buffer.
// For fixed capacity, manually pop_front when len exceeds desired size:
//   use std::collections::VecDeque;
//   let mut buf: VecDeque<f64> = VecDeque::with_capacity(5);
//   buf.push_back(value);
//   if buf.len() > 5 { buf.pop_front(); }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_below_capacity() {
        let mut rb = RingBuffer::new(5);
        rb.push(1);
        rb.push(2);
        rb.push(3);
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.get(0), Some(&1));
        assert_eq!(rb.get(2), Some(&3));
        let items: Vec<&i32> = rb.iter().collect();
        assert_eq!(items, vec![&1, &2, &3]);
    }

    #[test]
    fn test_at_capacity() {
        let mut rb = RingBuffer::new(3);
        rb.push(1);
        rb.push(2);
        rb.push(3);
        assert!(rb.is_full());
        rb.push(4); // overwrites 1
        assert_eq!(rb.len(), 3);
        let items: Vec<&i32> = rb.iter().collect();
        assert_eq!(items, vec![&2, &3, &4]);
        rb.push(5); // overwrites 2
        let items: Vec<&i32> = rb.iter().collect();
        assert_eq!(items, vec![&3, &4, &5]);
    }
}