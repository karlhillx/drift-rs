# Drift (Telemetry Sink)

A fast, lightweight, and memory-safe telemetry and simulation data sink for aerospace and satellite operations, written in Rust.

## Why Rust?

Given your work at **Jacobs/BlackLynx** on mission simulation and telemetry services, you need high throughput, low latency, and zero-cost abstractions. **Drift-rs** is a playground for exploring how Rust's safety guarantees and performance profile fit into the mission-critical aerospace landscape.

## Core Pillars

- **Safety**: Memory-safe ingestion of instrument data without a GC.
- **Speed**: Built on `tokio` and `axum` for asynchronous, non-blocking IO.
- **Interoperability**: First-class JSON support for telemetry packets, designed to sit alongside your Laravel/Python stacks.

## Getting Started

### Prerequisites

- [Rust & Cargo](https://rustup.rs/) (v1.60+)

### Running Locally

```bash
cargo run
```

### Ingesting Telemetry

```bash
curl -X POST http://127.0.0.1:3030/telemetry \
  -H "Content-Type: application/json" \
  -d '{
    "source_id": "SAT-01",
    "timestamp": "2026-03-17T17:43:00Z",
    "instrument_id": "RADAR-A",
    "readings": { "gain": 42.5, "status": "active" }
  }'
```

## Future Roadmap

- [ ] **Protobuf Support**: Transition from JSON to Protobuf for wire-efficiency.
- [ ] **Stream Persistence**: Direct integration with high-speed persistence layers (PostgreSQL/TimescaleDB).
- [ ] **Simulation Replay**: A lightweight agent to replay telemetry streams for team testing.
