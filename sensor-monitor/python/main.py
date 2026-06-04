from sensor_monitor.sliding_window import SlidingWindow
from sensor_monitor.priority_queue import PriorityQueue

# Simulated temperature readings (Celsius)
READINGS = [
    22.1, 23.4, 25.0, 24.8, 26.3,
    28.1, 30.5, 32.0, 29.7, 27.4,
    35.2, 36.8, 33.1, 31.0, 28.5,
    26.0, 24.5, 23.8, 22.9, 21.5,
]

WINDOW_SIZE = 5
ALERT_THRESHOLD = 30.0


def severity_from_temp(temp: float) -> float:
    """Higher temp = lower priority value = more urgent."""
    return -temp  # Negate so highest temps have lowest priority value

def main():
    window = SlidingWindow(WINDOW_SIZE)
    alerts = PriorityQueue()

    print(f"Sensor Monitor - Window Size: {WINDOW_SIZE}, Alert Threshold: {ALERT_THRESHOLD}C")
    print("=" * 60)

    for i, reading in enumerate(READINGS):
        window.push(reading)

        if len(window) == WINDOW_SIZE:
            avg = window.average()
            wmax = window.maximum()
            wmin = window.minimum()
            print(
                f"Reading {i+1:2d}: {reading:5.1f}C | "
                f"Window avg={avg:.1f} max={wmax:.1f} min={wmin:.1f}"
            )

            if reading > ALERT_THRESHOLD:
                alerts.push(severity_from_temp(reading), f"HIGH TEMP: {reading}C at reading {i+1}")
        else:
            print(f"Reading {i+1:2d}: {reading:5.1f}C | (filling window...)")

    print("\n" + "=" * 60)
    print("ALERTS (most severe first):")
    print("-" * 40)

    alert_num = 1
    while len(alerts) > 0:
        priority, message = alerts.pop()
        print(f"  {alert_num}. [{-priority:.1f}C] {message}")
        alert_num += 1

    if alert_num == 1:
        print("  No alerts generated.")


if __name__ == "__main__":
    main()