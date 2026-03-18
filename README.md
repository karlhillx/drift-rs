# 🛰️ Drift-RS: High-Performance Aerospace Telemetry Engine

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Drift-RS** is a professional-grade telemetry ingestion sink designed for the demanding requirements of modern aerospace and satellite operations. Built with Rust for safety and performance, it provides a robust architecture for high-throughput data processing and validation.

## 🚀 Key Features

- **Blazing Fast Ingestion:** Built on `axum` and `tokio` for non-blocking, asynchronous telemetry ingestion.
- **Mission-Critical Validation:** Built-in validation layer for bounds checking of satellite readings (voltage, current, etc.).
- **High-Performance Architecture:** Structured with modular logic, shared state for persistence, and thread-safe operations.
- **Professional Observability:** Fully integrated with `tracing` for granular logging and instrumentation.
- **Robust Error Handling:** Custom error types using `thiserror` for meaningful mission-ops feedback.
- **CLI-First:** Flexible configuration via command-line arguments for port, log levels, and buffer capacity.

## 🛠️ Architectural Overview

- **`api/`**: Axum router and handlers for telemetry and health-check endpoints.
- **`models/`**: Strongly-typed data structures for telemetry packets and responses.
- **`telemetry/`**: Core logic including validators, error types, and shared state management.
- **`tests/`**: Comprehensive test suite covering validation logic and system behavior.

## 📦 Installation & Usage

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

### Build
```bash
cargo build --release
```

### Run
```bash
./target/release/drift-rs --port 3030 --log-level info
```

### Configuration Options
- `-p, --port <PORT>`: Port to listen on (default: 3030)
- `-l, --log-level <LEVEL>`: Log level (trace, debug, info, warn, error)
- `-c, --capacity <CAPACITY>`: In-memory buffer size for telemetry packets

## 📡 Ingesting Telemetry

Submit telemetry packets via POST to `/telemetry`:

```bash
curl -X POST http://localhost:3030/telemetry \
  -H "Content-Type: application/json" \
  -d '{
    "source_id": "SAT-X1",
    "timestamp": "2026-03-18T11:08:00Z",
    "instrument_id": "EPS-01",
    "readings": {
      "battery_voltage": 24.5,
      "solar_panel_current": 1.2
    }
  }'
```

## 🧪 Testing
Run the suite of unit and integration tests:
```bash
cargo test
```

---
*Developed by Karl Hill — Staff Aerospace Software Engineer.*
