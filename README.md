# Federated Learning Orchestration for Financial Regulatory Authorities

Rust + Docker prototype for :
**Federated learning orchestration across heterogeneous edge and cloud nodes for financial regulatory authorities**

## What this prototype demonstrates

- Regulatory-aware participant eligibility
- Resource-aware participant selection
- Heterogeneous participant nodes
- Rust/Tokio asynchronous orchestration
- gRPC communication using tonic
- Docker Compose deployment
- Federated-learning round simulation
- Latency, communication and resource metadata collection
- Participant failure handling

 It is intentionally small so that it can be extended into the required 30-repetition experiments,
network degradation trials, and baseline comparisons.

## Architecture

Coordinator -> Scheduler -> Participants -> Aggregator

The scheduler checks:
1. regulatory eligibility
2. CPU capacity
3. memory capacity
4. network quality
5. participant availability

Only eligible participants are selected.

## Run

Requirements:
- Docker Desktop
- Docker Compose

From this directory:

```bash
docker compose up --build
```

The coordinator starts a demo FL round automatically.

View logs:

```bash
docker compose logs -f coordinator
```

Stop:

```bash
docker compose down
```

## Important limitation

This first prototype simulates local model training and aggregation. It does NOT
claim to implement production-grade secure aggregation or a real financial model.
Those should be added after the orchestration path is working.

## Next experimental extensions

- tc/netem network degradation
- 30+ independent repetitions per configuration
- p50/p95/p99 latency
- throughput
- CPU/memory measurements
- node failure experiments
- k3s deployment
- OpenStack/CRANE comparison
- OPA policy enforcement
- Prometheus/Grafana
