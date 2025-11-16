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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:26"]
pub mod base_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "64:9"]
    pub union x264_union16_t {
        pub i: uint16_t,
        pub b: [uint8_t; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:9"]
    pub union x264_union32_t {
        pub i: uint32_t,
        pub w: [uint16_t; 2],
        pub b: [uint8_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "66:9"]
    pub union x264_union64_t {
        pub i: uint64_t,
        pub d: [uint32_t; 2],
        pub w: [uint16_t; 4],
        pub b: [uint8_t; 8],
    }
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:26"]
pub mod osdep_h {
    #[c2rust::src_loc = "452:9"]
    pub const WORD_SIZE: uint64_t = ::core::mem::size_of::<*mut ::core::ffi::c_void>() as uint64_t;
    use super::stdint_uintn_h::uint64_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/rectangle.h:26"]
pub mod rectangle_h {
    #[inline(always)]
    #[c2rust::src_loc = "28:1"]
    pub unsafe extern "C" fn x264_macroblock_cache_rect(
        mut dst: *mut ::core::ffi::c_void,
        mut w: ::core::ffi::c_int,
        mut h: ::core::ffi::c_int,
        mut s: ::core::ffi::c_int,
        mut v: uint32_t,
    ) {
        let mut d: *mut uint8_t = dst as *mut uint8_t;
        let mut v2: uint16_t = (if s >= 2 as ::core::ffi::c_int {
            v
        } else {
            v.wrapping_mul(0x101 as uint32_t)
        }) as uint16_t;
        let mut v4: uint32_t = if s >= 4 as ::core::ffi::c_int {
            v
        } else if s >= 2 as ::core::ffi::c_int {
            v.wrapping_mul(0x10001 as uint32_t)
        } else {
            v.wrapping_mul(0x1010101 as uint32_t)
        };
        let mut v8: uint64_t =
            (v4 as uint64_t).wrapping_add((v4 as uint64_t) << 32 as ::core::ffi::c_int);
        s *= 8 as ::core::ffi::c_int;
        if w == 2 as ::core::ffi::c_int {
            (*(d.offset((s * 0 as ::core::ffi::c_int) as isize) as *mut x264_union16_t)).i = v2;
            if h == 1 as ::core::ffi::c_int {
                return;
            }
            (*(d.offset((s * 1 as ::core::ffi::c_int) as isize) as *mut x264_union16_t)).i = v2;
            if h == 2 as ::core::ffi::c_int {
                return;
            }
            (*(d.offset((s * 2 as ::core::ffi::c_int) as isize) as *mut x264_union16_t)).i = v2;
            (*(d.offset((s * 3 as ::core::ffi::c_int) as isize) as *mut x264_union16_t)).i = v2;
        } else if w == 4 as ::core::ffi::c_int {
            (*(d.offset((s * 0 as ::core::ffi::c_int) as isize) as *mut x264_union32_t)).i = v4;
            if h == 1 as ::core::ffi::c_int {
                return;
            }
            (*(d.offset((s * 1 as ::core::ffi::c_int) as isize) as *mut x264_union32_t)).i = v4;
            if h == 2 as ::core::ffi::c_int {
                return;
            }
            (*(d.offset((s * 2 as ::core::ffi::c_int) as isize) as *mut x264_union32_t)).i = v4;
            (*(d.offset((s * 3 as ::core::ffi::c_int) as isize) as *mut x264_union32_t)).i = v4;
        } else if w == 8 as ::core::ffi::c_int {
            if WORD_SIZE == 8 as uint64_t {
                (*(d.offset((s * 0 as ::core::ffi::c_int) as isize) as *mut x264_union64_t)).i = v8;
                if h == 1 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 1 as ::core::ffi::c_int) as isize) as *mut x264_union64_t)).i = v8;
                if h == 2 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 2 as ::core::ffi::c_int) as isize) as *mut x264_union64_t)).i = v8;
                (*(d.offset((s * 3 as ::core::ffi::c_int) as isize) as *mut x264_union64_t)).i = v8;
            } else {
                (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                if h == 1 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                if h == 2 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
            }
        } else if w == 16 as ::core::ffi::c_int {
            if h != 1 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"h != 1\0" as *const u8 as *const ::core::ffi::c_char,
                    b"common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                    82 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
            'c_27249: {
                if h != 1 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"h != 1\0" as *const u8 as *const ::core::ffi::c_char,
                        b"common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                        82 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
                    );
                }
            };
            if WORD_SIZE == 8 as uint64_t {
                loop {
                    (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut x264_union64_t))
                        .i = v8;
                    (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        as *mut x264_union64_t))
                        .i = v8;
                    (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut x264_union64_t))
                        .i = v8;
                    (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        as *mut x264_union64_t))
                        .i = v8;
                    h -= 2 as ::core::ffi::c_int;
                    d = d.offset((s * 2 as ::core::ffi::c_int) as isize);
                    if !(h != 0) {
                        break;
                    }
                }
            } else {
                loop {
                    (*(d.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union32_t)).i = v4;
                    (*(d.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union32_t)).i = v4;
                    (*(d.offset(8 as ::core::ffi::c_int as isize) as *mut x264_union32_t)).i = v4;
                    (*(d.offset(12 as ::core::ffi::c_int as isize) as *mut x264_union32_t)).i = v4;
                    d = d.offset(s as isize);
                    h -= 1;
                    if !(h != 0) {
                        break;
                    }
                }
            }
        } else {
            __assert_fail(
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
                b"common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                108 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
            'c_27015: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                    108 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
        };
    }
    use super::assert_h::{__assert_fail, __ASSERT_FUNCTION};
    use super::base_h::{x264_union16_t, x264_union32_t, x264_union64_t};
    use super::osdep_h::WORD_SIZE;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:26"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/assert.h:26"]
