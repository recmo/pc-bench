# Polynomial Commitment Benchmark

## Running on Android

### Prerequisites

#### Cargo Cross
```shell
cargo +nightly install cross
```

#### Android Platform Tools (ADB)
Can be downloaded and installed from [here](https://developer.android.com/studio/releases/platform-tools).

### Build and run (Rust benchmarks)
```shell
cross +nightly build --target=aarch64-linux-android --release
adb push target/aarch64-linux-android/release/pc_bench /data/local/tmp
adb shell /data/local/tmp/pc_bench ALGORITHM MAX_EXPONENT
```

### Build and run (Go benchmarks)
```shell
GOOS=linux GOARCH=arm64 go build -o pc_bench_gnark
adb push pc_bench_gnark /data/local/tmp
adb shell /data/local/tmp/pc_bench_gnark
```
