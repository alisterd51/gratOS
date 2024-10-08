use core::ffi::{c_char, c_int, c_size_t, c_void};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcpy(dest: *mut c_void, src: *const c_void, n: c_size_t) -> *mut c_void {
    let mut i: c_size_t = 0;

    while i < n {
        unsafe {
            *dest.cast::<c_char>().wrapping_add(i) = *src.cast::<c_char>().wrapping_add(i);
        }
        i += 1;
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memmove(
    dest: *mut c_void,
    src: *const c_void,
    n: c_size_t,
) -> *mut c_void {
    if dest.cast_const() < src {
        unsafe { memcpy(dest, src, n) }
    } else {
        let mut i: c_size_t = n;

        while 0 < i {
            i -= 1;
            unsafe {
                *dest.cast::<c_char>().wrapping_add(i) = *src.cast::<c_char>().wrapping_add(i);
            }
        }
        dest
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memset(dest: *mut c_void, c: c_int, n: c_size_t) -> *mut c_void {
    let mut i: c_size_t = 0;

    while i < n {
        unsafe {
            *dest.cast::<c_char>().wrapping_add(i) = (c & 0xFF) as c_char;
        }
        i += 1;
    }
    dest
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn memcmp(s1: *const c_void, s2: *const c_void, n: c_size_t) -> c_int {
    let mut i: c_size_t = 0;

    while i < n {
        let diff =
            unsafe { *s1.cast::<c_char>().wrapping_add(i) - *s2.cast::<c_char>().wrapping_add(i) };

        if diff != 0 {
            return c_int::from(diff);
        }
        i += 1;
    }
    0
}
