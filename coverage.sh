#!/bin/sh
rm *.profraw
RUSTFLAGS=-Cinstrument-coverage LLVM_PROFILE_FILE="risky-%p-%m.profraw" cargo +nightly test --all-features
rm -rf coverage
grcov . --binary-path target/debug/deps -t html -s . --branch --ignore-not-existing -o coverage/
grcov . --binary-path target/debug/deps -t lcov -s . --branch --ignore-not-existing -o coverage/lcov.info
