[![Build Status](https://travis-ci.com/jmaibaum/ltc-sys.svg?branch=master)](https://travis-ci.com/jmaibaum/ltc-sys)

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
