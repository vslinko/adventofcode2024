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

suffix="_day${day}"
msg="Running day $day"
if [ ! -z "$part" ]; then
    suffix="${suffix}_part${part}"
    msg="${msg} part $part"
fi

echo $msg

cargo test test$suffix -- --nocapture
cargo bench bench$suffix --config 'build.rustflags=["--cfg", "day19_series"]'
