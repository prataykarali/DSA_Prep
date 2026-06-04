import heapq


class PriorityQueue:
    """Min-heap priority queue implemented from scratch."""

    def __init__(self):
        self._heap = []

    def push(self, priority: float, item):
        """Insert an item with the given priority (lower = higher priority)."""
        self._heap.append((priority, item))
        self._sift_up(len(self._heap) - 1)

    def pop(self):
        """Remove and return (priority, item) with the lowest priority value."""
        if not self._heap:
            raise IndexError("Pop from empty priority queue")
        self._heap[0], self._heap[-1] = self._heap[-1], self._heap[0]
        result = self._heap.pop()
        if self._heap:
            self._sift_down(0)
        return result

    def peek(self):
        """View the highest-priority (lowest value) item without removing."""
        if not self._heap:
            raise IndexError("Peek on empty priority queue")
        return self._heap[0]

    def __len__(self) -> int:
        return len(self._heap)

    def _sift_up(self, index: int):
        while index > 0:
            parent = (index - 1) // 2
            if self._heap[index][0] < self._heap[parent][0]:
                self._heap[index], self._heap[parent] = self._heap[parent], self._heap[index]
                index = parent
            else:
                break

    def _sift_down(self, index: int):
        size = len(self._heap)
        while True:
            smallest = index
            left = 2 * index + 1
            right = 2 * index + 2
            if left < size and self._heap[left][0] < self._heap[smallest][0]:
                smallest = left
            if right < size and self._heap[right][0] < self._heap[smallest][0]:
                smallest = right
            if smallest != index:
                self._heap[index], self._heap[smallest] = self._heap[smallest], self._heap[index]
                index = smallest
            else:
                break


# --- stdlib equivalent ---
# import heapq
# pq = []
# heapq.heappush(pq, (priority, counter, item))
# priority, counter, item = heapq.heappop(pq)