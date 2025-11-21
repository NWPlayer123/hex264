use core::ffi::{c_char, c_double, c_int, c_uint, c_ulong, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::flv_bytestream_h::{flv_buffer, AMF_DATA_TYPE_NUMBER};
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use crate::stdio_h::{fopen, fwrite, stdout};
use crate::stdlib_h::{calloc, free, realloc};
use crate::string_h::{memcpy, strcmp, strlen};
use crate::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "31:13"]
union C2RustUnnamed_0 {
    f: c_double,
    i: uint64_t,
}
#[no_mangle]
#[c2rust::src_loc = "29:1"]
unsafe extern "C" fn flv_dbl2int(mut value: c_double) -> uint64_t {
    return C2RustUnnamed_0 { f: value }.i;
}
#[no_mangle]
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn flv_put_byte(mut c: *mut flv_buffer, mut b: uint8_t) {
    flv_append_data(c, &mut b, 1 as c_uint);
}
#[no_mangle]
#[c2rust::src_loc = "41:1"]
unsafe extern "C" fn flv_put_be32(mut c: *mut flv_buffer, mut val: uint32_t) {
    flv_put_byte(c, (val >> 24 as c_int) as uint8_t);
    flv_put_byte(c, (val >> 16 as c_int) as uint8_t);
    flv_put_byte(c, (val >> 8 as c_int) as uint8_t);
    flv_put_byte(c, val as uint8_t);
}
#[no_mangle]
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn flv_put_be64(mut c: *mut flv_buffer, mut val: uint64_t) {
    flv_put_be32(c, (val >> 32 as c_int) as uint32_t);
    flv_put_be32(c, val as uint32_t);
}
#[no_mangle]
#[c2rust::src_loc = "55:1"]
unsafe extern "C" fn flv_put_be16(mut c: *mut flv_buffer, mut val: uint16_t) {
    flv_put_byte(c, (val as c_int >> 8 as c_int) as uint8_t);
    flv_put_byte(c, val as uint8_t);
}
#[no_mangle]
#[c2rust::src_loc = "61:1"]
unsafe extern "C" fn flv_put_be24(mut c: *mut flv_buffer, mut val: uint32_t) {
    flv_put_be16(c, (val >> 8 as c_int) as uint16_t);
    flv_put_byte(c, val as uint8_t);
}
#[no_mangle]
#[c2rust::src_loc = "67:1"]
unsafe extern "C" fn flv_put_tag(mut c: *mut flv_buffer, mut tag: *const c_char) {
    while *tag != 0 {
        let fresh0 = tag;
        tag = tag.offset(1);
        flv_put_byte(c, *fresh0 as uint8_t);
    }
}
#[no_mangle]
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn flv_put_amf_string(mut c: *mut flv_buffer, mut str: *const c_char) {
    let mut len: uint16_t = strlen(str) as uint16_t;
    flv_put_be16(c, len);
    flv_append_data(c, str as *mut uint8_t, len as c_uint);
}
#[no_mangle]
#[c2rust::src_loc = "80:1"]
unsafe extern "C" fn flv_put_amf_double(mut c: *mut flv_buffer, mut d: c_double) {
    flv_put_byte(c, AMF_DATA_TYPE_NUMBER as c_int as uint8_t);
    flv_put_be64(c, flv_dbl2int(d));
}
#[no_mangle]
#[c2rust::src_loc = "88:1"]
unsafe extern "C" fn flv_create_writer(mut filename: *const c_char) -> *mut flv_buffer {
    let mut c: *mut flv_buffer =
        calloc(1 as size_t, ::core::mem::size_of::<flv_buffer>() as size_t) as *mut flv_buffer;
    if c.is_null() {
        return 0 as *mut flv_buffer;
    }
    if strcmp(filename, b"-\0" as *const u8 as *const c_char) == 0 {
        (*c).fp = stdout;
    } else {
        (*c).fp = fopen(filename, b"wb\0" as *const u8 as *const c_char) as *mut FILE;
    }
    if (*c).fp.is_null() {
        free(c as *mut c_void);
        return 0 as *mut flv_buffer;
    }
    return c;
}
#[no_mangle]
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn flv_append_data(
    mut c: *mut flv_buffer,
    mut data: *mut uint8_t,
    mut size: c_uint,
) -> c_int {
    let mut ns: c_uint = (*c).d_cur.wrapping_add(size);
    if ns > (*c).d_max {
        let mut dp: *mut c_void = 0 as *mut c_void;
        let mut dn: c_uint = 16 as c_uint;
        while ns > dn {
            dn <<= 1 as c_int;
        }
        dp = realloc((*c).data as *mut c_void, dn as size_t);
        if dp.is_null() {
            return -1;
        }
        (*c).data = dp as *mut uint8_t;
        (*c).d_max = dn;
    }
    memcpy(
        (*c).data.offset((*c).d_cur as isize) as *mut c_void,
        data as *const c_void,
        size as size_t,
    );
    (*c).d_cur = ns;
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "133:1"]
unsafe extern "C" fn flv_rewrite_amf_be24(
    mut c: *mut flv_buffer,
    mut length: c_uint,
    mut start: c_uint,
) {
    *(*c).data.offset(start as isize).offset(0) = (length >> 16 as c_int) as uint8_t;
    *(*c).data.offset(start as isize).offset(1) = (length >> 8 as c_int) as uint8_t;
    *(*c).data.offset(start as isize).offset(2) = (length >> 0 as c_int) as uint8_t;
}
#[no_mangle]
#[c2rust::src_loc = "140:1"]
unsafe extern "C" fn flv_flush_data(mut c: *mut flv_buffer) -> c_int {
    if (*c).d_cur == 0 {
        return 0 as c_int;
    }
    if fwrite(
        (*c).data as *const c_void,
        (*c).d_cur as size_t,
        1 as size_t,
        (*c).fp,
    ) != 1 as c_ulong
    {
        return -1;
    }
    (*c).d_total = (*c).d_total.wrapping_add((*c).d_cur as uint64_t);
    (*c).d_cur = 0 as c_uint;
    return 0 as c_int;
}
