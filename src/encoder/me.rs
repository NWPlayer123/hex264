// =============== BEGIN me_h ================
pub const COST_MAX: ::core::ffi::c_int = (1i32) << 28i32;
pub const COST_MAX64: ::core::ffi::c_ulonglong = (1u64) << 60i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_me_t {
    pub i_pixel: ::core::ffi::c_int,
    pub p_cost_mv: *mut crate::stdlib::uint16_t,
    pub i_ref_cost: ::core::ffi::c_int,
    pub i_ref: ::core::ffi::c_int,
    pub weight: *const crate::src::common::mc::x264_weight_t,
    pub p_fref: [*mut crate::src::common::common::pixel; 12],
    pub p_fref_w: *mut crate::src::common::common::pixel,
    pub p_fenc: [*mut crate::src::common::common::pixel; 3],
    pub integral: *mut crate::stdlib::uint16_t,
    pub i_stride: [::core::ffi::c_int; 3],
    pub mvp: [crate::stdlib::int16_t; 2],
    pub cost_mv: ::core::ffi::c_int,
    pub cost: ::core::ffi::c_int,
    pub mv: [crate::stdlib::int16_t; 2],
}
pub mod common_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_predictor_roundclip(
        mut dst: *mut [crate::stdlib::int16_t; 2],
        mut mvc: *mut [crate::stdlib::int16_t; 2],
        mut i_mvc: ::core::ffi::c_int,
        mut mv_limit: *mut [crate::stdlib::int16_t; 2],
        mut pmv: crate::stdlib::uint32_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut cnt = 0i32;
            let mut i = 0i32;
            while i < i_mvc {
                let mut mx = ((*mvc.offset(i as isize))[0usize] as ::core::ffi::c_int + 2i32) >> 2i32;
                let mut my = ((*mvc.offset(i as isize))[1usize] as ::core::ffi::c_int + 2i32) >> 2i32;
                let mut mv = pack16to32_mask(mx, my);
                if !(mv == 0 || mv == pmv) {
                    (*dst.offset(cnt as isize))[0usize] = x264_clip3(
                        mx,
                        (*mv_limit.offset(0isize))[0usize] as ::core::ffi::c_int,
                        (*mv_limit.offset(1isize))[0usize] as ::core::ffi::c_int,
                    ) as crate::stdlib::int16_t;
                    (*dst.offset(cnt as isize))[1usize] = x264_clip3(
                        my,
                        (*mv_limit.offset(0isize))[1usize] as ::core::ffi::c_int,
                        (*mv_limit.offset(1isize))[1usize] as ::core::ffi::c_int,
                    ) as crate::stdlib::int16_t;
                    cnt += 1;
                }
                i += 1;
            }
            cnt
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_predictor_clip(
        mut dst: *mut [crate::stdlib::int16_t; 2],
        mut mvc: *mut [crate::stdlib::int16_t; 2],
        mut i_mvc: ::core::ffi::c_int,
        mut mv_limit: *mut [crate::stdlib::int16_t; 2],
        mut pmv: crate::stdlib::uint32_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut cnt = 0i32;
            let mut i = 0i32;
            let mut qpel_limit = [
                ((*mv_limit.offset(0isize))[0usize] as ::core::ffi::c_int) << 2i32,
                ((*mv_limit.offset(0isize))[1usize] as ::core::ffi::c_int) << 2i32,
                ((*mv_limit.offset(1isize))[0usize] as ::core::ffi::c_int) << 2i32,
                ((*mv_limit.offset(1isize))[1usize] as ::core::ffi::c_int) << 2i32,
            ];
            while i < i_mvc {
                let mut mv =
                    (*(&raw mut *mvc.offset(i as isize) as *mut crate::src::common::base::x264_union32_t)).i;
                let mut mx = (*mvc.offset(i as isize))[0usize] as ::core::ffi::c_int;
                let mut my = (*mvc.offset(i as isize))[1usize] as ::core::ffi::c_int;
                if !(mv == 0 || mv == pmv) {
                    (*dst.offset(cnt as isize))[0usize] =
                        x264_clip3(mx, qpel_limit[0usize], qpel_limit[2usize]) as crate::stdlib::int16_t;
                    (*dst.offset(cnt as isize))[1usize] =
                        x264_clip3(my, qpel_limit[1usize], qpel_limit[3usize]) as crate::stdlib::int16_t;
                    cnt += 1;
                }
                i += 1;
            }
            cnt
        }
    }
    use crate::src::encoder::me::{base_h::x264_clip3, macroblock_h::pack16to32_mask};
}
pub mod pixel_h {
    pub static mut x264_pixel_size: [crate::src::common::pixel::C2Rust_Unnamed_18; 12] = [
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 16u8, h: 16u8 },
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 16u8, h: 8u8 },
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 8u8, h: 16u8 },
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 8u8, h: 8u8 },
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 8u8, h: 4u8 },
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 4u8, h: 8u8 },
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 4u8, h: 4u8 },
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 4u8, h: 16u8 },
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 4u8, h: 2u8 },
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 2u8, h: 8u8 },
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 2u8, h: 4u8 },
        crate::src::common::pixel::C2Rust_Unnamed_18 { w: 2u8, h: 2u8 },
    ];
}
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
    pub extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        if v < i_min {
            i_min
        } else if v > i_max {
            i_max
        } else {
            v
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_predictor_difference(
        mut mvc: *mut [crate::stdlib::int16_t; 2],
        mut i_mvc: crate::stdlib::intptr_t,
    ) -> ::core::ffi::c_int {
        unsafe {
            let mut sum = 0i32;
            let mut i = 0i32;
            while (i as crate::stdlib::intptr_t) < i_mvc - 1isize {
                sum += crate::stdlib::abs(
                    (*mvc.offset(i as isize))[0usize] as ::core::ffi::c_int
                        - (*mvc.offset((i + 1i32) as isize))[0usize] as ::core::ffi::c_int,
                ) + crate::stdlib::abs(
                    (*mvc.offset(i as isize))[1usize] as ::core::ffi::c_int
                        - (*mvc.offset((i + 1i32) as isize))[1usize] as ::core::ffi::c_int,
                );
                i += 1;
            }
            sum
        }
    }
}
pub mod macroblock_h {
    pub static mut block_idx_x: [crate::stdlib::uint8_t; 16] =
        [0u8, 1u8, 0u8, 1u8, 2u8, 3u8, 2u8, 3u8, 0u8, 1u8, 0u8, 1u8, 2u8, 3u8, 2u8, 3u8];
    pub static mut block_idx_y: [crate::stdlib::uint8_t; 16] =
        [0u8, 0u8, 1u8, 1u8, 0u8, 0u8, 1u8, 1u8, 2u8, 2u8, 3u8, 3u8, 2u8, 2u8, 3u8, 3u8];
    pub static mut block_idx_xy_fdec: [crate::stdlib::uint16_t; 16] = [
        (0i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (1i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (0i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (1i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (2i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (3i32 * 4i32 + 0i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (2i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (3i32 * 4i32 + 1i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (0i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (1i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (0i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (1i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (2i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (3i32 * 4i32 + 2i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (2i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
        (3i32 * 4i32 + 3i32 * 4i32 * crate::src::common::common::FDEC_STRIDE) as crate::stdlib::uint16_t,
    ];
    #[inline(always)]
    pub extern "C" fn pack8to16(
        mut a: crate::stdlib::uint32_t,
        mut b: crate::stdlib::uint32_t,
    ) -> crate::stdlib::uint32_t {
        a.wrapping_add(b << 8i32)
    }
    #[inline(always)]
    pub extern "C" fn pack16to32_mask(
        mut a: ::core::ffi::c_int,
        mut b: ::core::ffi::c_int,
    ) -> crate::stdlib::uint32_t {
        ((a & 0xFFFFi32) as crate::stdlib::uint32_t).wrapping_add((b as crate::stdlib::uint32_t) << 16i32)
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
            let mut v2 = (if s >= 2i32 { v } else { v.wrapping_mul(0x101u32) }) as crate::stdlib::uint16_t;
            let mut v4 = if s >= 4i32 {
                v
            } else if s >= 2i32 {
                v.wrapping_mul(0x10001u32)
            } else {
                v.wrapping_mul(0x1010101u32)
            };
            let mut v8 =
                (v4 as crate::stdlib::uint64_t).wrapping_add((v4 as crate::stdlib::uint64_t) << 32i32);
            s *= 8i32;
            if w == 2i32 {
                (*(d.offset((s * 0i32) as isize) as *mut crate::src::common::base::x264_union16_t)).i = v2;
                if h == 1i32 {
                    return;
                }
                (*(d.offset((s * 1i32) as isize) as *mut crate::src::common::base::x264_union16_t)).i = v2;
                if h == 2i32 {
                    return;
                }
                (*(d.offset((s * 2i32) as isize) as *mut crate::src::common::base::x264_union16_t)).i = v2;
                (*(d.offset((s * 3i32) as isize) as *mut crate::src::common::base::x264_union16_t)).i = v2;
            } else if w == 4i32 {
                (*(d.offset((s * 0i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i = v4;
                if h == 1i32 {
                    return;
                }
                (*(d.offset((s * 1i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i = v4;
                if h == 2i32 {
                    return;
                }
                (*(d.offset((s * 2i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i = v4;
                (*(d.offset((s * 3i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i = v4;
            } else if w == 8i32 {
                if crate::osdep_h::WORD_SIZE == 8i32 {
                    (*(d.offset((s * 0i32) as isize) as *mut crate::src::common::base::x264_union64_t)).i =
                        v8;
                    if h == 1i32 {
                        return;
                    }
                    (*(d.offset((s * 1i32) as isize) as *mut crate::src::common::base::x264_union64_t)).i =
                        v8;
                    if h == 2i32 {
                        return;
                    }
                    (*(d.offset((s * 2i32) as isize) as *mut crate::src::common::base::x264_union64_t)).i =
                        v8;
                    (*(d.offset((s * 3i32) as isize) as *mut crate::src::common::base::x264_union64_t)).i =
                        v8;
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
                            b"./common/rectangle.h\0".as_ptr() as *const ::core::ffi::c_char,
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
                        (*(d.offset(0isize) as *mut crate::src::common::base::x264_union32_t)).i = v4;
                        (*(d.offset(4isize) as *mut crate::src::common::base::x264_union32_t)).i = v4;
                        (*(d.offset(8isize) as *mut crate::src::common::base::x264_union32_t)).i = v4;
                        (*(d.offset(12isize) as *mut crate::src::common::base::x264_union32_t)).i = v4;
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
                        b"./common/rectangle.h\0".as_ptr() as *const ::core::ffi::c_char,
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
            let mut mv_cache = (&raw mut *(&raw mut (*h).mb.cache.mv
                as *mut [[crate::stdlib::int16_t; 2]; 40])
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
    pub unsafe extern "C" fn x264_macroblock_cache_mvd(
        mut h: *mut crate::src::common::common::x264_t,
        mut x: ::core::ffi::c_int,
        mut y: ::core::ffi::c_int,
        mut width: ::core::ffi::c_int,
        mut height: ::core::ffi::c_int,
        mut i_list: ::core::ffi::c_int,
        mut mvd: crate::stdlib::uint16_t,
    ) {
        unsafe {
            let mut mvd_cache = (&raw mut *(&raw mut (*h).mb.cache.mvd
                as *mut [[crate::stdlib::uint8_t; 2]; 40])
                .offset(i_list as isize) as *mut [crate::stdlib::uint8_t; 2])
                .offset((crate::src::common::base::X264_SCAN8_0 + x + 8i32 * y) as isize)
                as *mut ::core::ffi::c_void;
            if 0 == 0 || 0 == 0 {
                crate::src::common::rectangle::x264_8_cache_mvd_func_table
                    [(width + (height << 1i32) - 3i32) as usize]
                    .expect("non-null function pointer")(
                    mvd_cache, mvd as crate::stdlib::uint32_t
                );
            } else {
                x264_macroblock_cache_rect(
                    mvd_cache,
                    width * 2i32,
                    height,
                    2i32,
                    mvd as crate::stdlib::uint32_t,
                );
            };
        }
    }
}
use crate::src::encoder::me::{
    base_h::{x264_clip3, x264_predictor_difference, x264_scan8},
    common_h::{x264_predictor_clip, x264_predictor_roundclip},
    macroblock_h::{block_idx_x, block_idx_xy_fdec, block_idx_y, pack8to16, pack16to32_mask},
    pixel_h::x264_pixel_size,
    rectangle_h::{x264_macroblock_cache_mv, x264_macroblock_cache_mvd},
};
static mut subpel_iterations: [[crate::stdlib::uint8_t; 4]; 12] = [
    [0u8, 0u8, 0u8, 0u8],
    [1u8, 1u8, 0u8, 0u8],
    [0u8, 1u8, 1u8, 0u8],
    [0u8, 2u8, 1u8, 0u8],
    [0u8, 2u8, 1u8, 1u8],
    [0u8, 2u8, 1u8, 2u8],
    [0u8, 0u8, 2u8, 2u8],
    [0u8, 0u8, 2u8, 2u8],
    [0u8, 0u8, 4u8, 10u8],
    [0u8, 0u8, 4u8, 10u8],
    [0u8, 0u8, 4u8, 10u8],
    [0u8, 0u8, 4u8, 10u8],
];
static mut mod6m1: [crate::stdlib::uint8_t; 8] = [5u8, 0u8, 1u8, 2u8, 3u8, 4u8, 5u8, 0u8];
static mut hex2: [[crate::stdlib::int8_t; 2]; 8] =
    [[-1i8, -2i8], [-2i8, 0i8], [-1i8, 2i8], [1i8, 2i8], [2i8, 0i8], [1i8, -2i8], [-1i8, -2i8], [-2i8, 0i8]];
static mut square1: [[crate::stdlib::int8_t; 2]; 9] = [
    [0i8, 0i8],
    [0i8, -1i8],
    [0i8, 1i8],
    [-1i8, 0i8],
    [1i8, 0i8],
    [-1i8, -1i8],
    [-1i8, 1i8],
    [1i8, -1i8],
    [1i8, 1i8],
];
pub unsafe extern "C" fn x264_8_me_search_ref(
    mut h: *mut crate::src::common::common::x264_t,
    mut m: *mut crate::src::encoder::me::x264_me_t,
    mut mvc: *mut [crate::stdlib::int16_t; 2],
    mut i_mvc: ::core::ffi::c_int,
    mut p_halfpel_thresh: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut bmx = 0;
        let mut bmy = 0;
        let mut pmx = 0;
        let mut pmy = 0;
        let mut mvc_temp = [[0; 2]; 16];
        let mut costs = [0; 16];
        let mut pmv = 0;
        let mut bpred_mv = 0u32;
        let bw = x264_pixel_size[(*m).i_pixel as usize].w as ::core::ffi::c_int;
        let bh = x264_pixel_size[(*m).i_pixel as usize].h as ::core::ffi::c_int;
        let i_pixel = (*m).i_pixel;
        let stride = (*m).i_stride[0usize];
        let mut i_me_range = (*h).param.analyse.i_me_range;
        let mut bcost = crate::src::encoder::me::COST_MAX;
        let mut bpred_cost = crate::src::encoder::me::COST_MAX;
        let mut p_fenc = (*m).p_fenc[0usize];
        let mut p_fref_w = (*m).p_fref_w;
        let mut mv_x_min = (*h).mb.mv_limit_fpel[0usize][0usize] as ::core::ffi::c_int;
        let mut mv_y_min = (*h).mb.mv_limit_fpel[0usize][1usize] as ::core::ffi::c_int;
        let mut mv_x_max = (*h).mb.mv_limit_fpel[1usize][0usize] as ::core::ffi::c_int;
        let mut mv_y_max = (*h).mb.mv_limit_fpel[1usize][1usize] as ::core::ffi::c_int;
        let mut mv_min = (-mv_x_min as crate::stdlib::uint32_t) << 16i32
            | -mv_y_min as crate::stdlib::uint32_t & 0x7FFFu32;
        let mut mv_max = (mv_x_max as crate::stdlib::uint32_t) << 16i32
            | mv_y_max as crate::stdlib::uint32_t & 0x7FFFu32
            | 0x8000u32;
        let mut p_cost_mvx = (*m).p_cost_mv.offset(-((*m).mvp[0usize] as ::core::ffi::c_int as isize));
        let mut p_cost_mvy = (*m).p_cost_mv.offset(-((*m).mvp[1usize] as ::core::ffi::c_int as isize));
        if (*h).mb.i_subpel_refine >= 3i32 {
            let mut pix = [0; 256];
            let mut stride2 = 16isize;
            let mut bpred_mx =
                x264_clip3((*m).mvp[0usize] as ::core::ffi::c_int, mv_x_min * 4i32, mv_x_max * 4i32);
            let mut bpred_my =
                x264_clip3((*m).mvp[1usize] as ::core::ffi::c_int, mv_y_min * 4i32, mv_y_max * 4i32);
            pmv = pack16to32_mask(bpred_mx, bpred_my);
            pmx = (bpred_mx + 2i32) >> 2i32;
            pmy = (bpred_my + 2i32) >> 2i32;
            let mut src = (*h).mc.get_ref.expect("non-null function pointer")(
                &raw mut pix as *mut crate::src::common::common::pixel,
                &raw mut stride2,
                &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                stride as crate::stdlib::intptr_t,
                bpred_mx,
                bpred_my,
                bw,
                bh,
                (*m).weight.offset(0isize),
            );
            bpred_cost = (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                p_fenc,
                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                src,
                stride2,
            ) + *p_cost_mvx.offset(bpred_mx as isize) as ::core::ffi::c_int
                + *p_cost_mvy.offset(bpred_my as isize) as ::core::ffi::c_int;
            let mut pmv_cost = bpred_cost;
            if i_mvc > 0i32 {
                let mut valid_mvcs = x264_predictor_clip(
                    (&raw mut mvc_temp as *mut [crate::stdlib::int16_t; 2]).offset(2isize),
                    mvc,
                    i_mvc,
                    &raw mut (*h).mb.mv_limit_fpel as *mut [crate::stdlib::int16_t; 2],
                    pmv,
                );
                if valid_mvcs > 0i32 {
                    (*(&raw mut *(&raw mut mvc_temp as *mut [crate::stdlib::int16_t; 2]).offset(1isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = pmv;
                    bpred_cost <<= 4i32;
                    loop {
                        let mut i = 1i32;
                        let mut cost = 0;
                        let mut stride2_0 = 16isize;
                        let mut mx = mvc_temp[(i + 1i32) as usize][0usize];
                        let mut my = mvc_temp[(i + 1i32) as usize][1usize];
                        let mut src_0 = (*h).mc.get_ref.expect("non-null function pointer")(
                            &raw mut pix as *mut crate::src::common::common::pixel,
                            &raw mut stride2_0,
                            &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                            stride as crate::stdlib::intptr_t,
                            mx,
                            my,
                            bw,
                            bh,
                            (*m).weight.offset(0isize),
                        );
                        cost = (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                            p_fenc,
                            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                            src_0,
                            stride2_0,
                        ) + *p_cost_mvx.offset(mx as isize) as ::core::ffi::c_int
                            + *p_cost_mvy.offset(my as isize) as ::core::ffi::c_int;
                        if (cost << 4i32) + i < bpred_cost {
                            bpred_cost = (cost << 4i32) + i;
                        }
                        i += 1;
                        if i > valid_mvcs {
                            break;
                        }
                    }
                    bpred_mx = mvc_temp[((bpred_cost & 15i32) + 1i32) as usize][0usize];
                    bpred_my = mvc_temp[((bpred_cost & 15i32) + 1i32) as usize][1usize];
                    bpred_cost >>= 4i32;
                }
            }
            bmx = (bpred_mx + 2i32) >> 2i32;
            bmy = (bpred_my + 2i32) >> 2i32;
            bpred_mv = pack16to32_mask(bpred_mx, bpred_my);
            if bpred_mv & 0x30003u32 != 0 {
                let mut cost_0 = (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                    p_fenc,
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                    p_fref_w.offset((bmy * stride + bmx) as isize),
                    stride as crate::stdlib::intptr_t,
                ) + (*p_cost_mvx.offset((bmx * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset((bmy * 4i32) as isize) as ::core::ffi::c_int);
                if cost_0 < bcost {
                    bcost = cost_0;
                    bmx = bmx;
                    bmy = bmy;
                }
            } else {
                bcost = bpred_cost;
            }
            if pmv != 0 {
                if bmx | bmy != 0 {
                    let mut cost_1 = (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                        p_fenc,
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        p_fref_w.offset((0i32 * stride + 0i32) as isize),
                        stride as crate::stdlib::intptr_t,
                    ) + (*p_cost_mvx.offset((0i32 * 4i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset((0i32 * 4i32) as isize) as ::core::ffi::c_int);
                    if cost_1 < bcost {
                        bcost = cost_1;
                        bmx = 0i32;
                        bmy = 0i32;
                    }
                }
            } else if pmv_cost < bcost {
                bcost = pmv_cost;
                bmx = 0i32;
                bmy = 0i32;
            }
        } else {
            pmx = x264_clip3(((*m).mvp[0usize] as ::core::ffi::c_int + 2i32) >> 2i32, mv_x_min, mv_x_max);
            bmx = pmx;
            pmy = x264_clip3(((*m).mvp[1usize] as ::core::ffi::c_int + 2i32) >> 2i32, mv_y_min, mv_y_max);
            bmy = pmy;
            pmv = pack16to32_mask(bmx, bmy);
            bcost = (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                p_fenc,
                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                p_fref_w.offset((bmy * stride + bmx) as isize),
                stride as crate::stdlib::intptr_t,
            );
            if i_mvc > 0i32 {
                let mut valid_mvcs_0 = x264_predictor_roundclip(
                    (&raw mut mvc_temp as *mut [crate::stdlib::int16_t; 2]).offset(2isize),
                    mvc,
                    i_mvc,
                    &raw mut (*h).mb.mv_limit_fpel as *mut [crate::stdlib::int16_t; 2],
                    pmv,
                );
                if valid_mvcs_0 > 0i32 {
                    (*(&raw mut *(&raw mut mvc_temp as *mut [crate::stdlib::int16_t; 2]).offset(1isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = pmv;
                    bcost <<= 4i32;
                    loop {
                        let mut i_0 = 1i32;
                        let mut cost_2 = 0;
                        let mut mx_0 = mvc_temp[(i_0 + 1i32) as usize][0usize];
                        let mut my_0 = mvc_temp[(i_0 + 1i32) as usize][1usize];
                        cost_2 = (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                            p_fenc,
                            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                            p_fref_w.offset((my_0 * stride + mx_0) as isize),
                            stride as crate::stdlib::intptr_t,
                        ) + (*p_cost_mvx.offset((mx_0 * 4i32) as isize) as ::core::ffi::c_int
                            + *p_cost_mvy.offset((my_0 * 4i32) as isize) as ::core::ffi::c_int);
                        if (cost_2 << 4i32) + i_0 < bcost {
                            bcost = (cost_2 << 4i32) + i_0;
                        }
                        i_0 += 1;
                        if i_0 > valid_mvcs_0 {
                            break;
                        }
                    }
                    bmx = mvc_temp[((bcost & 15i32) + 1i32) as usize][0usize];
                    bmy = mvc_temp[((bcost & 15i32) + 1i32) as usize][1usize];
                    bcost >>= 4i32;
                }
            }
            if pmv != 0 {
                let mut cost_3 = (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                    p_fenc,
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                    p_fref_w.offset((0i32 * stride + 0i32) as isize),
                    stride as crate::stdlib::intptr_t,
                ) + (*p_cost_mvx.offset((0i32 * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset((0i32 * 4i32) as isize) as ::core::ffi::c_int);
                if cost_3 < bcost {
                    bcost = cost_3;
                    bmx = 0i32;
                    bmy = 0i32;
                }
            }
        }
        match (*h).mb.i_me_method {
            crate::x264_h::X264_ME_DIA_1 => {
                bcost <<= 4i32;
                let mut i_1 = i_me_range;
                loop {
                    let mut pix_base = p_fref_w.offset(bmx as isize).offset((bmy * stride) as isize);
                    (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                        p_fenc,
                        pix_base.offset(0isize).offset((-(1i32) * stride) as isize),
                        pix_base.offset(0isize).offset((1i32 * stride) as isize),
                        pix_base.offset(-1isize).offset((0i32 * stride) as isize),
                        pix_base.offset(1isize).offset((0i32 * stride) as isize),
                        stride as crate::stdlib::intptr_t,
                        &raw mut costs as *mut ::core::ffi::c_int,
                    );
                    costs[0usize] += *p_cost_mvx.offset(((bmx + 0i32) * 4i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(((bmy + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int;
                    costs[1usize] += *p_cost_mvx.offset(((bmx + 0i32) * 4i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(((bmy + 1i32) * 4i32) as isize) as ::core::ffi::c_int;
                    costs[2usize] += *p_cost_mvx.offset(((bmx + -(1i32)) * 4i32) as isize)
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(((bmy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                    costs[3usize] += *p_cost_mvx.offset(((bmx + 1i32) * 4i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(((bmy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                    if ((costs[0usize] << 4i32) + 1i32) < bcost {
                        bcost = (costs[0usize] << 4i32) + 1i32;
                    }
                    if ((costs[1usize] << 4i32) + 3i32) < bcost {
                        bcost = (costs[1usize] << 4i32) + 3i32;
                    }
                    if ((costs[2usize] << 4i32) + 4i32) < bcost {
                        bcost = (costs[2usize] << 4i32) + 4i32;
                    }
                    if ((costs[3usize] << 4i32) + 12i32) < bcost {
                        bcost = (costs[3usize] << 4i32) + 12i32;
                    }
                    if bcost & 15i32 == 0 {
                        break;
                    }
                    bmx -= ((bcost as crate::stdlib::uint32_t) << 28i32) as crate::stdlib::int32_t >> 30i32;
                    bmy -= ((bcost as crate::stdlib::uint32_t) << 30i32) as crate::stdlib::int32_t >> 30i32;
                    bcost &= !(15i32);
                    i_1 -= 1;
                    if !(i_1 != 0
                        && (((bmx as crate::stdlib::uint32_t) << 16i32
                            | bmy as crate::stdlib::uint32_t & 0x7FFFu32)
                            .wrapping_add(mv_min)
                            | mv_max.wrapping_sub(
                                (bmx as crate::stdlib::uint32_t) << 16i32
                                    | bmy as crate::stdlib::uint32_t & 0x7FFFu32,
                            ))
                            & 0x80004000u32
                            == 0)
                    {
                        break;
                    }
                }
                bcost >>= 4i32;
                c2rust_current_block = 14127502640287082657;
            }
            crate::x264_h::X264_ME_HEX_1 => {
                c2rust_current_block = 3583450109353833130;
            }
            crate::x264_h::X264_ME_UMH_1 => {
                let mut omx = 0;
                let mut omy = 0;
                static mut pixel_size_shift: [crate::stdlib::uint8_t; 7] =
                    [0u8, 1u8, 1u8, 2u8, 3u8, 3u8, 4u8];
                let mut ucost1 = bcost;
                omx = pmx;
                omy = pmy;
                let mut pix_base_5 = p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                    p_fenc,
                    pix_base_5.offset(0isize).offset((-(1i32) * stride) as isize),
                    pix_base_5.offset(0isize).offset((1i32 * stride) as isize),
                    pix_base_5.offset(-1isize).offset((0i32 * stride) as isize),
                    pix_base_5.offset(1isize).offset((0i32 * stride) as isize),
                    stride as crate::stdlib::intptr_t,
                    &raw mut costs as *mut ::core::ffi::c_int,
                );
                costs[0usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((omy + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int;
                costs[1usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((omy + 1i32) * 4i32) as isize) as ::core::ffi::c_int;
                costs[2usize] += *p_cost_mvx.offset(((omx + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                costs[3usize] += *p_cost_mvx.offset(((omx + 1i32) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                if costs[0usize] < bcost {
                    bcost = costs[0usize];
                    bmx = omx + 0i32;
                    bmy = omy + -(1i32);
                }
                if costs[1usize] < bcost {
                    bcost = costs[1usize];
                    bmx = omx + 0i32;
                    bmy = omy + 1i32;
                }
                if costs[2usize] < bcost {
                    bcost = costs[2usize];
                    bmx = omx + -(1i32);
                    bmy = omy + 0i32;
                }
                if costs[3usize] < bcost {
                    bcost = costs[3usize];
                    bmx = omx + 1i32;
                    bmy = omy + 0i32;
                }
                if pmx | pmy != 0 {
                    omx = 0i32;
                    omy = 0i32;
                    let mut pix_base_6 = p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                    (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                        p_fenc,
                        pix_base_6.offset(0isize).offset((-(1i32) * stride) as isize),
                        pix_base_6.offset(0isize).offset((1i32 * stride) as isize),
                        pix_base_6.offset(-1isize).offset((0i32 * stride) as isize),
                        pix_base_6.offset(1isize).offset((0i32 * stride) as isize),
                        stride as crate::stdlib::intptr_t,
                        &raw mut costs as *mut ::core::ffi::c_int,
                    );
                    costs[0usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(((omy + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int;
                    costs[1usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(((omy + 1i32) * 4i32) as isize) as ::core::ffi::c_int;
                    costs[2usize] += *p_cost_mvx.offset(((omx + -(1i32)) * 4i32) as isize)
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                    costs[3usize] += *p_cost_mvx.offset(((omx + 1i32) * 4i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                    if costs[0usize] < bcost {
                        bcost = costs[0usize];
                        bmx = omx + 0i32;
                        bmy = omy + -(1i32);
                    }
                    if costs[1usize] < bcost {
                        bcost = costs[1usize];
                        bmx = omx + 0i32;
                        bmy = omy + 1i32;
                    }
                    if costs[2usize] < bcost {
                        bcost = costs[2usize];
                        bmx = omx + -(1i32);
                        bmy = omy + 0i32;
                    }
                    if costs[3usize] < bcost {
                        bcost = costs[3usize];
                        bmx = omx + 1i32;
                        bmy = omy + 0i32;
                    }
                }
                if i_pixel == crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int {
                    c2rust_current_block = 3583450109353833130;
                } else {
                    let mut ucost2 = 0;
                    let mut cross_start = 1i32;
                    ucost2 = bcost;
                    if bmx | bmy != 0 && (bmx - pmx) | (bmy - pmy) != 0 {
                        omx = bmx;
                        omy = bmy;
                        let mut pix_base_7 = p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                        (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                            p_fenc,
                            pix_base_7.offset(0isize).offset((-(1i32) * stride) as isize),
                            pix_base_7.offset(0isize).offset((1i32 * stride) as isize),
                            pix_base_7.offset(-1isize).offset((0i32 * stride) as isize),
                            pix_base_7.offset(1isize).offset((0i32 * stride) as isize),
                            stride as crate::stdlib::intptr_t,
                            &raw mut costs as *mut ::core::ffi::c_int,
                        );
                        costs[0usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int;
                        costs[1usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + 1i32) * 4i32) as isize) as ::core::ffi::c_int;
                        costs[2usize] += *p_cost_mvx.offset(((omx + -(1i32)) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                        costs[3usize] += *p_cost_mvx.offset(((omx + 1i32) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                        if costs[0usize] < bcost {
                            bcost = costs[0usize];
                            bmx = omx + 0i32;
                            bmy = omy + -(1i32);
                        }
                        if costs[1usize] < bcost {
                            bcost = costs[1usize];
                            bmx = omx + 0i32;
                            bmy = omy + 1i32;
                        }
                        if costs[2usize] < bcost {
                            bcost = costs[2usize];
                            bmx = omx + -(1i32);
                            bmy = omy + 0i32;
                        }
                        if costs[3usize] < bcost {
                            bcost = costs[3usize];
                            bmx = omx + 1i32;
                            bmy = omy + 0i32;
                        }
                    }
                    if bcost == ucost2 {
                        cross_start = 3i32;
                    }
                    omx = bmx;
                    omy = bmy;
                    if bcost == ucost2
                        && bcost < 2000i32 >> pixel_size_shift[i_pixel as usize] as ::core::ffi::c_int
                    {
                        let mut pix_base_8 = p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                        (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                            p_fenc,
                            pix_base_8.offset(0isize).offset((-(2i32) * stride) as isize),
                            pix_base_8.offset(-1isize).offset((-(1i32) * stride) as isize),
                            pix_base_8.offset(1isize).offset((-(1i32) * stride) as isize),
                            pix_base_8.offset(-2isize).offset((0i32 * stride) as isize),
                            stride as crate::stdlib::intptr_t,
                            &raw mut costs as *mut ::core::ffi::c_int,
                        );
                        costs[0usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + -(2i32)) * 4i32) as isize) as ::core::ffi::c_int;
                        costs[1usize] += *p_cost_mvx.offset(((omx + -(1i32)) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int;
                        costs[2usize] += *p_cost_mvx.offset(((omx + 1i32) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int;
                        costs[3usize] += *p_cost_mvx.offset(((omx + -(2i32)) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                        if costs[0usize] < bcost {
                            bcost = costs[0usize];
                            bmx = omx + 0i32;
                            bmy = omy + -(2i32);
                        }
                        if costs[1usize] < bcost {
                            bcost = costs[1usize];
                            bmx = omx + -(1i32);
                            bmy = omy + -(1i32);
                        }
                        if costs[2usize] < bcost {
                            bcost = costs[2usize];
                            bmx = omx + 1i32;
                            bmy = omy + -(1i32);
                        }
                        if costs[3usize] < bcost {
                            bcost = costs[3usize];
                            bmx = omx + -(2i32);
                            bmy = omy + 0i32;
                        }
                        let mut pix_base_9 = p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                        (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                            p_fenc,
                            pix_base_9.offset(2isize).offset((0i32 * stride) as isize),
                            pix_base_9.offset(-1isize).offset((1i32 * stride) as isize),
                            pix_base_9.offset(1isize).offset((1i32 * stride) as isize),
                            pix_base_9.offset(0isize).offset((2i32 * stride) as isize),
                            stride as crate::stdlib::intptr_t,
                            &raw mut costs as *mut ::core::ffi::c_int,
                        );
                        costs[0usize] += *p_cost_mvx.offset(((omx + 2i32) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                        costs[1usize] += *p_cost_mvx.offset(((omx + -(1i32)) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + 1i32) * 4i32) as isize) as ::core::ffi::c_int;
                        costs[2usize] += *p_cost_mvx.offset(((omx + 1i32) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + 1i32) * 4i32) as isize) as ::core::ffi::c_int;
                        costs[3usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(((omy + 2i32) * 4i32) as isize) as ::core::ffi::c_int;
                        if costs[0usize] < bcost {
                            bcost = costs[0usize];
                            bmx = omx + 2i32;
                            bmy = omy + 0i32;
                        }
                        if costs[1usize] < bcost {
                            bcost = costs[1usize];
                            bmx = omx + -(1i32);
                            bmy = omy + 1i32;
                        }
                        if costs[2usize] < bcost {
                            bcost = costs[2usize];
                            bmx = omx + 1i32;
                            bmy = omy + 1i32;
                        }
                        if costs[3usize] < bcost {
                            bcost = costs[3usize];
                            bmx = omx + 0i32;
                            bmy = omy + 2i32;
                        }
                        if bcost == ucost1
                            && bcost < 500i32 >> pixel_size_shift[i_pixel as usize] as ::core::ffi::c_int
                        {
                            c2rust_current_block = 14127502640287082657;
                        } else if bcost == ucost2 {
                            let mut i_3 = 3i32;
                            let mut range = i_me_range >> 1i32 | 1i32;
                            if range
                                <= (if mv_x_max - omx < omx - mv_x_min {
                                    mv_x_max - omx
                                } else {
                                    omx - mv_x_min
                                })
                            {
                                while i_3 < range - 2i32 {
                                    let mut pix_base_10 =
                                        p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                                    (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        pix_base_10.offset(i_3 as isize).offset((0i32 * stride) as isize),
                                        pix_base_10.offset(-i_3 as isize).offset((0i32 * stride) as isize),
                                        pix_base_10
                                            .offset((i_3 + 2i32) as isize)
                                            .offset((0i32 * stride) as isize),
                                        pix_base_10
                                            .offset((-i_3 - 2i32) as isize)
                                            .offset((0i32 * stride) as isize),
                                        stride as crate::stdlib::intptr_t,
                                        &raw mut costs as *mut ::core::ffi::c_int,
                                    );
                                    costs[0usize] += *p_cost_mvx.offset(((omx + i_3) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[1usize] += *p_cost_mvx.offset(((omx + -i_3) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[2usize] += *p_cost_mvx
                                        .offset(((omx + (i_3 + 2i32)) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[3usize] += *p_cost_mvx
                                        .offset(((omx + (-i_3 - 2i32)) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    if costs[0usize] < bcost {
                                        bcost = costs[0usize];
                                        bmx = omx + i_3;
                                        bmy = omy + 0i32;
                                    }
                                    if costs[1usize] < bcost {
                                        bcost = costs[1usize];
                                        bmx = omx + -i_3;
                                        bmy = omy + 0i32;
                                    }
                                    if costs[2usize] < bcost {
                                        bcost = costs[2usize];
                                        bmx = omx + (i_3 + 2i32);
                                        bmy = omy + 0i32;
                                    }
                                    if costs[3usize] < bcost {
                                        bcost = costs[3usize];
                                        bmx = omx + (-i_3 - 2i32);
                                        bmy = omy + 0i32;
                                    }
                                    i_3 += 4i32;
                                }
                            }
                            while i_3 < range {
                                if omx + i_3 <= mv_x_max {
                                    let mut cost_4 = (*h).pixf.fpelcmp[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                        p_fref_w.offset((omy * stride + (omx + i_3)) as isize),
                                        stride as crate::stdlib::intptr_t,
                                    ) + (*p_cost_mvx.offset(((omx + i_3) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset((omy * 4i32) as isize) as ::core::ffi::c_int);
                                    if cost_4 < bcost {
                                        bcost = cost_4;
                                        bmx = omx + i_3;
                                        bmy = omy;
                                    }
                                }
                                if omx - i_3 >= mv_x_min {
                                    let mut cost_5 = (*h).pixf.fpelcmp[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                        p_fref_w.offset((omy * stride + (omx - i_3)) as isize),
                                        stride as crate::stdlib::intptr_t,
                                    ) + (*p_cost_mvx.offset(((omx - i_3) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset((omy * 4i32) as isize) as ::core::ffi::c_int);
                                    if cost_5 < bcost {
                                        bcost = cost_5;
                                        bmx = omx - i_3;
                                        bmy = omy;
                                    }
                                }
                                i_3 += 2i32;
                            }
                            i_3 = 3i32;
                            if range
                                <= (if mv_y_max - omy < omy - mv_y_min {
                                    mv_y_max - omy
                                } else {
                                    omy - mv_y_min
                                })
                            {
                                while i_3 < range - 2i32 {
                                    let mut pix_base_11 =
                                        p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                                    (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        pix_base_11.offset(0isize).offset((i_3 * stride) as isize),
                                        pix_base_11.offset(0isize).offset((-i_3 * stride) as isize),
                                        pix_base_11.offset(0isize).offset(((i_3 + 2i32) * stride) as isize),
                                        pix_base_11.offset(0isize).offset(((-i_3 - 2i32) * stride) as isize),
                                        stride as crate::stdlib::intptr_t,
                                        &raw mut costs as *mut ::core::ffi::c_int,
                                    );
                                    costs[0usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + i_3) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[1usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + -i_3) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[2usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + (i_3 + 2i32)) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[3usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + (-i_3 - 2i32)) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    if costs[0usize] < bcost {
                                        bcost = costs[0usize];
                                        bmx = omx + 0i32;
                                        bmy = omy + i_3;
                                    }
                                    if costs[1usize] < bcost {
                                        bcost = costs[1usize];
                                        bmx = omx + 0i32;
                                        bmy = omy + -i_3;
                                    }
                                    if costs[2usize] < bcost {
                                        bcost = costs[2usize];
                                        bmx = omx + 0i32;
                                        bmy = omy + (i_3 + 2i32);
                                    }
                                    if costs[3usize] < bcost {
                                        bcost = costs[3usize];
                                        bmx = omx + 0i32;
                                        bmy = omy + (-i_3 - 2i32);
                                    }
                                    i_3 += 4i32;
                                }
                            }
                            while i_3 < range {
                                if omy + i_3 <= mv_y_max {
                                    let mut cost_6 = (*h).pixf.fpelcmp[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                        p_fref_w.offset(((omy + i_3) * stride + omx) as isize),
                                        stride as crate::stdlib::intptr_t,
                                    ) + (*p_cost_mvx.offset((omx * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + i_3) * 4i32) as isize)
                                            as ::core::ffi::c_int);
                                    if cost_6 < bcost {
                                        bcost = cost_6;
                                        bmx = omx;
                                        bmy = omy + i_3;
                                    }
                                }
                                if omy - i_3 >= mv_y_min {
                                    let mut cost_7 = (*h).pixf.fpelcmp[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                        p_fref_w.offset(((omy - i_3) * stride + omx) as isize),
                                        stride as crate::stdlib::intptr_t,
                                    ) + (*p_cost_mvx.offset((omx * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy - i_3) * 4i32) as isize)
                                            as ::core::ffi::c_int);
                                    if cost_7 < bcost {
                                        bcost = cost_7;
                                        bmx = omx;
                                        bmy = omy - i_3;
                                    }
                                }
                                i_3 += 2i32;
                            }
                            let mut pix_base_12 =
                                p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                            (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                                p_fenc,
                                pix_base_12.offset(-1isize).offset((-(2i32) * stride) as isize),
                                pix_base_12.offset(1isize).offset((-(2i32) * stride) as isize),
                                pix_base_12.offset(-2isize).offset((-(1i32) * stride) as isize),
                                pix_base_12.offset(2isize).offset((-(1i32) * stride) as isize),
                                stride as crate::stdlib::intptr_t,
                                &raw mut costs as *mut ::core::ffi::c_int,
                            );
                            costs[0usize] += *p_cost_mvx.offset(((omx + -(1i32)) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + -(2i32)) * 4i32) as isize) as ::core::ffi::c_int;
                            costs[1usize] += *p_cost_mvx.offset(((omx + 1i32) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + -(2i32)) * 4i32) as isize) as ::core::ffi::c_int;
                            costs[2usize] += *p_cost_mvx.offset(((omx + -(2i32)) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int;
                            costs[3usize] += *p_cost_mvx.offset(((omx + 2i32) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int;
                            if costs[0usize] < bcost {
                                bcost = costs[0usize];
                                bmx = omx + -(1i32);
                                bmy = omy + -(2i32);
                            }
                            if costs[1usize] < bcost {
                                bcost = costs[1usize];
                                bmx = omx + 1i32;
                                bmy = omy + -(2i32);
                            }
                            if costs[2usize] < bcost {
                                bcost = costs[2usize];
                                bmx = omx + -(2i32);
                                bmy = omy + -(1i32);
                            }
                            if costs[3usize] < bcost {
                                bcost = costs[3usize];
                                bmx = omx + 2i32;
                                bmy = omy + -(1i32);
                            }
                            let mut pix_base_13 =
                                p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                            (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                                p_fenc,
                                pix_base_13.offset(-2isize).offset((1i32 * stride) as isize),
                                pix_base_13.offset(2isize).offset((1i32 * stride) as isize),
                                pix_base_13.offset(-1isize).offset((2i32 * stride) as isize),
                                pix_base_13.offset(1isize).offset((2i32 * stride) as isize),
                                stride as crate::stdlib::intptr_t,
                                &raw mut costs as *mut ::core::ffi::c_int,
                            );
                            costs[0usize] += *p_cost_mvx.offset(((omx + -(2i32)) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + 1i32) * 4i32) as isize) as ::core::ffi::c_int;
                            costs[1usize] += *p_cost_mvx.offset(((omx + 2i32) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + 1i32) * 4i32) as isize) as ::core::ffi::c_int;
                            costs[2usize] += *p_cost_mvx.offset(((omx + -(1i32)) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + 2i32) * 4i32) as isize) as ::core::ffi::c_int;
                            costs[3usize] += *p_cost_mvx.offset(((omx + 1i32) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + 2i32) * 4i32) as isize) as ::core::ffi::c_int;
                            if costs[0usize] < bcost {
                                bcost = costs[0usize];
                                bmx = omx + -(2i32);
                                bmy = omy + 1i32;
                            }
                            if costs[1usize] < bcost {
                                bcost = costs[1usize];
                                bmx = omx + 2i32;
                                bmy = omy + 1i32;
                            }
                            if costs[2usize] < bcost {
                                bcost = costs[2usize];
                                bmx = omx + -(1i32);
                                bmy = omy + 2i32;
                            }
                            if costs[3usize] < bcost {
                                bcost = costs[3usize];
                                bmx = omx + 1i32;
                                bmy = omy + 2i32;
                            }
                            if bcost == ucost2 {
                                c2rust_current_block = 14127502640287082657;
                            } else {
                                cross_start = range + 2i32;
                                c2rust_current_block = 3988218931164863998;
                            }
                        } else {
                            c2rust_current_block = 3988218931164863998;
                        }
                    } else {
                        c2rust_current_block = 3988218931164863998;
                    }
                    match c2rust_current_block {
                        14127502640287082657 => {}
                        _ => {
                            if i_mvc != 0 {
                                let mut mvd = 0;
                                let mut denom = 1i32;
                                static mut range_mul: [[crate::stdlib::uint8_t; 4]; 4] = [
                                    [3u8, 3u8, 4u8, 4u8],
                                    [3u8, 4u8, 4u8, 4u8],
                                    [4u8, 4u8, 4u8, 5u8],
                                    [4u8, 4u8, 5u8, 6u8],
                                ];
                                if i_mvc == 1i32 {
                                    if i_pixel == crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int
                                    {
                                        mvd = 25i32;
                                    } else {
                                        mvd = crate::stdlib::abs(
                                            (*m).mvp[0usize] as ::core::ffi::c_int
                                                - (*mvc.offset(0isize))[0usize] as ::core::ffi::c_int,
                                        ) + crate::stdlib::abs(
                                            (*m).mvp[1usize] as ::core::ffi::c_int
                                                - (*mvc.offset(0isize))[1usize] as ::core::ffi::c_int,
                                        );
                                    }
                                } else {
                                    denom = i_mvc - 1i32;
                                    mvd = 0i32;
                                    if i_pixel != crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int
                                    {
                                        mvd = crate::stdlib::abs(
                                            (*m).mvp[0usize] as ::core::ffi::c_int
                                                - (*mvc.offset(0isize))[0usize] as ::core::ffi::c_int,
                                        ) + crate::stdlib::abs(
                                            (*m).mvp[1usize] as ::core::ffi::c_int
                                                - (*mvc.offset(0isize))[1usize] as ::core::ffi::c_int,
                                        );
                                        denom += 1;
                                    }
                                    mvd += x264_predictor_difference(mvc, i_mvc as crate::stdlib::intptr_t);
                                }
                                let mut sad_ctx = if bcost
                                    < 1000i32 >> pixel_size_shift[i_pixel as usize] as ::core::ffi::c_int
                                {
                                    0i32
                                } else if bcost
                                    < 2000i32 >> pixel_size_shift[i_pixel as usize] as ::core::ffi::c_int
                                {
                                    1i32
                                } else if bcost
                                    < 4000i32 >> pixel_size_shift[i_pixel as usize] as ::core::ffi::c_int
                                {
                                    2i32
                                } else {
                                    3i32
                                };
                                let mut mvd_ctx = if mvd < 10i32 * denom {
                                    0i32
                                } else if mvd < 20i32 * denom {
                                    1i32
                                } else if mvd < 40i32 * denom {
                                    2i32
                                } else {
                                    3i32
                                };
                                i_me_range = (i_me_range
                                    * range_mul[mvd_ctx as usize][sad_ctx as usize] as ::core::ffi::c_int)
                                    >> 2i32;
                            }
                            let mut i_4 = cross_start;
                            if i_me_range
                                <= (if mv_x_max - omx < omx - mv_x_min {
                                    mv_x_max - omx
                                } else {
                                    omx - mv_x_min
                                })
                            {
                                while i_4 < i_me_range - 2i32 {
                                    let mut pix_base_14 =
                                        p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                                    (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        pix_base_14.offset(i_4 as isize).offset((0i32 * stride) as isize),
                                        pix_base_14.offset(-i_4 as isize).offset((0i32 * stride) as isize),
                                        pix_base_14
                                            .offset((i_4 + 2i32) as isize)
                                            .offset((0i32 * stride) as isize),
                                        pix_base_14
                                            .offset((-i_4 - 2i32) as isize)
                                            .offset((0i32 * stride) as isize),
                                        stride as crate::stdlib::intptr_t,
                                        &raw mut costs as *mut ::core::ffi::c_int,
                                    );
                                    costs[0usize] += *p_cost_mvx.offset(((omx + i_4) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[1usize] += *p_cost_mvx.offset(((omx + -i_4) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[2usize] += *p_cost_mvx
                                        .offset(((omx + (i_4 + 2i32)) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[3usize] += *p_cost_mvx
                                        .offset(((omx + (-i_4 - 2i32)) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + 0i32) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    if costs[0usize] < bcost {
                                        bcost = costs[0usize];
                                        bmx = omx + i_4;
                                        bmy = omy + 0i32;
                                    }
                                    if costs[1usize] < bcost {
                                        bcost = costs[1usize];
                                        bmx = omx + -i_4;
                                        bmy = omy + 0i32;
                                    }
                                    if costs[2usize] < bcost {
                                        bcost = costs[2usize];
                                        bmx = omx + (i_4 + 2i32);
                                        bmy = omy + 0i32;
                                    }
                                    if costs[3usize] < bcost {
                                        bcost = costs[3usize];
                                        bmx = omx + (-i_4 - 2i32);
                                        bmy = omy + 0i32;
                                    }
                                    i_4 += 4i32;
                                }
                            }
                            while i_4 < i_me_range {
                                if omx + i_4 <= mv_x_max {
                                    let mut cost_8 = (*h).pixf.fpelcmp[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                        p_fref_w.offset((omy * stride + (omx + i_4)) as isize),
                                        stride as crate::stdlib::intptr_t,
                                    ) + (*p_cost_mvx.offset(((omx + i_4) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset((omy * 4i32) as isize) as ::core::ffi::c_int);
                                    if cost_8 < bcost {
                                        bcost = cost_8;
                                        bmx = omx + i_4;
                                        bmy = omy;
                                    }
                                }
                                if omx - i_4 >= mv_x_min {
                                    let mut cost_9 = (*h).pixf.fpelcmp[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                        p_fref_w.offset((omy * stride + (omx - i_4)) as isize),
                                        stride as crate::stdlib::intptr_t,
                                    ) + (*p_cost_mvx.offset(((omx - i_4) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset((omy * 4i32) as isize) as ::core::ffi::c_int);
                                    if cost_9 < bcost {
                                        bcost = cost_9;
                                        bmx = omx - i_4;
                                        bmy = omy;
                                    }
                                }
                                i_4 += 2i32;
                            }
                            i_4 = cross_start;
                            if i_me_range >> 1i32
                                <= (if mv_y_max - omy < omy - mv_y_min {
                                    mv_y_max - omy
                                } else {
                                    omy - mv_y_min
                                })
                            {
                                while i_4 < (i_me_range >> 1i32) - 2i32 {
                                    let mut pix_base_15 =
                                        p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                                    (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        pix_base_15.offset(0isize).offset((i_4 * stride) as isize),
                                        pix_base_15.offset(0isize).offset((-i_4 * stride) as isize),
                                        pix_base_15.offset(0isize).offset(((i_4 + 2i32) * stride) as isize),
                                        pix_base_15.offset(0isize).offset(((-i_4 - 2i32) * stride) as isize),
                                        stride as crate::stdlib::intptr_t,
                                        &raw mut costs as *mut ::core::ffi::c_int,
                                    );
                                    costs[0usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + i_4) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[1usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + -i_4) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[2usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + (i_4 + 2i32)) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    costs[3usize] += *p_cost_mvx.offset(((omx + 0i32) * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + (-i_4 - 2i32)) * 4i32) as isize)
                                            as ::core::ffi::c_int;
                                    if costs[0usize] < bcost {
                                        bcost = costs[0usize];
                                        bmx = omx + 0i32;
                                        bmy = omy + i_4;
                                    }
                                    if costs[1usize] < bcost {
                                        bcost = costs[1usize];
                                        bmx = omx + 0i32;
                                        bmy = omy + -i_4;
                                    }
                                    if costs[2usize] < bcost {
                                        bcost = costs[2usize];
                                        bmx = omx + 0i32;
                                        bmy = omy + (i_4 + 2i32);
                                    }
                                    if costs[3usize] < bcost {
                                        bcost = costs[3usize];
                                        bmx = omx + 0i32;
                                        bmy = omy + (-i_4 - 2i32);
                                    }
                                    i_4 += 4i32;
                                }
                            }
                            while i_4 < i_me_range >> 1i32 {
                                if omy + i_4 <= mv_y_max {
                                    let mut cost_10 = (*h).pixf.fpelcmp[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                        p_fref_w.offset(((omy + i_4) * stride + omx) as isize),
                                        stride as crate::stdlib::intptr_t,
                                    ) + (*p_cost_mvx.offset((omx * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy + i_4) * 4i32) as isize)
                                            as ::core::ffi::c_int);
                                    if cost_10 < bcost {
                                        bcost = cost_10;
                                        bmx = omx;
                                        bmy = omy + i_4;
                                    }
                                }
                                if omy - i_4 >= mv_y_min {
                                    let mut cost_11 = (*h).pixf.fpelcmp[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                        p_fref_w.offset(((omy - i_4) * stride + omx) as isize),
                                        stride as crate::stdlib::intptr_t,
                                    ) + (*p_cost_mvx.offset((omx * 4i32) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_mvy.offset(((omy - i_4) * 4i32) as isize)
                                            as ::core::ffi::c_int);
                                    if cost_11 < bcost {
                                        bcost = cost_11;
                                        bmx = omx;
                                        bmy = omy - i_4;
                                    }
                                }
                                i_4 += 2i32;
                            }
                            let mut pix_base_16 =
                                p_fref_w.offset(omx as isize).offset((omy * stride) as isize);
                            (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                                p_fenc,
                                pix_base_16.offset(-2isize).offset((-(2i32) * stride) as isize),
                                pix_base_16.offset(-2isize).offset((2i32 * stride) as isize),
                                pix_base_16.offset(2isize).offset((-(2i32) * stride) as isize),
                                pix_base_16.offset(2isize).offset((2i32 * stride) as isize),
                                stride as crate::stdlib::intptr_t,
                                &raw mut costs as *mut ::core::ffi::c_int,
                            );
                            costs[0usize] += *p_cost_mvx.offset(((omx + -(2i32)) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + -(2i32)) * 4i32) as isize) as ::core::ffi::c_int;
                            costs[1usize] += *p_cost_mvx.offset(((omx + -(2i32)) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + 2i32) * 4i32) as isize) as ::core::ffi::c_int;
                            costs[2usize] += *p_cost_mvx.offset(((omx + 2i32) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + -(2i32)) * 4i32) as isize) as ::core::ffi::c_int;
                            costs[3usize] += *p_cost_mvx.offset(((omx + 2i32) * 4i32) as isize)
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset(((omy + 2i32) * 4i32) as isize) as ::core::ffi::c_int;
                            if costs[0usize] < bcost {
                                bcost = costs[0usize];
                                bmx = omx + -(2i32);
                                bmy = omy + -(2i32);
                            }
                            if costs[1usize] < bcost {
                                bcost = costs[1usize];
                                bmx = omx + -(2i32);
                                bmy = omy + 2i32;
                            }
                            if costs[2usize] < bcost {
                                bcost = costs[2usize];
                                bmx = omx + 2i32;
                                bmy = omy + -(2i32);
                            }
                            if costs[3usize] < bcost {
                                bcost = costs[3usize];
                                bmx = omx + 2i32;
                                bmy = omy + 2i32;
                            }
                            omx = bmx;
                            omy = bmy;
                            let mut p_cost_omvx = p_cost_mvx.offset((omx * 4i32) as isize);
                            let mut p_cost_omvy = p_cost_mvy.offset((omy * 4i32) as isize);
                            loop {
                                let mut i_5 = 1i32;
                                static mut hex4: [[crate::stdlib::int8_t; 2]; 16] = [
                                    [0i8, -4i8],
                                    [0i8, 4i8],
                                    [-2i8, -3i8],
                                    [2i8, -3i8],
                                    [-4i8, -2i8],
                                    [4i8, -2i8],
                                    [-4i8, -1i8],
                                    [4i8, -1i8],
                                    [-4i8, 0i8],
                                    [4i8, 0i8],
                                    [-4i8, 1i8],
                                    [4i8, 1i8],
                                    [-4i8, 2i8],
                                    [4i8, 2i8],
                                    [-2i8, 3i8],
                                    [2i8, 3i8],
                                ];
                                if 4i32 * i_5
                                    > (if mv_x_max - omx
                                        < (if omx - mv_x_min
                                            < (if mv_y_max - omy < omy - mv_y_min {
                                                mv_y_max - omy
                                            } else {
                                                omy - mv_y_min
                                            })
                                        {
                                            omx - mv_x_min
                                        } else {
                                            if mv_y_max - omy < omy - mv_y_min {
                                                mv_y_max - omy
                                            } else {
                                                omy - mv_y_min
                                            }
                                        })
                                    {
                                        mv_x_max - omx
                                    } else {
                                        if omx - mv_x_min
                                            < (if mv_y_max - omy < omy - mv_y_min {
                                                mv_y_max - omy
                                            } else {
                                                omy - mv_y_min
                                            })
                                        {
                                            omx - mv_x_min
                                        } else {
                                            if mv_y_max - omy < omy - mv_y_min {
                                                mv_y_max - omy
                                            } else {
                                                omy - mv_y_min
                                            }
                                        }
                                    })
                                {
                                    let mut j = 0i32;
                                    while j < 16i32 {
                                        let mut mx_1 =
                                            omx + hex4[j as usize][0usize] as ::core::ffi::c_int * i_5;
                                        let mut my_1 =
                                            omy + hex4[j as usize][1usize] as ::core::ffi::c_int * i_5;
                                        if (((mx_1 as crate::stdlib::uint32_t) << 16i32
                                            | my_1 as crate::stdlib::uint32_t & 0x7FFFu32)
                                            .wrapping_add(mv_min)
                                            | mv_max.wrapping_sub(
                                                (mx_1 as crate::stdlib::uint32_t) << 16i32
                                                    | my_1 as crate::stdlib::uint32_t & 0x7FFFu32,
                                            ))
                                            & 0x80004000u32
                                            == 0
                                        {
                                            let mut cost_12 = (*h).pixf.fpelcmp[i_pixel as usize]
                                                .expect("non-null function pointer")(
                                                p_fenc,
                                                crate::src::common::common::FENC_STRIDE
                                                    as crate::stdlib::intptr_t,
                                                p_fref_w.offset((my_1 * stride + mx_1) as isize),
                                                stride as crate::stdlib::intptr_t,
                                            ) + (*p_cost_mvx.offset((mx_1 * 4i32) as isize)
                                                as ::core::ffi::c_int
                                                + *p_cost_mvy.offset((my_1 * 4i32) as isize)
                                                    as ::core::ffi::c_int);
                                            if cost_12 < bcost {
                                                bcost = cost_12;
                                                bmx = mx_1;
                                                bmy = my_1;
                                            }
                                        }
                                        j += 1;
                                    }
                                } else {
                                    let mut dir_0 = 0i32;
                                    let mut pix_base_17 = p_fref_w
                                        .offset(omx as isize)
                                        .offset(((omy - 4i32 * i_5) * stride) as isize);
                                    let mut dy = i_5 * stride;
                                    (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        pix_base_17
                                            .offset((0i32 * i_5) as isize)
                                            .offset(((-(4i32) - 2i32 * 0i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset((0i32 * i_5) as isize)
                                            .offset(((4i32 - 2i32 * 0i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset(-((2i32 * i_5) as isize))
                                            .offset(((-(3i32) - 2i32 * 0i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset((2i32 * i_5) as isize)
                                            .offset(((-(3i32) - 2i32 * 0i32 + 4i32) * dy) as isize),
                                        stride as crate::stdlib::intptr_t,
                                        (&raw mut costs as *mut ::core::ffi::c_int)
                                            .offset((4i32 * 0i32) as isize),
                                    );
                                    pix_base_17 = pix_base_17.offset((2i32 * dy) as isize);
                                    (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        pix_base_17
                                            .offset(-((4i32 * i_5) as isize))
                                            .offset(((-(2i32) - 2i32 * 1i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset((4i32 * i_5) as isize)
                                            .offset(((-(2i32) - 2i32 * 1i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset(-((4i32 * i_5) as isize))
                                            .offset(((-(1i32) - 2i32 * 1i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset((4i32 * i_5) as isize)
                                            .offset(((-(1i32) - 2i32 * 1i32 + 4i32) * dy) as isize),
                                        stride as crate::stdlib::intptr_t,
                                        (&raw mut costs as *mut ::core::ffi::c_int)
                                            .offset((4i32 * 1i32) as isize),
                                    );
                                    pix_base_17 = pix_base_17.offset((2i32 * dy) as isize);
                                    (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        pix_base_17
                                            .offset(-((4i32 * i_5) as isize))
                                            .offset(((0i32 - 2i32 * 2i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset((4i32 * i_5) as isize)
                                            .offset(((0i32 - 2i32 * 2i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset(-((4i32 * i_5) as isize))
                                            .offset(((1i32 - 2i32 * 2i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset((4i32 * i_5) as isize)
                                            .offset(((1i32 - 2i32 * 2i32 + 4i32) * dy) as isize),
                                        stride as crate::stdlib::intptr_t,
                                        (&raw mut costs as *mut ::core::ffi::c_int)
                                            .offset((4i32 * 2i32) as isize),
                                    );
                                    pix_base_17 = pix_base_17.offset((2i32 * dy) as isize);
                                    (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                        .expect("non-null function pointer")(
                                        p_fenc,
                                        pix_base_17
                                            .offset(-((4i32 * i_5) as isize))
                                            .offset(((2i32 - 2i32 * 3i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset((4i32 * i_5) as isize)
                                            .offset(((2i32 - 2i32 * 3i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset(-((2i32 * i_5) as isize))
                                            .offset(((3i32 - 2i32 * 3i32 + 4i32) * dy) as isize),
                                        pix_base_17
                                            .offset((2i32 * i_5) as isize)
                                            .offset(((3i32 - 2i32 * 3i32 + 4i32) * dy) as isize),
                                        stride as crate::stdlib::intptr_t,
                                        (&raw mut costs as *mut ::core::ffi::c_int)
                                            .offset((4i32 * 3i32) as isize),
                                    );
                                    pix_base_17 = pix_base_17.offset((2i32 * dy) as isize);
                                    costs[0usize] += *p_cost_omvx.offset((0i32 * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((-(4i32) * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[1usize] += *p_cost_omvx.offset((0i32 * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((4i32 * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[2usize] += *p_cost_omvx.offset((-(2i32) * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((-(3i32) * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[3usize] += *p_cost_omvx.offset((2i32 * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((-(3i32) * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[4usize] += *p_cost_omvx.offset((-(4i32) * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((-(2i32) * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[5usize] += *p_cost_omvx.offset((4i32 * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((-(2i32) * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[6usize] += *p_cost_omvx.offset((-(4i32) * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((-(1i32) * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[7usize] += *p_cost_omvx.offset((4i32 * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((-(1i32) * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[8usize] += *p_cost_omvx.offset((-(4i32) * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((0i32 * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[9usize] += *p_cost_omvx.offset((4i32 * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((0i32 * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[10usize] += *p_cost_omvx.offset((-(4i32) * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((1i32 * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[11usize] += *p_cost_omvx.offset((4i32 * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((1i32 * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[12usize] += *p_cost_omvx.offset((-(4i32) * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((2i32 * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[13usize] += *p_cost_omvx.offset((4i32 * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((2i32 * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[14usize] += *p_cost_omvx.offset((-(2i32) * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((3i32 * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    costs[15usize] += *p_cost_omvx.offset((2i32 * 4i32 * i_5) as isize)
                                        as ::core::ffi::c_int
                                        + *p_cost_omvy.offset((3i32 * 4i32 * i_5) as isize)
                                            as ::core::ffi::c_int;
                                    if costs[0usize] < bcost {
                                        bcost = costs[0usize];
                                        dir_0 = 0i32 * 16i32 + (-(4i32) & 15i32);
                                    }
                                    if costs[1usize] < bcost {
                                        bcost = costs[1usize];
                                        dir_0 = 0i32 * 16i32 + (4i32 & 15i32);
                                    }
                                    if costs[2usize] < bcost {
                                        bcost = costs[2usize];
                                        dir_0 = -(2i32) * 16i32 + (-(3i32) & 15i32);
                                    }
                                    if costs[3usize] < bcost {
                                        bcost = costs[3usize];
                                        dir_0 = 2i32 * 16i32 + (-(3i32) & 15i32);
                                    }
                                    if costs[4usize] < bcost {
                                        bcost = costs[4usize];
                                        dir_0 = -(4i32) * 16i32 + (-(2i32) & 15i32);
                                    }
                                    if costs[5usize] < bcost {
                                        bcost = costs[5usize];
                                        dir_0 = 4i32 * 16i32 + (-(2i32) & 15i32);
                                    }
                                    if costs[6usize] < bcost {
                                        bcost = costs[6usize];
                                        dir_0 = -(4i32) * 16i32 + (-(1i32) & 15i32);
                                    }
                                    if costs[7usize] < bcost {
                                        bcost = costs[7usize];
                                        dir_0 = 4i32 * 16i32 + (-(1i32) & 15i32);
                                    }
                                    if costs[8usize] < bcost {
                                        bcost = costs[8usize];
                                        dir_0 = -(4i32) * 16i32 + (0i32 & 15i32);
                                    }
                                    if costs[9usize] < bcost {
                                        bcost = costs[9usize];
                                        dir_0 = 4i32 * 16i32 + (0i32 & 15i32);
                                    }
                                    if costs[10usize] < bcost {
                                        bcost = costs[10usize];
                                        dir_0 = -(4i32) * 16i32 + (1i32 & 15i32);
                                    }
                                    if costs[11usize] < bcost {
                                        bcost = costs[11usize];
                                        dir_0 = 4i32 * 16i32 + (1i32 & 15i32);
                                    }
                                    if costs[12usize] < bcost {
                                        bcost = costs[12usize];
                                        dir_0 = -(4i32) * 16i32 + (2i32 & 15i32);
                                    }
                                    if costs[13usize] < bcost {
                                        bcost = costs[13usize];
                                        dir_0 = 4i32 * 16i32 + (2i32 & 15i32);
                                    }
                                    if costs[14usize] < bcost {
                                        bcost = costs[14usize];
                                        dir_0 = -(2i32) * 16i32 + (3i32 & 15i32);
                                    }
                                    if costs[15usize] < bcost {
                                        bcost = costs[15usize];
                                        dir_0 = 2i32 * 16i32 + (3i32 & 15i32);
                                    }
                                    if dir_0 != 0 {
                                        bmx = omx + i_5 * (dir_0 >> 4i32);
                                        bmy = omy
                                            + i_5
                                                * (((dir_0 as crate::stdlib::uint32_t) << 28i32)
                                                    as crate::stdlib::int32_t
                                                    >> 28i32);
                                    }
                                }
                                i_5 += 1;
                                if i_5 > i_me_range >> 2i32 {
                                    break;
                                }
                            }
                            if bmy <= mv_y_max && bmy >= mv_y_min && bmx <= mv_x_max && bmx >= mv_x_min {
                                c2rust_current_block = 3583450109353833130;
                            } else {
                                c2rust_current_block = 14127502640287082657;
                            }
                        }
                    }
                }
            }
            crate::x264_h::X264_ME_ESA_1 | crate::x264_h::X264_ME_TESA => {
                let mut enc_dc = [0; 4];
                let mut xn = 0;
                let min_x = if bmx - i_me_range > mv_x_min { bmx - i_me_range } else { mv_x_min };
                let min_y = if bmy - i_me_range > mv_y_min { bmy - i_me_range } else { mv_y_min };
                let max_x = if bmx + i_me_range < mv_x_max { bmx + i_me_range } else { mv_x_max };
                let max_y = if bmy + i_me_range < mv_y_max { bmy + i_me_range } else { mv_y_max };
                let width = (max_x - min_x + 3i32) & !(3i32);
                let mut sums_base = (*m).integral;
                let mut sad_size = if i_pixel <= crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int {
                    crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int
                } else {
                    crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int
                };
                let mut delta = x264_pixel_size[sad_size as usize].w as ::core::ffi::c_int;
                let mut xs = (*h).scratch_buffer as *mut crate::stdlib::int16_t;
                let mut cost_fpel_mvx = (*h).cost_mv_fpel[(*h).mb.i_qp as usize]
                    [(-((*m).mvp[0usize] as ::core::ffi::c_int) & 3i32) as usize]
                    .offset((-((*m).mvp[0usize] as ::core::ffi::c_int) >> 2i32) as isize);
                (*h).pixf.sad_x4[sad_size as usize].expect("non-null function pointer")(
                    &raw mut crate::src::common::tables::x264_zero as *mut crate::src::common::common::pixel,
                    p_fenc,
                    p_fenc.offset(delta as isize),
                    p_fenc.offset((delta * crate::src::common::common::FENC_STRIDE) as isize),
                    p_fenc
                        .offset(delta as isize)
                        .offset((delta * crate::src::common::common::FENC_STRIDE) as isize),
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut enc_dc as *mut ::core::ffi::c_int,
                );
                if delta == 4i32 {
                    sums_base = sums_base.offset(
                        (stride * ((*(*h).fenc).i_lines[0usize] + crate::src::common::frame::PADV * 2i32))
                            as isize,
                    );
                }
                if i_pixel == crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int
                    || i_pixel == crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int
                    || i_pixel == crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int
                {
                    delta *= stride;
                }
                if i_pixel == crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int
                    || i_pixel == crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int
                {
                    enc_dc[1usize] = enc_dc[2usize];
                }
                if (*h).mb.i_me_method == crate::x264_h::X264_ME_TESA {
                    let mut nmvsad = 0i32;
                    let mut i_9 = 0i32;
                    let mut mvsads = xs.offset(((width + 31i32) & !(31i32)) as isize).offset(4isize)
                        as *mut crate::src::common::common::mvsad_t;
                    let mut sad_thresh = if i_me_range <= 16i32 {
                        10i32
                    } else if i_me_range <= 24i32 {
                        11i32
                    } else {
                        12i32
                    };
                    let mut bsad = (*h).pixf.sad[i_pixel as usize].expect("non-null function pointer")(
                        p_fenc,
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        p_fref_w.offset((bmy * stride) as isize).offset(bmx as isize),
                        stride as crate::stdlib::intptr_t,
                    ) + (*p_cost_mvx.offset((bmx * 4i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset((bmy * 4i32) as isize) as ::core::ffi::c_int);
                    let mut my_2 = min_y;
                    while my_2 <= max_y {
                        let mut ycost = *p_cost_mvy.offset((my_2 * 4i32) as isize) as ::core::ffi::c_int;
                        if bsad > ycost {
                            let mut i_6 = 0;
                            bsad -= ycost;
                            xn = (*h).pixf.ads[i_pixel as usize].expect("non-null function pointer")(
                                &raw mut enc_dc as *mut ::core::ffi::c_int,
                                sums_base.offset(min_x as isize).offset((my_2 * stride) as isize),
                                delta,
                                cost_fpel_mvx.offset(min_x as isize),
                                xs,
                                width,
                                (bsad * 17i32) >> 4i32,
                            );
                            i_6 = 0i32;
                            while i_6 < xn - 2i32 {
                                let mut sads = [0; 4];
                                let mut j_0 = 0i32;
                                let mut ref_0 =
                                    p_fref_w.offset(min_x as isize).offset((my_2 * stride) as isize);
                                (*h).pixf.sad_x3[i_pixel as usize].expect("non-null function pointer")(
                                    p_fenc,
                                    ref_0.offset(*xs.offset(i_6 as isize) as ::core::ffi::c_int as isize),
                                    ref_0.offset(
                                        *xs.offset((i_6 + 1i32) as isize) as ::core::ffi::c_int as isize
                                    ),
                                    ref_0.offset(
                                        *xs.offset((i_6 + 2i32) as isize) as ::core::ffi::c_int as isize
                                    ),
                                    stride as crate::stdlib::intptr_t,
                                    &raw mut sads as *mut ::core::ffi::c_int,
                                );
                                while j_0 < 3i32 {
                                    let mut sad = sads[j_0 as usize]
                                        + *cost_fpel_mvx.offset(*xs.offset((i_6 + j_0) as isize) as isize)
                                            as ::core::ffi::c_int;
                                    if sad < (bsad * sad_thresh) >> 3i32 {
                                        if sad < bsad {
                                            bsad = sad;
                                        }
                                        (*mvsads.offset(nmvsad as isize)).sad = sad + ycost;
                                        (*mvsads.offset(nmvsad as isize)).mv[0usize] = (min_x
                                            + *xs.offset((i_6 + j_0) as isize) as ::core::ffi::c_int)
                                            as crate::stdlib::int16_t;
                                        (*mvsads.offset(nmvsad as isize)).mv[1usize] =
                                            my_2 as crate::stdlib::int16_t;
                                        nmvsad += 1;
                                    }
                                    j_0 += 1;
                                }
                                i_6 += 3i32;
                            }
                            while i_6 < xn {
                                let mut mx_2 = min_x + *xs.offset(i_6 as isize) as ::core::ffi::c_int;
                                let mut sad_0 = (*h).pixf.sad[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                    p_fref_w.offset(mx_2 as isize).offset((my_2 * stride) as isize),
                                    stride as crate::stdlib::intptr_t,
                                ) + *cost_fpel_mvx.offset(*xs.offset(i_6 as isize) as isize)
                                    as ::core::ffi::c_int;
                                if sad_0 < (bsad * sad_thresh) >> 3i32 {
                                    if sad_0 < bsad {
                                        bsad = sad_0;
                                    }
                                    (*mvsads.offset(nmvsad as isize)).sad = sad_0 + ycost;
                                    (*mvsads.offset(nmvsad as isize)).mv[0usize] =
                                        mx_2 as crate::stdlib::int16_t;
                                    (*mvsads.offset(nmvsad as isize)).mv[1usize] =
                                        my_2 as crate::stdlib::int16_t;
                                    nmvsad += 1;
                                }
                                i_6 += 1;
                            }
                            bsad += ycost;
                        }
                        my_2 += 1;
                    }
                    let mut limit = i_me_range >> 1i32;
                    sad_thresh = (bsad * sad_thresh) >> 3i32;
                    while nmvsad > limit * 2i32 && sad_thresh > bsad {
                        let mut i_7 = 0i32;
                        sad_thresh = (sad_thresh + bsad) >> 1i32;
                        while i_7 < nmvsad && (*mvsads.offset(i_7 as isize)).sad <= sad_thresh {
                            i_7 += 1;
                        }
                        let mut j_1 = i_7;
                        while j_1 < nmvsad {
                            let mut sad_1 = 0;
                            if crate::osdep_h::WORD_SIZE == 8i32
                                && ::core::mem::size_of::<crate::src::common::common::mvsad_t>() == 8usize
                            {
                                let ref mut c2rust_fresh0 = (*(mvsads.offset(i_7 as isize)
                                    as *mut crate::src::common::base::x264_union64_t))
                                    .i;
                                *c2rust_fresh0 = (*(mvsads.offset(j_1 as isize)
                                    as *mut crate::src::common::base::x264_union64_t))
                                    .i;
                                let mut mvsad = *c2rust_fresh0;
                                sad_1 = mvsad as crate::stdlib::uint32_t;
                            } else {
                                sad_1 = (*mvsads.offset(j_1 as isize)).sad as crate::stdlib::uint32_t;
                                (*(&raw mut (*mvsads.offset(i_7 as isize)).mv
                                    as *mut crate::src::common::base::x264_union32_t))
                                    .i = (*(&raw mut (*mvsads.offset(j_1 as isize)).mv
                                    as *mut crate::src::common::base::x264_union32_t))
                                    .i;
                                (*mvsads.offset(i_7 as isize)).sad = sad_1 as ::core::ffi::c_int;
                            }
                            i_7 = (i_7 as crate::stdlib::uint32_t).wrapping_add(
                                sad_1.wrapping_sub((sad_thresh + 1i32) as crate::stdlib::uint32_t) >> 31i32,
                            ) as ::core::ffi::c_int;
                            j_1 += 1;
                        }
                        nmvsad = i_7;
                    }
                    while nmvsad > limit {
                        let mut bi = 0i32;
                        let mut i_8 = 1i32;
                        while i_8 < nmvsad {
                            if (*mvsads.offset(i_8 as isize)).sad > (*mvsads.offset(bi as isize)).sad {
                                bi = i_8;
                            }
                            i_8 += 1;
                        }
                        nmvsad -= 1;
                        if ::core::mem::size_of::<crate::src::common::common::mvsad_t>()
                            == ::core::mem::size_of::<crate::stdlib::uint64_t>()
                        {
                            (*(mvsads.offset(bi as isize)
                                as *mut crate::src::common::base::x264_union64_t))
                                .i = (*(mvsads.offset(nmvsad as isize)
                                as *mut crate::src::common::base::x264_union64_t))
                                .i;
                        } else {
                            *mvsads.offset(bi as isize) = *mvsads.offset(nmvsad as isize);
                        }
                    }
                    while i_9 < nmvsad {
                        let mut cost_13 = (*h).pixf.fpelcmp[i_pixel as usize]
                            .expect("non-null function pointer")(
                            p_fenc,
                            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                            p_fref_w.offset(
                                (*(&raw mut (*mvsads.offset(i_9 as isize)).mv as *mut crate::stdlib::int16_t)
                                    .offset(1isize) as ::core::ffi::c_int
                                    * stride
                                    + *(&raw mut (*mvsads.offset(i_9 as isize)).mv
                                        as *mut crate::stdlib::int16_t)
                                        .offset(0isize)
                                        as ::core::ffi::c_int) as isize,
                            ),
                            stride as crate::stdlib::intptr_t,
                        ) + (*p_cost_mvx.offset(
                            ((*mvsads.offset(i_9 as isize)).mv[0usize] as ::core::ffi::c_int * 4i32) as isize,
                        ) as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((*mvsads.offset(i_9 as isize)).mv[1usize] as ::core::ffi::c_int * 4i32)
                                    as isize,
                            ) as ::core::ffi::c_int);
                        if cost_13 < bcost {
                            bcost = cost_13;
                            bmx = (*mvsads.offset(i_9 as isize)).mv[0usize] as ::core::ffi::c_int;
                            bmy = (*mvsads.offset(i_9 as isize)).mv[1usize] as ::core::ffi::c_int;
                        }
                        i_9 += 1;
                    }
                } else {
                    let mut my_3 = min_y;
                    while my_3 <= max_y {
                        let mut ycost_0 = *p_cost_mvy.offset((my_3 * 4i32) as isize) as ::core::ffi::c_int;
                        if bcost > ycost_0 {
                            let mut i_10 = 0;
                            bcost -= ycost_0;
                            xn = (*h).pixf.ads[i_pixel as usize].expect("non-null function pointer")(
                                &raw mut enc_dc as *mut ::core::ffi::c_int,
                                sums_base.offset(min_x as isize).offset((my_3 * stride) as isize),
                                delta,
                                cost_fpel_mvx.offset(min_x as isize),
                                xs,
                                width,
                                bcost,
                            );
                            i_10 = 0i32;
                            while i_10 < xn - 2i32 {
                                (*h).pixf.fpelcmp_x3[i_pixel as usize].expect("non-null function pointer")(
                                    p_fenc,
                                    p_fref_w
                                        .offset(
                                            (min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset((my_3 * stride) as isize),
                                    p_fref_w
                                        .offset(
                                            (min_x + *xs.offset((i_10 + 1i32) as isize) as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset((my_3 * stride) as isize),
                                    p_fref_w
                                        .offset(
                                            (min_x + *xs.offset((i_10 + 2i32) as isize) as ::core::ffi::c_int)
                                                as isize,
                                        )
                                        .offset((my_3 * stride) as isize),
                                    stride as crate::stdlib::intptr_t,
                                    &raw mut costs as *mut ::core::ffi::c_int,
                                );
                                costs[0usize] += *p_cost_mvx.offset(
                                    ((min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int) * 4i32)
                                        as isize,
                                ) as ::core::ffi::c_int;
                                costs[1usize] += *p_cost_mvx.offset(
                                    ((min_x + *xs.offset((i_10 + 1i32) as isize) as ::core::ffi::c_int)
                                        * 4i32) as isize,
                                ) as ::core::ffi::c_int;
                                costs[2usize] += *p_cost_mvx.offset(
                                    ((min_x + *xs.offset((i_10 + 2i32) as isize) as ::core::ffi::c_int)
                                        * 4i32) as isize,
                                ) as ::core::ffi::c_int;
                                if costs[0usize] < bcost {
                                    bcost = costs[0usize];
                                    bmx = min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int;
                                    bmy = my_3;
                                }
                                if costs[1usize] < bcost {
                                    bcost = costs[1usize];
                                    bmx = min_x + *xs.offset((i_10 + 1i32) as isize) as ::core::ffi::c_int;
                                    bmy = my_3;
                                }
                                if costs[2usize] < bcost {
                                    bcost = costs[2usize];
                                    bmx = min_x + *xs.offset((i_10 + 2i32) as isize) as ::core::ffi::c_int;
                                    bmy = my_3;
                                }
                                i_10 += 3i32;
                            }
                            bcost += ycost_0;
                            while i_10 < xn {
                                let mut cost_14 =
                                    (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                                        p_fenc,
                                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                        p_fref_w.offset(
                                            (my_3 * stride
                                                + (min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int))
                                                as isize,
                                        ),
                                        stride as crate::stdlib::intptr_t,
                                    ) + (*p_cost_mvx.offset(
                                        ((min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int) * 4i32)
                                            as isize,
                                    ) as ::core::ffi::c_int
                                        + *p_cost_mvy.offset((my_3 * 4i32) as isize) as ::core::ffi::c_int);
                                if cost_14 < bcost {
                                    bcost = cost_14;
                                    bmx = min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int;
                                    bmy = my_3;
                                }
                                i_10 += 1;
                            }
                        }
                        my_3 += 1;
                    }
                }
                c2rust_current_block = 14127502640287082657;
            }
            _ => {
                c2rust_current_block = 14127502640287082657;
            }
        }
        match c2rust_current_block {
            3583450109353833130 => {
                let mut pix_base_0 = p_fref_w.offset(bmx as isize).offset((bmy * stride) as isize);
                (*h).pixf.fpelcmp_x3[i_pixel as usize].expect("non-null function pointer")(
                    p_fenc,
                    pix_base_0.offset(-2isize).offset((0i32 * stride) as isize),
                    pix_base_0.offset(-1isize).offset((2i32 * stride) as isize),
                    pix_base_0.offset(1isize).offset((2i32 * stride) as isize),
                    stride as crate::stdlib::intptr_t,
                    &raw mut costs as *mut ::core::ffi::c_int,
                );
                costs[0usize] += *p_cost_mvx.offset(((bmx + -(2i32)) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((bmy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                costs[1usize] += *p_cost_mvx.offset(((bmx + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((bmy + 2i32) * 4i32) as isize) as ::core::ffi::c_int;
                costs[2usize] += *p_cost_mvx.offset(((bmx + 1i32) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((bmy + 2i32) * 4i32) as isize) as ::core::ffi::c_int;
                let mut pix_base_1 = p_fref_w.offset(bmx as isize).offset((bmy * stride) as isize);
                (*h).pixf.fpelcmp_x3[i_pixel as usize].expect("non-null function pointer")(
                    p_fenc,
                    pix_base_1.offset(2isize).offset((0i32 * stride) as isize),
                    pix_base_1.offset(1isize).offset((-(2i32) * stride) as isize),
                    pix_base_1.offset(-1isize).offset((-(2i32) * stride) as isize),
                    stride as crate::stdlib::intptr_t,
                    (&raw mut costs as *mut ::core::ffi::c_int).offset(4isize),
                );
                *(&raw mut costs as *mut ::core::ffi::c_int).offset(4isize).offset(0isize) +=
                    *p_cost_mvx.offset(((bmx + 2i32) * 4i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(((bmy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                *(&raw mut costs as *mut ::core::ffi::c_int).offset(4isize).offset(1isize) +=
                    *p_cost_mvx.offset(((bmx + 1i32) * 4i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(((bmy + -(2i32)) * 4i32) as isize) as ::core::ffi::c_int;
                *(&raw mut costs as *mut ::core::ffi::c_int).offset(4isize).offset(2isize) +=
                    *p_cost_mvx.offset(((bmx + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(((bmy + -(2i32)) * 4i32) as isize) as ::core::ffi::c_int;
                bcost <<= 3i32;
                if ((costs[0usize] << 3i32) + 2i32) < bcost {
                    bcost = (costs[0usize] << 3i32) + 2i32;
                }
                if ((costs[1usize] << 3i32) + 3i32) < bcost {
                    bcost = (costs[1usize] << 3i32) + 3i32;
                }
                if ((costs[2usize] << 3i32) + 4i32) < bcost {
                    bcost = (costs[2usize] << 3i32) + 4i32;
                }
                if ((costs[4usize] << 3i32) + 5i32) < bcost {
                    bcost = (costs[4usize] << 3i32) + 5i32;
                }
                if ((costs[5usize] << 3i32) + 6i32) < bcost {
                    bcost = (costs[5usize] << 3i32) + 6i32;
                }
                if ((costs[6usize] << 3i32) + 7i32) < bcost {
                    bcost = (costs[6usize] << 3i32) + 7i32;
                }
                if bcost & 7i32 != 0 {
                    let mut dir = (bcost & 7i32) - 2i32;
                    bmx += hex2[(dir + 1i32) as usize][0usize] as ::core::ffi::c_int;
                    bmy += hex2[(dir + 1i32) as usize][1usize] as ::core::ffi::c_int;
                    let mut i_2 = (i_me_range >> 1i32) - 1i32;
                    while i_2 > 0i32
                        && (((bmx as crate::stdlib::uint32_t) << 16i32
                            | bmy as crate::stdlib::uint32_t & 0x7FFFu32)
                            .wrapping_add(mv_min)
                            | mv_max.wrapping_sub(
                                (bmx as crate::stdlib::uint32_t) << 16i32
                                    | bmy as crate::stdlib::uint32_t & 0x7FFFu32,
                            ))
                            & 0x80004000u32
                            == 0
                    {
                        let mut pix_base_2 = p_fref_w.offset(bmx as isize).offset((bmy * stride) as isize);
                        (*h).pixf.fpelcmp_x3[i_pixel as usize].expect("non-null function pointer")(
                            p_fenc,
                            pix_base_2
                                .offset(hex2[(dir + 0i32) as usize][0usize] as ::core::ffi::c_int as isize)
                                .offset(
                                    (hex2[(dir + 0i32) as usize][1usize] as ::core::ffi::c_int * stride)
                                        as isize,
                                ),
                            pix_base_2
                                .offset(hex2[(dir + 1i32) as usize][0usize] as ::core::ffi::c_int as isize)
                                .offset(
                                    (hex2[(dir + 1i32) as usize][1usize] as ::core::ffi::c_int * stride)
                                        as isize,
                                ),
                            pix_base_2
                                .offset(hex2[(dir + 2i32) as usize][0usize] as ::core::ffi::c_int as isize)
                                .offset(
                                    (hex2[(dir + 2i32) as usize][1usize] as ::core::ffi::c_int * stride)
                                        as isize,
                                ),
                            stride as crate::stdlib::intptr_t,
                            &raw mut costs as *mut ::core::ffi::c_int,
                        );
                        costs[0usize] += *p_cost_mvx.offset(
                            ((bmx + hex2[(dir + 0i32) as usize][0usize] as ::core::ffi::c_int) * 4i32)
                                as isize,
                        ) as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((bmy + hex2[(dir + 0i32) as usize][1usize] as ::core::ffi::c_int) * 4i32)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        costs[1usize] += *p_cost_mvx.offset(
                            ((bmx + hex2[(dir + 1i32) as usize][0usize] as ::core::ffi::c_int) * 4i32)
                                as isize,
                        ) as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((bmy + hex2[(dir + 1i32) as usize][1usize] as ::core::ffi::c_int) * 4i32)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        costs[2usize] += *p_cost_mvx.offset(
                            ((bmx + hex2[(dir + 2i32) as usize][0usize] as ::core::ffi::c_int) * 4i32)
                                as isize,
                        ) as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((bmy + hex2[(dir + 2i32) as usize][1usize] as ::core::ffi::c_int) * 4i32)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        bcost &= !(7i32);
                        if ((costs[0usize] << 3i32) + 1i32) < bcost {
                            bcost = (costs[0usize] << 3i32) + 1i32;
                        }
                        if ((costs[1usize] << 3i32) + 2i32) < bcost {
                            bcost = (costs[1usize] << 3i32) + 2i32;
                        }
                        if ((costs[2usize] << 3i32) + 3i32) < bcost {
                            bcost = (costs[2usize] << 3i32) + 3i32;
                        }
                        if bcost & 7i32 == 0 {
                            break;
                        }
                        dir += (bcost & 7i32) - 2i32;
                        dir = mod6m1[(dir + 1i32) as usize] as ::core::ffi::c_int;
                        bmx += hex2[(dir + 1i32) as usize][0usize] as ::core::ffi::c_int;
                        bmy += hex2[(dir + 1i32) as usize][1usize] as ::core::ffi::c_int;
                        i_2 -= 1;
                    }
                }
                bcost >>= 3i32;
                bcost <<= 4i32;
                let mut pix_base_3 = p_fref_w.offset(bmx as isize).offset((bmy * stride) as isize);
                (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                    p_fenc,
                    pix_base_3.offset(0isize).offset((-(1i32) * stride) as isize),
                    pix_base_3.offset(0isize).offset((1i32 * stride) as isize),
                    pix_base_3.offset(-1isize).offset((0i32 * stride) as isize),
                    pix_base_3.offset(1isize).offset((0i32 * stride) as isize),
                    stride as crate::stdlib::intptr_t,
                    &raw mut costs as *mut ::core::ffi::c_int,
                );
                costs[0usize] += *p_cost_mvx.offset(((bmx + 0i32) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((bmy + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int;
                costs[1usize] += *p_cost_mvx.offset(((bmx + 0i32) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((bmy + 1i32) * 4i32) as isize) as ::core::ffi::c_int;
                costs[2usize] += *p_cost_mvx.offset(((bmx + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((bmy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                costs[3usize] += *p_cost_mvx.offset(((bmx + 1i32) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((bmy + 0i32) * 4i32) as isize) as ::core::ffi::c_int;
                if ((costs[0usize] << 4i32) + 1i32) < bcost {
                    bcost = (costs[0usize] << 4i32) + 1i32;
                }
                if ((costs[1usize] << 4i32) + 2i32) < bcost {
                    bcost = (costs[1usize] << 4i32) + 2i32;
                }
                if ((costs[2usize] << 4i32) + 3i32) < bcost {
                    bcost = (costs[2usize] << 4i32) + 3i32;
                }
                if ((costs[3usize] << 4i32) + 4i32) < bcost {
                    bcost = (costs[3usize] << 4i32) + 4i32;
                }
                let mut pix_base_4 = p_fref_w.offset(bmx as isize).offset((bmy * stride) as isize);
                (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                    p_fenc,
                    pix_base_4.offset(-1isize).offset((-(1i32) * stride) as isize),
                    pix_base_4.offset(-1isize).offset((1i32 * stride) as isize),
                    pix_base_4.offset(1isize).offset((-(1i32) * stride) as isize),
                    pix_base_4.offset(1isize).offset((1i32 * stride) as isize),
                    stride as crate::stdlib::intptr_t,
                    &raw mut costs as *mut ::core::ffi::c_int,
                );
                costs[0usize] += *p_cost_mvx.offset(((bmx + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((bmy + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int;
                costs[1usize] += *p_cost_mvx.offset(((bmx + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((bmy + 1i32) * 4i32) as isize) as ::core::ffi::c_int;
                costs[2usize] += *p_cost_mvx.offset(((bmx + 1i32) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((bmy + -(1i32)) * 4i32) as isize) as ::core::ffi::c_int;
                costs[3usize] += *p_cost_mvx.offset(((bmx + 1i32) * 4i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(((bmy + 1i32) * 4i32) as isize) as ::core::ffi::c_int;
                if ((costs[0usize] << 4i32) + 5i32) < bcost {
                    bcost = (costs[0usize] << 4i32) + 5i32;
                }
                if ((costs[1usize] << 4i32) + 6i32) < bcost {
                    bcost = (costs[1usize] << 4i32) + 6i32;
                }
                if ((costs[2usize] << 4i32) + 7i32) < bcost {
                    bcost = (costs[2usize] << 4i32) + 7i32;
                }
                if ((costs[3usize] << 4i32) + 8i32) < bcost {
                    bcost = (costs[3usize] << 4i32) + 8i32;
                }
                bmx += square1[(bcost & 15i32) as usize][0usize] as ::core::ffi::c_int;
                bmy += square1[(bcost & 15i32) as usize][1usize] as ::core::ffi::c_int;
                bcost >>= 4i32;
            }
            _ => {}
        }
        let mut bmv = pack16to32_mask(bmx, bmy);
        let mut bmv_spel = bmv.wrapping_mul(4u32) & 0xFFFCFFFCu32;
        if (*h).mb.i_subpel_refine < 3i32 {
            (*m).cost_mv = *p_cost_mvx.offset((bmx * 4i32) as isize) as ::core::ffi::c_int
                + *p_cost_mvy.offset((bmy * 4i32) as isize) as ::core::ffi::c_int;
            (*m).cost = bcost;
            if bmv == pmv {
                (*m).cost += (*m).cost_mv;
            }
            (*(&raw mut (*m).mv as *mut crate::src::common::base::x264_union32_t)).i = bmv_spel;
        } else {
            (*(&raw mut (*m).mv as *mut crate::src::common::base::x264_union32_t)).i =
                if bpred_cost < bcost { bpred_mv } else { bmv_spel };
            (*m).cost = if bpred_cost < bcost { bpred_cost } else { bcost };
        }
        if (*h).mb.i_subpel_refine >= 2i32 {
            let mut hpel = subpel_iterations[(*h).mb.i_subpel_refine as usize][2usize] as ::core::ffi::c_int;
            let mut qpel = subpel_iterations[(*h).mb.i_subpel_refine as usize][3usize] as ::core::ffi::c_int;
            refine_subpel(h, m, hpel, qpel, p_halfpel_thresh, 0i32);
        }
    }
}
pub unsafe extern "C" fn x264_8_me_refine_qpel(
    mut h: *mut crate::src::common::common::x264_t,
    mut m: *mut crate::src::encoder::me::x264_me_t,
) {
    unsafe {
        let mut hpel = subpel_iterations[(*h).mb.i_subpel_refine as usize][0usize] as ::core::ffi::c_int;
        let mut qpel = subpel_iterations[(*h).mb.i_subpel_refine as usize][1usize] as ::core::ffi::c_int;
        if (*m).i_pixel <= crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int {
            (*m).cost -= (*m).i_ref_cost;
        }
        refine_subpel(h, m, hpel, qpel, ::core::ptr::null_mut::<::core::ffi::c_int>(), 1i32);
    }
}
pub unsafe extern "C" fn x264_8_me_refine_qpel_refdupe(
    mut h: *mut crate::src::common::common::x264_t,
    mut m: *mut crate::src::encoder::me::x264_me_t,
    mut p_halfpel_thresh: *mut ::core::ffi::c_int,
) {
    unsafe {
        refine_subpel(
            h,
            m,
            0i32,
            if (2i32) < subpel_iterations[(*h).mb.i_subpel_refine as usize][3usize] as ::core::ffi::c_int {
                2i32
            } else {
                subpel_iterations[(*h).mb.i_subpel_refine as usize][3usize] as ::core::ffi::c_int
            },
            p_halfpel_thresh,
            0i32,
        );
    }
}
unsafe extern "C" fn refine_subpel(
    mut h: *mut crate::src::common::common::x264_t,
    mut m: *mut crate::src::encoder::me::x264_me_t,
    mut hpel_iters: ::core::ffi::c_int,
    mut qpel_iters: ::core::ffi::c_int,
    mut p_halfpel_thresh: *mut ::core::ffi::c_int,
    mut b_refine_qpel: ::core::ffi::c_int,
) {
    unsafe {
        let mut pix = [0; 1152];
        let mut costs = [0; 4];
        let mut bdir = 0;
        let bw = x264_pixel_size[(*m).i_pixel as usize].w as ::core::ffi::c_int;
        let bh = x264_pixel_size[(*m).i_pixel as usize].h as ::core::ffi::c_int;
        let mut p_cost_mvx = (*m).p_cost_mv.offset(-((*m).mvp[0usize] as ::core::ffi::c_int as isize));
        let mut p_cost_mvy = (*m).p_cost_mv.offset(-((*m).mvp[1usize] as ::core::ffi::c_int as isize));
        let i_pixel = (*m).i_pixel;
        let b_chroma_me = ((*h).mb.chroma_me
            && (i_pixel <= crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int
                || (*h).sps.i_chroma_format_idc.is_444())) as ::core::ffi::c_int;
        let mut chromapix = (*h).luma2chroma_pixel[i_pixel as usize] as ::core::ffi::c_int;
        let mut chroma_v_shift = ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
        let mut mvy_offset = if chroma_v_shift & (*h).mb.interlaced as ::core::ffi::c_int & (*m).i_ref != 0 {
            ((*h).mb.i_mb_y & 1i32) * 4i32 - 2i32
        } else {
            0i32
        };
        let mut bmx = (*m).mv[0usize] as ::core::ffi::c_int;
        let mut bmy = (*m).mv[1usize] as ::core::ffi::c_int;
        let mut bcost = (*m).cost;
        let mut odir = -(1i32);
        if hpel_iters != 0 {
            if (*h).mb.i_subpel_refine < 3i32 {
                let mut mx = x264_clip3(
                    (*m).mvp[0usize] as ::core::ffi::c_int,
                    (*h).mb.mv_min_spel[0usize] + 2i32,
                    (*h).mb.mv_max_spel[0usize] - 2i32,
                );
                let mut my = x264_clip3(
                    (*m).mvp[1usize] as ::core::ffi::c_int,
                    (*h).mb.mv_min_spel[1usize] + 2i32,
                    (*h).mb.mv_max_spel[1usize] - 2i32,
                );
                if (mx - bmx) | (my - bmy) != 0 {
                    let mut stride = 16isize;
                    let mut src = (*h).mc.get_ref.expect("non-null function pointer")(
                        &raw mut pix as *mut crate::src::common::common::pixel,
                        &raw mut stride,
                        &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                        (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                        mx,
                        my,
                        bw,
                        bh,
                        (*m).weight.offset(0isize),
                    );
                    let mut cost = (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                        (*m).p_fenc[0usize],
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        src,
                        stride,
                    ) + *p_cost_mvx.offset(mx as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(my as isize) as ::core::ffi::c_int;
                    if cost < bcost {
                        bcost = cost;
                        bmx = mx;
                        bmy = my;
                    }
                }
            }
            bcost <<= 6i32;
            let mut i = hpel_iters;
            while i > 0i32 {
                let mut stride_0 = 64isize;
                let mut omx = bmx;
                let mut omy = bmy;
                let mut src0 = (*h).mc.get_ref.expect("non-null function pointer")(
                    &raw mut pix as *mut crate::src::common::common::pixel,
                    &raw mut stride_0,
                    &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                    (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                    omx,
                    omy - 2i32,
                    bw,
                    bh + 1i32,
                    (*m).weight.offset(0isize),
                );
                let mut src2 = (*h).mc.get_ref.expect("non-null function pointer")(
                    (&raw mut pix as *mut crate::src::common::common::pixel).offset(32isize),
                    &raw mut stride_0,
                    &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                    (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                    omx - 2i32,
                    omy,
                    bw + 4i32,
                    bh,
                    (*m).weight.offset(0isize),
                );
                let mut src1 = src0.offset(stride_0);
                let mut src3 = src2.offset(1isize);
                (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                    (*m).p_fenc[0usize],
                    src0,
                    src1,
                    src2,
                    src3,
                    stride_0,
                    &raw mut costs as *mut ::core::ffi::c_int,
                );
                costs[0usize] += *p_cost_mvx.offset(omx as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset((omy - 2i32) as isize) as ::core::ffi::c_int;
                costs[1usize] += *p_cost_mvx.offset(omx as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset((omy + 2i32) as isize) as ::core::ffi::c_int;
                costs[2usize] += *p_cost_mvx.offset((omx - 2i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(omy as isize) as ::core::ffi::c_int;
                costs[3usize] += *p_cost_mvx.offset((omx + 2i32) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(omy as isize) as ::core::ffi::c_int;
                if ((costs[0usize] << 6i32) + 2i32) < bcost {
                    bcost = (costs[0usize] << 6i32) + 2i32;
                }
                if ((costs[1usize] << 6i32) + 6i32) < bcost {
                    bcost = (costs[1usize] << 6i32) + 6i32;
                }
                if ((costs[2usize] << 6i32) + 16i32) < bcost {
                    bcost = (costs[2usize] << 6i32) + 16i32;
                }
                if ((costs[3usize] << 6i32) + 48i32) < bcost {
                    bcost = (costs[3usize] << 6i32) + 48i32;
                }
                if bcost & 63i32 == 0 {
                    break;
                }
                bmx -= ((bcost as crate::stdlib::uint32_t) << 26i32) as crate::stdlib::int32_t >> 29i32;
                bmy -= ((bcost as crate::stdlib::uint32_t) << 29i32) as crate::stdlib::int32_t >> 29i32;
                bcost &= !(63i32);
                i -= 1;
            }
            bcost >>= 6i32;
        }
        #[allow(unpredictable_function_pointer_comparisons)]
        if b_refine_qpel == 0
            && ((*h).pixf.mbcmp_unaligned[0usize] != (*h).pixf.fpelcmp[0usize] || b_chroma_me != 0)
        {
            bcost = crate::src::encoder::me::COST_MAX;
            if b_refine_qpel != 0 || -(1i32) ^ 1i32 != odir {
                let mut stride_1 = 16isize;
                let mut src_0 = (*h).mc.get_ref.expect("non-null function pointer")(
                    &raw mut pix as *mut crate::src::common::common::pixel,
                    &raw mut stride_1,
                    (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(0isize),
                    (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                    bmx,
                    bmy,
                    bw,
                    bh,
                    (*m).weight.offset(0isize),
                );
                let mut cost_0 = (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                    .expect("non-null function pointer")(
                    (*m).p_fenc[0usize],
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                    src_0,
                    stride_1,
                ) + *p_cost_mvx.offset(bmx as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(bmy as isize) as ::core::ffi::c_int;
                if b_chroma_me != 0 && cost_0 < bcost {
                    if (*h).sps.i_chroma_format_idc.is_444() {
                        stride_1 = 16isize;
                        src_0 = (*h).mc.get_ref.expect("non-null function pointer")(
                            &raw mut pix as *mut crate::src::common::common::pixel,
                            &raw mut stride_1,
                            (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                .offset(4isize),
                            (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                            bmx,
                            bmy,
                            bw,
                            bh,
                            (*m).weight.offset(1isize),
                        );
                        cost_0 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1usize],
                            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                            src_0,
                            stride_1,
                        );
                        if cost_0 < bcost {
                            stride_1 = 16isize;
                            src_0 = (*h).mc.get_ref.expect("non-null function pointer")(
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                &raw mut stride_1,
                                (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                    .offset(8isize),
                                (*m).i_stride[2usize] as crate::stdlib::intptr_t,
                                bmx,
                                bmy,
                                bw,
                                bh,
                                (*m).weight.offset(2isize),
                            );
                            cost_0 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2usize],
                                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                src_0,
                                stride_1,
                            );
                        }
                    } else {
                        (*h).mc.mc_chroma.expect("non-null function pointer")(
                            &raw mut pix as *mut crate::src::common::common::pixel,
                            (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                            16isize,
                            (*m).p_fref[4usize],
                            (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                            bmx,
                            (2i32 * (bmy + mvy_offset)) >> chroma_v_shift,
                            bw >> 1i32,
                            bh >> chroma_v_shift,
                        );
                        if !(*(*m).weight.offset(1isize)).weightfn.is_null() {
                            (*(*(*m).weight.offset(1isize)).weightfn.offset((bw >> 3i32) as isize))
                                .expect("non-null function pointer")(
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                16isize,
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                16isize,
                                (*m).weight.offset(1isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        cost_0 += (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
                            (*m).p_fenc[1usize],
                            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                            &raw mut pix as *mut crate::src::common::common::pixel,
                            16isize,
                        );
                        if cost_0 < bcost {
                            if !(*(*m).weight.offset(2isize)).weightfn.is_null() {
                                (*(*(*m).weight.offset(2isize)).weightfn.offset((bw >> 3i32) as isize))
                                    .expect("non-null function pointer")(
                                    (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                                    16isize,
                                    (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                                    16isize,
                                    (*m).weight.offset(2isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_0 += (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
                                (*m).p_fenc[2usize],
                                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                                16isize,
                            );
                        }
                    }
                }
                if cost_0 < bcost {
                    bcost = cost_0;
                    bmx = bmx;
                    bmy = bmy;
                    bdir = -(1i32);
                }
            }
        }
        if !p_halfpel_thresh.is_null() {
            if (bcost * 7i32) >> 3i32 > *p_halfpel_thresh {
                (*m).cost = bcost;
                (*m).mv[0usize] = bmx as crate::stdlib::int16_t;
                (*m).mv[1usize] = bmy as crate::stdlib::int16_t;
                return;
            } else if bcost < *p_halfpel_thresh {
                *p_halfpel_thresh = bcost;
            }
        }
        if (*h).mb.i_subpel_refine != 1i32 {
            bdir = -(1i32);
            let mut i_0 = qpel_iters;
            while i_0 > 0i32 {
                if bmy <= (*h).mb.mv_min_spel[1usize]
                    || bmy >= (*h).mb.mv_max_spel[1usize]
                    || bmx <= (*h).mb.mv_min_spel[0usize]
                    || bmx >= (*h).mb.mv_max_spel[0usize]
                {
                    break;
                }
                odir = bdir;
                let mut omx_0 = bmx;
                let mut omy_0 = bmy;
                if b_refine_qpel != 0 || 0i32 ^ 1i32 != odir {
                    let mut stride_2 = 16isize;
                    let mut src_1 = (*h).mc.get_ref.expect("non-null function pointer")(
                        &raw mut pix as *mut crate::src::common::common::pixel,
                        &raw mut stride_2,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(0isize),
                        (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                        omx_0,
                        omy_0 - 1i32,
                        bw,
                        bh,
                        (*m).weight.offset(0isize),
                    );
                    let mut cost_1 = (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                        .expect("non-null function pointer")(
                        (*m).p_fenc[0usize],
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        src_1,
                        stride_2,
                    ) + *p_cost_mvx.offset(omx_0 as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset((omy_0 - 1i32) as isize) as ::core::ffi::c_int;
                    if b_chroma_me != 0 && cost_1 < bcost {
                        if (*h).sps.i_chroma_format_idc.is_444() {
                            stride_2 = 16isize;
                            src_1 = (*h).mc.get_ref.expect("non-null function pointer")(
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                &raw mut stride_2,
                                (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                    .offset(4isize),
                                (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                omx_0,
                                omy_0 - 1i32,
                                bw,
                                bh,
                                (*m).weight.offset(1isize),
                            );
                            cost_1 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[1usize],
                                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                src_1,
                                stride_2,
                            );
                            if cost_1 < bcost {
                                stride_2 = 16isize;
                                src_1 = (*h).mc.get_ref.expect("non-null function pointer")(
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    &raw mut stride_2,
                                    (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                        .offset(8isize),
                                    (*m).i_stride[2usize] as crate::stdlib::intptr_t,
                                    omx_0,
                                    omy_0 - 1i32,
                                    bw,
                                    bh,
                                    (*m).weight.offset(2isize),
                                );
                                cost_1 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    (*m).p_fenc[2usize],
                                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                    src_1,
                                    stride_2,
                                );
                            }
                        } else {
                            (*h).mc.mc_chroma.expect("non-null function pointer")(
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                                16isize,
                                (*m).p_fref[4usize],
                                (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                omx_0,
                                (2i32 * (omy_0 - 1i32 + mvy_offset)) >> chroma_v_shift,
                                bw >> 1i32,
                                bh >> chroma_v_shift,
                            );
                            if !(*(*m).weight.offset(1isize)).weightfn.is_null() {
                                (*(*(*m).weight.offset(1isize)).weightfn.offset((bw >> 3i32) as isize))
                                    .expect("non-null function pointer")(
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    16isize,
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    16isize,
                                    (*m).weight.offset(1isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_1 += (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
                                (*m).p_fenc[1usize],
                                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                16isize,
                            );
                            if cost_1 < bcost {
                                if !(*(*m).weight.offset(2isize)).weightfn.is_null() {
                                    (*(*(*m).weight.offset(2isize)).weightfn.offset((bw >> 3i32) as isize))
                                        .expect("non-null function pointer")(
                                        (&raw mut pix as *mut crate::src::common::common::pixel)
                                            .offset(8isize),
                                        16isize,
                                        (&raw mut pix as *mut crate::src::common::common::pixel)
                                            .offset(8isize),
                                        16isize,
                                        (*m).weight.offset(2isize),
                                        bh >> chroma_v_shift,
                                    );
                                }
                                cost_1 += (*h).pixf.mbcmp[chromapix as usize]
                                    .expect("non-null function pointer")(
                                    (*m).p_fenc[2usize],
                                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                    (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                                    16isize,
                                );
                            }
                        }
                    }
                    if cost_1 < bcost {
                        bcost = cost_1;
                        bmx = omx_0;
                        bmy = omy_0 - 1i32;
                        bdir = 0i32;
                    }
                }
                if b_refine_qpel != 0 || 1i32 ^ 1i32 != odir {
                    let mut stride_3 = 16isize;
                    let mut src_2 = (*h).mc.get_ref.expect("non-null function pointer")(
                        &raw mut pix as *mut crate::src::common::common::pixel,
                        &raw mut stride_3,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(0isize),
                        (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                        omx_0,
                        omy_0 + 1i32,
                        bw,
                        bh,
                        (*m).weight.offset(0isize),
                    );
                    let mut cost_2 = (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                        .expect("non-null function pointer")(
                        (*m).p_fenc[0usize],
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        src_2,
                        stride_3,
                    ) + *p_cost_mvx.offset(omx_0 as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset((omy_0 + 1i32) as isize) as ::core::ffi::c_int;
                    if b_chroma_me != 0 && cost_2 < bcost {
                        if (*h).sps.i_chroma_format_idc.is_444() {
                            stride_3 = 16isize;
                            src_2 = (*h).mc.get_ref.expect("non-null function pointer")(
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                &raw mut stride_3,
                                (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                    .offset(4isize),
                                (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                omx_0,
                                omy_0 + 1i32,
                                bw,
                                bh,
                                (*m).weight.offset(1isize),
                            );
                            cost_2 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[1usize],
                                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                src_2,
                                stride_3,
                            );
                            if cost_2 < bcost {
                                stride_3 = 16isize;
                                src_2 = (*h).mc.get_ref.expect("non-null function pointer")(
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    &raw mut stride_3,
                                    (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                        .offset(8isize),
                                    (*m).i_stride[2usize] as crate::stdlib::intptr_t,
                                    omx_0,
                                    omy_0 + 1i32,
                                    bw,
                                    bh,
                                    (*m).weight.offset(2isize),
                                );
                                cost_2 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    (*m).p_fenc[2usize],
                                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                    src_2,
                                    stride_3,
                                );
                            }
                        } else {
                            (*h).mc.mc_chroma.expect("non-null function pointer")(
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                                16isize,
                                (*m).p_fref[4usize],
                                (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                omx_0,
                                (2i32 * (omy_0 + 1i32 + mvy_offset)) >> chroma_v_shift,
                                bw >> 1i32,
                                bh >> chroma_v_shift,
                            );
                            if !(*(*m).weight.offset(1isize)).weightfn.is_null() {
                                (*(*(*m).weight.offset(1isize)).weightfn.offset((bw >> 3i32) as isize))
                                    .expect("non-null function pointer")(
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    16isize,
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    16isize,
                                    (*m).weight.offset(1isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_2 += (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
                                (*m).p_fenc[1usize],
                                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                16isize,
                            );
                            if cost_2 < bcost {
                                if !(*(*m).weight.offset(2isize)).weightfn.is_null() {
                                    (*(*(*m).weight.offset(2isize)).weightfn.offset((bw >> 3i32) as isize))
                                        .expect("non-null function pointer")(
                                        (&raw mut pix as *mut crate::src::common::common::pixel)
                                            .offset(8isize),
                                        16isize,
                                        (&raw mut pix as *mut crate::src::common::common::pixel)
                                            .offset(8isize),
                                        16isize,
                                        (*m).weight.offset(2isize),
                                        bh >> chroma_v_shift,
                                    );
                                }
                                cost_2 += (*h).pixf.mbcmp[chromapix as usize]
                                    .expect("non-null function pointer")(
                                    (*m).p_fenc[2usize],
                                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                    (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                                    16isize,
                                );
                            }
                        }
                    }
                    if cost_2 < bcost {
                        bcost = cost_2;
                        bmx = omx_0;
                        bmy = omy_0 + 1i32;
                        bdir = 1i32;
                    }
                }
                if b_refine_qpel != 0 || 2i32 ^ 1i32 != odir {
                    let mut stride_4 = 16isize;
                    let mut src_3 = (*h).mc.get_ref.expect("non-null function pointer")(
                        &raw mut pix as *mut crate::src::common::common::pixel,
                        &raw mut stride_4,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(0isize),
                        (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                        omx_0 - 1i32,
                        omy_0,
                        bw,
                        bh,
                        (*m).weight.offset(0isize),
                    );
                    let mut cost_3 = (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                        .expect("non-null function pointer")(
                        (*m).p_fenc[0usize],
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        src_3,
                        stride_4,
                    ) + *p_cost_mvx.offset((omx_0 - 1i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(omy_0 as isize) as ::core::ffi::c_int;
                    if b_chroma_me != 0 && cost_3 < bcost {
                        if (*h).sps.i_chroma_format_idc.is_444() {
                            stride_4 = 16isize;
                            src_3 = (*h).mc.get_ref.expect("non-null function pointer")(
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                &raw mut stride_4,
                                (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                    .offset(4isize),
                                (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                omx_0 - 1i32,
                                omy_0,
                                bw,
                                bh,
                                (*m).weight.offset(1isize),
                            );
                            cost_3 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[1usize],
                                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                src_3,
                                stride_4,
                            );
                            if cost_3 < bcost {
                                stride_4 = 16isize;
                                src_3 = (*h).mc.get_ref.expect("non-null function pointer")(
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    &raw mut stride_4,
                                    (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                        .offset(8isize),
                                    (*m).i_stride[2usize] as crate::stdlib::intptr_t,
                                    omx_0 - 1i32,
                                    omy_0,
                                    bw,
                                    bh,
                                    (*m).weight.offset(2isize),
                                );
                                cost_3 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    (*m).p_fenc[2usize],
                                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                    src_3,
                                    stride_4,
                                );
                            }
                        } else {
                            (*h).mc.mc_chroma.expect("non-null function pointer")(
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                                16isize,
                                (*m).p_fref[4usize],
                                (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                omx_0 - 1i32,
                                (2i32 * (omy_0 + mvy_offset)) >> chroma_v_shift,
                                bw >> 1i32,
                                bh >> chroma_v_shift,
                            );
                            if !(*(*m).weight.offset(1isize)).weightfn.is_null() {
                                (*(*(*m).weight.offset(1isize)).weightfn.offset((bw >> 3i32) as isize))
                                    .expect("non-null function pointer")(
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    16isize,
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    16isize,
                                    (*m).weight.offset(1isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_3 += (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
                                (*m).p_fenc[1usize],
                                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                16isize,
                            );
                            if cost_3 < bcost {
                                if !(*(*m).weight.offset(2isize)).weightfn.is_null() {
                                    (*(*(*m).weight.offset(2isize)).weightfn.offset((bw >> 3i32) as isize))
                                        .expect("non-null function pointer")(
                                        (&raw mut pix as *mut crate::src::common::common::pixel)
                                            .offset(8isize),
                                        16isize,
                                        (&raw mut pix as *mut crate::src::common::common::pixel)
                                            .offset(8isize),
                                        16isize,
                                        (*m).weight.offset(2isize),
                                        bh >> chroma_v_shift,
                                    );
                                }
                                cost_3 += (*h).pixf.mbcmp[chromapix as usize]
                                    .expect("non-null function pointer")(
                                    (*m).p_fenc[2usize],
                                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                    (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                                    16isize,
                                );
                            }
                        }
                    }
                    if cost_3 < bcost {
                        bcost = cost_3;
                        bmx = omx_0 - 1i32;
                        bmy = omy_0;
                        bdir = 2i32;
                    }
                }
                if b_refine_qpel != 0 || 3i32 ^ 1i32 != odir {
                    let mut stride_5 = 16isize;
                    let mut src_4 = (*h).mc.get_ref.expect("non-null function pointer")(
                        &raw mut pix as *mut crate::src::common::common::pixel,
                        &raw mut stride_5,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(0isize),
                        (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                        omx_0 + 1i32,
                        omy_0,
                        bw,
                        bh,
                        (*m).weight.offset(0isize),
                    );
                    let mut cost_4 = (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                        .expect("non-null function pointer")(
                        (*m).p_fenc[0usize],
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        src_4,
                        stride_5,
                    ) + *p_cost_mvx.offset((omx_0 + 1i32) as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(omy_0 as isize) as ::core::ffi::c_int;
                    if b_chroma_me != 0 && cost_4 < bcost {
                        if (*h).sps.i_chroma_format_idc.is_444() {
                            stride_5 = 16isize;
                            src_4 = (*h).mc.get_ref.expect("non-null function pointer")(
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                &raw mut stride_5,
                                (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                    .offset(4isize),
                                (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                omx_0 + 1i32,
                                omy_0,
                                bw,
                                bh,
                                (*m).weight.offset(1isize),
                            );
                            cost_4 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[1usize],
                                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                src_4,
                                stride_5,
                            );
                            if cost_4 < bcost {
                                stride_5 = 16isize;
                                src_4 = (*h).mc.get_ref.expect("non-null function pointer")(
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    &raw mut stride_5,
                                    (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                        .offset(8isize),
                                    (*m).i_stride[2usize] as crate::stdlib::intptr_t,
                                    omx_0 + 1i32,
                                    omy_0,
                                    bw,
                                    bh,
                                    (*m).weight.offset(2isize),
                                );
                                cost_4 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    (*m).p_fenc[2usize],
                                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                    src_4,
                                    stride_5,
                                );
                            }
                        } else {
                            (*h).mc.mc_chroma.expect("non-null function pointer")(
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                                16isize,
                                (*m).p_fref[4usize],
                                (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                omx_0 + 1i32,
                                (2i32 * (omy_0 + mvy_offset)) >> chroma_v_shift,
                                bw >> 1i32,
                                bh >> chroma_v_shift,
                            );
                            if !(*(*m).weight.offset(1isize)).weightfn.is_null() {
                                (*(*(*m).weight.offset(1isize)).weightfn.offset((bw >> 3i32) as isize))
                                    .expect("non-null function pointer")(
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    16isize,
                                    &raw mut pix as *mut crate::src::common::common::pixel,
                                    16isize,
                                    (*m).weight.offset(1isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_4 += (*h).pixf.mbcmp[chromapix as usize].expect("non-null function pointer")(
                                (*m).p_fenc[1usize],
                                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                &raw mut pix as *mut crate::src::common::common::pixel,
                                16isize,
                            );
                            if cost_4 < bcost {
                                if !(*(*m).weight.offset(2isize)).weightfn.is_null() {
                                    (*(*(*m).weight.offset(2isize)).weightfn.offset((bw >> 3i32) as isize))
                                        .expect("non-null function pointer")(
                                        (&raw mut pix as *mut crate::src::common::common::pixel)
                                            .offset(8isize),
                                        16isize,
                                        (&raw mut pix as *mut crate::src::common::common::pixel)
                                            .offset(8isize),
                                        16isize,
                                        (*m).weight.offset(2isize),
                                        bh >> chroma_v_shift,
                                    );
                                }
                                cost_4 += (*h).pixf.mbcmp[chromapix as usize]
                                    .expect("non-null function pointer")(
                                    (*m).p_fenc[2usize],
                                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                                    (&raw mut pix as *mut crate::src::common::common::pixel).offset(8isize),
                                    16isize,
                                );
                            }
                        }
                    }
                    if cost_4 < bcost {
                        bcost = cost_4;
                        bmx = omx_0 + 1i32;
                        bmy = omy_0;
                        bdir = 3i32;
                    }
                }
                if (bmx == omx_0) as ::core::ffi::c_int & (bmy == omy_0) as ::core::ffi::c_int != 0 {
                    break;
                }
                i_0 -= 1;
            }
        } else if bmy > (*h).mb.mv_min_spel[1usize]
            && bmy < (*h).mb.mv_max_spel[1usize]
            && bmx > (*h).mb.mv_min_spel[0usize]
            && bmx < (*h).mb.mv_max_spel[0usize]
        {
            let mut omx_1 = bmx;
            let mut omy_1 = bmy;
            (*h).mc.mc_luma.expect("non-null function pointer")(
                &raw mut pix as *mut crate::src::common::common::pixel,
                64isize,
                &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                omx_1,
                omy_1 - 1i32,
                bw,
                bh,
                (*m).weight.offset(0isize),
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (&raw mut pix as *mut crate::src::common::common::pixel).offset(16isize),
                64isize,
                &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                omx_1,
                omy_1 + 1i32,
                bw,
                bh,
                (*m).weight.offset(0isize),
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (&raw mut pix as *mut crate::src::common::common::pixel).offset(32isize),
                64isize,
                &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                omx_1 - 1i32,
                omy_1,
                bw,
                bh,
                (*m).weight.offset(0isize),
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (&raw mut pix as *mut crate::src::common::common::pixel).offset(48isize),
                64isize,
                &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                omx_1 + 1i32,
                omy_1,
                bw,
                bh,
                (*m).weight.offset(0isize),
            );
            (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                (*m).p_fenc[0usize],
                &raw mut pix as *mut crate::src::common::common::pixel,
                (&raw mut pix as *mut crate::src::common::common::pixel).offset(16isize),
                (&raw mut pix as *mut crate::src::common::common::pixel).offset(32isize),
                (&raw mut pix as *mut crate::src::common::common::pixel).offset(48isize),
                64isize,
                &raw mut costs as *mut ::core::ffi::c_int,
            );
            costs[0usize] += *p_cost_mvx.offset(omx_1 as isize) as ::core::ffi::c_int
                + *p_cost_mvy.offset((omy_1 - 1i32) as isize) as ::core::ffi::c_int;
            costs[1usize] += *p_cost_mvx.offset(omx_1 as isize) as ::core::ffi::c_int
                + *p_cost_mvy.offset((omy_1 + 1i32) as isize) as ::core::ffi::c_int;
            costs[2usize] += *p_cost_mvx.offset((omx_1 - 1i32) as isize) as ::core::ffi::c_int
                + *p_cost_mvy.offset(omy_1 as isize) as ::core::ffi::c_int;
            costs[3usize] += *p_cost_mvx.offset((omx_1 + 1i32) as isize) as ::core::ffi::c_int
                + *p_cost_mvy.offset(omy_1 as isize) as ::core::ffi::c_int;
            bcost <<= 4i32;
            if ((costs[0usize] << 4i32) + 1i32) < bcost {
                bcost = (costs[0usize] << 4i32) + 1i32;
            }
            if ((costs[1usize] << 4i32) + 3i32) < bcost {
                bcost = (costs[1usize] << 4i32) + 3i32;
            }
            if ((costs[2usize] << 4i32) + 4i32) < bcost {
                bcost = (costs[2usize] << 4i32) + 4i32;
            }
            if ((costs[3usize] << 4i32) + 12i32) < bcost {
                bcost = (costs[3usize] << 4i32) + 12i32;
            }
            bmx -= ((bcost as crate::stdlib::uint32_t) << 28i32) as crate::stdlib::int32_t >> 30i32;
            bmy -= ((bcost as crate::stdlib::uint32_t) << 30i32) as crate::stdlib::int32_t >> 30i32;
            bcost >>= 4i32;
        }
        (*m).cost = bcost;
        (*m).mv[0usize] = bmx as crate::stdlib::int16_t;
        (*m).mv[1usize] = bmy as crate::stdlib::int16_t;
        (*m).cost_mv = *p_cost_mvx.offset(bmx as isize) as ::core::ffi::c_int
            + *p_cost_mvy.offset(bmy as isize) as ::core::ffi::c_int;
    }
}
pub static mut x264_8_iter_kludge: ::core::ffi::c_int = 0i32;
#[inline(always)]
unsafe extern "C" fn me_refine_bidir(
    mut h: *mut crate::src::common::common::x264_t,
    mut m0: *mut crate::src::encoder::me::x264_me_t,
    mut m1: *mut crate::src::encoder::me::x264_me_t,
    mut i_weight: ::core::ffi::c_int,
    mut i8: ::core::ffi::c_int,
    mut i_lambda2: ::core::ffi::c_int,
    mut rd: ::core::ffi::c_int,
) {
    unsafe {
        let mut visited = [[[0; 8]; 8]; 8];
        let mut pass = 0i32;
        let mut x = i8 & 1i32;
        let mut y = i8 >> 1i32;
        let mut s8 = crate::src::common::base::X264_SCAN8_0 + 2i32 * x + 16i32 * y;
        let mut cache0_mv = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
            as *mut [[crate::stdlib::int16_t; 2]; 40])
            .offset(0isize) as *mut [crate::stdlib::int16_t; 2])
            .offset(s8 as isize) as *mut crate::stdlib::int16_t;
        let mut cache1_mv = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
            as *mut [[crate::stdlib::int16_t; 2]; 40])
            .offset(1isize) as *mut [crate::stdlib::int16_t; 2])
            .offset(s8 as isize) as *mut crate::stdlib::int16_t;
        let i_pixel = (*m0).i_pixel;
        let bw = x264_pixel_size[i_pixel as usize].w as ::core::ffi::c_int;
        let bh = x264_pixel_size[i_pixel as usize].h as ::core::ffi::c_int;
        let mut chromapix = (*h).luma2chroma_pixel[i_pixel as usize] as ::core::ffi::c_int;
        let mut chroma_v_shift = ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
        let mut chroma_x = (8i32
            >> ((*h).sps.i_chroma_format_idc.is_420() || (*h).sps.i_chroma_format_idc.is_422())
                as ::core::ffi::c_int)
            * x;
        let mut chroma_y = (8i32 >> chroma_v_shift) * y;
        let mut pix = (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
            .offset(0isize))
        .offset((8i32 * x + 8i32 * y * crate::src::common::common::FDEC_STRIDE) as isize);
        let mut pixu = if !(*h).sps.i_chroma_format_idc.is_400() {
            (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel).offset(1isize))
                .offset((chroma_x + chroma_y * crate::src::common::common::FDEC_STRIDE) as isize)
        } else {
            ::core::ptr::null_mut::<crate::src::common::common::pixel>()
        };
        let mut pixv = if !(*h).sps.i_chroma_format_idc.is_400() {
            (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel).offset(2isize))
                .offset((chroma_x + chroma_y * crate::src::common::common::FDEC_STRIDE) as isize)
        } else {
            ::core::ptr::null_mut::<crate::src::common::common::pixel>()
        };
        let mut ref0 = (*h).mb.cache.ref_0[0usize][s8 as usize] as ::core::ffi::c_int;
        let mut ref1 = (*h).mb.cache.ref_0[1usize][s8 as usize] as ::core::ffi::c_int;
        let mv0y_offset = if chroma_v_shift & (*h).mb.interlaced as ::core::ffi::c_int & ref0 != 0 {
            ((*h).mb.i_mb_y & 1i32) * 4i32 - 2i32
        } else {
            0i32
        };
        let mv1y_offset = if chroma_v_shift & (*h).mb.interlaced as ::core::ffi::c_int & ref1 != 0 {
            ((*h).mb.i_mb_y & 1i32) * 4i32 - 2i32
        } else {
            0i32
        };
        let mut bm0x = (*m0).mv[0usize] as ::core::ffi::c_int;
        let mut bm0y = (*m0).mv[1usize] as ::core::ffi::c_int;
        let mut bm1x = (*m1).mv[0usize] as ::core::ffi::c_int;
        let mut bm1y = (*m1).mv[1usize] as ::core::ffi::c_int;
        let mut bcost = crate::src::encoder::me::COST_MAX;
        let mut bcostrd = crate::src::encoder::me::COST_MAX64;
        static mut dia4d: [[crate::stdlib::int8_t; 4]; 33] = [
            [0i8, 0i8, 0i8, 0i8],
            [0i8, 0i8, 0i8, 1i8],
            [0i8, 0i8, 0i8, -1i8],
            [0i8, 0i8, 1i8, 0i8],
            [0i8, 0i8, -1i8, 0i8],
            [0i8, 1i8, 0i8, 0i8],
            [0i8, -1i8, 0i8, 0i8],
            [1i8, 0i8, 0i8, 0i8],
            [-1i8, 0i8, 0i8, 0i8],
            [0i8, 0i8, 1i8, 1i8],
            [0i8, 0i8, -1i8, -1i8],
            [0i8, 1i8, 1i8, 0i8],
            [0i8, -1i8, -1i8, 0i8],
            [1i8, 1i8, 0i8, 0i8],
            [-1i8, -1i8, 0i8, 0i8],
            [1i8, 0i8, 0i8, 1i8],
            [-1i8, 0i8, 0i8, -1i8],
            [0i8, 1i8, 0i8, 1i8],
            [0i8, -1i8, 0i8, -1i8],
            [1i8, 0i8, 1i8, 0i8],
            [-1i8, 0i8, -1i8, 0i8],
            [0i8, 0i8, -1i8, 1i8],
            [0i8, 0i8, 1i8, -1i8],
            [0i8, -1i8, 1i8, 0i8],
            [0i8, 1i8, -1i8, 0i8],
            [-1i8, 1i8, 0i8, 0i8],
            [1i8, -1i8, 0i8, 0i8],
            [1i8, 0i8, 0i8, -1i8],
            [-1i8, 0i8, 0i8, 1i8],
            [0i8, -1i8, 0i8, 1i8],
            [0i8, 1i8, 0i8, -1i8],
            [-1i8, 0i8, 1i8, 0i8],
            [1i8, 0i8, -1i8, 0i8],
        ];
        if bm0y < (*h).mb.mv_min_spel[1usize] + 8i32
            || bm1y < (*h).mb.mv_min_spel[1usize] + 8i32
            || bm0y > (*h).mb.mv_max_spel[1usize] - 8i32
            || bm1y > (*h).mb.mv_max_spel[1usize] - 8i32
            || bm0x < (*h).mb.mv_min_spel[0usize] + 8i32
            || bm1x < (*h).mb.mv_min_spel[0usize] + 8i32
            || bm0x > (*h).mb.mv_max_spel[0usize] - 8i32
            || bm1x > (*h).mb.mv_max_spel[0usize] - 8i32
        {
            return;
        }
        if rd != 0
            && (*m0).i_pixel != crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int
            && i8 != 0i32
        {
            crate::src::common::mvpred::x264_8_mb_predict_mv(
                h,
                0i32,
                i8 << 2i32,
                bw >> 2i32,
                &raw mut (*m0).mvp as *mut crate::stdlib::int16_t,
            );
            crate::src::common::mvpred::x264_8_mb_predict_mv(
                h,
                1i32,
                i8 << 2i32,
                bw >> 2i32,
                &raw mut (*m1).mvp as *mut crate::stdlib::int16_t,
            );
        }
        let mut p_cost_m0x = (*m0).p_cost_mv.offset(-((*m0).mvp[0usize] as ::core::ffi::c_int as isize));
        let mut p_cost_m0y = (*m0).p_cost_mv.offset(-((*m0).mvp[1usize] as ::core::ffi::c_int as isize));
        let mut p_cost_m1x = (*m1).p_cost_mv.offset(-((*m1).mvp[0usize] as ::core::ffi::c_int as isize));
        let mut p_cost_m1y = (*m1).p_cost_mv.offset(-((*m1).mvp[1usize] as ::core::ffi::c_int as isize));
        (*h).mc.memzero_aligned.expect("non-null function pointer")(
            &raw mut visited as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[[[crate::stdlib::uint8_t; 8]; 8]; 8]>(),
        );
        while pass < 8i32 {
            let mut pixy_buf = [[[0; 256]; 9]; 2];
            let mut pixu_buf = [[[0; 256]; 9]; 2];
            let mut pixv_buf = [[[0; 256]; 9]; 2];
            let mut src = [[[::core::ptr::null_mut::<crate::src::common::common::pixel>(); 9]; 2]; 3];
            let mut stride = [[[0; 9]; 2]; 3];
            let mut mc_list0 = 1i32;
            let mut mc_list1 = 1i32;
            let mut bestj = 0i32;
            if mc_list0 != 0 {
                let mut j = x264_8_iter_kludge;
                while j < 9i32 {
                    let mut m = m0;
                    let mut i = 4i32
                        + 3i32 * square1[j as usize][0usize] as ::core::ffi::c_int
                        + square1[j as usize][1usize] as ::core::ffi::c_int;
                    let mut mvx = bm0x + square1[j as usize][0usize] as ::core::ffi::c_int;
                    let mut mvy = bm0y + square1[j as usize][1usize] as ::core::ffi::c_int;
                    stride[0usize][0usize][i as usize] = bw as crate::stdlib::intptr_t;
                    src[0usize][0usize][i as usize] = (*h).mc.get_ref.expect("non-null function pointer")(
                        &raw mut *(&raw mut *(&raw mut pixy_buf
                            as *mut [[crate::src::common::common::pixel; 256]; 9])
                            .offset(0isize)
                            as *mut [crate::src::common::common::pixel; 256])
                            .offset(i as isize)
                            as *mut crate::src::common::common::pixel,
                        (&raw mut *(&raw mut *(&raw mut stride as *mut [[crate::stdlib::intptr_t; 9]; 2])
                            .offset(0isize)
                            as *mut [crate::stdlib::intptr_t; 9])
                            .offset(0isize) as *mut crate::stdlib::intptr_t)
                            .offset(i as isize),
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(0isize),
                        (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                        mvx,
                        mvy,
                        bw,
                        bh,
                        &raw mut crate::src::common::tables::x264_zero
                            as *const crate::src::common::mc::x264_weight_t,
                    );
                    if rd != 0 {
                        if (*h).sps.i_chroma_format_idc.is_444() {
                            stride[1usize][0usize][i as usize] = bw as crate::stdlib::intptr_t;
                            src[1usize][0usize][i as usize] =
                                (*h).mc.get_ref.expect("non-null function pointer")(
                                    &raw mut *(&raw mut *(&raw mut pixu_buf
                                        as *mut [[crate::src::common::common::pixel; 256]; 9])
                                        .offset(0isize)
                                        as *mut [crate::src::common::common::pixel; 256])
                                        .offset(i as isize)
                                        as *mut crate::src::common::common::pixel,
                                    (&raw mut *(&raw mut *(&raw mut stride
                                        as *mut [[crate::stdlib::intptr_t; 9]; 2])
                                        .offset(1isize)
                                        as *mut [crate::stdlib::intptr_t; 9])
                                        .offset(0isize)
                                        as *mut crate::stdlib::intptr_t)
                                        .offset(i as isize),
                                    (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                        .offset(4isize),
                                    (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                    mvx,
                                    mvy,
                                    bw,
                                    bh,
                                    &raw mut crate::src::common::tables::x264_zero
                                        as *const crate::src::common::mc::x264_weight_t,
                                );
                            stride[2usize][0usize][i as usize] = bw as crate::stdlib::intptr_t;
                            src[2usize][0usize][i as usize] =
                                (*h).mc.get_ref.expect("non-null function pointer")(
                                    &raw mut *(&raw mut *(&raw mut pixv_buf
                                        as *mut [[crate::src::common::common::pixel; 256]; 9])
                                        .offset(0isize)
                                        as *mut [crate::src::common::common::pixel; 256])
                                        .offset(i as isize)
                                        as *mut crate::src::common::common::pixel,
                                    (&raw mut *(&raw mut *(&raw mut stride
                                        as *mut [[crate::stdlib::intptr_t; 9]; 2])
                                        .offset(2isize)
                                        as *mut [crate::stdlib::intptr_t; 9])
                                        .offset(0isize)
                                        as *mut crate::stdlib::intptr_t)
                                        .offset(i as isize),
                                    (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                        .offset(8isize),
                                    (*m).i_stride[2usize] as crate::stdlib::intptr_t,
                                    mvx,
                                    mvy,
                                    bw,
                                    bh,
                                    &raw mut crate::src::common::tables::x264_zero
                                        as *const crate::src::common::mc::x264_weight_t,
                                );
                        } else if !(*h).sps.i_chroma_format_idc.is_400() {
                            (*h).mc.mc_chroma.expect("non-null function pointer")(
                                &raw mut *(&raw mut *(&raw mut pixu_buf
                                    as *mut [[crate::src::common::common::pixel; 256]; 9])
                                    .offset(0isize)
                                    as *mut [crate::src::common::common::pixel; 256])
                                    .offset(i as isize)
                                    as *mut crate::src::common::common::pixel,
                                &raw mut *(&raw mut *(&raw mut pixv_buf
                                    as *mut [[crate::src::common::common::pixel; 256]; 9])
                                    .offset(0isize)
                                    as *mut [crate::src::common::common::pixel; 256])
                                    .offset(i as isize)
                                    as *mut crate::src::common::common::pixel,
                                8isize,
                                (*m).p_fref[4usize],
                                (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                mvx,
                                (2i32 * (mvy + mv0y_offset)) >> chroma_v_shift,
                                bw >> 1i32,
                                bh >> chroma_v_shift,
                            );
                        }
                    }
                    j += 1;
                }
            }
            if mc_list1 != 0 {
                let mut j_0 = x264_8_iter_kludge;
                while j_0 < 9i32 {
                    let mut m_0 = m1;
                    let mut i_0 = 4i32
                        + 3i32 * square1[j_0 as usize][0usize] as ::core::ffi::c_int
                        + square1[j_0 as usize][1usize] as ::core::ffi::c_int;
                    let mut mvx_0 = bm1x + square1[j_0 as usize][0usize] as ::core::ffi::c_int;
                    let mut mvy_0 = bm1y + square1[j_0 as usize][1usize] as ::core::ffi::c_int;
                    stride[0usize][1usize][i_0 as usize] = bw as crate::stdlib::intptr_t;
                    src[0usize][1usize][i_0 as usize] = (*h).mc.get_ref.expect("non-null function pointer")(
                        &raw mut *(&raw mut *(&raw mut pixy_buf
                            as *mut [[crate::src::common::common::pixel; 256]; 9])
                            .offset(1isize)
                            as *mut [crate::src::common::common::pixel; 256])
                            .offset(i_0 as isize)
                            as *mut crate::src::common::common::pixel,
                        (&raw mut *(&raw mut *(&raw mut stride as *mut [[crate::stdlib::intptr_t; 9]; 2])
                            .offset(0isize)
                            as *mut [crate::stdlib::intptr_t; 9])
                            .offset(1isize) as *mut crate::stdlib::intptr_t)
                            .offset(i_0 as isize),
                        (&raw mut (*m_0).p_fref as *mut *mut crate::src::common::common::pixel)
                            .offset(0isize),
                        (*m_0).i_stride[0usize] as crate::stdlib::intptr_t,
                        mvx_0,
                        mvy_0,
                        bw,
                        bh,
                        &raw mut crate::src::common::tables::x264_zero
                            as *const crate::src::common::mc::x264_weight_t,
                    );
                    if rd != 0 {
                        if (*h).sps.i_chroma_format_idc.is_444() {
                            stride[1usize][1usize][i_0 as usize] = bw as crate::stdlib::intptr_t;
                            src[1usize][1usize][i_0 as usize] =
                                (*h).mc.get_ref.expect("non-null function pointer")(
                                    &raw mut *(&raw mut *(&raw mut pixu_buf
                                        as *mut [[crate::src::common::common::pixel; 256]; 9])
                                        .offset(1isize)
                                        as *mut [crate::src::common::common::pixel; 256])
                                        .offset(i_0 as isize)
                                        as *mut crate::src::common::common::pixel,
                                    (&raw mut *(&raw mut *(&raw mut stride
                                        as *mut [[crate::stdlib::intptr_t; 9]; 2])
                                        .offset(1isize)
                                        as *mut [crate::stdlib::intptr_t; 9])
                                        .offset(1isize)
                                        as *mut crate::stdlib::intptr_t)
                                        .offset(i_0 as isize),
                                    (&raw mut (*m_0).p_fref as *mut *mut crate::src::common::common::pixel)
                                        .offset(4isize),
                                    (*m_0).i_stride[1usize] as crate::stdlib::intptr_t,
                                    mvx_0,
                                    mvy_0,
                                    bw,
                                    bh,
                                    &raw mut crate::src::common::tables::x264_zero
                                        as *const crate::src::common::mc::x264_weight_t,
                                );
                            stride[2usize][1usize][i_0 as usize] = bw as crate::stdlib::intptr_t;
                            src[2usize][1usize][i_0 as usize] =
                                (*h).mc.get_ref.expect("non-null function pointer")(
                                    &raw mut *(&raw mut *(&raw mut pixv_buf
                                        as *mut [[crate::src::common::common::pixel; 256]; 9])
                                        .offset(1isize)
                                        as *mut [crate::src::common::common::pixel; 256])
                                        .offset(i_0 as isize)
                                        as *mut crate::src::common::common::pixel,
                                    (&raw mut *(&raw mut *(&raw mut stride
                                        as *mut [[crate::stdlib::intptr_t; 9]; 2])
                                        .offset(2isize)
                                        as *mut [crate::stdlib::intptr_t; 9])
                                        .offset(1isize)
                                        as *mut crate::stdlib::intptr_t)
                                        .offset(i_0 as isize),
                                    (&raw mut (*m_0).p_fref as *mut *mut crate::src::common::common::pixel)
                                        .offset(8isize),
                                    (*m_0).i_stride[2usize] as crate::stdlib::intptr_t,
                                    mvx_0,
                                    mvy_0,
                                    bw,
                                    bh,
                                    &raw mut crate::src::common::tables::x264_zero
                                        as *const crate::src::common::mc::x264_weight_t,
                                );
                        } else if !(*h).sps.i_chroma_format_idc.is_400() {
                            (*h).mc.mc_chroma.expect("non-null function pointer")(
                                &raw mut *(&raw mut *(&raw mut pixu_buf
                                    as *mut [[crate::src::common::common::pixel; 256]; 9])
                                    .offset(1isize)
                                    as *mut [crate::src::common::common::pixel; 256])
                                    .offset(i_0 as isize)
                                    as *mut crate::src::common::common::pixel,
                                &raw mut *(&raw mut *(&raw mut pixv_buf
                                    as *mut [[crate::src::common::common::pixel; 256]; 9])
                                    .offset(1isize)
                                    as *mut [crate::src::common::common::pixel; 256])
                                    .offset(i_0 as isize)
                                    as *mut crate::src::common::common::pixel,
                                8isize,
                                (*m_0).p_fref[4usize],
                                (*m_0).i_stride[1usize] as crate::stdlib::intptr_t,
                                mvx_0,
                                (2i32 * (mvy_0 + mv1y_offset)) >> chroma_v_shift,
                                bw >> 1i32,
                                bh >> chroma_v_shift,
                            );
                        }
                    }
                    j_0 += 1;
                }
            }
            let mut j_1 = (pass != 0) as ::core::ffi::c_int;
            while j_1 < 33i32 {
                let mut m0x = dia4d[j_1 as usize][0usize] as ::core::ffi::c_int + bm0x;
                let mut m0y = dia4d[j_1 as usize][1usize] as ::core::ffi::c_int + bm0y;
                let mut m1x = dia4d[j_1 as usize][2usize] as ::core::ffi::c_int + bm1x;
                let mut m1y = dia4d[j_1 as usize][3usize] as ::core::ffi::c_int + bm1y;
                if pass == 0
                    || visited[(m0x & 7i32) as usize][(m0y & 7i32) as usize][(m1x & 7i32) as usize]
                        as ::core::ffi::c_int
                        & (1i32) << (m1y & 7i32)
                        == 0
                {
                    let mut i0 = 4i32
                        + 3i32 * dia4d[j_1 as usize][0usize] as ::core::ffi::c_int
                        + dia4d[j_1 as usize][1usize] as ::core::ffi::c_int;
                    let mut i1 = 4i32
                        + 3i32 * dia4d[j_1 as usize][2usize] as ::core::ffi::c_int
                        + dia4d[j_1 as usize][3usize] as ::core::ffi::c_int;
                    visited[(m0x & 7i32) as usize][(m0y & 7i32) as usize][(m1x & 7i32) as usize] =
                        (visited[(m0x & 7i32) as usize][(m0y & 7i32) as usize][(m1x & 7i32) as usize]
                            as ::core::ffi::c_int
                            | (1i32) << (m1y & 7i32)) as crate::stdlib::uint8_t;
                    (*h).mc.avg[i_pixel as usize].expect("non-null function pointer")(
                        pix,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        src[0usize][0usize][i0 as usize],
                        stride[0usize][0usize][i0 as usize],
                        src[0usize][1usize][i1 as usize],
                        stride[0usize][1usize][i1 as usize],
                        i_weight,
                    );
                    let mut cost = (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
                        (*m0).p_fenc[0usize],
                        crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                        pix,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    ) + *p_cost_m0x.offset(m0x as isize) as ::core::ffi::c_int
                        + *p_cost_m0y.offset(m0y as isize) as ::core::ffi::c_int
                        + *p_cost_m1x.offset(m1x as isize) as ::core::ffi::c_int
                        + *p_cost_m1y.offset(m1y as isize) as ::core::ffi::c_int;
                    if rd != 0 {
                        if cost < bcost + (bcost >> 4i32) {
                            bcost = if cost < bcost { cost } else { bcost };
                            (*(cache0_mv as *mut crate::src::common::base::x264_union32_t)).i =
                                pack16to32_mask(m0x, m0y);
                            (*(cache1_mv as *mut crate::src::common::base::x264_union32_t)).i =
                                pack16to32_mask(m1x, m1y);
                            if (*h).sps.i_chroma_format_idc.is_444() {
                                (*h).mc.avg[i_pixel as usize].expect("non-null function pointer")(
                                    pixu,
                                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                                    src[1usize][0usize][i0 as usize],
                                    stride[1usize][0usize][i0 as usize],
                                    src[1usize][1usize][i1 as usize],
                                    stride[1usize][1usize][i1 as usize],
                                    i_weight,
                                );
                                (*h).mc.avg[i_pixel as usize].expect("non-null function pointer")(
                                    pixv,
                                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                                    src[2usize][0usize][i0 as usize],
                                    stride[2usize][0usize][i0 as usize],
                                    src[2usize][1usize][i1 as usize],
                                    stride[2usize][1usize][i1 as usize],
                                    i_weight,
                                );
                            } else if !(*h).sps.i_chroma_format_idc.is_400() {
                                (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
                                    pixu,
                                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *(&raw mut pixu_buf
                                        as *mut [[crate::src::common::common::pixel; 256]; 9])
                                        .offset(0isize)
                                        as *mut [crate::src::common::common::pixel; 256])
                                        .offset(i0 as isize)
                                        as *mut crate::src::common::common::pixel,
                                    8isize,
                                    &raw mut *(&raw mut *(&raw mut pixu_buf
                                        as *mut [[crate::src::common::common::pixel; 256]; 9])
                                        .offset(1isize)
                                        as *mut [crate::src::common::common::pixel; 256])
                                        .offset(i1 as isize)
                                        as *mut crate::src::common::common::pixel,
                                    8isize,
                                    i_weight,
                                );
                                (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
                                    pixv,
                                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *(&raw mut pixv_buf
                                        as *mut [[crate::src::common::common::pixel; 256]; 9])
                                        .offset(0isize)
                                        as *mut [crate::src::common::common::pixel; 256])
                                        .offset(i0 as isize)
                                        as *mut crate::src::common::common::pixel,
                                    8isize,
                                    &raw mut *(&raw mut *(&raw mut pixv_buf
                                        as *mut [[crate::src::common::common::pixel; 256]; 9])
                                        .offset(1isize)
                                        as *mut [crate::src::common::common::pixel; 256])
                                        .offset(i1 as isize)
                                        as *mut crate::src::common::common::pixel,
                                    8isize,
                                    i_weight,
                                );
                            }
                            let mut costrd = crate::src::encoder::analyse::rdo_c::x264_8_rd_cost_part(
                                h,
                                i_lambda2,
                                i8 * 4i32,
                                (*m0).i_pixel,
                            );
                            if costrd < bcostrd {
                                bcostrd = costrd;
                                bestj = j_1;
                            }
                        }
                    } else if cost < bcost {
                        bcost = cost;
                        bestj = j_1;
                    }
                }
                j_1 += 1;
            }
            if bestj == 0 {
                break;
            }
            bm0x += dia4d[bestj as usize][0usize] as ::core::ffi::c_int;
            bm0y += dia4d[bestj as usize][1usize] as ::core::ffi::c_int;
            bm1x += dia4d[bestj as usize][2usize] as ::core::ffi::c_int;
            bm1y += dia4d[bestj as usize][3usize] as ::core::ffi::c_int;
            mc_list0 = (*((&raw const *(&raw const dia4d as *const [crate::stdlib::int8_t; 4])
                .offset(bestj as isize) as *const crate::stdlib::int8_t)
                .offset(0isize) as *mut crate::src::common::base::x264_union16_t))
                .i as ::core::ffi::c_int;
            mc_list1 = (*((&raw const *(&raw const dia4d as *const [crate::stdlib::int8_t; 4])
                .offset(bestj as isize) as *const crate::stdlib::int8_t)
                .offset(2isize) as *mut crate::src::common::base::x264_union16_t))
                .i as ::core::ffi::c_int;
            pass += 1;
        }
        if rd != 0 {
            let mut amvd = 0;
            x264_macroblock_cache_mv(
                h,
                2i32 * x,
                2i32 * y,
                bw >> 2i32,
                bh >> 2i32,
                0i32,
                pack16to32_mask(bm0x, bm0y),
            );
            amvd = pack8to16(
                (if crate::stdlib::abs(bm0x - (*m0).mvp[0usize] as ::core::ffi::c_int) < 33i32 {
                    crate::stdlib::abs(bm0x - (*m0).mvp[0usize] as ::core::ffi::c_int)
                } else {
                    33i32
                }) as crate::stdlib::uint32_t,
                (if crate::stdlib::abs(bm0y - (*m0).mvp[1usize] as ::core::ffi::c_int) < 33i32 {
                    crate::stdlib::abs(bm0y - (*m0).mvp[1usize] as ::core::ffi::c_int)
                } else {
                    33i32
                }) as crate::stdlib::uint32_t,
            ) as crate::stdlib::uint16_t;
            x264_macroblock_cache_mvd(h, 2i32 * x, 2i32 * y, bw >> 2i32, bh >> 2i32, 0i32, amvd);
            x264_macroblock_cache_mv(
                h,
                2i32 * x,
                2i32 * y,
                bw >> 2i32,
                bh >> 2i32,
                1i32,
                pack16to32_mask(bm1x, bm1y),
            );
            amvd = pack8to16(
                (if crate::stdlib::abs(bm1x - (*m1).mvp[0usize] as ::core::ffi::c_int) < 33i32 {
                    crate::stdlib::abs(bm1x - (*m1).mvp[0usize] as ::core::ffi::c_int)
                } else {
                    33i32
                }) as crate::stdlib::uint32_t,
                (if crate::stdlib::abs(bm1y - (*m1).mvp[1usize] as ::core::ffi::c_int) < 33i32 {
                    crate::stdlib::abs(bm1y - (*m1).mvp[1usize] as ::core::ffi::c_int)
                } else {
                    33i32
                }) as crate::stdlib::uint32_t,
            ) as crate::stdlib::uint16_t;
            x264_macroblock_cache_mvd(h, 2i32 * x, 2i32 * y, bw >> 2i32, bh >> 2i32, 1i32, amvd);
        }
        (*m0).mv[0usize] = bm0x as crate::stdlib::int16_t;
        (*m0).mv[1usize] = bm0y as crate::stdlib::int16_t;
        (*m1).mv[0usize] = bm1x as crate::stdlib::int16_t;
        (*m1).mv[1usize] = bm1y as crate::stdlib::int16_t;
    }
}
pub unsafe extern "C" fn x264_8_me_refine_bidir_satd(
    mut h: *mut crate::src::common::common::x264_t,
    mut m0: *mut crate::src::encoder::me::x264_me_t,
    mut m1: *mut crate::src::encoder::me::x264_me_t,
    mut i_weight: ::core::ffi::c_int,
) {
    unsafe {
        me_refine_bidir(h, m0, m1, i_weight, 0i32, 0i32, 0i32);
    }
}
pub unsafe extern "C" fn x264_8_me_refine_bidir_rd(
    mut h: *mut crate::src::common::common::x264_t,
    mut m0: *mut crate::src::encoder::me::x264_me_t,
    mut m1: *mut crate::src::encoder::me::x264_me_t,
    mut i_weight: ::core::ffi::c_int,
    mut i8: ::core::ffi::c_int,
    mut i_lambda2: ::core::ffi::c_int,
) {
    unsafe {
        (*h).mb.skip_mc = true;
        me_refine_bidir(h, m0, m1, i_weight, i8, i_lambda2, 1i32);
        (*h).mb.skip_mc = false;
    }
}
pub unsafe extern "C" fn x264_8_me_refine_qpel_rd(
    mut h: *mut crate::src::common::common::x264_t,
    mut m: *mut crate::src::encoder::me::x264_me_t,
    mut i_lambda2: ::core::ffi::c_int,
    mut i4: ::core::ffi::c_int,
    mut i_list: ::core::ffi::c_int,
) {
    unsafe {
        let mut satd = 0;
        let mut bsatd = 0;
        let mut pixu = ::core::ptr::null_mut::<crate::src::common::common::pixel>();
        let mut pixv = ::core::ptr::null_mut::<crate::src::common::common::pixel>();
        let mut j = 0i32;
        let mut i_0 = 0i32;
        let mut cache_mv = &raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
            as *mut [[crate::stdlib::int16_t; 2]; 40])
            .offset(i_list as isize)
            as *mut [crate::stdlib::int16_t; 2])
            .offset(*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(i4 as isize) as isize)
            as *mut crate::stdlib::int16_t;
        let bw = x264_pixel_size[(*m).i_pixel as usize].w as ::core::ffi::c_int;
        let bh = x264_pixel_size[(*m).i_pixel as usize].h as ::core::ffi::c_int;
        let i_pixel = (*m).i_pixel;
        let mut chroma_v_shift = ((*h).sps.i_chroma_format_idc.is_420()) as ::core::ffi::c_int;
        let mut mvy_offset = if chroma_v_shift & (*h).mb.interlaced as ::core::ffi::c_int & (*m).i_ref != 0 {
            ((*h).mb.i_mb_y & 1i32) * 4i32 - 2i32
        } else {
            0i32
        };
        let mut bcost = crate::src::encoder::me::COST_MAX64;
        let mut bmx = (*m).mv[0usize] as ::core::ffi::c_int;
        let mut bmy = (*m).mv[1usize] as ::core::ffi::c_int;
        let mut dir = -(2i32);
        let mut i8 = i4 >> 2i32;
        let mut pix = (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
            .offset(0isize))
        .offset(
            *(&raw const block_idx_xy_fdec as *const crate::stdlib::uint16_t).offset(i4 as isize) as isize
        );
        if (*h).sps.i_chroma_format_idc.is_444() {
            pixu = (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                .offset(1isize))
            .offset(*(&raw const block_idx_xy_fdec as *const crate::stdlib::uint16_t).offset(i4 as isize)
                as isize);
            pixv = (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                .offset(2isize))
            .offset(*(&raw const block_idx_xy_fdec as *const crate::stdlib::uint16_t).offset(i4 as isize)
                as isize);
        } else if !(*h).sps.i_chroma_format_idc.is_400() {
            pixu = (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                .offset(1isize))
            .offset(
                ((i8 >> 1i32) * ((8i32 * crate::src::common::common::FDEC_STRIDE) >> chroma_v_shift)
                    + (i8 & 1i32) * 4i32) as isize,
            );
            pixv = (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                .offset(2isize))
            .offset(
                ((i8 >> 1i32) * ((8i32 * crate::src::common::common::FDEC_STRIDE) >> chroma_v_shift)
                    + (i8 & 1i32) * 4i32) as isize,
            );
        } else {
            pixu = ::core::ptr::null_mut::<crate::src::common::common::pixel>();
            pixv = ::core::ptr::null_mut::<crate::src::common::common::pixel>();
        }
        (*h).mb.skip_mc = true;
        if (*m).i_pixel != crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int && i4 != 0i32 {
            crate::src::common::mvpred::x264_8_mb_predict_mv(
                h,
                i_list,
                i4,
                bw >> 2i32,
                &raw mut (*m).mvp as *mut crate::stdlib::int16_t,
            );
        }
        let mut pmx = (*m).mvp[0usize] as ::core::ffi::c_int;
        let mut pmy = (*m).mvp[1usize] as ::core::ffi::c_int;
        let mut p_cost_mvx = (*m).p_cost_mv.offset(-(pmx as isize));
        let mut p_cost_mvy = (*m).p_cost_mv.offset(-(pmy as isize));
        if 0i32 == 0 || !(bmx == pmx && bmy == pmy) {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                pix,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                bmx,
                bmy,
                bw,
                bh,
                (*m).weight.offset(0isize),
            );
            bsatd = (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
                (*m).p_fenc[0usize],
                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                pix,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            ) + *p_cost_mvx.offset(bmx as isize) as ::core::ffi::c_int
                + *p_cost_mvy.offset(bmy as isize) as ::core::ffi::c_int;
            if bsatd < bsatd {
                bsatd = bsatd;
            }
        } else {
            bsatd = crate::src::encoder::me::COST_MAX;
        }
        if (*m).i_pixel != crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int {
            if 0i32 <= bsatd + (bsatd >> 4i32) {
                (*(cache_mv as *mut crate::src::common::base::x264_union32_t)).i = pack16to32_mask(bmx, bmy);
                if (*h).sps.i_chroma_format_idc.is_444() {
                    (*h).mc.mc_luma.expect("non-null function pointer")(
                        pixu,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(4isize),
                        (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                        bmx,
                        bmy,
                        bw,
                        bh,
                        (*m).weight.offset(1isize),
                    );
                    (*h).mc.mc_luma.expect("non-null function pointer")(
                        pixv,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(8isize),
                        (*m).i_stride[2usize] as crate::stdlib::intptr_t,
                        bmx,
                        bmy,
                        bw,
                        bh,
                        (*m).weight.offset(2isize),
                    );
                } else if !(*h).sps.i_chroma_format_idc.is_400()
                    && (*m).i_pixel <= crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int
                {
                    (*h).mc.mc_chroma.expect("non-null function pointer")(
                        pixu,
                        pixv,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (*m).p_fref[4usize],
                        (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                        bmx,
                        (2i32 * (bmy + mvy_offset)) >> chroma_v_shift,
                        bw >> 1i32,
                        bh >> chroma_v_shift,
                    );
                    if !(*(*m).weight.offset(1isize)).weightfn.is_null() {
                        (*(*(*m).weight.offset(1isize)).weightfn.offset((bw >> 3i32) as isize))
                            .expect("non-null function pointer")(
                            pixu,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            pixu,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (*m).weight.offset(1isize),
                            bh >> chroma_v_shift,
                        );
                    }
                    if !(*(*m).weight.offset(2isize)).weightfn.is_null() {
                        (*(*(*m).weight.offset(2isize)).weightfn.offset((bw >> 3i32) as isize))
                            .expect("non-null function pointer")(
                            pixv,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            pixv,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (*m).weight.offset(2isize),
                            bh >> chroma_v_shift,
                        );
                    }
                }
                let mut cost =
                    crate::src::encoder::analyse::rdo_c::x264_8_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
                if cost < bcost {
                    bcost = cost;
                    bmx = bmx;
                    bmy = bmy;
                    dir = if 0i32 != 0 { 0i32 } else { dir };
                }
            }
        } else {
            bcost = (*m).cost as crate::stdlib::uint64_t;
        }
        if (bmx != pmx || bmy != pmy)
            && pmx >= (*h).mb.mv_min_spel[0usize]
            && pmx <= (*h).mb.mv_max_spel[0usize]
            && pmy >= (*h).mb.mv_min_spel[1usize]
            && pmy <= (*h).mb.mv_max_spel[1usize]
        {
            if 0i32 == 0 || !(pmx == pmx && pmy == pmy) {
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    pix,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                    (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                    pmx,
                    pmy,
                    bw,
                    bh,
                    (*m).weight.offset(0isize),
                );
                satd = (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
                    (*m).p_fenc[0usize],
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                    pix,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                ) + *p_cost_mvx.offset(pmx as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(pmy as isize) as ::core::ffi::c_int;
                if satd < bsatd {
                    bsatd = satd;
                }
            } else {
                satd = crate::src::encoder::me::COST_MAX;
            }
            if satd <= bsatd + (bsatd >> 4i32) {
                (*(cache_mv as *mut crate::src::common::base::x264_union32_t)).i = pack16to32_mask(pmx, pmy);
                if (*h).sps.i_chroma_format_idc.is_444() {
                    (*h).mc.mc_luma.expect("non-null function pointer")(
                        pixu,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(4isize),
                        (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                        pmx,
                        pmy,
                        bw,
                        bh,
                        (*m).weight.offset(1isize),
                    );
                    (*h).mc.mc_luma.expect("non-null function pointer")(
                        pixv,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(8isize),
                        (*m).i_stride[2usize] as crate::stdlib::intptr_t,
                        pmx,
                        pmy,
                        bw,
                        bh,
                        (*m).weight.offset(2isize),
                    );
                } else if !(*h).sps.i_chroma_format_idc.is_400()
                    && (*m).i_pixel <= crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int
                {
                    (*h).mc.mc_chroma.expect("non-null function pointer")(
                        pixu,
                        pixv,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (*m).p_fref[4usize],
                        (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                        pmx,
                        (2i32 * (pmy + mvy_offset)) >> chroma_v_shift,
                        bw >> 1i32,
                        bh >> chroma_v_shift,
                    );
                    if !(*(*m).weight.offset(1isize)).weightfn.is_null() {
                        (*(*(*m).weight.offset(1isize)).weightfn.offset((bw >> 3i32) as isize))
                            .expect("non-null function pointer")(
                            pixu,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            pixu,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (*m).weight.offset(1isize),
                            bh >> chroma_v_shift,
                        );
                    }
                    if !(*(*m).weight.offset(2isize)).weightfn.is_null() {
                        (*(*(*m).weight.offset(2isize)).weightfn.offset((bw >> 3i32) as isize))
                            .expect("non-null function pointer")(
                            pixv,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            pixv,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (*m).weight.offset(2isize),
                            bh >> chroma_v_shift,
                        );
                    }
                }
                let mut cost_0 =
                    crate::src::encoder::analyse::rdo_c::x264_8_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
                if cost_0 < bcost {
                    bcost = cost_0;
                    bmx = pmx;
                    bmy = pmy;
                    dir = if 0i32 != 0 { 0i32 } else { dir };
                }
            }
            if bmx == pmx && bmy == pmy {
                pmx = (*m).mv[0usize] as ::core::ffi::c_int;
                pmy = (*m).mv[1usize] as ::core::ffi::c_int;
            }
        }
        if bmy < (*h).mb.mv_min_spel[1usize] + 3i32
            || bmy > (*h).mb.mv_max_spel[1usize] - 3i32
            || bmx < (*h).mb.mv_min_spel[0usize] + 3i32
            || bmx > (*h).mb.mv_max_spel[0usize] - 3i32
        {
            (*h).mb.skip_mc = false;
            return;
        }
        dir = -(2i32);
        let mut omx = bmx;
        let mut omy = bmy;
        while j < 6i32 {
            if 1i32 == 0
                || !(omx + hex2[(j + 1i32) as usize][0usize] as ::core::ffi::c_int == pmx
                    && omy + hex2[(j + 1i32) as usize][1usize] as ::core::ffi::c_int == pmy)
            {
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    pix,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                    (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                    omx + hex2[(j + 1i32) as usize][0usize] as ::core::ffi::c_int,
                    omy + hex2[(j + 1i32) as usize][1usize] as ::core::ffi::c_int,
                    bw,
                    bh,
                    (*m).weight.offset(0isize),
                );
                satd = (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
                    (*m).p_fenc[0usize],
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                    pix,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                ) + *p_cost_mvx
                    .offset((omx + hex2[(j + 1i32) as usize][0usize] as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy
                        .offset((omy + hex2[(j + 1i32) as usize][1usize] as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int;
                if satd < bsatd {
                    bsatd = satd;
                }
            } else {
                satd = crate::src::encoder::me::COST_MAX;
            }
            if satd <= bsatd + (bsatd >> 4i32) {
                (*(cache_mv as *mut crate::src::common::base::x264_union32_t)).i = pack16to32_mask(
                    omx + hex2[(j + 1i32) as usize][0usize] as ::core::ffi::c_int,
                    omy + hex2[(j + 1i32) as usize][1usize] as ::core::ffi::c_int,
                );
                if (*h).sps.i_chroma_format_idc.is_444() {
                    (*h).mc.mc_luma.expect("non-null function pointer")(
                        pixu,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(4isize),
                        (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                        omx + hex2[(j + 1i32) as usize][0usize] as ::core::ffi::c_int,
                        omy + hex2[(j + 1i32) as usize][1usize] as ::core::ffi::c_int,
                        bw,
                        bh,
                        (*m).weight.offset(1isize),
                    );
                    (*h).mc.mc_luma.expect("non-null function pointer")(
                        pixv,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(8isize),
                        (*m).i_stride[2usize] as crate::stdlib::intptr_t,
                        omx + hex2[(j + 1i32) as usize][0usize] as ::core::ffi::c_int,
                        omy + hex2[(j + 1i32) as usize][1usize] as ::core::ffi::c_int,
                        bw,
                        bh,
                        (*m).weight.offset(2isize),
                    );
                } else if !(*h).sps.i_chroma_format_idc.is_400()
                    && (*m).i_pixel <= crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int
                {
                    (*h).mc.mc_chroma.expect("non-null function pointer")(
                        pixu,
                        pixv,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (*m).p_fref[4usize],
                        (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                        omx + hex2[(j + 1i32) as usize][0usize] as ::core::ffi::c_int,
                        (2i32 * (omy + hex2[(j + 1i32) as usize][1usize] as ::core::ffi::c_int + mvy_offset))
                            >> chroma_v_shift,
                        bw >> 1i32,
                        bh >> chroma_v_shift,
                    );
                    if !(*(*m).weight.offset(1isize)).weightfn.is_null() {
                        (*(*(*m).weight.offset(1isize)).weightfn.offset((bw >> 3i32) as isize))
                            .expect("non-null function pointer")(
                            pixu,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            pixu,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (*m).weight.offset(1isize),
                            bh >> chroma_v_shift,
                        );
                    }
                    if !(*(*m).weight.offset(2isize)).weightfn.is_null() {
                        (*(*(*m).weight.offset(2isize)).weightfn.offset((bw >> 3i32) as isize))
                            .expect("non-null function pointer")(
                            pixv,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            pixv,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (*m).weight.offset(2isize),
                            bh >> chroma_v_shift,
                        );
                    }
                }
                let mut cost_1 =
                    crate::src::encoder::analyse::rdo_c::x264_8_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
                if cost_1 < bcost {
                    bcost = cost_1;
                    bmx = omx + hex2[(j + 1i32) as usize][0usize] as ::core::ffi::c_int;
                    bmy = omy + hex2[(j + 1i32) as usize][1usize] as ::core::ffi::c_int;
                    dir = if 1i32 != 0 { j } else { dir };
                }
            }
            j += 1;
        }
        if dir != -(2i32) {
            let mut i = 1i32;
            while i < 10i32 {
                let mut j_0 = 0i32;
                let odir = mod6m1[(dir + 1i32) as usize] as ::core::ffi::c_int;
                if bmy < (*h).mb.mv_min_spel[1usize] + 3i32 || bmy > (*h).mb.mv_max_spel[1usize] - 3i32 {
                    break;
                }
                dir = -(2i32);
                omx = bmx;
                omy = bmy;
                while j_0 < 3i32 {
                    if 1i32 == 0
                        || !(omx + hex2[(odir + j_0) as usize][0usize] as ::core::ffi::c_int == pmx
                            && omy + hex2[(odir + j_0) as usize][1usize] as ::core::ffi::c_int == pmy)
                    {
                        (*h).mc.mc_luma.expect("non-null function pointer")(
                            pix,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                            (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                            omx + hex2[(odir + j_0) as usize][0usize] as ::core::ffi::c_int,
                            omy + hex2[(odir + j_0) as usize][1usize] as ::core::ffi::c_int,
                            bw,
                            bh,
                            (*m).weight.offset(0isize),
                        );
                        satd = (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
                            (*m).p_fenc[0usize],
                            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                            pix,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        ) + *p_cost_mvx.offset(
                            (omx + hex2[(odir + j_0) as usize][0usize] as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                (omy + hex2[(odir + j_0) as usize][1usize] as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_int;
                        if satd < bsatd {
                            bsatd = satd;
                        }
                    } else {
                        satd = crate::src::encoder::me::COST_MAX;
                    }
                    if satd <= bsatd + (bsatd >> 4i32) {
                        (*(cache_mv as *mut crate::src::common::base::x264_union32_t)).i = pack16to32_mask(
                            omx + hex2[(odir + j_0) as usize][0usize] as ::core::ffi::c_int,
                            omy + hex2[(odir + j_0) as usize][1usize] as ::core::ffi::c_int,
                        );
                        if (*h).sps.i_chroma_format_idc.is_444() {
                            (*h).mc.mc_luma.expect("non-null function pointer")(
                                pixu,
                                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                                (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                    .offset(4isize),
                                (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                omx + hex2[(odir + j_0) as usize][0usize] as ::core::ffi::c_int,
                                omy + hex2[(odir + j_0) as usize][1usize] as ::core::ffi::c_int,
                                bw,
                                bh,
                                (*m).weight.offset(1isize),
                            );
                            (*h).mc.mc_luma.expect("non-null function pointer")(
                                pixv,
                                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                                (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel)
                                    .offset(8isize),
                                (*m).i_stride[2usize] as crate::stdlib::intptr_t,
                                omx + hex2[(odir + j_0) as usize][0usize] as ::core::ffi::c_int,
                                omy + hex2[(odir + j_0) as usize][1usize] as ::core::ffi::c_int,
                                bw,
                                bh,
                                (*m).weight.offset(2isize),
                            );
                        } else if !(*h).sps.i_chroma_format_idc.is_400()
                            && (*m).i_pixel <= crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int
                        {
                            (*h).mc.mc_chroma.expect("non-null function pointer")(
                                pixu,
                                pixv,
                                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                                (*m).p_fref[4usize],
                                (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                                omx + hex2[(odir + j_0) as usize][0usize] as ::core::ffi::c_int,
                                (2i32
                                    * (omy
                                        + hex2[(odir + j_0) as usize][1usize] as ::core::ffi::c_int
                                        + mvy_offset))
                                    >> chroma_v_shift,
                                bw >> 1i32,
                                bh >> chroma_v_shift,
                            );
                            if !(*(*m).weight.offset(1isize)).weightfn.is_null() {
                                (*(*(*m).weight.offset(1isize)).weightfn.offset((bw >> 3i32) as isize))
                                    .expect("non-null function pointer")(
                                    pixu,
                                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                                    pixu,
                                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                                    (*m).weight.offset(1isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            if !(*(*m).weight.offset(2isize)).weightfn.is_null() {
                                (*(*(*m).weight.offset(2isize)).weightfn.offset((bw >> 3i32) as isize))
                                    .expect("non-null function pointer")(
                                    pixv,
                                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                                    pixv,
                                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                                    (*m).weight.offset(2isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                        }
                        let mut cost_2 = crate::src::encoder::analyse::rdo_c::x264_8_rd_cost_part(
                            h,
                            i_lambda2,
                            i4,
                            (*m).i_pixel,
                        );
                        if cost_2 < bcost {
                            bcost = cost_2;
                            bmx = omx + hex2[(odir + j_0) as usize][0usize] as ::core::ffi::c_int;
                            bmy = omy + hex2[(odir + j_0) as usize][1usize] as ::core::ffi::c_int;
                            dir = if 1i32 != 0 { odir - 1i32 + j_0 } else { dir };
                        }
                    }
                    j_0 += 1;
                }
                if dir == -(2i32) {
                    break;
                }
                i += 1;
            }
        }
        omx = bmx;
        omy = bmy;
        while i_0 < 8i32 {
            if 1i32 == 0
                || !(omx + square1[(i_0 + 1i32) as usize][0usize] as ::core::ffi::c_int == pmx
                    && omy + square1[(i_0 + 1i32) as usize][1usize] as ::core::ffi::c_int == pmy)
            {
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    pix,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel,
                    (*m).i_stride[0usize] as crate::stdlib::intptr_t,
                    omx + square1[(i_0 + 1i32) as usize][0usize] as ::core::ffi::c_int,
                    omy + square1[(i_0 + 1i32) as usize][1usize] as ::core::ffi::c_int,
                    bw,
                    bh,
                    (*m).weight.offset(0isize),
                );
                satd = (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
                    (*m).p_fenc[0usize],
                    crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                    pix,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                ) + *p_cost_mvx
                    .offset((omx + square1[(i_0 + 1i32) as usize][0usize] as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy
                        .offset((omy + square1[(i_0 + 1i32) as usize][1usize] as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int;
                if satd < bsatd {
                    bsatd = satd;
                }
            } else {
                satd = crate::src::encoder::me::COST_MAX;
            }
            if satd <= bsatd + (bsatd >> 4i32) {
                (*(cache_mv as *mut crate::src::common::base::x264_union32_t)).i = pack16to32_mask(
                    omx + square1[(i_0 + 1i32) as usize][0usize] as ::core::ffi::c_int,
                    omy + square1[(i_0 + 1i32) as usize][1usize] as ::core::ffi::c_int,
                );
                if (*h).sps.i_chroma_format_idc.is_444() {
                    (*h).mc.mc_luma.expect("non-null function pointer")(
                        pixu,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(4isize),
                        (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                        omx + square1[(i_0 + 1i32) as usize][0usize] as ::core::ffi::c_int,
                        omy + square1[(i_0 + 1i32) as usize][1usize] as ::core::ffi::c_int,
                        bw,
                        bh,
                        (*m).weight.offset(1isize),
                    );
                    (*h).mc.mc_luma.expect("non-null function pointer")(
                        pixv,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (&raw mut (*m).p_fref as *mut *mut crate::src::common::common::pixel).offset(8isize),
                        (*m).i_stride[2usize] as crate::stdlib::intptr_t,
                        omx + square1[(i_0 + 1i32) as usize][0usize] as ::core::ffi::c_int,
                        omy + square1[(i_0 + 1i32) as usize][1usize] as ::core::ffi::c_int,
                        bw,
                        bh,
                        (*m).weight.offset(2isize),
                    );
                } else if !(*h).sps.i_chroma_format_idc.is_400()
                    && (*m).i_pixel <= crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int
                {
                    (*h).mc.mc_chroma.expect("non-null function pointer")(
                        pixu,
                        pixv,
                        crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                        (*m).p_fref[4usize],
                        (*m).i_stride[1usize] as crate::stdlib::intptr_t,
                        omx + square1[(i_0 + 1i32) as usize][0usize] as ::core::ffi::c_int,
                        (2i32
                            * (omy
                                + square1[(i_0 + 1i32) as usize][1usize] as ::core::ffi::c_int
                                + mvy_offset))
                            >> chroma_v_shift,
                        bw >> 1i32,
                        bh >> chroma_v_shift,
                    );
                    if !(*(*m).weight.offset(1isize)).weightfn.is_null() {
                        (*(*(*m).weight.offset(1isize)).weightfn.offset((bw >> 3i32) as isize))
                            .expect("non-null function pointer")(
                            pixu,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            pixu,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (*m).weight.offset(1isize),
                            bh >> chroma_v_shift,
                        );
                    }
                    if !(*(*m).weight.offset(2isize)).weightfn.is_null() {
                        (*(*(*m).weight.offset(2isize)).weightfn.offset((bw >> 3i32) as isize))
                            .expect("non-null function pointer")(
                            pixv,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            pixv,
                            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                            (*m).weight.offset(2isize),
                            bh >> chroma_v_shift,
                        );
                    }
                }
                let mut cost_3 =
                    crate::src::encoder::analyse::rdo_c::x264_8_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
                if cost_3 < bcost {
                    bcost = cost_3;
                    bmx = omx + square1[(i_0 + 1i32) as usize][0usize] as ::core::ffi::c_int;
                    bmy = omy + square1[(i_0 + 1i32) as usize][1usize] as ::core::ffi::c_int;
                    dir = if 0i32 != 0 { 0i32 } else { dir };
                }
            }
            i_0 += 1;
        }
        (*m).cost = bcost as ::core::ffi::c_int;
        (*m).mv[0usize] = bmx as crate::stdlib::int16_t;
        (*m).mv[1usize] = bmy as crate::stdlib::int16_t;
        x264_macroblock_cache_mv(
            h,
            block_idx_x[i4 as usize] as ::core::ffi::c_int,
            block_idx_y[i4 as usize] as ::core::ffi::c_int,
            bw >> 2i32,
            bh >> 2i32,
            i_list,
            pack16to32_mask(bmx, bmy),
        );
        let mut amvd = pack8to16(
            (if crate::stdlib::abs(bmx - (*m).mvp[0usize] as ::core::ffi::c_int) < 66i32 {
                crate::stdlib::abs(bmx - (*m).mvp[0usize] as ::core::ffi::c_int)
            } else {
                66i32
            }) as crate::stdlib::uint32_t,
            (if crate::stdlib::abs(bmy - (*m).mvp[1usize] as ::core::ffi::c_int) < 66i32 {
                crate::stdlib::abs(bmy - (*m).mvp[1usize] as ::core::ffi::c_int)
            } else {
                66i32
            }) as crate::stdlib::uint32_t,
        ) as crate::stdlib::uint16_t;
        x264_macroblock_cache_mvd(
            h,
            block_idx_x[i4 as usize] as ::core::ffi::c_int,
            block_idx_y[i4 as usize] as ::core::ffi::c_int,
            bw >> 2i32,
            bh >> 2i32,
            i_list,
            amvd,
        );
        (*h).mb.skip_mc = false;
    }
}
