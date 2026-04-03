// =============== BEGIN macroblock_h ================
pub type macroblock_position_e = ::core::ffi::c_uint;
pub const MB_LEFT: crate::src::common::macroblock::macroblock_position_e = 1;
pub const MB_TOP: crate::src::common::macroblock::macroblock_position_e = 2;
pub const MB_TOPRIGHT: crate::src::common::macroblock::macroblock_position_e = 4;
pub const MB_TOPLEFT: crate::src::common::macroblock::macroblock_position_e = 8;
pub const MB_PRIVATE: crate::src::common::macroblock::macroblock_position_e = 16;
pub const ALL_NEIGHBORS: crate::src::common::macroblock::macroblock_position_e = 15;
pub type mb_class_e = ::core::ffi::c_uint;
pub const I_4x4: crate::src::common::macroblock::mb_class_e = 0;
pub const I_8x8: crate::src::common::macroblock::mb_class_e = 1;
pub const I_16x16: crate::src::common::macroblock::mb_class_e = 2;
pub const I_PCM: crate::src::common::macroblock::mb_class_e = 3;
pub const P_L0: crate::src::common::macroblock::mb_class_e = 4;
pub const P_8x8: crate::src::common::macroblock::mb_class_e = 5;
pub const P_SKIP: crate::src::common::macroblock::mb_class_e = 6;
pub const B_DIRECT: crate::src::common::macroblock::mb_class_e = 7;
pub const B_L0_L0: crate::src::common::macroblock::mb_class_e = 8;
pub const B_L0_L1: crate::src::common::macroblock::mb_class_e = 9;
pub const B_L0_BI: crate::src::common::macroblock::mb_class_e = 10;
pub const B_L1_L0: crate::src::common::macroblock::mb_class_e = 11;
pub const B_L1_L1: crate::src::common::macroblock::mb_class_e = 12;
pub const B_L1_BI: crate::src::common::macroblock::mb_class_e = 13;
pub const B_BI_L0: crate::src::common::macroblock::mb_class_e = 14;
pub const B_BI_L1: crate::src::common::macroblock::mb_class_e = 15;
pub const B_BI_BI: crate::src::common::macroblock::mb_class_e = 16;
pub const B_8x8: crate::src::common::macroblock::mb_class_e = 17;
pub const B_SKIP: crate::src::common::macroblock::mb_class_e = 18;
pub const X264_MBTYPE_MAX: crate::src::common::macroblock::mb_class_e = 19;
pub type mb_partition_e = ::core::ffi::c_uint;
pub const D_L0_4x4: crate::src::common::macroblock::mb_partition_e = 0;
pub const D_L0_8x4: crate::src::common::macroblock::mb_partition_e = 1;
pub const D_L0_4x8: crate::src::common::macroblock::mb_partition_e = 2;
pub const D_L0_8x8: crate::src::common::macroblock::mb_partition_e = 3;
pub const D_L1_4x4: crate::src::common::macroblock::mb_partition_e = 4;
pub const D_L1_8x4: crate::src::common::macroblock::mb_partition_e = 5;
pub const D_L1_4x8: crate::src::common::macroblock::mb_partition_e = 6;
pub const D_L1_8x8: crate::src::common::macroblock::mb_partition_e = 7;
pub const D_BI_4x4: crate::src::common::macroblock::mb_partition_e = 8;
pub const D_BI_8x4: crate::src::common::macroblock::mb_partition_e = 9;
pub const D_BI_4x8: crate::src::common::macroblock::mb_partition_e = 10;
pub const D_BI_8x8: crate::src::common::macroblock::mb_partition_e = 11;
pub const D_DIRECT_8x8: crate::src::common::macroblock::mb_partition_e = 12;
pub const D_8x8: crate::src::common::macroblock::mb_partition_e = 13;
pub const D_16x8: crate::src::common::macroblock::mb_partition_e = 14;
pub const D_8x16: crate::src::common::macroblock::mb_partition_e = 15;
pub const D_16x16: crate::src::common::macroblock::mb_partition_e = 16;
pub const X264_PARTTYPE_MAX: crate::src::common::macroblock::mb_partition_e = 17;
pub type cabac_ctx_block_cat_e = ::core::ffi::c_uint;
pub const DCT_LUMA_DC: crate::src::common::macroblock::cabac_ctx_block_cat_e = 0;
pub const DCT_LUMA_AC: crate::src::common::macroblock::cabac_ctx_block_cat_e = 1;
pub const DCT_LUMA_4x4: crate::src::common::macroblock::cabac_ctx_block_cat_e = 2;
pub const DCT_CHROMA_DC: crate::src::common::macroblock::cabac_ctx_block_cat_e = 3;
pub const DCT_CHROMA_AC: crate::src::common::macroblock::cabac_ctx_block_cat_e = 4;
pub const DCT_LUMA_8x8: crate::src::common::macroblock::cabac_ctx_block_cat_e = 5;
pub const DCT_CHROMAU_DC: crate::src::common::macroblock::cabac_ctx_block_cat_e = 6;
pub const DCT_CHROMAU_AC: crate::src::common::macroblock::cabac_ctx_block_cat_e = 7;
pub const DCT_CHROMAU_4x4: crate::src::common::macroblock::cabac_ctx_block_cat_e = 8;
pub const DCT_CHROMAU_8x8: crate::src::common::macroblock::cabac_ctx_block_cat_e = 9;
pub const DCT_CHROMAV_DC: crate::src::common::macroblock::cabac_ctx_block_cat_e = 10;
pub const DCT_CHROMAV_AC: crate::src::common::macroblock::cabac_ctx_block_cat_e = 11;
pub const DCT_CHROMAV_4x4: crate::src::common::macroblock::cabac_ctx_block_cat_e = 12;
pub const DCT_CHROMAV_8x8: crate::src::common::macroblock::cabac_ctx_block_cat_e = 13;
pub mod pixel_h {
    pub static mut x264_size2pixel: [[crate::stdlib::uint8_t; 5]; 5] = [
        [0u8, 0, 0, 0, 0],
        [
            0u8,
            crate::src::common::pixel::PIXEL_4x4 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x4 as crate::stdlib::uint8_t,
            0u8,
            0u8,
        ],
        [
            0u8,
            crate::src::common::pixel::PIXEL_4x8 as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x8 as crate::stdlib::uint8_t,
            0u8,
            crate::src::common::pixel::PIXEL_16x8 as crate::stdlib::uint8_t,
        ],
        [0u8, 0, 0, 0, 0],
        [
            0u8,
            0u8,
            crate::src::common::pixel::PIXEL_8x16 as crate::stdlib::uint8_t,
            0u8,
            crate::src::common::pixel::PIXEL_16x16 as crate::stdlib::uint8_t,
        ],
    ];
}
pub mod predict_h {
    pub static mut x264_mb_chroma_pred_mode_fix: [crate::stdlib::uint8_t; 7] = [
        crate::src::common::predict::I_PRED_CHROMA_DC as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_H as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_V as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_P as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as crate::stdlib::uint8_t,
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
}
pub mod macroblock_h {
    pub static mut x264_mb_type_fix: [crate::stdlib::uint8_t; 19] = [
        crate::src::common::macroblock::I_4x4 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::I_4x4 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::I_16x16 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::I_PCM as crate::stdlib::uint8_t,
        crate::src::common::macroblock::P_L0 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::P_8x8 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::P_SKIP as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_DIRECT as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L0_L0 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L0_L1 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L0_BI as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L1_L0 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L1_L1 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L1_BI as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_BI_L0 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_BI_L1 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_BI_BI as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_8x8 as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_SKIP as crate::stdlib::uint8_t,
    ];
    #[inline(always)]
    pub unsafe extern "C" fn pack16to32(
        mut a: crate::stdlib::uint32_t,
        mut b: crate::stdlib::uint32_t,
    ) -> crate::stdlib::uint32_t {
        return a.wrapping_add(b << 16i32);
    }
    #[inline(always)]
    pub unsafe extern "C" fn pack8to32(
        mut a: crate::stdlib::uint32_t,
        mut b: crate::stdlib::uint32_t,
        mut c: crate::stdlib::uint32_t,
        mut d: crate::stdlib::uint32_t,
    ) -> crate::stdlib::uint32_t {
        return a
            .wrapping_add(b << 8i32)
            .wrapping_add(c << 16i32)
            .wrapping_add(d << 24i32);
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
                            b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
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
                        b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                    );
                };
            };
        }
    }
    #[inline(always)]
    pub unsafe extern "C" fn x264_macroblock_cache_skip(
        mut h: *mut crate::src::common::common::x264_t,
        mut x: ::core::ffi::c_int,
        mut y: ::core::ffi::c_int,
        mut width: ::core::ffi::c_int,
        mut height: ::core::ffi::c_int,
        mut b_skip: ::core::ffi::c_int,
    ) {
        unsafe {
            x264_macroblock_cache_rect(
                (&raw mut (*h).mb.cache.skip as *mut crate::stdlib::int8_t)
                    .offset((crate::src::common::base::X264_SCAN8_0 + x + 8i32 * y) as isize)
                    as *mut ::core::ffi::c_void,
                width,
                height,
                1i32,
                b_skip as crate::stdlib::uint32_t,
            );
        }
    }
}
use crate::src::common::macroblock::base_h::x264_clip3;
use crate::src::common::macroblock::base_h::x264_scan8;
use crate::src::common::macroblock::macroblock_h::pack8to32;
use crate::src::common::macroblock::macroblock_h::pack16to32;
use crate::src::common::macroblock::macroblock_h::x264_mb_type_fix;
use crate::src::common::macroblock::pixel_h::x264_size2pixel;
use crate::src::common::macroblock::predict_h::x264_mb_chroma_pred_mode_fix;
use crate::src::common::macroblock::rectangle_h::x264_macroblock_cache_skip;
#[inline(never)]
unsafe extern "C" fn mb_mc_0xywh(
    mut h: *mut crate::src::common::common::x264_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        let mut i8 = x264_scan8[0usize] as ::core::ffi::c_int + x + 8i32 * y;
        let mut i_ref = (*h).mb.cache.ref_0[0usize][i8 as usize] as ::core::ffi::c_int;
        let mut mvx = x264_clip3(
            (*h).mb.cache.mv[0usize][i8 as usize][0usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[0usize],
            (*h).mb.mv_max[0usize],
        ) + 4i32 * 4i32 * x;
        let mut mvy = x264_clip3(
            (*h).mb.cache.mv[0usize][i8 as usize][1usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[1usize],
            (*h).mb.mv_max[1usize],
        ) + 4i32 * 4i32 * y;
        (*h).mc.mc_luma.expect("non-null function pointer")(
            (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                .offset(0isize))
            .offset((4i32 * y * crate::src::common::common::FDEC_STRIDE + 4i32 * x) as isize),
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                .offset(0isize)
                as *mut [*mut crate::src::common::common::pixel; 12])
                .offset(i_ref as isize)
                as *mut *mut crate::src::common::common::pixel)
                .offset((0i32 * 4i32) as isize),
            (*h).mb.pic.i_stride[0usize] as crate::stdlib::intptr_t,
            mvx,
            mvy,
            4i32 * width,
            4i32 * height,
            if 0i32 != 0 {
                &raw mut crate::src::common::tables::x264_zero
                    as *const crate::src::common::mc::x264_weight_t
            } else {
                (&raw mut *(&raw mut (*h).sh.weight
                    as *mut [crate::src::common::mc::x264_weight_t; 3])
                    .offset(i_ref as isize)
                    as *mut crate::src::common::mc::x264_weight_t)
                    .offset(0isize) as *const crate::src::common::mc::x264_weight_t
            },
        );
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1isize))
                .offset((4i32 * y * crate::src::common::common::FDEC_STRIDE + 4i32 * x) as isize),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(0isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((1i32 * 4i32) as isize),
                (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                mvx,
                mvy,
                4i32 * width,
                4i32 * height,
                if 0i32 != 0 {
                    &raw mut crate::src::common::tables::x264_zero
                        as *const crate::src::common::mc::x264_weight_t
                } else {
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(1isize)
                        as *const crate::src::common::mc::x264_weight_t
                },
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2isize))
                .offset((4i32 * y * crate::src::common::common::FDEC_STRIDE + 4i32 * x) as isize),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(0isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((2i32 * 4i32) as isize),
                (*h).mb.pic.i_stride[2usize] as crate::stdlib::intptr_t,
                mvx,
                mvy,
                4i32 * width,
                4i32 * height,
                if 0i32 != 0 {
                    &raw mut crate::src::common::tables::x264_zero
                        as *const crate::src::common::mc::x264_weight_t
                } else {
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(2isize)
                        as *const crate::src::common::mc::x264_weight_t
                },
            );
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            let mut v_shift = (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if v_shift & (*h).mb.interlaced as ::core::ffi::c_int & i_ref != 0 {
                mvy += ((*h).mb.i_mb_y & 1i32) * 4i32 - 2i32;
            }
            let mut offset =
                (4i32 * crate::src::common::common::FDEC_STRIDE >> v_shift) * y + 2i32 * x;
            height = 4i32 * height >> v_shift;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1isize))
                .offset(offset as isize),
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2isize))
                .offset(offset as isize),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fref[0usize][i_ref as usize][4usize],
                (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                mvx,
                2i32 * mvy >> v_shift,
                2i32 * width,
                height,
            );
            if !(*h).sh.weight[i_ref as usize][1usize].weightfn.is_null() {
                (*(*h).sh.weight[i_ref as usize][1usize]
                    .weightfn
                    .offset((width >> 1i32) as isize))
                .expect("non-null function pointer")(
                    (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                        .offset(1isize))
                    .offset(offset as isize),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                        .offset(1isize))
                    .offset(offset as isize),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(1isize),
                    height,
                );
            }
            if !(*h).sh.weight[i_ref as usize][2usize].weightfn.is_null() {
                (*(*h).sh.weight[i_ref as usize][2usize]
                    .weightfn
                    .offset((width >> 1i32) as isize))
                .expect("non-null function pointer")(
                    (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                        .offset(2isize))
                    .offset(offset as isize),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                        .offset(2isize))
                    .offset(offset as isize),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(2isize),
                    height,
                );
            }
        }
    }
}
#[inline(never)]
unsafe extern "C" fn mb_mc_1xywh(
    mut h: *mut crate::src::common::common::x264_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        let mut i8 = x264_scan8[0usize] as ::core::ffi::c_int + x + 8i32 * y;
        let mut i_ref = (*h).mb.cache.ref_0[1usize][i8 as usize] as ::core::ffi::c_int;
        let mut mvx = x264_clip3(
            (*h).mb.cache.mv[1usize][i8 as usize][0usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[0usize],
            (*h).mb.mv_max[0usize],
        ) + 4i32 * 4i32 * x;
        let mut mvy = x264_clip3(
            (*h).mb.cache.mv[1usize][i8 as usize][1usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[1usize],
            (*h).mb.mv_max[1usize],
        ) + 4i32 * 4i32 * y;
        (*h).mc.mc_luma.expect("non-null function pointer")(
            (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                .offset(0isize))
            .offset((4i32 * y * crate::src::common::common::FDEC_STRIDE + 4i32 * x) as isize),
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                .offset(1isize)
                as *mut [*mut crate::src::common::common::pixel; 12])
                .offset(i_ref as isize)
                as *mut *mut crate::src::common::common::pixel)
                .offset((0i32 * 4i32) as isize),
            (*h).mb.pic.i_stride[0usize] as crate::stdlib::intptr_t,
            mvx,
            mvy,
            4i32 * width,
            4i32 * height,
            if 1i32 != 0 {
                &raw mut crate::src::common::tables::x264_zero
                    as *const crate::src::common::mc::x264_weight_t
            } else {
                (&raw mut *(&raw mut (*h).sh.weight
                    as *mut [crate::src::common::mc::x264_weight_t; 3])
                    .offset(i_ref as isize)
                    as *mut crate::src::common::mc::x264_weight_t)
                    .offset(0isize) as *const crate::src::common::mc::x264_weight_t
            },
        );
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1isize))
                .offset((4i32 * y * crate::src::common::common::FDEC_STRIDE + 4i32 * x) as isize),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(1isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((1i32 * 4i32) as isize),
                (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                mvx,
                mvy,
                4i32 * width,
                4i32 * height,
                if 1i32 != 0 {
                    &raw mut crate::src::common::tables::x264_zero
                        as *const crate::src::common::mc::x264_weight_t
                } else {
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(1isize)
                        as *const crate::src::common::mc::x264_weight_t
                },
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2isize))
                .offset((4i32 * y * crate::src::common::common::FDEC_STRIDE + 4i32 * x) as isize),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(1isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((2i32 * 4i32) as isize),
                (*h).mb.pic.i_stride[2usize] as crate::stdlib::intptr_t,
                mvx,
                mvy,
                4i32 * width,
                4i32 * height,
                if 1i32 != 0 {
                    &raw mut crate::src::common::tables::x264_zero
                        as *const crate::src::common::mc::x264_weight_t
                } else {
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(2isize)
                        as *const crate::src::common::mc::x264_weight_t
                },
            );
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            let mut v_shift = (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if v_shift & (*h).mb.interlaced as ::core::ffi::c_int & i_ref != 0 {
                mvy += ((*h).mb.i_mb_y & 1i32) * 4i32 - 2i32;
            }
            let mut offset =
                (4i32 * crate::src::common::common::FDEC_STRIDE >> v_shift) * y + 2i32 * x;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1isize))
                .offset(offset as isize),
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2isize))
                .offset(offset as isize),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fref[1usize][i_ref as usize][4usize],
                (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                mvx,
                2i32 * mvy >> v_shift,
                2i32 * width,
                4i32 * height >> v_shift,
            );
        }
    }
}
#[inline(never)]
unsafe extern "C" fn mb_mc_01xywh(
    mut h: *mut crate::src::common::common::x264_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        let mut i_stride0 = 16isize;
        let mut i_stride1 = 16isize;
        let mut tmp0 = [0; 256];
        let mut tmp1 = [0; 256];
        let mut i8 = x264_scan8[0usize] as ::core::ffi::c_int + x + 8i32 * y;
        let mut i_ref0 = (*h).mb.cache.ref_0[0usize][i8 as usize] as ::core::ffi::c_int;
        let mut i_ref1 = (*h).mb.cache.ref_0[1usize][i8 as usize] as ::core::ffi::c_int;
        let mut weight =
            (*(*h).mb.bipred_weight.offset(i_ref0 as isize))[i_ref1 as usize] as ::core::ffi::c_int;
        let mut mvx0 = x264_clip3(
            (*h).mb.cache.mv[0usize][i8 as usize][0usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[0usize],
            (*h).mb.mv_max[0usize],
        ) + 4i32 * 4i32 * x;
        let mut mvx1 = x264_clip3(
            (*h).mb.cache.mv[1usize][i8 as usize][0usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[0usize],
            (*h).mb.mv_max[0usize],
        ) + 4i32 * 4i32 * x;
        let mut mvy0 = x264_clip3(
            (*h).mb.cache.mv[0usize][i8 as usize][1usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[1usize],
            (*h).mb.mv_max[1usize],
        ) + 4i32 * 4i32 * y;
        let mut mvy1 = x264_clip3(
            (*h).mb.cache.mv[1usize][i8 as usize][1usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[1usize],
            (*h).mb.mv_max[1usize],
        ) + 4i32 * 4i32 * y;
        let mut i_mode = x264_size2pixel[height as usize][width as usize] as ::core::ffi::c_int;
        let mut src0 = (*h).mc.get_ref.expect("non-null function pointer")(
            &raw mut tmp0 as *mut crate::src::common::common::pixel,
            &raw mut i_stride0,
            (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                .offset(0isize)
                as *mut [*mut crate::src::common::common::pixel; 12])
                .offset(i_ref0 as isize)
                as *mut *mut crate::src::common::common::pixel)
                .offset((0i32 * 4i32) as isize),
            (*h).mb.pic.i_stride[0usize] as crate::stdlib::intptr_t,
            mvx0,
            mvy0,
            4i32 * width,
            4i32 * height,
            &raw mut crate::src::common::tables::x264_zero
                as *const crate::src::common::mc::x264_weight_t,
        );
        let mut src1 = (*h).mc.get_ref.expect("non-null function pointer")(
            &raw mut tmp1 as *mut crate::src::common::common::pixel,
            &raw mut i_stride1,
            (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                .offset(1isize)
                as *mut [*mut crate::src::common::common::pixel; 12])
                .offset(i_ref1 as isize)
                as *mut *mut crate::src::common::common::pixel)
                .offset((0i32 * 4i32) as isize),
            (*h).mb.pic.i_stride[0usize] as crate::stdlib::intptr_t,
            mvx1,
            mvy1,
            4i32 * width,
            4i32 * height,
            &raw mut crate::src::common::tables::x264_zero
                as *const crate::src::common::mc::x264_weight_t,
        );
        (*h).mc.avg[i_mode as usize].expect("non-null function pointer")(
            (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                .offset(0isize))
            .offset((4i32 * y * crate::src::common::common::FDEC_STRIDE + 4i32 * x) as isize),
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            src0,
            i_stride0,
            src1,
            i_stride1,
            weight,
        );
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            src0 = (*h).mc.get_ref.expect("non-null function pointer")(
                &raw mut tmp0 as *mut crate::src::common::common::pixel,
                &raw mut i_stride0,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(0isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref0 as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((1i32 * 4i32) as isize),
                (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                mvx0,
                mvy0,
                4i32 * width,
                4i32 * height,
                &raw mut crate::src::common::tables::x264_zero
                    as *const crate::src::common::mc::x264_weight_t,
            );
            src1 = (*h).mc.get_ref.expect("non-null function pointer")(
                &raw mut tmp1 as *mut crate::src::common::common::pixel,
                &raw mut i_stride1,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(1isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref1 as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((1i32 * 4i32) as isize),
                (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                mvx1,
                mvy1,
                4i32 * width,
                4i32 * height,
                &raw mut crate::src::common::tables::x264_zero
                    as *const crate::src::common::mc::x264_weight_t,
            );
            (*h).mc.avg[i_mode as usize].expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1isize))
                .offset((4i32 * y * crate::src::common::common::FDEC_STRIDE + 4i32 * x) as isize),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                src0,
                i_stride0,
                src1,
                i_stride1,
                weight,
            );
            src0 = (*h).mc.get_ref.expect("non-null function pointer")(
                &raw mut tmp0 as *mut crate::src::common::common::pixel,
                &raw mut i_stride0,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(0isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref0 as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((2i32 * 4i32) as isize),
                (*h).mb.pic.i_stride[2usize] as crate::stdlib::intptr_t,
                mvx0,
                mvy0,
                4i32 * width,
                4i32 * height,
                &raw mut crate::src::common::tables::x264_zero
                    as *const crate::src::common::mc::x264_weight_t,
            );
            src1 = (*h).mc.get_ref.expect("non-null function pointer")(
                &raw mut tmp1 as *mut crate::src::common::common::pixel,
                &raw mut i_stride1,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(1isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref1 as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((2i32 * 4i32) as isize),
                (*h).mb.pic.i_stride[2usize] as crate::stdlib::intptr_t,
                mvx1,
                mvy1,
                4i32 * width,
                4i32 * height,
                &raw mut crate::src::common::tables::x264_zero
                    as *const crate::src::common::mc::x264_weight_t,
            );
            (*h).mc.avg[i_mode as usize].expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2isize))
                .offset((4i32 * y * crate::src::common::common::FDEC_STRIDE + 4i32 * x) as isize),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                src0,
                i_stride0,
                src1,
                i_stride1,
                weight,
            );
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            let mut v_shift = (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if v_shift & (*h).mb.interlaced as ::core::ffi::c_int & i_ref0 != 0 {
                mvy0 += ((*h).mb.i_mb_y & 1i32) * 4i32 - 2i32;
            }
            if v_shift & (*h).mb.interlaced as ::core::ffi::c_int & i_ref1 != 0 {
                mvy1 += ((*h).mb.i_mb_y & 1i32) * 4i32 - 2i32;
            }
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &raw mut tmp0 as *mut crate::src::common::common::pixel,
                (&raw mut tmp0 as *mut crate::src::common::common::pixel).offset(8isize),
                16isize,
                (*h).mb.pic.p_fref[0usize][i_ref0 as usize][4usize],
                (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                mvx0,
                2i32 * mvy0 >> v_shift,
                2i32 * width,
                4i32 * height >> v_shift,
            );
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &raw mut tmp1 as *mut crate::src::common::common::pixel,
                (&raw mut tmp1 as *mut crate::src::common::common::pixel).offset(8isize),
                16isize,
                (*h).mb.pic.p_fref[1usize][i_ref1 as usize][4usize],
                (*h).mb.pic.i_stride[1usize] as crate::stdlib::intptr_t,
                mvx1,
                2i32 * mvy1 >> v_shift,
                2i32 * width,
                4i32 * height >> v_shift,
            );
            let mut chromapix = (*h).luma2chroma_pixel[i_mode as usize] as ::core::ffi::c_int;
            let mut offset =
                (4i32 * crate::src::common::common::FDEC_STRIDE >> v_shift) * y + 2i32 * x;
            (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1isize))
                .offset(offset as isize),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut tmp0 as *mut crate::src::common::common::pixel,
                16isize,
                &raw mut tmp1 as *mut crate::src::common::common::pixel,
                16isize,
                weight,
            );
            (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2isize))
                .offset(offset as isize),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (&raw mut tmp0 as *mut crate::src::common::common::pixel).offset(8isize),
                16isize,
                (&raw mut tmp1 as *mut crate::src::common::common::pixel).offset(8isize),
                16isize,
                weight,
            );
        }
    }
}
pub unsafe extern "C" fn x264_8_mb_mc_8x8(
    mut h: *mut crate::src::common::common::x264_t,
    mut i8: ::core::ffi::c_int,
) {
    unsafe {
        let mut x = 2i32 * (i8 & 1i32);
        let mut y = 2i32 * (i8 >> 1i32);
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            match (*h).mb.i_sub_partition[i8 as usize] as ::core::ffi::c_int {
                3 => {
                    mb_mc_0xywh(h, x, y, 2i32, 2i32);
                }
                1 => {
                    mb_mc_0xywh(h, x, y + 0i32, 2i32, 1i32);
                    mb_mc_0xywh(h, x, y + 1i32, 2i32, 1i32);
                }
                2 => {
                    mb_mc_0xywh(h, x + 0i32, y, 1i32, 2i32);
                    mb_mc_0xywh(h, x + 1i32, y, 1i32, 2i32);
                }
                0 => {
                    mb_mc_0xywh(h, x + 0i32, y + 0i32, 1i32, 1i32);
                    mb_mc_0xywh(h, x + 1i32, y + 0i32, 1i32, 1i32);
                    mb_mc_0xywh(h, x + 0i32, y + 1i32, 1i32, 1i32);
                    mb_mc_0xywh(h, x + 1i32, y + 1i32, 1i32, 1i32);
                }
                _ => {}
            }
        } else {
            let mut scan8 = x264_scan8[0usize] as ::core::ffi::c_int + x + 8i32 * y;
            if (*h).mb.cache.ref_0[0usize][scan8 as usize] as ::core::ffi::c_int >= 0i32 {
                if (*h).mb.cache.ref_0[1usize][scan8 as usize] as ::core::ffi::c_int >= 0i32 {
                    mb_mc_01xywh(h, x, y, 2i32, 2i32);
                } else {
                    mb_mc_0xywh(h, x, y, 2i32, 2i32);
                }
            } else {
                mb_mc_1xywh(h, x, y, 2i32, 2i32);
            }
        };
    }
}
pub unsafe extern "C" fn x264_8_mb_mc(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        if (*h).mb.i_partition == crate::src::common::macroblock::D_8x8 as ::core::ffi::c_int {
            let mut i = 0i32;
            while i < 4i32 {
                x264_8_mb_mc_8x8(h, i);
                i += 1;
            }
        } else {
            let mut ref0a =
                (*h).mb.cache.ref_0[0usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int;
            let mut ref0b =
                (*h).mb.cache.ref_0[0usize][x264_scan8[12usize] as usize] as ::core::ffi::c_int;
            let mut ref1a =
                (*h).mb.cache.ref_0[1usize][x264_scan8[0usize] as usize] as ::core::ffi::c_int;
            let mut ref1b =
                (*h).mb.cache.ref_0[1usize][x264_scan8[12usize] as usize] as ::core::ffi::c_int;
            if (*h).mb.i_partition == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
            {
                if ref0a >= 0i32 {
                    if ref1a >= 0i32 {
                        mb_mc_01xywh(h, 0i32, 0i32, 4i32, 4i32);
                    } else {
                        mb_mc_0xywh(h, 0i32, 0i32, 4i32, 4i32);
                    }
                } else {
                    mb_mc_1xywh(h, 0i32, 0i32, 4i32, 4i32);
                }
            } else if (*h).mb.i_partition
                == crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int
            {
                if ref0a >= 0i32 {
                    if ref1a >= 0i32 {
                        mb_mc_01xywh(h, 0i32, 0i32, 4i32, 2i32);
                    } else {
                        mb_mc_0xywh(h, 0i32, 0i32, 4i32, 2i32);
                    }
                } else {
                    mb_mc_1xywh(h, 0i32, 0i32, 4i32, 2i32);
                }
                if ref0b >= 0i32 {
                    if ref1b >= 0i32 {
                        mb_mc_01xywh(h, 0i32, 2i32, 4i32, 2i32);
                    } else {
                        mb_mc_0xywh(h, 0i32, 2i32, 4i32, 2i32);
                    }
                } else {
                    mb_mc_1xywh(h, 0i32, 2i32, 4i32, 2i32);
                }
            } else if (*h).mb.i_partition
                == crate::src::common::macroblock::D_8x16 as ::core::ffi::c_int
            {
                if ref0a >= 0i32 {
                    if ref1a >= 0i32 {
                        mb_mc_01xywh(h, 0i32, 0i32, 2i32, 4i32);
                    } else {
                        mb_mc_0xywh(h, 0i32, 0i32, 2i32, 4i32);
                    }
                } else {
                    mb_mc_1xywh(h, 0i32, 0i32, 2i32, 4i32);
                }
                if ref0b >= 0i32 {
                    if ref1b >= 0i32 {
                        mb_mc_01xywh(h, 2i32, 0i32, 2i32, 4i32);
                    } else {
                        mb_mc_0xywh(h, 2i32, 0i32, 2i32, 4i32);
                    }
                } else {
                    mb_mc_1xywh(h, 2i32, 0i32, 2i32, 4i32);
                }
            }
        };
    }
}
pub unsafe extern "C" fn x264_8_macroblock_cache_allocate(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut prealloc_idx = 0i32;
        let mut prealloc_size = 0i64;
        let mut preallocs = [::core::ptr::null_mut::<*mut crate::stdlib::uint8_t>(); 1024];
        let mut i = 0i32;
        let mut i_mb_count = (*h).mb.i_mb_count;
        (*h).mb.i_mb_stride = (*h).mb.i_mb_width;
        (*h).mb.i_b8_stride = (*h).mb.i_mb_width * 2i32;
        (*h).mb.i_b4_stride = (*h).mb.i_mb_width * 4i32;
        (*h).mb.interlaced = (*h).param.interlaced;
        (*h).mb.qp = prealloc_size as *mut crate::stdlib::int8_t;
        let c2rust_fresh0 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh0 as usize] = &raw mut (*h).mb.qp as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += (i_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>())
            as crate::stdlib::int64_t
            + (64i32 - 1i32) as crate::stdlib::int64_t
            & !(64i32 - 1i32) as crate::stdlib::int64_t;
        (*h).mb.cbp = prealloc_size as *mut crate::stdlib::int16_t;
        let c2rust_fresh1 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh1 as usize] =
            &raw mut (*h).mb.cbp as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += (i_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>())
            as crate::stdlib::int64_t
            + (64i32 - 1i32) as crate::stdlib::int64_t
            & !(64i32 - 1i32) as crate::stdlib::int64_t;
        (*h).mb.mb_transform_size = prealloc_size as *mut crate::stdlib::int8_t;
        let c2rust_fresh2 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh2 as usize] =
            &raw mut (*h).mb.mb_transform_size as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += (i_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>())
            as crate::stdlib::int64_t
            + (64i32 - 1i32) as crate::stdlib::int64_t
            & !(64i32 - 1i32) as crate::stdlib::int64_t;
        (*h).mb.slice_table = prealloc_size as *mut crate::stdlib::int32_t;
        let c2rust_fresh3 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh3 as usize] =
            &raw mut (*h).mb.slice_table as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += (i_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int32_t>())
            as crate::stdlib::int64_t
            + (64i32 - 1i32) as crate::stdlib::int64_t
            & !(64i32 - 1i32) as crate::stdlib::int64_t;
        (*h).mb.intra4x4_pred_mode = prealloc_size as *mut [crate::stdlib::int8_t; 8];
        let c2rust_fresh4 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh4 as usize] =
            &raw mut (*h).mb.intra4x4_pred_mode as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += ((i_mb_count * 8i32) as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>())
            as crate::stdlib::int64_t
            + (64i32 - 1i32) as crate::stdlib::int64_t
            & !(64i32 - 1i32) as crate::stdlib::int64_t;
        (*h).mb.non_zero_count = prealloc_size as *mut [crate::stdlib::uint8_t; 48];
        let c2rust_fresh5 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh5 as usize] =
            &raw mut (*h).mb.non_zero_count as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += ((i_mb_count * 48i32) as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>())
            as crate::stdlib::int64_t
            + (64i32 - 1i32) as crate::stdlib::int64_t
            & !(64i32 - 1i32) as crate::stdlib::int64_t;
        if (*h).param.cabac {
            (*h).mb.skipbp = prealloc_size as *mut crate::stdlib::int8_t;
            let c2rust_fresh6 = prealloc_idx;
            prealloc_idx = prealloc_idx + 1;
            preallocs[c2rust_fresh6 as usize] =
                &raw mut (*h).mb.skipbp as *mut *mut crate::stdlib::uint8_t;
            prealloc_size += (i_mb_count as usize)
                .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>())
                as crate::stdlib::int64_t
                + (64i32 - 1i32) as crate::stdlib::int64_t
                & !(64i32 - 1i32) as crate::stdlib::int64_t;
            (*h).mb.chroma_pred_mode = prealloc_size as *mut crate::stdlib::int8_t;
            let c2rust_fresh7 = prealloc_idx;
            prealloc_idx = prealloc_idx + 1;
            preallocs[c2rust_fresh7 as usize] =
                &raw mut (*h).mb.chroma_pred_mode as *mut *mut crate::stdlib::uint8_t;
            prealloc_size += (i_mb_count as usize)
                .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>())
                as crate::stdlib::int64_t
                + (64i32 - 1i32) as crate::stdlib::int64_t
                & !(64i32 - 1i32) as crate::stdlib::int64_t;
            (*h).mb.mvd[0usize] = prealloc_size as *mut [[crate::stdlib::uint8_t; 2]; 8];
            let c2rust_fresh8 = prealloc_idx;
            prealloc_idx = prealloc_idx + 1;
            preallocs[c2rust_fresh8 as usize] =
                (&raw mut (*h).mb.mvd as *mut *mut [[crate::stdlib::uint8_t; 2]; 8]).offset(0isize)
                    as *mut *mut crate::stdlib::uint8_t;
            prealloc_size += (i_mb_count as usize)
                .wrapping_mul(::core::mem::size_of::<[[crate::stdlib::uint8_t; 2]; 8]>())
                as crate::stdlib::int64_t
                + (64i32 - 1i32) as crate::stdlib::int64_t
                & !(64i32 - 1i32) as crate::stdlib::int64_t;
            if (*h).param.i_bframe != 0 {
                (*h).mb.mvd[1usize] = prealloc_size as *mut [[crate::stdlib::uint8_t; 2]; 8];
                let c2rust_fresh9 = prealloc_idx;
                prealloc_idx = prealloc_idx + 1;
                preallocs[c2rust_fresh9 as usize] =
                    (&raw mut (*h).mb.mvd as *mut *mut [[crate::stdlib::uint8_t; 2]; 8])
                        .offset(1isize) as *mut *mut crate::stdlib::uint8_t;
                prealloc_size += (i_mb_count as usize)
                    .wrapping_mul(::core::mem::size_of::<[[crate::stdlib::uint8_t; 2]; 8]>())
                    as crate::stdlib::int64_t
                    + (64i32 - 1i32) as crate::stdlib::int64_t
                    & !(64i32 - 1i32) as crate::stdlib::int64_t;
            }
        }
        while i < 2i32 {
            let mut i_refs = (if (16i32)
                < (if i != 0 {
                    1i32 + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
                } else {
                    (*h).param.i_frame_reference
                }) {
                16i32
            } else {
                if i != 0 {
                    1i32 + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
                } else {
                    (*h).param.i_frame_reference
                }
            }) << (*h).param.interlaced as ::core::ffi::c_int;
            if (*h).param.analyse.i_weighted_pred == crate::x264_h::X264_WEIGHTP_SMART {
                i_refs = if (16i32) < i_refs + 1i32 + (8i32 == 8i32) as ::core::ffi::c_int {
                    16i32
                } else {
                    i_refs + 1i32 + (8i32 == 8i32) as ::core::ffi::c_int
                };
            }
            let mut j = (i == 0) as ::core::ffi::c_int;
            while j < i_refs {
                (*h).mb.mvr[i as usize][j as usize] =
                    prealloc_size as *mut [crate::stdlib::int16_t; 2];
                let c2rust_fresh10 = prealloc_idx;
                prealloc_idx = prealloc_idx + 1;
                preallocs[c2rust_fresh10 as usize] = (&raw mut *(&raw mut (*h).mb.mvr
                    as *mut [*mut [crate::stdlib::int16_t; 2]; 32])
                    .offset(i as isize)
                    as *mut *mut [crate::stdlib::int16_t; 2])
                    .offset(j as isize)
                    as *mut *mut crate::stdlib::uint8_t;
                prealloc_size += ((2i32 * (i_mb_count + 1i32)) as usize)
                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>())
                    as crate::stdlib::int64_t
                    + (64i32 - 1i32) as crate::stdlib::int64_t
                    & !(64i32 - 1i32) as crate::stdlib::int64_t;
                j += 1;
            }
            i += 1;
        }
        if (*h).param.analyse.i_weighted_pred != 0 {
            let mut luma_plane_size = 0i32;
            let mut numweightbuf = 0;
            let mut i_0 = 0i32;
            let mut i_padv =
                crate::src::common::frame::PADV << (*h).param.interlaced as ::core::ffi::c_int;
            if (*h).param.analyse.i_weighted_pred == crate::src::common::base::X264_WEIGHTP_FAKE {
                if (*h).param.i_sync_lookahead == 0
                    || h == (*h).thread[(*h).param.i_threads as usize]
                {
                    luma_plane_size =
                        (*(*h).fdec).i_stride_lowres * ((*h).mb.i_mb_height * 8i32 + 2i32 * i_padv);
                    numweightbuf = 1i32;
                } else {
                    numweightbuf = 0i32;
                }
            } else {
                luma_plane_size = (*(*h).fdec).i_stride[0usize]
                    * ((*h).mb.i_mb_height
                        * ((16i32)
                            << (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int)
                                as ::core::ffi::c_int)
                        + 2i32 * i_padv);
                if (*h).param.analyse.i_weighted_pred == crate::x264_h::X264_WEIGHTP_SMART {
                    numweightbuf =
                        1i32 + (crate::internal::BIT_DEPTH == 8i32) as ::core::ffi::c_int;
                } else {
                    numweightbuf = 1i32;
                }
            }
            while i_0 < numweightbuf {
                (*h).mb.p_weight_buf[i_0 as usize] =
                    prealloc_size as *mut crate::src::common::common::pixel;
                let c2rust_fresh11 = prealloc_idx;
                prealloc_idx = prealloc_idx + 1;
                preallocs[c2rust_fresh11 as usize] = (&raw mut (*h).mb.p_weight_buf
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(i_0 as isize);
                prealloc_size += (luma_plane_size
                    * ::core::mem::size_of::<crate::src::common::common::pixel>()
                        as ::core::ffi::c_int)
                    as crate::stdlib::int64_t
                    + (64i32 - 1i32) as crate::stdlib::int64_t
                    & !(64i32 - 1i32) as crate::stdlib::int64_t;
                i_0 += 1;
            }
        }
        (*h).mb.base =
            crate::src::common::base::x264_malloc(prealloc_size) as *mut crate::stdlib::uint8_t;
        if (*h).mb.base.is_null() {
            return -(1i32);
        } else {
            let mut i_1 = 0i32;
            loop {
                let c2rust_fresh12 = prealloc_idx;
                prealloc_idx = prealloc_idx - 1;
                if !(c2rust_fresh12 != 0) {
                    break;
                }
                *preallocs[prealloc_idx as usize] = (*preallocs[prealloc_idx as usize]
                    as crate::stdlib::intptr_t
                    + (*h).mb.base as crate::stdlib::intptr_t)
                    as *mut crate::stdlib::uint8_t;
            }
            crate::stdlib::memset(
                (*h).mb.slice_table as *mut ::core::ffi::c_void,
                -(1i32),
                (i_mb_count as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::int32_t>()),
            );
            while i_1 < 2i32 {
                let mut i_refs_0 = (if (16i32)
                    < (if i_1 != 0 {
                        1i32 + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
                    } else {
                        (*h).param.i_frame_reference
                    }) {
                    16i32
                } else {
                    if i_1 != 0 {
                        1i32 + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
                    } else {
                        (*h).param.i_frame_reference
                    }
                }) << (*h).param.interlaced as ::core::ffi::c_int;
                if (*h).param.analyse.i_weighted_pred == crate::x264_h::X264_WEIGHTP_SMART {
                    i_refs_0 = if (16i32) < i_refs_0 + 1i32 + (8i32 == 8i32) as ::core::ffi::c_int {
                        16i32
                    } else {
                        i_refs_0 + 1i32 + (8i32 == 8i32) as ::core::ffi::c_int
                    };
                }
                let mut j_0 = (i_1 == 0) as ::core::ffi::c_int;
                while j_0 < i_refs_0 {
                    (*(&raw mut *(*(&raw mut *(&raw mut (*h).mb.mvr
                        as *mut [*mut [crate::stdlib::int16_t; 2]; 32])
                        .offset(i_1 as isize)
                        as *mut *mut [crate::stdlib::int16_t; 2])
                        .offset(j_0 as isize))
                    .offset(0isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0u32;
                    (*h).mb.mvr[i_1 as usize][j_0 as usize] =
                        (*h).mb.mvr[i_1 as usize][j_0 as usize].offset(1);
                    j_0 += 1;
                }
                i_1 += 1;
            }
            return 0i32;
        };
    }
}
pub unsafe extern "C" fn x264_8_macroblock_cache_free(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        crate::src::common::base::x264_free((*h).mb.base as *mut ::core::ffi::c_void);
    }
}
pub unsafe extern "C" fn x264_8_macroblock_thread_allocate(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_lookahead: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        if b_lookahead == 0 {
            's_5: loop {
                let mut i = 0i32;
                let mut j = 0i32;
                if !(i < (if (*h).param.interlaced { 5i32 } else { 2i32 })) {
                    c2rust_current_block = 8515828400728868193;
                    break;
                }
                while j
                    < (if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    {
                        3i32
                    } else {
                        2i32
                    })
                {
                    (*h).intra_border_backup[i as usize][j as usize] =
                        crate::src::common::base::x264_malloc(
                            (((*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                .i_mb_width
                                * 16i32
                                + 32i32)
                                * ::core::mem::size_of::<crate::src::common::common::pixel>()
                                    as ::core::ffi::c_int)
                                as crate::stdlib::int64_t,
                        ) as *mut crate::src::common::common::pixel;
                    if (*h).intra_border_backup[i as usize][j as usize].is_null() {
                        c2rust_current_block = 2278402434551512179;
                        break 's_5;
                    }
                    (*h).intra_border_backup[i as usize][j as usize] =
                        (*h).intra_border_backup[i as usize][j as usize].offset(16isize);
                    j += 1;
                }
                i += 1;
            }
            match c2rust_current_block {
                2278402434551512179 => {}
                _ => loop {
                    let mut i_0 = 0i32;
                    if !(i_0 <= (*h).param.interlaced as ::core::ffi::c_int) {
                        c2rust_current_block = 5783071609795492627;
                        break;
                    }
                    if (*h).param.sliced_threads {
                        if h == (*h).thread[0usize] && i_0 == 0 {
                            (*h).deblock_strength[0usize] = crate::src::common::base::x264_malloc(
                                (::core::mem::size_of::<[[[crate::stdlib::uint8_t; 4]; 8]; 2]>())
                                    .wrapping_mul((*h).mb.i_mb_count as usize)
                                    as crate::stdlib::int64_t,
                            )
                                as *mut [[[crate::stdlib::uint8_t; 4]; 8]; 2];
                            if (*h).deblock_strength[0usize].is_null() {
                                c2rust_current_block = 2278402434551512179;
                                break;
                            }
                        } else {
                            (*h).deblock_strength[i_0 as usize] =
                                (*(*h).thread[0usize]).deblock_strength[0usize];
                        }
                    } else {
                        (*h).deblock_strength[i_0 as usize] = crate::src::common::base::x264_malloc(
                            (::core::mem::size_of::<[[[crate::stdlib::uint8_t; 4]; 8]; 2]>())
                                .wrapping_mul((*h).mb.i_mb_width as usize)
                                as crate::stdlib::int64_t,
                        )
                            as *mut [[[crate::stdlib::uint8_t; 4]; 8]; 2];
                        if (*h).deblock_strength[i_0 as usize].is_null() {
                            c2rust_current_block = 2278402434551512179;
                            break;
                        }
                    }
                    (*h).deblock_strength[1usize] = (*h).deblock_strength[i_0 as usize];
                    i_0 += 1;
                },
            }
        } else {
            c2rust_current_block = 5783071609795492627;
        }
        match c2rust_current_block {
            5783071609795492627 => {
                let mut scratch_size = 0;
                let mut buf_mbtree = 0;
                scratch_size = 0i32;
                if b_lookahead == 0 {
                    let mut buf_hpel =
                        (((*(*(*h).thread[0usize]).fdec).i_width[0usize] + 48i32 + 32i32) as usize)
                            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>())
                            as ::core::ffi::c_int;
                    let mut buf_ssim = (((*h).param.analyse.ssim as ::core::ffi::c_int
                        * 8i32
                        * ((*h).param.i_width / 4i32 + 3i32))
                        as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>())
                        as ::core::ffi::c_int;
                    let mut me_range =
                        if (*h).param.analyse.i_me_range < (*h).param.analyse.i_mv_range {
                            (*h).param.analyse.i_me_range
                        } else {
                            (*h).param.analyse.i_mv_range
                        };
                    let mut buf_tesa = (((*h).param.analyse.i_me_method
                        >= crate::x264_h::X264_ME_ESA)
                        as ::core::ffi::c_int as usize)
                        .wrapping_mul(
                            ((me_range * 2i32 + 24i32) as usize)
                                .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>())
                                .wrapping_add(
                                    (((me_range + 4i32) * (me_range + 1i32) * 4i32) as usize)
                                        .wrapping_mul(::core::mem::size_of::<
                                            crate::src::common::common::mvsad_t,
                                        >()),
                                ),
                        ) as ::core::ffi::c_int;
                    scratch_size = if buf_hpel
                        > (if buf_ssim > buf_tesa {
                            buf_ssim
                        } else {
                            buf_tesa
                        }) {
                        buf_hpel
                    } else if buf_ssim > buf_tesa {
                        buf_ssim
                    } else {
                        buf_tesa
                    };
                }
                buf_mbtree = ((*h).param.rc.mb_tree as usize).wrapping_mul(
                    ((*h).mb.i_mb_width as usize)
                        .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>())
                        .wrapping_add((64i32 - 1i32) as usize)
                        & !(64i32 - 1i32) as usize,
                ) as ::core::ffi::c_int;
                scratch_size = if scratch_size > buf_mbtree {
                    scratch_size
                } else {
                    buf_mbtree
                };
                if scratch_size != 0 {
                    (*h).scratch_buffer = crate::src::common::base::x264_malloc(
                        scratch_size as crate::stdlib::int64_t,
                    );
                    if (*h).scratch_buffer.is_null() {
                        c2rust_current_block = 2278402434551512179;
                    } else {
                        c2rust_current_block = 2891135413264362348;
                    }
                } else {
                    (*h).scratch_buffer = crate::__stddef_null_h::NULL;
                    c2rust_current_block = 2891135413264362348;
                }
                match c2rust_current_block {
                    2278402434551512179 => {}
                    _ => {
                        let mut buf_lookahead_threads = 0;
                        let mut buf_mbtree2 = 0;
                        buf_lookahead_threads = (((*h).mb.i_mb_height
                            + (4i32 + 32i32) * (*h).param.i_lookahead_threads)
                            as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>())
                            .wrapping_mul(2usize)
                            as ::core::ffi::c_int;
                        buf_mbtree2 = buf_mbtree * 12i32;
                        scratch_size = if buf_lookahead_threads > buf_mbtree2 {
                            buf_lookahead_threads
                        } else {
                            buf_mbtree2
                        };
                        (*h).scratch_buffer2 = crate::src::common::base::x264_malloc(
                            scratch_size as crate::stdlib::int64_t,
                        );
                        if !(*h).scratch_buffer2.is_null() {
                            return 0i32;
                        }
                    }
                }
            }
            _ => {}
        }
        return -(1i32);
    }
}
pub unsafe extern "C" fn x264_8_macroblock_thread_free(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_lookahead: ::core::ffi::c_int,
) {
    unsafe {
        if b_lookahead == 0 {
            let mut i = 0i32;
            let mut i_0 = 0i32;
            while i <= (*h).param.interlaced as ::core::ffi::c_int {
                if !(*h).param.sliced_threads || h == (*h).thread[0usize] && i == 0 {
                    crate::src::common::base::x264_free(
                        (*h).deblock_strength[i as usize] as *mut ::core::ffi::c_void,
                    );
                }
                i += 1;
            }
            while i_0 < (if (*h).param.interlaced { 5i32 } else { 2i32 }) {
                let mut j = 0i32;
                while j
                    < (if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    {
                        3i32
                    } else {
                        2i32
                    })
                {
                    crate::src::common::base::x264_free(
                        (*h).intra_border_backup[i_0 as usize][j as usize].offset(-(16isize))
                            as *mut ::core::ffi::c_void,
                    );
                    j += 1;
                }
                i_0 += 1;
            }
        }
        crate::src::common::base::x264_free((*h).scratch_buffer);
        crate::src::common::base::x264_free((*h).scratch_buffer2);
    }
}
pub unsafe extern "C" fn x264_8_macroblock_slice_init(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut i = 0i32;
        (*h).mb.mv[0usize] = (*(*h).fdec).mv[0usize];
        (*h).mb.mv[1usize] = (*(*h).fdec).mv[1usize];
        (*h).mb.mvr[0usize][0usize] = (*(*h).fdec).mv16x16;
        (*h).mb.ref_0[0usize] = (*(*h).fdec).ref_0[0usize];
        (*h).mb.ref_0[1usize] = (*(*h).fdec).ref_0[1usize];
        (*h).mb.type_0 = (*(*h).fdec).mb_type;
        (*h).mb.partition = (*(*h).fdec).mb_partition;
        (*h).mb.field = (*(*h).fdec).field;
        (*(*h).fdec).i_ref[0usize] = (*h).i_ref[0usize];
        (*(*h).fdec).i_ref[1usize] = (*h).i_ref[1usize];
        while i < (*h).i_ref[0usize] {
            (*(*h).fdec).ref_poc[0usize][i as usize] = (*(*h).fref[0usize][i as usize]).i_poc;
            i += 1;
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            let mut i_0 = 0i32;
            let mut i_1 = 0i32;
            while i_0 < (*h).i_ref[1usize] {
                (*(*h).fdec).ref_poc[1usize][i_0 as usize] =
                    (*(*h).fref[1usize][i_0 as usize]).i_poc;
                i_0 += 1;
            }
            (*h).mb.map_col_to_list0[(-(1i32) + 2i32) as usize] = -1i8;
            (*h).mb.map_col_to_list0[(-(2i32) + 2i32) as usize] = -2i8;
            while i_1 < (*(*h).fref[1usize][0usize]).i_ref[0usize] {
                let mut j = 0i32;
                let mut poc = (*(*h).fref[1usize][0usize]).ref_poc[0usize][i_1 as usize];
                (*h).mb.map_col_to_list0[(i_1 + 2i32) as usize] = -2i8;
                while j < (*h).i_ref[0usize] {
                    if (*(*h).fref[0usize][j as usize]).i_poc == poc {
                        (*h).mb.map_col_to_list0[(i_1 + 2i32) as usize] =
                            j as crate::stdlib::int8_t;
                        break;
                    } else {
                        j += 1;
                    }
                }
                i_1 += 1;
            }
        } else if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            if (*h).sh.i_disable_deblocking_filter_idc != 1i32
                && (*h).param.analyse.i_weighted_pred == crate::x264_h::X264_WEIGHTP_SMART
            {
                let mut i_2 = 0i32;
                (*h).mb.deblock_ref_table[(-(2i32) + 2i32) as usize] = -2i8;
                (*h).mb.deblock_ref_table[(-(1i32) + 2i32) as usize] = -1i8;
                while i_2 < (*h).i_ref[0usize] << (*h).sh.mbaff as ::core::ffi::c_int {
                    if !(*h).mb.interlaced {
                        (*h).mb.deblock_ref_table[(i_2 + 2i32) as usize] =
                            ((*(*h).fref[0usize][i_2 as usize]).i_frame_num & 63i32)
                                as crate::stdlib::int8_t;
                    } else {
                        (*h).mb.deblock_ref_table[(i_2 + 2i32) as usize] =
                            ((((*(*h).fref[0usize][(i_2 >> 1i32) as usize]).i_frame_num & 63i32)
                                << 1i32)
                                + (i_2 & 1i32))
                                as crate::stdlib::int8_t;
                    }
                    i_2 += 1;
                }
            }
        }
        crate::stdlib::memset(
            &raw mut (*h).mb.cache.ref_0 as *mut ::core::ffi::c_void,
            -(2i32),
            ::core::mem::size_of::<[[crate::stdlib::int8_t; 40]; 2]>(),
        );
        if (*h).i_ref[0usize] > 0i32 {
            let mut field = 0i32;
            while field <= (*h).sh.mbaff as ::core::ffi::c_int {
                let mut curpoc = (*(*h).fdec).i_poc + (*(*h).fdec).i_delta_poc[field as usize];
                let mut refpoc = (*(*h).fref[0usize][0usize]).i_poc
                    + (*(*h).fref[0usize][0usize]).i_delta_poc[field as usize];
                let mut delta = curpoc - refpoc;
                (*(*h).fdec).inv_ref_poc[field as usize] =
                    ((256i32 + delta / 2i32) / delta) as crate::stdlib::int16_t;
                field += 1;
            }
        }
        (*h).mb.i_neighbour4[14usize] = (crate::src::common::macroblock::MB_LEFT
            as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[12usize] = (*h).mb.i_neighbour4[14usize];
        (*h).mb.i_neighbour4[9usize] = (*h).mb.i_neighbour4[12usize];
        (*h).mb.i_neighbour4[6usize] = (*h).mb.i_neighbour4[9usize];
        (*h).mb.i_neighbour8[3usize] = (crate::src::common::macroblock::MB_LEFT
            as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[15usize] = (*h).mb.i_neighbour8[3usize];
        (*h).mb.i_neighbour4[13usize] = (*h).mb.i_neighbour4[15usize];
        (*h).mb.i_neighbour4[11usize] = (*h).mb.i_neighbour4[13usize];
        (*h).mb.i_neighbour4[7usize] = (*h).mb.i_neighbour4[11usize];
        (*h).mb.i_neighbour4[3usize] = (*h).mb.i_neighbour4[7usize];
    }
}
pub unsafe extern "C" fn x264_8_macroblock_thread_init(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        (*h).mb.i_me_method = (*h).param.analyse.i_me_method;
        (*h).mb.i_subpel_refine = (*h).param.analyse.i_subpel_refine;
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
            && ((*h).mb.i_subpel_refine == 6i32 || (*h).mb.i_subpel_refine == 8i32)
        {
            (*h).mb.i_subpel_refine -= 1;
        }
        (*h).mb.chroma_me = (*h).param.analyse.chroma_me
            && ((*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
                && (*h).mb.i_subpel_refine >= 5i32
                || (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
                    && (*h).mb.i_subpel_refine >= 9i32);
        (*h).mb.dct_decimate = (*h).sh.i_type
            == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
            || (*h).param.analyse.dct_decimate
                && (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int;
        (*h).mb.i_mb_prev_xy = -(1i32);
        (*h).mb.pic.p_fenc[0usize] =
            &raw mut (*h).mb.pic.fenc_buf as *mut crate::src::common::common::pixel;
        (*h).mb.pic.p_fdec[0usize] = (&raw mut (*h).mb.pic.fdec_buf
            as *mut crate::src::common::common::pixel)
            .offset((2i32 * crate::src::common::common::FDEC_STRIDE) as isize);
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            (*h).mb.pic.p_fenc[1usize] = (&raw mut (*h).mb.pic.fenc_buf
                as *mut crate::src::common::common::pixel)
                .offset((16i32 * crate::src::common::common::FENC_STRIDE) as isize);
            (*h).mb.pic.p_fdec[1usize] = (&raw mut (*h).mb.pic.fdec_buf
                as *mut crate::src::common::common::pixel)
                .offset((20i32 * crate::src::common::common::FDEC_STRIDE) as isize);
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                (*h).mb.pic.p_fenc[2usize] = (&raw mut (*h).mb.pic.fenc_buf
                    as *mut crate::src::common::common::pixel)
                    .offset((32i32 * crate::src::common::common::FENC_STRIDE) as isize);
                (*h).mb.pic.p_fdec[2usize] = (&raw mut (*h).mb.pic.fdec_buf
                    as *mut crate::src::common::common::pixel)
                    .offset((38i32 * crate::src::common::common::FDEC_STRIDE) as isize);
            } else {
                (*h).mb.pic.p_fenc[2usize] = (&raw mut (*h).mb.pic.fenc_buf
                    as *mut crate::src::common::common::pixel)
                    .offset((16i32 * crate::src::common::common::FENC_STRIDE) as isize)
                    .offset(8isize);
                (*h).mb.pic.p_fdec[2usize] = (&raw mut (*h).mb.pic.fdec_buf
                    as *mut crate::src::common::common::pixel)
                    .offset((20i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                    .offset(16isize);
            }
        }
    }
}
pub unsafe extern "C" fn x264_8_prefetch_fenc(
    mut h: *mut crate::src::common::common::x264_t,
    mut fenc: *mut crate::src::common::frame::x264_frame_t,
    mut i_mb_x: ::core::ffi::c_int,
    mut i_mb_y: ::core::ffi::c_int,
) {
    unsafe {
        let mut stride_y = (*fenc).i_stride[0usize];
        let mut stride_uv = (*fenc).i_stride[1usize];
        let mut off_y = 16i32 * i_mb_x + 16i32 * i_mb_y * stride_y;
        let mut off_uv = 16i32 * i_mb_x
            + (16i32 * i_mb_y * stride_uv
                >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
        (*h).mc.prefetch_fenc.expect("non-null function pointer")(
            (*fenc).plane[0usize].offset(off_y as isize),
            stride_y as crate::stdlib::intptr_t,
            if !(*fenc).plane[1usize].is_null() {
                (*fenc).plane[1usize].offset(off_uv as isize)
            } else {
                ::core::ptr::null_mut::<crate::src::common::common::pixel>()
            },
            stride_uv as crate::stdlib::intptr_t,
            i_mb_x,
        );
    }
}
#[inline(never)]
pub unsafe extern "C" fn x264_8_copy_column8(
    mut dst: *mut crate::src::common::common::pixel,
    mut src: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut i = -(4i32);
        while i < 4i32 {
            *dst.offset((i * crate::src::common::common::FDEC_STRIDE) as isize) =
                *src.offset((i * crate::src::common::common::FDEC_STRIDE) as isize);
            i += 1;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_load_pic_pointers(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut i: ::core::ffi::c_int,
    mut b_chroma: ::core::ffi::c_int,
    mut b_mbaff: ::core::ffi::c_int,
) {
    unsafe {
        let mut plane_src = ::core::ptr::null_mut::<crate::src::common::common::pixel>();
        let mut filtered_src = ::core::ptr::null_mut::<*mut crate::src::common::common::pixel>();
        let mut j_0 = 0i32;
        let mut mb_interlaced = (b_mbaff != 0 && (*h).mb.interlaced) as ::core::ffi::c_int;
        let mut height = if b_chroma != 0 {
            16i32
                >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
        } else {
            16i32
        };
        let mut i_stride = (*(*h).fdec).i_stride[i as usize];
        let mut i_stride2 = i_stride << mb_interlaced;
        let mut i_pix_offset = if mb_interlaced != 0 {
            16i32 * mb_x + height * (mb_y & !(1i32)) * i_stride + (mb_y & 1i32) * i_stride
        } else {
            16i32 * mb_x + height * mb_y * i_stride
        };
        let mut plane_fdec = (*(&raw mut (*(*h).fdec).plane
            as *mut *mut crate::src::common::common::pixel)
            .offset(i as isize))
        .offset(i_pix_offset as isize);
        let mut fdec_idx = if b_mbaff != 0 {
            if mb_interlaced != 0 {
                3i32 + (mb_y & 1i32)
            } else if mb_y & 1i32 != 0 {
                2i32
            } else {
                4i32
            }
        } else {
            (mb_y & 1i32 == 0) as ::core::ffi::c_int
        };
        let mut intra_fdec = (*(&raw mut *(&raw mut (*h).intra_border_backup
            as *mut [*mut crate::src::common::common::pixel; 3])
            .offset(fdec_idx as isize)
            as *mut *mut crate::src::common::common::pixel)
            .offset(i as isize))
        .offset((mb_x * 16i32) as isize);
        let mut ref_pix_offset = [i_pix_offset, i_pix_offset];
        if mb_interlaced != 0 {
            ref_pix_offset[1usize] += (1i32 - 2i32 * (mb_y & 1i32)) * i_stride;
        }
        (*h).mb.pic.i_stride[i as usize] = i_stride2;
        (*h).mb.pic.p_fenc_plane[i as usize] = (*(&raw mut (*(*h).fenc).plane
            as *mut *mut crate::src::common::common::pixel)
            .offset(i as isize))
        .offset(i_pix_offset as isize);
        if b_chroma != 0 {
            (*h).mc
                .load_deinterleave_chroma_fenc
                .expect("non-null function pointer")(
                (*h).mb.pic.p_fenc[1usize],
                (*h).mb.pic.p_fenc_plane[1usize],
                i_stride2 as crate::stdlib::intptr_t,
                height,
            );
            crate::stdlib::memcpy(
                (*h).mb.pic.p_fdec[1usize]
                    .offset(-(crate::src::common::common::FDEC_STRIDE as isize))
                    as *mut ::core::ffi::c_void,
                intra_fdec as *const ::core::ffi::c_void,
                (8i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                (*h).mb.pic.p_fdec[2usize]
                    .offset(-(crate::src::common::common::FDEC_STRIDE as isize))
                    as *mut ::core::ffi::c_void,
                intra_fdec.offset(8isize) as *const ::core::ffi::c_void,
                (8i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            *(*h).mb.pic.p_fdec[1usize]
                .offset((-crate::src::common::common::FDEC_STRIDE - 1i32) as isize) =
                *intra_fdec.offset((-(1i32) - 8i32) as isize);
            *(*h).mb.pic.p_fdec[2usize]
                .offset((-crate::src::common::common::FDEC_STRIDE - 1i32) as isize) =
                *intra_fdec.offset(-1isize);
        } else {
            (*h).mc.copy[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                (*h).mb.pic.p_fenc[i as usize],
                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fenc_plane[i as usize],
                i_stride2 as crate::stdlib::intptr_t,
                16i32,
            );
            crate::stdlib::memcpy(
                (*h).mb.pic.p_fdec[i as usize]
                    .offset(-(crate::src::common::common::FDEC_STRIDE as isize))
                    as *mut ::core::ffi::c_void,
                intra_fdec as *const ::core::ffi::c_void,
                (24i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            *(*h).mb.pic.p_fdec[i as usize]
                .offset((-crate::src::common::common::FDEC_STRIDE - 1i32) as isize) =
                *intra_fdec.offset(-1isize);
        }
        if b_mbaff != 0 || (*h).mb.reencode_mb {
            let mut j = 0i32;
            while j < height {
                if b_chroma != 0 {
                    *(*h).mb.pic.p_fdec[1usize]
                        .offset((-(1i32) + j * crate::src::common::common::FDEC_STRIDE) as isize) =
                        *plane_fdec.offset((-(2i32) + j * i_stride2) as isize);
                    *(*h).mb.pic.p_fdec[2usize]
                        .offset((-(1i32) + j * crate::src::common::common::FDEC_STRIDE) as isize) =
                        *plane_fdec.offset((-(1i32) + j * i_stride2) as isize);
                } else {
                    *(*h).mb.pic.p_fdec[i as usize]
                        .offset((-(1i32) + j * crate::src::common::common::FDEC_STRIDE) as isize) =
                        *plane_fdec.offset((-(1i32) + j * i_stride2) as isize);
                }
                j += 1;
            }
        }
        while j_0 < (*h).mb.pic.i_fref[0usize] {
            if mb_interlaced != 0 {
                plane_src = (*(*h).fref[0usize][(j_0 >> 1i32) as usize]).plane_fld[i as usize];
                filtered_src = &raw mut *(&raw mut (**(&raw mut *(&raw mut (*h).fref
                    as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                    .offset(0isize)
                    as *mut *mut crate::src::common::frame::x264_frame_t)
                    .offset((j_0 >> 1i32) as isize))
                .filtered_fld
                    as *mut [*mut crate::src::common::common::pixel; 4])
                    .offset(i as isize)
                    as *mut *mut crate::src::common::common::pixel;
            } else {
                plane_src = (*(*h).fref[0usize][j_0 as usize]).plane[i as usize];
                filtered_src = &raw mut *(&raw mut (**(&raw mut *(&raw mut (*h).fref
                    as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                    .offset(0isize)
                    as *mut *mut crate::src::common::frame::x264_frame_t)
                    .offset(j_0 as isize))
                .filtered
                    as *mut [*mut crate::src::common::common::pixel; 4])
                    .offset(i as isize)
                    as *mut *mut crate::src::common::common::pixel;
            }
            (*h).mb.pic.p_fref[0usize][j_0 as usize][(i * 4i32) as usize] =
                plane_src.offset(ref_pix_offset[(j_0 & 1i32) as usize] as isize);
            if b_chroma == 0 {
                if (*h).param.analyse.i_subpel_refine != 0 {
                    let mut k = 1i32;
                    while k < 4i32 {
                        (*h).mb.pic.p_fref[0usize][j_0 as usize][(i * 4i32 + k) as usize] =
                            (*filtered_src.offset(k as isize))
                                .offset(ref_pix_offset[(j_0 & 1i32) as usize] as isize);
                        k += 1;
                    }
                }
                if i == 0 {
                    if !(*h).sh.weight[j_0 as usize][0usize].weightfn.is_null() {
                        (*h).mb.pic.p_fref_w[j_0 as usize] = (*(&raw mut (*(*h).fenc).weighted
                            as *mut *mut crate::src::common::common::pixel)
                            .offset((j_0 >> mb_interlaced) as isize))
                        .offset(
                            *(&raw mut ref_pix_offset as *mut ::core::ffi::c_int)
                                .offset((j_0 & 1i32) as isize) as isize,
                        );
                    } else {
                        (*h).mb.pic.p_fref_w[j_0 as usize] =
                            (*h).mb.pic.p_fref[0usize][j_0 as usize][0usize];
                    }
                }
            }
            j_0 += 1;
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            let mut j_1 = 0i32;
            while j_1 < (*h).mb.pic.i_fref[1usize] {
                if mb_interlaced != 0 {
                    plane_src = (*(*h).fref[1usize][(j_1 >> 1i32) as usize]).plane_fld[i as usize];
                    filtered_src = &raw mut *(&raw mut (**(&raw mut *(&raw mut (*h).fref
                        as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                        .offset(1isize)
                        as *mut *mut crate::src::common::frame::x264_frame_t)
                        .offset((j_1 >> 1i32) as isize))
                    .filtered_fld
                        as *mut [*mut crate::src::common::common::pixel; 4])
                        .offset(i as isize)
                        as *mut *mut crate::src::common::common::pixel;
                } else {
                    plane_src = (*(*h).fref[1usize][j_1 as usize]).plane[i as usize];
                    filtered_src = &raw mut *(&raw mut (**(&raw mut *(&raw mut (*h).fref
                        as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                        .offset(1isize)
                        as *mut *mut crate::src::common::frame::x264_frame_t)
                        .offset(j_1 as isize))
                    .filtered
                        as *mut [*mut crate::src::common::common::pixel; 4])
                        .offset(i as isize)
                        as *mut *mut crate::src::common::common::pixel;
                }
                (*h).mb.pic.p_fref[1usize][j_1 as usize][(i * 4i32) as usize] =
                    plane_src.offset(ref_pix_offset[(j_1 & 1i32) as usize] as isize);
                if b_chroma == 0 && (*h).param.analyse.i_subpel_refine != 0 {
                    let mut k_0 = 1i32;
                    while k_0 < 4i32 {
                        (*h).mb.pic.p_fref[1usize][j_1 as usize][(i * 4i32 + k_0) as usize] =
                            (*filtered_src.offset(k_0 as isize))
                                .offset(ref_pix_offset[(j_1 & 1i32) as usize] as isize);
                        k_0 += 1;
                    }
                }
                j_1 += 1;
            }
        }
    }
}
static mut left_indices: [crate::src::common::common::x264_left_table_t; 4] = [
    crate::src::common::common::x264_left_table_t {
        intra: [4u8, 4u8, 5u8, 5u8],
        nnz: [3u8, 3u8, 7u8, 7u8],
        nnz_chroma: [
            (16i32 + 1i32) as crate::stdlib::uint8_t,
            (16i32 + 1i32) as crate::stdlib::uint8_t,
            (32i32 + 1i32) as crate::stdlib::uint8_t,
            (32i32 + 1i32) as crate::stdlib::uint8_t,
        ],
        mv: [0u8, 0u8, 1u8, 1u8],
        ref_0: [0u8, 0u8, 0u8, 0u8],
    },
    crate::src::common::common::x264_left_table_t {
        intra: [6u8, 6u8, 3u8, 3u8],
        nnz: [11u8, 11u8, 15u8, 15u8],
        nnz_chroma: [
            (16i32 + 5i32) as crate::stdlib::uint8_t,
            (16i32 + 5i32) as crate::stdlib::uint8_t,
            (32i32 + 5i32) as crate::stdlib::uint8_t,
            (32i32 + 5i32) as crate::stdlib::uint8_t,
        ],
        mv: [2u8, 2u8, 3u8, 3u8],
        ref_0: [1u8, 1u8, 1u8, 1u8],
    },
    crate::src::common::common::x264_left_table_t {
        intra: [4u8, 6u8, 4u8, 6u8],
        nnz: [3u8, 11u8, 3u8, 11u8],
        nnz_chroma: [
            (16i32 + 1i32) as crate::stdlib::uint8_t,
            (16i32 + 1i32) as crate::stdlib::uint8_t,
            (32i32 + 1i32) as crate::stdlib::uint8_t,
            (32i32 + 1i32) as crate::stdlib::uint8_t,
        ],
        mv: [0u8, 2u8, 0u8, 2u8],
        ref_0: [0u8, 1u8, 0u8, 1u8],
    },
    crate::src::common::common::x264_left_table_t {
        intra: [4u8, 5u8, 6u8, 3u8],
        nnz: [3u8, 7u8, 11u8, 15u8],
        nnz_chroma: [
            (16i32 + 1i32) as crate::stdlib::uint8_t,
            (16i32 + 5i32) as crate::stdlib::uint8_t,
            (32i32 + 1i32) as crate::stdlib::uint8_t,
            (32i32 + 5i32) as crate::stdlib::uint8_t,
        ],
        mv: [0u8, 1u8, 2u8, 3u8],
        ref_0: [0u8, 0u8, 1u8, 1u8],
    },
];
#[inline(always)]
unsafe extern "C" fn macroblock_cache_load_neighbours(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut b_interlaced: ::core::ffi::c_int,
) {
    unsafe {
        let mut left = [0; 2];
        let mb_interlaced = (b_interlaced != 0 && (*h).mb.interlaced) as ::core::ffi::c_int;
        let mut top_y = mb_y - ((1i32) << mb_interlaced);
        let mut top = top_y * (*h).mb.i_mb_stride + mb_x;
        (*h).mb.i_mb_x = mb_x;
        (*h).mb.i_mb_y = mb_y;
        (*h).mb.i_mb_xy = mb_y * (*h).mb.i_mb_stride + mb_x;
        (*h).mb.i_b8_xy = 2i32 * (mb_y * (*h).mb.i_b8_stride + mb_x);
        (*h).mb.i_b4_xy = 4i32 * (mb_y * (*h).mb.i_b4_stride + mb_x);
        (*h).mb.left_b8[1usize] = -(1i32);
        (*h).mb.left_b8[0usize] = (*h).mb.left_b8[1usize];
        (*h).mb.left_b4[1usize] = -(1i32);
        (*h).mb.left_b4[0usize] = (*h).mb.left_b4[1usize];
        (*h).mb.i_neighbour = 0u32;
        (*h).mb.i_neighbour_intra = 0u32;
        (*h).mb.i_neighbour_frame = 0u32;
        (*h).mb.i_mb_top_xy = -(1i32);
        (*h).mb.i_mb_top_y = -(1i32);
        (*h).mb.i_mb_left_xy[1usize] = -(1i32);
        (*h).mb.i_mb_left_xy[0usize] = (*h).mb.i_mb_left_xy[1usize];
        (*h).mb.i_mb_topleft_xy = -(1i32);
        (*h).mb.i_mb_topright_xy = -(1i32);
        (*h).mb.i_mb_type_top = -(1i32);
        (*h).mb.i_mb_type_left[1usize] = -(1i32);
        (*h).mb.i_mb_type_left[0usize] = (*h).mb.i_mb_type_left[1usize];
        (*h).mb.i_mb_type_topleft = -(1i32);
        (*h).mb.i_mb_type_topright = -(1i32);
        (*h).mb.left_index_table = (&raw const left_indices
            as *const crate::src::common::common::x264_left_table_t)
            .offset(3isize);
        (*h).mb.topleft_partition = 0i32;
        let mut topleft_y = top_y;
        let mut topright_y = top_y;
        left[1usize] = (*h).mb.i_mb_xy - 1i32;
        left[0usize] = left[1usize];
        (*h).mb.left_b8[1usize] = (*h).mb.i_b8_xy - 2i32;
        (*h).mb.left_b8[0usize] = (*h).mb.left_b8[1usize];
        (*h).mb.left_b4[1usize] = (*h).mb.i_b4_xy - 4i32;
        (*h).mb.left_b4[0usize] = (*h).mb.left_b4[1usize];
        if b_interlaced != 0 {
            (*h).mb.i_mb_top_mbpair_xy = (*h).mb.i_mb_xy - 2i32 * (*h).mb.i_mb_stride;
            (*h).mb.i_mb_topleft_y = -(1i32);
            (*h).mb.i_mb_topright_y = -(1i32);
            if mb_y & 1i32 != 0 {
                if mb_x != 0
                    && mb_interlaced
                        != *(*h).mb.field.offset(((*h).mb.i_mb_xy - 1i32) as isize)
                            as ::core::ffi::c_int
                {
                    left[1usize] = (*h).mb.i_mb_xy - 1i32 - (*h).mb.i_mb_stride;
                    left[0usize] = left[1usize];
                    (*h).mb.left_b8[1usize] = (*h).mb.i_b8_xy - 2i32 - 2i32 * (*h).mb.i_b8_stride;
                    (*h).mb.left_b8[0usize] = (*h).mb.left_b8[1usize];
                    (*h).mb.left_b4[1usize] = (*h).mb.i_b4_xy - 4i32 - 4i32 * (*h).mb.i_b4_stride;
                    (*h).mb.left_b4[0usize] = (*h).mb.left_b4[1usize];
                    if mb_interlaced != 0 {
                        (*h).mb.left_index_table = (&raw const left_indices
                            as *const crate::src::common::common::x264_left_table_t)
                            .offset(2isize);
                        left[1usize] += (*h).mb.i_mb_stride;
                        (*h).mb.left_b8[1usize] += 2i32 * (*h).mb.i_b8_stride;
                        (*h).mb.left_b4[1usize] += 4i32 * (*h).mb.i_b4_stride;
                    } else {
                        (*h).mb.left_index_table = (&raw const left_indices
                            as *const crate::src::common::common::x264_left_table_t)
                            .offset(1isize);
                        topleft_y += 1;
                        (*h).mb.topleft_partition = 1i32;
                    }
                }
                if mb_interlaced == 0 {
                    topright_y = -(1i32);
                }
            } else {
                if mb_interlaced != 0 && top >= 0i32 {
                    if *(*h).mb.field.offset(top as isize) == 0 {
                        top += (*h).mb.i_mb_stride;
                        top_y += 1;
                    }
                    if mb_x != 0 {
                        topleft_y += (*(*h)
                            .mb
                            .field
                            .offset(((*h).mb.i_mb_stride * topleft_y + mb_x - 1i32) as isize)
                            == 0) as ::core::ffi::c_int;
                    }
                    if mb_x < (*h).mb.i_mb_width - 1i32 {
                        topright_y += (*(*h)
                            .mb
                            .field
                            .offset(((*h).mb.i_mb_stride * topright_y + mb_x + 1i32) as isize)
                            == 0) as ::core::ffi::c_int;
                    }
                }
                if mb_x != 0
                    && mb_interlaced
                        != *(*h).mb.field.offset(((*h).mb.i_mb_xy - 1i32) as isize)
                            as ::core::ffi::c_int
                {
                    if mb_interlaced != 0 {
                        (*h).mb.left_index_table = (&raw const left_indices
                            as *const crate::src::common::common::x264_left_table_t)
                            .offset(2isize);
                        left[1usize] += (*h).mb.i_mb_stride;
                        (*h).mb.left_b8[1usize] += 2i32 * (*h).mb.i_b8_stride;
                        (*h).mb.left_b4[1usize] += 4i32 * (*h).mb.i_b4_stride;
                    } else {
                        (*h).mb.left_index_table = (&raw const left_indices
                            as *const crate::src::common::common::x264_left_table_t)
                            .offset(0isize);
                    }
                }
            }
        }
        if mb_x > 0i32 {
            (*h).mb.i_neighbour_frame |= crate::src::common::macroblock::MB_LEFT;
            (*h).mb.i_mb_left_xy[0usize] = left[0usize];
            (*h).mb.i_mb_left_xy[1usize] = left[1usize];
            (*h).mb.i_mb_type_left[0usize] =
                *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[0usize] as isize) as ::core::ffi::c_int;
            (*h).mb.i_mb_type_left[1usize] =
                *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[1usize] as isize) as ::core::ffi::c_int;
            if *(*h).mb.slice_table.offset(left[0usize] as isize) == (*h).sh.i_first_mb {
                (*h).mb.i_neighbour |= crate::src::common::macroblock::MB_LEFT;
                if !(*h).param.constrained_intra
                    || ((*h).mb.i_mb_type_left[0usize]
                        == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_left[0usize]
                            == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_left[0usize]
                            == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_left[0usize]
                            == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                {
                    (*h).mb.i_neighbour_intra |= crate::src::common::macroblock::MB_LEFT;
                }
            }
        }
        if (*h).i_threadslice_start >> mb_interlaced != mb_y >> mb_interlaced {
            if top >= 0i32 {
                (*h).mb.i_neighbour_frame |= crate::src::common::macroblock::MB_TOP;
                (*h).mb.i_mb_top_xy = top;
                (*h).mb.i_mb_top_y = top_y;
                (*h).mb.i_mb_type_top =
                    *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int;
                if *(*h).mb.slice_table.offset(top as isize) == (*h).sh.i_first_mb {
                    (*h).mb.i_neighbour |= crate::src::common::macroblock::MB_TOP;
                    if !(*h).param.constrained_intra
                        || ((*h).mb.i_mb_type_top
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_top
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_top
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_top
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    {
                        (*h).mb.i_neighbour_intra |= crate::src::common::macroblock::MB_TOP;
                    }
                }
            }
            if mb_x > 0i32 && topleft_y >= 0i32 {
                (*h).mb.i_neighbour_frame |= crate::src::common::macroblock::MB_TOPLEFT;
                (*h).mb.i_mb_topleft_xy = (*h).mb.i_mb_stride * topleft_y + mb_x - 1i32;
                (*h).mb.i_mb_topleft_y = topleft_y;
                (*h).mb.i_mb_type_topleft =
                    *(*h).mb.type_0.offset((*h).mb.i_mb_topleft_xy as isize) as ::core::ffi::c_int;
                if *(*h).mb.slice_table.offset((*h).mb.i_mb_topleft_xy as isize)
                    == (*h).sh.i_first_mb
                {
                    (*h).mb.i_neighbour |= crate::src::common::macroblock::MB_TOPLEFT;
                    if !(*h).param.constrained_intra
                        || ((*h).mb.i_mb_type_topleft
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topleft
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topleft
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topleft
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    {
                        (*h).mb.i_neighbour_intra |= crate::src::common::macroblock::MB_TOPLEFT;
                    }
                }
            }
            if mb_x < (*h).mb.i_mb_width - 1i32 && topright_y >= 0i32 {
                (*h).mb.i_neighbour_frame |= crate::src::common::macroblock::MB_TOPRIGHT;
                (*h).mb.i_mb_topright_xy = (*h).mb.i_mb_stride * topright_y + mb_x + 1i32;
                (*h).mb.i_mb_topright_y = topright_y;
                (*h).mb.i_mb_type_topright =
                    *(*h).mb.type_0.offset((*h).mb.i_mb_topright_xy as isize) as ::core::ffi::c_int;
                if *(*h)
                    .mb
                    .slice_table
                    .offset((*h).mb.i_mb_topright_xy as isize)
                    == (*h).sh.i_first_mb
                {
                    (*h).mb.i_neighbour |= crate::src::common::macroblock::MB_TOPRIGHT;
                    if !(*h).param.constrained_intra
                        || ((*h).mb.i_mb_type_topright
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topright
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topright
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topright
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    {
                        (*h).mb.i_neighbour_intra |= crate::src::common::macroblock::MB_TOPRIGHT;
                    }
                }
            }
        }
    }
}
pub const LTOP: ::core::ffi::c_int = 0i32;
pub const LBOT: ::core::ffi::c_int = 1i32;
#[inline(always)]
unsafe extern "C" fn macroblock_cache_load(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut b_mbaff: ::core::ffi::c_int,
) {
    unsafe {
        let mut l_0 = 0i32;
        macroblock_cache_load_neighbours(h, mb_x, mb_y, b_mbaff);
        let mut left = &raw mut (*h).mb.i_mb_left_xy as *mut ::core::ffi::c_int;
        let mut top = (*h).mb.i_mb_top_xy;
        let mut top_y = (*h).mb.i_mb_top_y;
        let mut s8x8 = (*h).mb.i_b8_stride;
        let mut s4x4 = (*h).mb.i_b4_stride;
        let mut top_8x8 = (2i32 * top_y + 1i32) * s8x8 + 2i32 * mb_x;
        let mut top_4x4 = (4i32 * top_y + 3i32) * s4x4 + 4i32 * mb_x;
        let mut lists = (1i32) << (*h).sh.i_type & 3i32;
        let mut i4x4 = (*h).mb.intra4x4_pred_mode;
        let mut nnz = (*h).mb.non_zero_count;
        let mut cbp = (*h).mb.cbp;
        let mut left_index_table = (*h).mb.left_index_table;
        (*h).mb.cache.deblock_strength = &raw mut *(*(&raw mut (*h).deblock_strength
            as *mut *mut [[[crate::stdlib::uint8_t; 4]; 8]; 2])
            .offset((mb_y & 1i32) as isize))
        .offset(
            (if (*h).param.sliced_threads {
                (*h).mb.i_mb_xy
            } else {
                mb_x
            }) as isize,
        ) as *mut [[crate::stdlib::uint8_t; 4]; 8];
        if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0 {
            (*h).mb.cache.i_cbp_top = *cbp.offset(top as isize) as ::core::ffi::c_int;
            (*((&raw mut (*h).mb.cache.intra4x4_pred_mode as *mut crate::stdlib::int8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                    as ::core::ffi::c_int
                    - 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut *i4x4.offset(top as isize) as *mut crate::stdlib::int8_t)
                .offset(0isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                    as ::core::ffi::c_int
                    - 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut *nnz.offset(top as isize) as *mut crate::stdlib::uint8_t)
                .offset(12isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(16isize)
                    as ::core::ffi::c_int
                    - 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut *nnz.offset(top as isize) as *mut crate::stdlib::uint8_t).offset(
                (16i32 - 4i32
                    + (16i32
                        >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                            as ::core::ffi::c_int)) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(32isize)
                    as ::core::ffi::c_int
                    - 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut *nnz.offset(top as isize) as *mut crate::stdlib::uint8_t).offset(
                (32i32 - 4i32
                    + (16i32
                        >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                            as ::core::ffi::c_int)) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i;
        } else {
            (*h).mb.cache.i_cbp_top = -(1i32);
            (*((&raw mut (*h).mb.cache.intra4x4_pred_mode as *mut crate::stdlib::int8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                    as ::core::ffi::c_int
                    - 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0xffffffffu32;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                    as ::core::ffi::c_int
                    - 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0x80808080u32;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(16isize)
                    as ::core::ffi::c_int
                    - 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0x80808080u32;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(32isize)
                    as ::core::ffi::c_int
                    - 8i32) as isize,
            ) as *mut crate::src::common::base::x264_union32_t))
                .i = 0x80808080u32;
        }
        if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0 {
            let mut ltop = *left.offset(LTOP as isize);
            let mut lbot = if b_mbaff != 0 {
                *left.offset(LBOT as isize)
            } else {
                ltop
            };
            if b_mbaff != 0 {
                let top_luma = (*cbp.offset(ltop as isize) as ::core::ffi::c_int
                    >> ((*left_index_table).mv[0usize] as ::core::ffi::c_int & !(1i32))
                    & 2i32) as crate::stdlib::int16_t;
                let bot_luma = (*cbp.offset(lbot as isize) as ::core::ffi::c_int
                    >> ((*left_index_table).mv[2usize] as ::core::ffi::c_int & !(1i32))
                    & 2i32) as crate::stdlib::int16_t;
                (*h).mb.cache.i_cbp_left = *cbp.offset(ltop as isize) as ::core::ffi::c_int
                    & 0xfff0i32
                    | (bot_luma as ::core::ffi::c_int) << 2i32
                    | top_luma as ::core::ffi::c_int;
            } else {
                (*h).mb.cache.i_cbp_left = *cbp.offset(ltop as isize) as ::core::ffi::c_int;
            }
            (*h).mb.cache.intra4x4_pred_mode
                [(x264_scan8[0usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*i4x4.offset(ltop as isize))[(*left_index_table).intra[0usize] as usize];
            (*h).mb.cache.intra4x4_pred_mode
                [(x264_scan8[2usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*i4x4.offset(ltop as isize))[(*left_index_table).intra[1usize] as usize];
            (*h).mb.cache.intra4x4_pred_mode
                [(x264_scan8[8usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*i4x4.offset(lbot as isize))[(*left_index_table).intra[2usize] as usize];
            (*h).mb.cache.intra4x4_pred_mode
                [(x264_scan8[10usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*i4x4.offset(lbot as isize))[(*left_index_table).intra[3usize] as usize];
            (*h).mb.cache.non_zero_count
                [(x264_scan8[0usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*nnz.offset(ltop as isize))[(*left_index_table).nnz[0usize] as usize];
            (*h).mb.cache.non_zero_count
                [(x264_scan8[2usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*nnz.offset(ltop as isize))[(*left_index_table).nnz[1usize] as usize];
            (*h).mb.cache.non_zero_count
                [(x264_scan8[8usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*nnz.offset(lbot as isize))[(*left_index_table).nnz[2usize] as usize];
            (*h).mb.cache.non_zero_count
                [(x264_scan8[10usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*nnz.offset(lbot as isize))[(*left_index_table).nnz[3usize] as usize];
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                >= crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
            {
                let mut offset = (4i32
                    >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        || crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int)
                        as ::core::ffi::c_int)
                    - 4i32;
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(16i32 + 0i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*nnz.offset(ltop as isize))[((*left_index_table).nnz[0usize]
                        as ::core::ffi::c_int
                        + 16i32
                        + offset) as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(16i32 + 2i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*nnz.offset(ltop as isize))[((*left_index_table).nnz[1usize]
                        as ::core::ffi::c_int
                        + 16i32
                        + offset) as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(16i32 + 8i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*nnz.offset(lbot as isize))[((*left_index_table).nnz[2usize]
                        as ::core::ffi::c_int
                        + 16i32
                        + offset) as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 + 10i32) as usize]
                    as ::core::ffi::c_int
                    - 1i32) as usize] =
                    (*nnz.offset(lbot as isize))[((*left_index_table).nnz[3usize]
                        as ::core::ffi::c_int
                        + 16i32
                        + offset) as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(32i32 + 0i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*nnz.offset(ltop as isize))[((*left_index_table).nnz[0usize]
                        as ::core::ffi::c_int
                        + 32i32
                        + offset) as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(32i32 + 2i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*nnz.offset(ltop as isize))[((*left_index_table).nnz[1usize]
                        as ::core::ffi::c_int
                        + 32i32
                        + offset) as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(32i32 + 8i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*nnz.offset(lbot as isize))[((*left_index_table).nnz[2usize]
                        as ::core::ffi::c_int
                        + 32i32
                        + offset) as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8[(32i32 + 10i32) as usize]
                    as ::core::ffi::c_int
                    - 1i32) as usize] =
                    (*nnz.offset(lbot as isize))[((*left_index_table).nnz[3usize]
                        as ::core::ffi::c_int
                        + 32i32
                        + offset) as usize];
            } else {
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(16i32 + 0i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*nnz.offset(ltop as isize))[(*left_index_table).nnz_chroma[0usize] as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(16i32 + 2i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*nnz.offset(lbot as isize))[(*left_index_table).nnz_chroma[1usize] as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(32i32 + 0i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*nnz.offset(ltop as isize))[(*left_index_table).nnz_chroma[2usize] as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(32i32 + 2i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*nnz.offset(lbot as isize))[(*left_index_table).nnz_chroma[3usize] as usize];
            }
        } else {
            (*h).mb.cache.i_cbp_left = -(1i32);
            (*h).mb.cache.intra4x4_pred_mode
                [(x264_scan8[10usize] as ::core::ffi::c_int - 1i32) as usize] = -1i8;
            (*h).mb.cache.intra4x4_pred_mode
                [(x264_scan8[8usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*h).mb.cache.intra4x4_pred_mode
                    [(x264_scan8[10usize] as ::core::ffi::c_int - 1i32) as usize];
            (*h).mb.cache.intra4x4_pred_mode
                [(x264_scan8[2usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*h).mb.cache.intra4x4_pred_mode
                    [(x264_scan8[8usize] as ::core::ffi::c_int - 1i32) as usize];
            (*h).mb.cache.intra4x4_pred_mode
                [(x264_scan8[0usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*h).mb.cache.intra4x4_pred_mode
                    [(x264_scan8[2usize] as ::core::ffi::c_int - 1i32) as usize];
            (*h).mb.cache.non_zero_count
                [(x264_scan8[(32i32 + 2i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                0x80u8;
            (*h).mb.cache.non_zero_count
                [(x264_scan8[(32i32 + 0i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(32i32 + 2i32) as usize] as ::core::ffi::c_int - 1i32) as usize];
            (*h).mb.cache.non_zero_count
                [(x264_scan8[(16i32 + 2i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(32i32 + 0i32) as usize] as ::core::ffi::c_int - 1i32) as usize];
            (*h).mb.cache.non_zero_count
                [(x264_scan8[(16i32 + 0i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(16i32 + 2i32) as usize] as ::core::ffi::c_int - 1i32) as usize];
            (*h).mb.cache.non_zero_count
                [(x264_scan8[10usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(16i32 + 0i32) as usize] as ::core::ffi::c_int - 1i32) as usize];
            (*h).mb.cache.non_zero_count
                [(x264_scan8[8usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[10usize] as ::core::ffi::c_int - 1i32) as usize];
            (*h).mb.cache.non_zero_count
                [(x264_scan8[2usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[8usize] as ::core::ffi::c_int - 1i32) as usize];
            (*h).mb.cache.non_zero_count
                [(x264_scan8[0usize] as ::core::ffi::c_int - 1i32) as usize] =
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[2usize] as ::core::ffi::c_int - 1i32) as usize];
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                >= crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
            {
                (*h).mb.cache.non_zero_count[(x264_scan8[(32i32 + 10i32) as usize]
                    as ::core::ffi::c_int
                    - 1i32) as usize] = 0x80u8;
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(32i32 + 8i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*h).mb.cache.non_zero_count[(x264_scan8[(32i32 + 10i32) as usize]
                        as ::core::ffi::c_int
                        - 1i32) as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 + 10i32) as usize]
                    as ::core::ffi::c_int
                    - 1i32) as usize] = (*h).mb.cache.non_zero_count
                    [(x264_scan8[(32i32 + 8i32) as usize] as ::core::ffi::c_int - 1i32) as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[(16i32 + 8i32) as usize] as ::core::ffi::c_int - 1i32) as usize] =
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16i32 + 10i32) as usize]
                        as ::core::ffi::c_int
                        - 1i32) as usize];
            }
        }
        if (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t)).transform_8x8_mode {
            (*h).mb.cache.i_neighbour_transform_size =
                ((*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset(*left.offset(0isize) as isize)
                        as ::core::ffi::c_int
                        != 0) as ::core::ffi::c_int
                    + ((*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0
                        && *(*h).mb.mb_transform_size.offset(top as isize) as ::core::ffi::c_int
                            != 0) as ::core::ffi::c_int;
        }
        if b_mbaff != 0 {
            (*h).mb.pic.i_fref[0usize] =
                (*h).i_ref[0usize] << (*h).mb.interlaced as ::core::ffi::c_int;
            (*h).mb.pic.i_fref[1usize] =
                (*h).i_ref[1usize] << (*h).mb.interlaced as ::core::ffi::c_int;
        }
        if b_mbaff == 0 {
            x264_8_copy_column8(
                (*h).mb.pic.p_fdec[0usize]
                    .offset(-(1isize))
                    .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                (*h).mb.pic.p_fdec[0usize]
                    .offset(15isize)
                    .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
            );
            x264_8_copy_column8(
                (*h).mb.pic.p_fdec[0usize]
                    .offset(-(1isize))
                    .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                (*h).mb.pic.p_fdec[0usize]
                    .offset(15isize)
                    .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
            );
            macroblock_load_pic_pointers(h, mb_x, mb_y, 0i32, 0i32, 0i32);
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[1usize]
                        .offset(-(1isize))
                        .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                    (*h).mb.pic.p_fdec[1usize]
                        .offset(15isize)
                        .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                );
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[1usize]
                        .offset(-(1isize))
                        .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                    (*h).mb.pic.p_fdec[1usize]
                        .offset(15isize)
                        .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                );
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[2usize]
                        .offset(-(1isize))
                        .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                    (*h).mb.pic.p_fdec[2usize]
                        .offset(15isize)
                        .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                );
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[2usize]
                        .offset(-(1isize))
                        .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                    (*h).mb.pic.p_fdec[2usize]
                        .offset(15isize)
                        .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                );
                macroblock_load_pic_pointers(h, mb_x, mb_y, 1i32, 0i32, 0i32);
                macroblock_load_pic_pointers(h, mb_x, mb_y, 2i32, 0i32, 0i32);
            } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[1usize]
                        .offset(-(1isize))
                        .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                    (*h).mb.pic.p_fdec[1usize]
                        .offset(7isize)
                        .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                );
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[2usize]
                        .offset(-(1isize))
                        .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                    (*h).mb.pic.p_fdec[2usize]
                        .offset(7isize)
                        .offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                );
                if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                {
                    x264_8_copy_column8(
                        (*h).mb.pic.p_fdec[1usize]
                            .offset(-(1isize))
                            .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                        (*h).mb.pic.p_fdec[1usize]
                            .offset(7isize)
                            .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                    );
                    x264_8_copy_column8(
                        (*h).mb.pic.p_fdec[2usize]
                            .offset(-(1isize))
                            .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                        (*h).mb.pic.p_fdec[2usize]
                            .offset(7isize)
                            .offset((12i32 * crate::src::common::common::FDEC_STRIDE) as isize),
                    );
                }
                macroblock_load_pic_pointers(h, mb_x, mb_y, 1i32, 1i32, 0i32);
            }
        } else {
            macroblock_load_pic_pointers(h, mb_x, mb_y, 0i32, 0i32, 1i32);
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                macroblock_load_pic_pointers(h, mb_x, mb_y, 1i32, 0i32, 1i32);
                macroblock_load_pic_pointers(h, mb_x, mb_y, 2i32, 0i32, 1i32);
            } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                macroblock_load_pic_pointers(h, mb_x, mb_y, 1i32, 1i32, 1i32);
            }
        }
        if !(*(*h).fdec).integral.is_null() {
            let mut list = 0i32;
            let mut offset_0 = 16i32 * (mb_x + mb_y * (*(*h).fdec).i_stride[0usize]);
            while list < 2i32 {
                let mut i = 0i32;
                while i < (*h).mb.pic.i_fref[list as usize] {
                    (*h).mb.pic.p_integral[list as usize][i as usize] =
                        (**(&raw mut *(&raw mut (*h).fref
                            as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                            .offset(list as isize)
                            as *mut *mut crate::src::common::frame::x264_frame_t)
                            .offset(i as isize))
                        .integral
                        .offset(offset_0 as isize);
                    i += 1;
                }
                list += 1;
            }
        }
        x264_8_prefetch_fenc(h, (*h).fenc, mb_x, mb_y);
        while l_0 < lists {
            let mut mv = (*h).mb.mv[l_0 as usize];
            let mut ref_0 = (*h).mb.ref_0[l_0 as usize];
            let mut i8 = x264_scan8[0usize] as ::core::ffi::c_int - 1i32 - 1i32 * 8i32;
            if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOPLEFT != 0 {
                let mut ir = if b_mbaff != 0 {
                    2i32 * (s8x8 * (*h).mb.i_mb_topleft_y + mb_x - 1i32) + 1i32 + s8x8
                } else {
                    top_8x8 - 1i32
                };
                let mut iv = if b_mbaff != 0 {
                    4i32 * (s4x4 * (*h).mb.i_mb_topleft_y + mb_x - 1i32) + 3i32 + 3i32 * s4x4
                } else {
                    top_4x4 - 1i32
                };
                if b_mbaff != 0 && (*h).mb.topleft_partition != 0 {
                    iv -= 2i32 * s4x4;
                    ir -= s8x8;
                }
                (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = *ref_0.offset(ir as isize);
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l_0 as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(i8 as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*(&raw mut *mv.offset(iv as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
            } else {
                (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = -2i8;
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l_0 as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(i8 as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = 0u32;
            }
            i8 = x264_scan8[0usize] as ::core::ffi::c_int - 8i32;
            if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0 {
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 1i32) as usize] =
                    *ref_0.offset((top_8x8 + 0i32) as isize);
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 0i32) as usize] =
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 1i32) as usize];
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 3i32) as usize] =
                    *ref_0.offset((top_8x8 + 1i32) as isize);
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 2i32) as usize] =
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 3i32) as usize];
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l_0 as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(i8 as isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = (*(&raw mut *mv.offset(top_4x4 as isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i;
            } else {
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l_0 as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(i8 as isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                (*((&raw mut *(&raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40])
                    .offset(l_0 as isize) as *mut crate::stdlib::int8_t)
                    .offset(i8 as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (-(2i32) as crate::stdlib::uint8_t as ::core::ffi::c_uint)
                    .wrapping_mul(0x1010101u32);
            }
            i8 = x264_scan8[0usize] as ::core::ffi::c_int + 4i32 - 1i32 * 8i32;
            if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOPRIGHT != 0 {
                let mut ir_0 = if b_mbaff != 0 {
                    2i32 * (s8x8 * (*h).mb.i_mb_topright_y + (mb_x + 1i32)) + s8x8
                } else {
                    top_8x8 + 2i32
                };
                let mut iv_0 = if b_mbaff != 0 {
                    4i32 * (s4x4 * (*h).mb.i_mb_topright_y + (mb_x + 1i32)) + 3i32 * s4x4
                } else {
                    top_4x4 + 4i32
                };
                (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = *ref_0.offset(ir_0 as isize);
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l_0 as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(i8 as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*(&raw mut *mv.offset(iv_0 as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
            } else {
                (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = -2i8;
            }
            i8 = x264_scan8[0usize] as ::core::ffi::c_int - 1i32;
            if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0 {
                if b_mbaff != 0 {
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 0i32 * 8i32) as usize] = *ref_0.offset(
                        ((*h).mb.left_b8[LTOP as usize]
                            + 1i32
                            + s8x8 * (*left_index_table).ref_0[0usize] as ::core::ffi::c_int)
                            as isize,
                    );
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 1i32 * 8i32) as usize] = *ref_0.offset(
                        ((*h).mb.left_b8[LTOP as usize]
                            + 1i32
                            + s8x8 * (*left_index_table).ref_0[1usize] as ::core::ffi::c_int)
                            as isize,
                    );
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 2i32 * 8i32) as usize] = *ref_0.offset(
                        ((*h).mb.left_b8[LBOT as usize]
                            + 1i32
                            + s8x8 * (*left_index_table).ref_0[2usize] as ::core::ffi::c_int)
                            as isize,
                    );
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 3i32 * 8i32) as usize] = *ref_0.offset(
                        ((*h).mb.left_b8[LBOT as usize]
                            + 1i32
                            + s8x8 * (*left_index_table).ref_0[3usize] as ::core::ffi::c_int)
                            as isize,
                    );
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 0i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(0isize)
                            + 3i32
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(0isize)
                                    as ::core::ffi::c_int) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 1i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(0isize)
                            + 3i32
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(1isize)
                                    as ::core::ffi::c_int) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 2i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(1isize)
                            + 3i32
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(2isize)
                                    as ::core::ffi::c_int) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 3i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(1isize)
                            + 3i32
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(3isize)
                                    as ::core::ffi::c_int) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                } else {
                    let ir_1 = (*h).mb.i_b8_xy - 1i32;
                    let iv_1 = (*h).mb.i_b4_xy - 1i32;
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 1i32 * 8i32) as usize] =
                        *ref_0.offset((ir_1 + 0i32 * s8x8) as isize);
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 0i32 * 8i32) as usize] =
                        (*h).mb.cache.ref_0[l_0 as usize][(i8 + 1i32 * 8i32) as usize];
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 3i32 * 8i32) as usize] =
                        *ref_0.offset((ir_1 + 1i32 * s8x8) as isize);
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 2i32 * 8i32) as usize] =
                        (*h).mb.cache.ref_0[l_0 as usize][(i8 + 3i32 * 8i32) as usize];
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 0i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset((iv_1 + 0i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 1i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset((iv_1 + 1i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 2i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset((iv_1 + 2i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 3i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset((iv_1 + 3i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                }
            } else {
                let mut i_0 = 0i32;
                while i_0 < 4i32 {
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + i_0 * 8i32) as usize] = -2i8;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + i_0 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0u32;
                    i_0 += 1;
                }
            }
            if b_mbaff != 0 && (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0 {
                if (*h).mb.interlaced
                    && *(*h).mb.field.offset(((*h).mb.i_mb_xy - 1i32) as isize) == 0
                {
                    (*h).mb.cache.topright_ref[l_0 as usize][0usize] =
                        *ref_0.offset(((*h).mb.left_b8[0usize] + 1i32 + s8x8 * 0i32) as isize);
                    (*h).mb.cache.topright_ref[l_0 as usize][1usize] =
                        *ref_0.offset(((*h).mb.left_b8[0usize] + 1i32 + s8x8 * 1i32) as isize);
                    (*h).mb.cache.topright_ref[l_0 as usize][2usize] =
                        *ref_0.offset(((*h).mb.left_b8[1usize] + 1i32 + s8x8 * 0i32) as isize);
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(0isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(0isize)
                            + 3i32
                            + s4x4
                                * (*(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(0isize)
                                    as ::core::ffi::c_int
                                    + 1i32)) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(1isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(0isize)
                            + 3i32
                            + s4x4
                                * (*(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(1isize)
                                    as ::core::ffi::c_int
                                    + 1i32)) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(2isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(1isize)
                            + 3i32
                            + s4x4
                                * (*(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(2isize)
                                    as ::core::ffi::c_int
                                    + 1i32)) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                } else if !(*h).mb.interlaced
                    && *(*h).mb.field.offset(((*h).mb.i_mb_xy - 1i32) as isize)
                        as ::core::ffi::c_int
                        != 0
                {
                    (*h).mb.cache.topright_ref[l_0 as usize][0usize] = *ref_0.offset(
                        ((*h).mb.left_b8[0usize]
                            + 1i32
                            + s8x8 * 2i32
                            + s8x8 * (*left_index_table).ref_0[0usize] as ::core::ffi::c_int)
                            as isize,
                    );
                    (*h).mb.cache.topright_ref[l_0 as usize][1usize] = *ref_0.offset(
                        ((*h).mb.left_b8[0usize]
                            + 1i32
                            + s8x8 * 2i32
                            + s8x8 * (*left_index_table).ref_0[1usize] as ::core::ffi::c_int)
                            as isize,
                    );
                    (*h).mb.cache.topright_ref[l_0 as usize][2usize] = *ref_0.offset(
                        ((*h).mb.left_b8[0usize]
                            + 1i32
                            + s8x8 * 2i32
                            + s8x8 * (*left_index_table).ref_0[2usize] as ::core::ffi::c_int)
                            as isize,
                    );
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(0isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(0isize)
                            + 3i32
                            + s4x4 * 4i32
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(0isize)
                                    as ::core::ffi::c_int) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(1isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(0isize)
                            + 3i32
                            + s4x4 * 4i32
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(1isize)
                                    as ::core::ffi::c_int) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(2isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(0isize)
                            + 3i32
                            + s4x4 * 4i32
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(2isize)
                                    as ::core::ffi::c_int) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                }
            }
            if (*h).param.cabac {
                let mut mvd = (*h).mb.mvd[l_0 as usize];
                if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0 {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0isize) as ::core::ffi::c_int
                                - 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = (*(&raw mut *(&raw mut *mvd.offset(top as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(0isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i;
                } else {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0isize) as ::core::ffi::c_int
                                - 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = 0u64;
                }
                if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
                    && (b_mbaff == 0
                        || (*h).mb.cache.ref_0[l_0 as usize]
                            [(x264_scan8[0usize] as ::core::ffi::c_int - 1i32) as usize]
                            as ::core::ffi::c_int
                            >= 0i32)
                {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0isize) as ::core::ffi::c_int
                                - 1i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *mvd.offset(*left.offset(0isize) as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const (*left_index_table).intra as *const crate::stdlib::uint8_t)
                                .offset(0isize) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(2isize) as ::core::ffi::c_int
                                - 1i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *mvd.offset(*left.offset(0isize) as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const (*left_index_table).intra as *const crate::stdlib::uint8_t)
                                .offset(1isize) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                } else {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0isize) as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = 0u16;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0isize) as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = 0u16;
                }
                if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
                    && (b_mbaff == 0
                        || (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            - 1i32
                            + 2i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32)
                {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(8isize) as ::core::ffi::c_int
                                - 1i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *mvd.offset(*left.offset(1isize) as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const (*left_index_table).intra as *const crate::stdlib::uint8_t)
                                .offset(2isize) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(10isize) as ::core::ffi::c_int
                                - 1i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *mvd.offset(*left.offset(1isize) as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const (*left_index_table).intra as *const crate::stdlib::uint8_t)
                                .offset(3isize) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                } else {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0isize) as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = 0u16;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0isize) as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = 0u16;
                }
            }
            if b_mbaff != 0 {
                if (*h).mb.interlaced {
                    if (*h).mb.i_mb_topleft_xy >= 0i32
                        && *(*h).mb.field.offset((*h).mb.i_mb_topleft_xy as isize) == 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            - 1i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                / 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if top >= 0i32 && *(*h).mb.field.offset(top as isize) == 0 {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            + 0i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                / 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            + 1i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                / 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            + 2i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                / 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            + 3i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                / 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if (*h).mb.i_mb_topright_xy >= 0i32
                        && *(*h).mb.field.offset((*h).mb.i_mb_topright_xy as isize) == 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            + 4i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                / 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if *left.offset(0isize) >= 0i32
                        && *(*h).mb.field.offset(*left.offset(0isize) as isize) == 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            - 1i32
                            + 0i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                / 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            - 1i32
                            + 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                / 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            - 1i32
                            + 2i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                / 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            - 1i32
                            + 3i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                / 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize][0usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize][0usize] =
                                (((*h).mb.cache.topright_ref[l_0 as usize][0usize]
                                    as ::core::ffi::c_int)
                                    << 1i32)
                                    as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize][0usize][1usize] =
                                ((*h).mb.cache.topright_mv[l_0 as usize][0usize][1usize]
                                    as ::core::ffi::c_int
                                    / 2i32)
                                    as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][0usize][1usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][0usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize][1usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize][1usize] =
                                (((*h).mb.cache.topright_ref[l_0 as usize][1usize]
                                    as ::core::ffi::c_int)
                                    << 1i32)
                                    as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize][1usize][1usize] =
                                ((*h).mb.cache.topright_mv[l_0 as usize][1usize][1usize]
                                    as ::core::ffi::c_int
                                    / 2i32)
                                    as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][1usize][1usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][1usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize][2usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize][2usize] =
                                (((*h).mb.cache.topright_ref[l_0 as usize][2usize]
                                    as ::core::ffi::c_int)
                                    << 1i32)
                                    as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize][2usize][1usize] =
                                ((*h).mb.cache.topright_mv[l_0 as usize][2usize][1usize]
                                    as ::core::ffi::c_int
                                    / 2i32)
                                    as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][2usize][1usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][2usize][1usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::uint8_t;
                        }
                    }
                } else {
                    if (*h).mb.i_mb_topleft_xy >= 0i32
                        && *(*h).mb.field.offset((*h).mb.i_mb_topleft_xy as isize)
                            as ::core::ffi::c_int
                            != 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            - 1i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                * 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize][1usize] = (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if top >= 0i32 && *(*h).mb.field.offset(top as isize) as ::core::ffi::c_int != 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            + 0i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                * 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize][1usize] = (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 0i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            + 1i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                * 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize][1usize] = (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 1i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            + 2i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                * 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize][1usize] = (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 2i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            + 3i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                * 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize][1usize] = (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 3i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if (*h).mb.i_mb_topright_xy >= 0i32
                        && *(*h).mb.field.offset((*h).mb.i_mb_topright_xy as isize)
                            as ::core::ffi::c_int
                            != 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            + 4i32
                            - 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                * 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize][1usize] = (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                + 4i32
                                - 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if *left.offset(0isize) >= 0i32
                        && *(*h).mb.field.offset(*left.offset(0isize) as isize)
                            as ::core::ffi::c_int
                            != 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            - 1i32
                            + 0i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                * 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize][1usize] = (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 0i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            - 1i32
                            + 1i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                * 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize][1usize] = (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 1i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            - 1i32
                            + 2i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                * 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize][1usize] = (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 2i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                            as ::core::ffi::c_int
                            - 1i32
                            + 3i32 * 8i32)
                            as usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1i32)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize][1usize] = ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int
                                * 2i32)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8[0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize][1usize] = (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0usize]
                                as ::core::ffi::c_int
                                - 1i32
                                + 3i32 * 8i32)
                                as usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize][0usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize][0usize] =
                                ((*h).mb.cache.topright_ref[l_0 as usize][0usize]
                                    as ::core::ffi::c_int
                                    >> 1i32)
                                    as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize][0usize][1usize] =
                                ((*h).mb.cache.topright_mv[l_0 as usize][0usize][1usize]
                                    as ::core::ffi::c_int
                                    * 2i32)
                                    as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][0usize][1usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][0usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize][1usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize][1usize] =
                                ((*h).mb.cache.topright_ref[l_0 as usize][1usize]
                                    as ::core::ffi::c_int
                                    >> 1i32)
                                    as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize][1usize][1usize] =
                                ((*h).mb.cache.topright_mv[l_0 as usize][1usize][1usize]
                                    as ::core::ffi::c_int
                                    * 2i32)
                                    as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][1usize][1usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][1usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize][2usize] as ::core::ffi::c_int
                            >= 0i32
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize][2usize] =
                                ((*h).mb.cache.topright_ref[l_0 as usize][2usize]
                                    as ::core::ffi::c_int
                                    >> 1i32)
                                    as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize][2usize][1usize] =
                                ((*h).mb.cache.topright_mv[l_0 as usize][2usize][1usize]
                                    as ::core::ffi::c_int
                                    * 2i32)
                                    as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][2usize][1usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][2usize][1usize]
                                as ::core::ffi::c_int)
                                << 1i32)
                                as crate::stdlib::uint8_t;
                        }
                    }
                }
            }
            l_0 += 1;
        }
        if b_mbaff != 0 && mb_x == 0i32 && mb_y & 1i32 == 0 {
            if (*h).mb.i_mb_top_xy >= (*h).sh.i_first_mb {
                (*h).mb.field_decoding_flag =
                    *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int;
            } else {
                (*h).mb.field_decoding_flag = 0i32;
            }
        }
        (*h).mb.allow_skip = true;
        if b_mbaff != 0 {
            if (*h).mb.interlaced as ::core::ffi::c_int != (*h).mb.field_decoding_flag
                && mb_y & 1i32 != 0
                && (*(*h)
                    .mb
                    .type_0
                    .offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
                    as ::core::ffi::c_int
                    == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                    || *(*h)
                        .mb
                        .type_0
                        .offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
                        as ::core::ffi::c_int
                        == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int)
            {
                (*h).mb.allow_skip = false;
            }
        }
        if (*h).param.cabac {
            if b_mbaff != 0 {
                let mut top_xy = 0;
                let mut mb_xy = mb_x + (mb_y & !(1i32)) * (*h).mb.i_mb_stride;
                let mut left_xy = mb_xy - 1i32;
                if mb_y & 1i32 != 0
                    && mb_x > 0i32
                    && (*h).mb.field_decoding_flag
                        == *(*h).mb.field.offset(left_xy as isize) as ::core::ffi::c_int
                {
                    left_xy += (*h).mb.i_mb_stride;
                }
                if (*h).mb.field_decoding_flag != 0 {
                    top_xy = mb_xy - (*h).mb.i_mb_stride;
                    if mb_y & 1i32 == 0
                        && top_xy >= 0i32
                        && *(*h).mb.slice_table.offset(top_xy as isize) == (*h).sh.i_first_mb
                        && *(*h).mb.field.offset(top_xy as isize) as ::core::ffi::c_int != 0
                    {
                        top_xy -= (*h).mb.i_mb_stride;
                    }
                } else {
                    top_xy = mb_x + (mb_y - 1i32) * (*h).mb.i_mb_stride;
                }
                (*h).mb.cache.i_neighbour_skip = (mb_x > 0i32
                    && *(*h).mb.slice_table.offset(left_xy as isize) == (*h).sh.i_first_mb
                    && !(*(*h).mb.type_0.offset(left_xy as isize) as ::core::ffi::c_int
                        == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                        || *(*h).mb.type_0.offset(left_xy as isize) as ::core::ffi::c_int
                            == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int))
                    as ::core::ffi::c_int
                    + (top_xy >= 0i32
                        && *(*h).mb.slice_table.offset(top_xy as isize) == (*h).sh.i_first_mb
                        && !(*(*h).mb.type_0.offset(top_xy as isize) as ::core::ffi::c_int
                            == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset(top_xy as isize) as ::core::ffi::c_int
                                == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int))
                        as ::core::ffi::c_int;
            } else {
                (*h).mb.cache.i_neighbour_skip = ((*h).mb.i_neighbour
                    & crate::src::common::macroblock::MB_LEFT
                    != 0
                    && !((*h).mb.i_mb_type_left[0usize]
                        == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_left[0usize]
                            == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int))
                    as ::core::ffi::c_int
                    + ((*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0
                        && !((*h).mb.i_mb_type_top
                            == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_top
                                == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int))
                        as ::core::ffi::c_int;
            }
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            (*h).mb.bipred_weight = &raw mut *(&raw mut *(&raw mut (*h).mb.bipred_weight_buf
                as *mut [[[crate::stdlib::int8_t; 4]; 32]; 2])
                .offset((*h).mb.interlaced as isize)
                as *mut [[crate::stdlib::int8_t; 4]; 32])
                .offset(((*h).mb.interlaced as ::core::ffi::c_int & (mb_y & 1i32)) as isize)
                as *mut [crate::stdlib::int8_t; 4];
            (*h).mb.dist_scale_factor = &raw mut *(&raw mut *(&raw mut (*h).mb.dist_scale_factor_buf
                as *mut [[[crate::stdlib::int16_t; 4]; 32]; 2])
                .offset((*h).mb.interlaced as isize)
                as *mut [[crate::stdlib::int16_t; 4]; 32])
                .offset(((*h).mb.interlaced as ::core::ffi::c_int & (mb_y & 1i32)) as isize)
                as *mut [crate::stdlib::int16_t; 4];
            if (*h).param.cabac {
                let mut skipbp = 0;
                x264_macroblock_cache_skip(h, 0i32, 0i32, 4i32, 4i32, 0i32);
                if b_mbaff != 0 {
                    skipbp = (if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
                    {
                        *(*h).mb.skipbp.offset(*left.offset(LTOP as isize) as isize)
                            as ::core::ffi::c_int
                    } else {
                        0i32
                    }) as crate::stdlib::uint8_t;
                    (*h).mb.cache.skip
                        [(x264_scan8[0usize] as ::core::ffi::c_int - 1i32) as usize] = (skipbp
                        as ::core::ffi::c_int
                        >> 1i32 + ((*left_index_table).mv[0usize] as ::core::ffi::c_int & !(1i32))
                        & 1i32)
                        as crate::stdlib::int8_t;
                    skipbp = (if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
                    {
                        *(*h).mb.skipbp.offset(*left.offset(LBOT as isize) as isize)
                            as ::core::ffi::c_int
                    } else {
                        0i32
                    }) as crate::stdlib::uint8_t;
                    (*h).mb.cache.skip
                        [(x264_scan8[8usize] as ::core::ffi::c_int - 1i32) as usize] = (skipbp
                        as ::core::ffi::c_int
                        >> 1i32 + ((*left_index_table).mv[2usize] as ::core::ffi::c_int & !(1i32))
                        & 1i32)
                        as crate::stdlib::int8_t;
                } else {
                    skipbp = (if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
                    {
                        *(*h).mb.skipbp.offset(*left.offset(0isize) as isize) as ::core::ffi::c_int
                    } else {
                        0i32
                    }) as crate::stdlib::uint8_t;
                    (*h).mb.cache.skip
                        [(x264_scan8[0usize] as ::core::ffi::c_int - 1i32) as usize] =
                        (skipbp as ::core::ffi::c_int & 0x2i32) as crate::stdlib::int8_t;
                    (*h).mb.cache.skip
                        [(x264_scan8[8usize] as ::core::ffi::c_int - 1i32) as usize] =
                        (skipbp as ::core::ffi::c_int & 0x8i32) as crate::stdlib::int8_t;
                }
                skipbp = (if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0 {
                    *(*h).mb.skipbp.offset(top as isize) as ::core::ffi::c_int
                } else {
                    0i32
                }) as crate::stdlib::uint8_t;
                (*h).mb.cache.skip[(x264_scan8[0usize] as ::core::ffi::c_int - 8i32) as usize] =
                    (skipbp as ::core::ffi::c_int & 0x4i32) as crate::stdlib::int8_t;
                (*h).mb.cache.skip[(x264_scan8[4usize] as ::core::ffi::c_int - 8i32) as usize] =
                    (skipbp as ::core::ffi::c_int & 0x8i32) as crate::stdlib::int8_t;
            }
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            crate::src::common::mvpred::x264_8_mb_predict_mv_pskip(
                h,
                &raw mut (*h).mb.cache.pskip_mv as *mut crate::stdlib::int16_t,
            );
        }
        (*h).mb.i_neighbour8[0usize] = (*h).mb.i_neighbour_intra
            & (crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                | crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            | (if (*h).mb.i_neighbour_intra & crate::src::common::macroblock::MB_TOP != 0 {
                crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int
            } else {
                0i32
            }) as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[0usize] = (*h).mb.i_neighbour8[0usize];
        (*h).mb.i_neighbour4[1usize] = (crate::src::common::macroblock::MB_LEFT
            as ::core::ffi::c_int
            | (if (*h).mb.i_neighbour_intra & crate::src::common::macroblock::MB_TOP != 0 {
                crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                    | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
                    | crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int
            } else {
                0i32
            })) as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[4usize] = (*h).mb.i_neighbour4[1usize];
        (*h).mb.i_neighbour8[2usize] = (crate::src::common::macroblock::MB_TOP
            as ::core::ffi::c_int
            | crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int
            | (if (*h).mb.i_neighbour_intra & crate::src::common::macroblock::MB_LEFT != 0 {
                crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                    | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
            } else {
                0i32
            })) as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[10usize] = (*h).mb.i_neighbour8[2usize];
        (*h).mb.i_neighbour4[8usize] = (*h).mb.i_neighbour4[10usize];
        (*h).mb.i_neighbour4[2usize] = (*h).mb.i_neighbour4[8usize];
        (*h).mb.i_neighbour8[1usize] = crate::src::common::macroblock::MB_LEFT
            | (*h).mb.i_neighbour_intra & crate::src::common::macroblock::MB_TOPRIGHT
            | (if (*h).mb.i_neighbour_intra & crate::src::common::macroblock::MB_TOP != 0 {
                crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                    | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
            } else {
                0i32
            }) as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[5usize] = (*h).mb.i_neighbour8[1usize];
    }
}
pub unsafe extern "C" fn x264_8_macroblock_cache_load_progressive(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) {
    unsafe {
        macroblock_cache_load(h, mb_x, mb_y, 0i32);
    }
}
pub unsafe extern "C" fn x264_8_macroblock_cache_load_interlaced(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) {
    unsafe {
        macroblock_cache_load(h, mb_x, mb_y, 1i32);
    }
}
unsafe extern "C" fn macroblock_deblock_strength_mbaff(
    mut h: *mut crate::src::common::common::x264_t,
    mut bs: *mut [[crate::stdlib::uint8_t; 4]; 8],
) {
    unsafe {
        if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
            && *(*h).mb.field.offset((*h).mb.i_mb_left_xy[0usize] as isize) as ::core::ffi::c_int
                != (*h).mb.interlaced as ::core::ffi::c_int
        {
            let mut tmpbs = [0; 8];
            let mut i = 0i32;
            static mut offset: [[[crate::stdlib::uint8_t; 8]; 2]; 2] = [
                [
                    [0u8, 0u8, 0u8, 0u8, 1u8, 1u8, 1u8, 1u8],
                    [2u8, 2u8, 2u8, 2u8, 3u8, 3u8, 3u8, 3u8],
                ],
                [
                    [0u8, 1u8, 2u8, 3u8, 0u8, 1u8, 2u8, 3u8],
                    [0u8, 1u8, 2u8, 3u8, 0u8, 1u8, 2u8, 3u8],
                ],
            ];
            let mut off = &raw const *(&raw const *(&raw const offset
                as *const [[crate::stdlib::uint8_t; 8]; 2])
                .offset((*h).mb.interlaced as isize)
                as *const [crate::stdlib::uint8_t; 8])
                .offset(((*h).mb.i_mb_y & 1i32) as isize)
                as *const crate::stdlib::uint8_t;
            let mut nnz = (*h).mb.non_zero_count;
            while i < 8i32 {
                let mut left = (*h).mb.i_mb_left_xy[(if (*h).mb.interlaced {
                    i >> 2i32
                } else {
                    i & 1i32
                }) as usize];
                let mut nnz_this = (*h).mb.cache.non_zero_count
                    [(x264_scan8[0usize] as ::core::ffi::c_int + 8i32 * (i >> 1i32)) as usize]
                    as ::core::ffi::c_int;
                let mut nnz_left = (*nnz.offset(left as isize))
                    [(3i32 + 4i32 * *off.offset(i as isize) as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
                if !(*h).param.cabac
                    && (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                        .transform_8x8_mode
                {
                    let mut j = *off.offset(i as isize) as ::core::ffi::c_int & !(1i32);
                    if *(*h).mb.mb_transform_size.offset(left as isize) != 0 {
                        nnz_left = ((*((&raw mut *nnz.offset(left as isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset((2i32 + 4i32 * j) as isize)
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int
                            | (*((&raw mut *nnz.offset(left as isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset((2i32 + 4i32 * (1i32 + j)) as isize)
                                as *mut crate::src::common::base::x264_union16_t))
                                .i as ::core::ffi::c_int
                            != 0) as ::core::ffi::c_int;
                    }
                }
                tmpbs[i as usize] = (if nnz_left != 0 || nnz_this != 0 {
                    2i32
                } else {
                    1i32
                }) as crate::stdlib::uint8_t;
                i += 1;
            }
            if (*h).mb.interlaced {
                (*(&raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(0isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*((&raw mut tmpbs as *mut crate::stdlib::uint8_t).offset(0isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
                (*(&raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(4isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*((&raw mut tmpbs as *mut crate::stdlib::uint8_t).offset(4isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
            } else {
                let mut i_0 = 0i32;
                let mut i_1 = 0i32;
                while i_0 < 4i32 {
                    (*bs.offset(0isize))[0usize][i_0 as usize] = tmpbs[(2i32 * i_0) as usize];
                    i_0 += 1;
                }
                while i_1 < 4i32 {
                    (*bs.offset(0isize))[4usize][i_1 as usize] =
                        tmpbs[(1i32 + 2i32 * i_1) as usize];
                    i_1 += 1;
                }
            }
        }
        if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0
            && (*h).mb.interlaced as ::core::ffi::c_int
                != *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
        {
            if (*h).mb.i_mb_y & 1i32 == 0 && !(*h).mb.interlaced {
                let mut j_0 = 0i32;
                let mut mbn_xy = (*h).mb.i_mb_xy - 2i32 * (*h).mb.i_mb_stride;
                let mut nnz_cur =
                    (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                            as isize,
                    );
                while j_0 < 2i32 {
                    let mut nnz_top = [0; 4];
                    let mut i_2 = 0i32;
                    let mut nnz_0 = (*h).mb.non_zero_count;
                    (*(&raw mut nnz_top as *mut crate::src::common::base::x264_union32_t)).i =
                        (*((&raw mut *nnz_0.offset(mbn_xy as isize) as *mut crate::stdlib::uint8_t)
                            .offset((3i32 * 4i32) as isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i;
                    if !(*h).param.cabac
                        && (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                            .transform_8x8_mode
                        && *(*h).mb.mb_transform_size.offset(mbn_xy as isize) as ::core::ffi::c_int
                            != 0
                    {
                        nnz_top[1usize] = ((*((&raw mut *nnz_0.offset(mbn_xy as isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(8isize)
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int
                            != 0
                            || (*((&raw mut *nnz_0.offset(mbn_xy as isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset(12isize)
                                as *mut crate::src::common::base::x264_union16_t))
                                .i as ::core::ffi::c_int
                                != 0)
                            as crate::stdlib::uint8_t;
                        nnz_top[0usize] = nnz_top[1usize];
                        nnz_top[3usize] = ((*((&raw mut *nnz_0.offset(mbn_xy as isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(10isize)
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int
                            != 0
                            || (*((&raw mut *nnz_0.offset(mbn_xy as isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset(14isize)
                                as *mut crate::src::common::base::x264_union16_t))
                                .i as ::core::ffi::c_int
                                != 0)
                            as crate::stdlib::uint8_t;
                        nnz_top[2usize] = nnz_top[3usize];
                    }
                    while i_2 < 4i32 {
                        (*bs.offset(1isize))[(4i32 * j_0) as usize][i_2 as usize] =
                            (if *nnz_cur.offset(i_2 as isize) as ::core::ffi::c_int != 0
                                || nnz_top[i_2 as usize] as ::core::ffi::c_int != 0
                            {
                                2i32
                            } else {
                                1i32
                            }) as crate::stdlib::uint8_t;
                        i_2 += 1;
                    }
                    j_0 += 1;
                    mbn_xy += (*h).mb.i_mb_stride;
                }
            } else {
                let mut i_3 = 0i32;
                while i_3 < 4i32 {
                    (*bs.offset(1isize))[0usize][i_3 as usize] =
                        (if (*bs.offset(1isize))[0usize][i_3 as usize] as ::core::ffi::c_int > 1i32
                        {
                            (*bs.offset(1isize))[0usize][i_3 as usize] as ::core::ffi::c_int
                        } else {
                            1i32
                        }) as crate::stdlib::uint8_t;
                    i_3 += 1;
                }
            }
        }
    }
}
pub unsafe extern "C" fn x264_8_macroblock_deblock_strength(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut neighbour_changed = 0i32;
        let mut bs = (*h).mb.cache.deblock_strength;
        if (*h).mb.i_type == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int
        {
            (*(&raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                .offset(1isize) as *mut crate::src::common::base::x264_union32_t))
                .i = 0x3030303u32;
            (*(&raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                .offset(2isize) as *mut crate::src::common::base::x264_union64_t))
                .i = 0x303030303030303u64;
            (*(&raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                .offset(1isize) as *mut crate::src::common::base::x264_union32_t))
                .i = 0x3030303u32;
            (*(&raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                .offset(2isize) as *mut crate::src::common::base::x264_union64_t))
                .i = 0x303030303030303u64;
            return;
        }
        if (*h).mb.transform_8x8
            && !(crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int)
        {
            let mut cbp_mask = 0xfi32
                >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            if (*h).mb.i_cbp_luma & cbp_mask == cbp_mask {
                (*(&raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(0isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = 0x2020202u32;
                (*(&raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = 0x2020202u32;
                (*(&raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(4isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = 0x2020202u32;
                (*(&raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(0isize)
                    as *mut crate::src::common::base::x264_union64_t))
                    .i = 0x202020202020202u64;
                (*(&raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2isize)
                    as *mut crate::src::common::base::x264_union64_t))
                    .i = 0x202020202020202u64;
                (*(&raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(4isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = 0x2020202u32;
                return;
            }
        }
        if (*h).sh.i_disable_deblocking_filter_idc != 2i32 {
            neighbour_changed =
                ((*h).mb.i_neighbour_frame & !(*h).mb.i_neighbour) as ::core::ffi::c_int;
            (*h).mb.i_neighbour = (*h).mb.i_neighbour_frame;
        }
        if (*h).sh.mbaff
            && (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0
            && *(*h).mb.field.offset(((*h).mb.i_mb_xy - 1i32) as isize) as ::core::ffi::c_int
                != (*h).mb.interlaced as ::core::ffi::c_int
        {
            (*h).mb.i_mb_left_xy[0usize] = (*h).mb.i_mb_xy - 1i32;
            (*h).mb.i_mb_left_xy[1usize] = (*h).mb.i_mb_left_xy[0usize];
            if (*h).mb.i_mb_y & 1i32 != 0 {
                (*h).mb.i_mb_left_xy[0usize] -= (*h).mb.i_mb_stride;
            } else {
                (*h).mb.i_mb_left_xy[1usize] += (*h).mb.i_mb_stride;
            }
        }
        if neighbour_changed != 0 {
            let mut l = 0i32;
            let mut top_y = (*h).mb.i_mb_top_y;
            let mut top_8x8 = (2i32 * top_y + 1i32) * (*h).mb.i_b8_stride + 2i32 * (*h).mb.i_mb_x;
            let mut top_4x4 = (4i32 * top_y + 3i32) * (*h).mb.i_b4_stride + 4i32 * (*h).mb.i_mb_x;
            let mut s8x8 = (*h).mb.i_b8_stride;
            let mut s4x4 = (*h).mb.i_b4_stride;
            let mut nnz = (*h).mb.non_zero_count;
            let mut left_index_table = if (*h).sh.mbaff {
                (*h).mb.left_index_table
            } else {
                (&raw const left_indices as *const crate::src::common::common::x264_left_table_t)
                    .offset(3isize)
            };
            if neighbour_changed & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int != 0
            {
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                        as ::core::ffi::c_int
                        - 8i32) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = (*((&raw mut *nnz.offset((*h).mb.i_mb_top_xy as isize)
                    as *mut crate::stdlib::uint8_t)
                    .offset(12isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
            }
            if neighbour_changed & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                != 0
            {
                let mut left = &raw mut (*h).mb.i_mb_left_xy as *mut ::core::ffi::c_int;
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[0usize] as ::core::ffi::c_int - 1i32) as usize] = (*nnz
                    .offset(*left.offset(0isize) as isize))
                    [(*left_index_table).nnz[0usize] as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[2usize] as ::core::ffi::c_int - 1i32) as usize] = (*nnz
                    .offset(*left.offset(0isize) as isize))
                    [(*left_index_table).nnz[1usize] as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[8usize] as ::core::ffi::c_int - 1i32) as usize] = (*nnz
                    .offset(*left.offset(1isize) as isize))
                    [(*left_index_table).nnz[2usize] as usize];
                (*h).mb.cache.non_zero_count
                    [(x264_scan8[10usize] as ::core::ffi::c_int - 1i32) as usize] = (*nnz
                    .offset(*left.offset(1isize) as isize))
                    [(*left_index_table).nnz[3usize] as usize];
            }
            while l
                <= ((*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            {
                let mut mv = (*h).mb.mv[l as usize];
                let mut ref_0 = (*h).mb.ref_0[l as usize];
                let mut i8 = x264_scan8[0usize] as ::core::ffi::c_int - 8i32;
                if neighbour_changed & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                    != 0
                {
                    (*h).mb.cache.ref_0[l as usize][(i8 + 1i32) as usize] =
                        *ref_0.offset((top_8x8 + 0i32) as isize);
                    (*h).mb.cache.ref_0[l as usize][(i8 + 0i32) as usize] =
                        (*h).mb.cache.ref_0[l as usize][(i8 + 1i32) as usize];
                    (*h).mb.cache.ref_0[l as usize][(i8 + 3i32) as usize] =
                        *ref_0.offset((top_8x8 + 1i32) as isize);
                    (*h).mb.cache.ref_0[l as usize][(i8 + 2i32) as usize] =
                        (*h).mb.cache.ref_0[l as usize][(i8 + 3i32) as usize];
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(i8 as isize)
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = (*(&raw mut *mv.offset(top_4x4 as isize)
                        as *mut crate::src::common::base::x264_union128_t))
                        .i;
                }
                i8 = x264_scan8[0usize] as ::core::ffi::c_int - 1i32;
                if neighbour_changed & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                    != 0
                {
                    (*h).mb.cache.ref_0[l as usize][(i8 + 1i32 * 8i32) as usize] = *ref_0.offset(
                        ((*h).mb.left_b8[0usize]
                            + 1i32
                            + s8x8 * (*left_index_table).ref_0[0usize] as ::core::ffi::c_int)
                            as isize,
                    );
                    (*h).mb.cache.ref_0[l as usize][(i8 + 0i32 * 8i32) as usize] =
                        (*h).mb.cache.ref_0[l as usize][(i8 + 1i32 * 8i32) as usize];
                    (*h).mb.cache.ref_0[l as usize][(i8 + 3i32 * 8i32) as usize] = *ref_0.offset(
                        ((*h).mb.left_b8[1usize]
                            + 1i32
                            + s8x8 * (*left_index_table).ref_0[2usize] as ::core::ffi::c_int)
                            as isize,
                    );
                    (*h).mb.cache.ref_0[l as usize][(i8 + 2i32 * 8i32) as usize] =
                        (*h).mb.cache.ref_0[l as usize][(i8 + 3i32 * 8i32) as usize];
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 0i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(0isize)
                            + 3i32
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(0isize)
                                    as ::core::ffi::c_int) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 1i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(0isize)
                            + 3i32
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(1isize)
                                    as ::core::ffi::c_int) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 2i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(1isize)
                            + 3i32
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(2isize)
                                    as ::core::ffi::c_int) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 3i32 * 8i32) as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int).offset(1isize)
                            + 3i32
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(3isize)
                                    as ::core::ffi::c_int) as isize,
                    )
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                }
                l += 1;
            }
        }
        if (*h).param.analyse.i_weighted_pred == crate::x264_h::X264_WEIGHTP_SMART
            && (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
        {
            let mut i8_0 = x264_scan8[0usize] as ::core::ffi::c_int - 8i32;
            (*h).mb.cache.ref_0[0usize][(i8_0 + 1i32) as usize] =
                (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0usize][(i8_0 + 0i32) as usize]
                    as ::core::ffi::c_int
                    + 2i32) as usize];
            (*h).mb.cache.ref_0[0usize][(i8_0 + 0i32) as usize] =
                (*h).mb.cache.ref_0[0usize][(i8_0 + 1i32) as usize];
            (*h).mb.cache.ref_0[0usize][(i8_0 + 3i32) as usize] =
                (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0usize][(i8_0 + 2i32) as usize]
                    as ::core::ffi::c_int
                    + 2i32) as usize];
            (*h).mb.cache.ref_0[0usize][(i8_0 + 2i32) as usize] =
                (*h).mb.cache.ref_0[0usize][(i8_0 + 3i32) as usize];
            i8_0 = x264_scan8[0usize] as ::core::ffi::c_int - 1i32;
            (*h).mb.cache.ref_0[0usize][(i8_0 + 1i32 * 8i32) as usize] = (*h).mb.deblock_ref_table
                [((*h).mb.cache.ref_0[0usize][(i8_0 + 0i32 * 8i32) as usize] as ::core::ffi::c_int
                    + 2i32) as usize];
            (*h).mb.cache.ref_0[0usize][(i8_0 + 0i32 * 8i32) as usize] =
                (*h).mb.cache.ref_0[0usize][(i8_0 + 1i32 * 8i32) as usize];
            (*h).mb.cache.ref_0[0usize][(i8_0 + 3i32 * 8i32) as usize] = (*h).mb.deblock_ref_table
                [((*h).mb.cache.ref_0[0usize][(i8_0 + 2i32 * 8i32) as usize] as ::core::ffi::c_int
                    + 2i32) as usize];
            (*h).mb.cache.ref_0[0usize][(i8_0 + 2i32 * 8i32) as usize] =
                (*h).mb.cache.ref_0[0usize][(i8_0 + 3i32 * 8i32) as usize];
            let mut ref0 = (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0usize]
                [x264_scan8[0usize] as usize]
                as ::core::ffi::c_int
                + 2i32) as usize] as ::core::ffi::c_int;
            let mut ref1 = (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0usize]
                [x264_scan8[4usize] as usize]
                as ::core::ffi::c_int
                + 2i32) as usize] as ::core::ffi::c_int;
            let mut ref2 = (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0usize]
                [x264_scan8[8usize] as usize]
                as ::core::ffi::c_int
                + 2i32) as usize] as ::core::ffi::c_int;
            let mut ref3 = (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0usize]
                [x264_scan8[12usize] as usize]
                as ::core::ffi::c_int
                + 2i32) as usize] as ::core::ffi::c_int;
            let mut reftop = pack16to32(
                ref0 as crate::stdlib::uint8_t as crate::stdlib::uint32_t,
                ref1 as crate::stdlib::uint8_t as crate::stdlib::uint32_t,
            )
            .wrapping_mul(0x101u32);
            let mut refbot = pack16to32(
                ref2 as crate::stdlib::uint8_t as crate::stdlib::uint32_t,
                ref3 as crate::stdlib::uint8_t as crate::stdlib::uint32_t,
            )
            .wrapping_mul(0x101u32);
            (*((&raw mut *(&raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40])
                .offset(0isize) as *mut crate::stdlib::int8_t)
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                        as ::core::ffi::c_int
                        + 8i32 * 0i32) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                .i = reftop;
            (*((&raw mut *(&raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40])
                .offset(0isize) as *mut crate::stdlib::int8_t)
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                        as ::core::ffi::c_int
                        + 8i32 * 1i32) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                .i = reftop;
            (*((&raw mut *(&raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40])
                .offset(0isize) as *mut crate::stdlib::int8_t)
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                        as ::core::ffi::c_int
                        + 8i32 * 2i32) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                .i = refbot;
            (*((&raw mut *(&raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40])
                .offset(0isize) as *mut crate::stdlib::int8_t)
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                        as ::core::ffi::c_int
                        + 8i32 * 3i32) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                .i = refbot;
        }
        if !(*h).param.cabac
            && (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t)).transform_8x8_mode
        {
            let mut nnz_0 = (*h).mb.non_zero_count;
            let mut top = (*h).mb.i_mb_top_xy;
            let mut left_0 = &raw mut (*h).mb.i_mb_left_xy as *mut ::core::ffi::c_int;
            if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0
                && *(*h).mb.mb_transform_size.offset(top as isize) as ::core::ffi::c_int != 0
            {
                let mut i8_1 = x264_scan8[0usize] as ::core::ffi::c_int - 8i32;
                let mut nnz_top0 = (*((&raw mut *nnz_0.offset(top as isize)
                    as *mut crate::stdlib::uint8_t)
                    .offset(8isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i as ::core::ffi::c_int
                    | (*((&raw mut *nnz_0.offset(top as isize) as *mut crate::stdlib::uint8_t)
                        .offset(12isize)
                        as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int;
                let mut nnz_top1 = (*((&raw mut *nnz_0.offset(top as isize)
                    as *mut crate::stdlib::uint8_t)
                    .offset(10isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i as ::core::ffi::c_int
                    | (*((&raw mut *nnz_0.offset(top as isize) as *mut crate::stdlib::uint8_t)
                        .offset(14isize)
                        as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                    .offset((i8_1 + 0i32) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (if nnz_top0 != 0 { 0x101i32 } else { 0i32 }) as crate::stdlib::uint16_t;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                    .offset((i8_1 + 2i32) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (if nnz_top1 != 0 { 0x101i32 } else { 0i32 }) as crate::stdlib::uint16_t;
            }
            if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0 {
                let mut i8_2 = x264_scan8[0usize] as ::core::ffi::c_int - 1i32;
                if *(*h)
                    .mb
                    .mb_transform_size
                    .offset(*left_0.offset(0isize) as isize)
                    != 0
                {
                    let mut nnz_left0 = (*((&raw mut *nnz_0.offset(*left_0.offset(0isize) as isize)
                        as *mut crate::stdlib::uint8_t)
                        .offset(2isize)
                        as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*((&raw mut *nnz_0.offset(*left_0.offset(0isize) as isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(6isize)
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                    (*h).mb.cache.non_zero_count[(i8_2 + 8i32 * 0i32) as usize] =
                        (nnz_left0 != 0) as crate::stdlib::uint8_t;
                    (*h).mb.cache.non_zero_count[(i8_2 + 8i32 * 1i32) as usize] =
                        (nnz_left0 != 0) as crate::stdlib::uint8_t;
                }
                if *(*h)
                    .mb
                    .mb_transform_size
                    .offset(*left_0.offset(1isize) as isize)
                    != 0
                {
                    let mut nnz_left1 = (*((&raw mut *nnz_0.offset(*left_0.offset(1isize) as isize)
                        as *mut crate::stdlib::uint8_t)
                        .offset(10isize)
                        as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*((&raw mut *nnz_0.offset(*left_0.offset(1isize) as isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(14isize)
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                    (*h).mb.cache.non_zero_count[(i8_2 + 8i32 * 2i32) as usize] =
                        (nnz_left1 != 0) as crate::stdlib::uint8_t;
                    (*h).mb.cache.non_zero_count[(i8_2 + 8i32 * 3i32) as usize] =
                        (nnz_left1 != 0) as crate::stdlib::uint8_t;
                }
            }
            if (*h).mb.transform_8x8 {
                let mut nnz0 =
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                                as isize,
                        ) as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset(2isize) as isize,
                            )
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                let mut nnz1 =
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(4isize)
                                as isize,
                        ) as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset(6isize) as isize,
                            )
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                let mut nnz2 =
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(8isize)
                                as isize,
                        ) as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset(10isize) as isize,
                            )
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                let mut nnz3 =
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(12isize) as isize,
                        ) as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset(14isize) as isize,
                            )
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                let mut nnztop = pack16to32(
                    (nnz0 != 0) as crate::stdlib::uint32_t,
                    (nnz1 != 0) as crate::stdlib::uint32_t,
                )
                .wrapping_mul(0x101u32);
                let mut nnzbot = pack16to32(
                    (nnz2 != 0) as crate::stdlib::uint32_t,
                    (nnz3 != 0) as crate::stdlib::uint32_t,
                )
                .wrapping_mul(0x101u32);
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                        as ::core::ffi::c_int
                        + 8i32 * 0i32) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = nnztop;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                        as ::core::ffi::c_int
                        + 8i32 * 1i32) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = nnztop;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                        as ::core::ffi::c_int
                        + 8i32 * 2i32) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = nnzbot;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                        as ::core::ffi::c_int
                        + 8i32 * 3i32) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                    .i = nnzbot;
            }
        }
        (*h).loopf
            .deblock_strength
            .expect("non-null function pointer")(
            &raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t,
            &raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40],
            &raw mut (*h).mb.cache.mv as *mut [[crate::stdlib::int16_t; 2]; 40],
            bs,
            4i32 >> (*h).mb.interlaced as ::core::ffi::c_int,
            ((*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int)
                as ::core::ffi::c_int,
        );
        if (*h).sh.mbaff {
            macroblock_deblock_strength_mbaff(h, bs);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_store_pic(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut i: ::core::ffi::c_int,
    mut b_chroma: ::core::ffi::c_int,
    mut b_mbaff: ::core::ffi::c_int,
) {
    unsafe {
        let mut height = if b_chroma != 0 {
            16i32
                >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
        } else {
            16i32
        };
        let mut i_stride = (*(*h).fdec).i_stride[i as usize];
        let mut i_stride2 = i_stride << (b_mbaff != 0 && (*h).mb.interlaced) as ::core::ffi::c_int;
        let mut i_pix_offset = if b_mbaff != 0 && (*h).mb.interlaced {
            16i32 * mb_x + height * (mb_y & !(1i32)) * i_stride + (mb_y & 1i32) * i_stride
        } else {
            16i32 * mb_x + height * mb_y * i_stride
        };
        if b_chroma != 0 {
            (*h).mc
                .store_interleave_chroma
                .expect("non-null function pointer")(
                (*(&raw mut (*(*h).fdec).plane as *mut *mut crate::src::common::common::pixel)
                    .offset(1isize))
                .offset(i_pix_offset as isize),
                i_stride2 as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fdec[1usize],
                (*h).mb.pic.p_fdec[2usize],
                height,
            );
        } else {
            (*h).mc.copy[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                (*(&raw mut (*(*h).fdec).plane as *mut *mut crate::src::common::common::pixel)
                    .offset(i as isize))
                .offset(i_pix_offset as isize),
                i_stride2 as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fdec[i as usize],
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                16i32,
            );
        };
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_backup_intra(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut b_mbaff: ::core::ffi::c_int,
) {
    unsafe {
        let mut backup_dst = if b_mbaff == 0 {
            mb_y & 1i32
        } else if mb_y & 1i32 != 0 {
            1i32
        } else if (*h).mb.interlaced {
            0i32
        } else {
            2i32
        };
        crate::stdlib::memcpy(
            (*(&raw mut *(&raw mut (*h).intra_border_backup
                as *mut [*mut crate::src::common::common::pixel; 3])
                .offset(backup_dst as isize)
                as *mut *mut crate::src::common::common::pixel)
                .offset(0isize))
            .offset((mb_x * 16i32) as isize) as *mut ::core::ffi::c_void,
            (*h).mb.pic.p_fdec[0usize]
                .offset((crate::src::common::common::FDEC_STRIDE * 15i32) as isize)
                as *const ::core::ffi::c_void,
            (16i32 * crate::src::common::common::SIZEOF_PIXEL) as crate::__stddef_size_t_h::size_t,
        );
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            crate::stdlib::memcpy(
                (*(&raw mut *(&raw mut (*h).intra_border_backup
                    as *mut [*mut crate::src::common::common::pixel; 3])
                    .offset(backup_dst as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(1isize))
                .offset((mb_x * 16i32) as isize) as *mut ::core::ffi::c_void,
                (*h).mb.pic.p_fdec[1usize]
                    .offset((crate::src::common::common::FDEC_STRIDE * 15i32) as isize)
                    as *const ::core::ffi::c_void,
                (16i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                (*(&raw mut *(&raw mut (*h).intra_border_backup
                    as *mut [*mut crate::src::common::common::pixel; 3])
                    .offset(backup_dst as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(2isize))
                .offset((mb_x * 16i32) as isize) as *mut ::core::ffi::c_void,
                (*h).mb.pic.p_fdec[2usize]
                    .offset((crate::src::common::common::FDEC_STRIDE * 15i32) as isize)
                    as *const ::core::ffi::c_void,
                (16i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            let mut backup_src = (15i32
                >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
                * crate::src::common::common::FDEC_STRIDE;
            crate::stdlib::memcpy(
                (*(&raw mut *(&raw mut (*h).intra_border_backup
                    as *mut [*mut crate::src::common::common::pixel; 3])
                    .offset(backup_dst as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(1isize))
                .offset((mb_x * 16i32) as isize) as *mut ::core::ffi::c_void,
                (*h).mb.pic.p_fdec[1usize].offset(backup_src as isize)
                    as *const ::core::ffi::c_void,
                (8i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                (*(&raw mut *(&raw mut (*h).intra_border_backup
                    as *mut [*mut crate::src::common::common::pixel; 3])
                    .offset(backup_dst as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(1isize))
                .offset((mb_x * 16i32 + 8i32) as isize) as *mut ::core::ffi::c_void,
                (*h).mb.pic.p_fdec[2usize].offset(backup_src as isize)
                    as *const ::core::ffi::c_void,
                (8i32 * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
        }
        if b_mbaff != 0 {
            if mb_y & 1i32 != 0 {
                let mut backup_src_0 = (if (*h).mb.interlaced { 7i32 } else { 14i32 })
                    * crate::src::common::common::FDEC_STRIDE;
                backup_dst = if (*h).mb.interlaced { 2i32 } else { 0i32 };
                crate::stdlib::memcpy(
                    (*(&raw mut *(&raw mut (*h).intra_border_backup
                        as *mut [*mut crate::src::common::common::pixel; 3])
                        .offset(backup_dst as isize)
                        as *mut *mut crate::src::common::common::pixel)
                        .offset(0isize))
                    .offset((mb_x * 16i32) as isize)
                        as *mut ::core::ffi::c_void,
                    (*h).mb.pic.p_fdec[0usize].offset(backup_src_0 as isize)
                        as *const ::core::ffi::c_void,
                    (16i32 * crate::src::common::common::SIZEOF_PIXEL)
                        as crate::__stddef_size_t_h::size_t,
                );
                if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                {
                    crate::stdlib::memcpy(
                        (*(&raw mut *(&raw mut (*h).intra_border_backup
                            as *mut [*mut crate::src::common::common::pixel; 3])
                            .offset(backup_dst as isize)
                            as *mut *mut crate::src::common::common::pixel)
                            .offset(1isize))
                        .offset((mb_x * 16i32) as isize)
                            as *mut ::core::ffi::c_void,
                        (*h).mb.pic.p_fdec[1usize].offset(backup_src_0 as isize)
                            as *const ::core::ffi::c_void,
                        (16i32 * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                    );
                    crate::stdlib::memcpy(
                        (*(&raw mut *(&raw mut (*h).intra_border_backup
                            as *mut [*mut crate::src::common::common::pixel; 3])
                            .offset(backup_dst as isize)
                            as *mut *mut crate::src::common::common::pixel)
                            .offset(2isize))
                        .offset((mb_x * 16i32) as isize)
                            as *mut ::core::ffi::c_void,
                        (*h).mb.pic.p_fdec[2usize].offset(backup_src_0 as isize)
                            as *const ::core::ffi::c_void,
                        (16i32 * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                    );
                } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                    if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                    {
                        backup_src_0 = (if (*h).mb.interlaced { 3i32 } else { 6i32 })
                            * crate::src::common::common::FDEC_STRIDE;
                    }
                    crate::stdlib::memcpy(
                        (*(&raw mut *(&raw mut (*h).intra_border_backup
                            as *mut [*mut crate::src::common::common::pixel; 3])
                            .offset(backup_dst as isize)
                            as *mut *mut crate::src::common::common::pixel)
                            .offset(1isize))
                        .offset((mb_x * 16i32) as isize)
                            as *mut ::core::ffi::c_void,
                        (*h).mb.pic.p_fdec[1usize].offset(backup_src_0 as isize)
                            as *const ::core::ffi::c_void,
                        (8i32 * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                    );
                    crate::stdlib::memcpy(
                        (*(&raw mut *(&raw mut (*h).intra_border_backup
                            as *mut [*mut crate::src::common::common::pixel; 3])
                            .offset(backup_dst as isize)
                            as *mut *mut crate::src::common::common::pixel)
                            .offset(1isize))
                        .offset((mb_x * 16i32 + 8i32) as isize)
                            as *mut ::core::ffi::c_void,
                        (*h).mb.pic.p_fdec[2usize].offset(backup_src_0 as isize)
                            as *const ::core::ffi::c_void,
                        (8i32 * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                    );
                }
            }
        }
    }
}
pub unsafe extern "C" fn x264_8_macroblock_cache_save(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let i_mb_xy = (*h).mb.i_mb_xy;
        let i_mb_type = x264_mb_type_fix[(*h).mb.i_type as usize] as ::core::ffi::c_int;
        let s8x8 = (*h).mb.i_b8_stride;
        let s4x4 = (*h).mb.i_b4_stride;
        let i_mb_4x4 = (*h).mb.i_b4_xy;
        let i_mb_8x8 = (*h).mb.i_b8_xy;
        let mut i4x4 = &raw mut *(*h).mb.intra4x4_pred_mode.offset(i_mb_xy as isize)
            as *mut crate::stdlib::int8_t;
        let mut nnz = &raw mut *(*h).mb.non_zero_count.offset(i_mb_xy as isize)
            as *mut crate::stdlib::uint8_t;
        if (*h).sh.mbaff {
            macroblock_backup_intra(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 1i32);
            macroblock_store_pic(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 0i32, 0i32, 1i32);
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                macroblock_store_pic(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 1i32, 0i32, 1i32);
                macroblock_store_pic(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 2i32, 0i32, 1i32);
            } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                macroblock_store_pic(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 1i32, 1i32, 1i32);
            }
        } else {
            macroblock_backup_intra(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 0i32);
            macroblock_store_pic(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 0i32, 0i32, 0i32);
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                macroblock_store_pic(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 1i32, 0i32, 0i32);
                macroblock_store_pic(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 2i32, 0i32, 0i32);
            } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                macroblock_store_pic(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 1i32, 1i32, 0i32);
            }
        }
        x264_8_prefetch_fenc(h, (*h).fdec, (*h).mb.i_mb_x, (*h).mb.i_mb_y);
        *(*h).mb.type_0.offset(i_mb_xy as isize) = i_mb_type as crate::stdlib::int8_t;
        *(*h).mb.slice_table.offset(i_mb_xy as isize) = (*h).sh.i_first_mb;
        *(*h).mb.partition.offset(i_mb_xy as isize) = (if i_mb_type
            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
            || i_mb_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
            || i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
            || i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int
        {
            crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
        } else {
            (*h).mb.i_partition
        }) as crate::stdlib::uint8_t;
        (*h).mb.i_mb_prev_xy = i_mb_xy;
        if i_mb_type == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int {
            (*(i4x4.offset(0isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut (*h).mb.cache.intra4x4_pred_mode as *mut crate::stdlib::int8_t)
                .offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset(10isize) as isize,
                )
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*(i4x4.offset(4isize) as *mut crate::src::common::base::x264_union32_t)).i = pack8to32(
                (*h).mb.cache.intra4x4_pred_mode[x264_scan8[5usize] as usize]
                    as crate::stdlib::uint32_t,
                (*h).mb.cache.intra4x4_pred_mode[x264_scan8[7usize] as usize]
                    as crate::stdlib::uint32_t,
                (*h).mb.cache.intra4x4_pred_mode[x264_scan8[13usize] as usize]
                    as crate::stdlib::uint32_t,
                0u32,
            );
        } else if !(*h).param.constrained_intra
            || (i_mb_type == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
        {
            (*(i4x4 as *mut crate::src::common::base::x264_union64_t)).i =
                (crate::src::common::predict::I_PRED_4x4_DC as ::core::ffi::c_int
                    as ::core::ffi::c_ulonglong)
                    .wrapping_mul(0x101010101010101u64);
        } else {
            (*(i4x4 as *mut crate::src::common::base::x264_union64_t)).i =
                (-(1i32) as crate::stdlib::uint8_t as ::core::ffi::c_ulonglong)
                    .wrapping_mul(0x101010101010101u64);
        }
        if i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int {
            let mut i = 0i32;
            *(*h).mb.qp.offset(i_mb_xy as isize) = 0i8;
            (*h).mb.i_last_dqp = 0i32;
            (*h).mb.i_cbp_chroma = if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                0i32
            } else {
                2i32
            };
            (*h).mb.i_cbp_luma = 0xfi32;
            *(*h).mb.cbp.offset(i_mb_xy as isize) =
                ((*h).mb.i_cbp_chroma << 4i32 | (*h).mb.i_cbp_luma | 0x1700i32)
                    as crate::stdlib::int16_t;
            (*h).mb.transform_8x8 = false;
            while i < 48i32 {
                (*h).mb.cache.non_zero_count[x264_scan8[i as usize] as usize] =
                    (if (*h).param.cabac { 1i32 } else { 16i32 }) as crate::stdlib::uint8_t;
                i += 1;
            }
        } else {
            if (*h).mb.i_type != crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                && (*h).mb.i_cbp_luma == 0i32
                && (*h).mb.i_cbp_chroma == 0i32
            {
                (*h).mb.i_qp = (*h).mb.i_last_qp;
            }
            *(*h).mb.qp.offset(i_mb_xy as isize) = (*h).mb.i_qp as crate::stdlib::int8_t;
            (*h).mb.i_last_dqp = (*h).mb.i_qp - (*h).mb.i_last_qp;
            (*h).mb.i_last_qp = (*h).mb.i_qp;
        }
        (*(nnz.offset((0i32 + 0i32 * 4i32) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset((0i32 + 1i32 * 4i32) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(2isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset((0i32 + 2i32 * 4i32) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(8isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset((0i32 + 3i32 * 4i32) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(10isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset((16i32 + 0i32 * 4i32) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((16i32 + 0i32) as isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset((16i32 + 1i32 * 4i32) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((16i32 + 2i32) as isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset((32i32 + 0i32 * 4i32) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((32i32 + 0i32) as isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset((32i32 + 1i32 * 4i32) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((32i32 + 2i32) as isize) as isize,
        ) as *mut crate::src::common::base::x264_union32_t))
            .i;
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            >= crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
        {
            (*(nnz.offset((16i32 + 2i32 * 4i32) as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                .offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((16i32 + 8i32) as isize) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*(nnz.offset((16i32 + 3i32 * 4i32) as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                .offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((16i32 + 10i32) as isize) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*(nnz.offset((32i32 + 2i32 * 4i32) as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                .offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((32i32 + 8i32) as isize) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*(nnz.offset((32i32 + 3i32 * 4i32) as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                .offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((32i32 + 10i32) as isize) as isize,
                ) as *mut crate::src::common::base::x264_union32_t))
                .i;
        }
        if (*h).mb.i_cbp_luma == 0i32
            && (*h).mb.i_type != crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
        {
            (*h).mb.transform_8x8 = false;
        }
        *(*h).mb.mb_transform_size.offset(i_mb_xy as isize) =
            (*h).mb.transform_8x8 as crate::stdlib::int8_t;
        if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            let mut mv0 = (*(&raw mut (*h).mb.mv as *mut *mut [crate::stdlib::int16_t; 2])
                .offset(0isize))
            .offset(i_mb_4x4 as isize);
            let mut ref0 = (*(&raw mut (*h).mb.ref_0 as *mut *mut crate::stdlib::int8_t)
                .offset(0isize))
            .offset(i_mb_8x8 as isize);
            if !(i_mb_type == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
            {
                *ref0.offset((0i32 + 0i32 * s8x8) as isize) =
                    (*h).mb.cache.ref_0[0usize][x264_scan8[0usize] as usize];
                *ref0.offset((1i32 + 0i32 * s8x8) as isize) =
                    (*h).mb.cache.ref_0[0usize][x264_scan8[4usize] as usize];
                *ref0.offset((0i32 + 1i32 * s8x8) as isize) =
                    (*h).mb.cache.ref_0[0usize][x264_scan8[8usize] as usize];
                *ref0.offset((1i32 + 1i32 * s8x8) as isize) =
                    (*h).mb.cache.ref_0[0usize][x264_scan8[12usize] as usize];
                (*(mv0.offset((0i32 * s4x4) as isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(0isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                            as ::core::ffi::c_int
                            + 8i32 * 0i32) as isize,
                    )
                    as *mut crate::src::common::base::x264_union128_t))
                    .i;
                (*(mv0.offset((1i32 * s4x4) as isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(0isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                            as ::core::ffi::c_int
                            + 8i32 * 1i32) as isize,
                    )
                    as *mut crate::src::common::base::x264_union128_t))
                    .i;
                (*(mv0.offset((2i32 * s4x4) as isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(0isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                            as ::core::ffi::c_int
                            + 8i32 * 2i32) as isize,
                    )
                    as *mut crate::src::common::base::x264_union128_t))
                    .i;
                (*(mv0.offset((3i32 * s4x4) as isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(0isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(0isize)
                            as ::core::ffi::c_int
                            + 8i32 * 3i32) as isize,
                    )
                    as *mut crate::src::common::base::x264_union128_t))
                    .i;
                if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                    let mut mv1 = (*(&raw mut (*h).mb.mv as *mut *mut [crate::stdlib::int16_t; 2])
                        .offset(1isize))
                    .offset(i_mb_4x4 as isize);
                    let mut ref1 = (*(&raw mut (*h).mb.ref_0 as *mut *mut crate::stdlib::int8_t)
                        .offset(1isize))
                    .offset(i_mb_8x8 as isize);
                    *ref1.offset((0i32 + 0i32 * s8x8) as isize) =
                        (*h).mb.cache.ref_0[1usize][x264_scan8[0usize] as usize];
                    *ref1.offset((1i32 + 0i32 * s8x8) as isize) =
                        (*h).mb.cache.ref_0[1usize][x264_scan8[4usize] as usize];
                    *ref1.offset((0i32 + 1i32 * s8x8) as isize) =
                        (*h).mb.cache.ref_0[1usize][x264_scan8[8usize] as usize];
                    *ref1.offset((1i32 + 1i32 * s8x8) as isize) =
                        (*h).mb.cache.ref_0[1usize][x264_scan8[12usize] as usize];
                    (*(mv1.offset((0i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(1isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0isize) as ::core::ffi::c_int
                                + 8i32 * 0i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union128_t))
                        .i;
                    (*(mv1.offset((1i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(1isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0isize) as ::core::ffi::c_int
                                + 8i32 * 1i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union128_t))
                        .i;
                    (*(mv1.offset((2i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(1isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0isize) as ::core::ffi::c_int
                                + 8i32 * 2i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union128_t))
                        .i;
                    (*(mv1.offset((3i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(1isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0isize) as ::core::ffi::c_int
                                + 8i32 * 3i32) as isize,
                        )
                        as *mut crate::src::common::base::x264_union128_t))
                        .i;
                }
            } else {
                (*(ref0.offset((0i32 * s8x8) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (-(1i32) as crate::stdlib::uint8_t as ::core::ffi::c_int * 0x101i32)
                    as crate::stdlib::uint16_t;
                (*(ref0.offset((1i32 * s8x8) as isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (-(1i32) as crate::stdlib::uint8_t as ::core::ffi::c_int * 0x101i32)
                    as crate::stdlib::uint16_t;
                (*(mv0.offset((0i32 * s4x4) as isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                (*(mv0.offset((1i32 * s4x4) as isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                (*(mv0.offset((2i32 * s4x4) as isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                (*(mv0.offset((3i32 * s4x4) as isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                    let mut mv1_0 = (*(&raw mut (*h).mb.mv
                        as *mut *mut [crate::stdlib::int16_t; 2])
                        .offset(1isize))
                    .offset(i_mb_4x4 as isize);
                    let mut ref1_0 = (*(&raw mut (*h).mb.ref_0 as *mut *mut crate::stdlib::int8_t)
                        .offset(1isize))
                    .offset(i_mb_8x8 as isize);
                    (*(ref1_0.offset((0i32 * s8x8) as isize)
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (-(1i32) as crate::stdlib::uint8_t as ::core::ffi::c_int * 0x101i32)
                        as crate::stdlib::uint16_t;
                    (*(ref1_0.offset((1i32 * s8x8) as isize)
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (-(1i32) as crate::stdlib::uint8_t as ::core::ffi::c_int * 0x101i32)
                        as crate::stdlib::uint16_t;
                    (*(mv1_0.offset((0i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = crate::src::common::base::M128_ZERO;
                    (*(mv1_0.offset((1i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = crate::src::common::base::M128_ZERO;
                    (*(mv1_0.offset((2i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = crate::src::common::base::M128_ZERO;
                    (*(mv1_0.offset((3i32 * s4x4) as isize)
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = crate::src::common::base::M128_ZERO;
                }
            }
        }
        if (*h).param.cabac {
            let mut mvd0 =
                &raw mut *(*(&raw mut (*h).mb.mvd as *mut *mut [[crate::stdlib::uint8_t; 2]; 8])
                    .offset(0isize))
                .offset(i_mb_xy as isize) as *mut [crate::stdlib::uint8_t; 2];
            if (i_mb_type == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                && i_mb_type != crate::src::common::macroblock::I_PCM as ::core::ffi::c_int
            {
                *(*h).mb.chroma_pred_mode.offset(i_mb_xy as isize) = x264_mb_chroma_pred_mode_fix
                    [(*h).mb.i_chroma_pred_mode as usize]
                    as crate::stdlib::int8_t;
            } else {
                *(*h).mb.chroma_pred_mode.offset(i_mb_xy as isize) =
                    crate::src::common::predict::I_PRED_CHROMA_DC as crate::stdlib::int8_t;
            }
            if 0x3ff30i32 >> i_mb_type & 1i32 != 0 {
                (*(&raw mut *mvd0.offset(0isize)
                    as *mut crate::src::common::base::x264_union64_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                    as *mut [[crate::stdlib::uint8_t; 2]; 40])
                    .offset(0isize)
                    as *mut [crate::stdlib::uint8_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(10isize)
                            as isize,
                    )
                    as *mut crate::src::common::base::x264_union64_t))
                    .i;
                (*(&raw mut *mvd0.offset(4isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                    as *mut [[crate::stdlib::uint8_t; 2]; 40])
                    .offset(0isize)
                    as *mut [crate::stdlib::uint8_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(5isize)
                            as isize,
                    )
                    as *mut crate::src::common::base::x264_union16_t))
                    .i;
                (*(&raw mut *mvd0.offset(5isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                    as *mut [[crate::stdlib::uint8_t; 2]; 40])
                    .offset(0isize)
                    as *mut [crate::stdlib::uint8_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(7isize)
                            as isize,
                    )
                    as *mut crate::src::common::base::x264_union16_t))
                    .i;
                (*(&raw mut *mvd0.offset(6isize)
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                    as *mut [[crate::stdlib::uint8_t; 2]; 40])
                    .offset(0isize)
                    as *mut [crate::stdlib::uint8_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(13isize)
                            as isize,
                    )
                    as *mut crate::src::common::base::x264_union16_t))
                    .i;
                if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                    let mut mvd1 = &raw mut *(*(&raw mut (*h).mb.mvd
                        as *mut *mut [[crate::stdlib::uint8_t; 2]; 8])
                        .offset(1isize))
                    .offset(i_mb_xy as isize)
                        as *mut [crate::stdlib::uint8_t; 2];
                    (*(&raw mut *mvd1.offset(0isize)
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(1isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(10isize) as isize,
                        )
                        as *mut crate::src::common::base::x264_union64_t))
                        .i;
                    (*(&raw mut *mvd1.offset(4isize)
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(1isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(5isize)
                                as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                    (*(&raw mut *mvd1.offset(5isize)
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(1isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t).offset(7isize)
                                as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                    (*(&raw mut *mvd1.offset(6isize)
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(1isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(13isize) as isize,
                        )
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                }
            } else {
                (*(&raw mut *mvd0.offset(0isize)
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                    let mut mvd1_0 = &raw mut *(*(&raw mut (*h).mb.mvd
                        as *mut *mut [[crate::stdlib::uint8_t; 2]; 8])
                        .offset(1isize))
                    .offset(i_mb_xy as isize)
                        as *mut [crate::stdlib::uint8_t; 2];
                    (*(&raw mut *mvd1_0.offset(0isize)
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = crate::src::common::base::M128_ZERO;
                }
            }
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                if i_mb_type == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int
                    || i_mb_type == crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int
                {
                    *(*h).mb.skipbp.offset(i_mb_xy as isize) = 0xfi8;
                } else if i_mb_type == crate::src::common::macroblock::B_8x8 as ::core::ffi::c_int {
                    let mut skipbp = (((*h).mb.i_sub_partition[0usize] as ::core::ffi::c_int
                        == crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int)
                        as ::core::ffi::c_int)
                        << 0i32;
                    skipbp |= (((*h).mb.i_sub_partition[1usize] as ::core::ffi::c_int
                        == crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int)
                        as ::core::ffi::c_int)
                        << 1i32;
                    skipbp |= (((*h).mb.i_sub_partition[2usize] as ::core::ffi::c_int
                        == crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int)
                        as ::core::ffi::c_int)
                        << 2i32;
                    skipbp |= (((*h).mb.i_sub_partition[3usize] as ::core::ffi::c_int
                        == crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int)
                        as ::core::ffi::c_int)
                        << 3i32;
                    *(*h).mb.skipbp.offset(i_mb_xy as isize) = skipbp as crate::stdlib::int8_t;
                } else {
                    *(*h).mb.skipbp.offset(i_mb_xy as isize) = 0i8;
                }
            }
        }
    }
}
pub unsafe extern "C" fn x264_8_macroblock_bipred_init(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut mbfield = 0i32;
        while mbfield <= (*h).sh.mbaff as ::core::ffi::c_int {
            let mut field = 0i32;
            while field <= (*h).sh.mbaff as ::core::ffi::c_int {
                let mut i_ref0 = 0i32;
                while i_ref0 < (*h).i_ref[0usize] << mbfield {
                    let mut i_ref1 = 0i32;
                    let mut l0 = (*h).fref[0usize][(i_ref0 >> mbfield) as usize];
                    let mut poc0 =
                        (*l0).i_poc + mbfield * (*l0).i_delta_poc[(field ^ i_ref0 & 1i32) as usize];
                    while i_ref1 < (*h).i_ref[1usize] << mbfield {
                        let mut l1 = (*h).fref[1usize][(i_ref1 >> mbfield) as usize];
                        let mut cur_poc =
                            (*(*h).fdec).i_poc + mbfield * (*(*h).fdec).i_delta_poc[field as usize];
                        let mut poc1 = (*l1).i_poc
                            + mbfield * (*l1).i_delta_poc[(field ^ i_ref1 & 1i32) as usize];
                        let mut td = x264_clip3(poc1 - poc0, -(128i32), 127i32);
                        if td == 0i32 {
                            (*h).mb.dist_scale_factor_buf[mbfield as usize][field as usize]
                                [i_ref0 as usize][i_ref1 as usize] = 256i16;
                            (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                                [i_ref0 as usize][i_ref1 as usize] = 32i8;
                        } else {
                            let mut tb = x264_clip3(cur_poc - poc0, -(128i32), 127i32);
                            let mut tx = (16384i32 + (crate::stdlib::abs(td) >> 1i32)) / td;
                            let mut dist_scale_factor =
                                x264_clip3(tb * tx + 32i32 >> 6i32, -(1024i32), 1023i32);
                            (*h).mb.dist_scale_factor_buf[mbfield as usize][field as usize]
                                [i_ref0 as usize][i_ref1 as usize] =
                                dist_scale_factor as crate::stdlib::int16_t;
                            dist_scale_factor >>= 2i32;
                            if (*h).param.analyse.weighted_bipred
                                && dist_scale_factor >= -(64i32)
                                && dist_scale_factor <= 128i32
                            {
                                (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                                    [i_ref0 as usize][i_ref1 as usize] =
                                    (64i32 - dist_scale_factor) as crate::stdlib::int8_t;
                                '_c2rust_label: {
                                    if dist_scale_factor >= -(63i32) && dist_scale_factor <= 127i32
                                    {
                                    } else {
                                        crate::stdlib::__assert_fail(
                                            b"dist_scale_factor >= -63 && dist_scale_factor <= 127\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"common/macroblock.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1918u32,
                                            b"void x264_8_macroblock_bipred_init(x264_t *)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                };
                            } else {
                                (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                                    [i_ref0 as usize][i_ref1 as usize] = 32i8;
                            }
                        }
                        i_ref1 += 1;
                    }
                    i_ref0 += 1;
                }
                field += 1;
            }
            mbfield += 1;
        }
    }
}
