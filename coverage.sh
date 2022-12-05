#!/bin/sh
rm *.profraw
RUSTFLAGS=-Cinstrument-coverage LLVM_PROFILE_FILE="risky-%p-%m.profraw" cargo +nightly test --all-features
rm -rf coverage
grcov . --binary-path target/debug/deps -t html -s . --branch -o coverage/
grcov . --binary-path target/debug/deps -t lcov -s . --branch -o coverage/lcov.info
