#!/bin/bash

for example in $(find examples -name '*.rs' -exec basename {} .rs \;); do
  echo "Running example: $example"
  cargo run --example "$example" || exit 1
done
