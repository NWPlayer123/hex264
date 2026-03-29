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
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0,
            0,
            0,
            0,
        ],
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0,
            0,
            0,
            0,
        ],
        [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
    ];

    use crate::stdlib::uint8_t;
}

pub mod predict_h {

    pub static mut x264_mb_chroma_pred_mode_fix: [crate::stdlib::uint8_t; 7] = [
        crate::src::common::predict::I_PRED_CHROMA_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_H as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_V as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_P as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
        crate::src::common::predict::I_PRED_CHROMA_DC as ::core::ffi::c_int
            as crate::stdlib::uint8_t,
    ];

    use crate::stdlib::uint8_t;
}

pub mod base_h {

    pub static mut x264_scan8: [crate::stdlib::uint8_t; 51] = [
        (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (4 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (5 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (6 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (7 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
        (0 as ::core::ffi::c_int + 10 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as crate::stdlib::uint8_t,
    ];
    #[inline(always)]

    pub unsafe extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        unsafe {
            return if v < i_min {
                i_min
            } else if v > i_max {
                i_max
            } else {
                v
            };
        }
    }

    use crate::stdlib::uint8_t;
}

pub mod macroblock_h {

    pub static mut x264_mb_type_fix: [crate::stdlib::uint8_t; 19] = [
        crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::I_PCM as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::P_L0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::P_8x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L0_L0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L0_L1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L0_BI as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L1_L0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L1_L1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_L1_BI as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_BI_L0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_BI_L1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_BI_BI as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_8x8 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ];
    #[inline(always)]

    pub unsafe extern "C" fn pack16to32(
        mut a: crate::stdlib::uint32_t,
        mut b: crate::stdlib::uint32_t,
    ) -> crate::stdlib::uint32_t {
        unsafe {
            return a.wrapping_add(b << 16 as ::core::ffi::c_int);
        }
    }
    #[inline(always)]

    pub unsafe extern "C" fn pack8to32(
        mut a: crate::stdlib::uint32_t,
        mut b: crate::stdlib::uint32_t,
        mut c: crate::stdlib::uint32_t,
        mut d: crate::stdlib::uint32_t,
    ) -> crate::stdlib::uint32_t {
        unsafe {
            return a
                .wrapping_add(b << 8 as ::core::ffi::c_int)
                .wrapping_add(c << 16 as ::core::ffi::c_int)
                .wrapping_add(d << 24 as ::core::ffi::c_int);
        }
    }

    use crate::stdlib::uint8_t;
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
                            b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
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
                (&raw mut (*h).mb.cache.skip as *mut crate::stdlib::int8_t).offset(
                    (crate::src::common::base::X264_SCAN8_0 + x + 8 as ::core::ffi::c_int * y)
                        as isize,
                ) as *mut crate::stdlib::int8_t as *mut ::core::ffi::c_void,
                width,
                height,
                1 as ::core::ffi::c_int,
                b_skip as crate::stdlib::uint32_t,
            );
        }
    }
    use crate::src::common::base::x264_union16_t;
    use crate::src::common::base::x264_union32_t;
    use crate::src::common::base::x264_union64_t;
    use crate::src::common::base::X264_SCAN8_0;
    use crate::stdlib::__assert_fail;

    use crate::osdep_h::WORD_SIZE;
    use crate::stdlib::int8_t;
    use crate::stdlib::uint16_t;
    use crate::stdlib::uint32_t;
    use crate::stdlib::uint64_t;
    use crate::stdlib::uint8_t;
}

pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::common::base::chroma_format_e;
pub use crate::src::common::base::slice_type_e;
pub use crate::src::common::base::x264_free;
pub use crate::src::common::base::x264_malloc;
pub use crate::src::common::base::x264_uint128_t;
pub use crate::src::common::base::x264_union128_t;
pub use crate::src::common::base::x264_union16_t;
pub use crate::src::common::base::x264_union32_t;
pub use crate::src::common::base::x264_union64_t;
pub use crate::src::common::base::CHROMA_400;
pub use crate::src::common::base::CHROMA_420;
pub use crate::src::common::base::CHROMA_422;
pub use crate::src::common::base::CHROMA_444;
pub use crate::src::common::base::M128_ZERO;
pub use crate::src::common::base::SLICE_TYPE_B;
pub use crate::src::common::base::SLICE_TYPE_I;
pub use crate::src::common::base::SLICE_TYPE_P;
pub use crate::src::common::base::X264_SCAN8_0;
pub use crate::src::common::base::X264_WEIGHTP_FAKE;
pub use crate::src::common::bitstream::bs_s;
pub use crate::src::common::bitstream::bs_t;
pub use crate::src::common::bitstream::x264_bitstream_function_t;
pub use crate::src::common::bitstream::x264_run_level_t;
pub use crate::src::common::cabac::x264_cabac_t;
pub use crate::src::common::macroblock::base_h::x264_clip3;
pub use crate::src::common::macroblock::base_h::x264_scan8;
pub use crate::stdlib::C2Rust_Unnamed_7;
use crate::stdlib::__assert_fail;
use crate::stdlib::__assert_single_arg;
pub use crate::stdlib::__atomic_wide_counter;

pub use crate::internal::__va_list_tag;
pub use crate::internal::BIT_DEPTH;
pub use crate::osdep_h::WORD_SIZE;
pub use crate::src::common::common::dctcoef;
pub use crate::src::common::common::mvsad_t;
pub use crate::src::common::common::pixel;
pub use crate::src::common::common::udctcoef;
pub use crate::src::common::common::x264_frame_stat_t;
pub use crate::src::common::common::x264_left_table_t;
pub use crate::src::common::common::x264_lookahead_t;
pub use crate::src::common::common::x264_ratecontrol_t;
pub use crate::src::common::common::x264_slice_header_t;
pub use crate::src::common::common::x264_t;
pub use crate::src::common::common::C2Rust_Unnamed_10;
pub use crate::src::common::common::C2Rust_Unnamed_11;
pub use crate::src::common::common::C2Rust_Unnamed_12;
pub use crate::src::common::common::C2Rust_Unnamed_13;
pub use crate::src::common::common::C2Rust_Unnamed_14;
pub use crate::src::common::common::C2Rust_Unnamed_15;
pub use crate::src::common::common::C2Rust_Unnamed_16;
pub use crate::src::common::common::C2Rust_Unnamed_17;
pub use crate::src::common::common::C2Rust_Unnamed_8;
pub use crate::src::common::common::C2Rust_Unnamed_9;
pub use crate::src::common::common::FDEC_STRIDE;
pub use crate::src::common::common::FENC_STRIDE;
pub use crate::src::common::common::SIZEOF_PIXEL;
pub use crate::src::common::dct::x264_dct_function_t;
pub use crate::src::common::dct::x264_zigzag_function_t;
pub use crate::src::common::frame::x264_deblock_function_t;
pub use crate::src::common::frame::x264_deblock_inter_t;
pub use crate::src::common::frame::x264_deblock_intra_t;
pub use crate::src::common::frame::x264_frame;
pub use crate::src::common::frame::x264_frame_t;
pub use crate::src::common::frame::x264_sync_frame_list_t;
pub use crate::src::common::frame::PADV;
pub use crate::src::common::macroblock::macroblock_h::pack16to32;
pub use crate::src::common::macroblock::macroblock_h::pack8to32;
pub use crate::src::common::macroblock::macroblock_h::x264_mb_type_fix;
pub use crate::src::common::mc::weight_fn_t;
pub use crate::src::common::mc::x264_mc_functions_t_5;
pub use crate::src::common::mc::x264_weight_t;
pub use crate::src::common::mvpred::x264_8_mb_predict_mv_pskip;

pub use crate::src::common::macroblock::pixel_h::x264_size2pixel;
pub use crate::src::common::macroblock::predict_h::x264_mb_chroma_pred_mode_fix;
pub use crate::src::common::pixel::x264_pixel_cmp_t;
pub use crate::src::common::pixel::x264_pixel_cmp_x3_t;
pub use crate::src::common::pixel::x264_pixel_cmp_x4_t;
pub use crate::src::common::pixel::x264_pixel_function_t;
pub use crate::src::common::pixel::PIXEL_16x16;
pub use crate::src::common::pixel::PIXEL_16x8;
pub use crate::src::common::pixel::PIXEL_2x2;
pub use crate::src::common::pixel::PIXEL_2x4;
pub use crate::src::common::pixel::PIXEL_2x8;
pub use crate::src::common::pixel::PIXEL_4x16;
pub use crate::src::common::pixel::PIXEL_4x2;
pub use crate::src::common::pixel::PIXEL_4x4;
pub use crate::src::common::pixel::PIXEL_4x8;
pub use crate::src::common::pixel::PIXEL_8x16;
pub use crate::src::common::pixel::PIXEL_8x4;
pub use crate::src::common::pixel::PIXEL_8x8;
pub use crate::src::common::predict::intra4x4_pred_e;
pub use crate::src::common::predict::intra_chroma_pred_e;
pub use crate::src::common::predict::x264_predict8x8_t;
pub use crate::src::common::predict::x264_predict_8x8_filter_t;
pub use crate::src::common::predict::x264_predict_t;
pub use crate::src::common::predict::I_PRED_4x4_DC;
pub use crate::src::common::predict::I_PRED_4x4_DC_128;
pub use crate::src::common::predict::I_PRED_4x4_DC_LEFT;
pub use crate::src::common::predict::I_PRED_4x4_DC_TOP;
pub use crate::src::common::predict::I_PRED_4x4_DDL;
pub use crate::src::common::predict::I_PRED_4x4_DDR;
pub use crate::src::common::predict::I_PRED_4x4_H;
pub use crate::src::common::predict::I_PRED_4x4_HD;
pub use crate::src::common::predict::I_PRED_4x4_HU;
pub use crate::src::common::predict::I_PRED_4x4_V;
pub use crate::src::common::predict::I_PRED_4x4_VL;
pub use crate::src::common::predict::I_PRED_4x4_VR;
pub use crate::src::common::predict::I_PRED_CHROMA_DC;
pub use crate::src::common::predict::I_PRED_CHROMA_DC_128;
pub use crate::src::common::predict::I_PRED_CHROMA_DC_LEFT;
pub use crate::src::common::predict::I_PRED_CHROMA_DC_TOP;
pub use crate::src::common::predict::I_PRED_CHROMA_H;
pub use crate::src::common::predict::I_PRED_CHROMA_P;
pub use crate::src::common::predict::I_PRED_CHROMA_V;
pub use crate::src::common::quant::x264_quant_function_t;
pub use crate::stdlib::pthread_cond_t;
pub use crate::stdlib::pthread_mutex_t;
pub use crate::stdlib::pthread_t;
pub use crate::stdlib::C2Rust_Unnamed_6;

pub use crate::src::common::macroblock::rectangle_h::x264_macroblock_cache_rect;
pub use crate::src::common::macroblock::rectangle_h::x264_macroblock_cache_skip;
pub use crate::src::common::set::x264_pps_t;
pub use crate::src::common::set::x264_sps_t;
pub use crate::src::common::set::C2Rust_Unnamed_24;
pub use crate::src::common::set::C2Rust_Unnamed_25;
pub use crate::src::common::set::C2Rust_Unnamed_26;
use crate::src::common::tables::x264_zero;
use crate::src::common::threadpool::x264_threadpool_t;
pub use crate::stdlib::__pthread_cond_s;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;
pub use crate::stdlib::__pthread_mutex_s;
use crate::stdlib::abs;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::int8_t;
pub use crate::stdlib::intptr_t;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::uintptr_t;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__int8_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::__uint8_t;
pub use crate::x264_h::x264_hrd_t;
pub use crate::x264_h::x264_nal_t;
pub use crate::x264_h::x264_param_t;
pub use crate::x264_h::x264_sei_payload_t;
pub use crate::x264_h::x264_sei_t;
pub use crate::x264_h::x264_zone_t;
pub use crate::x264_h::C2Rust_Unnamed_0;
pub use crate::x264_h::C2Rust_Unnamed_1;
pub use crate::x264_h::C2Rust_Unnamed_2;
pub use crate::x264_h::C2Rust_Unnamed_3;
pub use crate::x264_h::C2Rust_Unnamed_4;
pub use crate::x264_h::C2Rust_Unnamed_5;
pub use crate::x264_h::X264_ME_ESA;
pub use crate::x264_h::X264_WEIGHTP_SMART;
#[inline(never)]

unsafe extern "C" fn mb_mc_0xywh(
    mut h: *mut crate::src::common::common::x264_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    unsafe {
        let mut i8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            + x
            + 8 as ::core::ffi::c_int * y;
        let mut i_ref: ::core::ffi::c_int = (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [i8 as usize] as ::core::ffi::c_int;
        let mut mvx: ::core::ffi::c_int = x264_clip3(
            (*h).mb.cache.mv[0 as ::core::ffi::c_int as usize][i8 as usize]
                [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[0 as ::core::ffi::c_int as usize],
            (*h).mb.mv_max[0 as ::core::ffi::c_int as usize],
        ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * x;
        let mut mvy: ::core::ffi::c_int = x264_clip3(
            (*h).mb.cache.mv[0 as ::core::ffi::c_int as usize][i8 as usize]
                [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[1 as ::core::ffi::c_int as usize],
            (*h).mb.mv_max[1 as ::core::ffi::c_int as usize],
        ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * y;
        (*h).mc.mc_luma.expect("non-null function pointer")(
            (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                .offset(0 as ::core::ffi::c_int as isize))
            .offset(
                (4 as ::core::ffi::c_int * y * crate::src::common::common::FDEC_STRIDE
                    + 4 as ::core::ffi::c_int * x) as isize,
            ) as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut [*mut crate::src::common::common::pixel; 12])
                .offset(i_ref as isize) as *mut *mut crate::src::common::common::pixel)
                .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                as *mut *mut crate::src::common::common::pixel,
            (*h).mb.pic.i_stride[0 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
            mvx,
            mvy,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            if 0 as ::core::ffi::c_int != 0 {
                &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                    as *const crate::src::common::mc::x264_weight_t
            } else {
                (&raw mut *(&raw mut (*h).sh.weight
                    as *mut [crate::src::common::mc::x264_weight_t; 3])
                    .offset(i_ref as isize)
                    as *mut crate::src::common::mc::x264_weight_t)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::mc::x264_weight_t
                    as *const crate::src::common::mc::x264_weight_t
            },
        );
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(
                    (4 as ::core::ffi::c_int * y * crate::src::common::common::FDEC_STRIDE
                        + 4 as ::core::ffi::c_int * x) as isize,
                ) as *mut crate::src::common::common::pixel,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as *mut *mut crate::src::common::common::pixel,
                (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx,
                mvy,
                4 as ::core::ffi::c_int * width,
                4 as ::core::ffi::c_int * height,
                if 0 as ::core::ffi::c_int != 0 {
                    &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                        as *const crate::src::common::mc::x264_weight_t
                } else {
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::mc::x264_weight_t
                        as *const crate::src::common::mc::x264_weight_t
                },
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2 as ::core::ffi::c_int as isize))
                .offset(
                    (4 as ::core::ffi::c_int * y * crate::src::common::common::FDEC_STRIDE
                        + 4 as ::core::ffi::c_int * x) as isize,
                ) as *mut crate::src::common::common::pixel,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as *mut *mut crate::src::common::common::pixel,
                (*h).mb.pic.i_stride[2 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx,
                mvy,
                4 as ::core::ffi::c_int * width,
                4 as ::core::ffi::c_int * height,
                if 0 as ::core::ffi::c_int != 0 {
                    &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                        as *const crate::src::common::mc::x264_weight_t
                } else {
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::mc::x264_weight_t
                        as *const crate::src::common::mc::x264_weight_t
                },
            );
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            let mut v_shift: ::core::ffi::c_int = (crate::src::common::base::CHROMA_444
                as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if v_shift & (*h).mb.b_interlaced & i_ref != 0 {
                mvy += ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int;
            }
            let mut offset: ::core::ffi::c_int =
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE >> v_shift) * y
                    + 2 as ::core::ffi::c_int * x;
            height = 4 as ::core::ffi::c_int * height >> v_shift;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(offset as isize) as *mut crate::src::common::common::pixel,
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2 as ::core::ffi::c_int as isize))
                .offset(offset as isize) as *mut crate::src::common::common::pixel,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fref[0 as ::core::ffi::c_int as usize][i_ref as usize]
                    [4 as ::core::ffi::c_int as usize],
                (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx,
                2 as ::core::ffi::c_int * mvy >> v_shift,
                2 as ::core::ffi::c_int * width,
                height,
            );
            if !(*h).sh.weight[i_ref as usize][1 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null()
            {
                (*(*h).sh.weight[i_ref as usize][1 as ::core::ffi::c_int as usize]
                    .weightfn
                    .offset((width >> 1 as ::core::ffi::c_int) as isize))
                .expect("non-null function pointer")(
                    (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                        .offset(1 as ::core::ffi::c_int as isize))
                    .offset(offset as isize)
                        as *mut crate::src::common::common::pixel,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                        .offset(1 as ::core::ffi::c_int as isize))
                    .offset(offset as isize)
                        as *mut crate::src::common::common::pixel,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::mc::x264_weight_t,
                    height,
                );
            }
            if !(*h).sh.weight[i_ref as usize][2 as ::core::ffi::c_int as usize]
                .weightfn
                .is_null()
            {
                (*(*h).sh.weight[i_ref as usize][2 as ::core::ffi::c_int as usize]
                    .weightfn
                    .offset((width >> 1 as ::core::ffi::c_int) as isize))
                .expect("non-null function pointer")(
                    (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                        .offset(2 as ::core::ffi::c_int as isize))
                    .offset(offset as isize)
                        as *mut crate::src::common::common::pixel,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                        .offset(2 as ::core::ffi::c_int as isize))
                    .offset(offset as isize)
                        as *mut crate::src::common::common::pixel,
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::mc::x264_weight_t,
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
        let mut i8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            + x
            + 8 as ::core::ffi::c_int * y;
        let mut i_ref: ::core::ffi::c_int = (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
            [i8 as usize] as ::core::ffi::c_int;
        let mut mvx: ::core::ffi::c_int = x264_clip3(
            (*h).mb.cache.mv[1 as ::core::ffi::c_int as usize][i8 as usize]
                [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[0 as ::core::ffi::c_int as usize],
            (*h).mb.mv_max[0 as ::core::ffi::c_int as usize],
        ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * x;
        let mut mvy: ::core::ffi::c_int = x264_clip3(
            (*h).mb.cache.mv[1 as ::core::ffi::c_int as usize][i8 as usize]
                [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            (*h).mb.mv_min[1 as ::core::ffi::c_int as usize],
            (*h).mb.mv_max[1 as ::core::ffi::c_int as usize],
        ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * y;
        (*h).mc.mc_luma.expect("non-null function pointer")(
            (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                .offset(0 as ::core::ffi::c_int as isize))
            .offset(
                (4 as ::core::ffi::c_int * y * crate::src::common::common::FDEC_STRIDE
                    + 4 as ::core::ffi::c_int * x) as isize,
            ) as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                .offset(1 as ::core::ffi::c_int as isize)
                as *mut [*mut crate::src::common::common::pixel; 12])
                .offset(i_ref as isize) as *mut *mut crate::src::common::common::pixel)
                .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                as *mut *mut crate::src::common::common::pixel,
            (*h).mb.pic.i_stride[0 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
            mvx,
            mvy,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            if 1 as ::core::ffi::c_int != 0 {
                &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                    as *const crate::src::common::mc::x264_weight_t
            } else {
                (&raw mut *(&raw mut (*h).sh.weight
                    as *mut [crate::src::common::mc::x264_weight_t; 3])
                    .offset(i_ref as isize)
                    as *mut crate::src::common::mc::x264_weight_t)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::mc::x264_weight_t
                    as *const crate::src::common::mc::x264_weight_t
            },
        );
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(
                    (4 as ::core::ffi::c_int * y * crate::src::common::common::FDEC_STRIDE
                        + 4 as ::core::ffi::c_int * x) as isize,
                ) as *mut crate::src::common::common::pixel,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as *mut *mut crate::src::common::common::pixel,
                (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx,
                mvy,
                4 as ::core::ffi::c_int * width,
                4 as ::core::ffi::c_int * height,
                if 1 as ::core::ffi::c_int != 0 {
                    &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                        as *const crate::src::common::mc::x264_weight_t
                } else {
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::mc::x264_weight_t
                        as *const crate::src::common::mc::x264_weight_t
                },
            );
            (*h).mc.mc_luma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2 as ::core::ffi::c_int as isize))
                .offset(
                    (4 as ::core::ffi::c_int * y * crate::src::common::common::FDEC_STRIDE
                        + 4 as ::core::ffi::c_int * x) as isize,
                ) as *mut crate::src::common::common::pixel,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as *mut *mut crate::src::common::common::pixel,
                (*h).mb.pic.i_stride[2 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx,
                mvy,
                4 as ::core::ffi::c_int * width,
                4 as ::core::ffi::c_int * height,
                if 1 as ::core::ffi::c_int != 0 {
                    &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                        as *const crate::src::common::mc::x264_weight_t
                } else {
                    (&raw mut *(&raw mut (*h).sh.weight
                        as *mut [crate::src::common::mc::x264_weight_t; 3])
                        .offset(i_ref as isize)
                        as *mut crate::src::common::mc::x264_weight_t)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::mc::x264_weight_t
                        as *const crate::src::common::mc::x264_weight_t
                },
            );
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            let mut v_shift: ::core::ffi::c_int = (crate::src::common::base::CHROMA_444
                as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if v_shift & (*h).mb.b_interlaced & i_ref != 0 {
                mvy += ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int;
            }
            let mut offset: ::core::ffi::c_int =
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE >> v_shift) * y
                    + 2 as ::core::ffi::c_int * x;
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(offset as isize) as *mut crate::src::common::common::pixel,
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2 as ::core::ffi::c_int as isize))
                .offset(offset as isize) as *mut crate::src::common::common::pixel,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fref[1 as ::core::ffi::c_int as usize][i_ref as usize]
                    [4 as ::core::ffi::c_int as usize],
                (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx,
                2 as ::core::ffi::c_int * mvy >> v_shift,
                2 as ::core::ffi::c_int * width,
                4 as ::core::ffi::c_int * height >> v_shift,
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
        let mut i8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            + x
            + 8 as ::core::ffi::c_int * y;
        let mut i_ref0: ::core::ffi::c_int = (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [i8 as usize] as ::core::ffi::c_int;
        let mut i_ref1: ::core::ffi::c_int = (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
            [i8 as usize] as ::core::ffi::c_int;
        let mut weight: ::core::ffi::c_int =
            (*(*h).mb.bipred_weight.offset(i_ref0 as isize))[i_ref1 as usize] as ::core::ffi::c_int;
        let mut mvx0: ::core::ffi::c_int =
            x264_clip3(
                (*h).mb.cache.mv[0 as ::core::ffi::c_int as usize][i8 as usize]
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                (*h).mb.mv_min[0 as ::core::ffi::c_int as usize],
                (*h).mb.mv_max[0 as ::core::ffi::c_int as usize],
            ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * x;
        let mut mvx1: ::core::ffi::c_int =
            x264_clip3(
                (*h).mb.cache.mv[1 as ::core::ffi::c_int as usize][i8 as usize]
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                (*h).mb.mv_min[0 as ::core::ffi::c_int as usize],
                (*h).mb.mv_max[0 as ::core::ffi::c_int as usize],
            ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * x;
        let mut mvy0: ::core::ffi::c_int =
            x264_clip3(
                (*h).mb.cache.mv[0 as ::core::ffi::c_int as usize][i8 as usize]
                    [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                (*h).mb.mv_min[1 as ::core::ffi::c_int as usize],
                (*h).mb.mv_max[1 as ::core::ffi::c_int as usize],
            ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * y;
        let mut mvy1: ::core::ffi::c_int =
            x264_clip3(
                (*h).mb.cache.mv[1 as ::core::ffi::c_int as usize][i8 as usize]
                    [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                (*h).mb.mv_min[1 as ::core::ffi::c_int as usize],
                (*h).mb.mv_max[1 as ::core::ffi::c_int as usize],
            ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * y;
        let mut i_mode: ::core::ffi::c_int =
            x264_size2pixel[height as usize][width as usize] as ::core::ffi::c_int;
        let mut i_stride0: crate::stdlib::intptr_t =
            16 as ::core::ffi::c_int as crate::stdlib::intptr_t;
        let mut i_stride1: crate::stdlib::intptr_t =
            16 as ::core::ffi::c_int as crate::stdlib::intptr_t;
        let mut tmp0: [crate::src::common::common::pixel; 256] = [0; 256];
        let mut tmp1: [crate::src::common::common::pixel; 256] = [0; 256];
        let mut src0: *mut crate::src::common::common::pixel =
            ::core::ptr::null_mut::<crate::src::common::common::pixel>();
        let mut src1: *mut crate::src::common::common::pixel =
            ::core::ptr::null_mut::<crate::src::common::common::pixel>();
        src0 = (*h).mc.get_ref.expect("non-null function pointer")(
            &raw mut tmp0 as *mut crate::src::common::common::pixel,
            &raw mut i_stride0,
            (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut [*mut crate::src::common::common::pixel; 12])
                .offset(i_ref0 as isize)
                as *mut *mut crate::src::common::common::pixel)
                .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                as *mut *mut crate::src::common::common::pixel,
            (*h).mb.pic.i_stride[0 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
            mvx0,
            mvy0,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                as *const crate::src::common::mc::x264_weight_t,
        );
        src1 = (*h).mc.get_ref.expect("non-null function pointer")(
            &raw mut tmp1 as *mut crate::src::common::common::pixel,
            &raw mut i_stride1,
            (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                .offset(1 as ::core::ffi::c_int as isize)
                as *mut [*mut crate::src::common::common::pixel; 12])
                .offset(i_ref1 as isize)
                as *mut *mut crate::src::common::common::pixel)
                .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                as *mut *mut crate::src::common::common::pixel,
            (*h).mb.pic.i_stride[0 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
            mvx1,
            mvy1,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                as *const crate::src::common::mc::x264_weight_t,
        );
        (*h).mc.avg[i_mode as usize].expect("non-null function pointer")(
            (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                .offset(0 as ::core::ffi::c_int as isize))
            .offset(
                (4 as ::core::ffi::c_int * y * crate::src::common::common::FDEC_STRIDE
                    + 4 as ::core::ffi::c_int * x) as isize,
            ) as *mut crate::src::common::common::pixel,
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
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref0 as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as *mut *mut crate::src::common::common::pixel,
                (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx0,
                mvy0,
                4 as ::core::ffi::c_int * width,
                4 as ::core::ffi::c_int * height,
                &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                    as *const crate::src::common::mc::x264_weight_t,
            );
            src1 = (*h).mc.get_ref.expect("non-null function pointer")(
                &raw mut tmp1 as *mut crate::src::common::common::pixel,
                &raw mut i_stride1,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref1 as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as *mut *mut crate::src::common::common::pixel,
                (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx1,
                mvy1,
                4 as ::core::ffi::c_int * width,
                4 as ::core::ffi::c_int * height,
                &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                    as *const crate::src::common::mc::x264_weight_t,
            );
            (*h).mc.avg[i_mode as usize].expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(
                    (4 as ::core::ffi::c_int * y * crate::src::common::common::FDEC_STRIDE
                        + 4 as ::core::ffi::c_int * x) as isize,
                ) as *mut crate::src::common::common::pixel,
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
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref0 as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as *mut *mut crate::src::common::common::pixel,
                (*h).mb.pic.i_stride[2 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx0,
                mvy0,
                4 as ::core::ffi::c_int * width,
                4 as ::core::ffi::c_int * height,
                &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                    as *const crate::src::common::mc::x264_weight_t,
            );
            src1 = (*h).mc.get_ref.expect("non-null function pointer")(
                &raw mut tmp1 as *mut crate::src::common::common::pixel,
                &raw mut i_stride1,
                (&raw mut *(&raw mut *(&raw mut (*h).mb.pic.p_fref
                    as *mut [[*mut crate::src::common::common::pixel; 12]; 32])
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut [*mut crate::src::common::common::pixel; 12])
                    .offset(i_ref1 as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as *mut *mut crate::src::common::common::pixel,
                (*h).mb.pic.i_stride[2 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx1,
                mvy1,
                4 as ::core::ffi::c_int * width,
                4 as ::core::ffi::c_int * height,
                &raw mut crate::src::common::tables::x264_zero as *mut crate::stdlib::uint8_t
                    as *const crate::src::common::mc::x264_weight_t,
            );
            (*h).mc.avg[i_mode as usize].expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2 as ::core::ffi::c_int as isize))
                .offset(
                    (4 as ::core::ffi::c_int * y * crate::src::common::common::FDEC_STRIDE
                        + 4 as ::core::ffi::c_int * x) as isize,
                ) as *mut crate::src::common::common::pixel,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                src0,
                i_stride0,
                src1,
                i_stride1,
                weight,
            );
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            let mut v_shift: ::core::ffi::c_int = (crate::src::common::base::CHROMA_444
                as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if v_shift & (*h).mb.b_interlaced & i_ref0 != 0 {
                mvy0 += ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int;
            }
            if v_shift & (*h).mb.b_interlaced & i_ref1 != 0 {
                mvy1 += ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int;
            }
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &raw mut tmp0 as *mut crate::src::common::common::pixel,
                (&raw mut tmp0 as *mut crate::src::common::common::pixel)
                    .offset(8 as ::core::ffi::c_int as isize),
                16 as ::core::ffi::c_int as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fref[0 as ::core::ffi::c_int as usize][i_ref0 as usize]
                    [4 as ::core::ffi::c_int as usize],
                (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx0,
                2 as ::core::ffi::c_int * mvy0 >> v_shift,
                2 as ::core::ffi::c_int * width,
                4 as ::core::ffi::c_int * height >> v_shift,
            );
            (*h).mc.mc_chroma.expect("non-null function pointer")(
                &raw mut tmp1 as *mut crate::src::common::common::pixel,
                (&raw mut tmp1 as *mut crate::src::common::common::pixel)
                    .offset(8 as ::core::ffi::c_int as isize),
                16 as ::core::ffi::c_int as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fref[1 as ::core::ffi::c_int as usize][i_ref1 as usize]
                    [4 as ::core::ffi::c_int as usize],
                (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as crate::stdlib::intptr_t,
                mvx1,
                2 as ::core::ffi::c_int * mvy1 >> v_shift,
                2 as ::core::ffi::c_int * width,
                4 as ::core::ffi::c_int * height >> v_shift,
            );
            let mut chromapix: ::core::ffi::c_int =
                (*h).luma2chroma_pixel[i_mode as usize] as ::core::ffi::c_int;
            let mut offset: ::core::ffi::c_int =
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE >> v_shift) * y
                    + 2 as ::core::ffi::c_int * x;
            (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(offset as isize) as *mut crate::src::common::common::pixel,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut tmp0 as *mut crate::src::common::common::pixel,
                16 as ::core::ffi::c_int as crate::stdlib::intptr_t,
                &raw mut tmp1 as *mut crate::src::common::common::pixel,
                16 as ::core::ffi::c_int as crate::stdlib::intptr_t,
                weight,
            );
            (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
                (*(&raw mut (*h).mb.pic.p_fdec as *mut *mut crate::src::common::common::pixel)
                    .offset(2 as ::core::ffi::c_int as isize))
                .offset(offset as isize) as *mut crate::src::common::common::pixel,
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                (&raw mut tmp0 as *mut crate::src::common::common::pixel)
                    .offset(8 as ::core::ffi::c_int as isize),
                16 as ::core::ffi::c_int as crate::stdlib::intptr_t,
                (&raw mut tmp1 as *mut crate::src::common::common::pixel)
                    .offset(8 as ::core::ffi::c_int as isize),
                16 as ::core::ffi::c_int as crate::stdlib::intptr_t,
                weight,
            );
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_mb_mc_8x8(
    mut h: *mut crate::src::common::common::x264_t,
    mut i8: ::core::ffi::c_int,
) {
    unsafe {
        let mut x: ::core::ffi::c_int = 2 as ::core::ffi::c_int * (i8 & 1 as ::core::ffi::c_int);
        let mut y: ::core::ffi::c_int = 2 as ::core::ffi::c_int * (i8 >> 1 as ::core::ffi::c_int);
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            match (*h).mb.i_sub_partition[i8 as usize] as ::core::ffi::c_int {
                3 => {
                    mb_mc_0xywh(h, x, y, 2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int);
                }
                1 => {
                    mb_mc_0xywh(
                        h,
                        x,
                        y + 0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    mb_mc_0xywh(
                        h,
                        x,
                        y + 1 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                }
                2 => {
                    mb_mc_0xywh(
                        h,
                        x + 0 as ::core::ffi::c_int,
                        y,
                        1 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                    mb_mc_0xywh(
                        h,
                        x + 1 as ::core::ffi::c_int,
                        y,
                        1 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                }
                0 => {
                    mb_mc_0xywh(
                        h,
                        x + 0 as ::core::ffi::c_int,
                        y + 0 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    mb_mc_0xywh(
                        h,
                        x + 1 as ::core::ffi::c_int,
                        y + 0 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    mb_mc_0xywh(
                        h,
                        x + 0 as ::core::ffi::c_int,
                        y + 1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    mb_mc_0xywh(
                        h,
                        x + 1 as ::core::ffi::c_int,
                        y + 1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                }
                _ => {}
            }
        } else {
            let mut scan8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                + x
                + 8 as ::core::ffi::c_int * y;
            if (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize][scan8 as usize]
                as ::core::ffi::c_int
                >= 0 as ::core::ffi::c_int
            {
                if (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize][scan8 as usize]
                    as ::core::ffi::c_int
                    >= 0 as ::core::ffi::c_int
                {
                    mb_mc_01xywh(h, x, y, 2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int);
                } else {
                    mb_mc_0xywh(h, x, y, 2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int);
                }
            } else {
                mb_mc_1xywh(h, x, y, 2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int);
            }
        };
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_mb_mc(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        if (*h).mb.i_partition == crate::src::common::macroblock::D_8x8 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                x264_8_mb_mc_8x8(h, i);
                i += 1;
            }
        } else {
            let mut ref0a: ::core::ffi::c_int = (*h).mb.cache.ref_0
                [0 as ::core::ffi::c_int as usize]
                [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int;
            let mut ref0b: ::core::ffi::c_int = (*h).mb.cache.ref_0
                [0 as ::core::ffi::c_int as usize]
                [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int;
            let mut ref1a: ::core::ffi::c_int = (*h).mb.cache.ref_0
                [1 as ::core::ffi::c_int as usize]
                [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int;
            let mut ref1b: ::core::ffi::c_int = (*h).mb.cache.ref_0
                [1 as ::core::ffi::c_int as usize]
                [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int;
            if (*h).mb.i_partition == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
            {
                if ref0a >= 0 as ::core::ffi::c_int {
                    if ref1a >= 0 as ::core::ffi::c_int {
                        mb_mc_01xywh(
                            h,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                    } else {
                        mb_mc_0xywh(
                            h,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    mb_mc_1xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                }
            } else if (*h).mb.i_partition
                == crate::src::common::macroblock::D_16x8 as ::core::ffi::c_int
            {
                if ref0a >= 0 as ::core::ffi::c_int {
                    if ref1a >= 0 as ::core::ffi::c_int {
                        mb_mc_01xywh(
                            h,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                        );
                    } else {
                        mb_mc_0xywh(
                            h,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    mb_mc_1xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                }
                if ref0b >= 0 as ::core::ffi::c_int {
                    if ref1b >= 0 as ::core::ffi::c_int {
                        mb_mc_01xywh(
                            h,
                            0 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                        );
                    } else {
                        mb_mc_0xywh(
                            h,
                            0 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    mb_mc_1xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                }
            } else if (*h).mb.i_partition
                == crate::src::common::macroblock::D_8x16 as ::core::ffi::c_int
            {
                if ref0a >= 0 as ::core::ffi::c_int {
                    if ref1a >= 0 as ::core::ffi::c_int {
                        mb_mc_01xywh(
                            h,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                    } else {
                        mb_mc_0xywh(
                            h,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    mb_mc_1xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                }
                if ref0b >= 0 as ::core::ffi::c_int {
                    if ref1b >= 0 as ::core::ffi::c_int {
                        mb_mc_01xywh(
                            h,
                            2 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                    } else {
                        mb_mc_0xywh(
                            h,
                            2 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    mb_mc_1xywh(
                        h,
                        2 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                }
            }
        };
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_macroblock_cache_allocate(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_mb_count: ::core::ffi::c_int = (*h).mb.i_mb_count;
        (*h).mb.i_mb_stride = (*h).mb.i_mb_width;
        (*h).mb.i_b8_stride = (*h).mb.i_mb_width * 2 as ::core::ffi::c_int;
        (*h).mb.i_b4_stride = (*h).mb.i_mb_width * 4 as ::core::ffi::c_int;
        (*h).mb.b_interlaced = (*h).param.b_interlaced;
        let mut prealloc_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut prealloc_size: crate::stdlib::int64_t = 0 as crate::stdlib::int64_t;
        let mut preallocs: [*mut *mut crate::stdlib::uint8_t; 1024] =
            [::core::ptr::null_mut::<*mut crate::stdlib::uint8_t>(); 1024];
        (*h).mb.qp = prealloc_size as crate::stdlib::intptr_t as *mut ::core::ffi::c_void
            as *mut crate::stdlib::int8_t;
        let c2rust_fresh0 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh0 as usize] = &raw mut (*h).mb.qp as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += (i_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>() as usize)
            as crate::stdlib::int64_t
            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t
            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t;
        (*h).mb.cbp = prealloc_size as crate::stdlib::intptr_t as *mut ::core::ffi::c_void
            as *mut crate::stdlib::int16_t;
        let c2rust_fresh1 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh1 as usize] =
            &raw mut (*h).mb.cbp as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += (i_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>() as usize)
            as crate::stdlib::int64_t
            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t
            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t;
        (*h).mb.mb_transform_size = prealloc_size as crate::stdlib::intptr_t
            as *mut ::core::ffi::c_void
            as *mut crate::stdlib::int8_t;
        let c2rust_fresh2 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh2 as usize] =
            &raw mut (*h).mb.mb_transform_size as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += (i_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>() as usize)
            as crate::stdlib::int64_t
            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t
            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t;
        (*h).mb.slice_table = prealloc_size as crate::stdlib::intptr_t as *mut ::core::ffi::c_void
            as *mut crate::stdlib::int32_t;
        let c2rust_fresh3 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh3 as usize] =
            &raw mut (*h).mb.slice_table as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += (i_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int32_t>() as usize)
            as crate::stdlib::int64_t
            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t
            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t;
        (*h).mb.intra4x4_pred_mode = prealloc_size as crate::stdlib::intptr_t
            as *mut ::core::ffi::c_void
            as *mut [crate::stdlib::int8_t; 8];
        let c2rust_fresh4 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh4 as usize] =
            &raw mut (*h).mb.intra4x4_pred_mode as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += ((i_mb_count * 8 as ::core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>() as usize)
            as crate::stdlib::int64_t
            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t
            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t;
        (*h).mb.non_zero_count = prealloc_size as crate::stdlib::intptr_t
            as *mut ::core::ffi::c_void
            as *mut [crate::stdlib::uint8_t; 48];
        let c2rust_fresh5 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[c2rust_fresh5 as usize] =
            &raw mut (*h).mb.non_zero_count as *mut *mut crate::stdlib::uint8_t;
        prealloc_size += ((i_mb_count * 48 as ::core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>() as usize)
            as crate::stdlib::int64_t
            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t
            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t;
        if (*h).param.b_cabac != 0 {
            (*h).mb.skipbp = prealloc_size as crate::stdlib::intptr_t as *mut ::core::ffi::c_void
                as *mut crate::stdlib::int8_t;
            let c2rust_fresh6 = prealloc_idx;
            prealloc_idx = prealloc_idx + 1;
            preallocs[c2rust_fresh6 as usize] =
                &raw mut (*h).mb.skipbp as *mut *mut crate::stdlib::uint8_t;
            prealloc_size += (i_mb_count as usize)
                .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>() as usize)
                as crate::stdlib::int64_t
                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t
                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t;
            (*h).mb.chroma_pred_mode = prealloc_size as crate::stdlib::intptr_t
                as *mut ::core::ffi::c_void
                as *mut crate::stdlib::int8_t;
            let c2rust_fresh7 = prealloc_idx;
            prealloc_idx = prealloc_idx + 1;
            preallocs[c2rust_fresh7 as usize] =
                &raw mut (*h).mb.chroma_pred_mode as *mut *mut crate::stdlib::uint8_t;
            prealloc_size += (i_mb_count as usize)
                .wrapping_mul(::core::mem::size_of::<crate::stdlib::int8_t>() as usize)
                as crate::stdlib::int64_t
                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t
                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t;
            (*h).mb.mvd[0 as ::core::ffi::c_int as usize] = prealloc_size as crate::stdlib::intptr_t
                as *mut ::core::ffi::c_void
                as *mut [[crate::stdlib::uint8_t; 2]; 8];
            let c2rust_fresh8 = prealloc_idx;
            prealloc_idx = prealloc_idx + 1;
            preallocs[c2rust_fresh8 as usize] = (&raw mut (*h).mb.mvd
                as *mut *mut [[crate::stdlib::uint8_t; 2]; 8])
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut *mut [[crate::stdlib::uint8_t; 2]; 8]
                as *mut *mut crate::stdlib::uint8_t;
            prealloc_size += (i_mb_count as usize)
                .wrapping_mul(::core::mem::size_of::<[[crate::stdlib::uint8_t; 2]; 8]>() as usize)
                as crate::stdlib::int64_t
                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t
                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as crate::stdlib::int64_t;
            if (*h).param.i_bframe != 0 {
                (*h).mb.mvd[1 as ::core::ffi::c_int as usize] =
                    prealloc_size as crate::stdlib::intptr_t as *mut ::core::ffi::c_void
                        as *mut [[crate::stdlib::uint8_t; 2]; 8];
                let c2rust_fresh9 = prealloc_idx;
                prealloc_idx = prealloc_idx + 1;
                preallocs[c2rust_fresh9 as usize] = (&raw mut (*h).mb.mvd
                    as *mut *mut [[crate::stdlib::uint8_t; 2]; 8])
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut *mut [[crate::stdlib::uint8_t; 2]; 8]
                    as *mut *mut crate::stdlib::uint8_t;
                prealloc_size += (i_mb_count as usize).wrapping_mul(::core::mem::size_of::<
                    [[crate::stdlib::uint8_t; 2]; 8],
                >() as usize) as crate::stdlib::int64_t
                    + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as crate::stdlib::int64_t
                    & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as crate::stdlib::int64_t;
            }
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 2 as ::core::ffi::c_int {
            let mut i_refs: ::core::ffi::c_int = (if (16 as ::core::ffi::c_int)
                < (if i != 0 {
                    1 as ::core::ffi::c_int
                        + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
                } else {
                    (*h).param.i_frame_reference
                }) {
                16 as ::core::ffi::c_int
            } else {
                (if i != 0 {
                    1 as ::core::ffi::c_int
                        + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
                } else {
                    (*h).param.i_frame_reference
                })
            }) << (*h).param.b_interlaced;
            if (*h).param.analyse.i_weighted_pred == crate::x264_h::X264_WEIGHTP_SMART {
                i_refs = if (16 as ::core::ffi::c_int)
                    < i_refs
                        + 1 as ::core::ffi::c_int
                        + (8 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) as ::core::ffi::c_int
                {
                    16 as ::core::ffi::c_int
                } else {
                    i_refs
                        + 1 as ::core::ffi::c_int
                        + (8 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) as ::core::ffi::c_int
                };
            }
            let mut j: ::core::ffi::c_int = (i == 0) as ::core::ffi::c_int;
            while j < i_refs {
                (*h).mb.mvr[i as usize][j as usize] = prealloc_size as crate::stdlib::intptr_t
                    as *mut ::core::ffi::c_void
                    as *mut [crate::stdlib::int16_t; 2];
                let c2rust_fresh10 = prealloc_idx;
                prealloc_idx = prealloc_idx + 1;
                preallocs[c2rust_fresh10 as usize] = (&raw mut *(&raw mut (*h).mb.mvr
                    as *mut [*mut [crate::stdlib::int16_t; 2]; 32])
                    .offset(i as isize)
                    as *mut *mut [crate::stdlib::int16_t; 2])
                    .offset(j as isize)
                    as *mut *mut [crate::stdlib::int16_t; 2]
                    as *mut *mut crate::stdlib::uint8_t;
                prealloc_size += ((2 as ::core::ffi::c_int * (i_mb_count + 1 as ::core::ffi::c_int))
                    as usize)
                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>() as usize)
                    as crate::stdlib::int64_t
                    + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as crate::stdlib::int64_t
                    & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as crate::stdlib::int64_t;
                j += 1;
            }
            i += 1;
        }
        if (*h).param.analyse.i_weighted_pred != 0 {
            let mut i_padv: ::core::ffi::c_int =
                crate::src::common::frame::PADV << (*h).param.b_interlaced;
            let mut luma_plane_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut numweightbuf: ::core::ffi::c_int = 0;
            if (*h).param.analyse.i_weighted_pred == crate::src::common::base::X264_WEIGHTP_FAKE {
                if (*h).param.i_sync_lookahead == 0
                    || h == (*h).thread[(*h).param.i_threads as usize]
                {
                    luma_plane_size = (*(*h).fdec).i_stride_lowres
                        * ((*h).mb.i_mb_height * 8 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * i_padv);
                    numweightbuf = 1 as ::core::ffi::c_int;
                } else {
                    numweightbuf = 0 as ::core::ffi::c_int;
                }
            } else {
                luma_plane_size = (*(*h).fdec).i_stride[0 as ::core::ffi::c_int as usize]
                    * ((*h).mb.i_mb_height
                        * ((16 as ::core::ffi::c_int)
                            << (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                                == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int)
                                as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_int * i_padv);
                if (*h).param.analyse.i_weighted_pred == crate::x264_h::X264_WEIGHTP_SMART {
                    numweightbuf = 1 as ::core::ffi::c_int
                        + (crate::internal::BIT_DEPTH == 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                } else {
                    numweightbuf = 1 as ::core::ffi::c_int;
                }
            }
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < numweightbuf {
                (*h).mb.p_weight_buf[i_0 as usize] = prealloc_size as crate::stdlib::intptr_t
                    as *mut ::core::ffi::c_void
                    as *mut crate::src::common::common::pixel;
                let c2rust_fresh11 = prealloc_idx;
                prealloc_idx = prealloc_idx + 1;
                preallocs[c2rust_fresh11 as usize] = (&raw mut (*h).mb.p_weight_buf
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(i_0 as isize)
                    as *mut *mut crate::src::common::common::pixel
                    as *mut *mut crate::stdlib::uint8_t;
                prealloc_size += (luma_plane_size
                    * ::core::mem::size_of::<crate::src::common::common::pixel>()
                        as ::core::ffi::c_int)
                    as crate::stdlib::int64_t
                    + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as crate::stdlib::int64_t
                    & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as crate::stdlib::int64_t;
                i_0 += 1;
            }
        }
        (*h).mb.base =
            crate::src::common::base::x264_malloc(prealloc_size) as *mut crate::stdlib::uint8_t;
        if (*h).mb.base.is_null() {
            return -(1 as ::core::ffi::c_int);
        } else {
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
                -(1 as ::core::ffi::c_int),
                (i_mb_count as crate::__stddef_size_t_h::size_t)
                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::int32_t>()
                        as crate::__stddef_size_t_h::size_t),
            );
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < 2 as ::core::ffi::c_int {
                let mut i_refs_0: ::core::ffi::c_int = (if (16 as ::core::ffi::c_int)
                    < (if i_1 != 0 {
                        1 as ::core::ffi::c_int
                            + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
                    } else {
                        (*h).param.i_frame_reference
                    }) {
                    16 as ::core::ffi::c_int
                } else {
                    (if i_1 != 0 {
                        1 as ::core::ffi::c_int
                            + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
                    } else {
                        (*h).param.i_frame_reference
                    })
                }) << (*h).param.b_interlaced;
                if (*h).param.analyse.i_weighted_pred == crate::x264_h::X264_WEIGHTP_SMART {
                    i_refs_0 = if (16 as ::core::ffi::c_int)
                        < i_refs_0
                            + 1 as ::core::ffi::c_int
                            + (8 as ::core::ffi::c_int == 8 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                    {
                        16 as ::core::ffi::c_int
                    } else {
                        i_refs_0
                            + 1 as ::core::ffi::c_int
                            + (8 as ::core::ffi::c_int == 8 as ::core::ffi::c_int)
                                as ::core::ffi::c_int
                    };
                }
                let mut j_0: ::core::ffi::c_int = (i_1 == 0) as ::core::ffi::c_int;
                while j_0 < i_refs_0 {
                    (*(&raw mut *(*(&raw mut *(&raw mut (*h).mb.mvr
                        as *mut [*mut [crate::stdlib::int16_t; 2]; 32])
                        .offset(i_1 as isize)
                        as *mut *mut [crate::stdlib::int16_t; 2])
                        .offset(j_0 as isize))
                    .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0 as crate::stdlib::uint32_t;
                    (*h).mb.mvr[i_1 as usize][j_0 as usize] =
                        (*h).mb.mvr[i_1 as usize][j_0 as usize].offset(1);
                    j_0 += 1;
                }
                i_1 += 1;
            }
            return 0 as ::core::ffi::c_int;
        };
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_macroblock_cache_free(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        crate::src::common::base::x264_free((*h).mb.base as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_macroblock_thread_allocate(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_lookahead: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut scratch_size: ::core::ffi::c_int = 0;
        let mut buf_mbtree: ::core::ffi::c_int = 0;
        let mut buf_lookahead_threads: ::core::ffi::c_int = 0;
        let mut buf_mbtree2: ::core::ffi::c_int = 0;
        let mut c2rust_current_block: u64;
        if b_lookahead == 0 {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            's_5: loop {
                if !(i
                    < (if (*h).param.b_interlaced != 0 {
                        5 as ::core::ffi::c_int
                    } else {
                        2 as ::core::ffi::c_int
                    }))
                {
                    c2rust_current_block = 8515828400728868193;
                    break;
                }
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j
                    < (if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        2 as ::core::ffi::c_int
                    })
                {
                    (*h).intra_border_backup[i as usize][j as usize] =
                        crate::src::common::base::x264_malloc(
                            (((*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                .i_mb_width
                                * 16 as ::core::ffi::c_int
                                + 32 as ::core::ffi::c_int)
                                * ::core::mem::size_of::<crate::src::common::common::pixel>()
                                    as ::core::ffi::c_int)
                                as crate::stdlib::int64_t,
                        ) as *mut crate::src::common::common::pixel;
                    if (*h).intra_border_backup[i as usize][j as usize].is_null() {
                        c2rust_current_block = 2278402434551512179;
                        break 's_5;
                    }
                    (*h).intra_border_backup[i as usize][j as usize] = (*h).intra_border_backup
                        [i as usize][j as usize]
                        .offset(16 as ::core::ffi::c_int as isize);
                    j += 1;
                }
                i += 1;
            }
            match c2rust_current_block {
                2278402434551512179 => {}
                _ => {
                    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    loop {
                        if !(i_0 <= (*h).param.b_interlaced) {
                            c2rust_current_block = 5783071609795492627;
                            break;
                        }
                        if (*h).param.b_sliced_threads != 0 {
                            if h == (*h).thread[0 as ::core::ffi::c_int as usize] && i_0 == 0 {
                                (*h).deblock_strength[0 as ::core::ffi::c_int as usize] =
                                    crate::src::common::base::x264_malloc(
                                        (::core::mem::size_of::<[[[crate::stdlib::uint8_t; 4]; 8]; 2]>() as usize)
                                            .wrapping_mul((*h).mb.i_mb_count as usize)
                                            as crate::stdlib::int64_t,
                                    )
                                        as *mut [[[crate::stdlib::uint8_t; 4]; 8]; 2];
                                if (*h).deblock_strength[0 as ::core::ffi::c_int as usize].is_null()
                                {
                                    c2rust_current_block = 2278402434551512179;
                                    break;
                                }
                            } else {
                                (*h).deblock_strength[i_0 as usize] = (*(*h).thread
                                    [0 as ::core::ffi::c_int as usize])
                                    .deblock_strength
                                    [0 as ::core::ffi::c_int as usize];
                            }
                        } else {
                            (*h).deblock_strength[i_0 as usize] =
                                crate::src::common::base::x264_malloc(
                                    (::core::mem::size_of::<[[[crate::stdlib::uint8_t; 4]; 8]; 2]>()
                                        as usize)
                                        .wrapping_mul((*h).mb.i_mb_width as usize)
                                        as crate::stdlib::int64_t,
                                )
                                    as *mut [[[crate::stdlib::uint8_t; 4]; 8]; 2];
                            if (*h).deblock_strength[i_0 as usize].is_null() {
                                c2rust_current_block = 2278402434551512179;
                                break;
                            }
                        }
                        (*h).deblock_strength[1 as ::core::ffi::c_int as usize] =
                            (*h).deblock_strength[i_0 as usize];
                        i_0 += 1;
                    }
                }
            }
        } else {
            c2rust_current_block = 5783071609795492627;
        }
        match c2rust_current_block {
            5783071609795492627 => {
                scratch_size = 0 as ::core::ffi::c_int;
                if b_lookahead == 0 {
                    let mut buf_hpel: ::core::ffi::c_int =
                        (((*(*(*h).thread[0 as ::core::ffi::c_int as usize]).fdec).i_width
                            [0 as ::core::ffi::c_int as usize]
                            + 48 as ::core::ffi::c_int
                            + 32 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>() as usize)
                            as ::core::ffi::c_int;
                    let mut buf_ssim: ::core::ffi::c_int = (((*h).param.analyse.b_ssim
                        * 8 as ::core::ffi::c_int
                        * ((*h).param.i_width / 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int))
                        as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        as ::core::ffi::c_int;
                    let mut me_range: ::core::ffi::c_int =
                        if (*h).param.analyse.i_me_range < (*h).param.analyse.i_mv_range {
                            (*h).param.analyse.i_me_range
                        } else {
                            (*h).param.analyse.i_mv_range
                        };
                    let mut buf_tesa: ::core::ffi::c_int =
                        (((*h).param.analyse.i_me_method >= crate::x264_h::X264_ME_ESA)
                            as ::core::ffi::c_int as usize)
                            .wrapping_mul(
                                ((me_range * 2 as ::core::ffi::c_int + 24 as ::core::ffi::c_int)
                                    as usize)
                                    .wrapping_mul(
                                        ::core::mem::size_of::<crate::stdlib::int16_t>() as usize
                                    )
                                    .wrapping_add(
                                        (((me_range + 4 as ::core::ffi::c_int)
                                            * (me_range + 1 as ::core::ffi::c_int)
                                            * 4 as ::core::ffi::c_int)
                                            as usize)
                                            .wrapping_mul(::core::mem::size_of::<
                                                crate::src::common::common::mvsad_t,
                                            >(
                                            )
                                                as usize),
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
                buf_mbtree = ((*h).param.rc.b_mb_tree as usize).wrapping_mul(
                    ((*h).mb.i_mb_width as usize)
                        .wrapping_mul(::core::mem::size_of::<crate::stdlib::int16_t>() as usize)
                        .wrapping_add(
                            (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize,
                        )
                        & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize,
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
                        buf_lookahead_threads = (((*h).mb.i_mb_height
                            + (4 as ::core::ffi::c_int + 32 as ::core::ffi::c_int)
                                * (*h).param.i_lookahead_threads)
                            as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                            .wrapping_mul(2 as usize)
                            as ::core::ffi::c_int;
                        buf_mbtree2 = buf_mbtree * 12 as ::core::ffi::c_int;
                        scratch_size = if buf_lookahead_threads > buf_mbtree2 {
                            buf_lookahead_threads
                        } else {
                            buf_mbtree2
                        };
                        (*h).scratch_buffer2 = crate::src::common::base::x264_malloc(
                            scratch_size as crate::stdlib::int64_t,
                        );
                        if !(*h).scratch_buffer2.is_null() {
                            return 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
            _ => {}
        }
        return -(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_macroblock_thread_free(
    mut h: *mut crate::src::common::common::x264_t,
    mut b_lookahead: ::core::ffi::c_int,
) {
    unsafe {
        if b_lookahead == 0 {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i <= (*h).param.b_interlaced {
                if (*h).param.b_sliced_threads == 0
                    || h == (*h).thread[0 as ::core::ffi::c_int as usize] && i == 0
                {
                    crate::src::common::base::x264_free(
                        (*h).deblock_strength[i as usize] as *mut ::core::ffi::c_void,
                    );
                }
                i += 1;
            }
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0
                < (if (*h).param.b_interlaced != 0 {
                    5 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                })
            {
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j
                    < (if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    {
                        3 as ::core::ffi::c_int
                    } else {
                        2 as ::core::ffi::c_int
                    })
                {
                    crate::src::common::base::x264_free(
                        (*h).intra_border_backup[i_0 as usize][j as usize]
                            .offset(-(16 as ::core::ffi::c_int as isize))
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
#[no_mangle]

pub unsafe extern "C" fn x264_8_macroblock_slice_init(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        (*h).mb.mv[0 as ::core::ffi::c_int as usize] =
            (*(*h).fdec).mv[0 as ::core::ffi::c_int as usize];
        (*h).mb.mv[1 as ::core::ffi::c_int as usize] =
            (*(*h).fdec).mv[1 as ::core::ffi::c_int as usize];
        (*h).mb.mvr[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] =
            (*(*h).fdec).mv16x16;
        (*h).mb.ref_0[0 as ::core::ffi::c_int as usize] =
            (*(*h).fdec).ref_0[0 as ::core::ffi::c_int as usize];
        (*h).mb.ref_0[1 as ::core::ffi::c_int as usize] =
            (*(*h).fdec).ref_0[1 as ::core::ffi::c_int as usize];
        (*h).mb.type_0 = (*(*h).fdec).mb_type;
        (*h).mb.partition = (*(*h).fdec).mb_partition;
        (*h).mb.field = (*(*h).fdec).field;
        (*(*h).fdec).i_ref[0 as ::core::ffi::c_int as usize] =
            (*h).i_ref[0 as ::core::ffi::c_int as usize];
        (*(*h).fdec).i_ref[1 as ::core::ffi::c_int as usize] =
            (*h).i_ref[1 as ::core::ffi::c_int as usize];
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
            (*(*h).fdec).ref_poc[0 as ::core::ffi::c_int as usize][i as usize] =
                (*(*h).fref[0 as ::core::ffi::c_int as usize][i as usize]).i_poc;
            i += 1;
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < (*h).i_ref[1 as ::core::ffi::c_int as usize] {
                (*(*h).fdec).ref_poc[1 as ::core::ffi::c_int as usize][i_0 as usize] =
                    (*(*h).fref[1 as ::core::ffi::c_int as usize][i_0 as usize]).i_poc;
                i_0 += 1;
            }
            (*h).mb.map_col_to_list0
                [(-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int) as usize] =
                -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t;
            (*h).mb.map_col_to_list0
                [(-(2 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int) as usize] =
                -(2 as ::core::ffi::c_int) as crate::stdlib::int8_t;
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1
                < (*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .i_ref[0 as ::core::ffi::c_int as usize]
            {
                let mut poc: ::core::ffi::c_int = (*(*h).fref[1 as ::core::ffi::c_int as usize]
                    [0 as ::core::ffi::c_int as usize])
                    .ref_poc[0 as ::core::ffi::c_int as usize][i_1 as usize];
                (*h).mb.map_col_to_list0[(i_1 + 2 as ::core::ffi::c_int) as usize] =
                    -(2 as ::core::ffi::c_int) as crate::stdlib::int8_t;
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
                    if (*(*h).fref[0 as ::core::ffi::c_int as usize][j as usize]).i_poc == poc {
                        (*h).mb.map_col_to_list0[(i_1 + 2 as ::core::ffi::c_int) as usize] =
                            j as crate::stdlib::int8_t;
                        break;
                    } else {
                        j += 1;
                    }
                }
                i_1 += 1;
            }
        } else if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            if (*h).sh.i_disable_deblocking_filter_idc != 1 as ::core::ffi::c_int
                && (*h).param.analyse.i_weighted_pred == crate::x264_h::X264_WEIGHTP_SMART
            {
                (*h).mb.deblock_ref_table
                    [(-(2 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int) as usize] =
                    -(2 as ::core::ffi::c_int) as crate::stdlib::int8_t;
                (*h).mb.deblock_ref_table
                    [(-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int) as usize] =
                    -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t;
                let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_2 < (*h).i_ref[0 as ::core::ffi::c_int as usize] << (*h).sh.b_mbaff {
                    if (*h).mb.b_interlaced == 0 {
                        (*h).mb.deblock_ref_table[(i_2 + 2 as ::core::ffi::c_int) as usize] =
                            ((*(*h).fref[0 as ::core::ffi::c_int as usize][i_2 as usize])
                                .i_frame_num
                                & 63 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                    } else {
                        (*h).mb.deblock_ref_table[(i_2 + 2 as ::core::ffi::c_int) as usize] =
                            ((((*(*h).fref[0 as ::core::ffi::c_int as usize]
                                [(i_2 >> 1 as ::core::ffi::c_int) as usize])
                                .i_frame_num
                                & 63 as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                + (i_2 & 1 as ::core::ffi::c_int))
                                as crate::stdlib::int8_t;
                    }
                    i_2 += 1;
                }
            }
        }
        crate::stdlib::memset(
            &raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40]
                as *mut ::core::ffi::c_void,
            -(2 as ::core::ffi::c_int),
            ::core::mem::size_of::<[[crate::stdlib::int8_t; 40]; 2]>()
                as crate::__stddef_size_t_h::size_t,
        );
        if (*h).i_ref[0 as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
            let mut field: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while field <= (*h).sh.b_mbaff {
                let mut curpoc: ::core::ffi::c_int =
                    (*(*h).fdec).i_poc + (*(*h).fdec).i_delta_poc[field as usize];
                let mut refpoc: ::core::ffi::c_int = (*(*h).fref[0 as ::core::ffi::c_int as usize]
                    [0 as ::core::ffi::c_int as usize])
                    .i_poc
                    + (*(*h).fref[0 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .i_delta_poc[field as usize];
                let mut delta: ::core::ffi::c_int = curpoc - refpoc;
                (*(*h).fdec).inv_ref_poc[field as usize] =
                    ((256 as ::core::ffi::c_int + delta / 2 as ::core::ffi::c_int) / delta)
                        as crate::stdlib::int16_t;
                field += 1;
            }
        }
        (*h).mb.i_neighbour4[14 as ::core::ffi::c_int as usize] =
            (crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                | crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
                | crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[12 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour4[14 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour4[9 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour4[12 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour4[6 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour4[9 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour8[3 as ::core::ffi::c_int as usize] =
            (crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                | crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[15 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour8[3 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour4[13 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour4[15 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour4[11 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour4[13 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour4[7 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour4[11 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour4[3 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour4[7 as ::core::ffi::c_int as usize];
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_macroblock_thread_init(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        (*h).mb.i_me_method = (*h).param.analyse.i_me_method;
        (*h).mb.i_subpel_refine = (*h).param.analyse.i_subpel_refine;
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
            && ((*h).mb.i_subpel_refine == 6 as ::core::ffi::c_int
                || (*h).mb.i_subpel_refine == 8 as ::core::ffi::c_int)
        {
            (*h).mb.i_subpel_refine -= 1;
        }
        (*h).mb.b_chroma_me = ((*h).param.analyse.b_chroma_me != 0
            && ((*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
                && (*h).mb.i_subpel_refine >= 5 as ::core::ffi::c_int
                || (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
                    && (*h).mb.i_subpel_refine >= 9 as ::core::ffi::c_int))
            as ::core::ffi::c_int;
        (*h).mb.b_dct_decimate = ((*h).sh.i_type
            == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int
            || (*h).param.analyse.b_dct_decimate != 0
                && (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        (*h).mb.i_mb_prev_xy = -(1 as ::core::ffi::c_int);
        (*h).mb.pic.p_fenc[0 as ::core::ffi::c_int as usize] =
            &raw mut (*h).mb.pic.fenc_buf as *mut crate::src::common::common::pixel;
        (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize] = (&raw mut (*h).mb.pic.fdec_buf
            as *mut crate::src::common::common::pixel)
            .offset((2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE) as isize);
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            (*h).mb.pic.p_fenc[1 as ::core::ffi::c_int as usize] =
                (&raw mut (*h).mb.pic.fenc_buf as *mut crate::src::common::common::pixel).offset(
                    (16 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE) as isize,
                );
            (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize] =
                (&raw mut (*h).mb.pic.fdec_buf as *mut crate::src::common::common::pixel).offset(
                    (20 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE) as isize,
                );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                (*h).mb.pic.p_fenc[2 as ::core::ffi::c_int as usize] =
                    (&raw mut (*h).mb.pic.fenc_buf as *mut crate::src::common::common::pixel)
                        .offset(
                            (32 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE)
                                as isize,
                        );
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize] =
                    (&raw mut (*h).mb.pic.fdec_buf as *mut crate::src::common::common::pixel)
                        .offset(
                            (38 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        );
            } else {
                (*h).mb.pic.p_fenc[2 as ::core::ffi::c_int as usize] =
                    (&raw mut (*h).mb.pic.fenc_buf as *mut crate::src::common::common::pixel)
                        .offset(
                            (16 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE)
                                as isize,
                        )
                        .offset(8 as ::core::ffi::c_int as isize);
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize] =
                    (&raw mut (*h).mb.pic.fdec_buf as *mut crate::src::common::common::pixel)
                        .offset(
                            (20 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        )
                        .offset(16 as ::core::ffi::c_int as isize);
            }
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_prefetch_fenc(
    mut h: *mut crate::src::common::common::x264_t,
    mut fenc: *mut crate::src::common::frame::x264_frame_t,
    mut i_mb_x: ::core::ffi::c_int,
    mut i_mb_y: ::core::ffi::c_int,
) {
    unsafe {
        let mut stride_y: ::core::ffi::c_int = (*fenc).i_stride[0 as ::core::ffi::c_int as usize];
        let mut stride_uv: ::core::ffi::c_int = (*fenc).i_stride[1 as ::core::ffi::c_int as usize];
        let mut off_y: ::core::ffi::c_int =
            16 as ::core::ffi::c_int * i_mb_x + 16 as ::core::ffi::c_int * i_mb_y * stride_y;
        let mut off_uv: ::core::ffi::c_int = 16 as ::core::ffi::c_int * i_mb_x
            + (16 as ::core::ffi::c_int * i_mb_y * stride_uv
                >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
        (*h).mc.prefetch_fenc.expect("non-null function pointer")(
            (*fenc).plane[0 as ::core::ffi::c_int as usize].offset(off_y as isize),
            stride_y as crate::stdlib::intptr_t,
            if !(*fenc).plane[1 as ::core::ffi::c_int as usize].is_null() {
                (*fenc).plane[1 as ::core::ffi::c_int as usize].offset(off_uv as isize)
            } else {
                ::core::ptr::null_mut::<crate::src::common::common::pixel>()
            },
            stride_uv as crate::stdlib::intptr_t,
            i_mb_x,
        );
    }
}
#[no_mangle]
#[inline(never)]

pub unsafe extern "C" fn x264_8_copy_column8(
    mut dst: *mut crate::src::common::common::pixel,
    mut src: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
        while i < 4 as ::core::ffi::c_int {
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
        let mut mb_interlaced: ::core::ffi::c_int =
            (b_mbaff != 0 && (*h).mb.b_interlaced != 0) as ::core::ffi::c_int;
        let mut height: ::core::ffi::c_int = if b_chroma != 0 {
            16 as ::core::ffi::c_int
                >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        };
        let mut i_stride: ::core::ffi::c_int = (*(*h).fdec).i_stride[i as usize];
        let mut i_stride2: ::core::ffi::c_int = i_stride << mb_interlaced;
        let mut i_pix_offset: ::core::ffi::c_int = if mb_interlaced != 0 {
            16 as ::core::ffi::c_int * mb_x
                + height * (mb_y & !(1 as ::core::ffi::c_int)) * i_stride
                + (mb_y & 1 as ::core::ffi::c_int) * i_stride
        } else {
            16 as ::core::ffi::c_int * mb_x + height * mb_y * i_stride
        };
        let mut plane_fdec: *mut crate::src::common::common::pixel =
            (*(&raw mut (*(*h).fdec).plane as *mut *mut crate::src::common::common::pixel)
                .offset(i as isize))
            .offset(i_pix_offset as isize) as *mut crate::src::common::common::pixel;
        let mut fdec_idx: ::core::ffi::c_int = if b_mbaff != 0 {
            if mb_interlaced != 0 {
                3 as ::core::ffi::c_int + (mb_y & 1 as ::core::ffi::c_int)
            } else if mb_y & 1 as ::core::ffi::c_int != 0 {
                2 as ::core::ffi::c_int
            } else {
                4 as ::core::ffi::c_int
            }
        } else {
            (mb_y & 1 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
        };
        let mut intra_fdec: *mut crate::src::common::common::pixel = (*(&raw mut *(&raw mut (*h)
            .intra_border_backup
            as *mut [*mut crate::src::common::common::pixel; 3])
            .offset(fdec_idx as isize)
            as *mut *mut crate::src::common::common::pixel)
            .offset(i as isize))
        .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::common::pixel;
        let mut ref_pix_offset: [::core::ffi::c_int; 2] = [i_pix_offset, i_pix_offset];
        if mb_interlaced != 0 {
            ref_pix_offset[1 as ::core::ffi::c_int as usize] += (1 as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int * (mb_y & 1 as ::core::ffi::c_int))
                * i_stride;
        }
        (*h).mb.pic.i_stride[i as usize] = i_stride2;
        (*h).mb.pic.p_fenc_plane[i as usize] =
            (*(&raw mut (*(*h).fenc).plane as *mut *mut crate::src::common::common::pixel)
                .offset(i as isize))
            .offset(i_pix_offset as isize) as *mut crate::src::common::common::pixel;
        if b_chroma != 0 {
            (*h).mc
                .load_deinterleave_chroma_fenc
                .expect("non-null function pointer")(
                (*h).mb.pic.p_fenc[1 as ::core::ffi::c_int as usize],
                (*h).mb.pic.p_fenc_plane[1 as ::core::ffi::c_int as usize],
                i_stride2 as crate::stdlib::intptr_t,
                height,
            );
            crate::stdlib::memcpy(
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                    .offset(-(crate::src::common::common::FDEC_STRIDE as isize))
                    as *mut ::core::ffi::c_void,
                intra_fdec as *const ::core::ffi::c_void,
                (8 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                    .offset(-(crate::src::common::common::FDEC_STRIDE as isize))
                    as *mut ::core::ffi::c_void,
                intra_fdec.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                (8 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            *(*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(
                (-crate::src::common::common::FDEC_STRIDE - 1 as ::core::ffi::c_int) as isize,
            ) = *intra_fdec.offset((-(1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int) as isize);
            *(*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(
                (-crate::src::common::common::FDEC_STRIDE - 1 as ::core::ffi::c_int) as isize,
            ) = *intra_fdec.offset(-(1 as ::core::ffi::c_int) as isize);
        } else {
            (*h).mc.copy[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                (*h).mb.pic.p_fenc[i as usize],
                crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fenc_plane[i as usize],
                i_stride2 as crate::stdlib::intptr_t,
                16 as ::core::ffi::c_int,
            );
            crate::stdlib::memcpy(
                (*h).mb.pic.p_fdec[i as usize]
                    .offset(-(crate::src::common::common::FDEC_STRIDE as isize))
                    as *mut ::core::ffi::c_void,
                intra_fdec as *const ::core::ffi::c_void,
                (24 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            *(*h).mb.pic.p_fdec[i as usize].offset(
                (-crate::src::common::common::FDEC_STRIDE - 1 as ::core::ffi::c_int) as isize,
            ) = *intra_fdec.offset(-(1 as ::core::ffi::c_int) as isize);
        }
        if b_mbaff != 0 || (*h).mb.b_reencode_mb != 0 {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < height {
                if b_chroma != 0 {
                    *(*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(
                        (-(1 as ::core::ffi::c_int) + j * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) = *plane_fdec.offset((-(2 as ::core::ffi::c_int) + j * i_stride2) as isize);
                    *(*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(
                        (-(1 as ::core::ffi::c_int) + j * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) = *plane_fdec.offset((-(1 as ::core::ffi::c_int) + j * i_stride2) as isize);
                } else {
                    *(*h).mb.pic.p_fdec[i as usize].offset(
                        (-(1 as ::core::ffi::c_int) + j * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) = *plane_fdec.offset((-(1 as ::core::ffi::c_int) + j * i_stride2) as isize);
                }
                j += 1;
            }
        }
        let mut plane_src: *mut crate::src::common::common::pixel =
            ::core::ptr::null_mut::<crate::src::common::common::pixel>();
        let mut filtered_src: *mut *mut crate::src::common::common::pixel =
            ::core::ptr::null_mut::<*mut crate::src::common::common::pixel>();
        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_0 < (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] {
            if mb_interlaced != 0 {
                plane_src = (*(*h).fref[0 as ::core::ffi::c_int as usize]
                    [(j_0 >> 1 as ::core::ffi::c_int) as usize])
                    .plane_fld[i as usize];
                filtered_src = &raw mut *(&raw mut (**(&raw mut *(&raw mut (*h).fref
                    as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut *mut crate::src::common::frame::x264_frame_t)
                    .offset((j_0 >> 1 as ::core::ffi::c_int) as isize))
                .filtered_fld
                    as *mut [*mut crate::src::common::common::pixel; 4])
                    .offset(i as isize)
                    as *mut *mut crate::src::common::common::pixel;
            } else {
                plane_src =
                    (*(*h).fref[0 as ::core::ffi::c_int as usize][j_0 as usize]).plane[i as usize];
                filtered_src = &raw mut *(&raw mut (**(&raw mut *(&raw mut (*h).fref
                    as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut *mut crate::src::common::frame::x264_frame_t)
                    .offset(j_0 as isize))
                .filtered
                    as *mut [*mut crate::src::common::common::pixel; 4])
                    .offset(i as isize)
                    as *mut *mut crate::src::common::common::pixel;
            }
            (*h).mb.pic.p_fref[0 as ::core::ffi::c_int as usize][j_0 as usize]
                [(i * 4 as ::core::ffi::c_int) as usize] =
                plane_src.offset(ref_pix_offset[(j_0 & 1 as ::core::ffi::c_int) as usize] as isize);
            if b_chroma == 0 {
                if (*h).param.analyse.i_subpel_refine != 0 {
                    let mut k: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    while k < 4 as ::core::ffi::c_int {
                        (*h).mb.pic.p_fref[0 as ::core::ffi::c_int as usize][j_0 as usize]
                            [(i * 4 as ::core::ffi::c_int + k) as usize] = (*filtered_src
                            .offset(k as isize))
                        .offset(ref_pix_offset[(j_0 & 1 as ::core::ffi::c_int) as usize] as isize);
                        k += 1;
                    }
                }
                if i == 0 {
                    if !(*h).sh.weight[j_0 as usize][0 as ::core::ffi::c_int as usize]
                        .weightfn
                        .is_null()
                    {
                        (*h).mb.pic.p_fref_w[j_0 as usize] = (*(&raw mut (*(*h).fenc).weighted
                            as *mut *mut crate::src::common::common::pixel)
                            .offset((j_0 >> mb_interlaced) as isize))
                        .offset(
                            *(&raw mut ref_pix_offset as *mut ::core::ffi::c_int)
                                .offset((j_0 & 1 as ::core::ffi::c_int) as isize)
                                as isize,
                        )
                            as *mut crate::src::common::common::pixel;
                    } else {
                        (*h).mb.pic.p_fref_w[j_0 as usize] = (*h).mb.pic.p_fref
                            [0 as ::core::ffi::c_int as usize][j_0 as usize]
                            [0 as ::core::ffi::c_int as usize];
                    }
                }
            }
            j_0 += 1;
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
            let mut j_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_1 < (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] {
                if mb_interlaced != 0 {
                    plane_src = (*(*h).fref[1 as ::core::ffi::c_int as usize]
                        [(j_1 >> 1 as ::core::ffi::c_int) as usize])
                        .plane_fld[i as usize];
                    filtered_src = &raw mut *(&raw mut (**(&raw mut *(&raw mut (*h).fref
                        as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut *mut crate::src::common::frame::x264_frame_t)
                        .offset((j_1 >> 1 as ::core::ffi::c_int) as isize))
                    .filtered_fld
                        as *mut [*mut crate::src::common::common::pixel; 4])
                        .offset(i as isize)
                        as *mut *mut crate::src::common::common::pixel;
                } else {
                    plane_src = (*(*h).fref[1 as ::core::ffi::c_int as usize][j_1 as usize]).plane
                        [i as usize];
                    filtered_src = &raw mut *(&raw mut (**(&raw mut *(&raw mut (*h).fref
                        as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut *mut crate::src::common::frame::x264_frame_t)
                        .offset(j_1 as isize))
                    .filtered
                        as *mut [*mut crate::src::common::common::pixel; 4])
                        .offset(i as isize)
                        as *mut *mut crate::src::common::common::pixel;
                }
                (*h).mb.pic.p_fref[1 as ::core::ffi::c_int as usize][j_1 as usize]
                    [(i * 4 as ::core::ffi::c_int) as usize] = plane_src
                    .offset(ref_pix_offset[(j_1 & 1 as ::core::ffi::c_int) as usize] as isize);
                if b_chroma == 0 && (*h).param.analyse.i_subpel_refine != 0 {
                    let mut k_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    while k_0 < 4 as ::core::ffi::c_int {
                        (*h).mb.pic.p_fref[1 as ::core::ffi::c_int as usize][j_1 as usize]
                            [(i * 4 as ::core::ffi::c_int + k_0) as usize] = (*filtered_src
                            .offset(k_0 as isize))
                        .offset(ref_pix_offset[(j_1 & 1 as ::core::ffi::c_int) as usize] as isize);
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
        intra: [
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        nnz: [
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        nnz_chroma: [
            (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
        ],
        mv: [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        ref_0: [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
    },
    crate::src::common::common::x264_left_table_t {
        intra: [
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        nnz: [
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        nnz_chroma: [
            (16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (32 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (32 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
        ],
        mv: [
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        ref_0: [
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
    },
    crate::src::common::common::x264_left_table_t {
        intra: [
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        nnz: [
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        nnz_chroma: [
            (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
        ],
        mv: [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        ref_0: [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
    },
    crate::src::common::common::x264_left_table_t {
        intra: [
            4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            6 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        nnz: [
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            7 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            15 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        nnz_chroma: [
            (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
            (32 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as crate::stdlib::uint8_t,
        ],
        mv: [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
        ref_0: [
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
            1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        ],
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
        let mb_interlaced: ::core::ffi::c_int =
            (b_interlaced != 0 && (*h).mb.b_interlaced != 0) as ::core::ffi::c_int;
        let mut top_y: ::core::ffi::c_int = mb_y - ((1 as ::core::ffi::c_int) << mb_interlaced);
        let mut top: ::core::ffi::c_int = top_y * (*h).mb.i_mb_stride + mb_x;
        (*h).mb.i_mb_x = mb_x;
        (*h).mb.i_mb_y = mb_y;
        (*h).mb.i_mb_xy = mb_y * (*h).mb.i_mb_stride + mb_x;
        (*h).mb.i_b8_xy = 2 as ::core::ffi::c_int * (mb_y * (*h).mb.i_b8_stride + mb_x);
        (*h).mb.i_b4_xy = 4 as ::core::ffi::c_int * (mb_y * (*h).mb.i_b4_stride + mb_x);
        (*h).mb.left_b8[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
        (*h).mb.left_b8[0 as ::core::ffi::c_int as usize] =
            (*h).mb.left_b8[1 as ::core::ffi::c_int as usize];
        (*h).mb.left_b4[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
        (*h).mb.left_b4[0 as ::core::ffi::c_int as usize] =
            (*h).mb.left_b4[1 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour = 0 as ::core::ffi::c_uint;
        (*h).mb.i_neighbour_intra = 0 as ::core::ffi::c_uint;
        (*h).mb.i_neighbour_frame = 0 as ::core::ffi::c_uint;
        (*h).mb.i_mb_top_xy = -(1 as ::core::ffi::c_int);
        (*h).mb.i_mb_top_y = -(1 as ::core::ffi::c_int);
        (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
        (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] =
            (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize];
        (*h).mb.i_mb_topleft_xy = -(1 as ::core::ffi::c_int);
        (*h).mb.i_mb_topright_xy = -(1 as ::core::ffi::c_int);
        (*h).mb.i_mb_type_top = -(1 as ::core::ffi::c_int);
        (*h).mb.i_mb_type_left[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
        (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize] =
            (*h).mb.i_mb_type_left[1 as ::core::ffi::c_int as usize];
        (*h).mb.i_mb_type_topleft = -(1 as ::core::ffi::c_int);
        (*h).mb.i_mb_type_topright = -(1 as ::core::ffi::c_int);
        (*h).mb.left_index_table = (&raw const left_indices
            as *const crate::src::common::common::x264_left_table_t)
            .offset(3 as ::core::ffi::c_int as isize)
            as *const crate::src::common::common::x264_left_table_t;
        (*h).mb.topleft_partition = 0 as ::core::ffi::c_int;
        let mut topleft_y: ::core::ffi::c_int = top_y;
        let mut topright_y: ::core::ffi::c_int = top_y;
        let mut left: [::core::ffi::c_int; 2] = [0; 2];
        left[1 as ::core::ffi::c_int as usize] = (*h).mb.i_mb_xy - 1 as ::core::ffi::c_int;
        left[0 as ::core::ffi::c_int as usize] = left[1 as ::core::ffi::c_int as usize];
        (*h).mb.left_b8[1 as ::core::ffi::c_int as usize] =
            (*h).mb.i_b8_xy - 2 as ::core::ffi::c_int;
        (*h).mb.left_b8[0 as ::core::ffi::c_int as usize] =
            (*h).mb.left_b8[1 as ::core::ffi::c_int as usize];
        (*h).mb.left_b4[1 as ::core::ffi::c_int as usize] =
            (*h).mb.i_b4_xy - 4 as ::core::ffi::c_int;
        (*h).mb.left_b4[0 as ::core::ffi::c_int as usize] =
            (*h).mb.left_b4[1 as ::core::ffi::c_int as usize];
        if b_interlaced != 0 {
            (*h).mb.i_mb_top_mbpair_xy =
                (*h).mb.i_mb_xy - 2 as ::core::ffi::c_int * (*h).mb.i_mb_stride;
            (*h).mb.i_mb_topleft_y = -(1 as ::core::ffi::c_int);
            (*h).mb.i_mb_topright_y = -(1 as ::core::ffi::c_int);
            if mb_y & 1 as ::core::ffi::c_int != 0 {
                if mb_x != 0
                    && mb_interlaced
                        != *(*h)
                            .mb
                            .field
                            .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                {
                    left[1 as ::core::ffi::c_int as usize] =
                        (*h).mb.i_mb_xy - 1 as ::core::ffi::c_int - (*h).mb.i_mb_stride;
                    left[0 as ::core::ffi::c_int as usize] = left[1 as ::core::ffi::c_int as usize];
                    (*h).mb.left_b8[1 as ::core::ffi::c_int as usize] = (*h).mb.i_b8_xy
                        - 2 as ::core::ffi::c_int
                        - 2 as ::core::ffi::c_int * (*h).mb.i_b8_stride;
                    (*h).mb.left_b8[0 as ::core::ffi::c_int as usize] =
                        (*h).mb.left_b8[1 as ::core::ffi::c_int as usize];
                    (*h).mb.left_b4[1 as ::core::ffi::c_int as usize] = (*h).mb.i_b4_xy
                        - 4 as ::core::ffi::c_int
                        - 4 as ::core::ffi::c_int * (*h).mb.i_b4_stride;
                    (*h).mb.left_b4[0 as ::core::ffi::c_int as usize] =
                        (*h).mb.left_b4[1 as ::core::ffi::c_int as usize];
                    if mb_interlaced != 0 {
                        (*h).mb.left_index_table = (&raw const left_indices
                            as *const crate::src::common::common::x264_left_table_t)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *const crate::src::common::common::x264_left_table_t;
                        left[1 as ::core::ffi::c_int as usize] += (*h).mb.i_mb_stride;
                        (*h).mb.left_b8[1 as ::core::ffi::c_int as usize] +=
                            2 as ::core::ffi::c_int * (*h).mb.i_b8_stride;
                        (*h).mb.left_b4[1 as ::core::ffi::c_int as usize] +=
                            4 as ::core::ffi::c_int * (*h).mb.i_b4_stride;
                    } else {
                        (*h).mb.left_index_table = (&raw const left_indices
                            as *const crate::src::common::common::x264_left_table_t)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *const crate::src::common::common::x264_left_table_t;
                        topleft_y += 1;
                        (*h).mb.topleft_partition = 1 as ::core::ffi::c_int;
                    }
                }
                if mb_interlaced == 0 {
                    topright_y = -(1 as ::core::ffi::c_int);
                }
            } else {
                if mb_interlaced != 0 && top >= 0 as ::core::ffi::c_int {
                    if *(*h).mb.field.offset(top as isize) == 0 {
                        top += (*h).mb.i_mb_stride;
                        top_y += 1;
                    }
                    if mb_x != 0 {
                        topleft_y += (*(*h).mb.field.offset(
                            ((*h).mb.i_mb_stride * topleft_y + mb_x - 1 as ::core::ffi::c_int)
                                as isize,
                        ) == 0) as ::core::ffi::c_int;
                    }
                    if mb_x < (*h).mb.i_mb_width - 1 as ::core::ffi::c_int {
                        topright_y += (*(*h).mb.field.offset(
                            ((*h).mb.i_mb_stride * topright_y + mb_x + 1 as ::core::ffi::c_int)
                                as isize,
                        ) == 0) as ::core::ffi::c_int;
                    }
                }
                if mb_x != 0
                    && mb_interlaced
                        != *(*h)
                            .mb
                            .field
                            .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                {
                    if mb_interlaced != 0 {
                        (*h).mb.left_index_table = (&raw const left_indices
                            as *const crate::src::common::common::x264_left_table_t)
                            .offset(2 as ::core::ffi::c_int as isize)
                            as *const crate::src::common::common::x264_left_table_t;
                        left[1 as ::core::ffi::c_int as usize] += (*h).mb.i_mb_stride;
                        (*h).mb.left_b8[1 as ::core::ffi::c_int as usize] +=
                            2 as ::core::ffi::c_int * (*h).mb.i_b8_stride;
                        (*h).mb.left_b4[1 as ::core::ffi::c_int as usize] +=
                            4 as ::core::ffi::c_int * (*h).mb.i_b4_stride;
                    } else {
                        (*h).mb.left_index_table = (&raw const left_indices
                            as *const crate::src::common::common::x264_left_table_t)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *const crate::src::common::common::x264_left_table_t;
                    }
                }
            }
        }
        if mb_x > 0 as ::core::ffi::c_int {
            (*h).mb.i_neighbour_frame |= crate::src::common::macroblock::MB_LEFT
                as ::core::ffi::c_int
                as ::core::ffi::c_uint;
            (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] =
                left[0 as ::core::ffi::c_int as usize];
            (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] =
                left[1 as ::core::ffi::c_int as usize];
            (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize] = *(*h)
                .mb
                .type_0
                .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                as ::core::ffi::c_int;
            (*h).mb.i_mb_type_left[1 as ::core::ffi::c_int as usize] = *(*h)
                .mb
                .type_0
                .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
                as ::core::ffi::c_int;
            if *(*h)
                .mb
                .slice_table
                .offset(left[0 as ::core::ffi::c_int as usize] as isize)
                == (*h).sh.i_first_mb as crate::stdlib::int32_t
            {
                (*h).mb.i_neighbour |= crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                    as ::core::ffi::c_uint;
                if (*h).param.b_constrained_intra == 0
                    || ((*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                        == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                            == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                            == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                            == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                {
                    (*h).mb.i_neighbour_intra |= crate::src::common::macroblock::MB_LEFT
                        as ::core::ffi::c_int
                        as ::core::ffi::c_uint;
                }
            }
        }
        if (*h).i_threadslice_start >> mb_interlaced != mb_y >> mb_interlaced {
            if top >= 0 as ::core::ffi::c_int {
                (*h).mb.i_neighbour_frame |= crate::src::common::macroblock::MB_TOP
                    as ::core::ffi::c_int
                    as ::core::ffi::c_uint;
                (*h).mb.i_mb_top_xy = top;
                (*h).mb.i_mb_top_y = top_y;
                (*h).mb.i_mb_type_top =
                    *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int;
                if *(*h).mb.slice_table.offset(top as isize)
                    == (*h).sh.i_first_mb as crate::stdlib::int32_t
                {
                    (*h).mb.i_neighbour |= crate::src::common::macroblock::MB_TOP
                        as ::core::ffi::c_int
                        as ::core::ffi::c_uint;
                    if (*h).param.b_constrained_intra == 0
                        || ((*h).mb.i_mb_type_top
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_top
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_top
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_top
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    {
                        (*h).mb.i_neighbour_intra |= crate::src::common::macroblock::MB_TOP
                            as ::core::ffi::c_int
                            as ::core::ffi::c_uint;
                    }
                    (*h).mb.cbp.offset(top as isize) as *mut crate::stdlib::int16_t;
                    (&raw mut *(*h).mb.non_zero_count.offset(top as isize)
                        as *mut crate::stdlib::uint8_t)
                        .offset(12 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t;
                    (*h).mb.mb_transform_size.offset(top as isize) as *mut crate::stdlib::int8_t;
                    if (*h).param.b_cabac != 0 {
                        (*h).mb.skipbp.offset(top as isize) as *mut crate::stdlib::int8_t;
                    }
                }
            }
            if mb_x > 0 as ::core::ffi::c_int && topleft_y >= 0 as ::core::ffi::c_int {
                (*h).mb.i_neighbour_frame |= crate::src::common::macroblock::MB_TOPLEFT
                    as ::core::ffi::c_int
                    as ::core::ffi::c_uint;
                (*h).mb.i_mb_topleft_xy =
                    (*h).mb.i_mb_stride * topleft_y + mb_x - 1 as ::core::ffi::c_int;
                (*h).mb.i_mb_topleft_y = topleft_y;
                (*h).mb.i_mb_type_topleft =
                    *(*h).mb.type_0.offset((*h).mb.i_mb_topleft_xy as isize) as ::core::ffi::c_int;
                if *(*h).mb.slice_table.offset((*h).mb.i_mb_topleft_xy as isize)
                    == (*h).sh.i_first_mb as crate::stdlib::int32_t
                {
                    (*h).mb.i_neighbour |= crate::src::common::macroblock::MB_TOPLEFT
                        as ::core::ffi::c_int
                        as ::core::ffi::c_uint;
                    if (*h).param.b_constrained_intra == 0
                        || ((*h).mb.i_mb_type_topleft
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topleft
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topleft
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topleft
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    {
                        (*h).mb.i_neighbour_intra |= crate::src::common::macroblock::MB_TOPLEFT
                            as ::core::ffi::c_int
                            as ::core::ffi::c_uint;
                    }
                }
            }
            if mb_x < (*h).mb.i_mb_width - 1 as ::core::ffi::c_int
                && topright_y >= 0 as ::core::ffi::c_int
            {
                (*h).mb.i_neighbour_frame |= crate::src::common::macroblock::MB_TOPRIGHT
                    as ::core::ffi::c_int
                    as ::core::ffi::c_uint;
                (*h).mb.i_mb_topright_xy =
                    (*h).mb.i_mb_stride * topright_y + mb_x + 1 as ::core::ffi::c_int;
                (*h).mb.i_mb_topright_y = topright_y;
                (*h).mb.i_mb_type_topright =
                    *(*h).mb.type_0.offset((*h).mb.i_mb_topright_xy as isize) as ::core::ffi::c_int;
                if *(*h)
                    .mb
                    .slice_table
                    .offset((*h).mb.i_mb_topright_xy as isize)
                    == (*h).sh.i_first_mb as crate::stdlib::int32_t
                {
                    (*h).mb.i_neighbour |= crate::src::common::macroblock::MB_TOPRIGHT
                        as ::core::ffi::c_int
                        as ::core::ffi::c_uint;
                    if (*h).param.b_constrained_intra == 0
                        || ((*h).mb.i_mb_type_topright
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topright
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topright
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_topright
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    {
                        (*h).mb.i_neighbour_intra |= crate::src::common::macroblock::MB_TOPRIGHT
                            as ::core::ffi::c_int
                            as ::core::ffi::c_uint;
                    }
                }
            }
        }
    }
}

pub const LTOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

pub const LBOT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline(always)]

unsafe extern "C" fn macroblock_cache_load(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut b_mbaff: ::core::ffi::c_int,
) {
    unsafe {
        macroblock_cache_load_neighbours(h, mb_x, mb_y, b_mbaff);
        let mut left: *mut ::core::ffi::c_int =
            &raw mut (*h).mb.i_mb_left_xy as *mut ::core::ffi::c_int;
        let mut top: ::core::ffi::c_int = (*h).mb.i_mb_top_xy;
        let mut top_y: ::core::ffi::c_int = (*h).mb.i_mb_top_y;
        let mut s8x8: ::core::ffi::c_int = (*h).mb.i_b8_stride;
        let mut s4x4: ::core::ffi::c_int = (*h).mb.i_b4_stride;
        let mut top_8x8: ::core::ffi::c_int =
            (2 as ::core::ffi::c_int * top_y + 1 as ::core::ffi::c_int) * s8x8
                + 2 as ::core::ffi::c_int * mb_x;
        let mut top_4x4: ::core::ffi::c_int =
            (4 as ::core::ffi::c_int * top_y + 3 as ::core::ffi::c_int) * s4x4
                + 4 as ::core::ffi::c_int * mb_x;
        let mut lists: ::core::ffi::c_int =
            (1 as ::core::ffi::c_int) << (*h).sh.i_type & 3 as ::core::ffi::c_int;
        let mut i4x4: *mut [crate::stdlib::int8_t; 8] = (*h).mb.intra4x4_pred_mode;
        let mut nnz: *mut [crate::stdlib::uint8_t; 48] = (*h).mb.non_zero_count;
        let mut cbp: *mut crate::stdlib::int16_t = (*h).mb.cbp;
        let mut left_index_table: *const crate::src::common::common::x264_left_table_t =
            (*h).mb.left_index_table;
        (*h).mb.cache.deblock_strength = &raw mut *(*(&raw mut (*h).deblock_strength
            as *mut *mut [[[crate::stdlib::uint8_t; 4]; 8]; 2])
            .offset((mb_y & 1 as ::core::ffi::c_int) as isize))
        .offset(
            (if (*h).param.b_sliced_threads != 0 {
                (*h).mb.i_mb_xy
            } else {
                mb_x
            }) as isize,
        ) as *mut [[crate::stdlib::uint8_t; 4]; 8]
            as *mut [[crate::stdlib::uint8_t; 4]; 8];
        if (*h).mb.i_neighbour
            & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            (*h).mb.cache.i_cbp_top = *cbp.offset(top as isize) as ::core::ffi::c_int;
            (*((&raw mut (*h).mb.cache.intra4x4_pred_mode as *mut crate::stdlib::int8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::stdlib::int8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut *i4x4.offset(top as isize) as *mut crate::stdlib::int8_t)
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut *nnz.offset(top as isize) as *mut crate::stdlib::uint8_t)
                .offset(12 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset(16 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut *nnz.offset(top as isize) as *mut crate::stdlib::uint8_t).offset(
                (16 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    + (16 as ::core::ffi::c_int
                        >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                            as ::core::ffi::c_int)) as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset(32 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut *nnz.offset(top as isize) as *mut crate::stdlib::uint8_t).offset(
                (32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                    + (16 as ::core::ffi::c_int
                        >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                            as ::core::ffi::c_int)) as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            let mut l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while l < lists {
                (*(&raw mut (*h).mb.mv as *mut *mut [crate::stdlib::int16_t; 2]).offset(l as isize))
                    .offset((top_4x4 - 1 as ::core::ffi::c_int) as isize)
                    as *mut [crate::stdlib::int16_t; 2];
                (*(&raw mut (*h).mb.mv as *mut *mut [crate::stdlib::int16_t; 2]).offset(l as isize))
                    .offset((top_4x4 + 4 as ::core::ffi::c_int) as isize)
                    as *mut [crate::stdlib::int16_t; 2];
                (*(&raw mut (*h).mb.ref_0 as *mut *mut crate::stdlib::int8_t).offset(l as isize))
                    .offset((top_8x8 - 1 as ::core::ffi::c_int) as isize)
                    as *mut crate::stdlib::int8_t;
                if (*h).param.b_cabac != 0 {
                    (*(&raw mut (*h).mb.mvd as *mut *mut [[crate::stdlib::uint8_t; 2]; 8])
                        .offset(l as isize))
                    .offset(top as isize)
                        as *mut [[crate::stdlib::uint8_t; 2]; 8];
                }
                l += 1;
            }
        } else {
            (*h).mb.cache.i_cbp_top = -(1 as ::core::ffi::c_int);
            (*((&raw mut (*h).mb.cache.intra4x4_pred_mode as *mut crate::stdlib::int8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::stdlib::int8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = 0xffffffff as ::core::ffi::c_uint as crate::stdlib::uint32_t;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = 0x80808080 as ::core::ffi::c_uint as crate::stdlib::uint32_t;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset(16 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = 0x80808080 as ::core::ffi::c_uint as crate::stdlib::uint32_t;
            (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                    .offset(32 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = 0x80808080 as ::core::ffi::c_uint as crate::stdlib::uint32_t;
        }
        if (*h).mb.i_neighbour
            & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
        {
            let mut ltop: ::core::ffi::c_int = *left.offset(LTOP as isize);
            let mut lbot: ::core::ffi::c_int = if b_mbaff != 0 {
                *left.offset(LBOT as isize)
            } else {
                ltop
            };
            if b_mbaff != 0 {
                let top_luma: crate::stdlib::int16_t =
                    (*cbp.offset(ltop as isize) as ::core::ffi::c_int
                        >> ((*left_index_table).mv[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            & !(1 as ::core::ffi::c_int))
                        & 2 as ::core::ffi::c_int) as crate::stdlib::int16_t;
                let bot_luma: crate::stdlib::int16_t =
                    (*cbp.offset(lbot as isize) as ::core::ffi::c_int
                        >> ((*left_index_table).mv[2 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            & !(1 as ::core::ffi::c_int))
                        & 2 as ::core::ffi::c_int) as crate::stdlib::int16_t;
                (*h).mb.cache.i_cbp_left = *cbp.offset(ltop as isize) as ::core::ffi::c_int
                    & 0xfff0 as ::core::ffi::c_int
                    | (bot_luma as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | top_luma as ::core::ffi::c_int;
            } else {
                (*h).mb.cache.i_cbp_left = *cbp.offset(ltop as isize) as ::core::ffi::c_int;
            }
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*i4x4.offset(ltop as isize))
                [(*left_index_table).intra[0 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*i4x4.offset(ltop as isize))
                [(*left_index_table).intra[1 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*i4x4.offset(lbot as isize))
                [(*left_index_table).intra[2 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*i4x4.offset(lbot as isize))
                [(*left_index_table).intra[3 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(ltop as isize))
                [(*left_index_table).nnz[0 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(ltop as isize))
                [(*left_index_table).nnz[1 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(lbot as isize))
                [(*left_index_table).nnz[2 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(lbot as isize))
                [(*left_index_table).nnz[3 as ::core::ffi::c_int as usize] as usize];
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                >= crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
            {
                let mut offset: ::core::ffi::c_int = (4 as ::core::ffi::c_int
                    >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        || crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int)
                        as ::core::ffi::c_int)
                    - 4 as ::core::ffi::c_int;
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(ltop as isize))[((*left_index_table).nnz
                    [0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                    + offset)
                    as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(ltop as isize))[((*left_index_table).nnz
                    [1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                    + offset)
                    as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(lbot as isize))[((*left_index_table).nnz
                    [2 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                    + offset)
                    as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(lbot as isize))[((*left_index_table).nnz
                    [3 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                    + offset)
                    as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(ltop as isize))[((*left_index_table).nnz
                    [0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int
                    + offset)
                    as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(ltop as isize))[((*left_index_table).nnz
                    [1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int
                    + offset)
                    as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(lbot as isize))[((*left_index_table).nnz
                    [2 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int
                    + offset)
                    as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(32 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(lbot as isize))[((*left_index_table).nnz
                    [3 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int
                    + offset)
                    as usize];
            } else {
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(ltop as isize))
                    [(*left_index_table).nnz_chroma[0 as ::core::ffi::c_int as usize] as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(lbot as isize))
                    [(*left_index_table).nnz_chroma[1 as ::core::ffi::c_int as usize] as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(ltop as isize))
                    [(*left_index_table).nnz_chroma[2 as ::core::ffi::c_int as usize] as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz.offset(lbot as isize))
                    [(*left_index_table).nnz_chroma[3 as ::core::ffi::c_int as usize] as usize];
            }
        } else {
            (*h).mb.cache.i_cbp_left = -(1 as ::core::ffi::c_int);
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] =
                -(1 as ::core::ffi::c_int) as crate::stdlib::int8_t;
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.intra4x4_pred_mode[(x264_scan8
                [10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.intra4x4_pred_mode[(x264_scan8
                [8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.intra4x4_pred_mode[(x264_scan8
                [2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = 0x80 as crate::stdlib::uint8_t;
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                >= crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
            {
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(32 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = 0x80 as crate::stdlib::uint8_t;
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                    [(32 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                    [(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8
                    [(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                    [(16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize];
            }
        }
        if (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t)).b_transform_8x8_mode
            != 0
        {
            (*h).mb.cache.i_neighbour_transform_size = ((*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
                && *(*h)
                    .mb
                    .mb_transform_size
                    .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize)
                    as ::core::ffi::c_int
                    != 0)
                as ::core::ffi::c_int
                + ((*h).mb.i_neighbour
                    & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                    && *(*h).mb.mb_transform_size.offset(top as isize) as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int;
        }
        if b_mbaff != 0 {
            (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] =
                (*h).i_ref[0 as ::core::ffi::c_int as usize] << (*h).mb.b_interlaced;
            (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] =
                (*h).i_ref[1 as ::core::ffi::c_int as usize] << (*h).mb.b_interlaced;
        }
        if b_mbaff == 0 {
            x264_8_copy_column8(
                (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize]
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset(
                        (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ),
                (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize]
                    .offset(15 as ::core::ffi::c_int as isize)
                    .offset(
                        (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ),
            );
            x264_8_copy_column8(
                (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize]
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset(
                        (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ),
                (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize]
                    .offset(15 as ::core::ffi::c_int as isize)
                    .offset(
                        (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ),
            );
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        .offset(
                            (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                        .offset(15 as ::core::ffi::c_int as isize)
                        .offset(
                            (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                );
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        .offset(
                            (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                        .offset(15 as ::core::ffi::c_int as isize)
                        .offset(
                            (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                );
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        .offset(
                            (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                        .offset(15 as ::core::ffi::c_int as isize)
                        .offset(
                            (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                );
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        .offset(
                            (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                        .offset(15 as ::core::ffi::c_int as isize)
                        .offset(
                            (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                );
                macroblock_load_pic_pointers(
                    h,
                    mb_x,
                    mb_y,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                macroblock_load_pic_pointers(
                    h,
                    mb_x,
                    mb_y,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
            } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        .offset(
                            (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                        .offset(7 as ::core::ffi::c_int as isize)
                        .offset(
                            (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                );
                x264_8_copy_column8(
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        .offset(
                            (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                        .offset(7 as ::core::ffi::c_int as isize)
                        .offset(
                            (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                as isize,
                        ),
                );
                if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                {
                    x264_8_copy_column8(
                        (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                            .offset(-(1 as ::core::ffi::c_int as isize))
                            .offset(
                                (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                    as isize,
                            ),
                        (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                            .offset(7 as ::core::ffi::c_int as isize)
                            .offset(
                                (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                    as isize,
                            ),
                    );
                    x264_8_copy_column8(
                        (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                            .offset(-(1 as ::core::ffi::c_int as isize))
                            .offset(
                                (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                    as isize,
                            ),
                        (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                            .offset(7 as ::core::ffi::c_int as isize)
                            .offset(
                                (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                                    as isize,
                            ),
                    );
                }
                macroblock_load_pic_pointers(
                    h,
                    mb_x,
                    mb_y,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
            }
        } else {
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                macroblock_load_pic_pointers(
                    h,
                    mb_x,
                    mb_y,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                macroblock_load_pic_pointers(
                    h,
                    mb_x,
                    mb_y,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                macroblock_load_pic_pointers(
                    h,
                    mb_x,
                    mb_y,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            }
        }
        if !(*(*h).fdec).integral.is_null() {
            let mut offset_0: ::core::ffi::c_int = 16 as ::core::ffi::c_int
                * (mb_x + mb_y * (*(*h).fdec).i_stride[0 as ::core::ffi::c_int as usize]);
            let mut list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while list < 2 as ::core::ffi::c_int {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < (*h).mb.pic.i_fref[list as usize] {
                    (*h).mb.pic.p_integral[list as usize][i as usize] =
                        (**(&raw mut *(&raw mut (*h).fref
                            as *mut [*mut crate::src::common::frame::x264_frame_t; 19])
                            .offset(list as isize)
                            as *mut *mut crate::src::common::frame::x264_frame_t)
                            .offset(i as isize))
                        .integral
                        .offset(offset_0 as isize)
                            as *mut crate::stdlib::uint16_t;
                    i += 1;
                }
                list += 1;
            }
        }
        x264_8_prefetch_fenc(h, (*h).fenc, mb_x, mb_y);
        let mut l_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while l_0 < lists {
            let mut mv: *mut [crate::stdlib::int16_t; 2] = (*h).mb.mv[l_0 as usize];
            let mut ref_0: *mut crate::stdlib::int8_t = (*h).mb.ref_0[l_0 as usize];
            let mut i8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
            if (*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            {
                let mut ir: ::core::ffi::c_int = if b_mbaff != 0 {
                    2 as ::core::ffi::c_int
                        * (s8x8 * (*h).mb.i_mb_topleft_y + mb_x - 1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int
                        + s8x8
                } else {
                    top_8x8 - 1 as ::core::ffi::c_int
                };
                let mut iv: ::core::ffi::c_int = if b_mbaff != 0 {
                    4 as ::core::ffi::c_int
                        * (s4x4 * (*h).mb.i_mb_topleft_y + mb_x - 1 as ::core::ffi::c_int)
                        + 3 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int * s4x4
                } else {
                    top_4x4 - 1 as ::core::ffi::c_int
                };
                if b_mbaff != 0 && (*h).mb.topleft_partition != 0 {
                    iv -= 2 as ::core::ffi::c_int * s4x4;
                    ir -= s8x8;
                }
                (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = *ref_0.offset(ir as isize);
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l_0 as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(i8 as isize) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*(&raw mut *mv.offset(iv as isize) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
            } else {
                (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] =
                    -(2 as ::core::ffi::c_int) as crate::stdlib::int8_t;
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l_0 as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(i8 as isize) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = 0 as crate::stdlib::uint32_t;
            }
            i8 = x264_scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int;
            if (*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            {
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 1 as ::core::ffi::c_int) as usize] =
                    *ref_0.offset((top_8x8 + 0 as ::core::ffi::c_int) as isize);
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 0 as ::core::ffi::c_int) as usize] =
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 1 as ::core::ffi::c_int) as usize];
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 3 as ::core::ffi::c_int) as usize] =
                    *ref_0.offset((top_8x8 + 1 as ::core::ffi::c_int) as isize);
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 2 as ::core::ffi::c_int) as usize] =
                    (*h).mb.cache.ref_0[l_0 as usize][(i8 + 3 as ::core::ffi::c_int) as usize];
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l_0 as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(i8 as isize) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = (*(&raw mut *mv.offset(top_4x4 as isize) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union128_t))
                    .i;
            } else {
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l_0 as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(i8 as isize) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                (*((&raw mut *(&raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40])
                    .offset(l_0 as isize) as *mut crate::stdlib::int8_t)
                    .offset(i8 as isize) as *mut crate::stdlib::int8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (-(2 as ::core::ffi::c_int) as crate::stdlib::uint8_t
                    as ::core::ffi::c_uint)
                    .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                    as crate::stdlib::uint32_t;
            }
            i8 = x264_scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
            if (*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            {
                let mut ir_0: ::core::ffi::c_int = if b_mbaff != 0 {
                    2 as ::core::ffi::c_int
                        * (s8x8 * (*h).mb.i_mb_topright_y + (mb_x + 1 as ::core::ffi::c_int))
                        + s8x8
                } else {
                    top_8x8 + 2 as ::core::ffi::c_int
                };
                let mut iv_0: ::core::ffi::c_int = if b_mbaff != 0 {
                    4 as ::core::ffi::c_int
                        * (s4x4 * (*h).mb.i_mb_topright_y + (mb_x + 1 as ::core::ffi::c_int))
                        + 3 as ::core::ffi::c_int * s4x4
                } else {
                    top_4x4 + 4 as ::core::ffi::c_int
                };
                (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = *ref_0.offset(ir_0 as isize);
                (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(l_0 as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(i8 as isize) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*(&raw mut *mv.offset(iv_0 as isize) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
            } else {
                (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] =
                    -(2 as ::core::ffi::c_int) as crate::stdlib::int8_t;
            }
            i8 = x264_scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int;
            if (*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            {
                if b_mbaff != 0 {
                    (*h).mb.cache.ref_0[l_0 as usize]
                        [(i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[LTOP as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8
                                    * (*left_index_table).ref_0[0 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int) as isize,
                        );
                    (*h).mb.cache.ref_0[l_0 as usize]
                        [(i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[LTOP as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8
                                    * (*left_index_table).ref_0[1 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int) as isize,
                        );
                    (*h).mb.cache.ref_0[l_0 as usize]
                        [(i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[LBOT as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8
                                    * (*left_index_table).ref_0[2 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int) as isize,
                        );
                    (*h).mb.cache.ref_0[l_0 as usize]
                        [(i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[LBOT as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8
                                    * (*left_index_table).ref_0[3 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int) as isize,
                        );
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(0 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(0 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(1 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(1 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                } else {
                    let ir_1: ::core::ffi::c_int = (*h).mb.i_b8_xy - 1 as ::core::ffi::c_int;
                    let iv_1: ::core::ffi::c_int = (*h).mb.i_b4_xy - 1 as ::core::ffi::c_int;
                    (*h).mb.cache.ref_0[l_0 as usize]
                        [(i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        *ref_0.offset((ir_1 + 0 as ::core::ffi::c_int * s8x8) as isize);
                    (*h).mb.cache.ref_0[l_0 as usize]
                        [(i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        (*h).mb.cache.ref_0[l_0 as usize]
                            [(i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
                    (*h).mb.cache.ref_0[l_0 as usize]
                        [(i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        *ref_0.offset((ir_1 + 1 as ::core::ffi::c_int * s8x8) as isize);
                    (*h).mb.cache.ref_0[l_0 as usize]
                        [(i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        (*h).mb.cache.ref_0[l_0 as usize]
                            [(i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv
                        .offset((iv_1 + 0 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv
                        .offset((iv_1 + 1 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv
                        .offset((iv_1 + 2 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv
                        .offset((iv_1 + 3 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                }
            } else {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < 4 as ::core::ffi::c_int {
                    (*h).mb.cache.ref_0[l_0 as usize]
                        [(i8 + i_0 * 8 as ::core::ffi::c_int) as usize] =
                        -(2 as ::core::ffi::c_int) as crate::stdlib::int8_t;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + i_0 * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = 0 as crate::stdlib::uint32_t;
                    i_0 += 1;
                }
            }
            if b_mbaff != 0
                && (*h).mb.i_neighbour
                    & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
            {
                if (*h).mb.b_interlaced != 0
                    && *(*h)
                        .mb
                        .field
                        .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                        == 0
                {
                    (*h).mb.cache.topright_ref[l_0 as usize][0 as ::core::ffi::c_int as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8 * 0 as ::core::ffi::c_int)
                                as isize,
                        );
                    (*h).mb.cache.topright_ref[l_0 as usize][1 as ::core::ffi::c_int as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8 * 1 as ::core::ffi::c_int)
                                as isize,
                        );
                    (*h).mb.cache.topright_ref[l_0 as usize][2 as ::core::ffi::c_int as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[1 as ::core::ffi::c_int as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8 * 0 as ::core::ffi::c_int)
                                as isize,
                        );
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(0 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4
                                * (*(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int)) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(0 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4
                                * (*(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int)) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(1 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4
                                * (*(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int)) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                } else if (*h).mb.b_interlaced == 0
                    && *(*h)
                        .mb
                        .field
                        .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        != 0
                {
                    (*h).mb.cache.topright_ref[l_0 as usize][0 as ::core::ffi::c_int as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8 * 2 as ::core::ffi::c_int
                                + s8x8
                                    * (*left_index_table).ref_0[0 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int) as isize,
                        );
                    (*h).mb.cache.topright_ref[l_0 as usize][1 as ::core::ffi::c_int as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8 * 2 as ::core::ffi::c_int
                                + s8x8
                                    * (*left_index_table).ref_0[1 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int) as isize,
                        );
                    (*h).mb.cache.topright_ref[l_0 as usize][2 as ::core::ffi::c_int as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8 * 2 as ::core::ffi::c_int
                                + s8x8
                                    * (*left_index_table).ref_0[2 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int) as isize,
                        );
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(0 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4 * 4 as ::core::ffi::c_int
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(0 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4 * 4 as ::core::ffi::c_int
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.topright_mv
                        as *mut [[crate::stdlib::int16_t; 2]; 3])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(0 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4 * 4 as ::core::ffi::c_int
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                }
            }
            if (*h).param.b_cabac != 0 {
                let mut mvd: *mut [[crate::stdlib::uint8_t; 2]; 8] = (*h).mb.mvd[l_0 as usize];
                if (*h).mb.i_neighbour
                    & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                - 8 as ::core::ffi::c_int) as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = (*(&raw mut *(&raw mut *mvd.offset(top as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union64_t))
                        .i;
                } else {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                - 8 as ::core::ffi::c_int) as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = 0 as crate::stdlib::uint64_t;
                }
                if (*h).mb.i_neighbour
                    & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                    && (b_mbaff == 0
                        || (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int)
                {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *mvd
                        .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const (*left_index_table).intra as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *mvd
                        .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const (*left_index_table).intra as *const crate::stdlib::uint8_t)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                } else {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = 0 as crate::stdlib::uint16_t;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = 0 as crate::stdlib::uint16_t;
                }
                if (*h).mb.i_neighbour
                    & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                    && (b_mbaff == 0
                        || (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int)
                {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *mvd
                        .offset(*left.offset(1 as ::core::ffi::c_int as isize) as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const (*left_index_table).intra as *const crate::stdlib::uint8_t)
                                .offset(2 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(10 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int) as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *mvd
                        .offset(*left.offset(1 as ::core::ffi::c_int as isize) as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const (*left_index_table).intra as *const crate::stdlib::uint8_t)
                                .offset(3 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                } else {
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = 0 as crate::stdlib::uint16_t;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(l_0 as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = 0 as crate::stdlib::uint16_t;
                }
            }
            if b_mbaff != 0 {
                if (*h).mb.b_interlaced != 0 {
                    if (*h).mb.i_mb_topleft_xy >= 0 as ::core::ffi::c_int
                        && *(*h).mb.field.offset((*h).mb.i_mb_topleft_xy as isize) == 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if top >= 0 as ::core::ffi::c_int && *(*h).mb.field.offset(top as isize) == 0 {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if (*h).mb.i_mb_topright_xy >= 0 as ::core::ffi::c_int
                        && *(*h).mb.field.offset((*h).mb.i_mb_topright_xy as isize) == 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if *left.offset(0 as ::core::ffi::c_int as isize) >= 0 as ::core::ffi::c_int
                        && *(*h)
                            .mb
                            .field
                            .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize)
                            == 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize]
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize]
                                [0 as ::core::ffi::c_int as usize] = (((*h).mb.cache.topright_ref
                                [l_0 as usize][0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize]
                                [0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                                [l_0 as usize][0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize]
                            [1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize]
                                [1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.topright_ref
                                [l_0 as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize]
                                [1 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                                [l_0 as usize][1 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][1 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][1 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize]
                            [2 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize]
                                [2 as ::core::ffi::c_int as usize] = (((*h).mb.cache.topright_ref
                                [l_0 as usize][2 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize]
                                [2 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                                [l_0 as usize][2 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][2 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mvd
                                [l_0 as usize][2 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                    }
                } else {
                    if (*h).mb.i_mb_topleft_xy >= 0 as ::core::ffi::c_int
                        && *(*h).mb.field.offset((*h).mb.i_mb_topleft_xy as isize)
                            as ::core::ffi::c_int
                            != 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if top >= 0 as ::core::ffi::c_int
                        && *(*h).mb.field.offset(top as isize) as ::core::ffi::c_int != 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if (*h).mb.i_mb_topright_xy >= 0 as ::core::ffi::c_int
                        && *(*h).mb.field.offset((*h).mb.i_mb_topright_xy as isize)
                            as ::core::ffi::c_int
                            != 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                    }
                    if *left.offset(0 as ::core::ffi::c_int as isize) >= 0 as ::core::ffi::c_int
                        && *(*h)
                            .mb
                            .field
                            .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize)
                            as ::core::ffi::c_int
                            != 0
                    {
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] = ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.mv
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize]
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize]
                                [0 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_ref
                                [l_0 as usize][0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize]
                                [0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                                [l_0 as usize][0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize]
                            [1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize]
                                [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_ref
                                [l_0 as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize]
                                [1 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                                [l_0 as usize][1 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][1 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][1 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                        if (*h).mb.cache.topright_ref[l_0 as usize]
                            [2 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            >= 0 as ::core::ffi::c_int
                        {
                            (*h).mb.cache.topright_ref[l_0 as usize]
                                [2 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_ref
                                [l_0 as usize][2 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int)
                                as crate::stdlib::int8_t;
                            (*h).mb.cache.topright_mv[l_0 as usize]
                                [2 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                                [l_0 as usize][2 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int)
                                as crate::stdlib::int16_t;
                            (*h).mb.cache.mvd[l_0 as usize][2 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.mvd
                                [l_0 as usize][2 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int)
                                as crate::stdlib::uint8_t;
                        }
                    }
                }
            }
            l_0 += 1;
        }
        if b_mbaff != 0 && mb_x == 0 as ::core::ffi::c_int && mb_y & 1 as ::core::ffi::c_int == 0 {
            if (*h).mb.i_mb_top_xy >= (*h).sh.i_first_mb {
                (*h).mb.field_decoding_flag =
                    *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int;
            } else {
                (*h).mb.field_decoding_flag = 0 as ::core::ffi::c_int;
            }
        }
        (*h).mb.b_allow_skip = 1 as ::core::ffi::c_int;
        if b_mbaff != 0 {
            if (*h).mb.b_interlaced != (*h).mb.field_decoding_flag
                && mb_y & 1 as ::core::ffi::c_int != 0
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
                (*h).mb.b_allow_skip = 0 as ::core::ffi::c_int;
            }
        }
        if (*h).param.b_cabac != 0 {
            if b_mbaff != 0 {
                let mut left_xy: ::core::ffi::c_int = 0;
                let mut top_xy: ::core::ffi::c_int = 0;
                let mut mb_xy: ::core::ffi::c_int =
                    mb_x + (mb_y & !(1 as ::core::ffi::c_int)) * (*h).mb.i_mb_stride;
                left_xy = mb_xy - 1 as ::core::ffi::c_int;
                if mb_y & 1 as ::core::ffi::c_int != 0
                    && mb_x > 0 as ::core::ffi::c_int
                    && (*h).mb.field_decoding_flag
                        == *(*h).mb.field.offset(left_xy as isize) as ::core::ffi::c_int
                {
                    left_xy += (*h).mb.i_mb_stride;
                }
                if (*h).mb.field_decoding_flag != 0 {
                    top_xy = mb_xy - (*h).mb.i_mb_stride;
                    if mb_y & 1 as ::core::ffi::c_int == 0
                        && top_xy >= 0 as ::core::ffi::c_int
                        && *(*h).mb.slice_table.offset(top_xy as isize)
                            == (*h).sh.i_first_mb as crate::stdlib::int32_t
                        && *(*h).mb.field.offset(top_xy as isize) as ::core::ffi::c_int != 0
                    {
                        top_xy -= (*h).mb.i_mb_stride;
                    }
                } else {
                    top_xy = mb_x + (mb_y - 1 as ::core::ffi::c_int) * (*h).mb.i_mb_stride;
                }
                (*h).mb.cache.i_neighbour_skip = (mb_x > 0 as ::core::ffi::c_int
                    && *(*h).mb.slice_table.offset(left_xy as isize)
                        == (*h).sh.i_first_mb as crate::stdlib::int32_t
                    && !(*(*h).mb.type_0.offset(left_xy as isize) as ::core::ffi::c_int
                        == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                        || *(*h).mb.type_0.offset(left_xy as isize) as ::core::ffi::c_int
                            == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int))
                    as ::core::ffi::c_int
                    + (top_xy >= 0 as ::core::ffi::c_int
                        && *(*h).mb.slice_table.offset(top_xy as isize)
                            == (*h).sh.i_first_mb as crate::stdlib::int32_t
                        && !(*(*h).mb.type_0.offset(top_xy as isize) as ::core::ffi::c_int
                            == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset(top_xy as isize) as ::core::ffi::c_int
                                == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int))
                        as ::core::ffi::c_int;
            } else {
                (*h).mb.cache.i_neighbour_skip = ((*h).mb.i_neighbour
                    & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                    && !((*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                        == crate::src::common::macroblock::P_SKIP as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                            == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int))
                    as ::core::ffi::c_int
                    + ((*h).mb.i_neighbour
                        & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
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
                .offset((*h).mb.b_interlaced as isize)
                as *mut [[crate::stdlib::int8_t; 4]; 32])
                .offset(((*h).mb.b_interlaced & (mb_y & 1 as ::core::ffi::c_int)) as isize)
                as *mut [crate::stdlib::int8_t; 4]
                as *mut [crate::stdlib::int8_t; 4];
            (*h).mb.dist_scale_factor = &raw mut *(&raw mut *(&raw mut (*h).mb.dist_scale_factor_buf
                as *mut [[[crate::stdlib::int16_t; 4]; 32]; 2])
                .offset((*h).mb.b_interlaced as isize)
                as *mut [[crate::stdlib::int16_t; 4]; 32])
                .offset(((*h).mb.b_interlaced & (mb_y & 1 as ::core::ffi::c_int)) as isize)
                as *mut [crate::stdlib::int16_t; 4]
                as *mut [crate::stdlib::int16_t; 4];
            if (*h).param.b_cabac != 0 {
                let mut skipbp: crate::stdlib::uint8_t = 0;
                x264_macroblock_cache_skip(
                    h,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                if b_mbaff != 0 {
                    skipbp = (if (*h).mb.i_neighbour
                        & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                    {
                        *(*h).mb.skipbp.offset(*left.offset(LTOP as isize) as isize)
                            as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as crate::stdlib::uint8_t;
                    (*h).mb.cache.skip[(x264_scan8[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = (skipbp as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                            + ((*left_index_table).mv[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                & !(1 as ::core::ffi::c_int))
                        & 1 as ::core::ffi::c_int)
                        as crate::stdlib::int8_t;
                    skipbp = (if (*h).mb.i_neighbour
                        & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                    {
                        *(*h).mb.skipbp.offset(*left.offset(LBOT as isize) as isize)
                            as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as crate::stdlib::uint8_t;
                    (*h).mb.cache.skip[(x264_scan8[8 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = (skipbp as ::core::ffi::c_int
                        >> 1 as ::core::ffi::c_int
                            + ((*left_index_table).mv[2 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                & !(1 as ::core::ffi::c_int))
                        & 1 as ::core::ffi::c_int)
                        as crate::stdlib::int8_t;
                } else {
                    skipbp = (if (*h).mb.i_neighbour
                        & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        != 0
                    {
                        *(*h)
                            .mb
                            .skipbp
                            .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize)
                            as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as crate::stdlib::uint8_t;
                    (*h).mb.cache.skip[(x264_scan8[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = (skipbp as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int)
                        as crate::stdlib::int8_t;
                    (*h).mb.cache.skip[(x264_scan8[8 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = (skipbp as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int)
                        as crate::stdlib::int8_t;
                }
                skipbp = (if (*h).mb.i_neighbour
                    & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                {
                    *(*h).mb.skipbp.offset(top as isize) as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as crate::stdlib::uint8_t;
                (*h).mb.cache.skip[(x264_scan8[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as usize] = (skipbp as ::core::ffi::c_int
                    & 0x4 as ::core::ffi::c_int)
                    as crate::stdlib::int8_t;
                (*h).mb.cache.skip[(x264_scan8[4 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as usize] = (skipbp as ::core::ffi::c_int
                    & 0x8 as ::core::ffi::c_int)
                    as crate::stdlib::int8_t;
            }
        }
        if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int {
            crate::src::common::mvpred::x264_8_mb_predict_mv_pskip(
                h as *mut crate::src::common::common::x264_t,
                &raw mut (*h).mb.cache.pskip_mv as *mut crate::stdlib::int16_t,
            );
        }
        (*h).mb.i_neighbour8[0 as ::core::ffi::c_int as usize] = (*h).mb.i_neighbour_intra
            & (crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                | crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            | (if (*h).mb.i_neighbour_intra
                & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            {
                crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[0 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour8[0 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour4[1 as ::core::ffi::c_int as usize] =
            (crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                | (if (*h).mb.i_neighbour_intra
                    & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                {
                    crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                        | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
                        | crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[4 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour4[1 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour8[2 as ::core::ffi::c_int as usize] =
            (crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                | crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int
                | (if (*h).mb.i_neighbour_intra
                    & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                {
                    crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                        | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                })) as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[10 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour8[2 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour4[8 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour4[10 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour4[2 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour4[8 as ::core::ffi::c_int as usize];
        (*h).mb.i_neighbour8[1 as ::core::ffi::c_int as usize] =
            crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                | (*h).mb.i_neighbour_intra
                    & crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                | (if (*h).mb.i_neighbour_intra
                    & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    != 0
                {
                    crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                        | crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint;
        (*h).mb.i_neighbour4[5 as ::core::ffi::c_int as usize] =
            (*h).mb.i_neighbour8[1 as ::core::ffi::c_int as usize];
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_macroblock_cache_load_progressive(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) {
    unsafe {
        macroblock_cache_load(h, mb_x, mb_y, 0 as ::core::ffi::c_int);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_macroblock_cache_load_interlaced(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) {
    unsafe {
        macroblock_cache_load(h, mb_x, mb_y, 1 as ::core::ffi::c_int);
    }
}

unsafe extern "C" fn macroblock_deblock_strength_mbaff(
    mut h: *mut crate::src::common::common::x264_t,
    mut bs: *mut [[crate::stdlib::uint8_t; 4]; 8],
) {
    unsafe {
        if (*h).mb.i_neighbour
            & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && *(*h)
                .mb
                .field
                .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                as ::core::ffi::c_int
                != (*h).mb.b_interlaced
        {
            static mut offset: [[[crate::stdlib::uint8_t; 8]; 2]; 2] = [
                [
                    [
                        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                    ],
                    [
                        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                    ],
                ],
                [
                    [
                        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                    ],
                    [
                        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                        3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
                    ],
                ],
            ];
            let mut tmpbs: [crate::stdlib::uint8_t; 8] = [0; 8];
            let mut off: *const crate::stdlib::uint8_t = &raw const *(&raw const *(&raw const offset
                as *const [[crate::stdlib::uint8_t; 8]; 2])
                .offset((*h).mb.b_interlaced as isize)
                as *const [crate::stdlib::uint8_t; 8])
                .offset(((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) as isize)
                as *const crate::stdlib::uint8_t;
            let mut nnz: *mut [crate::stdlib::uint8_t; 48] = (*h).mb.non_zero_count;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 8 as ::core::ffi::c_int {
                let mut left: ::core::ffi::c_int =
                    (*h).mb.i_mb_left_xy[(if (*h).mb.b_interlaced != 0 {
                        i >> 2 as ::core::ffi::c_int
                    } else {
                        i & 1 as ::core::ffi::c_int
                    }) as usize];
                let mut nnz_this: ::core::ffi::c_int =
                    (*h).mb.cache.non_zero_count[(x264_scan8[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * (i >> 1 as ::core::ffi::c_int))
                        as usize] as ::core::ffi::c_int;
                let mut nnz_left: ::core::ffi::c_int =
                    (*nnz.offset(left as isize))[(3 as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int * *off.offset(i as isize) as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int;
                if (*h).param.b_cabac == 0
                    && (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                        .b_transform_8x8_mode
                        != 0
                {
                    let mut j: ::core::ffi::c_int =
                        *off.offset(i as isize) as ::core::ffi::c_int & !(1 as ::core::ffi::c_int);
                    if *(*h).mb.mb_transform_size.offset(left as isize) != 0 {
                        nnz_left = ((*((&raw mut *nnz.offset(left as isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(
                                (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * j) as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int
                            | (*((&raw mut *nnz.offset(left as isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset(
                                    (2 as ::core::ffi::c_int
                                        + 4 as ::core::ffi::c_int * (1 as ::core::ffi::c_int + j))
                                        as isize,
                                ) as *mut crate::stdlib::uint8_t
                                as *mut crate::src::common::base::x264_union16_t))
                                .i as ::core::ffi::c_int
                            != 0) as ::core::ffi::c_int;
                    }
                }
                tmpbs[i as usize] = (if nnz_left != 0 || nnz_this != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as crate::stdlib::uint8_t;
                i += 1;
            }
            if (*h).mb.b_interlaced != 0 {
                (*(&raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*((&raw mut tmpbs as *mut crate::stdlib::uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
                (*(&raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*((&raw mut tmpbs as *mut crate::stdlib::uint8_t)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
            } else {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < 4 as ::core::ffi::c_int {
                    (*bs.offset(0 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize][i_0 as usize] =
                        tmpbs[(2 as ::core::ffi::c_int * i_0) as usize];
                    i_0 += 1;
                }
                let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_1 < 4 as ::core::ffi::c_int {
                    (*bs.offset(0 as ::core::ffi::c_int as isize))
                        [4 as ::core::ffi::c_int as usize][i_1 as usize] =
                        tmpbs[(1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i_1) as usize];
                    i_1 += 1;
                }
            }
        }
        if (*h).mb.i_neighbour
            & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            && (*h).mb.b_interlaced
                != *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
        {
            if (*h).mb.i_mb_y & 1 as ::core::ffi::c_int == 0 && (*h).mb.b_interlaced == 0 {
                let mut mbn_xy: ::core::ffi::c_int =
                    (*h).mb.i_mb_xy - 2 as ::core::ffi::c_int * (*h).mb.i_mb_stride;
                let mut nnz_cur: *mut crate::stdlib::uint8_t =
                    (&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as isize,
                    ) as *mut crate::stdlib::uint8_t;
                let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j_0 < 2 as ::core::ffi::c_int {
                    let mut nnz_0: *mut [crate::stdlib::uint8_t; 48] = (*h).mb.non_zero_count;
                    let mut nnz_top: [crate::stdlib::uint8_t; 4] = [0; 4];
                    (*(&raw mut nnz_top as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*((&raw mut *nnz_0.offset(mbn_xy as isize)
                        as *mut crate::stdlib::uint8_t)
                        .offset((3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    if (*h).param.b_cabac == 0
                        && (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                            .b_transform_8x8_mode
                            != 0
                        && *(*h).mb.mb_transform_size.offset(mbn_xy as isize) as ::core::ffi::c_int
                            != 0
                    {
                        nnz_top[1 as ::core::ffi::c_int as usize] =
                            ((*((&raw mut *nnz_0.offset(mbn_xy as isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t
                                as *mut crate::src::common::base::x264_union16_t))
                                .i as ::core::ffi::c_int
                                != 0
                                || (*((&raw mut *nnz_0.offset(mbn_xy as isize)
                                    as *mut crate::stdlib::uint8_t)
                                    .offset(12 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t
                                    as *mut crate::src::common::base::x264_union16_t))
                                    .i as ::core::ffi::c_int
                                    != 0) as ::core::ffi::c_int
                                as crate::stdlib::uint8_t;
                        nnz_top[0 as ::core::ffi::c_int as usize] =
                            nnz_top[1 as ::core::ffi::c_int as usize];
                        nnz_top[3 as ::core::ffi::c_int as usize] =
                            ((*((&raw mut *nnz_0.offset(mbn_xy as isize)
                                as *mut crate::stdlib::uint8_t)
                                .offset(10 as ::core::ffi::c_int as isize)
                                as *mut crate::stdlib::uint8_t
                                as *mut crate::src::common::base::x264_union16_t))
                                .i as ::core::ffi::c_int
                                != 0
                                || (*((&raw mut *nnz_0.offset(mbn_xy as isize)
                                    as *mut crate::stdlib::uint8_t)
                                    .offset(14 as ::core::ffi::c_int as isize)
                                    as *mut crate::stdlib::uint8_t
                                    as *mut crate::src::common::base::x264_union16_t))
                                    .i as ::core::ffi::c_int
                                    != 0) as ::core::ffi::c_int
                                as crate::stdlib::uint8_t;
                        nnz_top[2 as ::core::ffi::c_int as usize] =
                            nnz_top[3 as ::core::ffi::c_int as usize];
                    }
                    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_2 < 4 as ::core::ffi::c_int {
                        (*bs.offset(1 as ::core::ffi::c_int as isize))
                            [(4 as ::core::ffi::c_int * j_0) as usize][i_2 as usize] =
                            (if *nnz_cur.offset(i_2 as isize) as ::core::ffi::c_int != 0
                                || nnz_top[i_2 as usize] as ::core::ffi::c_int != 0
                            {
                                2 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            }) as crate::stdlib::uint8_t;
                        i_2 += 1;
                    }
                    j_0 += 1;
                    mbn_xy += (*h).mb.i_mb_stride;
                }
            } else {
                let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_3 < 4 as ::core::ffi::c_int {
                    (*bs.offset(1 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize][i_3 as usize] =
                        (if (*bs.offset(1 as ::core::ffi::c_int as isize))
                            [0 as ::core::ffi::c_int as usize][i_3 as usize]
                            as ::core::ffi::c_int
                            > 1 as ::core::ffi::c_int
                        {
                            (*bs.offset(1 as ::core::ffi::c_int as isize))
                                [0 as ::core::ffi::c_int as usize][i_3 as usize]
                                as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        }) as crate::stdlib::uint8_t;
                    i_3 += 1;
                }
            }
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_macroblock_deblock_strength(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut bs: *mut [[crate::stdlib::uint8_t; 4]; 8] = (*h).mb.cache.deblock_strength;
        if (*h).mb.i_type == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int
        {
            (*(&raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                as *mut [crate::stdlib::uint8_t; 4])
                .offset(1 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = 0x3030303 as crate::stdlib::uint32_t;
            (*(&raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                as *mut [crate::stdlib::uint8_t; 4])
                .offset(2 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union64_t))
                .i = 0x303030303030303 as crate::stdlib::uint64_t;
            (*(&raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                as *mut [crate::stdlib::uint8_t; 4])
                .offset(1 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = 0x3030303 as crate::stdlib::uint32_t;
            (*(&raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                as *mut [crate::stdlib::uint8_t; 4])
                .offset(2 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union64_t))
                .i = 0x303030303030303 as crate::stdlib::uint64_t;
            return;
        }
        if (*h).mb.b_transform_8x8 != 0
            && !(crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int)
        {
            let mut cbp_mask: ::core::ffi::c_int = 0xf as ::core::ffi::c_int
                >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            if (*h).mb.i_cbp_luma & cbp_mask == cbp_mask {
                (*(&raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = 0x2020202 as crate::stdlib::uint32_t;
                (*(&raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = 0x2020202 as crate::stdlib::uint32_t;
                (*(&raw mut *(&raw mut *bs.offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = 0x2020202 as crate::stdlib::uint32_t;
                (*(&raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union64_t))
                    .i = 0x202020202020202 as crate::stdlib::uint64_t;
                (*(&raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union64_t))
                    .i = 0x202020202020202 as crate::stdlib::uint64_t;
                (*(&raw mut *(&raw mut *bs.offset(1 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 4])
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = 0x2020202 as crate::stdlib::uint32_t;
                return;
            }
        }
        let mut neighbour_changed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*h).sh.i_disable_deblocking_filter_idc != 2 as ::core::ffi::c_int {
            neighbour_changed =
                ((*h).mb.i_neighbour_frame & !(*h).mb.i_neighbour) as ::core::ffi::c_int;
            (*h).mb.i_neighbour = (*h).mb.i_neighbour_frame;
        }
        if (*h).sh.b_mbaff != 0
            && (*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            && *(*h)
                .mb
                .field
                .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                != (*h).mb.b_interlaced
        {
            (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] =
                (*h).mb.i_mb_xy - 1 as ::core::ffi::c_int;
            (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] =
                (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize];
            if (*h).mb.i_mb_y & 1 as ::core::ffi::c_int != 0 {
                (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] -= (*h).mb.i_mb_stride;
            } else {
                (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] += (*h).mb.i_mb_stride;
            }
        }
        if neighbour_changed != 0 {
            let mut top_y: ::core::ffi::c_int = (*h).mb.i_mb_top_y;
            let mut top_8x8: ::core::ffi::c_int =
                (2 as ::core::ffi::c_int * top_y + 1 as ::core::ffi::c_int) * (*h).mb.i_b8_stride
                    + 2 as ::core::ffi::c_int * (*h).mb.i_mb_x;
            let mut top_4x4: ::core::ffi::c_int =
                (4 as ::core::ffi::c_int * top_y + 3 as ::core::ffi::c_int) * (*h).mb.i_b4_stride
                    + 4 as ::core::ffi::c_int * (*h).mb.i_mb_x;
            let mut s8x8: ::core::ffi::c_int = (*h).mb.i_b8_stride;
            let mut s4x4: ::core::ffi::c_int = (*h).mb.i_b4_stride;
            let mut nnz: *mut [crate::stdlib::uint8_t; 48] = (*h).mb.non_zero_count;
            let mut left_index_table: *const crate::src::common::common::x264_left_table_t =
                if (*h).sh.b_mbaff != 0 {
                    (*h).mb.left_index_table
                } else {
                    (&raw const left_indices
                        as *const crate::src::common::common::x264_left_table_t)
                        .offset(3 as ::core::ffi::c_int as isize)
                        as *const crate::src::common::common::x264_left_table_t
                };
            if neighbour_changed & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int != 0
            {
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        - 8 as ::core::ffi::c_int) as isize,
                ) as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = (*((&raw mut *nnz.offset((*h).mb.i_mb_top_xy as isize)
                    as *mut crate::stdlib::uint8_t)
                    .offset(12 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i;
            }
            if neighbour_changed & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                != 0
            {
                let mut left: *mut ::core::ffi::c_int =
                    &raw mut (*h).mb.i_mb_left_xy as *mut ::core::ffi::c_int;
                (*h).mb.cache.non_zero_count[(x264_scan8[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz
                    .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize))
                    [(*left_index_table).nnz[0 as ::core::ffi::c_int as usize] as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8[2 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz
                    .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize))
                    [(*left_index_table).nnz[1 as ::core::ffi::c_int as usize] as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8[8 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz
                    .offset(*left.offset(1 as ::core::ffi::c_int as isize) as isize))
                    [(*left_index_table).nnz[2 as ::core::ffi::c_int as usize] as usize];
                (*h).mb.cache.non_zero_count[(x264_scan8[10 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int)
                    as usize] = (*nnz
                    .offset(*left.offset(1 as ::core::ffi::c_int as isize) as isize))
                    [(*left_index_table).nnz[3 as ::core::ffi::c_int as usize] as usize];
            }
            let mut l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while l
                <= ((*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            {
                let mut mv: *mut [crate::stdlib::int16_t; 2] = (*h).mb.mv[l as usize];
                let mut ref_0: *mut crate::stdlib::int8_t = (*h).mb.ref_0[l as usize];
                let mut i8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int;
                if neighbour_changed & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                    != 0
                {
                    (*h).mb.cache.ref_0[l as usize][(i8 + 1 as ::core::ffi::c_int) as usize] =
                        *ref_0.offset((top_8x8 + 0 as ::core::ffi::c_int) as isize);
                    (*h).mb.cache.ref_0[l as usize][(i8 + 0 as ::core::ffi::c_int) as usize] =
                        (*h).mb.cache.ref_0[l as usize][(i8 + 1 as ::core::ffi::c_int) as usize];
                    (*h).mb.cache.ref_0[l as usize][(i8 + 3 as ::core::ffi::c_int) as usize] =
                        *ref_0.offset((top_8x8 + 1 as ::core::ffi::c_int) as isize);
                    (*h).mb.cache.ref_0[l as usize][(i8 + 2 as ::core::ffi::c_int) as usize] =
                        (*h).mb.cache.ref_0[l as usize][(i8 + 3 as ::core::ffi::c_int) as usize];
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(i8 as isize) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union128_t))
                        .i =
                        (*(&raw mut *mv.offset(top_4x4 as isize) as *mut crate::stdlib::int16_t
                            as *mut crate::src::common::base::x264_union128_t))
                            .i;
                }
                i8 = x264_scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int;
                if neighbour_changed & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                    != 0
                {
                    (*h).mb.cache.ref_0[l as usize]
                        [(i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8
                                    * (*left_index_table).ref_0[0 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int) as isize,
                        );
                    (*h).mb.cache.ref_0[l as usize]
                        [(i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        (*h).mb.cache.ref_0[l as usize]
                            [(i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
                    (*h).mb.cache.ref_0[l as usize]
                        [(i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        *ref_0.offset(
                            ((*h).mb.left_b8[1 as ::core::ffi::c_int as usize]
                                + 1 as ::core::ffi::c_int
                                + s8x8
                                    * (*left_index_table).ref_0[2 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int) as isize,
                        );
                    (*h).mb.cache.ref_0[l as usize]
                        [(i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                        (*h).mb.cache.ref_0[l as usize]
                            [(i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(0 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(0 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(1 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                    (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(l as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset((i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize)
                        as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*(&raw mut *mv.offset(
                        (*(&raw mut (*h).mb.left_b4 as *mut ::core::ffi::c_int)
                            .offset(1 as ::core::ffi::c_int as isize)
                            + 3 as ::core::ffi::c_int
                            + s4x4
                                * *(&raw const (*left_index_table).mv
                                    as *const crate::stdlib::uint8_t)
                                    .offset(3 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                    ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union32_t))
                        .i;
                }
                l += 1;
            }
        }
        if (*h).param.analyse.i_weighted_pred == crate::x264_h::X264_WEIGHTP_SMART
            && (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_P as ::core::ffi::c_int
        {
            let mut i8_0: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int;
            (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 1 as ::core::ffi::c_int) as usize] =
                (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [(i8_0 + 0 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize];
            (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 0 as ::core::ffi::c_int) as usize] = (*h).mb.cache.ref_0
                [0 as ::core::ffi::c_int as usize][(i8_0 + 1 as ::core::ffi::c_int) as usize];
            (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 3 as ::core::ffi::c_int) as usize] =
                (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [(i8_0 + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize];
            (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 2 as ::core::ffi::c_int) as usize] = (*h).mb.cache.ref_0
                [0 as ::core::ffi::c_int as usize][(i8_0 + 3 as ::core::ffi::c_int) as usize];
            i8_0 = x264_scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int;
            (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [(i8_0 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize];
            (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [(i8_0 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
            (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [(i8_0 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize];
            (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [(i8_0 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
            let mut ref0: ::core::ffi::c_int =
                (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
            let mut ref1: ::core::ffi::c_int =
                (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[4 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
            let mut ref2: ::core::ffi::c_int =
                (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[8 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
            let mut ref3: ::core::ffi::c_int =
                (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
            let mut reftop: crate::stdlib::uint32_t = pack16to32(
                ref0 as crate::stdlib::uint8_t as crate::stdlib::uint32_t,
                ref1 as crate::stdlib::uint8_t as crate::stdlib::uint32_t,
            )
            .wrapping_mul(0x101 as crate::stdlib::uint32_t);
            let mut refbot: crate::stdlib::uint32_t = pack16to32(
                ref2 as crate::stdlib::uint8_t as crate::stdlib::uint32_t,
                ref3 as crate::stdlib::uint8_t as crate::stdlib::uint32_t,
            )
            .wrapping_mul(0x101 as crate::stdlib::uint32_t);
            (*((&raw mut *(&raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40])
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int8_t)
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
                        as isize,
                ) as *mut crate::stdlib::int8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = reftop;
            (*((&raw mut *(&raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40])
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int8_t)
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                        as isize,
                ) as *mut crate::stdlib::int8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = reftop;
            (*((&raw mut *(&raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40])
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int8_t)
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        as isize,
                ) as *mut crate::stdlib::int8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = refbot;
            (*((&raw mut *(&raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40])
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::stdlib::int8_t)
                .offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                        as isize,
                ) as *mut crate::stdlib::int8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = refbot;
        }
        if (*h).param.b_cabac == 0
            && (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                .b_transform_8x8_mode
                != 0
        {
            let mut nnz_0: *mut [crate::stdlib::uint8_t; 48] = (*h).mb.non_zero_count;
            let mut top: ::core::ffi::c_int = (*h).mb.i_mb_top_xy;
            let mut left_0: *mut ::core::ffi::c_int =
                &raw mut (*h).mb.i_mb_left_xy as *mut ::core::ffi::c_int;
            if (*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
                && *(*h).mb.mb_transform_size.offset(top as isize) as ::core::ffi::c_int != 0
            {
                let mut i8_1: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int;
                let mut nnz_top0: ::core::ffi::c_int = (*((&raw mut *nnz_0.offset(top as isize)
                    as *mut crate::stdlib::uint8_t)
                    .offset(8 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i as ::core::ffi::c_int
                    | (*((&raw mut *nnz_0.offset(top as isize) as *mut crate::stdlib::uint8_t)
                        .offset(12 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int;
                let mut nnz_top1: ::core::ffi::c_int = (*((&raw mut *nnz_0.offset(top as isize)
                    as *mut crate::stdlib::uint8_t)
                    .offset(10 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i as ::core::ffi::c_int
                    | (*((&raw mut *nnz_0.offset(top as isize) as *mut crate::stdlib::uint8_t)
                        .offset(14 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                    .offset((i8_1 + 0 as ::core::ffi::c_int) as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (if nnz_top0 != 0 {
                    0x101 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as crate::stdlib::uint16_t;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                    .offset((i8_1 + 2 as ::core::ffi::c_int) as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (if nnz_top1 != 0 {
                    0x101 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as crate::stdlib::uint16_t;
            }
            if (*h).mb.i_neighbour
                & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                != 0
            {
                let mut i8_2: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int;
                if *(*h)
                    .mb
                    .mb_transform_size
                    .offset(*left_0.offset(0 as ::core::ffi::c_int as isize) as isize)
                    != 0
                {
                    let mut nnz_left0: ::core::ffi::c_int = (*((&raw mut *nnz_0
                        .offset(*left_0.offset(0 as ::core::ffi::c_int as isize) as isize)
                        as *mut crate::stdlib::uint8_t)
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i
                        as ::core::ffi::c_int
                        | (*((&raw mut *nnz_0
                            .offset(*left_0.offset(0 as ::core::ffi::c_int as isize) as isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(6 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                    (*h).mb.cache.non_zero_count
                        [(i8_2 + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as usize] =
                        (nnz_left0 != 0) as ::core::ffi::c_int as crate::stdlib::uint8_t;
                    (*h).mb.cache.non_zero_count
                        [(i8_2 + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as usize] =
                        (nnz_left0 != 0) as ::core::ffi::c_int as crate::stdlib::uint8_t;
                }
                if *(*h)
                    .mb
                    .mb_transform_size
                    .offset(*left_0.offset(1 as ::core::ffi::c_int as isize) as isize)
                    != 0
                {
                    let mut nnz_left1: ::core::ffi::c_int = (*((&raw mut *nnz_0
                        .offset(*left_0.offset(1 as ::core::ffi::c_int as isize) as isize)
                        as *mut crate::stdlib::uint8_t)
                        .offset(10 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i
                        as ::core::ffi::c_int
                        | (*((&raw mut *nnz_0
                            .offset(*left_0.offset(1 as ::core::ffi::c_int as isize) as isize)
                            as *mut crate::stdlib::uint8_t)
                            .offset(14 as ::core::ffi::c_int as isize)
                            as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                    (*h).mb.cache.non_zero_count
                        [(i8_2 + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize] =
                        (nnz_left1 != 0) as ::core::ffi::c_int as crate::stdlib::uint8_t;
                    (*h).mb.cache.non_zero_count
                        [(i8_2 + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as usize] =
                        (nnz_left1 != 0) as ::core::ffi::c_int as crate::stdlib::uint8_t;
                }
            }
            if (*h).mb.b_transform_8x8 != 0 {
                let mut nnz0: ::core::ffi::c_int =
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                let mut nnz1: ::core::ffi::c_int =
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(4 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset(6 as ::core::ffi::c_int as isize)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                let mut nnz2: ::core::ffi::c_int =
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(8 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset(10 as ::core::ffi::c_int as isize)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                let mut nnz3: ::core::ffi::c_int =
                    (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(12 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                            .offset(
                                *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                    .offset(14 as ::core::ffi::c_int as isize)
                                    as isize,
                            ) as *mut crate::stdlib::uint8_t
                            as *mut crate::src::common::base::x264_union16_t))
                            .i as ::core::ffi::c_int;
                let mut nnztop: crate::stdlib::uint32_t = pack16to32(
                    (nnz0 != 0) as ::core::ffi::c_int as crate::stdlib::uint32_t,
                    (nnz1 != 0) as ::core::ffi::c_int as crate::stdlib::uint32_t,
                )
                .wrapping_mul(0x101 as crate::stdlib::uint32_t);
                let mut nnzbot: crate::stdlib::uint32_t = pack16to32(
                    (nnz2 != 0) as ::core::ffi::c_int as crate::stdlib::uint32_t,
                    (nnz3 != 0) as ::core::ffi::c_int as crate::stdlib::uint32_t,
                )
                .wrapping_mul(0x101 as crate::stdlib::uint32_t);
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
                        as isize,
                ) as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = nnztop;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                        as isize,
                ) as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = nnztop;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                        as isize,
                ) as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = nnzbot;
                (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
                    (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                        as isize,
                ) as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = nnzbot;
            }
        }
        (*h).loopf
            .deblock_strength
            .expect("non-null function pointer")(
            &raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t,
            &raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40],
            &raw mut (*h).mb.cache.mv as *mut [[crate::stdlib::int16_t; 2]; 40],
            bs as *mut [[crate::stdlib::uint8_t; 4]; 8],
            4 as ::core::ffi::c_int >> (*h).mb.b_interlaced,
            ((*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int)
                as ::core::ffi::c_int,
        );
        if (*h).sh.b_mbaff != 0 {
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
        let mut height: ::core::ffi::c_int = if b_chroma != 0 {
            16 as ::core::ffi::c_int
                >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        };
        let mut i_stride: ::core::ffi::c_int = (*(*h).fdec).i_stride[i as usize];
        let mut i_stride2: ::core::ffi::c_int =
            i_stride << (b_mbaff != 0 && (*h).mb.b_interlaced != 0) as ::core::ffi::c_int;
        let mut i_pix_offset: ::core::ffi::c_int = if b_mbaff != 0 && (*h).mb.b_interlaced != 0 {
            16 as ::core::ffi::c_int * mb_x
                + height * (mb_y & !(1 as ::core::ffi::c_int)) * i_stride
                + (mb_y & 1 as ::core::ffi::c_int) * i_stride
        } else {
            16 as ::core::ffi::c_int * mb_x + height * mb_y * i_stride
        };
        if b_chroma != 0 {
            (*h).mc
                .store_interleave_chroma
                .expect("non-null function pointer")(
                (*(&raw mut (*(*h).fdec).plane as *mut *mut crate::src::common::common::pixel)
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(i_pix_offset as isize)
                    as *mut crate::src::common::common::pixel,
                i_stride2 as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize],
                height,
            );
        } else {
            (*h).mc.copy[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                .expect("non-null function pointer")(
                (*(&raw mut (*(*h).fdec).plane as *mut *mut crate::src::common::common::pixel)
                    .offset(i as isize))
                .offset(i_pix_offset as isize)
                    as *mut crate::src::common::common::pixel,
                i_stride2 as crate::stdlib::intptr_t,
                (*h).mb.pic.p_fdec[i as usize],
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                16 as ::core::ffi::c_int,
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
        let mut backup_dst: ::core::ffi::c_int = if b_mbaff == 0 {
            mb_y & 1 as ::core::ffi::c_int
        } else if mb_y & 1 as ::core::ffi::c_int != 0 {
            1 as ::core::ffi::c_int
        } else if (*h).mb.b_interlaced != 0 {
            0 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        };
        crate::stdlib::memcpy(
            (*(&raw mut *(&raw mut (*h).intra_border_backup
                as *mut [*mut crate::src::common::common::pixel; 3])
                .offset(backup_dst as isize)
                as *mut *mut crate::src::common::common::pixel)
                .offset(0 as ::core::ffi::c_int as isize))
            .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                as *mut crate::src::common::common::pixel as *mut ::core::ffi::c_void,
            (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize].offset(
                (crate::src::common::common::FDEC_STRIDE * 15 as ::core::ffi::c_int) as isize,
            ) as *const ::core::ffi::c_void,
            (16 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
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
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::common::pixel
                    as *mut ::core::ffi::c_void,
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(
                    (crate::src::common::common::FDEC_STRIDE * 15 as ::core::ffi::c_int) as isize,
                ) as *const ::core::ffi::c_void,
                (16 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                (*(&raw mut *(&raw mut (*h).intra_border_backup
                    as *mut [*mut crate::src::common::common::pixel; 3])
                    .offset(backup_dst as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(2 as ::core::ffi::c_int as isize))
                .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::common::pixel
                    as *mut ::core::ffi::c_void,
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(
                    (crate::src::common::common::FDEC_STRIDE * 15 as ::core::ffi::c_int) as isize,
                ) as *const ::core::ffi::c_void,
                (16 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
        } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
            let mut backup_src: ::core::ffi::c_int = (15 as ::core::ffi::c_int
                >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                    == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
                * crate::src::common::common::FDEC_STRIDE;
            crate::stdlib::memcpy(
                (*(&raw mut *(&raw mut (*h).intra_border_backup
                    as *mut [*mut crate::src::common::common::pixel; 3])
                    .offset(backup_dst as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::common::pixel
                    as *mut ::core::ffi::c_void,
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(backup_src as isize)
                    as *const ::core::ffi::c_void,
                (8 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
            crate::stdlib::memcpy(
                (*(&raw mut *(&raw mut (*h).intra_border_backup
                    as *mut [*mut crate::src::common::common::pixel; 3])
                    .offset(backup_dst as isize)
                    as *mut *mut crate::src::common::common::pixel)
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset((mb_x * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                    as *mut crate::src::common::common::pixel
                    as *mut ::core::ffi::c_void,
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(backup_src as isize)
                    as *const ::core::ffi::c_void,
                (8 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
                    as crate::__stddef_size_t_h::size_t,
            );
        }
        if b_mbaff != 0 {
            if mb_y & 1 as ::core::ffi::c_int != 0 {
                let mut backup_src_0: ::core::ffi::c_int = (if (*h).mb.b_interlaced != 0 {
                    7 as ::core::ffi::c_int
                } else {
                    14 as ::core::ffi::c_int
                })
                    * crate::src::common::common::FDEC_STRIDE;
                backup_dst = if (*h).mb.b_interlaced != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
                crate::stdlib::memcpy(
                    (*(&raw mut *(&raw mut (*h).intra_border_backup
                        as *mut [*mut crate::src::common::common::pixel; 3])
                        .offset(backup_dst as isize)
                        as *mut *mut crate::src::common::common::pixel)
                        .offset(0 as ::core::ffi::c_int as isize))
                    .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                        as *mut crate::src::common::common::pixel
                        as *mut ::core::ffi::c_void,
                    (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize]
                        .offset(backup_src_0 as isize)
                        as *const ::core::ffi::c_void,
                    (16 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
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
                            .offset(1 as ::core::ffi::c_int as isize))
                        .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                            as *mut crate::src::common::common::pixel
                            as *mut ::core::ffi::c_void,
                        (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                            .offset(backup_src_0 as isize)
                            as *const ::core::ffi::c_void,
                        (16 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                    );
                    crate::stdlib::memcpy(
                        (*(&raw mut *(&raw mut (*h).intra_border_backup
                            as *mut [*mut crate::src::common::common::pixel; 3])
                            .offset(backup_dst as isize)
                            as *mut *mut crate::src::common::common::pixel)
                            .offset(2 as ::core::ffi::c_int as isize))
                        .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                            as *mut crate::src::common::common::pixel
                            as *mut ::core::ffi::c_void,
                        (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                            .offset(backup_src_0 as isize)
                            as *const ::core::ffi::c_void,
                        (16 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                    );
                } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                    if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                    {
                        backup_src_0 = (if (*h).mb.b_interlaced != 0 {
                            3 as ::core::ffi::c_int
                        } else {
                            6 as ::core::ffi::c_int
                        }) * crate::src::common::common::FDEC_STRIDE;
                    }
                    crate::stdlib::memcpy(
                        (*(&raw mut *(&raw mut (*h).intra_border_backup
                            as *mut [*mut crate::src::common::common::pixel; 3])
                            .offset(backup_dst as isize)
                            as *mut *mut crate::src::common::common::pixel)
                            .offset(1 as ::core::ffi::c_int as isize))
                        .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                            as *mut crate::src::common::common::pixel
                            as *mut ::core::ffi::c_void,
                        (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                            .offset(backup_src_0 as isize)
                            as *const ::core::ffi::c_void,
                        (8 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                    );
                    crate::stdlib::memcpy(
                        (*(&raw mut *(&raw mut (*h).intra_border_backup
                            as *mut [*mut crate::src::common::common::pixel; 3])
                            .offset(backup_dst as isize)
                            as *mut *mut crate::src::common::common::pixel)
                            .offset(1 as ::core::ffi::c_int as isize))
                        .offset(
                            (mb_x * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                        ) as *mut crate::src::common::common::pixel
                            as *mut ::core::ffi::c_void,
                        (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                            .offset(backup_src_0 as isize)
                            as *const ::core::ffi::c_void,
                        (8 as ::core::ffi::c_int * crate::src::common::common::SIZEOF_PIXEL)
                            as crate::__stddef_size_t_h::size_t,
                    );
                }
            }
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_macroblock_cache_save(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let i_mb_xy: ::core::ffi::c_int = (*h).mb.i_mb_xy;
        let i_mb_type: ::core::ffi::c_int =
            x264_mb_type_fix[(*h).mb.i_type as usize] as ::core::ffi::c_int;
        let s8x8: ::core::ffi::c_int = (*h).mb.i_b8_stride;
        let s4x4: ::core::ffi::c_int = (*h).mb.i_b4_stride;
        let i_mb_4x4: ::core::ffi::c_int = (*h).mb.i_b4_xy;
        let i_mb_8x8: ::core::ffi::c_int = (*h).mb.i_b8_xy;
        let mut i4x4: *mut crate::stdlib::int8_t =
            &raw mut *(*h).mb.intra4x4_pred_mode.offset(i_mb_xy as isize)
                as *mut crate::stdlib::int8_t;
        let mut nnz: *mut crate::stdlib::uint8_t =
            &raw mut *(*h).mb.non_zero_count.offset(i_mb_xy as isize)
                as *mut crate::stdlib::uint8_t;
        if (*h).sh.b_mbaff != 0 {
            macroblock_backup_intra(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 1 as ::core::ffi::c_int);
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                macroblock_store_pic(
                    h,
                    (*h).mb.i_mb_x,
                    (*h).mb.i_mb_y,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                macroblock_store_pic(
                    h,
                    (*h).mb.i_mb_x,
                    (*h).mb.i_mb_y,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                macroblock_store_pic(
                    h,
                    (*h).mb.i_mb_x,
                    (*h).mb.i_mb_y,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            }
        } else {
            macroblock_backup_intra(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 0 as ::core::ffi::c_int);
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                macroblock_store_pic(
                    h,
                    (*h).mb.i_mb_x,
                    (*h).mb.i_mb_y,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                macroblock_store_pic(
                    h,
                    (*h).mb.i_mb_x,
                    (*h).mb.i_mb_y,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
            } else if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                macroblock_store_pic(
                    h,
                    (*h).mb.i_mb_x,
                    (*h).mb.i_mb_y,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
            }
        }
        x264_8_prefetch_fenc(h, (*h).fdec, (*h).mb.i_mb_x, (*h).mb.i_mb_y);
        *(*h).mb.type_0.offset(i_mb_xy as isize) = i_mb_type as crate::stdlib::int8_t;
        *(*h).mb.slice_table.offset(i_mb_xy as isize) =
            (*h).sh.i_first_mb as crate::stdlib::int32_t;
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
            (*(i4x4.offset(0 as ::core::ffi::c_int as isize) as *mut crate::stdlib::int8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut (*h).mb.cache.intra4x4_pred_mode as *mut crate::stdlib::int8_t)
                .offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset(10 as ::core::ffi::c_int as isize) as isize,
                ) as *mut crate::stdlib::int8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*(i4x4.offset(4 as ::core::ffi::c_int as isize) as *mut crate::stdlib::int8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = pack8to32(
                (*h).mb.cache.intra4x4_pred_mode
                    [x264_scan8[5 as ::core::ffi::c_int as usize] as usize]
                    as crate::stdlib::uint32_t,
                (*h).mb.cache.intra4x4_pred_mode
                    [x264_scan8[7 as ::core::ffi::c_int as usize] as usize]
                    as crate::stdlib::uint32_t,
                (*h).mb.cache.intra4x4_pred_mode
                    [x264_scan8[13 as ::core::ffi::c_int as usize] as usize]
                    as crate::stdlib::uint32_t,
                0 as crate::stdlib::uint32_t,
            );
        } else if (*h).param.b_constrained_intra == 0
            || (i_mb_type == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
        {
            (*(i4x4 as *mut crate::src::common::base::x264_union64_t)).i =
                (crate::src::common::predict::I_PRED_4x4_DC as ::core::ffi::c_int
                    as ::core::ffi::c_ulonglong)
                    .wrapping_mul(0x101010101010101 as ::core::ffi::c_ulonglong)
                    as crate::stdlib::uint64_t;
        } else {
            (*(i4x4 as *mut crate::src::common::base::x264_union64_t)).i =
                (-(1 as ::core::ffi::c_int) as crate::stdlib::uint8_t as ::core::ffi::c_ulonglong)
                    .wrapping_mul(0x101010101010101 as ::core::ffi::c_ulonglong)
                    as crate::stdlib::uint64_t;
        }
        if i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int {
            *(*h).mb.qp.offset(i_mb_xy as isize) = 0 as crate::stdlib::int8_t;
            (*h).mb.i_last_dqp = 0 as ::core::ffi::c_int;
            (*h).mb.i_cbp_chroma = if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                0 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            };
            (*h).mb.i_cbp_luma = 0xf as ::core::ffi::c_int;
            *(*h).mb.cbp.offset(i_mb_xy as isize) =
                ((*h).mb.i_cbp_chroma << 4 as ::core::ffi::c_int
                    | (*h).mb.i_cbp_luma
                    | 0x1700 as ::core::ffi::c_int) as crate::stdlib::int16_t;
            (*h).mb.b_transform_8x8 = 0 as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 48 as ::core::ffi::c_int {
                (*h).mb.cache.non_zero_count[x264_scan8[i as usize] as usize] =
                    (if (*h).param.b_cabac != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        16 as ::core::ffi::c_int
                    }) as crate::stdlib::uint8_t;
                i += 1;
            }
        } else {
            if (*h).mb.i_type != crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                && (*h).mb.i_cbp_luma == 0 as ::core::ffi::c_int
                && (*h).mb.i_cbp_chroma == 0 as ::core::ffi::c_int
            {
                (*h).mb.i_qp = (*h).mb.i_last_qp;
            }
            *(*h).mb.qp.offset(i_mb_xy as isize) = (*h).mb.i_qp as crate::stdlib::int8_t;
            (*h).mb.i_last_dqp = (*h).mb.i_qp - (*h).mb.i_last_qp;
            (*h).mb.i_last_qp = (*h).mb.i_qp;
        }
        (*(nnz.offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset(0 as ::core::ffi::c_int as isize) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset(2 as ::core::ffi::c_int as isize) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset(8 as ::core::ffi::c_int as isize) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset(10 as ::core::ffi::c_int as isize) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset(
            (16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset(
            (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset(
            (32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(nnz.offset(
            (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t).offset(
            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                .offset((32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut crate::stdlib::uint8_t
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            >= crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
        {
            (*(nnz.offset(
                (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                    as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                .offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                        as isize,
                ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*(nnz.offset(
                (16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                    as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                .offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize)
                        as isize,
                ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*(nnz.offset(
                (32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                    as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                .offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                        as isize,
                ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i;
            (*(nnz.offset(
                (32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                    as isize,
            ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i = (*((&raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t)
                .offset(
                    *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                        .offset((32 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize)
                        as isize,
                ) as *mut crate::stdlib::uint8_t
                as *mut crate::src::common::base::x264_union32_t))
                .i;
        }
        if (*h).mb.i_cbp_luma == 0 as ::core::ffi::c_int
            && (*h).mb.i_type != crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
        {
            (*h).mb.b_transform_8x8 = 0 as ::core::ffi::c_int;
        }
        *(*h).mb.mb_transform_size.offset(i_mb_xy as isize) =
            (*h).mb.b_transform_8x8 as crate::stdlib::int8_t;
        if (*h).sh.i_type != crate::src::common::base::SLICE_TYPE_I as ::core::ffi::c_int {
            let mut mv0: *mut [crate::stdlib::int16_t; 2] =
                (*(&raw mut (*h).mb.mv as *mut *mut [crate::stdlib::int16_t; 2])
                    .offset(0 as ::core::ffi::c_int as isize))
                .offset(i_mb_4x4 as isize) as *mut [crate::stdlib::int16_t; 2];
            let mut ref0: *mut crate::stdlib::int8_t =
                (*(&raw mut (*h).mb.ref_0 as *mut *mut crate::stdlib::int8_t)
                    .offset(0 as ::core::ffi::c_int as isize))
                .offset(i_mb_8x8 as isize) as *mut crate::stdlib::int8_t;
            if !(i_mb_type == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || i_mb_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
            {
                *ref0.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                        [x264_scan8[0 as ::core::ffi::c_int as usize] as usize];
                *ref0.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                        [x264_scan8[4 as ::core::ffi::c_int as usize] as usize];
                *ref0.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                        [x264_scan8[8 as ::core::ffi::c_int as usize] as usize];
                *ref0.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                        [x264_scan8[12 as ::core::ffi::c_int as usize] as usize];
                (*(mv0.offset((0 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [crate::stdlib::int16_t; 2]
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union128_t))
                    .i;
                (*(mv0.offset((1 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [crate::stdlib::int16_t; 2]
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union128_t))
                    .i;
                (*(mv0.offset((2 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [crate::stdlib::int16_t; 2]
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union128_t))
                    .i;
                (*(mv0.offset((3 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [crate::stdlib::int16_t; 2]
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                    as *mut [[crate::stdlib::int16_t; 2]; 40])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::int16_t; 2])
                    .offset(
                        (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut crate::stdlib::int16_t
                    as *mut crate::src::common::base::x264_union128_t))
                    .i;
                if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                    let mut mv1: *mut [crate::stdlib::int16_t; 2] = (*(&raw mut (*h).mb.mv
                        as *mut *mut [crate::stdlib::int16_t; 2])
                        .offset(1 as ::core::ffi::c_int as isize))
                    .offset(i_mb_4x4 as isize)
                        as *mut [crate::stdlib::int16_t; 2];
                    let mut ref1: *mut crate::stdlib::int8_t = (*(&raw mut (*h).mb.ref_0
                        as *mut *mut crate::stdlib::int8_t)
                        .offset(1 as ::core::ffi::c_int as isize))
                    .offset(i_mb_8x8 as isize)
                        as *mut crate::stdlib::int8_t;
                    *ref1.offset(
                        (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * s8x8) as isize,
                    ) = (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                        [x264_scan8[0 as ::core::ffi::c_int as usize] as usize];
                    *ref1.offset(
                        (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * s8x8) as isize,
                    ) = (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                        [x264_scan8[4 as ::core::ffi::c_int as usize] as usize];
                    *ref1.offset(
                        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * s8x8) as isize,
                    ) = (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                        [x264_scan8[8 as ::core::ffi::c_int as usize] as usize];
                    *ref1.offset(
                        (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * s8x8) as isize,
                    ) = (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                        [x264_scan8[12 as ::core::ffi::c_int as usize] as usize];
                    (*(mv1.offset((0 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut [crate::stdlib::int16_t; 2]
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union128_t))
                        .i;
                    (*(mv1.offset((1 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut [crate::stdlib::int16_t; 2]
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union128_t))
                        .i;
                    (*(mv1.offset((2 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut [crate::stdlib::int16_t; 2]
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union128_t))
                        .i;
                    (*(mv1.offset((3 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut [crate::stdlib::int16_t; 2]
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mv
                        as *mut [[crate::stdlib::int16_t; 2]; 40])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::int16_t; 2])
                        .offset(
                            (*(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut crate::stdlib::int16_t
                        as *mut crate::src::common::base::x264_union128_t))
                        .i;
                }
            } else {
                (*(ref0.offset((0 as ::core::ffi::c_int * s8x8) as isize)
                    as *mut crate::stdlib::int8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (-(1 as ::core::ffi::c_int) as crate::stdlib::uint8_t
                    as ::core::ffi::c_int
                    * 0x101 as ::core::ffi::c_int)
                    as crate::stdlib::uint16_t;
                (*(ref0.offset((1 as ::core::ffi::c_int * s8x8) as isize)
                    as *mut crate::stdlib::int8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (-(1 as ::core::ffi::c_int) as crate::stdlib::uint8_t
                    as ::core::ffi::c_int
                    * 0x101 as ::core::ffi::c_int)
                    as crate::stdlib::uint16_t;
                (*(mv0.offset((0 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [crate::stdlib::int16_t; 2]
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                (*(mv0.offset((1 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [crate::stdlib::int16_t; 2]
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                (*(mv0.offset((2 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [crate::stdlib::int16_t; 2]
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                (*(mv0.offset((3 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [crate::stdlib::int16_t; 2]
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                    let mut mv1_0: *mut [crate::stdlib::int16_t; 2] = (*(&raw mut (*h).mb.mv
                        as *mut *mut [crate::stdlib::int16_t; 2])
                        .offset(1 as ::core::ffi::c_int as isize))
                    .offset(i_mb_4x4 as isize)
                        as *mut [crate::stdlib::int16_t; 2];
                    let mut ref1_0: *mut crate::stdlib::int8_t = (*(&raw mut (*h).mb.ref_0
                        as *mut *mut crate::stdlib::int8_t)
                        .offset(1 as ::core::ffi::c_int as isize))
                    .offset(i_mb_8x8 as isize)
                        as *mut crate::stdlib::int8_t;
                    (*(ref1_0.offset((0 as ::core::ffi::c_int * s8x8) as isize)
                        as *mut crate::stdlib::int8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (-(1 as ::core::ffi::c_int) as crate::stdlib::uint8_t
                        as ::core::ffi::c_int
                        * 0x101 as ::core::ffi::c_int)
                        as crate::stdlib::uint16_t;
                    (*(ref1_0.offset((1 as ::core::ffi::c_int * s8x8) as isize)
                        as *mut crate::stdlib::int8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (-(1 as ::core::ffi::c_int) as crate::stdlib::uint8_t
                        as ::core::ffi::c_int
                        * 0x101 as ::core::ffi::c_int)
                        as crate::stdlib::uint16_t;
                    (*(mv1_0.offset((0 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut [crate::stdlib::int16_t; 2]
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = crate::src::common::base::M128_ZERO;
                    (*(mv1_0.offset((1 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut [crate::stdlib::int16_t; 2]
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = crate::src::common::base::M128_ZERO;
                    (*(mv1_0.offset((2 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut [crate::stdlib::int16_t; 2]
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = crate::src::common::base::M128_ZERO;
                    (*(mv1_0.offset((3 as ::core::ffi::c_int * s4x4) as isize)
                        as *mut [crate::stdlib::int16_t; 2]
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = crate::src::common::base::M128_ZERO;
                }
            }
        }
        if (*h).param.b_cabac != 0 {
            let mut mvd0: *mut [crate::stdlib::uint8_t; 2] =
                &raw mut *(*(&raw mut (*h).mb.mvd as *mut *mut [[crate::stdlib::uint8_t; 2]; 8])
                    .offset(0 as ::core::ffi::c_int as isize))
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
                    crate::src::common::predict::I_PRED_CHROMA_DC as ::core::ffi::c_int
                        as crate::stdlib::int8_t;
            }
            if 0x3ff30 as ::core::ffi::c_int >> i_mb_type & 1 as ::core::ffi::c_int != 0 {
                (*(&raw mut *mvd0.offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union64_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                    as *mut [[crate::stdlib::uint8_t; 2]; 40])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                            .offset(10 as ::core::ffi::c_int as isize)
                            as isize,
                    ) as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union64_t))
                    .i;
                (*(&raw mut *mvd0.offset(4 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                    as *mut [[crate::stdlib::uint8_t; 2]; 40])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                            .offset(5 as ::core::ffi::c_int as isize)
                            as isize,
                    ) as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i;
                (*(&raw mut *mvd0.offset(5 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                    as *mut [[crate::stdlib::uint8_t; 2]; 40])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                            .offset(7 as ::core::ffi::c_int as isize)
                            as isize,
                    ) as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i;
                (*(&raw mut *mvd0.offset(6 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                    as *mut [[crate::stdlib::uint8_t; 2]; 40])
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut [crate::stdlib::uint8_t; 2])
                    .offset(
                        *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                            .offset(13 as ::core::ffi::c_int as isize)
                            as isize,
                    ) as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union16_t))
                    .i;
                if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                    let mut mvd1: *mut [crate::stdlib::uint8_t; 2] =
                        &raw mut *(*(&raw mut (*h).mb.mvd
                            as *mut *mut [[crate::stdlib::uint8_t; 2]; 8])
                            .offset(1 as ::core::ffi::c_int as isize))
                        .offset(i_mb_xy as isize)
                            as *mut [crate::stdlib::uint8_t; 2];
                    (*(&raw mut *mvd1.offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union64_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(10 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union64_t))
                        .i;
                    (*(&raw mut *mvd1.offset(4 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(5 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                    (*(&raw mut *mvd1.offset(5 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(7 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                    (*(&raw mut *mvd1.offset(6 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i = (*(&raw mut *(&raw mut *(&raw mut (*h).mb.cache.mvd
                        as *mut [[crate::stdlib::uint8_t; 2]; 40])
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *mut [crate::stdlib::uint8_t; 2])
                        .offset(
                            *(&raw const x264_scan8 as *const crate::stdlib::uint8_t)
                                .offset(13 as ::core::ffi::c_int as isize)
                                as isize,
                        ) as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union16_t))
                        .i;
                }
            } else {
                (*(&raw mut *mvd0.offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint8_t
                    as *mut crate::src::common::base::x264_union128_t))
                    .i = crate::src::common::base::M128_ZERO;
                if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                    let mut mvd1_0: *mut [crate::stdlib::uint8_t; 2] =
                        &raw mut *(*(&raw mut (*h).mb.mvd
                            as *mut *mut [[crate::stdlib::uint8_t; 2]; 8])
                            .offset(1 as ::core::ffi::c_int as isize))
                        .offset(i_mb_xy as isize)
                            as *mut [crate::stdlib::uint8_t; 2];
                    (*(&raw mut *mvd1_0.offset(0 as ::core::ffi::c_int as isize)
                        as *mut crate::stdlib::uint8_t
                        as *mut crate::src::common::base::x264_union128_t))
                        .i = crate::src::common::base::M128_ZERO;
                }
            }
            if (*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int {
                if i_mb_type == crate::src::common::macroblock::B_SKIP as ::core::ffi::c_int
                    || i_mb_type == crate::src::common::macroblock::B_DIRECT as ::core::ffi::c_int
                {
                    *(*h).mb.skipbp.offset(i_mb_xy as isize) = 0xf as crate::stdlib::int8_t;
                } else if i_mb_type == crate::src::common::macroblock::B_8x8 as ::core::ffi::c_int {
                    let mut skipbp: ::core::ffi::c_int = (((*h).mb.i_sub_partition
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        == crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int)
                        as ::core::ffi::c_int)
                        << 0 as ::core::ffi::c_int;
                    skipbp |= (((*h).mb.i_sub_partition[1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        == crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int)
                        as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int;
                    skipbp |= (((*h).mb.i_sub_partition[2 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        == crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int)
                        as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int;
                    skipbp |= (((*h).mb.i_sub_partition[3 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        == crate::src::common::macroblock::D_DIRECT_8x8 as ::core::ffi::c_int)
                        as ::core::ffi::c_int)
                        << 3 as ::core::ffi::c_int;
                    *(*h).mb.skipbp.offset(i_mb_xy as isize) = skipbp as crate::stdlib::int8_t;
                } else {
                    *(*h).mb.skipbp.offset(i_mb_xy as isize) = 0 as crate::stdlib::int8_t;
                }
            }
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_macroblock_bipred_init(
    mut h: *mut crate::src::common::common::x264_t,
) {
    unsafe {
        let mut mbfield: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while mbfield <= (*h).sh.b_mbaff {
            let mut field: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while field <= (*h).sh.b_mbaff {
                let mut i_ref0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_ref0 < (*h).i_ref[0 as ::core::ffi::c_int as usize] << mbfield {
                    let mut l0: *mut crate::src::common::frame::x264_frame_t =
                        (*h).fref[0 as ::core::ffi::c_int as usize][(i_ref0 >> mbfield) as usize];
                    let mut poc0: ::core::ffi::c_int = (*l0).i_poc
                        + mbfield
                            * (*l0).i_delta_poc
                                [(field ^ i_ref0 & 1 as ::core::ffi::c_int) as usize];
                    let mut i_ref1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_ref1 < (*h).i_ref[1 as ::core::ffi::c_int as usize] << mbfield {
                        let mut l1: *mut crate::src::common::frame::x264_frame_t = (*h).fref
                            [1 as ::core::ffi::c_int as usize]
                            [(i_ref1 >> mbfield) as usize];
                        let mut cur_poc: ::core::ffi::c_int =
                            (*(*h).fdec).i_poc + mbfield * (*(*h).fdec).i_delta_poc[field as usize];
                        let mut poc1: ::core::ffi::c_int = (*l1).i_poc
                            + mbfield
                                * (*l1).i_delta_poc
                                    [(field ^ i_ref1 & 1 as ::core::ffi::c_int) as usize];
                        let mut td: ::core::ffi::c_int = x264_clip3(
                            poc1 - poc0,
                            -(128 as ::core::ffi::c_int),
                            127 as ::core::ffi::c_int,
                        );
                        if td == 0 as ::core::ffi::c_int {
                            (*h).mb.dist_scale_factor_buf[mbfield as usize][field as usize]
                                [i_ref0 as usize][i_ref1 as usize] = 256 as crate::stdlib::int16_t;
                            (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                                [i_ref0 as usize][i_ref1 as usize] = 32 as crate::stdlib::int8_t;
                        } else {
                            let mut tb: ::core::ffi::c_int = x264_clip3(
                                cur_poc - poc0,
                                -(128 as ::core::ffi::c_int),
                                127 as ::core::ffi::c_int,
                            );
                            let mut tx: ::core::ffi::c_int = (16384 as ::core::ffi::c_int
                                + (crate::stdlib::abs(td) >> 1 as ::core::ffi::c_int))
                                / td;
                            let mut dist_scale_factor: ::core::ffi::c_int = x264_clip3(
                                tb * tx + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int,
                                -(1024 as ::core::ffi::c_int),
                                1023 as ::core::ffi::c_int,
                            );
                            (*h).mb.dist_scale_factor_buf[mbfield as usize][field as usize]
                                [i_ref0 as usize][i_ref1 as usize] =
                                dist_scale_factor as crate::stdlib::int16_t;
                            dist_scale_factor >>= 2 as ::core::ffi::c_int;
                            if (*h).param.analyse.b_weighted_bipred != 0
                                && dist_scale_factor >= -(64 as ::core::ffi::c_int)
                                && dist_scale_factor <= 128 as ::core::ffi::c_int
                            {
                                (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                                    [i_ref0 as usize][i_ref1 as usize] = (64 as ::core::ffi::c_int
                                    - dist_scale_factor)
                                    as crate::stdlib::int8_t;
                                '_c2rust_label: {
                                    if dist_scale_factor >= -(63 as ::core::ffi::c_int)
                                        && dist_scale_factor <= 127 as ::core::ffi::c_int
                                    {
                                    } else {
                                        crate::stdlib::__assert_fail(
                                            b"dist_scale_factor >= -63 && dist_scale_factor <= 127\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            b"common/macroblock.c\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            1918 as ::core::ffi::c_uint,
                                            b"void x264_8_macroblock_bipred_init(x264_t *)\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                        );
                                    }
                                };
                            } else {
                                (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                                    [i_ref0 as usize][i_ref1 as usize] =
                                    32 as crate::stdlib::int8_t;
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
