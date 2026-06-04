from sensor_monitor.ring_buffer import RingBuffer
class SlidingWindow:
    """Sliding window statistics over a fixed-size ring buffer."""

    def __init__(self, window_size: int):
        self._buffer = RingBuffer(window_size)

    def push(self, value: float):
        """Add a reading to the window."""
        self._buffer.push(value)

    def average(self) -> float:
        """Compute the mean of the current window."""
        if len(self._buffer) == 0:
            return 0.0
        total = sum(self._buffer)
        return total / len(self._buffer)

    def maximum(self) -> float:
        """Return the max of the current window."""
        if len(self._buffer) == 0:
            return float("-inf")
        return max(self._buffer)

    def minimum(self) -> float:
        """Return the min of the current window."""
        if len(self._buffer) == 0:
            return float("inf")
        return min(self._buffer)

    def __len__(self) -> int:
        return len(self._buffer)