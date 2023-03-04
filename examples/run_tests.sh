#!/bin/bash

TARGET=x86_64-pc-windows-gnu

for c in 5 10 15 20 30 40 50 60; do
	cargo run --target $TARGET --release --example benchmark -- -c $c

	for g in 1 5 10 15 20 30 40; do
		cargo run --target $TARGET --release --example simpbenchmark -- -c $c -u -g $g
	done
done
