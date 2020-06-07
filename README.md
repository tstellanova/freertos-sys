# freertos-sys

Add this crate to your rust applications Cargo.toml to ensure that
the FreeRTOS library is included in your build.

This crate builds a static freertos library from source, 
and provide it as a dependency crate for hybrid rust apps.
Currently this uses `make` and ARM gcc cross-compilation. 
It does not use, for example, the `cc` crate.  
This means that you will need to have arm cross-compilation
tools installed on your system, such as 
`arm-none-eabi-gcc`

This crate exports the [CMSIS RTOS2 API](https://www.keil.com/pack/doc/CMSIS/RTOS2/html/index.htm)
 to make operating
FreeRTOS tasks using rust as simple as possible.

## Usage
In your `Cargo.toml` select a device family by using a feature:

``` toml
freertos-sys = {version="0.1.1", features=["stm32f4x"] }
```

Example application tested on stm32f401CxUx: [rolkien](https://www.github.com/tstellanova/rolkien)

## License

BSD-3-Clause: See LICENSE file.

## Status

Currently the library build is tuned for the options we've found most useful.
If you'd like to see a library configuration option exposed in eg
a crate feature, please open a pull request or issue.

- [x] Supports stm32f3, f4, f7, h7
- [x] Supports release library build
- [x] Supports debug library build
- [x] Example application
- [ ] CI build and test

