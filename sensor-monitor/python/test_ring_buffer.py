from sensor_monitor.ring_buffer import RingBuffer
from sensor_monitor.priority_queue import PriorityQueue
from sensor_monitor.sliding_window import SlidingWindow


def test_ring_buffer_below_capacity():
    rb = RingBuffer(5)
    rb.push(1); rb.push(2); rb.push(3)
    assert len(rb) == 3
    assert rb.get(0) == 1
    assert rb.get(2) == 3
    assert list(rb) == [1, 2, 3]


def test_ring_buffer_at_capacity():
    rb = RingBuffer(3)
    rb.push(1); rb.push(2); rb.push(3)
    assert rb.is_full()
    rb.push(4)
    assert len(rb) == 3
    assert list(rb) == [2, 3, 4]
    rb.push(5)
    assert list(rb) == [3, 4, 5]


def test_sliding_window():
    sw = SlidingWindow(3)
    sw.push(10.0); sw.push(20.0); sw.push(30.0)
    assert sw.average() == 20.0
    assert sw.maximum() == 30.0
    assert sw.minimum() == 10.0
    sw.push(40.0)
    assert sw.average() == 30.0
    assert sw.maximum() == 40.0


def test_priority_queue():
    pq = PriorityQueue()
    pq.push(3, "low")
    pq.push(1, "high")
    pq.push(2, "medium")
    assert pq.peek() == (1, "high")
    assert pq.pop() == (1, "high")
    assert pq.pop() == (2, "medium")
    assert pq.pop() == (3, "low")


if __name__ == "__main__":
    test_ring_buffer_below_capacity()
    test_ring_buffer_at_capacity()
    test_sliding_window()
    test_priority_queue()
    print("All tests passed!")