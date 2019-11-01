# Bindings to libltc

## Bindgen commandline

The following commandline was used to create the initial bindings:

```bash
bindgen vendor/src/ltc.h -o src/bindings.rs --with-derive-defaults
```
