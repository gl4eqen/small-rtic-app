```sh
$ cargo install stack-sizes

cargo +nightly build && stack-sizes target/thumbv7em-none-eabi/debug/small-rtic-app | sort -k 2 -n
```
