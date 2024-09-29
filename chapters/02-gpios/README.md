# Chapter 2 - GPIOs

Now that we got a working Rust program running on the Raspberry Pi, it's time
to start interacting with the GPIOs.

## What are the GPIOs?

GPIO stands for General Purpose Input/Output. It's a generic term for pins on a
microcontroller that can be used for either input or output. On the Raspberry
Pi, the GPIOs are the pins that are exposed on the board and can be used to
interact with the outside world.

The Raspberry Pi has a 40-pin GPIO header that provides access to a variety of
GPIO pins. These pins can be used for a wide range of tasks, from simple
input/output to more complex tasks like SPI, I2C, or UART communication.

In this chapter, we will focus on basic input and output operations using the
GPIOs, and we will attempt to blink an LED connected to one of the GPIO pins
all the while building a small foundation for future chapters.

If you want to know more, make sure to checkout the
[Electronics](../../resources/electronics.md) resources page for some
additional information on GPIOs.

## Interfacing with GPIOs

There are several ways to interface with the GPIOs on the Raspberry Pi. The
traditional way is to use the
[`SysFs`](https://www.man7.org/linux/man-pages/man5/sysfs.5.html) interface,
which provides a filesystem-like interface to the GPIO pins. However, this
method is deprecated and is not recommended for new projects.

It has now been replaced by the [Linux GPIO Character
Device](https://www.kernel.org/doc/html/latest/driver-api/gpio/using-gpio.html).

This is the method we will use in this project, as it provides a more reliable
and capable interface for interacting with the GPIOs. We will integrate the
[`libgpio`](https://git.kernel.org/pub/scm/libs/libgpiod/libgpiod.git/about/)
library using `unsafe` Rust code.

### A note on the [https://docs.rs/gpiod/latest/gpiod/](gpiod) crate

The `gpiod` crate is a Rust wrapper around the `libgpio` library, which
provides exactly what we need to interact with the GPIOs on the Raspberry Pi.
However, the goal of this project is to learn how to interact with the GPIOs at
a lower level, so we will not be using the `gpiod` crate in this chapter. Feel
free to check it out if you want to fast-track your way to GPIO interaction in
Rust.

## Making sure things work

Before we start writing code, we need to make sure that the Raspberry Pi is
correctly set up and that the GPIO pins are working as expected.

To do so we will plug a simple LED into one of the GPIO pins and use the
provided command-line tool `gpio` to interact with it.

Raspberry Pi GPIO pins are 3.3V, so we need to make sure that the LED we use is
probably wired up with a resistor to avoid burning it. A typical red LED
requires a 330 Ohm resistor but you can use a 220 Ohm resistor if you want it
to be a bit brighter. It doesn't really matter for this check.

We need to wire the LED to the Raspberry Pi. The long leg of the LED is the
anode and should receive the 3.3 voltage. The short leg is the cathode and
should be connected to the ground. The resistor should be connected in series
with the LED, either before or after it: it really doesn't matter (elecricity
is weird like that).

Here is a simple diagram of how to wire the LED:

```
GPIO pin -> Resistor -> LED (Anode) -> LED (Cathode) -> Ground (that's another pin)
```

I used GPIO 27 (which is pin 13 on the Raspberry Pi) for this test but you can
use pretty much any GPIO pin you want, as long as its not a ground or a power
pin. Refer to the [Raspberry Pi Pinout](https://pinout.xyz) for more
information.

Once you have wired up the LED, you can use the `gpio` command-line tool.

Connect to the Raspberry Pi and install the `gpiod` tools:

```bash
$ sudo apt update
$ sudo apt install gpiod
```

Then, to turn it on:

```bash
$ gpioset gpiochip0 27=1
```

And to turn it off:

```bash
$ gpioset gpiochip0 27=0
```

If the LED turns on and off as expected, then you are good to go.

If not, make sure that the LED is correctly wired up and that the GPIO pin is
working as expected. Try also with another LED if you have one: it's possible
that the LED is dead or that you burned it by mistake.

## Installing the `libgpio` library

We will write code that links against the `libgpio` library to interact with
the GPIOs.

If we were compiling the code on the same platform as we intend to run it (in
our case, directly on the Raspberry Pi), we'd only need to do:

```bash
$ sudo apt install libgpiod-dev
```

However, we are cross-compiling the code on our development machine, so we need
to compile the `libgpio` library for the Raspberry Pi, using the same toolchain
we used to compile the Rust code in the first chapter.

First, install the necessary dependencies:

```bash
$ sudo apt install build-essential autoconf-archive
```

Then clone the `libgpio` repository (also provided to you in the
`sources/chapters/02-gpios/third-party/libgpiod` directory).

*Important:* At the time of writing, the current `libgpiod` version available
on the Raspbian distribution is `1.6.3`. Make sure to use the matching branch
or tag when cloning the repository. We will link dynamically against the
version so it's important to check that the version matches.

To compile it, run:

```bash
$ ./autogen.sh --enable-tools=false --prefix=/opt/gcc-arm-10.3-2021.07-x86_64-aarch64-none-linux-gnu/ --host=aarch64-none-linux-gnu
$ make
$ sudo "PATH=$PATH" make install
```

This will, in order, configure the build-process (by disabling the build of the
tools - which we don't need on this machine), compile the library, and install
it in the toolchain directory.

If all goes well, at this point you should have the `libgpio` library installed
for your cross-compilation toolchain.

### Warning

If you are using a Raspberry Pi 3 or 4, make sure to adapt the `--host`
argument to use the appropriate toolchain as well as the `--prefix` argument to
point to the correct directory.

## Writing the Rust code

Now that we have the `libgpio` library installed, we can start writing the Rust
code. Start a new Rust project using `cargo new`. Configure it to use the
cross-compilation toolchain, as we did in the previous chapter.

Add the following dependencies to your `Cargo.toml`:

```toml
[build-dependencies]
pkg-config = "0.3"
```

Then create a `build.rs` file at the root of your project with the following content:

```rust
fn main() {
    pkg_config::Config::new().probe("libgpiod").unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
```

Configure your `.cargo/config.toml` file to instruct `pkg-config` to
use your cross-compilation toolchain:

```toml
[env]
PKG_CONFIG_SYSROOT_DIR = "/"
PKG_CONFIG_PATH = "/opt/gcc-arm-10.3-2021.07-x86_64-aarch64-none-linux-gnu/lib/pkgconfig"
```

Finally, add the following code to your `src/main.rs` file (we will explain it
later, no worries):

```rust
use std::marker::{PhantomData, PhantomPinned};

#[repr(C)]
pub struct GpiodChip {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[link(name = "gpiod")]
extern "C" {
    fn gpiod_chip_open(path: *const libc::c_char) -> *mut GpiodChip;
    fn gpiod_chip_close(chip: *mut GpiodChip);
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    let path = args.get(1).map(|s| s.as_str()).unwrap_or("/dev/gpiochip0");

    println!("Opening GPIO chip at `{path}`...");

    let path = std::ffi::CString::new(path).expect("path should be a valid CString");

    let chip = unsafe { gpiod_chip_open(path.as_ptr()) };

    if chip.is_null() {
        eprintln!("Failed to open GPIO chip");
        std::process::exit(1);
    }

    println!("Successfully opened GPIO chip");

    unsafe {
        gpiod_chip_close(chip);
    }

    println!("Successfully closed GPIO chip");
}
```

You should now be able to run `cargo build` without any issues. If not,
investigate the error messages.

Once the binary is built, you can copy it to the Raspberry Pi and run it.

It should display:

```bash
$ ./led-control
Opening GPIO chip at `/dev/gpiochip0`...
Successfully opened GPIO chip
Successfully closed GPIO chip
```

If instead you get:

```bash
$ ./led-control
./led-control: error while loading shared libraries: libgpiod.so.3: cannot open shared object file: No such file or directory
```

It means you have linked against the wrong version of the `libgpiod` library.
You can type `ldd led-control` to see which version of the library is being
used. In that case, go back to the `libgpiod` repository and compile the proper
branch or tag and reinstall it.

### Static linking

If you really can't find the proper branch or tag of the `libgpiod` library to
use, you can also statically link against it. This will make the binary bigger
but it will also make it self-contained and should work on any Raspberry Pi.
It's slightly less "cool" but it works.

To do so, simply set `PKG_CONFIG_ALL_STATIC=1` in your `.cargo/config.toml`:

```toml
[env]
PKG_CONFIG_ALL_STATIC = "1"
```

And then clean & rebuild your project. It should now work.

But don't do it if you can avoid it.
