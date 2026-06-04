mod priority_queue;
mod ring_buffer;
mod sliding_window;
mod monotonic_deque;

use priority_queue::MinHeap;
use sliding_window::SlidingWindow;

const WINDOW_SIZE: usize = 5;
const ALERT_THRESHOLD: f64 = 30.0;

fn main() {
    let readings: Vec<f64> = vec![
        22.1, 23.4, 25.0, 24.8, 26.3, 28.1, 30.5, 32.0, 29.7, 27.4, 35.2, 36.8, 33.1, 31.0,
        28.5, 26.0, 24.5, 23.8, 22.9, 21.5,
    ];

    let mut window = SlidingWindow::new(WINDOW_SIZE);
    // Store alerts as (negative_temp_as_i64_for_ordering, message)
    // We use a wrapper to get min-heap on negative temp (most severe first)
    let mut alerts: MinHeap<(i64, String)> = MinHeap::new();

    println!(
        "Sensor Monitor - Window Size: {}, Alert Threshold: {}C",
        WINDOW_SIZE, ALERT_THRESHOLD
    );
    println!("{}", "=".repeat(60));

    for (i, &reading) in readings.iter().enumerate() {
        window.push(reading);

        if window.len() == WINDOW_SIZE {
            let avg = window.average();
            let wmax = window.maximum();
            let wmin = window.minimum();
            println!(
                "Reading {:2}: {:5.1}C | Window avg={:.1} max={:.1} min={:.1}",
                i + 1,
                reading,
                avg,
                wmax,
                wmin
            );

            if reading > ALERT_THRESHOLD {
                // Negate for min-heap: most severe (highest temp) gets lowest value
                let severity = -(reading * 10.0) as i64;
                let message = format!("HIGH TEMP: {}C at reading {}", reading, i + 1);
                alerts.push((severity, message));
            }
        } else {
            println!("Reading {:2}: {:5.1}C | (filling window...)", i + 1, reading);
        }
    }

    println!("\n{}", "=".repeat(60));
    println!("ALERTS (most severe first):");
    println!("{}", "-".repeat(40));

    let mut alert_num = 1;
    while let Some((severity, message)) = alerts.pop() {
        let temp = -(severity as f64) / 10.0;
        println!("  {}. [{:.1}C] {}", alert_num, temp, message);
        alert_num += 1;
    }

    if alert_num == 1 {
        println!("  No alerts generated.");
    }
        monotonic_deque::benchmark();
}