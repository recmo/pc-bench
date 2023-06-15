#!/usr/bin/env bash

if [ "$#" -ne 2 ]
then
    echo "Usage (note: only call inside xcode!):"
    echo "compile-library.sh <FFI_TARGET> <buildvariant>"
    exit 1
fi

# what to pass to cargo build -p, e.g. your_lib_ffi
FFI_TARGET=$1
# buildvariant from our xcconfigs
BUILDVARIANT=$2

RELFLAG=
if [[ "$BUILDVARIANT" != "debug" ]]; then
  RELFLAG=--release
fi

set -euvx

if [[ -n "${DEVELOPER_SDK_DIR:-}" ]]; then
  # Assume we're in Xcode, which means we're probably cross-compiling.
  # In this case, we need to add an extra library search path for build scripts and proc-macros,
  # which run on the host instead of the target.
  # (macOS Big Sur does not have linkable libraries in /usr/lib/.)
  export LIBRARY_PATH="${DEVELOPER_SDK_DIR}/MacOSX.sdk/usr/lib:${LIBRARY_PATH:-}"
fi

IS_SIMULATOR=0
if [ "${LLVM_TARGET_TRIPLE_SUFFIX-}" = "-simulator" ]; then
  IS_SIMULATOR=1
fi

cd "../bindings"

for arch in $ARCHS; do
  case "$arch" in
    x86_64)
      if [ $IS_SIMULATOR -eq 0 ]; then
        echo "Building for x86_64, but not a simulator build. What's going on?" >&2
        exit 2
      fi

      # Intel iOS simulator
      export CFLAGS_x86_64_apple_ios="-target x86_64-apple-ios"
      $HOME/.cargo/bin/cargo +nightly build -p $FFI_TARGET --lib $RELFLAG --target x86_64-apple-ios
      cbindgen src/lib.rs -l c > target/x86_64-apple-ios/release/bindings.h
      ;;

    arm64)
      if [ $IS_SIMULATOR -eq 0 ]; then
        # Hardware iOS targets
        $HOME/.cargo/bin/cargo +nightly build -p $FFI_TARGET --lib $RELFLAG --target aarch64-apple-ios
        /opt/homebrew/bin/cbindgen src/lib.rs -l c > target/aarch64-apple-ios/release/bindings.h
      else
        $HOME/.cargo/bin/cargo +nightly build -p $FFI_TARGET --lib $RELFLAG --target aarch64-apple-ios-sim
        /opt/homebrew/bin/cbindgen src/lib.rs -l c > target/aarch64-apple-ios-sim/release/bindings.h
      fi
  esac
done

cp target/aarch64-apple-ios/release/bindings.h $PROJECT_DIR/Bindings/
cp target/aarch64-apple-ios/release/libbindings.a $PROJECT_DIR/Bindings/
