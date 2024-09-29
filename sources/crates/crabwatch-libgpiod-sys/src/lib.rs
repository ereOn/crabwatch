use std::ffi::CStr;

#[link(name = "gpiod")]
extern "C" {
    fn gpiod_api_version() -> *const libc::c_char;
}

/// Returns the version of the libgpiod API.
pub fn api_version() -> String {
    // SAFETY: The function returns a pointer to a static null-terminated string.
    let v = unsafe { CStr::from_ptr(gpiod_api_version()) };

    v.to_string_lossy().into_owned()
}
