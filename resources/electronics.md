# Electronics

## Raspberry Pi

There is **plenty** of useful documentation on the Raspberry Pi so the goal
here is **not** to write-up an exhaustive list of those, but rather to mention
the particularly useful ones for our project.

The Raspberry Pi in all its forms comes with a bunch of GPIO ports that can be
used for various purposes.

One resource that I use **all the time** is the [Raspberry Pi
Pinout](https://pinout.xyz) which gives - in the most satisfying UI/UX -
detailed information about the GPIO pin layout for the Raspberry Pi. Very, very
useful when hacking things around.

<p align="center">
<img src="https://github.com/user-attachments/assets/47cbe1c3-6331-441b-94b1-5b5342d59dd2" />
</p>

In particular, don't confuse the GPIO pin number and the GPIO ports number. The
little gray numbers right next to the little circle or square for each pin is
the pin number. The GPIO port number is usually prefixed with the word "GPIO".
The latter is usually the one you need when configuring something.

## GPIO

Piloting the GPIO ports on Linux was **traditionally** done through the use of
[SysFs](https://www.man7.org/linux/man-pages/man5/sysfs.5.html), like is
explained in [this
page](https://www.ics.com/blog/gpio-programming-using-sysfs-interface).

Nowadays in modern Linux, [it is
recommended](https://docs.kernel.org/next/admin-guide/gpio/sysfs.html) to use
the [Linux GPIO Character
Device](https://www.kernel.org/doc/html/latest/driver-api/gpio/using-gpio.html)
to pilot those, at it is less error-prone and offers a more reliable - as well
as capable - interface and behaviors. There are [lots of existing higher-level
drivers](https://docs.kernel.org/next/driver-api/gpio/drivers-on-gpio.html)
that can really fasten embedded programming integration and these are worth
checking out.

To be more specific, the recommended way to interact with GPIO is to integrate
the [libgpio](https://git.kernel.org/pub/scm/libs/libgpiod/libgpiod.git/about/)
library. Since we want to do that in Rust, we will likely have to integrate
some [unsafe](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html) code.
Cool!
