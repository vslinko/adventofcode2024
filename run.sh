#!/bin/bash

set -e

day="$1"
part="$2"

if [ -z "$day" ]; then
  echo "Usage: $0 <day> [part]"
  exit 1
fi

if [ ! -f "src/day${day}.rs" ]; then
  echo "Day $day not found"
  exit 1
fi

if [ ! -z "$part" ] && [ "$part" -ne 1 ] && [ "$part" -ne 2 ]; then
  echo "Part $part not found"
  exit 1
fi

msg="Running day $day"
tests="test_day${day}_part"
benches="bench_day${day}_part"
if [ ! -z "$part" ]; then
    msg="${msg} part $part"
    tests="test_day${day}_part${part}"
    benches="bench${day}_part${part}"
fi

echo $msg

cargo test $tests -- --nocapture
cargo bench $benches --config 'build.rustflags=["--cfg", "day19_series"]'
