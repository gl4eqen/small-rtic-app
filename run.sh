#!/usr/bin/env sh

rustc +nightly -g -C opt-level=0 --target thumbv7em-none-eabi --emit=obj -Z emit-stack-sizes big-functions.rs && \
arm-none-eabi-objdump -S -C -d big-functions.o && \
echo && echo && \
stack-sizes big-functions.o
