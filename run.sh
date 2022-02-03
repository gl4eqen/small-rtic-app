#!/usr/bin/env sh

rustc +nightly --target thumbv7em-none-eabi --emit=obj -Z emit-stack-sizes big-functions.rs && \
stack-sizes big-functions.o
