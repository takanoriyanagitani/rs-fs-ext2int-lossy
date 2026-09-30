#!/bin/sh

set -u

wsm="./target/wasm32-wasip1/release-wasi/fs-ext2int-lossy.wasm"

(
  echo /path/to/Dockerfile
  echo /path/to/image.jpg
  echo /path/to/image.jpeg
  echo /path/to/image.png
  echo /path/to/image.bmp
  echo /path/to/image.bmp.gz
  echo /path/to/main.go
  echo /path/to/main.rs
) |
  wasmtime run "${wsm}" |
  xxd
