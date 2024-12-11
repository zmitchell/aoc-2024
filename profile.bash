#!/usr/bin/env bash

solution="$1"

cargo build -q -r -p "$solution"

valgrind \
	--tool=callgrind \
	--trace-children=yes \
	--cache-sim=yes \
	--branch-sim=yes \
	--collect-systime=nsec \
	./target/release/"$solution"
