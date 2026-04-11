pub mod base_h {
    pub static mut x264_scan8: [crate::stdlib::uint8_t; 51] = [
        (4i32 + 1i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 1i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 2i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 2i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 1i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 1i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 2i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 2i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 3i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 3i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 4i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 4i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 3i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 3i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 4i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 4i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 6i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 6i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 7i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 7i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 6i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 6i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 7i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 7i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 8i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 8i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 9i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 9i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 8i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 8i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 9i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 9i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 11i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 11i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 12i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 12i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 11i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 11i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 12i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 12i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 13i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 13i32 * 8i32) as crate::stdlib::uint8_t,
        (4i32 + 14i32 * 8i32) as crate::stdlib::uint8_t,
        (5i32 + 14i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 13i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 13i32 * 8i32) as crate::stdlib::uint8_t,
        (6i32 + 14i32 * 8i32) as crate::stdlib::uint8_t,
        (7i32 + 14i32 * 8i32) as crate::stdlib::uint8_t,
        (0i32 + 0i32 * 8i32) as crate::stdlib::uint8_t,
        (0i32 + 5i32 * 8i32) as crate::stdlib::uint8_t,
        (0i32 + 10i32 * 8i32) as crate::stdlib::uint8_t,
    ];
    #[inline(always)]
    pub unsafe extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return if v < i_min {
            i_min
        } else if v > i_max {
            i_max
        } else {
            v
        };
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_median(
        mut a: ::core::ffi::c_int,
        mut b: ::core::ffi::c_int,
        mut c: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut t = a - b & a - b >> 31i32;
        a -= t;
        b += t;
        b -= b - c & b - c >> 31i32;
        b += a - b & a - b >> 31i32;
        return b;
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_median_mv(
        mut dst: *mut crate::stdlib::int16_t,
        mut a: *mut crate::stdlib::int16_t,
        mut b: *mut crate::stdlib::int16_t,
        mut c: *mut crate::stdlib::int16_t,
    ) {
        unsafe {
            *dst.offset(0isize) = x264_median(
                *a.offset(0isize) as ::core::ffi::c_int,
                *b.offset(0isize) as ::core::ffi::c_int,
                *c.offset(0isize) as ::core::ffi::c_int,
            ) as crate::stdlib::int16_t;
            *dst.offset(1isize) = x264_median(
                *a.offset(1isize) as ::core::ffi::c_int,
                *b.offset(1isize) as ::core::ffi::c_int,
                *c.offset(1isize) as ::core::ffi::c_int,
            ) as crate::stdlib::int16_t;
        }
    }
}
pub mod macroblock_h {
    #[inline(always)]
    pub unsafe extern "C" fn pack16to32_mask(
        mut a: ::core::ffi::c_int,
        mut b: ::core::ffi::c_int,
    ) -> crate::stdlib::uint32_t {
        return ((a & 0xffffi32) as crate::stdlib::uint32_t)
            .wrapping_add((b as crate::stdlib::uint32_t) << 16i32);
    }
}
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
                        if !(h != 0) {
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
                        118u32,
                        crate::stdlib::__ASSERT_FUNCTION.as_ptr(),
                    );
                };
            };
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_macroblock_cache_mv(
        mut h: *mut crate::src::common::common::x264_t,
        mut x: ::core::ffi::c_int,
        mut y: ::core::ffi::c_int,
        mut width: ::core::ffi::c_int,
        mut height: ::core::ffi::c_int,
        mut i_list: ::core::ffi::c_int,
        mut mv: crate::stdlib::uint32_t,
    ) {
        unsafe {
            let mut mv_cache =
                (&raw mut *(&raw mut (*h).mb.cache.mv as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(i_list as isize) as *mut [crate::stdlib::int16_t; 2])
                    .offset((crate::src::common::base::X264_SCAN8_0 + x + 8i32 * y) as isize)
                    as *mut ::core::ffi::c_void;
            if 0 == 0 || 0 == 0 {
                crate::src::common::rectangle::x264_8_cache_mv_func_table
                    [(width + (height << 1i32) - 3i32) as usize]
                    .expect("non-null function pointer")(mv_cache, mv);
            } else {
                x264_macroblock_cache_rect(mv_cache, width * 4i32, height, 4i32, mv);
            };
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_macroblock_cache_ref(
        mut h: *mut crate::src::common::common::x264_t,
        mut x: ::core::ffi::c_int,
        mut y: ::core::ffi::c_int,
        mut width: ::core::ffi::c_int,
        mut height: ::core::ffi::c_int,
        mut i_list: ::core::ffi::c_int,
        mut ref_0: crate::stdlib::int8_t,
    ) {
        unsafe {
            let mut ref_cache =
                (&raw mut *(&raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40])
                    .offset(i_list as isize) as *mut crate::stdlib::int8_t)
                    .offset((crate::src::common::base::X264_SCAN8_0 + x + 8i32 * y) as isize)
                    as *mut ::core::ffi::c_void;
            if 0 == 0 || 0 == 0 {
                crate::src::common::rectangle::x264_8_cache_ref_func_table
                    [(width + (height << 1i32) - 3i32) as usize]
                    .expect("non-null function pointer")(
                    ref_cache,
                    ref_0 as crate::stdlib::uint8_t as crate::stdlib::uint32_t,
                );
            } else {
                x264_macroblock_cache_rect(
                    ref_cache,
                    width,
                    height,
                    1i32,
                    ref_0 as crate::stdlib::uint8_t as crate::stdlib::uint32_t,
                );
            };
        }
    }
}
use crate::src::common::mvpred::base_h::x264_median_mv;
use crate::src::common::mvpred::base_h::x264_scan8;
use crate::src::common::mvpred::macroblock_h::pack16to32_mask;
use crate::src::common::mvpred::rectangle_h::x264_macroblock_cache_mv;
use crate::src::common::mvpred::rectangle_h::x264_macroblock_cache_ref;
pub unsafe extern "C" fn x264_8_mb_predict_mv(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_list: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut mvp: *mut crate::stdlib::int16_t,
) {
    unsafe {
        let mut c2rust_current_block_51: u64;
        let i8 = x264_scan8[idx as usize] as ::core::ffi::c_int;
        let i_ref = (*h).mb.cache.ref_0[i_list as usize][i8 as usize] as ::core::ffi::c_int;
        let mut i_refa =
            (*h).mb.cache.ref_0[i_list as usize][(i8 - 1i32) as usize] as ::core::ffi::c_int;
        let mut mv_a = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
            as *mut [[crate::stdlib::int16_t; 2]; 40])
            .offset(i_list as isize)
            as *mut [crate::stdlib::int16_t; 2])
            .offset((i8 - 1i32) as isize) as *mut crate::stdlib::int16_t;
        let mut i_refb =
            (*h).mb.cache.ref_0[i_list as usize][(i8 - 8i32) as usize] as ::core::ffi::c_int;
        let mut mv_b = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
            as *mut [[crate::stdlib::int16_t; 2]; 40])
            .offset(i_list as isize)
            as *mut [crate::stdlib::int16_t; 2])
            .offset((i8 - 8i32) as isize) as *mut crate::stdlib::int16_t;
        let mut i_refc = (*h).mb.cache.ref_0[i_list as usize][(i8 - 8i32 + i_width) as usize]
            as ::core::ffi::c_int;
        let mut mv_c =
            &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                as *mut [[crate::stdlib::int16_t; 2]; 40])
                .offset(i_list as isize) as *mut [crate::stdlib::int16_t; 2])
                .offset((i8 - 8i32 + i_width) as isize) as *mut crate::stdlib::int16_t;
        if idx & 3i32 >= 2i32 + (i_width & 1i32) || i_refc == -(2i32) {
            i_refc = (*h).mb.cache.ref_0[i_list as usize][(i8 - 8i32 - 1i32) as usize]
                as ::core::ffi::c_int;
            mv_c = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                as *mut [[crate::stdlib::int16_t; 2]; 40])
                .offset(i_list as isize)
                as *mut [crate::stdlib::int16_t; 2])
                .offset((i8 - 8i32 - 1i32) as isize)
                as *mut crate::stdlib::int16_t;
            if (*h).sh.mbaff
                && (*h).mb.cache.ref_0[i_list as usize]
                    [(x264_scan8[0usize] as ::core::ffi::c_int - 1i32) as usize]
                    as ::core::ffi::c_int
                    != -(2i32)
                && (*h).mb.interlaced as ::core::ffi::c_int
                    != *(*h).mb.field.offset((*h).mb.i_mb_left_xy[0usize] as isize)
                        as ::core::ffi::c_int
            {
                if idx == 2i32 {
                    mv_c = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(i_list as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(0isize) as *mut crate::stdlib::int16_t;
                    i_refc =
                        (*h).mb.cache.topright_ref[i_list as usize][0usize] as ::core::ffi::c_int;
                } else if idx == 8i32 {
                    mv_c = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(i_list as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(1isize) as *mut crate::stdlib::int16_t;
                    i_refc =
                        (*h).mb.cache.topright_ref[i_list as usize][1usize] as ::core::ffi::c_int;
                } else if idx == 10i32 {
                    mv_c = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(i_list as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(2isize) as *mut crate::stdlib::int16_t;
                    i_refc =
                        (*h).mb.cache.topright_ref[i_list as usize][2usize] as ::core::ffi::c_int;
                }
            }
        }
        if (*h).mb.i_partition == crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int {
            if idx == 0i32 {
                if i_refb == i_ref {
                    (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                        (*(mv_b as *mut crate::src::common::base::x264_union32_t)).i;
                    return;
                }
            } else if i_refa == i_ref {
                (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                    (*(mv_a as *mut crate::src::common::base::x264_union32_t)).i;
                return;
            }
        } else if (*h).mb.i_partition
            == crate::src::common::macroblock::D_8x16 as ::core::ffi::c_int
        {
            if idx == 0i32 {
                if i_refa == i_ref {
                    (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                        (*(mv_a as *mut crate::src::common::base::x264_union32_t)).i;
                    return;
                }
            } else if i_refc == i_ref {
                (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                    (*(mv_c as *mut crate::src::common::base::x264_union32_t)).i;
                return;
            }
        }
        let mut i_count = (i_refa == i_ref) as ::core::ffi::c_int
            + (i_refb == i_ref) as ::core::ffi::c_int
            + (i_refc == i_ref) as ::core::ffi::c_int;
        if i_count > 1i32 {
            c2rust_current_block_51 = 3332763991548347748;
        } else if i_count == 1i32 {
            if i_refa == i_ref {
                (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                    (*(mv_a as *mut crate::src::common::base::x264_union32_t)).i;
            } else if i_refb == i_ref {
                (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                    (*(mv_b as *mut crate::src::common::base::x264_union32_t)).i;
            } else {
                (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                    (*(mv_c as *mut crate::src::common::base::x264_union32_t)).i;
            }
            c2rust_current_block_51 = 10150597327160359210;
        } else if i_refb == -(2i32) && i_refc == -(2i32) && i_refa != -(2i32) {
            (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                (*(mv_a as *mut crate::src::common::base::x264_union32_t)).i;
            c2rust_current_block_51 = 10150597327160359210;
        } else {
            c2rust_current_block_51 = 3332763991548347748;
        }
        match c2rust_current_block_51 {
            3332763991548347748 => {
                x264_median_mv(mvp, mv_a, mv_b, mv_c);
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn x264_8_mb_predict_mv_16x16(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_list: ::core::ffi::c_int,
    mut i_ref: ::core::ffi::c_int,
    mut mvp: *mut crate::stdlib::int16_t,
) {
    unsafe {
        let mut c2rust_current_block_11: u64;
        let mut i_refa = (*h).mb.cache.ref_0[i_list as usize]
            [(crate::src::common::base::X264_SCAN8_0 - 1i32) as usize]
            as ::core::ffi::c_int;
        let mut mv_a = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
            as *mut [[crate::stdlib::int16_t; 2]; 40])
            .offset(i_list as isize)
            as *mut [crate::stdlib::int16_t; 2])
            .offset((crate::src::common::base::X264_SCAN8_0 - 1i32) as isize)
            as *mut crate::stdlib::int16_t;
        let mut i_refb = (*h).mb.cache.ref_0[i_list as usize]
            [(crate::src::common::base::X264_SCAN8_0 - 8i32) as usize]
            as ::core::ffi::c_int;
        let mut mv_b = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
            as *mut [[crate::stdlib::int16_t; 2]; 40])
            .offset(i_list as isize)
            as *mut [crate::stdlib::int16_t; 2])
            .offset((crate::src::common::base::X264_SCAN8_0 - 8i32) as isize)
            as *mut crate::stdlib::int16_t;
        let mut i_refc = (*h).mb.cache.ref_0[i_list as usize]
            [(crate::src::common::base::X264_SCAN8_0 - 8i32 + 4i32) as usize]
            as ::core::ffi::c_int;
        let mut mv_c = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
            as *mut [[crate::stdlib::int16_t; 2]; 40])
            .offset(i_list as isize)
            as *mut [crate::stdlib::int16_t; 2])
            .offset((crate::src::common::base::X264_SCAN8_0 - 8i32 + 4i32) as isize)
            as *mut crate::stdlib::int16_t;
        if i_refc == -(2i32) {
            i_refc = (*h).mb.cache.ref_0[i_list as usize]
                [(crate::src::common::base::X264_SCAN8_0 - 8i32 - 1i32) as usize]
                as ::core::ffi::c_int;
            mv_c = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                as *mut [[crate::stdlib::int16_t; 2]; 40])
                .offset(i_list as isize)
                as *mut [crate::stdlib::int16_t; 2])
                .offset((crate::src::common::base::X264_SCAN8_0 - 8i32 - 1i32) as isize)
                as *mut crate::stdlib::int16_t;
        }
        let mut i_count = (i_refa == i_ref) as ::core::ffi::c_int
            + (i_refb == i_ref) as ::core::ffi::c_int
            + (i_refc == i_ref) as ::core::ffi::c_int;
        if i_count > 1i32 {
            c2rust_current_block_11 = 15136211980786176673;
        } else if i_count == 1i32 {
            if i_refa == i_ref {
                (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                    (*(mv_a as *mut crate::src::common::base::x264_union32_t)).i;
            } else if i_refb == i_ref {
                (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                    (*(mv_b as *mut crate::src::common::base::x264_union32_t)).i;
            } else {
                (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                    (*(mv_c as *mut crate::src::common::base::x264_union32_t)).i;
            }
            c2rust_current_block_11 = 13056961889198038528;
        } else if i_refb == -(2i32) && i_refc == -(2i32) && i_refa != -(2i32) {
            (*(mvp as *mut crate::src::common::base::x264_union32_t)).i =
                (*(mv_a as *mut crate::src::common::base::x264_union32_t)).i;
            c2rust_current_block_11 = 13056961889198038528;
        } else {
            c2rust_current_block_11 = 15136211980786176673;
        }
        match c2rust_current_block_11 {
            15136211980786176673 => {
                x264_median_mv(mvp, mv_a, mv_b, mv_c);
            }
            _ => {}
        };
    }
}
pub unsafe extern "C" fn x264_8_mb_predict_mv_pskip(
    mut h: *mut crate::src::common::common::x264_t,
    mut mv: *mut crate::stdlib::int16_t,
) {
    unsafe {
        let mut i_refa = (*h).mb.cache.ref_0[0usize]
            [(crate::src::common::base::X264_SCAN8_0 - 1i32) as usize]
            as ::core::ffi::c_int;
        let mut i_refb = (*h).mb.cache.ref_0[0usize]
            [(crate::src::common::base::X264_SCAN8_0 - 8i32) as usize]
            as ::core::ffi::c_int;
        let mut mv_a = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
            as *mut [[crate::stdlib::int16_t; 2]; 40])
            .offset(0isize) as *mut [crate::stdlib::int16_t; 2])
            .offset((crate::src::common::base::X264_SCAN8_0 - 1i32) as isize)
            as *mut crate::stdlib::int16_t;
        let mut mv_b = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
            as *mut [[crate::stdlib::int16_t; 2]; 40])
            .offset(0isize) as *mut [crate::stdlib::int16_t; 2])
            .offset((crate::src::common::base::X264_SCAN8_0 - 8i32) as isize)
            as *mut crate::stdlib::int16_t;
        if i_refa == -(2i32)
            || i_refb == -(2i32)
            || i_refa as crate::stdlib::uint32_t
                | (*(mv_a as *mut crate::src::common::base::x264_union32_t)).i
                == 0
            || i_refb as crate::stdlib::uint32_t
                | (*(mv_b as *mut crate::src::common::base::x264_union32_t)).i
                == 0
        {
            (*(mv as *mut crate::src::common::base::x264_union32_t)).i = 0u32;
        } else {
            x264_8_mb_predict_mv_16x16(h, 0i32, 0i32, mv);
        };
    }
}
unsafe extern "C" fn mb_predict_mv_direct16x16_temporal(
    mut h: *mut crate::src::common::common::x264_t,
) -> bool {
    unsafe {
        let mut offset = 1i32;
        let mut yshift = 1i32;
        let mut i8 = 0i32;
        let mut mb_x = (*h).mb.i_mb_x;
        let mut mb_y = (*h).mb.i_mb_y;
        let mut mb_xy = (*h).mb.i_mb_xy;
        let mut type_col = [
            *(*(*h).fref[1usize][0usize]).mb_type.offset(mb_xy as isize) as ::core::ffi::c_int,
            *(*(*h).fref[1usize][0usize]).mb_type.offset(mb_xy as isize) as ::core::ffi::c_int,
        ];
        let mut partition_col = [
            *(*(*h).fref[1usize][0usize])
                .mb_partition
                .offset(mb_xy as isize) as ::core::ffi::c_int,
            *(*(*h).fref[1usize][0usize])
                .mb_partition
                .offset(mb_xy as isize) as ::core::ffi::c_int,
        ];
        let mut preshift = (*h).mb.interlaced;
        let mut postshift = (*h).mb.interlaced;
        (*h).mb.i_partition = partition_col[0usize];
        if (*h).param.interlaced
            && *(*(*h).fref[1usize][0usize]).field.offset(mb_xy as isize) as ::core::ffi::c_int
                != (*h).mb.interlaced as ::core::ffi::c_int
        {
            if (*h).mb.interlaced {
                mb_y = (*h).mb.i_mb_y & !(1i32);
                mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
                type_col[0usize] = *(*(*h).fref[1usize][0usize]).mb_type.offset(mb_xy as isize)
                    as ::core::ffi::c_int;
                type_col[1usize] = *(*(*h).fref[1usize][0usize])
                    .mb_type
                    .offset((mb_xy + (*h).mb.i_mb_stride) as isize)
                    as ::core::ffi::c_int;
                partition_col[0usize] = *(*(*h).fref[1usize][0usize])
                    .mb_partition
                    .offset(mb_xy as isize)
                    as ::core::ffi::c_int;
                partition_col[1usize] = *(*(*h).fref[1usize][0usize])
                    .mb_partition
                    .offset((mb_xy + (*h).mb.i_mb_stride) as isize)
                    as ::core::ffi::c_int;
                preshift = false;
                yshift = 0i32;
                if (type_col[0usize] == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                    || type_col[0usize]
                        == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                    || type_col[0usize]
                        == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                    || type_col[0usize]
                        == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int
                    || partition_col[0usize]
                        == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int)
                    && (type_col[1usize]
                        == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                        || type_col[1usize]
                            == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                        || type_col[1usize]
                            == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                        || type_col[1usize]
                            == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int
                        || partition_col[1usize]
                            == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int)
                    && partition_col[0usize]
                        != crate::src::common::macroblock::D_8x8 as ::core::ffi::c_int
                {
                    (*h).mb.i_partition =
                        crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int;
                } else {
                    (*h).mb.i_partition =
                        crate::src::common::macroblock::D_8x8 as ::core::ffi::c_int;
                }
            } else {
                let mut cur_poc = (*(*h).fdec).i_poc
                    + (*(*h).fdec).i_delta_poc[((*h).mb.interlaced as ::core::ffi::c_int
                        & (*h).mb.i_mb_y
                        & 1i32) as usize];
                let mut col_parity = (crate::stdlib::abs(
                    (*(*h).fref[1usize][0usize]).i_poc
                        + (*(*h).fref[1usize][0usize]).i_delta_poc[0usize]
                        - cur_poc,
                ) >= crate::stdlib::abs(
                    (*(*h).fref[1usize][0usize]).i_poc
                        + (*(*h).fref[1usize][0usize]).i_delta_poc[1usize]
                        - cur_poc,
                )) as ::core::ffi::c_int;
                mb_y = ((*h).mb.i_mb_y & !(1i32)) + col_parity;
                mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
                type_col[1usize] = *(*(*h).fref[1usize][0usize]).mb_type.offset(mb_xy as isize)
                    as ::core::ffi::c_int;
                type_col[0usize] = type_col[1usize];
                partition_col[1usize] = *(*(*h).fref[1usize][0usize])
                    .mb_partition
                    .offset(mb_xy as isize)
                    as ::core::ffi::c_int;
                partition_col[0usize] = partition_col[1usize];
                preshift = true;
                yshift = 2i32;
                (*h).mb.i_partition = partition_col[0usize];
            }
            offset = 0i32;
        }
        let mut i_mb_4x4 = 16i32 * (*h).mb.i_mb_stride * mb_y + 4i32 * mb_x;
        let mut i_mb_8x8 = 4i32 * (*h).mb.i_mb_stride * mb_y + 2i32 * mb_x;
        x264_macroblock_cache_ref(h, 0i32, 0i32, 4i32, 4i32, 1i32, 0i8);
        let mut max_i8 = crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
            - (*h).mb.i_partition
            + 1i32;
        let mut step = ((*h).mb.i_partition
            == crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            + 1i32;
        let mut width = 4i32
            >> (crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                - (*h).mb.i_partition
                & 1i32);
        let mut height = 4i32
            >> (crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                - (*h).mb.i_partition
                >> 1i32);
        while i8 < max_i8 {
            let mut x8 = i8 & 1i32;
            let mut y8 = i8 >> 1i32;
            let mut ypart = if (*h).sh.mbaff
                && *(*(*h).fref[1usize][0usize]).field.offset(mb_xy as isize) as ::core::ffi::c_int
                    != (*h).mb.interlaced as ::core::ffi::c_int
            {
                if (*h).mb.interlaced {
                    y8 * 6i32
                } else {
                    2i32 * ((*h).mb.i_mb_y & 1i32) + y8
                }
            } else {
                3i32 * y8
            };
            if type_col[y8 as usize] == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                || type_col[y8 as usize]
                    == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                || type_col[y8 as usize]
                    == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || type_col[y8 as usize]
                    == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int
            {
                x264_macroblock_cache_ref(h, 2i32 * x8, 2i32 * y8, width, height, 0i32, 0i8);
                x264_macroblock_cache_mv(h, 2i32 * x8, 2i32 * y8, width, height, 0i32, 0u32);
                x264_macroblock_cache_mv(h, 2i32 * x8, 2i32 * y8, width, height, 1i32, 0u32);
            } else {
                let mut i_part_8x8 = i_mb_8x8 + x8 + (ypart >> 1i32) * (*h).mb.i_b8_stride;
                let mut i_ref1_ref = *(*(*h).fref[1usize][0usize]).ref_0[0usize]
                    .offset(i_part_8x8 as isize)
                    as ::core::ffi::c_int;
                let mut i_ref = (*h).mb.map_col_to_list0
                    [((i_ref1_ref >> preshift as ::core::ffi::c_int) + 2i32) as usize]
                    as ::core::ffi::c_int
                    * ((1i32) << postshift as ::core::ffi::c_int)
                    + (offset & i_ref1_ref & (*h).mb.interlaced as ::core::ffi::c_int);
                if i_ref >= 0i32 {
                    let mut dist_scale_factor = (*(*h).mb.dist_scale_factor.offset(i_ref as isize))
                        [0usize]
                        as ::core::ffi::c_int;
                    let mut mv_col = &raw mut *(*(&raw mut (**(&raw mut *(&raw mut (*h).fref
                        as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                        .offset(1isize)
                        as *mut *mut crate::src::common::frame::x264_frame_t)
                        .offset(0isize))
                    .mv
                        as *mut *mut [crate::stdlib::int16_t; 2])
                        .offset(0isize))
                    .offset((i_mb_4x4 + 3i32 * x8 + ypart * (*h).mb.i_b4_stride) as isize)
                        as *mut crate::stdlib::int16_t;
                    let mut mv_y = (*mv_col.offset(1isize) as ::core::ffi::c_int
                        * ((1i32) << yshift)
                        / 2i32) as crate::stdlib::int16_t;
                    let mut l0x = dist_scale_factor * *mv_col.offset(0isize) as ::core::ffi::c_int
                        + 128i32
                        >> 8i32;
                    let mut l0y = dist_scale_factor * mv_y as ::core::ffi::c_int + 128i32 >> 8i32;
                    if (*h).param.i_threads > 1i32
                        && (l0y > (*h).mb.mv_max_spel[1usize]
                            || l0y - mv_y as ::core::ffi::c_int > (*h).mb.mv_max_spel[1usize])
                    {
                        return false;
                    }
                    x264_macroblock_cache_ref(
                        h,
                        2i32 * x8,
                        2i32 * y8,
                        width,
                        height,
                        0i32,
                        i_ref as crate::stdlib::int8_t,
                    );
                    x264_macroblock_cache_mv(
                        h,
                        2i32 * x8,
                        2i32 * y8,
                        width,
                        height,
                        0i32,
                        pack16to32_mask(l0x, l0y),
                    );
                    x264_macroblock_cache_mv(
                        h,
                        2i32 * x8,
                        2i32 * y8,
                        width,
                        height,
                        1i32,
                        pack16to32_mask(
                            l0x - *mv_col.offset(0isize) as ::core::ffi::c_int,
                            l0y - mv_y as ::core::ffi::c_int,
                        ),
                    );
                } else {
                    return false;
                }
            }
            i8 += step;
        }
        return true;
    }
}
#[inline(always)]
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_interlaced: ::core::ffi::c_int,
) -> bool {
    unsafe {
        let mut ref_0 = [0; 2];
        let mut mv = [[0; 2]; 2];
        let mut i_list = 0i32;
        let mut i8 = 0i32;
        while i_list < 2i32 {
            let mut i_refa = (*h).mb.cache.ref_0[i_list as usize]
                [(crate::src::common::base::X264_SCAN8_0 - 1i32) as usize]
                as ::core::ffi::c_int;
            let mut mv_a = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                as *mut [[crate::stdlib::int16_t; 2]; 40])
                .offset(i_list as isize)
                as *mut [crate::stdlib::int16_t; 2])
                .offset((crate::src::common::base::X264_SCAN8_0 - 1i32) as isize)
                as *mut crate::stdlib::int16_t;
            let mut i_refb = (*h).mb.cache.ref_0[i_list as usize]
                [(crate::src::common::base::X264_SCAN8_0 - 8i32) as usize]
                as ::core::ffi::c_int;
            let mut mv_b = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                as *mut [[crate::stdlib::int16_t; 2]; 40])
                .offset(i_list as isize)
                as *mut [crate::stdlib::int16_t; 2])
                .offset((crate::src::common::base::X264_SCAN8_0 - 8i32) as isize)
                as *mut crate::stdlib::int16_t;
            let mut i_refc = (*h).mb.cache.ref_0[i_list as usize]
                [(crate::src::common::base::X264_SCAN8_0 - 8i32 + 4i32) as usize]
                as ::core::ffi::c_int;
            let mut mv_c = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                as *mut [[crate::stdlib::int16_t; 2]; 40])
                .offset(i_list as isize)
                as *mut [crate::stdlib::int16_t; 2])
                .offset((crate::src::common::base::X264_SCAN8_0 - 8i32 + 4i32) as isize)
                as *mut crate::stdlib::int16_t;
            if i_refc == -(2i32) {
                i_refc = (*h).mb.cache.ref_0[i_list as usize]
                    [(crate::src::common::base::X264_SCAN8_0 - 8i32 - 1i32) as usize]
                    as ::core::ffi::c_int;
                mv_c = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(i_list as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset((crate::src::common::base::X264_SCAN8_0 - 8i32 - 1i32) as isize)
                    as *mut crate::stdlib::int16_t;
            }
            let mut i_ref = (if (i_refa as ::core::ffi::c_uint)
                < (if (i_refb as ::core::ffi::c_uint) < i_refc as ::core::ffi::c_uint {
                    i_refb as ::core::ffi::c_uint
                } else {
                    i_refc as ::core::ffi::c_uint
                }) {
                i_refa as ::core::ffi::c_uint
            } else if (i_refb as ::core::ffi::c_uint) < i_refc as ::core::ffi::c_uint {
                i_refb as ::core::ffi::c_uint
            } else {
                i_refc as ::core::ffi::c_uint
            }) as ::core::ffi::c_int;
            if i_ref < 0i32 {
                i_ref = -(1i32);
                (*(&raw mut *(&raw mut mv as *mut [crate::stdlib::int16_t; 2])
                    .offset(i_list as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = 0u32;
            } else {
                let mut i_count = (i_refa == i_ref) as ::core::ffi::c_int
                    + (i_refb == i_ref) as ::core::ffi::c_int
                    + (i_refc == i_ref) as ::core::ffi::c_int;
                if i_count > 1i32 {
                    x264_median_mv(
                        &raw mut *(&raw mut mv as *mut [crate::stdlib::int16_t; 2])
                            .offset(i_list as isize)
                            as *mut crate::stdlib::int16_t,
                        mv_a,
                        mv_b,
                        mv_c,
                    );
                } else if i_refa == i_ref {
                    (*(&raw mut *(&raw mut mv as *mut [crate::stdlib::int16_t; 2])
                        .offset(i_list as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(mv_a as *mut crate::src::common::base::x264_union32_t)).i;
                } else if i_refb == i_ref {
                    (*(&raw mut *(&raw mut mv as *mut [crate::stdlib::int16_t; 2])
                        .offset(i_list as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(mv_b as *mut crate::src::common::base::x264_union32_t)).i;
                } else {
                    (*(&raw mut *(&raw mut mv as *mut [crate::stdlib::int16_t; 2])
                        .offset(i_list as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(mv_c as *mut crate::src::common::base::x264_union32_t)).i;
                }
            }
            x264_macroblock_cache_ref(
                h,
                0i32,
                0i32,
                4i32,
                4i32,
                i_list,
                i_ref as crate::stdlib::int8_t,
            );
            x264_macroblock_cache_mv(
                h,
                0i32,
                0i32,
                4i32,
                4i32,
                i_list,
                (*(&raw mut *(&raw mut mv as *mut [crate::stdlib::int16_t; 2])
                    .offset(i_list as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i,
            );
            ref_0[i_list as usize] = i_ref as crate::stdlib::int8_t;
            i_list += 1;
        }
        let mut mb_x = (*h).mb.i_mb_x;
        let mut mb_y = (*h).mb.i_mb_y;
        let mut mb_xy = (*h).mb.i_mb_xy;
        let mut type_col = [
            *(*(*h).fref[1usize][0usize]).mb_type.offset(mb_xy as isize) as ::core::ffi::c_int,
            *(*(*h).fref[1usize][0usize]).mb_type.offset(mb_xy as isize) as ::core::ffi::c_int,
        ];
        let mut partition_col = [
            *(*(*h).fref[1usize][0usize])
                .mb_partition
                .offset(mb_xy as isize) as ::core::ffi::c_int,
            *(*(*h).fref[1usize][0usize])
                .mb_partition
                .offset(mb_xy as isize) as ::core::ffi::c_int,
        ];
        (*h).mb.i_partition = partition_col[0usize];
        if b_interlaced != 0
            && *(*(*h).fref[1usize][0usize]).field.offset(mb_xy as isize) as ::core::ffi::c_int
                != (*h).mb.interlaced as ::core::ffi::c_int
        {
            if (*h).mb.interlaced {
                mb_y = (*h).mb.i_mb_y & !(1i32);
                mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
                type_col[0usize] = *(*(*h).fref[1usize][0usize]).mb_type.offset(mb_xy as isize)
                    as ::core::ffi::c_int;
                type_col[1usize] = *(*(*h).fref[1usize][0usize])
                    .mb_type
                    .offset((mb_xy + (*h).mb.i_mb_stride) as isize)
                    as ::core::ffi::c_int;
                partition_col[0usize] = *(*(*h).fref[1usize][0usize])
                    .mb_partition
                    .offset(mb_xy as isize)
                    as ::core::ffi::c_int;
                partition_col[1usize] = *(*(*h).fref[1usize][0usize])
                    .mb_partition
                    .offset((mb_xy + (*h).mb.i_mb_stride) as isize)
                    as ::core::ffi::c_int;
                if (type_col[0usize] == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                    || type_col[0usize]
                        == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                    || type_col[0usize]
                        == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                    || type_col[0usize]
                        == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int
                    || partition_col[0usize]
                        == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int)
                    && (type_col[1usize]
                        == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                        || type_col[1usize]
                            == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                        || type_col[1usize]
                            == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                        || type_col[1usize]
                            == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int
                        || partition_col[1usize]
                            == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int)
                    && partition_col[0usize]
                        != crate::src::common::macroblock::D_8x8 as ::core::ffi::c_int
                {
                    (*h).mb.i_partition =
                        crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int;
                } else {
                    (*h).mb.i_partition =
                        crate::src::common::macroblock::D_8x8 as ::core::ffi::c_int;
                }
            } else {
                let mut cur_poc = (*(*h).fdec).i_poc
                    + (*(*h).fdec).i_delta_poc[((*h).mb.interlaced as ::core::ffi::c_int
                        & (*h).mb.i_mb_y
                        & 1i32) as usize];
                let mut col_parity = (crate::stdlib::abs(
                    (*(*h).fref[1usize][0usize]).i_poc
                        + (*(*h).fref[1usize][0usize]).i_delta_poc[0usize]
                        - cur_poc,
                ) >= crate::stdlib::abs(
                    (*(*h).fref[1usize][0usize]).i_poc
                        + (*(*h).fref[1usize][0usize]).i_delta_poc[1usize]
                        - cur_poc,
                )) as ::core::ffi::c_int;
                mb_y = ((*h).mb.i_mb_y & !(1i32)) + col_parity;
                mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
                type_col[1usize] = *(*(*h).fref[1usize][0usize]).mb_type.offset(mb_xy as isize)
                    as ::core::ffi::c_int;
                type_col[0usize] = type_col[1usize];
                partition_col[1usize] = *(*(*h).fref[1usize][0usize])
                    .mb_partition
                    .offset(mb_xy as isize)
                    as ::core::ffi::c_int;
                partition_col[0usize] = partition_col[1usize];
                (*h).mb.i_partition = partition_col[0usize];
            }
        }
        let mut i_mb_4x4 = if b_interlaced != 0 {
            4i32 * ((*h).mb.i_b4_stride * mb_y + mb_x)
        } else {
            (*h).mb.i_b4_xy
        };
        let mut i_mb_8x8 = if b_interlaced != 0 {
            2i32 * ((*h).mb.i_b8_stride * mb_y + mb_x)
        } else {
            (*h).mb.i_b8_xy
        };
        let mut l1ref0 = (*(&raw mut (**(&raw mut *(&raw mut (*h).fref
            as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
            .offset(1isize)
            as *mut *mut crate::src::common::frame::x264_frame_t)
            .offset(0isize))
        .ref_0 as *mut *mut crate::stdlib::int8_t)
            .offset(0isize))
        .offset(i_mb_8x8 as isize);
        let mut l1ref1 = (*(&raw mut (**(&raw mut *(&raw mut (*h).fref
            as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
            .offset(1isize)
            as *mut *mut crate::src::common::frame::x264_frame_t)
            .offset(0isize))
        .ref_0 as *mut *mut crate::stdlib::int8_t)
            .offset(1isize))
        .offset(i_mb_8x8 as isize);
        let mut l1mv = [
            (*(&raw mut (**(&raw mut *(&raw mut (*h).fref
                as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                .offset(1isize)
                as *mut *mut crate::src::common::frame::x264_frame_t)
                .offset(0isize))
            .mv as *mut *mut [crate::stdlib::int16_t; 2])
                .offset(0isize))
            .offset(i_mb_4x4 as isize),
            (*(&raw mut (**(&raw mut *(&raw mut (*h).fref
                as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                .offset(1isize)
                as *mut *mut crate::src::common::frame::x264_frame_t)
                .offset(0isize))
            .mv as *mut *mut [crate::stdlib::int16_t; 2])
                .offset(1isize))
            .offset(i_mb_4x4 as isize),
        ];
        if (*(&raw mut ref_0 as *mut crate::src::common::base::x264_union16_t)).i
            as ::core::ffi::c_int
            & 0x8080i32
            == 0x8080i32
        {
            x264_macroblock_cache_ref(h, 0i32, 0i32, 4i32, 4i32, 0i32, 0i8);
            x264_macroblock_cache_ref(h, 0i32, 0i32, 4i32, 4i32, 1i32, 0i8);
            return true;
        }
        if (*h).param.i_threads > 1i32
            && (mv[0usize][1usize] > (*h).mb.mv_max_spel[1usize]
                || mv[1usize][1usize] > (*h).mb.mv_max_spel[1usize])
        {
            return false;
        }
        if (*(&raw mut mv as *mut crate::src::common::base::x264_union64_t)).i == 0
            || b_interlaced == 0
                && (type_col[0usize] == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                    || type_col[0usize]
                        == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                    || type_col[0usize]
                        == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                    || type_col[0usize]
                        == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
            || ref_0[0usize] as ::core::ffi::c_int != 0 && ref_0[1usize] as ::core::ffi::c_int != 0
        {
            return true;
        }
        let mut max_i8 = crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
            - (*h).mb.i_partition
            + 1i32;
        let mut step = ((*h).mb.i_partition
            == crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            + 1i32;
        let mut width = 4i32
            >> (crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                - (*h).mb.i_partition
                & 1i32);
        let mut height = 4i32
            >> (crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                - (*h).mb.i_partition
                >> 1i32);
        while i8 < max_i8 {
            let x8 = i8 & 1i32;
            let y8 = i8 >> 1i32;
            let mut ypart = if b_interlaced != 0
                && *(*(*h).fref[1usize][0usize]).field.offset(mb_xy as isize) as ::core::ffi::c_int
                    != (*h).mb.interlaced as ::core::ffi::c_int
            {
                if (*h).mb.interlaced {
                    y8 * 6i32
                } else {
                    2i32 * ((*h).mb.i_mb_y & 1i32) + y8
                }
            } else {
                3i32 * y8
            };
            let mut o8 = x8 + (ypart >> 1i32) * (*h).mb.i_b8_stride;
            let mut o4 = 3i32 * x8 + ypart * (*h).mb.i_b4_stride;
            if !(b_interlaced != 0
                && (type_col[y8 as usize]
                    == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                    || type_col[y8 as usize]
                        == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                    || type_col[y8 as usize]
                        == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                    || type_col[y8 as usize]
                        == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int))
            {
                let mut c2rust_current_block_59: u64;
                let mut idx = 0;
                if *l1ref0.offset(o8 as isize) as ::core::ffi::c_int == 0i32 {
                    idx = 0i32;
                    c2rust_current_block_59 = 6528285054092551010;
                } else if (*l1ref0.offset(o8 as isize) as ::core::ffi::c_int) < 0i32
                    && *l1ref1.offset(o8 as isize) as ::core::ffi::c_int == 0i32
                {
                    idx = 1i32;
                    c2rust_current_block_59 = 6528285054092551010;
                } else {
                    c2rust_current_block_59 = 1423531122933789233;
                }
                match c2rust_current_block_59 {
                    1423531122933789233 => {}
                    _ => {
                        if crate::stdlib::abs(
                            (*l1mv[idx as usize].offset(o4 as isize))[0usize] as ::core::ffi::c_int,
                        ) <= 1i32
                            && crate::stdlib::abs(
                                (*l1mv[idx as usize].offset(o4 as isize))[1usize]
                                    as ::core::ffi::c_int,
                            ) <= 1i32
                        {
                            if ref_0[0usize] as ::core::ffi::c_int == 0i32 {
                                x264_macroblock_cache_mv(
                                    h,
                                    2i32 * x8,
                                    2i32 * y8,
                                    width,
                                    height,
                                    0i32,
                                    0u32,
                                );
                            }
                            if ref_0[1usize] as ::core::ffi::c_int == 0i32 {
                                x264_macroblock_cache_mv(
                                    h,
                                    2i32 * x8,
                                    2i32 * y8,
                                    width,
                                    height,
                                    1i32,
                                    0u32,
                                );
                            }
                        }
                    }
                }
            }
            i8 += step;
        }
        return true;
    }
}
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial_interlaced(
    mut h: *mut crate::src::common::common::x264_t,
) -> bool {
    unsafe {
        return mb_predict_mv_direct16x16_spatial(h, 1i32);
    }
}
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial_progressive(
    mut h: *mut crate::src::common::common::x264_t,
) -> bool {
    unsafe {
        return mb_predict_mv_direct16x16_spatial(h, 0i32);
    }
}
pub unsafe extern "C" fn x264_8_mb_predict_mv_direct16x16(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_changed: *mut ::core::ffi::c_int,
) -> bool {
    unsafe {
        let mut available: bool = false;
        if (*h).param.analyse.i_direct_mv_pred == crate::x264_h::X264_DIRECT_PRED_NONE {
            return false;
        } else if (*h).sh.direct_spatial_mv_pred {
            if (*h).sh.mbaff {
                available = mb_predict_mv_direct16x16_spatial_interlaced(h);
            } else {
                available = mb_predict_mv_direct16x16_spatial_progressive(h);
            }
        } else {
            available = mb_predict_mv_direct16x16_temporal(h);
        }
        if !b_changed.is_null() && available {
            let mut changed = ((*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                as *mut [[crate::stdlib::int16_t; 2]; 4])
                .offset(0isize)
                as *mut [crate::stdlib::int16_t; 2])
                .offset(0isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i
                ^ (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(0isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                            as isize,
                    ) as *mut crate::src::common::base::x264_union32_t))
                    .i) as ::core::ffi::c_int;
            changed |= ((*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                as *mut [[crate::stdlib::int16_t; 2]; 4])
                .offset(1isize)
                as *mut [crate::stdlib::int16_t; 2])
                .offset(0isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i
                ^ (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(1isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                            as isize,
                    ) as *mut crate::src::common::base::x264_union32_t))
                    .i) as ::core::ffi::c_int;
            changed |= (*h).mb.cache.direct_ref[0usize][0usize] as ::core::ffi::c_int
                ^ (*h).mb.cache.ref_0[0usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int;
            changed |= (*h).mb.cache.direct_ref[1usize][0usize] as ::core::ffi::c_int
                ^ (*h).mb.cache.ref_0[1usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int;
            if changed == 0
                && (*h).mb.i_partition
                    != crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
            {
                changed |= ((*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                    as *mut [[crate::stdlib::int16_t; 2]; 4])
                    .offset(0isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(3isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i
                    ^ (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(0isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(12isize) as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i) as ::core::ffi::c_int;
                changed |= ((*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                    as *mut [[crate::stdlib::int16_t; 2]; 4])
                    .offset(1isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(3isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i
                    ^ (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(1isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(12isize) as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i) as ::core::ffi::c_int;
                changed |= (*h).mb.cache.direct_ref[0usize][3usize] as ::core::ffi::c_int
                    ^ (*h).mb.cache.ref_0[0usize][x264_scan8[12usize] as usize]
                        as ::core::ffi::c_int;
                changed |= (*h).mb.cache.direct_ref[1usize][3usize] as ::core::ffi::c_int
                    ^ (*h).mb.cache.ref_0[1usize][x264_scan8[12usize] as usize]
                        as ::core::ffi::c_int;
            }
            if changed == 0
                && (*h).mb.i_partition
                    == crate::src::common::macroblock::D_8x8 as ::core::ffi::c_int
            {
                changed |= ((*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                    as *mut [[crate::stdlib::int16_t; 2]; 4])
                    .offset(0isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(1isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i
                    ^ (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(0isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(4isize)
                                as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i) as ::core::ffi::c_int;
                changed |= ((*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                    as *mut [[crate::stdlib::int16_t; 2]; 4])
                    .offset(1isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(1isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i
                    ^ (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(1isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(4isize)
                                as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i) as ::core::ffi::c_int;
                changed |= ((*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                    as *mut [[crate::stdlib::int16_t; 2]; 4])
                    .offset(0isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(2isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i
                    ^ (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(0isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(8isize)
                                as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i) as ::core::ffi::c_int;
                changed |= ((*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                    as *mut [[crate::stdlib::int16_t; 2]; 4])
                    .offset(1isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(2isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i
                    ^ (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(1isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(8isize)
                                as isize,
                        )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i) as ::core::ffi::c_int;
                changed |= (*h).mb.cache.direct_ref[0usize][1usize] as ::core::ffi::c_int
                    ^ (*h).mb.cache.ref_0[0usize][x264_scan8[4usize] as usize]
                        as ::core::ffi::c_int;
                changed |= (*h).mb.cache.direct_ref[1usize][1usize] as ::core::ffi::c_int
                    ^ (*h).mb.cache.ref_0[1usize][x264_scan8[4usize] as usize]
                        as ::core::ffi::c_int;
                changed |= (*h).mb.cache.direct_ref[0usize][2usize] as ::core::ffi::c_int
                    ^ (*h).mb.cache.ref_0[0usize][x264_scan8[8usize] as usize]
                        as ::core::ffi::c_int;
                changed |= (*h).mb.cache.direct_ref[1usize][2usize] as ::core::ffi::c_int
                    ^ (*h).mb.cache.ref_0[1usize][x264_scan8[8usize] as usize]
                        as ::core::ffi::c_int;
            }
            *b_changed = changed;
            if changed == 0 {
                return available;
            }
        }
        if available {
            let mut l = 0i32;
            while l < 2i32 {
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                    as *mut [[crate::stdlib::int16_t; 2]; 4])
                    .offset(l as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(0isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                            as isize,
                    )
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                    as *mut [[crate::stdlib::int16_t; 2]; 4])
                    .offset(l as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(1isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(4isize)
                            as isize,
                    )
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                    as *mut [[crate::stdlib::int16_t; 2]; 4])
                    .offset(l as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(2isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(8isize)
                            as isize,
                    )
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.direct_mv
                    as *mut [[crate::stdlib::int16_t; 2]; 4])
                    .offset(l as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(3isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(12isize)
                            as isize,
                    )
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
                (*h).mb.cache.direct_ref[l as usize][0usize] =
                    (*h).mb.cache.ref_0[l as usize][x264_scan8[0usize] as usize];
                (*h).mb.cache.direct_ref[l as usize][1usize] =
                    (*h).mb.cache.ref_0[l as usize][x264_scan8[4usize] as usize];
                (*h).mb.cache.direct_ref[l as usize][2usize] =
                    (*h).mb.cache.ref_0[l as usize][x264_scan8[8usize] as usize];
                (*h).mb.cache.direct_ref[l as usize][3usize] =
                    (*h).mb.cache.ref_0[l as usize][x264_scan8[12usize] as usize];
                (*h).mb.cache.direct_partition = (*h).mb.i_partition;
                l += 1;
            }
        }
        return available;
    }
}
pub unsafe extern "C" fn x264_8_mb_predict_mv_ref16x16(
    mut h: *mut crate::src::common::common::x264_t,
    mut i_list: ::core::ffi::c_int,
    mut i_ref: ::core::ffi::c_int,
    mut mvc: *mut [crate::stdlib::int16_t; 2],
    mut i_mvc: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut i = 0i32;
        let mut mvr = (*h).mb.mvr[i_list as usize][i_ref as usize];
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
            && (*h).mb.cache.ref_0[i_list as usize][x264_scan8[12usize] as usize]
                as ::core::ffi::c_int
                == i_ref
        {
            (*(&raw mut *mvc.offset(i as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                as *mut [[crate::stdlib::int16_t; 2]; 40])
                .offset(i_list as isize)
                as *mut [crate::stdlib::int16_t; 2])
                .offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(12isize)
                        as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                .i;
            i += 1;
        }
        if i_ref == 0i32 && (*h).frames.have_lowres {
            let mut idx = if i_list != 0 {
                (*(*h).fref[1usize][0usize]).i_frame - (*(*h).fenc).i_frame - 1i32
            } else {
                (*(*h).fenc).i_frame - (*(*h).fref[0usize][0usize]).i_frame - 1i32
            };
            if idx <= (*h).param.i_bframe {
                let mut lowres_mv = (*(*h).fenc).lowres_mvs[i_list as usize][idx as usize];
                if (*lowres_mv.offset(0isize))[0usize] as ::core::ffi::c_int != 0x7fffi32 {
                    (*(&raw mut *mvc.offset(i as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *lowres_mv.offset((*h).mb.i_mb_xy as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i
                        .wrapping_mul(2u32)
                        & 0xfffeffffu32;
                    i += 1;
                }
            }
        }
        if (*h).sh.mbaff {
            if (*h).mb.i_mb_left_xy[0usize] >= 0i32 {
                let mut shift = 1i32 + (*h).mb.interlaced as ::core::ffi::c_int
                    - *(*h).mb.field.offset((*h).mb.i_mb_left_xy[0usize] as isize)
                        as ::core::ffi::c_int;
                let mut mvp = &raw mut *(*(&raw mut *(&raw mut (*h).mb.mvr
                    as *mut [*mut [crate::stdlib::int16_t; 2]; 32])
                    .offset(i_list as isize)
                    as *mut *mut [crate::stdlib::int16_t; 2])
                    .offset((i_ref << 1i32 >> shift) as isize))
                .offset(
                    *(&raw mut (*h).mb.i_mb_left_xy as *mut ::core::ffi::c_int).offset(0isize)
                        as isize,
                ) as *mut crate::stdlib::int16_t;
                (*mvc.offset(i as isize))[0usize] = *mvp.offset(0isize);
                (*mvc.offset(i as isize))[1usize] =
                    (*mvp.offset(1isize) as ::core::ffi::c_int * 2i32 >> shift)
                        as crate::stdlib::int16_t;
                i += 1;
            }
            if (*h).mb.i_mb_top_xy >= 0i32 {
                let mut shift_0 = 1i32 + (*h).mb.interlaced as ::core::ffi::c_int
                    - *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int;
                let mut mvp_0 = &raw mut *(*(&raw mut *(&raw mut (*h).mb.mvr
                    as *mut [*mut [crate::stdlib::int16_t; 2]; 32])
                    .offset(i_list as isize)
                    as *mut *mut [crate::stdlib::int16_t; 2])
                    .offset((i_ref << 1i32 >> shift_0) as isize))
                .offset((*h).mb.i_mb_top_xy as isize)
                    as *mut crate::stdlib::int16_t;
                (*mvc.offset(i as isize))[0usize] = *mvp_0.offset(0isize);
                (*mvc.offset(i as isize))[1usize] =
                    (*mvp_0.offset(1isize) as ::core::ffi::c_int * 2i32 >> shift_0)
                        as crate::stdlib::int16_t;
                i += 1;
            }
            if (*h).mb.i_mb_topleft_xy >= 0i32 {
                let mut shift_1 = 1i32 + (*h).mb.interlaced as ::core::ffi::c_int
                    - *(*h).mb.field.offset((*h).mb.i_mb_topleft_xy as isize) as ::core::ffi::c_int;
                let mut mvp_1 = &raw mut *(*(&raw mut *(&raw mut (*h).mb.mvr
                    as *mut [*mut [crate::stdlib::int16_t; 2]; 32])
                    .offset(i_list as isize)
                    as *mut *mut [crate::stdlib::int16_t; 2])
                    .offset((i_ref << 1i32 >> shift_1) as isize))
                .offset((*h).mb.i_mb_topleft_xy as isize)
                    as *mut crate::stdlib::int16_t;
                (*mvc.offset(i as isize))[0usize] = *mvp_1.offset(0isize);
                (*mvc.offset(i as isize))[1usize] =
                    (*mvp_1.offset(1isize) as ::core::ffi::c_int * 2i32 >> shift_1)
                        as crate::stdlib::int16_t;
                i += 1;
            }
            if (*h).mb.i_mb_topright_xy >= 0i32 {
                let mut shift_2 = 1i32 + (*h).mb.interlaced as ::core::ffi::c_int
                    - *(*h).mb.field.offset((*h).mb.i_mb_topright_xy as isize)
                        as ::core::ffi::c_int;
                let mut mvp_2 = &raw mut *(*(&raw mut *(&raw mut (*h).mb.mvr
                    as *mut [*mut [crate::stdlib::int16_t; 2]; 32])
                    .offset(i_list as isize)
                    as *mut *mut [crate::stdlib::int16_t; 2])
                    .offset((i_ref << 1i32 >> shift_2) as isize))
                .offset((*h).mb.i_mb_topright_xy as isize)
                    as *mut crate::stdlib::int16_t;
                (*mvc.offset(i as isize))[0usize] = *mvp_2.offset(0isize);
                (*mvc.offset(i as isize))[1usize] =
                    (*mvp_2.offset(1isize) as ::core::ffi::c_int * 2i32 >> shift_2)
                        as crate::stdlib::int16_t;
                i += 1;
            }
        } else {
            (*(&raw mut *mvc.offset(i as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*(&raw mut *mvr.offset(
                *(&raw mut (*h).mb.i_mb_left_xy as *mut ::core::ffi::c_int).offset(0isize) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i;
            i += 1;
            (*(&raw mut *mvc.offset(i as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*(&raw mut *mvr.offset((*h).mb.i_mb_top_xy as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            i += 1;
            (*(&raw mut *mvc.offset(i as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*(&raw mut *mvr.offset((*h).mb.i_mb_topleft_xy as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            i += 1;
            (*(&raw mut *mvc.offset(i as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*(&raw mut *mvr.offset((*h).mb.i_mb_topright_xy as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            i += 1;
        }
        if (*(*h).fref[0usize][0usize]).i_ref[0usize] > 0i32 {
            let mut l0 = (*h).fref[0usize][0usize];
            let mut field = (*h).mb.i_mb_y & 1i32;
            let mut curpoc = (*(*h).fdec).i_poc + (*(*h).fdec).i_delta_poc[field as usize];
            let mut refpoc = (*(*h).fref[i_list as usize]
                [(i_ref >> (*h).sh.mbaff as ::core::ffi::c_int) as usize])
                .i_poc;
            refpoc += (*l0).i_delta_poc[(field ^ i_ref & 1i32) as usize];
            let mut mb_index = (*h).mb.i_mb_xy + 0i32 + 0i32 * (*h).mb.i_mb_stride;
            let mut scale = (curpoc - refpoc)
                * (*l0).inv_ref_poc[((*h).mb.interlaced as ::core::ffi::c_int & field) as usize]
                    as ::core::ffi::c_int;
            (*mvc.offset(i as isize))[0usize] =
                ((*(*l0).mv16x16.offset(mb_index as isize))[0usize] as ::core::ffi::c_int * scale
                    + 128i32
                    >> 8i32)
                    .saturating_truncate();
            (*mvc.offset(i as isize))[1usize] =
                ((*(*l0).mv16x16.offset(mb_index as isize))[1usize] as ::core::ffi::c_int * scale
                    + 128i32
                    >> 8i32)
                    .saturating_truncate();
            i += 1;
            if (*h).mb.i_mb_x < (*h).mb.i_mb_width - 1i32 {
                let mut mb_index_0 = (*h).mb.i_mb_xy + 1i32 + 0i32 * (*h).mb.i_mb_stride;
                let mut scale_0 = (curpoc - refpoc)
                    * (*l0).inv_ref_poc[((*h).mb.interlaced as ::core::ffi::c_int & field) as usize]
                        as ::core::ffi::c_int;
                (*mvc.offset(i as isize))[0usize] =
                    ((*(*l0).mv16x16.offset(mb_index_0 as isize))[0usize] as ::core::ffi::c_int
                        * scale_0
                        + 128i32
                        >> 8i32)
                        .saturating_truncate();
                (*mvc.offset(i as isize))[1usize] =
                    ((*(*l0).mv16x16.offset(mb_index_0 as isize))[1usize] as ::core::ffi::c_int
                        * scale_0
                        + 128i32
                        >> 8i32)
                        .saturating_truncate();
                i += 1;
            }
            if (*h).mb.i_mb_y < (*h).mb.i_mb_height - 1i32 {
                let mut mb_index_1 = (*h).mb.i_mb_xy + 0i32 + 1i32 * (*h).mb.i_mb_stride;
                let mut scale_1 = (curpoc - refpoc)
                    * (*l0).inv_ref_poc[((*h).mb.interlaced as ::core::ffi::c_int & field) as usize]
                        as ::core::ffi::c_int;
                (*mvc.offset(i as isize))[0usize] =
                    ((*(*l0).mv16x16.offset(mb_index_1 as isize))[0usize] as ::core::ffi::c_int
                        * scale_1
                        + 128i32
                        >> 8i32)
                        .saturating_truncate();
                (*mvc.offset(i as isize))[1usize] =
                    ((*(*l0).mv16x16.offset(mb_index_1 as isize))[1usize] as ::core::ffi::c_int
                        * scale_1
                        + 128i32
                        >> 8i32)
                        .saturating_truncate();
                i += 1;
            }
        }
        *i_mvc = i;
    }
}
