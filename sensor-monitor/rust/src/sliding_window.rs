use crate::ring_buffer::RingBuffer;

/// Sliding window statistics over a fixed-size ring buffer.
pub struct SlidingWindow {
    buffer: RingBuffer<f64>,
}

impl SlidingWindow {
    pub fn new(window_size: usize) -> Self {
        SlidingWindow {
            buffer: RingBuffer::new(window_size),
        }
    }

    pub fn push(&mut self, value: f64) {
        self.buffer.push(value);
    }

    pub fn average(&self) -> f64 {
        if self.buffer.len() == 0 {
            return 0.0;
        }
        let sum: f64 = self.buffer.iter().sum();
        sum / self.buffer.len() as f64
    }

    pub fn maximum(&self) -> f64 {
        self.buffer
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
    }

    pub fn minimum(&self) -> f64 {
        self.buffer
            .iter()
            .copied()
            .fold(f64::INFINITY, f64::min)
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window() {
        let mut sw = SlidingWindow::new(3);
        sw.push(10.0); sw.push(20.0); sw.push(30.0);
        assert!((sw.average() - 20.0).abs() < 1e-10);
        assert!((sw.maximum() - 30.0).abs() < 1e-10);
        assert!((sw.minimum() - 10.0).abs() < 1e-10);
        sw.push(40.0);
        assert!((sw.average() - 30.0).abs() < 1e-10);
        assert!((sw.maximum() - 40.0).abs() < 1e-10);
    }
}