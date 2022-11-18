# cocos

Cocos strives to be a direct replacement for `coach-os`, the legacy
implementation of the middleware controlling the Coachbots at Northwestern
University.

## Compiling

Currently, `cocos` has two targets you can build:

* `cocos_simulated`
* `cocos_rpi`

`cocos_simulated` is targetted at SIL testing and can be built on `x86` by
simply running:

```
cargo build --bin cocos_simulated
```

`cocos_rpi`, on the other hand, is the firmware that is run on the Coachbots.
It is very likely that you are running an `x86_64` machine so you will need to
cross-compile for the rpi. In order to do that, you need to fetch some
dependencies first:
```bash
rustup target add armv7-unknown-linux-gnueabihf
sudo apt-get install build-essential
sudo apt-get install g++-arm-linux-gnueabihf
sudo apt-get install gdb-multiarch
```

When you have all the dependencies, running
```bash
RUSTFLAGS="-C linker=arm-linux-gnueabihf-gcc" cargo build --bin cocos_rpi \
    --target=armv7-unknown-linux-gnueabihf --release
```

should compile `cocos_rpi`. Find it in
`target/armv7-unknown-linux-gnueabihf/release`. Simply copy paste that onto the
coachbot and you should be good to go! There are no dynamic dependencies and
running the singular file will get `cocos_rpi` running!

## Design Decisions

This section outlines the design decisions that were made when writing `cocos`.

### Split Architecture

`cocos` has a split architecture with a rust firmware controlling the
coachbot hardware (ie. making IO operations) and a python API that interfaces
via ZMQ IPC with the rust firmware.

The reasoning behind this is that the python API must be sandboxed to prevent
malicious actors from comitting accidental and/or incidental harm to the
system.

### Rust

Rust was chosen as a memory-safe language to drive the firmware. The main
reasoning behind this, rather than C++, was to have a language that does not
impose OOP paradigms (enabling a closer-to-C experience), while still
preserving memory safety and safe practices.

In hindsight, C++ would have been, regardless of these advantages, a better
language due to the relative unfamiliarity of both the department and myself
with the language.

### Python

Python was chosen as the API language due to the fact that it supports great
numerical capacity -- sympy, numpy and scipy are all packages that research
heavily depends on. Because of this, python is pretty much a necessity.
Furthermore, legacy scripts would not operate unless this was chosen.
