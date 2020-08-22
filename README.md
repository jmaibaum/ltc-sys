[![Build Status](https://github.com/jmaibaum/ltc-sys/workflows/Continuous%20Integration/badge.svg)](https://github.com/jmaibaum/ltc-sys/actions?query=workflow%3A%22Continuous+Integration%22)

# [WIP] Rust FFI bindings for libltc

This is a work-in-progress Rust FFI bindings crate for
https://github.com/x42/libltc.


## Rust wrapper

There is a (also work-in-progress) Rust wrapper crate available at
https://github.com/jmaibaum/ltc.


## Bindgen commandline

The following commandline was used to create the initial bindings:

```bash
bindgen vendor/src/ltc.h -o src/bindings.rs --with-derive-defaults
```
