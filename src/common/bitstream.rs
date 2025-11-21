use core::ffi::{c_int, c_long, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::bitstream_h::x264_bitstream_function_t;
use crate::common_h::{x264_t, NALU_OVERHEAD};
use crate::stdint_uintn_h::{uint32_t, uint8_t};
use crate::string_h::memset;
use crate::x264_h::x264_nal_t;
#[c2rust::src_loc = "29:1"]
unsafe extern "C" fn nal_escape_c(
    mut dst: *mut uint8_t,
    mut src: *mut uint8_t,
    mut end: *mut uint8_t,
) -> *mut uint8_t {
    if src < end {
        let fresh0 = src;
        src = src.offset(1);
        let fresh1 = dst;
        dst = dst.offset(1);
        *fresh1 = *fresh0;
    }
    if src < end {
        let fresh2 = src;
        src = src.offset(1);
        let fresh3 = dst;
        dst = dst.offset(1);
        *fresh3 = *fresh2;
    }
    while src < end {
        if *src.offset(0) as c_int <= 0x3 as c_int
            && *dst.offset(-(2 as c_int) as isize) == 0
            && *dst.offset(-1 as isize) == 0
        {
            let fresh4 = dst;
            dst = dst.offset(1);
            *fresh4 = 0x3 as uint8_t;
        }
        let fresh5 = src;
        src = src.offset(1);
        let fresh6 = dst;
        dst = dst.offset(1);
        *fresh6 = *fresh5;
    }
    return dst;
}
#[no_mangle]
#[c2rust::src_loc = "55:1"]
unsafe extern "C" fn x264_10_nal_encode(
    mut h: *mut x264_t,
    mut dst: *mut uint8_t,
    mut nal: *mut x264_nal_t,
) {
    let mut src: *mut uint8_t = (*nal).p_payload;
    let mut end: *mut uint8_t = (*nal).p_payload.offset((*nal).i_payload as isize);
    let mut orig_dst: *mut uint8_t = dst;
    if (*h).param.b_annexb != 0 {
        if (*nal).b_long_startcode != 0 {
            let fresh7 = dst;
            dst = dst.offset(1);
            *fresh7 = 0 as uint8_t;
        }
        let fresh8 = dst;
        dst = dst.offset(1);
        *fresh8 = 0 as uint8_t;
        let fresh9 = dst;
        dst = dst.offset(1);
        *fresh9 = 0 as uint8_t;
        let fresh10 = dst;
        dst = dst.offset(1);
        *fresh10 = 0x1 as uint8_t;
    } else {
        dst = dst.offset(4);
    }
    let fresh11 = dst;
    dst = dst.offset(1);
    *fresh11 =
        ((0 as c_int) << 7 as c_int | (*nal).i_ref_idc << 5 as c_int | (*nal).i_type) as uint8_t;
    dst = (*h).bsf.nal_escape.expect("non-null function pointer")(dst, src, end);
    let mut size: c_int = dst.offset_from(orig_dst) as c_long as c_int;
    if (*h).param.i_avcintra_class != 0 {
        let mut padding: c_int = (*nal).i_payload + (*nal).i_padding + NALU_OVERHEAD - size;
        if padding > 0 as c_int {
            memset(dst as *mut c_void, 0 as c_int, padding as size_t);
            size += padding;
        }
        (*nal).i_padding = if padding > 0 as c_int {
            padding
        } else {
            0 as c_int
        };
    }
    if (*h).param.b_annexb == 0 {
        let mut chunk_size: c_int = size - 4 as c_int;
        *orig_dst.offset(0) = (chunk_size >> 24 as c_int) as uint8_t;
        *orig_dst.offset(1) = (chunk_size >> 16 as c_int) as uint8_t;
        *orig_dst.offset(2) = (chunk_size >> 8 as c_int) as uint8_t;
        *orig_dst.offset(3) = (chunk_size >> 0 as c_int) as uint8_t;
    }
    (*nal).i_payload = size;
    (*nal).p_payload = orig_dst;
}
#[no_mangle]
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn x264_10_bitstream_init(
    mut _cpu: uint32_t,
    mut pf: *mut x264_bitstream_function_t,
) {
    memset(
        pf as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<x264_bitstream_function_t>() as size_t,
    );
    (*pf).nal_escape = Some(
        nal_escape_c
            as unsafe extern "C" fn(*mut uint8_t, *mut uint8_t, *mut uint8_t) -> *mut uint8_t,
    )
        as Option<unsafe extern "C" fn(*mut uint8_t, *mut uint8_t, *mut uint8_t) -> *mut uint8_t>;
}
