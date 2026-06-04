from collections import deque


class RingBuffer:
    """Fixed-capacity circular buffer. Overwrites oldest on overflow."""

    def __init__(self, capacity: int):
        if capacity <= 0:
            raise ValueError("Capacity must be positive")
        self.capacity = capacity
        self._buffer = [None] * capacity
        self._head = 0  # index of oldest element
        self._tail = 0  # index of next write position
        self._size = 0

    def push(self, value):
        """Insert a value. If full, overwrites the oldest element."""
        self._buffer[self._tail] = value
        if self._size == self.capacity:
            # Overwriting oldest: advance head
            self._head = (self._head + 1) % self.capacity
        else:
            self._size += 1
        self._tail = (self._tail + 1) % self.capacity
    

    def get(self, index: int):
        """Access element at logical index (0 = oldest)."""
        if index < 0 or index >= self._size:
            raise IndexError(f"Index {index} out of range (size={self._size})")
        actual = (self._head + index) % self.capacity
        return self._buffer[actual]

    def __len__(self) -> int:
        return self._size

    def is_full(self) -> bool:
        return self._size == self.capacity

    def __iter__(self):
        """Iterate from oldest to newest."""
        for i in range(self._size):
            yield self._buffer[(self._head + i) % self.capacity]
