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
            let mut d = dst as *mut crate::stdlib::uint8_t;
            let mut v2 = (if s >= 2i32 {
                v
            } else {
                v.wrapping_mul(0x101u32)
            }) as crate::stdlib::uint16_t;
            let mut v4 = if s >= 4i32 {
                v
            } else if s >= 2i32 {
                v.wrapping_mul(0x10001u32)
            } else {
                v.wrapping_mul(0x1010101u32)
            };
            let mut v8 = (v4 as crate::stdlib::uint64_t)
                .wrapping_add((v4 as crate::stdlib::uint64_t) << 32i32);
            s *= 8i32;
            if w == 2i32 {
                (*(d.offset((s * 0i32) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                if h == 1i32 {
                    return;
                }
                (*(d.offset((s * 1i32) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                if h == 2i32 {
                    return;
                }
                (*(d.offset((s * 2i32) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
                (*(d.offset((s * 3i32) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = v2;
            } else if w == 4i32 {
                (*(d.offset((s * 0i32) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                if h == 1i32 {
                    return;
                }
                (*(d.offset((s * 1i32) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                if h == 2i32 {
                    return;
                }
                (*(d.offset((s * 2i32) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 3i32) as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = v4;
            } else if w == 8i32 {
                if crate::osdep_h::WORD_SIZE == 8i32 {
                    (*(d.offset((s * 0i32) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    if h == 1i32 {
                        return;
                    }
                    (*(d.offset((s * 1i32) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    if h == 2i32 {
                        return;
                    }
                    (*(d.offset((s * 2i32) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                    (*(d.offset((s * 3i32) as isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = v8;
                } else {
                    (*(d.offset((s * 0i32) as isize).offset(0isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 0i32) as isize).offset(4isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    if h == 1i32 {
                        return;
                    }
                    (*(d.offset((s * 1i32) as isize).offset(0isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 1i32) as isize).offset(4isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    if h == 2i32 {
                        return;
                    }
                    (*(d.offset((s * 2i32) as isize).offset(0isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 2i32) as isize).offset(4isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 3i32) as isize).offset(0isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                    (*(d.offset((s * 3i32) as isize).offset(4isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = v4;
                }
            } else if w == 16i32 {
                '_c2rust_label: {
                    if h != 1i32 {
                    } else {
                        crate::stdlib::__assert_fail(
                            b"h != 1\0".as_ptr() as *const ::core::ffi::c_char,
                            b"common/rectangle.h\0".as_ptr() as *const ::core::ffi::c_char,
                            82u32,
                            crate::stdlib::__ASSERT_FUNCTION.as_ptr(),
                        );
                    }
                };
                if crate::osdep_h::WORD_SIZE == 8i32 {
                    loop {
                        (*(d.offset((s * 0i32) as isize).offset(0isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 0i32) as isize).offset(8isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 1i32) as isize).offset(0isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        (*(d.offset((s * 1i32) as isize).offset(8isize)
                            as *mut crate::src::common::base::x264_union64_t))
                            .i = v8;
                        h -= 2i32;
                        d = d.offset((s * 2i32) as isize);
                        if h == 0 {
                            break;
                        }
                    }
                } else {
                    loop {
                        (*(d.offset(0isize) as *mut crate::src::common::base::x264_union32_t)).i =
                            v4;
                        (*(d.offset(4isize) as *mut crate::src::common::base::x264_union32_t)).i =
                            v4;
                        (*(d.offset(8isize) as *mut crate::src::common::base::x264_union32_t)).i =
                            v4;
                        (*(d.offset(12isize) as *mut crate::src::common::base::x264_union32_t)).i =
                            v4;
                        d = d.offset(s as isize);
                        h -= 1;
                        if h == 0 {
                            break;
                        }
                    }
                }
            } else {
                '_c2rust_label_0: {
                    crate::stdlib::__assert_fail(
                        b"0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"common/rectangle.h\0".as_ptr() as *const ::core::ffi::c_char,
                        118u32,
                        crate::stdlib::__ASSERT_FUNCTION.as_ptr(),
                    );
                };
            };
        }
    }
}
use crate::src::common::rectangle::rectangle_h::x264_macroblock_cache_rect;
pub static mut x264_8_cache_mv_func_table: [Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
>; 10] = [
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
];
unsafe extern "C" fn macroblock_cache_mv_1_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 1i32 * 4i32, 1i32, 4i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mv_2_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 2i32 * 4i32, 1i32, 4i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mv_1_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 1i32 * 4i32, 2i32, 4i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mv_2_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 2i32 * 4i32, 2i32, 4i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mv_4_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 4i32 * 4i32, 2i32, 4i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mv_2_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 2i32 * 4i32, 4i32, 4i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mv_4_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 4i32 * 4i32, 4i32, 4i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mvd_2_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 2i32 * 2i32, 4i32, 2i32, val);
    }
}
pub static mut x264_8_cache_mvd_func_table: [Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
>; 10] = [
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
];
unsafe extern "C" fn macroblock_cache_mvd_1_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 1i32 * 2i32, 1i32, 2i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mvd_2_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 2i32 * 2i32, 1i32, 2i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mvd_1_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 1i32 * 2i32, 2i32, 2i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mvd_2_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 2i32 * 2i32, 2i32, 2i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mvd_4_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 4i32 * 2i32, 2i32, 2i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_mvd_4_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 4i32 * 2i32, 4i32, 2i32, val);
    }
}
pub static mut x264_8_cache_ref_func_table: [Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, crate::stdlib::uint32_t) -> (),
>; 10] = [
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
];
unsafe extern "C" fn macroblock_cache_ref_1_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 1i32 * 1i32, 1i32, 1i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_ref_2_1(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 2i32 * 1i32, 1i32, 1i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_ref_1_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 1i32 * 1i32, 2i32, 1i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_ref_2_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 2i32 * 1i32, 2i32, 1i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_ref_4_2(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 4i32 * 1i32, 2i32, 1i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_ref_2_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 2i32 * 1i32, 4i32, 1i32, val);
    }
}
unsafe extern "C" fn macroblock_cache_ref_4_4(
    mut target: *mut ::core::ffi::c_void,
    mut val: crate::stdlib::uint32_t,
) {
    unsafe {
        x264_macroblock_cache_rect(target, 4i32 * 1i32, 4i32, 1i32, val);
    }
}