pub mod assert_h {
    #[c2rust::src_loc = "137:12"]
    pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 65] = unsafe {
        ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
            *b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
        )
    };
    extern "C" {
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub use self::__stddef_null_h::NULL;
pub use self::assert_h::{__assert_fail, __ASSERT_FUNCTION};
pub use self::base_h::{x264_union16_t, x264_union32_t, x264_union64_t};
pub use self::osdep_h::WORD_SIZE;
pub use self::rectangle_h::x264_macroblock_cache_rect;
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
pub use self::types_h::{__uint16_t, __uint32_t, __uint64_t, __uint8_t};
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_4_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        val,
    );
}
#[no_mangle]
#[c2rust::src_loc = "56:1"]
pub static mut x264_10_cache_mv_func_table: [Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
>; 10] = unsafe {
    [
        Some(
            macroblock_cache_mv_1_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mv_2_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mv_1_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mv_2_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mv_4_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mv_2_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mv_4_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
    ]
};
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_4_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_2_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_1_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_2_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_1_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "56:1"]
unsafe extern "C" fn macroblock_cache_mv_2_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_1_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_4_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        val,
    );
}
#[no_mangle]
#[c2rust::src_loc = "57:1"]
pub static mut x264_10_cache_mvd_func_table: [Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
>; 10] = unsafe {
    [
        Some(
            macroblock_cache_mvd_1_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mvd_2_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mvd_1_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mvd_2_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mvd_4_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mvd_2_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mvd_4_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
    ]
};
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_2_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_4_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_2_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_2_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn macroblock_cache_mvd_1_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_4_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        4 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        val,
    );
}
#[no_mangle]
#[c2rust::src_loc = "58:1"]
pub static mut x264_10_cache_ref_func_table: [Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
>; 10] = unsafe {
    [
        Some(
            macroblock_cache_ref_1_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        Some(
            macroblock_cache_ref_2_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        Some(
            macroblock_cache_ref_1_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        Some(
            macroblock_cache_ref_2_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_ref_4_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_ref_2_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_ref_4_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> (),
        ),
    ]
};
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_4_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        4 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_2_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_2_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_2_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_1_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        val,
    );
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn macroblock_cache_ref_1_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: uint32_t,
) {
    x264_macroblock_cache_rect(
        target,
        1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        val,
    );
}
