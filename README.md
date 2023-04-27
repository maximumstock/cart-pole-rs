# cart-pole-rs

A Rust implementation of a CartPole RL inference service based on the
[Axum web framework](https://github.com/tokio-rs/axum).

## Requirements

In order to run this webservice and inference model, the following dependencies are required:

- Rust Toolchain (see [Rustup](https://rustup.rs/))
- `libstdc++`
- `libz`
- `libtorch`

In addition, you need a compatible PyTorch version of the mentioned `CartPole-v1` model.

This project already includes the correct one (see `./models/CartPole-v1/model_traced.pt`) but
if you want to export the latest `CartPole` model as a traced PyTorch model yourself, you need
the following Python dependencies:

- `rl_zoo3`
- `stable-baselines3`

Afterwards `make download && make export` downloads the `rl_zoo3` `CartPole` model and exports
it for PyTorch usage under `./models/CartPole-v1/model_traced.pt`.

## How To Run

Start the service on port `3000` via `cargo run [--release]`.

You can then run inferences via querying the HTTP endpoint `/inference`, like:

`curl http://localhost:3000/inference?position=0&velocity=0&angle=-1&angular_velocity=0`

and receive something like:

```HAR
HTTP/1.1 200 OK
content-type: application/json
content-length: 55
date: Sun, 12 Feb 2023 19:52:51 GMT

{"left":15.974628448486328,"right":-15.953770637512207}
```

This setup already supports request input validation and expects all query parameters to be included.

## Benchmarks

_NOTE:_ The current setup assumes a CPU backend for PyTorch.

In order to assess service performance, I employ the following benchmark setup.

1. A micro-benchmark to test local, static-input inference on a specific set of hardware

   This is useful for an initial baseline assessment of performance on a specific set of hardware.
   We can also quicky evaluate if code changes or dependency updates affect performance to keep dev cycles short.

2. An HTTP benchmark setup that tests the entire HTTP service as a black box

Needless to say, without deeper knowledge about service usage (requests per second) and service level
agreements, these benchmarks are kind of meaningless for real-world discussions and can only act as a
proxy for service performance.

Ultimately, we're bounded by the model's complexity and the performance of the [PyTorch C++ bindings](https://pytorch.org/cppdocs/).

As a next step, we could profile our binaries to verify the majority of time is spent in the inference step
and try to minimise the HTTP layer work through inlined, hand-written HTTP (de)serialization code.

Also, various compiler optimisation flags can be tested for performance gains.
I assume specifying target architectures of production machines could lead to significant performance gains
in the light of advanced instruction set features like SIMD (or a GPU altogether).

### Inference Micro-Benchmark

You can run the micro-benchmark `cartpole-inference` by executing `cargo bench --profile release` to test
repeated, local inference on static input.

A 2020 M1 MacBook yields:

```bash
cartpole-inference      time:   [43.446 µs 43.647 µs 43.868 µs]
                        change: [+0.1330% +0.5581% +0.9806%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

```

### HTTP Service Benchmark

Run the service in release mode via `cargo run --release` and use `make bench` in the project root to run [vegeta](https://github.com/tsenart/vegeta) as a HTTP load testing tool, making requests to endpoints specified in `benchmark/targets.txt`.
See `benchmark/bench.sh` for details (200 req/s, running for 30s).

A 2020 M1 MacBook yields:

```bash
$ make bench

Requests      [total, rate, throughput]         6000, 200.03, 200.03
Duration      [total, attack, wait]             29.995s, 29.995s, 580.166µs
Latencies     [min, mean, 50, 90, 95, 99, max]  172.959µs, 635.856µs, 592.215µs, 805.851µs, 925.803µs, 1.406ms, 14.236ms
Bytes In      [total, mean]                     324000, 54.00
Bytes Out     [total, mean]                     0, 0.00
Success       [ratio]                           100.00%
Status Codes  [code:count]                      200:6000
Error Set:
```

Again, without knowing any service performance expectations, sub `2ms` response times for the 99th percentile might be totally fine.
More elaborate production load tests with actual traffic would be necessary to actually validate service performance.
