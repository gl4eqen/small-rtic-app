#!/usr/bin/env sh

rustc +nightly --emit=obj -Z emit-stack-sizes big-functions.rs
stack-sizes big-functions.o
