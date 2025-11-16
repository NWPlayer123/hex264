#[c2rust::header_src = "/usr/include/bits/types.h:32"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/common.h:32"]
pub mod common_h {
    #[c2rust::src_loc = "94:5"]
    pub type pixel = uint16_t;
    #[c2rust::src_loc = "95:5"]
    pub type pixel4 = uint64_t;
    #[c2rust::src_loc = "61:9"]
    pub const PIXEL_MAX: ::core::ffi::c_int = ((1 as ::core::ffi::c_int) << BIT_DEPTH)
        - 1 as ::core::ffi::c_int;
    #[inline(always)]
    #[c2rust::src_loc = "145:1"]
    pub unsafe extern "C" fn x264_clip_pixel(mut x: ::core::ffi::c_int) -> pixel {
        return (if x & !PIXEL_MAX != 0 {
            -x >> 31 as ::core::ffi::c_int & PIXEL_MAX
        } else {
            x
        }) as pixel;
    }
    #[c2rust::src_loc = "571:9"]
    pub const FDEC_STRIDE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    use super::stdint_uintn_h::{uint16_t, uint64_t};
    use super::internal::BIT_DEPTH;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/predict.h:32"]
pub mod predict_h {
    #[c2rust::src_loc = "32:1"]
    pub type x264_predict_8x8_filter_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "30:1"]
    pub type x264_predict_t = Option<unsafe extern "C" fn(*mut pixel) -> ()>;
    #[c2rust::src_loc = "31:1"]
    pub type x264_predict8x8_t = Option<
        unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    >;
    #[c2rust::src_loc = "34:1"]
    pub type intra_chroma_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "43:5"]
    pub const I_PRED_CHROMA_DC_128: intra_chroma_pred_e = 6;
    #[c2rust::src_loc = "42:5"]
    pub const I_PRED_CHROMA_DC_TOP: intra_chroma_pred_e = 5;
    #[c2rust::src_loc = "41:5"]
    pub const I_PRED_CHROMA_DC_LEFT: intra_chroma_pred_e = 4;
    #[c2rust::src_loc = "39:5"]
    pub const I_PRED_CHROMA_P: intra_chroma_pred_e = 3;
    #[c2rust::src_loc = "38:5"]
    pub const I_PRED_CHROMA_V: intra_chroma_pred_e = 2;
    #[c2rust::src_loc = "37:5"]
    pub const I_PRED_CHROMA_H: intra_chroma_pred_e = 1;
    #[c2rust::src_loc = "36:5"]
    pub const I_PRED_CHROMA_DC: intra_chroma_pred_e = 0;
    #[c2rust::src_loc = "51:1"]
    pub type intra16x16_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "60:5"]
    pub const I_PRED_16x16_DC_128: intra16x16_pred_e = 6;
    #[c2rust::src_loc = "59:5"]
    pub const I_PRED_16x16_DC_TOP: intra16x16_pred_e = 5;
    #[c2rust::src_loc = "58:5"]
    pub const I_PRED_16x16_DC_LEFT: intra16x16_pred_e = 4;
    #[c2rust::src_loc = "56:5"]
    pub const I_PRED_16x16_P: intra16x16_pred_e = 3;
    #[c2rust::src_loc = "55:5"]
    pub const I_PRED_16x16_DC: intra16x16_pred_e = 2;
    #[c2rust::src_loc = "54:5"]
    pub const I_PRED_16x16_H: intra16x16_pred_e = 1;
    #[c2rust::src_loc = "53:5"]
    pub const I_PRED_16x16_V: intra16x16_pred_e = 0;
    #[c2rust::src_loc = "68:1"]
    pub type intra4x4_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "82:5"]
    pub const I_PRED_4x4_DC_128: intra4x4_pred_e = 11;
    #[c2rust::src_loc = "81:5"]
    pub const I_PRED_4x4_DC_TOP: intra4x4_pred_e = 10;
    #[c2rust::src_loc = "80:5"]
    pub const I_PRED_4x4_DC_LEFT: intra4x4_pred_e = 9;
    #[c2rust::src_loc = "78:5"]
    pub const I_PRED_4x4_HU: intra4x4_pred_e = 8;
    #[c2rust::src_loc = "77:5"]
    pub const I_PRED_4x4_VL: intra4x4_pred_e = 7;
    #[c2rust::src_loc = "76:5"]
    pub const I_PRED_4x4_HD: intra4x4_pred_e = 6;
    #[c2rust::src_loc = "75:5"]
    pub const I_PRED_4x4_VR: intra4x4_pred_e = 5;
    #[c2rust::src_loc = "74:5"]
    pub const I_PRED_4x4_DDR: intra4x4_pred_e = 4;
    #[c2rust::src_loc = "73:5"]
    pub const I_PRED_4x4_DDL: intra4x4_pred_e = 3;
    #[c2rust::src_loc = "72:5"]
    pub const I_PRED_4x4_DC: intra4x4_pred_e = 2;
    #[c2rust::src_loc = "71:5"]
    pub const I_PRED_4x4_H: intra4x4_pred_e = 1;
    #[c2rust::src_loc = "70:5"]
    pub const I_PRED_4x4_V: intra4x4_pred_e = 0;
    #[c2rust::src_loc = "95:1"]
    pub type intra8x8_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "109:5"]
    pub const I_PRED_8x8_DC_128: intra8x8_pred_e = 11;
    #[c2rust::src_loc = "108:5"]
    pub const I_PRED_8x8_DC_TOP: intra8x8_pred_e = 10;
    #[c2rust::src_loc = "107:5"]
    pub const I_PRED_8x8_DC_LEFT: intra8x8_pred_e = 9;
    #[c2rust::src_loc = "105:5"]
    pub const I_PRED_8x8_HU: intra8x8_pred_e = 8;
    #[c2rust::src_loc = "104:5"]
    pub const I_PRED_8x8_VL: intra8x8_pred_e = 7;
    #[c2rust::src_loc = "103:5"]
    pub const I_PRED_8x8_HD: intra8x8_pred_e = 6;
    #[c2rust::src_loc = "102:5"]
    pub const I_PRED_8x8_VR: intra8x8_pred_e = 5;
    #[c2rust::src_loc = "101:5"]
    pub const I_PRED_8x8_DDR: intra8x8_pred_e = 4;
    #[c2rust::src_loc = "100:5"]
    pub const I_PRED_8x8_DDL: intra8x8_pred_e = 3;
    #[c2rust::src_loc = "99:5"]
    pub const I_PRED_8x8_DC: intra8x8_pred_e = 2;
    #[c2rust::src_loc = "98:5"]
    pub const I_PRED_8x8_H: intra8x8_pred_e = 1;
    #[c2rust::src_loc = "97:5"]
    pub const I_PRED_8x8_V: intra8x8_pred_e = 0;
    use super::common_h::pixel;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:32"]
