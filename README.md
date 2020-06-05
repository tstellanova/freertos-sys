# freertos-sys

Add this crate to your rust applications Cargo.toml to ensure that
the FreeRTOS library is included in your build.

Uses rust facilities to build freertos from source, 
and provide it as a dependency crate for hybrid rust apps.
Currently this uses `make` and ARM gcc cross-compilation. 
It does not use the `cc` crate. 

This crate exports the [CMSIS RTOS2 API]() to make operating
FreeRTOS tasks using rust as simple as possible.

## Usage
In your `Cargo.toml` select a device family by using a feature:

``` toml
freertos-sys = {version="0.1.1", features=["stm32f4x"] }
```

## License

BSD-3-Clause: See LICENSE file.

## Status

Currently the library build is tuned for the options we've found most useful.
If you'd like to see a library configuration option exposed in eg
a crate feature, please open a pull request or issue.

- [x] Supports stm32f3, f4, f7, h7
- [x] Supports release library build
- [ ] Supports debug library build
- [ ] Example application
- [ ] CI build and test

