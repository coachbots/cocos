# cocos

Cocos strives to be a direct replacement for `coach-os`, the legacy
implementation of the middleware controlling the Coachbots at Northwestern
University.

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
