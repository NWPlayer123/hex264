pub mod rectangle_h {
    #[inline(always)]

    pub unsafe extern "C" fn x264_macroblock_cache_rect(
        mut dst: *mut ::core::ffi::c_void,
        mut w: ::core::ffi::c_int,
        mut h: ::core::ffi::c_int,
        mut s: ::core::ffi::c_int,
        mut v: crate::stdlib::uint32_t,
    ) {
        unsafe {
            let mut d: *mut crate::stdlib::uint8_t = dst as *mut crate::stdlib::uint8_t;
            let mut v2: crate::stdlib::uint16_t = (if s >= 2 as ::core::ffi::c_int {
                v
            } else {
                v.wrapping_mul(0x101 as crate::stdlib::uint32_t)
            }) as crate::stdlib::uint16_t;
            let mut v4: crate::stdlib::uint32_t = if s >= 4 as ::core::ffi::c_int {
                v
            } else if s >= 2 as ::core::ffi::c_int {
                v.wrapping_mul(0x10001 as crate::stdlib::uint32_t)
            } else {
                v.wrapping_mul(0x1010101 as crate::stdlib::uint32_t)
            };
            let mut v8: crate::stdlib::uint64_t = (v4 as crate::stdlib::uint64_t)
                .wrapping_add((v4 as crate::stdlib::uint64_t) << 32 as ::core::ffi::c_int);
            s *= 8 as ::core::ffi::c_int;
            if w == 2 as ::core::ffi::c_int {
                (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                if h == 1 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                if h == 2 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
            } else if w == 4 as ::core::ffi::c_int {
                (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                if h == 1 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                if h == 2 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
            } else if w == 8 as ::core::ffi::c_int {
                if crate::osdep_h::WORD_SIZE == 8 as ::core::ffi::c_int {
                    (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    if h == 1 as ::core::ffi::c_int {
                        return;
                    }
                    (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    if h == 2 as ::core::ffi::c_int {
                        return;
                    }
                    (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                } else {
                    (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    if h == 1 as ::core::ffi::c_int {
                        return;
                    }
                    (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    if h == 2 as ::core::ffi::c_int {
                        return;
                    }
                    (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                }
            } else if w == 16 as ::core::ffi::c_int {
                '_c2rust_label: {
                    if h != 1 as ::core::ffi::c_int {
                    } else {
                        crate::stdlib::__assert_fail(
                            b"h != 1\0".as_ptr() as *const ::core::ffi::c_char,
                            b"common/rectangle.h\0".as_ptr() as *const ::core::ffi::c_char,
                            82 as ::core::ffi::c_uint,
                            crate::stdlib::__ASSERT_FUNCTION.as_ptr(),
                        );
                    }
                };
                if crate::osdep_h::WORD_SIZE == 8 as ::core::ffi::c_int {
                    loop {
                        (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        h -= 2 as ::core::ffi::c_int;
                        d = d.offset((s * 2 as ::core::ffi::c_int) as isize);
                        if !(h != 0) {
                            break;
                        }
                    }
                } else {
                    loop {
                        (*(d.offset(0 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = v4;
                        (*(d.offset(4 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = v4;
                        (*(d.offset(8 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = v4;
                        (*(d.offset(12 as ::core::ffi::c_int as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i = v4;
                        d = d.offset(s as isize);
                        h -= 1;
                        if !(h != 0) {
                            break;
                        }
                    }
                }
            } else {
                '_c2rust_label_0: {
                    crate::stdlib::__assert_fail(
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"common/rectangle.h\0".as_ptr() as *const ::core::ffi::c_char,
                        118 as ::core::ffi::c_uint,
                        crate::stdlib::__ASSERT_FUNCTION.as_ptr(),
                    );
                };
            };
        }
    }

}


use crate::src::common::rectangle::rectangle_h::x264_macroblock_cache_rect;

#[no_mangle]

pub static mut x264_8_cache_mv_func_table: [Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
>; 10] = 
    [
        Some(
            macroblock_cache_mv_1_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mv_2_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mv_1_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mv_2_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mv_4_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mv_2_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mv_4_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
    ]
;

unsafe extern "C" fn macroblock_cache_mv_1_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mv_2_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mv_1_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mv_2_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mv_4_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mv_2_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mv_4_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mvd_2_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            val,
        );
    }
}
#[no_mangle]

pub static mut x264_8_cache_mvd_func_table: [Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
>; 10] = 
    [
        Some(
            macroblock_cache_mvd_1_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mvd_2_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mvd_1_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        Some(
            macroblock_cache_mvd_2_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mvd_4_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mvd_2_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_mvd_4_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
    ]
;

unsafe extern "C" fn macroblock_cache_mvd_1_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mvd_2_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mvd_1_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mvd_2_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            2 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mvd_4_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_mvd_4_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            val,
        );
    }
}
#[no_mangle]

pub static mut x264_8_cache_ref_func_table: [Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
>; 10] = 
    [
        Some(
            macroblock_cache_ref_1_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        Some(
            macroblock_cache_ref_2_1
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        Some(
            macroblock_cache_ref_1_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        Some(
            macroblock_cache_ref_2_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_ref_4_2
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_ref_2_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
        None,
        Some(
            macroblock_cache_ref_4_4
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
        ),
    ]
;

unsafe extern "C" fn macroblock_cache_ref_1_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_ref_2_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_ref_1_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            1 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_ref_2_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_ref_4_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            4 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_ref_2_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            val,
        );
    }
}

unsafe extern "C" fn macroblock_cache_ref_4_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(
            target,
            4 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            val,
        );
    }
}
