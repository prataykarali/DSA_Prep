/// A min-heap priority queue.
/// Elements must implement Ord. The smallest element is at the root.
pub struct MinHeap<T: Ord> {
    heap: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap { heap: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.heap.push(value);
        self.sift_up(self.heap.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let last = self.heap.len() - 1;
        self.heap.swap(0, last);
        let result = self.heap.pop();
        if !self.heap.is_empty() {
            self.sift_down(0);
        }
        result
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.first()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = (index - 1) / 2;
            if self.heap[index] < self.heap[parent] {
                self.heap.swap(index, parent);
                index = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        let size = self.heap.len();
        loop {
            let mut smallest = index;
            let left = 2 * index + 1;
            let right = 2 * index + 2;
            if left < size && self.heap[left] < self.heap[smallest] {
                smallest = left;
            }
            if right < size && self.heap[right] < self.heap[smallest] {
                smallest = right;
            }
            if smallest != index {
                self.heap.swap(index, smallest);
                index = smallest;
            } else {
                break;
            }
        }
    }
}

// --- stdlib equivalent ---
// std::collections::BinaryHeap is a max-heap by default.
// For a min-heap, wrap values with std::cmp::Reverse:
//   use std::collections::BinaryHeap;
//   use std::cmp::Reverse;
//   let mut heap = BinaryHeap::new();
//   heap.push(Reverse((priority, item)));
//   let Reverse((p, item)) = heap.pop().unwrap();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_queue() {
        let mut pq = MinHeap::new();
        pq.push(3); pq.push(1); pq.push(2);
        assert_eq!(pq.peek(), Some(&1));
        assert_eq!(pq.pop(), Some(1));
        assert_eq!(pq.pop(), Some(2));
        assert_eq!(pq.pop(), Some(3));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_with_tuples() {
        let mut pq: MinHeap<(i32, &str)> = MinHeap::new();
        pq.push((3, "low")); pq.push((1, "high")); pq.push((2, "medium"));
        assert_eq!(pq.pop(), Some((1, "high")));
        assert_eq!(pq.pop(), Some((2, "medium")));
        assert_eq!(pq.pop(), Some((3, "low")));
    }
}