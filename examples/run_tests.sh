#!/bin/bash

TARGET=x86_64-pc-windows-gnu
QUERY_RADII="5 10 15 20 30 40 50 60"
GRID_SIZES="1 5 10 20 40 60 80 100 150 200"
POINT_COUNTS="1000 5000 10000 20000 40000 80000"

V_MAX="10.0"
ROUNDS=1
POINTS=10000

for QUERY_RADIUS in $QUERY_RADII; do
	cargo run --target $TARGET --release \
		--example benchmark -- \
		--query-radius $QUERY_RADIUS \
		--v-max $V_MAX \
		--rounds $ROUNDS \
		--points $POINTS

	for GRID_SIZE in $GRID_SIZES; do
		cargo run --target $TARGET --release \
			--example benchmark -- \
			--shm \
			--grid-size $GRID_SIZE \
			--query-radius $QUERY_RADIUS \
			--v-max $V_MAX \
			--rounds $ROUNDS \
			--points $POINTS
	done
done

QUERY_RADIUS=20

for POINTS in $POINT_COUNTS; do
	for GRID_SIZE in $GRID_SIZES; do
		cargo run --target $TARGET --release \
			--example benchmark -- \
			--shm \
			--grid-size $GRID_SIZE \
			--query-radius $QUERY_RADIUS \
			--v-max $V_MAX \
			--rounds $ROUNDS \
			--points $POINTS
	done
done
