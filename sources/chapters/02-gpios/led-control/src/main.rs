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
