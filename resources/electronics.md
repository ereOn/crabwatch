# Electronics

## GPIO

The Raspberry Pi in all its forms comes with a bunch of GPIO ports that can be
used for various purposes.

Piloting these ports was **traditionally** done through the use of
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
