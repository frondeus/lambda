#!/bin/bash

# mkdir -p ~/.config/helix/runtime/queries/lambda
cp ./cst/queries/* ~/.config/helix/runtime/queries/lambda/
cargo build
hx -g fetch
hx -g build
