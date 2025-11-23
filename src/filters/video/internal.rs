use core::ffi::{c_char, c_float, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::input_h::{
    cli_pic_t, x264_cli_csp_depth_factor, x264_cli_csp_is_invalid, x264_cli_csps,
};
use crate::src::x264::x264_cli_log;
use crate::stdint_uintn_h::uint8_t;
use crate::string_h::memcpy;
use crate::x264_h::{X264_CSP_MASK, X264_LOG_ERROR};
#[no_mangle]
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn x264_cli_plane_copy(
    mut dst: *mut uint8_t,
    mut i_dst: c_int,
    mut src: *mut uint8_t,
    mut i_src: c_int,
    mut w: c_int,
    mut h: c_int,
) {
    loop {
        let fresh0 = h;
        h = h - 1;
        if !(fresh0 != 0) {
            break;
        }
        memcpy(dst as *mut c_void, src as *const c_void, w as size_t);
        dst = dst.offset(i_dst as isize);
        src = src.offset(i_src as isize);
    }
}
#[no_mangle]
#[c2rust::src_loc = "40:1"]
unsafe extern "C" fn x264_cli_pic_copy(mut out: *mut cli_pic_t, mut in_0: *mut cli_pic_t) -> c_int {
    let mut csp: c_int = (*in_0).img.csp & X264_CSP_MASK;
    if x264_cli_csp_is_invalid((*in_0).img.csp) != 0 {
        x264_cli_log(
            b"x264\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"invalid colorspace arg %d\n\0" as *const u8 as *const c_char,
            (*in_0).img.csp,
        );
        return -1;
    }
    if (*in_0).img.csp != (*out).img.csp
        || (*in_0).img.height != (*out).img.height
        || (*in_0).img.width != (*out).img.width
    {
        x264_cli_log(
            b"x264\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"incompatible frame properties\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    (*out).duration = (*in_0).duration;
    (*out).pts = (*in_0).pts;
    (*out).opaque = (*in_0).opaque;
    let mut i: c_int = 0 as c_int;
    while i < (*out).img.planes {
        let mut height: c_int = ((*in_0).img.height as c_float
            * (*x264_cli_csps.as_ptr().offset(csp as isize)).height[i as usize])
            as c_int;
        let mut width: c_int = ((*in_0).img.width as c_float
            * (*x264_cli_csps.as_ptr().offset(csp as isize)).width[i as usize])
            as c_int;
        width *= x264_cli_csp_depth_factor((*in_0).img.csp);
        x264_cli_plane_copy(
            (*out).img.plane[i as usize],
            (*out).img.stride[i as usize],
            (*in_0).img.plane[i as usize],
            (*in_0).img.stride[i as usize],
            width,
            height,
        );
        i += 1;
    }
    return 0 as c_int;
}
