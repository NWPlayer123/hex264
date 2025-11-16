use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:26"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:26"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "51:8"]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: __uint64_t,
        pub _unused2: [::core::ffi::c_char; 8],
    }
    #[c2rust::src_loc = "45:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off64_t, __off_t, __uint64_t};
    extern "C" {
        #[c2rust::src_loc = "40:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "39:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "38:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:26"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:26"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t, __uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/output/flv_bytestream.h:26"]
pub mod flv_bytestream_h {
    #[c2rust::src_loc = "97:9"]
    pub type C2RustUnnamed = ::core::ffi::c_uint;
    #[c2rust::src_loc = "111:5"]
    pub const AMF_DATA_TYPE_UNSUPPORTED: C2RustUnnamed = 13;
    #[c2rust::src_loc = "110:5"]
    pub const AMF_DATA_TYPE_LONG_STRING: C2RustUnnamed = 12;
    #[c2rust::src_loc = "109:5"]
    pub const AMF_DATA_TYPE_DATE: C2RustUnnamed = 11;
    #[c2rust::src_loc = "108:5"]
    pub const AMF_DATA_TYPE_ARRAY: C2RustUnnamed = 10;
    #[c2rust::src_loc = "107:5"]
    pub const AMF_DATA_TYPE_OBJECT_END: C2RustUnnamed = 9;
    #[c2rust::src_loc = "106:5"]
    pub const AMF_DATA_TYPE_MIXEDARRAY: C2RustUnnamed = 8;
    #[c2rust::src_loc = "105:5"]
    pub const AMF_DATA_TYPE_REFERENCE: C2RustUnnamed = 7;
    #[c2rust::src_loc = "104:5"]
    pub const AMF_DATA_TYPE_UNDEFINED: C2RustUnnamed = 6;
    #[c2rust::src_loc = "103:5"]
    pub const AMF_DATA_TYPE_NULL: C2RustUnnamed = 5;
    #[c2rust::src_loc = "102:5"]
    pub const AMF_DATA_TYPE_OBJECT: C2RustUnnamed = 3;
    #[c2rust::src_loc = "101:5"]
    pub const AMF_DATA_TYPE_STRING: C2RustUnnamed = 2;
    #[c2rust::src_loc = "100:5"]
    pub const AMF_DATA_TYPE_BOOL: C2RustUnnamed = 1;
    #[c2rust::src_loc = "99:5"]
    pub const AMF_DATA_TYPE_NUMBER: C2RustUnnamed = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:16"]
    pub struct flv_buffer {
        pub data: *mut uint8_t,
        pub d_cur: ::core::ffi::c_uint,
        pub d_max: ::core::ffi::c_uint,
        pub fp: *mut FILE,
        pub d_total: uint64_t,
    }
    use super::stdint_uintn_h::{uint64_t, uint8_t};
    use super::FILE_h::FILE;
}
#[c2rust::header_src = "/usr/include/stdio.h:26"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "150:14"]
        pub static mut stdout: *mut FILE;
        #[c2rust::src_loc = "279:1"]
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        #[c2rust::src_loc = "735:1"]
        pub fn fwrite(
            __ptr: *const ::core::ffi::c_void,
            __size: size_t,
            __n: size_t,
            __s: *mut FILE,
        ) -> ::core::ffi::c_ulong;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:26"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "675:1"]
        pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "683:1"]
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/usr/include/string.h:26"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "156:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:26"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::flv_bytestream_h::{
    flv_buffer, C2RustUnnamed, AMF_DATA_TYPE_ARRAY, AMF_DATA_TYPE_BOOL, AMF_DATA_TYPE_DATE,
    AMF_DATA_TYPE_LONG_STRING, AMF_DATA_TYPE_MIXEDARRAY, AMF_DATA_TYPE_NULL, AMF_DATA_TYPE_NUMBER,
    AMF_DATA_TYPE_OBJECT, AMF_DATA_TYPE_OBJECT_END, AMF_DATA_TYPE_REFERENCE, AMF_DATA_TYPE_STRING,
    AMF_DATA_TYPE_UNDEFINED, AMF_DATA_TYPE_UNSUPPORTED,
};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use self::stdio_h::{fopen, fwrite, stdout};
use self::stdlib_h::{calloc, free, realloc};
use self::string_h::{memcpy, strcmp, strlen};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__off64_t, __off_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t};
pub use self::FILE_h::FILE;
pub use self::__stddef_null_h::NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "31:13"]
pub union C2RustUnnamed_0 {
    pub f: ::core::ffi::c_double,
    pub i: uint64_t,
}
#[no_mangle]
#[c2rust::src_loc = "29:1"]
pub unsafe extern "C" fn flv_dbl2int(mut value: ::core::ffi::c_double) -> uint64_t {
    return C2RustUnnamed_0 { f: value }.i;
}
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn flv_put_byte(mut c: *mut flv_buffer, mut b: uint8_t) {
    flv_append_data(c, &mut b, 1 as ::core::ffi::c_uint);
}
#[no_mangle]
#[c2rust::src_loc = "41:1"]
pub unsafe extern "C" fn flv_put_be32(mut c: *mut flv_buffer, mut val: uint32_t) {
    flv_put_byte(c, (val >> 24 as ::core::ffi::c_int) as uint8_t);
    flv_put_byte(c, (val >> 16 as ::core::ffi::c_int) as uint8_t);
    flv_put_byte(c, (val >> 8 as ::core::ffi::c_int) as uint8_t);
    flv_put_byte(c, val as uint8_t);
}
#[no_mangle]
#[c2rust::src_loc = "49:1"]
pub unsafe extern "C" fn flv_put_be64(mut c: *mut flv_buffer, mut val: uint64_t) {
    flv_put_be32(c, (val >> 32 as ::core::ffi::c_int) as uint32_t);
    flv_put_be32(c, val as uint32_t);
}
#[no_mangle]
#[c2rust::src_loc = "55:1"]
pub unsafe extern "C" fn flv_put_be16(mut c: *mut flv_buffer, mut val: uint16_t) {
    flv_put_byte(
        c,
        (val as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint8_t,
    );
    flv_put_byte(c, val as uint8_t);
}
#[no_mangle]
#[c2rust::src_loc = "61:1"]
pub unsafe extern "C" fn flv_put_be24(mut c: *mut flv_buffer, mut val: uint32_t) {
    flv_put_be16(c, (val >> 8 as ::core::ffi::c_int) as uint16_t);
    flv_put_byte(c, val as uint8_t);
}
#[no_mangle]
#[c2rust::src_loc = "67:1"]
pub unsafe extern "C" fn flv_put_tag(mut c: *mut flv_buffer, mut tag: *const ::core::ffi::c_char) {
    while *tag != 0 {
        let fresh0 = tag;
        tag = tag.offset(1);
        flv_put_byte(c, *fresh0 as uint8_t);
    }
}
#[no_mangle]
#[c2rust::src_loc = "73:1"]
pub unsafe extern "C" fn flv_put_amf_string(
    mut c: *mut flv_buffer,
    mut str: *const ::core::ffi::c_char,
) {
    let mut len: uint16_t = strlen(str) as uint16_t;
    flv_put_be16(c, len);
    flv_append_data(c, str as *mut uint8_t, len as ::core::ffi::c_uint);
}
#[no_mangle]
#[c2rust::src_loc = "80:1"]
pub unsafe extern "C" fn flv_put_amf_double(mut c: *mut flv_buffer, mut d: ::core::ffi::c_double) {
    flv_put_byte(c, AMF_DATA_TYPE_NUMBER as ::core::ffi::c_int as uint8_t);
    flv_put_be64(c, flv_dbl2int(d));
}
#[no_mangle]
#[c2rust::src_loc = "88:1"]
pub unsafe extern "C" fn flv_create_writer(
    mut filename: *const ::core::ffi::c_char,
) -> *mut flv_buffer {
    let mut c: *mut flv_buffer =
        calloc(1 as size_t, ::core::mem::size_of::<flv_buffer>() as size_t) as *mut flv_buffer;
    if c.is_null() {
        return 0 as *mut flv_buffer;
    }
    if strcmp(filename, b"-\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*c).fp = stdout;
    } else {
        (*c).fp = fopen(filename, b"wb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    }
    if (*c).fp.is_null() {
        free(c as *mut ::core::ffi::c_void);
        return 0 as *mut flv_buffer;
    }
    return c;
}
#[no_mangle]
#[c2rust::src_loc = "107:1"]
pub unsafe extern "C" fn flv_append_data(
    mut c: *mut flv_buffer,
    mut data: *mut uint8_t,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut ns: ::core::ffi::c_uint = (*c).d_cur.wrapping_add(size);
    if ns > (*c).d_max {
        let mut dp: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
        let mut dn: ::core::ffi::c_uint = 16 as ::core::ffi::c_uint;
        while ns > dn {
            dn <<= 1 as ::core::ffi::c_int;
        }
        dp = realloc((*c).data as *mut ::core::ffi::c_void, dn as size_t);
        if dp.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        (*c).data = dp as *mut uint8_t;
        (*c).d_max = dn;
    }
    memcpy(
        (*c).data.offset((*c).d_cur as isize) as *mut ::core::ffi::c_void,
        data as *const ::core::ffi::c_void,
        size as size_t,
    );
    (*c).d_cur = ns;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "133:1"]
pub unsafe extern "C" fn flv_rewrite_amf_be24(
    mut c: *mut flv_buffer,
    mut length: ::core::ffi::c_uint,
    mut start: ::core::ffi::c_uint,
) {
    *(*c)
        .data
        .offset(start as isize)
        .offset(0 as ::core::ffi::c_int as isize) = (length >> 16 as ::core::ffi::c_int) as uint8_t;
    *(*c)
        .data
        .offset(start as isize)
        .offset(1 as ::core::ffi::c_int as isize) = (length >> 8 as ::core::ffi::c_int) as uint8_t;
    *(*c)
        .data
        .offset(start as isize)
        .offset(2 as ::core::ffi::c_int as isize) = (length >> 0 as ::core::ffi::c_int) as uint8_t;
}
#[no_mangle]
#[c2rust::src_loc = "140:1"]
pub unsafe extern "C" fn flv_flush_data(mut c: *mut flv_buffer) -> ::core::ffi::c_int {
    if (*c).d_cur == 0 {
        return 0 as ::core::ffi::c_int;
    }
    if fwrite(
        (*c).data as *const ::core::ffi::c_void,
        (*c).d_cur as size_t,
        1 as size_t,
        (*c).fp,
    ) != 1 as ::core::ffi::c_ulong
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*c).d_total = (*c).d_total.wrapping_add((*c).d_cur as uint64_t);
    (*c).d_cur = 0 as ::core::ffi::c_uint;
    return 0 as ::core::ffi::c_int;
}
