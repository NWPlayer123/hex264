use core::ffi::{c_char, c_int};

use crate::__stddef_null_h::NULL;
use crate::base_h::x264_log_default;
use crate::common_h::x264_t;
#[no_mangle]
#[c2rust::src_loc = "32:1"]
unsafe extern "C" fn x264_10_log(
    mut h: *mut x264_t,
    mut i_level: c_int,
    mut psz_fmt: *const c_char,
    mut args: ...
) {
    if h.is_null() || i_level <= (*h).param.i_log_level {
        let mut arg: ::core::ffi::VaListImpl;
        arg = args.clone();
        if h.is_null() {
            x264_log_default(NULL, i_level, psz_fmt, arg.as_va_list());
        } else {
            (*h).param.pf_log.expect("non-null function pointer")(
                (*h).param.p_log_private,
                i_level,
                psz_fmt,
                arg.as_va_list(),
            );
        }
    }
}
