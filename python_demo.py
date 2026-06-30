"""
Demo: driving the `logferry` Rust/PyO3 extension from Python.

Build first (from inside the logferry/ directory):
    pip install maturin
    maturin develop --release

Then run:
    python python_demo.py
"""
import json
import random
import time

import logferry

LEVELS = ["INFO", "INFO", "INFO", "WARN", "ERROR"]


def make_line(i: int) -> str:
    return json.dumps(
        {
            "level": random.choice(LEVELS),
            "service": "inference-server",
            # every 47th request "forgot" to set a message - on purpose,
            # so validation_errors isn't always zero
            "message": f"handled request {i}" if i % 47 else "",
            "latency_ms": round(random.uniform(5, 250), 2),
        }
    )


def main() -> None:
    lines = [make_line(i) for i in range(200_000)]
    lines.append("{not valid json")  # one malformed row, on purpose

    start = time.perf_counter()
    stats = logferry.ingest_logs(lines, num_threads=8)
    elapsed = time.perf_counter() - start

    print(f"ingested {stats.total_lines} lines in {elapsed:.3f}s")
    print(f"  parsed_ok          = {stats.parsed_ok}")
    print(f"  parse_errors       = {stats.parse_errors}")
    print(f"  validation_errors  = {stats.validation_errors}")
    print(f"  avg_latency_ms     = {stats.avg_latency_ms:.2f}")
    print(f"  by_level           = {stats.by_level}")
    print(f"  sample_errors      = {stats.sample_errors}")

    # validate_line: same validation rule, callable standalone
    assert logferry.validate_line(make_line(1)) is True
    try:
        logferry.validate_line('{"level":"INFO","service":"x","message":""}')
    except ValueError as e:
        print(f"\nvalidate_line correctly raised ValueError: {e}")


if __name__ == "__main__":
    main()
