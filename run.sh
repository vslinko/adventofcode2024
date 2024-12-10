#!/bin/bash

set -e

day=$1
part=$2

if [ -z "$day" ] || [ -z "$part" ]; then
  echo "Usage: $0 <day> <part>"
  exit 1
fi

echo "Running day $day part $part"

cargo test -p solution test_day${day}_part$part -- --nocapture
cargo bench bench_day${day}_part$part
