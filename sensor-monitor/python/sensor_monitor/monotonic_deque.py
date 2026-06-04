import time
from collections import deque


def sliding_window_max_naive(data: list[float], k: int) -> list[float]:
    """O(n*k) sliding window maximum."""
    if not data or k <= 0:
        return []
    result = []
    for i in range(len(data) - k + 1):
        result.append(max(data[i : i + k]))
    return result


def sliding_window_max_deque(data: list[float], k: int) -> list[float]:
    """O(n) sliding window maximum using a monotonic decreasing deque."""
    if not data or k <= 0:
        return []
    dq = deque()  # stores indices; front is always the max
    result = []

    for i, val in enumerate(data):
        # Remove indices outside the window
        while dq and dq[0] < i - k + 1:
            dq.popleft()
        # Maintain decreasing order: remove smaller elements from back
        while dq and data[dq[-1]] <= val:
            dq.pop()
        dq.append(i)
        # Start recording once we have a full window
        if i >= k - 1:
            result.append(data[dq[0]])

    return result


def benchmark():
    """Compare naive vs deque approach on a large dataset."""
    import random
    random.seed(42)
    n = 100_000
    k = 1_000
    data = [random.uniform(0, 1000) for _ in range(n)]

    # Naive approach
    start = time.perf_counter()
    result_naive = sliding_window_max_naive(data, k)
    naive_time = time.perf_counter() - start

    # Deque approach
    start = time.perf_counter()
    result_deque = sliding_window_max_deque(data, k)
    deque_time = time.perf_counter() - start

    assert result_naive == result_deque, "Results differ!"

    print(f"Dataset: {n} elements, window size: {k}")
    print(f"Naive O(n*k):  {naive_time:.3f}s")
    print(f"Deque O(n):    {deque_time:.3f}s")
    print(f"Speedup:       {naive_time / deque_time:.1f}x")


if __name__ == "__main__":
    benchmark()