pub mod base_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "66:9"]
    pub union x264_union64_t {
        pub i: uint64_t,
        pub d: [uint32_t; 2],
        pub w: [uint16_t; 4],
        pub b: [uint8_t; 8],
    }
    use super::stdint_uintn_h::{uint64_t, uint32_t, uint16_t, uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/macroblock.h:32"]
pub mod macroblock_h {
    #[c2rust::src_loc = "35:5"]
    pub const MB_TOPRIGHT: macroblock_position_e = 4;
    #[c2rust::src_loc = "36:5"]
    pub const MB_TOPLEFT: macroblock_position_e = 8;
    #[c2rust::src_loc = "34:5"]
    pub const MB_TOP: macroblock_position_e = 2;
    #[c2rust::src_loc = "33:5"]
    pub const MB_LEFT: macroblock_position_e = 1;
    #[c2rust::src_loc = "31:1"]
    pub type macroblock_position_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "40:5"]
    pub const ALL_NEIGHBORS: macroblock_position_e = 15;
    #[c2rust::src_loc = "38:5"]
    pub const MB_PRIVATE: macroblock_position_e = 16;
    #[inline(always)]
    #[c2rust::src_loc = "371:1"]
    pub unsafe extern "C" fn pack16to32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
        return a.wrapping_add(b << 16 as ::core::ffi::c_int);
    }
    #[inline(always)]
    #[c2rust::src_loc = "403:1"]
    pub unsafe extern "C" fn pack32to64(mut a: uint32_t, mut b: uint32_t) -> uint64_t {
        return (a as uint64_t).wrapping_add((b as uint64_t) << 32 as ::core::ffi::c_int);
    }
    use super::stdint_uintn_h::{uint32_t, uint64_t};
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "3:9"]
    pub const BIT_DEPTH: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
}
pub use self::types_h::{__uint8_t, __uint16_t, __uint32_t, __uint64_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::common_h::{pixel, pixel4, PIXEL_MAX, x264_clip_pixel, FDEC_STRIDE};
pub use self::predict_h::{
    x264_predict_8x8_filter_t, x264_predict_t, x264_predict8x8_t, intra_chroma_pred_e,
    I_PRED_CHROMA_DC_128, I_PRED_CHROMA_DC_TOP, I_PRED_CHROMA_DC_LEFT, I_PRED_CHROMA_P,
    I_PRED_CHROMA_V, I_PRED_CHROMA_H, I_PRED_CHROMA_DC, intra16x16_pred_e,
    I_PRED_16x16_DC_128, I_PRED_16x16_DC_TOP, I_PRED_16x16_DC_LEFT, I_PRED_16x16_P,
    I_PRED_16x16_DC, I_PRED_16x16_H, I_PRED_16x16_V, intra4x4_pred_e, I_PRED_4x4_DC_128,
    I_PRED_4x4_DC_TOP, I_PRED_4x4_DC_LEFT, I_PRED_4x4_HU, I_PRED_4x4_VL, I_PRED_4x4_HD,
    I_PRED_4x4_VR, I_PRED_4x4_DDR, I_PRED_4x4_DDL, I_PRED_4x4_DC, I_PRED_4x4_H,
    I_PRED_4x4_V, intra8x8_pred_e, I_PRED_8x8_DC_128, I_PRED_8x8_DC_TOP,
    I_PRED_8x8_DC_LEFT, I_PRED_8x8_HU, I_PRED_8x8_VL, I_PRED_8x8_HD, I_PRED_8x8_VR,
    I_PRED_8x8_DDR, I_PRED_8x8_DDL, I_PRED_8x8_DC, I_PRED_8x8_H, I_PRED_8x8_V,
};
pub use self::base_h::x264_union64_t;
pub use self::macroblock_h::{
    MB_TOPRIGHT, MB_TOPLEFT, MB_TOP, MB_LEFT, macroblock_position_e, ALL_NEIGHBORS,
    MB_PRIVATE, pack16to32, pack32to64,
};
pub use self::internal::BIT_DEPTH;
#[no_mangle]
#[c2rust::src_loc = "67:1"]
pub unsafe extern "C" fn x264_10_predict_16x16_dc_c(mut src: *mut pixel) {
    let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        dc
            += *src.offset((-(1 as ::core::ffi::c_int) + i * FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
        dc += *src.offset((i - FDEC_STRIDE) as isize) as ::core::ffi::c_int;
        i += 1;
    }
    let mut dcsplat: pixel4 = ((dc + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        i_0 += 1;
    }
}
#[c2rust::src_loc = "80:1"]
unsafe extern "C" fn predict_16x16_dc_left_c(mut src: *mut pixel) {
    let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        dc
            += *src.offset((-(1 as ::core::ffi::c_int) + i * FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
        i += 1;
    }
    let mut dcsplat: pixel4 = ((dc + 8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        i_0 += 1;
    }
}
#[c2rust::src_loc = "90:1"]
unsafe extern "C" fn predict_16x16_dc_top_c(mut src: *mut pixel) {
    let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        dc += *src.offset((i - FDEC_STRIDE) as isize) as ::core::ffi::c_int;
        i += 1;
    }
    let mut dcsplat: pixel4 = ((dc + 8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        i_0 += 1;
    }
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn predict_16x16_dc_128_c(mut src: *mut pixel) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = (((1
            as ::core::ffi::c_int) << 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = (((1
            as ::core::ffi::c_int) << 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = (((1
            as ::core::ffi::c_int) << 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = (((1
            as ::core::ffi::c_int) << 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn x264_10_predict_16x16_h_c(mut src: *mut pixel) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        let v: pixel4 = (*src.offset(-(1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v
            as uint64_t;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v
            as uint64_t;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "116:1"]
pub unsafe extern "C" fn x264_10_predict_16x16_v_c(mut src: *mut pixel) {
    let mut v0: pixel4 = (*(&mut *src
        .offset((0 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as isize)
        as *mut pixel as *mut x264_union64_t))
        .i as pixel4;
    let mut v1: pixel4 = (*(&mut *src
        .offset((4 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as isize)
        as *mut pixel as *mut x264_union64_t))
        .i as pixel4;
    let mut v2: pixel4 = (*(&mut *src
        .offset((8 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as isize)
        as *mut pixel as *mut x264_union64_t))
        .i as pixel4;
    let mut v3: pixel4 = (*(&mut *src
        .offset((12 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as isize)
        as *mut pixel as *mut x264_union64_t))
        .i as pixel4;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v0
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v1
            as uint64_t;
        (*(src.offset(8 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v2
            as uint64_t;
        (*(src.offset(12 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v3
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "132:1"]
pub unsafe extern "C" fn x264_10_predict_16x16_p_c(mut src: *mut pixel) {
    let mut H: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut V: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i <= 7 as ::core::ffi::c_int {
        H
            += (i + 1 as ::core::ffi::c_int)
                * (*src.offset((8 as ::core::ffi::c_int + i - FDEC_STRIDE) as isize)
                    as ::core::ffi::c_int
                    - *src.offset((6 as ::core::ffi::c_int - i - FDEC_STRIDE) as isize)
                        as ::core::ffi::c_int);
        V
            += (i + 1 as ::core::ffi::c_int)
                * (*src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + (8 as ::core::ffi::c_int + i) * FDEC_STRIDE) as isize,
                    ) as ::core::ffi::c_int
                    - *src
                        .offset(
                            (-(1 as ::core::ffi::c_int)
                                + (6 as ::core::ffi::c_int - i) * FDEC_STRIDE) as isize,
                        ) as ::core::ffi::c_int);
        i += 1;
    }
    let mut a: ::core::ffi::c_int = 16 as ::core::ffi::c_int
        * (*src
            .offset(
                (-(1 as ::core::ffi::c_int) + 15 as ::core::ffi::c_int * FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset((15 as ::core::ffi::c_int - FDEC_STRIDE) as isize)
                as ::core::ffi::c_int);
    let mut b: ::core::ffi::c_int = 5 as ::core::ffi::c_int * H
        + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 5 as ::core::ffi::c_int * V
        + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int;
    let mut i00: ::core::ffi::c_int = a - b * 7 as ::core::ffi::c_int
        - c * 7 as ::core::ffi::c_int + 16 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut pix: ::core::ffi::c_int = i00;
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 16 as ::core::ffi::c_int {
            *src.offset(x as isize) = x264_clip_pixel(pix >> 5 as ::core::ffi::c_int);
            pix += b;
            x += 1;
        }
        src = src.offset(FDEC_STRIDE as isize);
        i00 += c;
        y += 1;
    }
}
#[c2rust::src_loc = "167:1"]
unsafe extern "C" fn predict_8x8c_dc_128_c(mut src: *mut pixel) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = (((1
            as ::core::ffi::c_int) << 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = (((1
            as ::core::ffi::c_int) << 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y += 1;
    }
}
#[c2rust::src_loc = "176:1"]
unsafe extern "C" fn predict_8x8c_dc_left_c(mut src: *mut pixel) {
    let mut dc0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dc1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        dc0
            += *src.offset((y * FDEC_STRIDE - 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int;
        dc1
            += *src
                .offset(
                    ((y + 4 as ::core::ffi::c_int) * FDEC_STRIDE
                        - 1 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int;
        y += 1;
    }
    let mut dc0splat: pixel4 = ((dc0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc1splat: pixel4 = ((dc1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y_0 < 4 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc0splat
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc0splat
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y_0 += 1;
    }
    let mut y_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y_1 < 4 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc1splat
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc1splat
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y_1 += 1;
    }
}
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn predict_8x8c_dc_top_c(mut src: *mut pixel) {
    let mut dc0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dc1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while x < 4 as ::core::ffi::c_int {
        dc0 += *src.offset((x - FDEC_STRIDE) as isize) as ::core::ffi::c_int;
        dc1
            += *src.offset((x + 4 as ::core::ffi::c_int - FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
        x += 1;
    }
    let mut dc0splat: pixel4 = ((dc0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc1splat: pixel4 = ((dc1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc0splat
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc1splat
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "221:1"]
pub unsafe extern "C" fn x264_10_predict_8x8c_dc_c(mut src: *mut pixel) {
    let mut s0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        s0 += *src.offset((i - FDEC_STRIDE) as isize) as ::core::ffi::c_int;
        s1
            += *src.offset((i + 4 as ::core::ffi::c_int - FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
        s2
            += *src.offset((-(1 as ::core::ffi::c_int) + i * FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
        s3
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + (i + 4 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                ) as ::core::ffi::c_int;
        i += 1;
    }
    let mut dc0: pixel4 = ((s0 + s2 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc1: pixel4 = ((s1 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc2: pixel4 = ((s3 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc3: pixel4 = ((s1 + s3 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc0
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc1
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y += 1;
    }
    let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y_0 < 4 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc2
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc3
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y_0 += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "260:1"]
pub unsafe extern "C" fn x264_10_predict_8x8c_h_c(mut src: *mut pixel) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let mut v: pixel4 = (*src.offset(-(1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "270:1"]
pub unsafe extern "C" fn x264_10_predict_8x8c_v_c(mut src: *mut pixel) {
    let mut v0: pixel4 = (*(src
        .offset(0 as ::core::ffi::c_int as isize)
        .offset(-(32 as ::core::ffi::c_int as isize)) as *mut x264_union64_t))
        .i as pixel4;
    let mut v1: pixel4 = (*(src
        .offset(4 as ::core::ffi::c_int as isize)
        .offset(-(32 as ::core::ffi::c_int as isize)) as *mut x264_union64_t))
        .i as pixel4;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v0
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v1
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "282:1"]
pub unsafe extern "C" fn x264_10_predict_8x8c_p_c(mut src: *mut pixel) {
    let mut H: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut V: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        H
            += (i + 1 as ::core::ffi::c_int)
                * (*src.offset((4 as ::core::ffi::c_int + i - FDEC_STRIDE) as isize)
                    as ::core::ffi::c_int
                    - *src.offset((2 as ::core::ffi::c_int - i - FDEC_STRIDE) as isize)
                        as ::core::ffi::c_int);
        V
            += (i + 1 as ::core::ffi::c_int)
                * (*src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + (i + 4 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                    ) as ::core::ffi::c_int
                    - *src
                        .offset(
                            (-(1 as ::core::ffi::c_int)
                                + (2 as ::core::ffi::c_int - i) * FDEC_STRIDE) as isize,
                        ) as ::core::ffi::c_int);
        i += 1;
    }
    let mut a: ::core::ffi::c_int = 16 as ::core::ffi::c_int
        * (*src
            .offset(
                (-(1 as ::core::ffi::c_int) + 7 as ::core::ffi::c_int * FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset((7 as ::core::ffi::c_int - FDEC_STRIDE) as isize)
                as ::core::ffi::c_int);
    let mut b: ::core::ffi::c_int = 17 as ::core::ffi::c_int * H
        + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 17 as ::core::ffi::c_int * V
        + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int;
    let mut i00: ::core::ffi::c_int = a - 3 as ::core::ffi::c_int * b
        - 3 as ::core::ffi::c_int * c + 16 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        let mut pix: ::core::ffi::c_int = i00;
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            *src.offset(x as isize) = x264_clip_pixel(pix >> 5 as ::core::ffi::c_int);
            pix += b;
            x += 1;
        }
        src = src.offset(FDEC_STRIDE as isize);
        i00 += c;
        y += 1;
    }
}
#[c2rust::src_loc = "314:1"]
unsafe extern "C" fn predict_8x16c_dc_128_c(mut src: *mut pixel) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = (((1
            as ::core::ffi::c_int) << 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = (((1
            as ::core::ffi::c_int) << 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y += 1;
    }
}
#[c2rust::src_loc = "323:1"]
unsafe extern "C" fn predict_8x16c_dc_left_c(mut src: *mut pixel) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 4 as ::core::ffi::c_int {
            dc
                += *src.offset((y * FDEC_STRIDE - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            y += 1;
        }
        let mut dcsplat: pixel4 = ((dc + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < 4 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
                as uint64_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dcsplat
                as uint64_t;
            src = src.offset(FDEC_STRIDE as isize);
            y_0 += 1;
        }
        i += 1;
    }
}
#[c2rust::src_loc = "342:1"]
unsafe extern "C" fn predict_8x16c_dc_top_c(mut src: *mut pixel) {
    let mut dc0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dc1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while x < 4 as ::core::ffi::c_int {
        dc0 += *src.offset((x - FDEC_STRIDE) as isize) as ::core::ffi::c_int;
        dc1
            += *src.offset((x + 4 as ::core::ffi::c_int - FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
        x += 1;
    }
    let mut dc0splat: pixel4 = ((dc0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc1splat: pixel4 = ((dc1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc0splat
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc1splat
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "361:1"]
pub unsafe extern "C" fn x264_10_predict_8x16c_dc_c(mut src: *mut pixel) {
    let mut s0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        s0
            += *src.offset((i + 0 as ::core::ffi::c_int - FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
        s1
            += *src.offset((i + 4 as ::core::ffi::c_int - FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
        s2
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + (i + 0 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                ) as ::core::ffi::c_int;
        s3
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + (i + 4 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                ) as ::core::ffi::c_int;
        s4
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + (i + 8 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                ) as ::core::ffi::c_int;
        s5
            += *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + (i + 12 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                ) as ::core::ffi::c_int;
        i += 1;
    }
    let mut dc0: pixel4 = ((s0 + s2 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc1: pixel4 = ((s1 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc2: pixel4 = ((s3 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc3: pixel4 = ((s1 + s3 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc4: pixel4 = ((s4 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc5: pixel4 = ((s1 + s4 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc6: pixel4 = ((s5 + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut dc7: pixel4 = ((s1 + s5 + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc0
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc1
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y += 1;
    }
    let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y_0 < 4 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc2
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc3
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y_0 += 1;
    }
    let mut y_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y_1 < 4 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc4
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc5
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y_1 += 1;
    }
    let mut y_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y_2 < 4 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc6
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc7
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y_2 += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "421:1"]
pub unsafe extern "C" fn x264_10_predict_8x16c_h_c(mut src: *mut pixel) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        let mut v: pixel4 = (*src.offset(-(1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "431:1"]
pub unsafe extern "C" fn x264_10_predict_8x16c_v_c(mut src: *mut pixel) {
    let mut v0: pixel4 = (*(src
        .offset(0 as ::core::ffi::c_int as isize)
        .offset(-(32 as ::core::ffi::c_int as isize)) as *mut x264_union64_t))
        .i as pixel4;
    let mut v1: pixel4 = (*(src
        .offset(4 as ::core::ffi::c_int as isize)
        .offset(-(32 as ::core::ffi::c_int as isize)) as *mut x264_union64_t))
        .i as pixel4;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v0
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = v1
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "443:1"]
pub unsafe extern "C" fn x264_10_predict_8x16c_p_c(mut src: *mut pixel) {
    let mut H: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut V: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        H
            += (i + 1 as ::core::ffi::c_int)
                * (*src.offset((4 as ::core::ffi::c_int + i - FDEC_STRIDE) as isize)
                    as ::core::ffi::c_int
                    - *src.offset((2 as ::core::ffi::c_int - i - FDEC_STRIDE) as isize)
                        as ::core::ffi::c_int);
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 8 as ::core::ffi::c_int {
        V
            += (i_0 + 1 as ::core::ffi::c_int)
                * (*src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + (i_0 + 8 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                    ) as ::core::ffi::c_int
                    - *src
                        .offset(
                            (-(1 as ::core::ffi::c_int)
                                + (6 as ::core::ffi::c_int - i_0) * FDEC_STRIDE) as isize,
                        ) as ::core::ffi::c_int);
        i_0 += 1;
    }
    let mut a: ::core::ffi::c_int = 16 as ::core::ffi::c_int
        * (*src
            .offset(
                (-(1 as ::core::ffi::c_int) + 15 as ::core::ffi::c_int * FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset((7 as ::core::ffi::c_int - FDEC_STRIDE) as isize)
                as ::core::ffi::c_int);
    let mut b: ::core::ffi::c_int = 17 as ::core::ffi::c_int * H
        + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 5 as ::core::ffi::c_int * V
        + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int;
    let mut i00: ::core::ffi::c_int = a - 3 as ::core::ffi::c_int * b
        - 7 as ::core::ffi::c_int * c + 16 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut pix: ::core::ffi::c_int = i00;
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            *src.offset(x as isize) = x264_clip_pixel(pix >> 5 as ::core::ffi::c_int);
            pix += b;
            x += 1;
        }
        src = src.offset(FDEC_STRIDE as isize);
        i00 += c;
        y += 1;
    }
}
#[c2rust::src_loc = "481:1"]
unsafe extern "C" fn predict_4x4_dc_128_c(mut src: *mut pixel) {
    let ref mut fresh47 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh47 = (((1 as ::core::ffi::c_int)
        << 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    let ref mut fresh48 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh48 = *fresh47;
    let ref mut fresh49 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh49 = *fresh48;
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh49;
}
#[c2rust::src_loc = "485:1"]
unsafe extern "C" fn predict_4x4_dc_left_c(mut src: *mut pixel) {
    let mut dc: pixel4 = ((*src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let ref mut fresh53 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh53 = dc as uint64_t;
    let ref mut fresh54 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh54 = *fresh53;
    let ref mut fresh55 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh55 = *fresh54;
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh55;
}
#[c2rust::src_loc = "490:1"]
unsafe extern "C" fn predict_4x4_dc_top_c(mut src: *mut pixel) {
    let mut dc: pixel4 = ((*src
        .offset(
            (0 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (2 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (3 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let ref mut fresh50 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh50 = dc as uint64_t;
    let ref mut fresh51 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh51 = *fresh50;
    let ref mut fresh52 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh52 = *fresh51;
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh52;
}
#[no_mangle]
#[c2rust::src_loc = "495:1"]
pub unsafe extern "C" fn x264_10_predict_4x4_dc_c(mut src: *mut pixel) {
    let mut dc: pixel4 = ((*src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (0 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (2 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
        + *src
            .offset(
                (3 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let ref mut fresh8 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh8 = dc as uint64_t;
    let ref mut fresh9 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh9 = *fresh8;
    let ref mut fresh10 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh10 = *fresh9;
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh10;
}
#[no_mangle]
#[c2rust::src_loc = "501:1"]
pub unsafe extern "C" fn x264_10_predict_4x4_h_c(mut src: *mut pixel) {
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = (*src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = (*src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = (*src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = (*src
        .offset(
            (-(1 as ::core::ffi::c_int)
                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
}
#[no_mangle]
#[c2rust::src_loc = "508:1"]
pub unsafe extern "C" fn x264_10_predict_4x4_v_c(mut src: *mut pixel) {
    let ref mut fresh11 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh11 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    let ref mut fresh12 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh12 = *fresh11;
    let ref mut fresh13 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh13 = *fresh12;
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh13;
}
#[c2rust::src_loc = "534:1"]
unsafe extern "C" fn predict_4x4_ddl_c(mut src: *mut pixel) {
    let mut t0: ::core::ffi::c_int = *src
        .offset(
            (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t4: ::core::ffi::c_int = *src
        .offset(
            (4 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t5: ::core::ffi::c_int = *src
        .offset(
            (5 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t6: ::core::ffi::c_int = *src
        .offset(
            (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t7: ::core::ffi::c_int = *src
        .offset(
            (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh92 = *src
        .offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh92 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh92;
    let ref mut fresh93 = *src
        .offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh93 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh94 = *src
        .offset(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh94 = *fresh93;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh94;
    let ref mut fresh95 = *src
        .offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh95 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh96 = *src
        .offset(
            (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh96 = *fresh95;
    let ref mut fresh97 = *src
        .offset(
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh97 = *fresh96;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh97;
    let ref mut fresh98 = *src
        .offset(
            (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh98 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh99 = *src
        .offset(
            (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh99 = *fresh98;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh99;
    let ref mut fresh100 = *src
        .offset(
            (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh100 = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh100;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t6 + 2 as ::core::ffi::c_int * t7 + t7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
}
#[c2rust::src_loc = "546:1"]
unsafe extern "C" fn predict_4x4_ddr_c(mut src: *mut pixel) {
    let mut lt: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
    let mut l0: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t0: ::core::ffi::c_int = *src
        .offset(
            (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t3 + 2 as ::core::ffi::c_int * t2 + t1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh83 = *src
        .offset(
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh83 = (t2 + 2 as ::core::ffi::c_int * t1 + t0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh83;
    let ref mut fresh84 = *src
        .offset(
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh84 = (t1 + 2 as ::core::ffi::c_int * t0 + lt + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh85 = *src
        .offset(
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh85 = *fresh84;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh85;
    let ref mut fresh86 = *src
        .offset(
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh86 = (t0 + 2 as ::core::ffi::c_int * lt + l0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh87 = *src
        .offset(
            (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh87 = *fresh86;
    let ref mut fresh88 = *src
        .offset(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh88 = *fresh87;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh88;
    let ref mut fresh89 = *src
        .offset(
            (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh89 = (lt + 2 as ::core::ffi::c_int * l0 + l1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh90 = *src
        .offset(
            (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh90 = *fresh89;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh90;
    let ref mut fresh91 = *src
        .offset(
            (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh91 = (l0 + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh91;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
}
#[c2rust::src_loc = "560:1"]
unsafe extern "C" fn predict_4x4_vr_c(mut src: *mut pixel) {
    let mut lt: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
    let mut l0: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t0: ::core::ffi::c_int = *src
        .offset(
            (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (l2 + 2 as ::core::ffi::c_int * l1 + l0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (l1 + 2 as ::core::ffi::c_int * l0 + lt + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh77 = *src
        .offset(
            (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh77 = (l0 + 2 as ::core::ffi::c_int * lt + t0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh77;
    let ref mut fresh78 = *src
        .offset(
            (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh78 = (lt + t0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh78;
    let ref mut fresh79 = *src
        .offset(
            (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh79 = (lt + 2 as ::core::ffi::c_int * t0 + t1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh79;
    let ref mut fresh80 = *src
        .offset(
            (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh80 = (t0 + t1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh80;
    let ref mut fresh81 = *src
        .offset(
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh81 = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh81;
    let ref mut fresh82 = *src
        .offset(
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh82 = (t1 + t2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh82;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t2 + t3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
}
#[c2rust::src_loc = "577:1"]
unsafe extern "C" fn predict_4x4_hd_c(mut src: *mut pixel) {
    let mut lt: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + -(1 as ::core::ffi::c_int) * FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
    let mut l0: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t0: ::core::ffi::c_int = *src
        .offset(
            (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (l2 + l3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh71 = *src
        .offset(
            (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh71 = (l1 + l2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh71;
    let ref mut fresh72 = *src
        .offset(
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh72 = (l0 + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh72;
    let ref mut fresh73 = *src
        .offset(
            (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh73 = (l0 + l1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh73;
    let ref mut fresh74 = *src
        .offset(
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh74 = (lt + 2 as ::core::ffi::c_int * l0 + l1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh74;
    let ref mut fresh75 = *src
        .offset(
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh75 = (lt + l0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh75;
    let ref mut fresh76 = *src
        .offset(
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh76 = (t0 + 2 as ::core::ffi::c_int * lt + l0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh76;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t1 + 2 as ::core::ffi::c_int * t0 + lt + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t2 + 2 as ::core::ffi::c_int * t1 + t0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
}
#[c2rust::src_loc = "594:1"]
unsafe extern "C" fn predict_4x4_vl_c(mut src: *mut pixel) {
    let mut t0: ::core::ffi::c_int = *src
        .offset(
            (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *src
        .offset(
            (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *src
        .offset(
            (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *src
        .offset(
            (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t4: ::core::ffi::c_int = *src
        .offset(
            (4 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t5: ::core::ffi::c_int = *src
        .offset(
            (5 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t6: ::core::ffi::c_int = *src
        .offset(
            (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut t7: ::core::ffi::c_int = *src
        .offset(
            (7 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t0 + t1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh65 = *src
        .offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh65 = (t1 + t2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh65;
    let ref mut fresh66 = *src
        .offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh66 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh66;
    let ref mut fresh67 = *src
        .offset(
            (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh67 = (t2 + t3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh67;
    let ref mut fresh68 = *src
        .offset(
            (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh68 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh68;
    let ref mut fresh69 = *src
        .offset(
            (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh69 = (t3 + t4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh69;
    let ref mut fresh70 = *src
        .offset(
            (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh70 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh70;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t4 + t5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
}
#[c2rust::src_loc = "610:1"]
unsafe extern "C" fn predict_4x4_hu_c(mut src: *mut pixel) {
    let mut l0: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l1: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l2: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    let mut l3: ::core::ffi::c_int = *src
        .offset(
            (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) as ::core::ffi::c_int;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (l0 + l1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (l0 + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh56 = *src
        .offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh56 = (l1 + l2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh56;
    let ref mut fresh57 = *src
        .offset(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh57 = (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh57;
    let ref mut fresh58 = *src
        .offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh58 = (l2 + l3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh58;
    let ref mut fresh59 = *src
        .offset(
            (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh59 = (l2 + 2 as ::core::ffi::c_int * l3 + l3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh59;
    let ref mut fresh60 = *src
        .offset(
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh60 = l3 as pixel;
    let ref mut fresh61 = *src
        .offset(
            (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh61 = *fresh60;
    let ref mut fresh62 = *src
        .offset(
            (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh62 = *fresh61;
    let ref mut fresh63 = *src
        .offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh63 = *fresh62;
    let ref mut fresh64 = *src
        .offset(
            (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh64 = *fresh63;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh64;
}
#[c2rust::src_loc = "632:1"]
unsafe extern "C" fn predict_8x8_filter_c(
    mut src: *mut pixel,
    mut edge: *mut pixel,
    mut i_neighbor: ::core::ffi::c_int,
    mut i_filters: ::core::ffi::c_int,
) {
    let mut have_lt: ::core::ffi::c_int = i_neighbor & MB_TOPLEFT as ::core::ffi::c_int;
    if i_filters & MB_LEFT as ::core::ffi::c_int != 0 {
        *edge.offset(15 as ::core::ffi::c_int as isize) = (*src
            .offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset(14 as ::core::ffi::c_int as isize) = ((if have_lt != 0 {
            *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                ) as ::core::ffi::c_int
        } else {
            *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
        })
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((14 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((14 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + 4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((14 as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + 5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((14 as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (-(1 as ::core::ffi::c_int)
                    + (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + 6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        let ref mut fresh113 = *edge.offset(7 as ::core::ffi::c_int as isize);
        *fresh113 = (*src
            .offset(
                (-(1 as ::core::ffi::c_int) + 6 as ::core::ffi::c_int * FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int
                * *src
                    .offset(
                        (-(1 as ::core::ffi::c_int)
                            + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
                    ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset(6 as ::core::ffi::c_int as isize) = *fresh113;
    }
    if i_filters & MB_TOP as ::core::ffi::c_int != 0 {
        let mut have_tr: ::core::ffi::c_int = i_neighbor
            & MB_TOPRIGHT as ::core::ffi::c_int;
        *edge.offset(16 as ::core::ffi::c_int as isize) = ((if have_lt != 0 {
            *src
                .offset(
                    (-(1 as ::core::ffi::c_int)
                        + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                ) as ::core::ffi::c_int
        } else {
            *src
                .offset(
                    (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
        })
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (0 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (2 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (3 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (4 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (5 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset((16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize) = (*src
            .offset(
                (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (6 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
            + *src
                .offset(
                    (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as pixel;
        *edge.offset(23 as ::core::ffi::c_int as isize) = (*src
            .offset(
                (6 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
                * *src
                    .offset(
                        (7 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                    ) as ::core::ffi::c_int
            + (if have_tr != 0 {
                *src
                    .offset(
                        (8 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                    ) as ::core::ffi::c_int
            } else {
                *src
                    .offset(
                        (7 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                    ) as ::core::ffi::c_int
            }) + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as pixel;
        if i_filters & MB_TOPRIGHT as ::core::ffi::c_int != 0 {
            if have_tr != 0 {
                *edge
                    .offset(
                        (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                    ) = (*src
                    .offset(
                        (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src
                            .offset(
                                (8 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                    + *src
                        .offset(
                            (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as pixel;
                *edge
                    .offset(
                        (16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize,
                    ) = (*src
                    .offset(
                        (9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src
                            .offset(
                                (9 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                    + *src
                        .offset(
                            (9 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as pixel;
                *edge
                    .offset(
                        (16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                    ) = (*src
                    .offset(
                        (10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src
                            .offset(
                                (10 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                    + *src
                        .offset(
                            (10 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as pixel;
                *edge
                    .offset(
                        (16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize,
                    ) = (*src
                    .offset(
                        (11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src
                            .offset(
                                (11 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                    + *src
                        .offset(
                            (11 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as pixel;
                *edge
                    .offset(
                        (16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize,
                    ) = (*src
                    .offset(
                        (12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src
                            .offset(
                                (12 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                    + *src
                        .offset(
                            (12 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as pixel;
                *edge
                    .offset(
                        (16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize,
                    ) = (*src
                    .offset(
                        (13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src
                            .offset(
                                (13 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                    + *src
                        .offset(
                            (13 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as pixel;
                *edge
                    .offset(
                        (16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize,
                    ) = (*src
                    .offset(
                        (14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src
                            .offset(
                                (14 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                    + *src
                        .offset(
                            (14 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as pixel;
                let ref mut fresh114 = *edge.offset(32 as ::core::ffi::c_int as isize);
                *fresh114 = (*src
                    .offset(
                        (14 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                    ) as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int
                        * *src
                            .offset(
                                (15 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                            ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as pixel;
                *edge.offset(31 as ::core::ffi::c_int as isize) = *fresh114;
            } else {
                (*(edge.offset(24 as ::core::ffi::c_int as isize)
                    as *mut x264_union64_t))
                    .i = (*src
                    .offset(
                        (7 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_ulonglong)
                    .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong)
                    as uint64_t;
                (*(edge.offset(28 as ::core::ffi::c_int as isize)
                    as *mut x264_union64_t))
                    .i = (*src
                    .offset(
                        (7 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_ulonglong)
                    .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong)
                    as uint64_t;
                *edge.offset(32 as ::core::ffi::c_int as isize) = *src
                    .offset(
                        (7 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                    );
            }
        }
    }
}
#[c2rust::src_loc = "700:1"]
unsafe extern "C" fn predict_8x8_dc_128_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = (((1
            as ::core::ffi::c_int) << 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = (((1
            as ::core::ffi::c_int) << 10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y += 1;
    }
}
#[c2rust::src_loc = "704:1"]
unsafe extern "C" fn predict_8x8_dc_left_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut l0: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l1: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l2: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l3: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l4: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l5: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l6: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l7: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut dc: pixel4 = ((l0 + l1 + l2 + l3 + l4 + l5 + l6 + l7
        + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y += 1;
    }
}
#[c2rust::src_loc = "710:1"]
unsafe extern "C" fn predict_8x8_dc_top_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t4: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t5: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t6: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t7: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut dc: pixel4 = ((t0 + t1 + t2 + t3 + t4 + t5 + t6 + t7
        + 4 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "716:1"]
pub unsafe extern "C" fn x264_10_predict_8x8_dc_c(
    mut src: *mut pixel,
    mut edge: *mut pixel,
) {
    let mut l0: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l1: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l2: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l3: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l4: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l5: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l6: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l7: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t0: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t4: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t5: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t6: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t7: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut dc: pixel4 = ((l0 + l1 + l2 + l3 + l4 + l5 + l6 + l7 + t0 + t1 + t2 + t3 + t4
        + t5 + t6 + t7 + 8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as pixel4;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*(src.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc
            as uint64_t;
        (*(src.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i = dc
            as uint64_t;
        src = src.offset(FDEC_STRIDE as isize);
        y += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "723:1"]
pub unsafe extern "C" fn x264_10_predict_8x8_h_c(
    mut src: *mut pixel,
    mut edge: *mut pixel,
) {
    let mut l0: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l1: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l2: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l3: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l4: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l5: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l6: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l7: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let ref mut fresh0 = (*(src
        .offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i;
    *fresh0 = (l0 as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    (*(src
        .offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i = *fresh0;
    let ref mut fresh1 = (*(src
        .offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i;
    *fresh1 = (l1 as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    (*(src
        .offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i = *fresh1;
    let ref mut fresh2 = (*(src
        .offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i;
    *fresh2 = (l2 as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    (*(src
        .offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i = *fresh2;
    let ref mut fresh3 = (*(src
        .offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i;
    *fresh3 = (l3 as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    (*(src
        .offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i = *fresh3;
    let ref mut fresh4 = (*(src
        .offset((4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i;
    *fresh4 = (l4 as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    (*(src
        .offset((4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i = *fresh4;
    let ref mut fresh5 = (*(src
        .offset((5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i;
    *fresh5 = (l5 as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    (*(src
        .offset((5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i = *fresh5;
    let ref mut fresh6 = (*(src
        .offset((6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i;
    *fresh6 = (l6 as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    (*(src
        .offset((6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i = *fresh6;
    let ref mut fresh7 = (*(src
        .offset((7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i;
    *fresh7 = (l7 as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) as uint64_t;
    (*(src
        .offset((7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
        .offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
        .i = *fresh7;
}
#[no_mangle]
#[c2rust::src_loc = "731:1"]
pub unsafe extern "C" fn x264_10_predict_8x8_v_c(
    mut src: *mut pixel,
    mut edge: *mut pixel,
) {
    let mut top: [pixel4; 2] = [
        (*(edge.offset(16 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i,
        (*(edge.offset(20 as ::core::ffi::c_int as isize) as *mut x264_union64_t)).i,
    ];
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        (*(src
            .offset((y * 32 as ::core::ffi::c_int) as isize)
            .offset(0 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
            .i = top[0 as ::core::ffi::c_int as usize] as uint64_t;
        (*(src
            .offset((y * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize) as *mut x264_union64_t))
            .i = top[1 as ::core::ffi::c_int as usize] as uint64_t;
        y += 1;
    }
}
#[c2rust::src_loc = "741:1"]
unsafe extern "C" fn predict_8x8_ddl_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t4: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t5: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t6: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t7: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t8: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t9: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t10: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t11: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t12: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t13: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t14: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t15: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh262 = *src
        .offset(
            (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh262 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh262;
    let ref mut fresh263 = *src
        .offset(
            (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh263 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh264 = *src
        .offset(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh264 = *fresh263;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh264;
    let ref mut fresh265 = *src
        .offset(
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh265 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh266 = *src
        .offset(
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh266 = *fresh265;
    let ref mut fresh267 = *src
        .offset(
            (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh267 = *fresh266;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh267;
    let ref mut fresh268 = *src
        .offset(
            (4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh268 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh269 = *src
        .offset(
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh269 = *fresh268;
    let ref mut fresh270 = *src
        .offset(
            (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh270 = *fresh269;
    let ref mut fresh271 = *src
        .offset(
            (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh271 = *fresh270;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh271;
    let ref mut fresh272 = *src
        .offset(
            (5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh272 = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh273 = *src
        .offset(
            (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh273 = *fresh272;
    let ref mut fresh274 = *src
        .offset(
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh274 = *fresh273;
    let ref mut fresh275 = *src
        .offset(
            (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh275 = *fresh274;
    let ref mut fresh276 = *src
        .offset(
            (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh276 = *fresh275;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh276;
    let ref mut fresh277 = *src
        .offset(
            (6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh277 = (t6 + 2 as ::core::ffi::c_int * t7 + t8 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh278 = *src
        .offset(
            (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh278 = *fresh277;
    let ref mut fresh279 = *src
        .offset(
            (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh279 = *fresh278;
    let ref mut fresh280 = *src
        .offset(
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh280 = *fresh279;
    let ref mut fresh281 = *src
        .offset(
            (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh281 = *fresh280;
    let ref mut fresh282 = *src
        .offset(
            (1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh282 = *fresh281;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh282;
    let ref mut fresh283 = *src
        .offset(
            (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh283 = (t7 + 2 as ::core::ffi::c_int * t8 + t9 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh284 = *src
        .offset(
            (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh284 = *fresh283;
    let ref mut fresh285 = *src
        .offset(
            (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh285 = *fresh284;
    let ref mut fresh286 = *src
        .offset(
            (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh286 = *fresh285;
    let ref mut fresh287 = *src
        .offset(
            (3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh287 = *fresh286;
    let ref mut fresh288 = *src
        .offset(
            (2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh288 = *fresh287;
    let ref mut fresh289 = *src
        .offset(
            (1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh289 = *fresh288;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh289;
    let ref mut fresh290 = *src
        .offset(
            (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh290 = (t8 + 2 as ::core::ffi::c_int * t9 + t10 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh291 = *src
        .offset(
            (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh291 = *fresh290;
    let ref mut fresh292 = *src
        .offset(
            (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh292 = *fresh291;
    let ref mut fresh293 = *src
        .offset(
            (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh293 = *fresh292;
    let ref mut fresh294 = *src
        .offset(
            (3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh294 = *fresh293;
    let ref mut fresh295 = *src
        .offset(
            (2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh295 = *fresh294;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh295;
    let ref mut fresh296 = *src
        .offset(
            (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh296 = (t9 + 2 as ::core::ffi::c_int * t10 + t11 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh297 = *src
        .offset(
            (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh297 = *fresh296;
    let ref mut fresh298 = *src
        .offset(
            (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh298 = *fresh297;
    let ref mut fresh299 = *src
        .offset(
            (4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh299 = *fresh298;
    let ref mut fresh300 = *src
        .offset(
            (3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh300 = *fresh299;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh300;
    let ref mut fresh301 = *src
        .offset(
            (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh301 = (t10 + 2 as ::core::ffi::c_int * t11 + t12 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh302 = *src
        .offset(
            (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh302 = *fresh301;
    let ref mut fresh303 = *src
        .offset(
            (5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh303 = *fresh302;
    let ref mut fresh304 = *src
        .offset(
            (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh304 = *fresh303;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh304;
    let ref mut fresh305 = *src
        .offset(
            (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh305 = (t11 + 2 as ::core::ffi::c_int * t12 + t13 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh306 = *src
        .offset(
            (6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh306 = *fresh305;
    let ref mut fresh307 = *src
        .offset(
            (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh307 = *fresh306;
    *src
        .offset(
            (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh307;
    let ref mut fresh308 = *src
        .offset(
            (7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh308 = (t12 + 2 as ::core::ffi::c_int * t13 + t14 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh309 = *src
        .offset(
            (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh309 = *fresh308;
    *src
        .offset(
            (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh309;
    let ref mut fresh310 = *src
        .offset(
            (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh310 = (t13 + 2 as ::core::ffi::c_int * t14 + t15 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh310;
    *src
        .offset(
            (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t14 + 2 as ::core::ffi::c_int * t15 + t15 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
}
#[c2rust::src_loc = "761:1"]
unsafe extern "C" fn predict_8x8_ddr_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t4: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t5: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t6: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t7: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l0: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l1: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l2: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l3: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l4: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l5: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l6: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l7: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut lt: ::core::ffi::c_int = *edge.offset(15 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (l7 + 2 as ::core::ffi::c_int * l6 + l5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh213 = *src
        .offset(
            (1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh213 = (l6 + 2 as ::core::ffi::c_int * l5 + l4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh213;
    let ref mut fresh214 = *src
        .offset(
            (2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh214 = (l5 + 2 as ::core::ffi::c_int * l4 + l3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh215 = *src
        .offset(
            (1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh215 = *fresh214;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh215;
    let ref mut fresh216 = *src
        .offset(
            (3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh216 = (l4 + 2 as ::core::ffi::c_int * l3 + l2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh217 = *src
        .offset(
            (2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh217 = *fresh216;
    let ref mut fresh218 = *src
        .offset(
            (1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh218 = *fresh217;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh218;
    let ref mut fresh219 = *src
        .offset(
            (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh219 = (l3 + 2 as ::core::ffi::c_int * l2 + l1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh220 = *src
        .offset(
            (3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh220 = *fresh219;
    let ref mut fresh221 = *src
        .offset(
            (2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh221 = *fresh220;
    let ref mut fresh222 = *src
        .offset(
            (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh222 = *fresh221;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh222;
    let ref mut fresh223 = *src
        .offset(
            (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh223 = (l2 + 2 as ::core::ffi::c_int * l1 + l0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh224 = *src
        .offset(
            (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh224 = *fresh223;
    let ref mut fresh225 = *src
        .offset(
            (3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh225 = *fresh224;
    let ref mut fresh226 = *src
        .offset(
            (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh226 = *fresh225;
    let ref mut fresh227 = *src
        .offset(
            (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh227 = *fresh226;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh227;
    let ref mut fresh228 = *src
        .offset(
            (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh228 = (l1 + 2 as ::core::ffi::c_int * l0 + lt + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh229 = *src
        .offset(
            (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh229 = *fresh228;
    let ref mut fresh230 = *src
        .offset(
            (4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh230 = *fresh229;
    let ref mut fresh231 = *src
        .offset(
            (3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh231 = *fresh230;
    let ref mut fresh232 = *src
        .offset(
            (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh232 = *fresh231;
    let ref mut fresh233 = *src
        .offset(
            (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh233 = *fresh232;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh233;
    let ref mut fresh234 = *src
        .offset(
            (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh234 = (l0 + 2 as ::core::ffi::c_int * lt + t0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh235 = *src
        .offset(
            (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh235 = *fresh234;
    let ref mut fresh236 = *src
        .offset(
            (5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh236 = *fresh235;
    let ref mut fresh237 = *src
        .offset(
            (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh237 = *fresh236;
    let ref mut fresh238 = *src
        .offset(
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh238 = *fresh237;
    let ref mut fresh239 = *src
        .offset(
            (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh239 = *fresh238;
    let ref mut fresh240 = *src
        .offset(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh240 = *fresh239;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh240;
    let ref mut fresh241 = *src
        .offset(
            (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh241 = (lt + 2 as ::core::ffi::c_int * t0 + t1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh242 = *src
        .offset(
            (6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh242 = *fresh241;
    let ref mut fresh243 = *src
        .offset(
            (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh243 = *fresh242;
    let ref mut fresh244 = *src
        .offset(
            (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh244 = *fresh243;
    let ref mut fresh245 = *src
        .offset(
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh245 = *fresh244;
    let ref mut fresh246 = *src
        .offset(
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh246 = *fresh245;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh246;
    let ref mut fresh247 = *src
        .offset(
            (7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh247 = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh248 = *src
        .offset(
            (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh248 = *fresh247;
    let ref mut fresh249 = *src
        .offset(
            (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh249 = *fresh248;
    let ref mut fresh250 = *src
        .offset(
            (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh250 = *fresh249;
    let ref mut fresh251 = *src
        .offset(
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh251 = *fresh250;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh251;
    let ref mut fresh252 = *src
        .offset(
            (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh252 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh253 = *src
        .offset(
            (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh253 = *fresh252;
    let ref mut fresh254 = *src
        .offset(
            (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh254 = *fresh253;
    let ref mut fresh255 = *src
        .offset(
            (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh255 = *fresh254;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh255;
    let ref mut fresh256 = *src
        .offset(
            (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh256 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh257 = *src
        .offset(
            (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh257 = *fresh256;
    let ref mut fresh258 = *src
        .offset(
            (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh258 = *fresh257;
    *src
        .offset(
            (4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh258;
    let ref mut fresh259 = *src
        .offset(
            (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh259 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh260 = *src
        .offset(
            (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh260 = *fresh259;
    *src
        .offset(
            (5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh260;
    let ref mut fresh261 = *src
        .offset(
            (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh261 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh261;
    *src
        .offset(
            (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
}
#[c2rust::src_loc = "783:1"]
unsafe extern "C" fn predict_8x8_vr_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t4: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t5: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t6: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t7: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l0: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l1: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l2: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l3: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l4: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l5: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l6: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l7: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut lt: ::core::ffi::c_int = *edge.offset(15 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (l5 + 2 as ::core::ffi::c_int * l4 + l3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (l6 + 2 as ::core::ffi::c_int * l5 + l4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh171 = *src
        .offset(
            (1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh171 = (l3 + 2 as ::core::ffi::c_int * l2 + l1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh171;
    let ref mut fresh172 = *src
        .offset(
            (1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh172 = (l4 + 2 as ::core::ffi::c_int * l3 + l2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh172;
    let ref mut fresh173 = *src
        .offset(
            (2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh173 = (l1 + 2 as ::core::ffi::c_int * l0 + lt + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh174 = *src
        .offset(
            (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh174 = *fresh173;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh174;
    let ref mut fresh175 = *src
        .offset(
            (2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh175 = (l2 + 2 as ::core::ffi::c_int * l1 + l0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh176 = *src
        .offset(
            (1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh176 = *fresh175;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh176;
    let ref mut fresh177 = *src
        .offset(
            (3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh177 = (l0 + 2 as ::core::ffi::c_int * lt + t0 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh178 = *src
        .offset(
            (2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh178 = *fresh177;
    let ref mut fresh179 = *src
        .offset(
            (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh179 = *fresh178;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh179;
    let ref mut fresh180 = *src
        .offset(
            (3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh180 = (lt + t0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh181 = *src
        .offset(
            (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh181 = *fresh180;
    let ref mut fresh182 = *src
        .offset(
            (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh182 = *fresh181;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh182;
    let ref mut fresh183 = *src
        .offset(
            (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh183 = (lt + 2 as ::core::ffi::c_int * t0 + t1 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh184 = *src
        .offset(
            (3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh184 = *fresh183;
    let ref mut fresh185 = *src
        .offset(
            (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh185 = *fresh184;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh185;
    let ref mut fresh186 = *src
        .offset(
            (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh186 = (t0 + t1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh187 = *src
        .offset(
            (3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh187 = *fresh186;
    let ref mut fresh188 = *src
        .offset(
            (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh188 = *fresh187;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh188;
    let ref mut fresh189 = *src
        .offset(
            (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh189 = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh190 = *src
        .offset(
            (4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh190 = *fresh189;
    let ref mut fresh191 = *src
        .offset(
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh191 = *fresh190;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh191;
    let ref mut fresh192 = *src
        .offset(
            (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh192 = (t1 + t2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh193 = *src
        .offset(
            (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh193 = *fresh192;
    let ref mut fresh194 = *src
        .offset(
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh194 = *fresh193;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh194;
    let ref mut fresh195 = *src
        .offset(
            (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh195 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh196 = *src
        .offset(
            (5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh196 = *fresh195;
    let ref mut fresh197 = *src
        .offset(
            (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh197 = *fresh196;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh197;
    let ref mut fresh198 = *src
        .offset(
            (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh198 = (t2 + t3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh199 = *src
        .offset(
            (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh199 = *fresh198;
    let ref mut fresh200 = *src
        .offset(
            (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh200 = *fresh199;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh200;
    let ref mut fresh201 = *src
        .offset(
            (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh201 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh202 = *src
        .offset(
            (6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh202 = *fresh201;
    let ref mut fresh203 = *src
        .offset(
            (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh203 = *fresh202;
    *src
        .offset(
            (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh203;
    let ref mut fresh204 = *src
        .offset(
            (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh204 = (t3 + t4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh205 = *src
        .offset(
            (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh205 = *fresh204;
    let ref mut fresh206 = *src
        .offset(
            (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh206 = *fresh205;
    *src
        .offset(
            (4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh206;
    let ref mut fresh207 = *src
        .offset(
            (7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh207 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh208 = *src
        .offset(
            (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh208 = *fresh207;
    *src
        .offset(
            (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh208;
    let ref mut fresh209 = *src
        .offset(
            (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh209 = (t4 + t5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh210 = *src
        .offset(
            (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh210 = *fresh209;
    *src
        .offset(
            (5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh210;
    let ref mut fresh211 = *src
        .offset(
            (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh211 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh211;
    let ref mut fresh212 = *src
        .offset(
            (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh212 = (t5 + t6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh212;
    *src
        .offset(
            (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t6 + t7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
}
#[c2rust::src_loc = "811:1"]
unsafe extern "C" fn predict_8x8_hd_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t4: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t5: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t6: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t7: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l0: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l1: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l2: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l3: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l4: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l5: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l6: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l7: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut lt: ::core::ffi::c_int = *edge.offset(15 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int;
    let mut p1: ::core::ffi::c_int = pack16to32(
        (l6 + l7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l5 + 2 as ::core::ffi::c_int * l6 + l7 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p2: ::core::ffi::c_int = pack16to32(
        (l5 + l6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l4 + 2 as ::core::ffi::c_int * l5 + l6 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p3: ::core::ffi::c_int = pack16to32(
        (l4 + l5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l3 + 2 as ::core::ffi::c_int * l4 + l5 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p4: ::core::ffi::c_int = pack16to32(
        (l3 + l4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l2 + 2 as ::core::ffi::c_int * l3 + l4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p5: ::core::ffi::c_int = pack16to32(
        (l2 + l3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p6: ::core::ffi::c_int = pack16to32(
        (l1 + l2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l0 + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p7: ::core::ffi::c_int = pack16to32(
        (l0 + l1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (lt + 2 as ::core::ffi::c_int * l0 + l1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p8: ::core::ffi::c_int = pack16to32(
        (lt + l0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l0 + 2 as ::core::ffi::c_int * lt + t0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p9: ::core::ffi::c_int = pack16to32(
        (t1 + 2 as ::core::ffi::c_int * t0 + lt + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
        (t2 + 2 as ::core::ffi::c_int * t1 + t0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p10: ::core::ffi::c_int = pack16to32(
        (t3 + 2 as ::core::ffi::c_int * t2 + t1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
        (t4 + 2 as ::core::ffi::c_int * t3 + t2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p11: ::core::ffi::c_int = pack16to32(
        (t5 + 2 as ::core::ffi::c_int * t4 + t3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
        (t6 + 2 as ::core::ffi::c_int * t5 + t4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = pack32to64(p1 as uint32_t, p2 as uint32_t);
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = pack32to64(p2 as uint32_t, p3 as uint32_t);
    let ref mut fresh165 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh165 = pack32to64(p3 as uint32_t, p4 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh165;
    let ref mut fresh166 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh166 = pack32to64(p4 as uint32_t, p5 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh166;
    let ref mut fresh167 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh167 = pack32to64(p5 as uint32_t, p6 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh167;
    let ref mut fresh168 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh168 = pack32to64(p6 as uint32_t, p7 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh168;
    let ref mut fresh169 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh169 = pack32to64(p7 as uint32_t, p8 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh169;
    let ref mut fresh170 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh170 = pack32to64(p8 as uint32_t, p9 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh170;
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = pack32to64(p9 as uint32_t, p10 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = pack32to64(p10 as uint32_t, p11 as uint32_t);
}
#[c2rust::src_loc = "838:1"]
unsafe extern "C" fn predict_8x8_vl_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut t0: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t1: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t2: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t3: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t4: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t5: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t6: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t7: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t8: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t9: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t10: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t11: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t12: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t13: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t14: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut t15: ::core::ffi::c_int = *edge
        .offset((16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t0 + t1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh123 = *src
        .offset(
            (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh123 = (t1 + t2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh123;
    let ref mut fresh124 = *src
        .offset(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh124 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh124;
    let ref mut fresh125 = *src
        .offset(
            (2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh125 = (t2 + t3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh126 = *src
        .offset(
            (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh126 = *fresh125;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh126;
    let ref mut fresh127 = *src
        .offset(
            (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh127 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh128 = *src
        .offset(
            (1 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh128 = *fresh127;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh128;
    let ref mut fresh129 = *src
        .offset(
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh129 = (t3 + t4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh130 = *src
        .offset(
            (2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh130 = *fresh129;
    let ref mut fresh131 = *src
        .offset(
            (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh131 = *fresh130;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh131;
    let ref mut fresh132 = *src
        .offset(
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh132 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh133 = *src
        .offset(
            (2 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh133 = *fresh132;
    let ref mut fresh134 = *src
        .offset(
            (1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh134 = *fresh133;
    *src
        .offset(
            (0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh134;
    let ref mut fresh135 = *src
        .offset(
            (4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh135 = (t4 + t5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh136 = *src
        .offset(
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh136 = *fresh135;
    let ref mut fresh137 = *src
        .offset(
            (2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh137 = *fresh136;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh137;
    let ref mut fresh138 = *src
        .offset(
            (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh138 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh139 = *src
        .offset(
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh139 = *fresh138;
    let ref mut fresh140 = *src
        .offset(
            (2 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh140 = *fresh139;
    *src
        .offset(
            (1 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh140;
    let ref mut fresh141 = *src
        .offset(
            (5 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh141 = (t5 + t6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh142 = *src
        .offset(
            (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh142 = *fresh141;
    let ref mut fresh143 = *src
        .offset(
            (3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh143 = *fresh142;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh143;
    let ref mut fresh144 = *src
        .offset(
            (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh144 = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh145 = *src
        .offset(
            (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh145 = *fresh144;
    let ref mut fresh146 = *src
        .offset(
            (3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh146 = *fresh145;
    *src
        .offset(
            (2 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh146;
    let ref mut fresh147 = *src
        .offset(
            (6 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh147 = (t6 + t7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh148 = *src
        .offset(
            (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh148 = *fresh147;
    let ref mut fresh149 = *src
        .offset(
            (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh149 = *fresh148;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh149;
    let ref mut fresh150 = *src
        .offset(
            (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh150 = (t6 + 2 as ::core::ffi::c_int * t7 + t8 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh151 = *src
        .offset(
            (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh151 = *fresh150;
    let ref mut fresh152 = *src
        .offset(
            (4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh152 = *fresh151;
    *src
        .offset(
            (3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh152;
    let ref mut fresh153 = *src
        .offset(
            (7 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh153 = (t7 + t8 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh154 = *src
        .offset(
            (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh154 = *fresh153;
    let ref mut fresh155 = *src
        .offset(
            (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh155 = *fresh154;
    *src
        .offset(
            (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh155;
    let ref mut fresh156 = *src
        .offset(
            (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh156 = (t7 + 2 as ::core::ffi::c_int * t8 + t9 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh157 = *src
        .offset(
            (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh157 = *fresh156;
    let ref mut fresh158 = *src
        .offset(
            (5 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh158 = *fresh157;
    *src
        .offset(
            (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh158;
    let ref mut fresh159 = *src
        .offset(
            (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh159 = (t8 + t9 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    let ref mut fresh160 = *src
        .offset(
            (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh160 = *fresh159;
    *src
        .offset(
            (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh160;
    let ref mut fresh161 = *src
        .offset(
            (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh161 = (t8 + 2 as ::core::ffi::c_int * t9 + t10 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    let ref mut fresh162 = *src
        .offset(
            (6 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh162 = *fresh161;
    *src
        .offset(
            (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh162;
    let ref mut fresh163 = *src
        .offset(
            (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh163 = (t9 + t10 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh163;
    let ref mut fresh164 = *src
        .offset(
            (7 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        );
    *fresh164 = (t9 + 2 as ::core::ffi::c_int * t10 + t11 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = *fresh164;
    *src
        .offset(
            (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t10 + t11 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
    *src
        .offset(
            (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
        ) = (t10 + 2 as ::core::ffi::c_int * t11 + t12 + 2 as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int) as pixel;
}
#[c2rust::src_loc = "865:1"]
unsafe extern "C" fn predict_8x8_hu_c(mut src: *mut pixel, mut edge: *mut pixel) {
    let mut l0: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l1: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l2: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l3: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l4: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l5: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l6: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut l7: ::core::ffi::c_int = *edge
        .offset((14 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int;
    let mut p1: ::core::ffi::c_int = pack16to32(
        (l0 + l1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l0 + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p2: ::core::ffi::c_int = pack16to32(
        (l1 + l2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p3: ::core::ffi::c_int = pack16to32(
        (l2 + l3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l2 + 2 as ::core::ffi::c_int * l3 + l4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p4: ::core::ffi::c_int = pack16to32(
        (l3 + l4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l3 + 2 as ::core::ffi::c_int * l4 + l5 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p5: ::core::ffi::c_int = pack16to32(
        (l4 + l5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l4 + 2 as ::core::ffi::c_int * l5 + l6 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p6: ::core::ffi::c_int = pack16to32(
        (l5 + l6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l5 + 2 as ::core::ffi::c_int * l6 + l7 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p7: ::core::ffi::c_int = pack16to32(
        (l6 + l7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as uint32_t,
        (l6 + 2 as ::core::ffi::c_int * l7 + l7 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as uint32_t,
    ) as ::core::ffi::c_int;
    let mut p8: ::core::ffi::c_int = pack16to32(l7 as uint32_t, l7 as uint32_t)
        as ::core::ffi::c_int;
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = pack32to64(p1 as uint32_t, p2 as uint32_t);
    (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = pack32to64(p2 as uint32_t, p3 as uint32_t);
    let ref mut fresh115 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh115 = pack32to64(p3 as uint32_t, p4 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh115;
    let ref mut fresh116 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh116 = pack32to64(p4 as uint32_t, p5 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh116;
    let ref mut fresh117 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh117 = pack32to64(p5 as uint32_t, p6 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh117;
    let ref mut fresh118 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh118 = pack32to64(p6 as uint32_t, p7 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh118;
    let ref mut fresh119 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh119 = pack32to64(p7 as uint32_t, p8 as uint32_t);
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh119;
    let ref mut fresh120 = (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh120 = pack32to64(p8 as uint32_t, p8 as uint32_t);
    let ref mut fresh121 = (*(&mut *src
        .offset(
            (0 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh121 = *fresh120;
    let ref mut fresh122 = (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i;
    *fresh122 = *fresh121;
    (*(&mut *src
        .offset(
            (4 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut pixel as *mut x264_union64_t))
        .i = *fresh122;
}
#[no_mangle]
#[c2rust::src_loc = "889:1"]
pub unsafe extern "C" fn x264_10_predict_16x16_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_predict_t,
) {
    let ref mut fresh14 = *pf.offset(I_PRED_16x16_V as ::core::ffi::c_int as isize);
    *fresh14 = Some(x264_10_predict_16x16_v_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh15 = *pf.offset(I_PRED_16x16_H as ::core::ffi::c_int as isize);
    *fresh15 = Some(x264_10_predict_16x16_h_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh16 = *pf.offset(I_PRED_16x16_DC as ::core::ffi::c_int as isize);
    *fresh16 = Some(x264_10_predict_16x16_dc_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh17 = *pf.offset(I_PRED_16x16_P as ::core::ffi::c_int as isize);
    *fresh17 = Some(x264_10_predict_16x16_p_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh18 = *pf
        .offset(I_PRED_16x16_DC_LEFT as ::core::ffi::c_int as isize);
    *fresh18 = Some(predict_16x16_dc_left_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh19 = *pf.offset(I_PRED_16x16_DC_TOP as ::core::ffi::c_int as isize);
    *fresh19 = Some(predict_16x16_dc_top_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh20 = *pf.offset(I_PRED_16x16_DC_128 as ::core::ffi::c_int as isize);
    *fresh20 = Some(predict_16x16_dc_128_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
}
#[no_mangle]
#[c2rust::src_loc = "936:1"]
pub unsafe extern "C" fn x264_10_predict_8x8c_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_predict_t,
) {
    let ref mut fresh21 = *pf.offset(I_PRED_CHROMA_V as ::core::ffi::c_int as isize);
    *fresh21 = Some(x264_10_predict_8x8c_v_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh22 = *pf.offset(I_PRED_CHROMA_H as ::core::ffi::c_int as isize);
    *fresh22 = Some(x264_10_predict_8x8c_h_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh23 = *pf.offset(I_PRED_CHROMA_DC as ::core::ffi::c_int as isize);
    *fresh23 = Some(x264_10_predict_8x8c_dc_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh24 = *pf.offset(I_PRED_CHROMA_P as ::core::ffi::c_int as isize);
    *fresh24 = Some(x264_10_predict_8x8c_p_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh25 = *pf
        .offset(I_PRED_CHROMA_DC_LEFT as ::core::ffi::c_int as isize);
    *fresh25 = Some(predict_8x8c_dc_left_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh26 = *pf
        .offset(I_PRED_CHROMA_DC_TOP as ::core::ffi::c_int as isize);
    *fresh26 = Some(predict_8x8c_dc_top_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh27 = *pf
        .offset(I_PRED_CHROMA_DC_128 as ::core::ffi::c_int as isize);
    *fresh27 = Some(predict_8x8c_dc_128_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
}
#[no_mangle]
#[c2rust::src_loc = "977:1"]
pub unsafe extern "C" fn x264_10_predict_8x16c_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_predict_t,
) {
    let ref mut fresh28 = *pf.offset(I_PRED_CHROMA_V as ::core::ffi::c_int as isize);
    *fresh28 = Some(x264_10_predict_8x16c_v_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh29 = *pf.offset(I_PRED_CHROMA_H as ::core::ffi::c_int as isize);
    *fresh29 = Some(x264_10_predict_8x16c_h_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh30 = *pf.offset(I_PRED_CHROMA_DC as ::core::ffi::c_int as isize);
    *fresh30 = Some(x264_10_predict_8x16c_dc_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh31 = *pf.offset(I_PRED_CHROMA_P as ::core::ffi::c_int as isize);
    *fresh31 = Some(x264_10_predict_8x16c_p_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh32 = *pf
        .offset(I_PRED_CHROMA_DC_LEFT as ::core::ffi::c_int as isize);
    *fresh32 = Some(predict_8x16c_dc_left_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh33 = *pf
        .offset(I_PRED_CHROMA_DC_TOP as ::core::ffi::c_int as isize);
    *fresh33 = Some(predict_8x16c_dc_top_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh34 = *pf
        .offset(I_PRED_CHROMA_DC_128 as ::core::ffi::c_int as isize);
    *fresh34 = Some(predict_8x16c_dc_128_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
}
#[no_mangle]
#[c2rust::src_loc = "1000:1"]
pub unsafe extern "C" fn x264_10_predict_8x8_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_predict8x8_t,
    mut predict_filter: *mut x264_predict_8x8_filter_t,
) {
    let ref mut fresh101 = *pf.offset(I_PRED_8x8_V as ::core::ffi::c_int as isize);
    *fresh101 = Some(
        x264_10_predict_8x8_v_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    let ref mut fresh102 = *pf.offset(I_PRED_8x8_H as ::core::ffi::c_int as isize);
    *fresh102 = Some(
        x264_10_predict_8x8_h_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    let ref mut fresh103 = *pf.offset(I_PRED_8x8_DC as ::core::ffi::c_int as isize);
    *fresh103 = Some(
        x264_10_predict_8x8_dc_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    let ref mut fresh104 = *pf.offset(I_PRED_8x8_DDL as ::core::ffi::c_int as isize);
    *fresh104 = Some(
        predict_8x8_ddl_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    let ref mut fresh105 = *pf.offset(I_PRED_8x8_DDR as ::core::ffi::c_int as isize);
    *fresh105 = Some(
        predict_8x8_ddr_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    let ref mut fresh106 = *pf.offset(I_PRED_8x8_VR as ::core::ffi::c_int as isize);
    *fresh106 = Some(
        predict_8x8_vr_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    let ref mut fresh107 = *pf.offset(I_PRED_8x8_HD as ::core::ffi::c_int as isize);
    *fresh107 = Some(
        predict_8x8_hd_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    let ref mut fresh108 = *pf.offset(I_PRED_8x8_VL as ::core::ffi::c_int as isize);
    *fresh108 = Some(
        predict_8x8_vl_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    let ref mut fresh109 = *pf.offset(I_PRED_8x8_HU as ::core::ffi::c_int as isize);
    *fresh109 = Some(
        predict_8x8_hu_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    let ref mut fresh110 = *pf.offset(I_PRED_8x8_DC_LEFT as ::core::ffi::c_int as isize);
    *fresh110 = Some(
        predict_8x8_dc_left_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    let ref mut fresh111 = *pf.offset(I_PRED_8x8_DC_TOP as ::core::ffi::c_int as isize);
    *fresh111 = Some(
        predict_8x8_dc_top_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    let ref mut fresh112 = *pf.offset(I_PRED_8x8_DC_128 as ::core::ffi::c_int as isize);
    *fresh112 = Some(
        predict_8x8_dc_128_c as unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    ) as x264_predict8x8_t;
    *predict_filter = Some(
        predict_8x8_filter_c
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    ) as x264_predict_8x8_filter_t;
}
#[no_mangle]
#[c2rust::src_loc = "1042:1"]
pub unsafe extern "C" fn x264_10_predict_4x4_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_predict_t,
) {
    let ref mut fresh35 = *pf.offset(I_PRED_4x4_V as ::core::ffi::c_int as isize);
    *fresh35 = Some(x264_10_predict_4x4_v_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh36 = *pf.offset(I_PRED_4x4_H as ::core::ffi::c_int as isize);
    *fresh36 = Some(x264_10_predict_4x4_h_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh37 = *pf.offset(I_PRED_4x4_DC as ::core::ffi::c_int as isize);
    *fresh37 = Some(x264_10_predict_4x4_dc_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh38 = *pf.offset(I_PRED_4x4_DDL as ::core::ffi::c_int as isize);
    *fresh38 = Some(predict_4x4_ddl_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh39 = *pf.offset(I_PRED_4x4_DDR as ::core::ffi::c_int as isize);
    *fresh39 = Some(predict_4x4_ddr_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh40 = *pf.offset(I_PRED_4x4_VR as ::core::ffi::c_int as isize);
    *fresh40 = Some(predict_4x4_vr_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh41 = *pf.offset(I_PRED_4x4_HD as ::core::ffi::c_int as isize);
    *fresh41 = Some(predict_4x4_hd_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh42 = *pf.offset(I_PRED_4x4_VL as ::core::ffi::c_int as isize);
    *fresh42 = Some(predict_4x4_vl_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh43 = *pf.offset(I_PRED_4x4_HU as ::core::ffi::c_int as isize);
    *fresh43 = Some(predict_4x4_hu_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh44 = *pf.offset(I_PRED_4x4_DC_LEFT as ::core::ffi::c_int as isize);
    *fresh44 = Some(predict_4x4_dc_left_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh45 = *pf.offset(I_PRED_4x4_DC_TOP as ::core::ffi::c_int as isize);
    *fresh45 = Some(predict_4x4_dc_top_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
    let ref mut fresh46 = *pf.offset(I_PRED_4x4_DC_128 as ::core::ffi::c_int as isize);
    *fresh46 = Some(predict_4x4_dc_128_c as unsafe extern "C" fn(*mut pixel) -> ())
        as x264_predict_t;
}
