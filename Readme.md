# Polynomial Commitment Benchmark

## Testing on iOS

1. Ensure `cbindgen` is installed in the system
2. Clone https://github.com/gswirski/recmo-pc-bench/tree/eth-stark
2. `git submodule init && git submodule update`
3. `cd eth-stark/vendor-eth-stark`
4. `./install_deps_arm64.sh`
5. `./compile_arm64.sh`
6. Open ios/App/App.xcodeproj in Xcode and change development team.
7. Run.