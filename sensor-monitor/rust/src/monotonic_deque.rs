use std::collections::VecDeque;
use std::time::Instant;

/// O(n*k) sliding window maximum.
pub fn sliding_window_max_naive(data: &[f64], k: usize) -> Vec<f64> {
    if data.is_empty() || k == 0 {
        return Vec::new();
    }
    let mut result = Vec::with_capacity(data.len().saturating_sub(k - 1));
    for i in 0..=(data.len().saturating_sub(k)) {
        let window_max = data[i..i + k]
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max);
        result.push(window_max);
    }
    result
}

/// O(n) sliding window maximum using a monotonic decreasing deque.
pub fn sliding_window_max_deque(data: &[f64], k: usize) -> Vec<f64> {
    if data.is_empty() || k == 0 {
        return Vec::new();
    }
    let mut dq: VecDeque<usize> = VecDeque::new();
    let mut result = Vec::with_capacity(data.len().saturating_sub(k - 1));

    for i in 0..data.len() {
        // Remove indices outside the window
        while let Some(&front) = dq.front() {
            if front + k <= i {
                dq.pop_front();
            } else {
                break;
            }
        }
        // Maintain decreasing order: remove smaller elements from back
        while let Some(&back) = dq.back() {
            if data[back] <= data[i] {
                dq.pop_back();
            } else {
                break;
            }
        }
        dq.push_back(i);
        // Start recording once we have a full window
        if i >= k - 1 {
            result.push(data[*dq.front().unwrap()]);
        }
    }
    result
}

pub fn benchmark() {
    let n = 100_000;
    let k = 1_000;

    // Simple deterministic pseudo-random data
    let mut data = Vec::with_capacity(n);
    let mut state: u64 = 42;
    for _ in 0..n {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        data.push((state as f64) / (u64::MAX as f64) * 1000.0);
    }

    // Naive
    let start = Instant::now();
    let result_naive = sliding_window_max_naive(&data, k);
    let naive_time = start.elapsed();

    // Deque
    let start = Instant::now();
    let result_deque = sliding_window_max_deque(&data, k);
    let deque_time = start.elapsed();

    assert_eq!(result_naive.len(), result_deque.len());
    for (a, b) in result_naive.iter().zip(result_deque.iter()) {
        assert!((a - b).abs() < 1e-10, "Results differ!");
    }

    println!("Dataset: {} elements, window size: {}", n, k);
    println!("Naive O(n*k):  {:.3?}", naive_time);
    println!("Deque O(n):    {:.3?}", deque_time);
    println!(
        "Speedup:       {:.1}x",
        naive_time.as_secs_f64() / deque_time.as_secs_f64()
    );
}