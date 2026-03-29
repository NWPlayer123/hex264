// =============== BEGIN predict_h ================
pub type x264_predict_t =
    Option<unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> ()>;

pub type x264_predict8x8_t = Option<
    unsafe extern "C" fn(
        *mut crate::src::common::common::pixel,
        *mut crate::src::common::common::pixel,
    ) -> (),
>;

pub type x264_predict_8x8_filter_t = Option<
    unsafe extern "C" fn(
        *mut crate::src::common::common::pixel,
        *mut crate::src::common::common::pixel,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;

pub type intra_chroma_pred_e = ::core::ffi::c_uint;

pub const I_PRED_CHROMA_DC: crate::src::common::predict::intra_chroma_pred_e = 0;

pub const I_PRED_CHROMA_H: crate::src::common::predict::intra_chroma_pred_e = 1;

pub const I_PRED_CHROMA_V: crate::src::common::predict::intra_chroma_pred_e = 2;

pub const I_PRED_CHROMA_P: crate::src::common::predict::intra_chroma_pred_e = 3;

pub const I_PRED_CHROMA_DC_LEFT: crate::src::common::predict::intra_chroma_pred_e = 4;

pub const I_PRED_CHROMA_DC_TOP: crate::src::common::predict::intra_chroma_pred_e = 5;

pub const I_PRED_CHROMA_DC_128: crate::src::common::predict::intra_chroma_pred_e = 6;

pub type intra16x16_pred_e = ::core::ffi::c_uint;

pub const I_PRED_16x16_V: crate::src::common::predict::intra16x16_pred_e = 0;

pub const I_PRED_16x16_H: crate::src::common::predict::intra16x16_pred_e = 1;

pub const I_PRED_16x16_DC: crate::src::common::predict::intra16x16_pred_e = 2;

pub const I_PRED_16x16_P: crate::src::common::predict::intra16x16_pred_e = 3;

pub const I_PRED_16x16_DC_LEFT: crate::src::common::predict::intra16x16_pred_e = 4;

pub const I_PRED_16x16_DC_TOP: crate::src::common::predict::intra16x16_pred_e = 5;

pub const I_PRED_16x16_DC_128: crate::src::common::predict::intra16x16_pred_e = 6;

pub type intra4x4_pred_e = ::core::ffi::c_uint;

pub const I_PRED_4x4_V: crate::src::common::predict::intra4x4_pred_e = 0;

pub const I_PRED_4x4_H: crate::src::common::predict::intra4x4_pred_e = 1;

pub const I_PRED_4x4_DC: crate::src::common::predict::intra4x4_pred_e = 2;

pub const I_PRED_4x4_DDL: crate::src::common::predict::intra4x4_pred_e = 3;

pub const I_PRED_4x4_DDR: crate::src::common::predict::intra4x4_pred_e = 4;

pub const I_PRED_4x4_VR: crate::src::common::predict::intra4x4_pred_e = 5;

pub const I_PRED_4x4_HD: crate::src::common::predict::intra4x4_pred_e = 6;

pub const I_PRED_4x4_VL: crate::src::common::predict::intra4x4_pred_e = 7;

pub const I_PRED_4x4_HU: crate::src::common::predict::intra4x4_pred_e = 8;

pub const I_PRED_4x4_DC_LEFT: crate::src::common::predict::intra4x4_pred_e = 9;

pub const I_PRED_4x4_DC_TOP: crate::src::common::predict::intra4x4_pred_e = 10;

pub const I_PRED_4x4_DC_128: crate::src::common::predict::intra4x4_pred_e = 11;

pub type intra8x8_pred_e = ::core::ffi::c_uint;

pub const I_PRED_8x8_V: crate::src::common::predict::intra8x8_pred_e = 0;

pub const I_PRED_8x8_H: crate::src::common::predict::intra8x8_pred_e = 1;

pub const I_PRED_8x8_DC: crate::src::common::predict::intra8x8_pred_e = 2;

pub const I_PRED_8x8_DDL: crate::src::common::predict::intra8x8_pred_e = 3;

pub const I_PRED_8x8_DDR: crate::src::common::predict::intra8x8_pred_e = 4;

pub const I_PRED_8x8_VR: crate::src::common::predict::intra8x8_pred_e = 5;

pub const I_PRED_8x8_HD: crate::src::common::predict::intra8x8_pred_e = 6;

pub const I_PRED_8x8_VL: crate::src::common::predict::intra8x8_pred_e = 7;

pub const I_PRED_8x8_HU: crate::src::common::predict::intra8x8_pred_e = 8;

pub const I_PRED_8x8_DC_LEFT: crate::src::common::predict::intra8x8_pred_e = 9;

pub const I_PRED_8x8_DC_TOP: crate::src::common::predict::intra8x8_pred_e = 10;

pub const I_PRED_8x8_DC_128: crate::src::common::predict::intra8x8_pred_e = 11;

pub mod common_h {

    #[inline(always)]

    pub unsafe extern "C" fn x264_clip_pixel(
        mut x: ::core::ffi::c_int,
    ) -> crate::src::common::common::pixel {
            return (if x & !crate::src::common::common::PIXEL_MAX != 0 {
                -x >> 31 as ::core::ffi::c_int & crate::src::common::common::PIXEL_MAX
            } else {
                x
            }) as crate::src::common::common::pixel;
       
    }
}

pub mod macroblock_h {

    #[inline(always)]

    pub  extern "C" fn pack16to32(
        mut a: crate::stdlib::uint32_t,
        mut b: crate::stdlib::uint32_t,
    ) -> crate::stdlib::uint32_t {
            return a.wrapping_add(b << 16 as ::core::ffi::c_int);
  
    }
    #[inline(always)]

    pub  extern "C" fn pack8to16(
        mut a: crate::stdlib::uint32_t,
        mut b: crate::stdlib::uint32_t,
    ) -> crate::stdlib::uint32_t {
     
            return a.wrapping_add(b << 8 as ::core::ffi::c_int);
       
    }
}

use crate::src::common::predict::common_h::x264_clip_pixel;
use crate::src::common::predict::macroblock_h::pack16to32;
use crate::src::common::predict::macroblock_h::pack8to16;
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_16x16_dc_c(
    mut src: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            dc += *src.offset(
                (-(1 as ::core::ffi::c_int) + i * crate::src::common::common::FDEC_STRIDE) as isize,
            ) as ::core::ffi::c_int;
            dc += *src.offset((i - crate::src::common::common::FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
            i += 1;
        }
        let mut dcsplat: crate::src::common::common::pixel4 = ((dc + 16 as ::core::ffi::c_int
            >> 5 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 16 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            (*(src.offset(8 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            (*(src.offset(12 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i_0 += 1;
        }
    }
}

unsafe extern "C" fn predict_16x16_dc_left_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            dc += *src.offset(
                (-(1 as ::core::ffi::c_int) + i * crate::src::common::common::FDEC_STRIDE) as isize,
            ) as ::core::ffi::c_int;
            i += 1;
        }
        let mut dcsplat: crate::src::common::common::pixel4 = ((dc + 8 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 16 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            (*(src.offset(8 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            (*(src.offset(12 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i_0 += 1;
        }
    }
}

unsafe extern "C" fn predict_16x16_dc_top_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            dc += *src.offset((i - crate::src::common::common::FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
            i += 1;
        }
        let mut dcsplat: crate::src::common::common::pixel4 = ((dc + 8 as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 16 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            (*(src.offset(8 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            (*(src.offset(12 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dcsplat as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i_0 += 1;
        }
    }
}

unsafe extern "C" fn predict_16x16_dc_128_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                as crate::stdlib::uint32_t;
            (*(src.offset(8 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                as crate::stdlib::uint32_t;
            (*(src.offset(12 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_16x16_h_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            let v: crate::src::common::common::pixel4 = (*src
                .offset(-(1 as ::core::ffi::c_int) as isize)
                as crate::src::common::common::pixel4)
                .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v as crate::stdlib::uint32_t;
            (*(src.offset(8 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v as crate::stdlib::uint32_t;
            (*(src.offset(12 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_16x16_v_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut v0: crate::src::common::common::pixel4 =
            (*(src.offset((0 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as isize)
                as *mut crate::src::common::common::pixel
                as *mut crate::src::common::base::x264_union32_t))
                .i as crate::src::common::common::pixel4;
        let mut v1: crate::src::common::common::pixel4 =
            (*(src.offset((4 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as isize)
                as *mut crate::src::common::common::pixel
                as *mut crate::src::common::base::x264_union32_t))
                .i as crate::src::common::common::pixel4;
        let mut v2: crate::src::common::common::pixel4 =
            (*(src.offset((8 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as isize)
                as *mut crate::src::common::common::pixel
                as *mut crate::src::common::base::x264_union32_t))
                .i as crate::src::common::common::pixel4;
        let mut v3: crate::src::common::common::pixel4 =
            (*(src.offset((12 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as isize)
                as *mut crate::src::common::common::pixel
                as *mut crate::src::common::base::x264_union32_t))
                .i as crate::src::common::common::pixel4;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v0 as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v1 as crate::stdlib::uint32_t;
            (*(src.offset(8 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v2 as crate::stdlib::uint32_t;
            (*(src.offset(12 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v3 as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_16x16_p_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut H: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut V: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i <= 7 as ::core::ffi::c_int {
            H += (i + 1 as ::core::ffi::c_int)
                * (*src.offset(
                    (8 as ::core::ffi::c_int + i - crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    - *src.offset(
                        (6 as ::core::ffi::c_int - i - crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int);
            V += (i + 1 as ::core::ffi::c_int)
                * (*src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + (8 as ::core::ffi::c_int + i) * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    - *src.offset(
                        (-(1 as ::core::ffi::c_int)
                            + (6 as ::core::ffi::c_int - i)
                                * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int);
            i += 1;
        }
        let mut a: ::core::ffi::c_int = 16 as ::core::ffi::c_int
            * (*src.offset(
                (-(1 as ::core::ffi::c_int)
                    + 15 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
                + *src.offset(
                    (15 as ::core::ffi::c_int - crate::src::common::common::FDEC_STRIDE) as isize,
                ) as ::core::ffi::c_int);
        let mut b: ::core::ffi::c_int =
            5 as ::core::ffi::c_int * H + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int;
        let mut c: ::core::ffi::c_int =
            5 as ::core::ffi::c_int * V + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int;
        let mut i00: ::core::ffi::c_int =
            a - b * 7 as ::core::ffi::c_int - c * 7 as ::core::ffi::c_int
                + 16 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            let mut pix: ::core::ffi::c_int = i00;
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 16 as ::core::ffi::c_int {
                *src.offset(x as isize) = x264_clip_pixel(pix >> 5 as ::core::ffi::c_int);
                pix += b;
                x += 1;
            }
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i00 += c;
            y += 1;
        }
    }
}

unsafe extern "C" fn predict_8x8c_dc_128_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
    }
}

unsafe extern "C" fn predict_8x8c_dc_left_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut dc0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut dc1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 4 as ::core::ffi::c_int {
            dc0 += *src.offset(
                (y * crate::src::common::common::FDEC_STRIDE - 1 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int;
            dc1 += *src.offset(
                ((y + 4 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE
                    - 1 as ::core::ffi::c_int) as isize,
            ) as ::core::ffi::c_int;
            y += 1;
        }
        let mut dc0splat: crate::src::common::common::pixel4 = ((dc0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc1splat: crate::src::common::common::pixel4 = ((dc1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < 4 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc0splat as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc0splat as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y_0 += 1;
        }
        let mut y_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_1 < 4 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc1splat as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc1splat as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y_1 += 1;
        }
    }
}

unsafe extern "C" fn predict_8x8c_dc_top_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut dc0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut dc1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            dc0 += *src.offset((x - crate::src::common::common::FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
            dc1 += *src.offset(
                (x + 4 as ::core::ffi::c_int - crate::src::common::common::FDEC_STRIDE) as isize,
            ) as ::core::ffi::c_int;
            x += 1;
        }
        let mut dc0splat: crate::src::common::common::pixel4 = ((dc0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc1splat: crate::src::common::common::pixel4 = ((dc1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc0splat as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc1splat as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x8c_dc_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut s0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut s1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut s2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut s3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            s0 += *src.offset((i - crate::src::common::common::FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
            s1 += *src.offset(
                (i + 4 as ::core::ffi::c_int - crate::src::common::common::FDEC_STRIDE) as isize,
            ) as ::core::ffi::c_int;
            s2 += *src.offset(
                (-(1 as ::core::ffi::c_int) + i * crate::src::common::common::FDEC_STRIDE) as isize,
            ) as ::core::ffi::c_int;
            s3 += *src.offset(
                (-(1 as ::core::ffi::c_int)
                    + (i + 4 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int;
            i += 1;
        }
        let mut dc0: crate::src::common::common::pixel4 = ((s0 + s2 + 4 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc1: crate::src::common::common::pixel4 = ((s1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc2: crate::src::common::common::pixel4 = ((s3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc3: crate::src::common::common::pixel4 = ((s1 + s3 + 4 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 4 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc0 as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc1 as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < 4 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc2 as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc3 as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y_0 += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x8c_h_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let mut v: crate::src::common::common::pixel4 = (*src
                .offset(-(1 as ::core::ffi::c_int) as isize)
                as crate::src::common::common::pixel4)
                .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x8c_v_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut v0: crate::src::common::common::pixel4 = (*(src
            .offset(0 as ::core::ffi::c_int as isize)
            .offset(-(32 as ::core::ffi::c_int as isize))
            as *mut crate::src::common::base::x264_union32_t))
            .i
            as crate::src::common::common::pixel4;
        let mut v1: crate::src::common::common::pixel4 = (*(src
            .offset(4 as ::core::ffi::c_int as isize)
            .offset(-(32 as ::core::ffi::c_int as isize))
            as *mut crate::src::common::base::x264_union32_t))
            .i
            as crate::src::common::common::pixel4;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v0 as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v1 as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x8c_p_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut H: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut V: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            H += (i + 1 as ::core::ffi::c_int)
                * (*src.offset(
                    (4 as ::core::ffi::c_int + i - crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    - *src.offset(
                        (2 as ::core::ffi::c_int - i - crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int);
            V += (i + 1 as ::core::ffi::c_int)
                * (*src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + (i + 4 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    - *src.offset(
                        (-(1 as ::core::ffi::c_int)
                            + (2 as ::core::ffi::c_int - i)
                                * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int);
            i += 1;
        }
        let mut a: ::core::ffi::c_int = 16 as ::core::ffi::c_int
            * (*src.offset(
                (-(1 as ::core::ffi::c_int)
                    + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
                + *src.offset(
                    (7 as ::core::ffi::c_int - crate::src::common::common::FDEC_STRIDE) as isize,
                ) as ::core::ffi::c_int);
        let mut b: ::core::ffi::c_int =
            17 as ::core::ffi::c_int * H + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int;
        let mut c: ::core::ffi::c_int =
            17 as ::core::ffi::c_int * V + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int;
        let mut i00: ::core::ffi::c_int =
            a - 3 as ::core::ffi::c_int * b - 3 as ::core::ffi::c_int * c
                + 16 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            let mut pix: ::core::ffi::c_int = i00;
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                *src.offset(x as isize) = x264_clip_pixel(pix >> 5 as ::core::ffi::c_int);
                pix += b;
                x += 1;
            }
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i00 += c;
            y += 1;
        }
    }
}

unsafe extern "C" fn predict_8x16c_dc_128_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
    }
}

unsafe extern "C" fn predict_8x16c_dc_left_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let mut dc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y < 4 as ::core::ffi::c_int {
                dc += *src.offset(
                    (y * crate::src::common::common::FDEC_STRIDE - 1 as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_int;
                y += 1;
            }
            let mut dcsplat: crate::src::common::common::pixel4 = ((dc + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                as crate::src::common::common::pixel4)
                .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
            let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y_0 < 4 as ::core::ffi::c_int {
                (*(src.offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = dcsplat as crate::stdlib::uint32_t;
                (*(src.offset(4 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::base::x264_union32_t))
                    .i = dcsplat as crate::stdlib::uint32_t;
                src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
                y_0 += 1;
            }
            i += 1;
        }
    }
}

unsafe extern "C" fn predict_8x16c_dc_top_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut dc0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut dc1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            dc0 += *src.offset((x - crate::src::common::common::FDEC_STRIDE) as isize)
                as ::core::ffi::c_int;
            dc1 += *src.offset(
                (x + 4 as ::core::ffi::c_int - crate::src::common::common::FDEC_STRIDE) as isize,
            ) as ::core::ffi::c_int;
            x += 1;
        }
        let mut dc0splat: crate::src::common::common::pixel4 = ((dc0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc1splat: crate::src::common::common::pixel4 = ((dc1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc0splat as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc1splat as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x16c_dc_c(
    mut src: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut s0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut s1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut s2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut s3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut s4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut s5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            s0 += *src.offset(
                (i + 0 as ::core::ffi::c_int - crate::src::common::common::FDEC_STRIDE) as isize,
            ) as ::core::ffi::c_int;
            s1 += *src.offset(
                (i + 4 as ::core::ffi::c_int - crate::src::common::common::FDEC_STRIDE) as isize,
            ) as ::core::ffi::c_int;
            s2 += *src.offset(
                (-(1 as ::core::ffi::c_int)
                    + (i + 0 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int;
            s3 += *src.offset(
                (-(1 as ::core::ffi::c_int)
                    + (i + 4 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int;
            s4 += *src.offset(
                (-(1 as ::core::ffi::c_int)
                    + (i + 8 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int;
            s5 += *src.offset(
                (-(1 as ::core::ffi::c_int)
                    + (i + 12 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int;
            i += 1;
        }
        let mut dc0: crate::src::common::common::pixel4 = ((s0 + s2 + 4 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc1: crate::src::common::common::pixel4 = ((s1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc2: crate::src::common::common::pixel4 = ((s3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc3: crate::src::common::common::pixel4 = ((s1 + s3 + 4 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc4: crate::src::common::common::pixel4 = ((s4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc5: crate::src::common::common::pixel4 = ((s1 + s4 + 4 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc6: crate::src::common::common::pixel4 = ((s5 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut dc7: crate::src::common::common::pixel4 = ((s1 + s5 + 4 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 4 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc0 as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc1 as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < 4 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc2 as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc3 as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y_0 += 1;
        }
        let mut y_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_1 < 4 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc4 as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc5 as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y_1 += 1;
        }
        let mut y_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_2 < 4 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc6 as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc7 as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y_2 += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x16c_h_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            let mut v: crate::src::common::common::pixel4 = (*src
                .offset(-(1 as ::core::ffi::c_int) as isize)
                as crate::src::common::common::pixel4)
                .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x16c_v_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut v0: crate::src::common::common::pixel4 = (*(src
            .offset(0 as ::core::ffi::c_int as isize)
            .offset(-(32 as ::core::ffi::c_int as isize))
            as *mut crate::src::common::base::x264_union32_t))
            .i
            as crate::src::common::common::pixel4;
        let mut v1: crate::src::common::common::pixel4 = (*(src
            .offset(4 as ::core::ffi::c_int as isize)
            .offset(-(32 as ::core::ffi::c_int as isize))
            as *mut crate::src::common::base::x264_union32_t))
            .i
            as crate::src::common::common::pixel4;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v0 as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = v1 as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x16c_p_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut H: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut V: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            H += (i + 1 as ::core::ffi::c_int)
                * (*src.offset(
                    (4 as ::core::ffi::c_int + i - crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    - *src.offset(
                        (2 as ::core::ffi::c_int - i - crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int);
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 8 as ::core::ffi::c_int {
            V += (i_0 + 1 as ::core::ffi::c_int)
                * (*src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + (i_0 + 8 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    - *src.offset(
                        (-(1 as ::core::ffi::c_int)
                            + (6 as ::core::ffi::c_int - i_0)
                                * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int);
            i_0 += 1;
        }
        let mut a: ::core::ffi::c_int = 16 as ::core::ffi::c_int
            * (*src.offset(
                (-(1 as ::core::ffi::c_int)
                    + 15 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
                + *src.offset(
                    (7 as ::core::ffi::c_int - crate::src::common::common::FDEC_STRIDE) as isize,
                ) as ::core::ffi::c_int);
        let mut b: ::core::ffi::c_int =
            17 as ::core::ffi::c_int * H + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int;
        let mut c: ::core::ffi::c_int =
            5 as ::core::ffi::c_int * V + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int;
        let mut i00: ::core::ffi::c_int =
            a - 3 as ::core::ffi::c_int * b - 7 as ::core::ffi::c_int * c
                + 16 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            let mut pix: ::core::ffi::c_int = i00;
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                *src.offset(x as isize) = x264_clip_pixel(pix >> 5 as ::core::ffi::c_int);
                pix += b;
                x += 1;
            }
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            i00 += c;
            y += 1;
        }
    }
}

unsafe extern "C" fn predict_4x4_dc_128_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let ref mut c2rust_fresh47 = (*(src.offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh47 = (((1 as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        let ref mut c2rust_fresh48 = (*(src.offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh48 = *c2rust_fresh47;
        let ref mut c2rust_fresh49 = (*(src.offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh49 = *c2rust_fresh48;
        (*(src.offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh49;
    }
}

unsafe extern "C" fn predict_4x4_dc_left_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut dc: crate::src::common::common::pixel4 = ((*src.offset(
            (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_int
            + *src.offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset(
                (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset(
                (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let ref mut c2rust_fresh53 = (*(src.offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh53 = dc as crate::stdlib::uint32_t;
        let ref mut c2rust_fresh54 = (*(src.offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh54 = *c2rust_fresh53;
        let ref mut c2rust_fresh55 = (*(src.offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh55 = *c2rust_fresh54;
        (*(src.offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh55;
    }
}

unsafe extern "C" fn predict_4x4_dc_top_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut dc: crate::src::common::common::pixel4 = ((*src.offset(
            (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_int
            + *src.offset(
                (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset(
                (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset(
                (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let ref mut c2rust_fresh50 = (*(src.offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh50 = dc as crate::stdlib::uint32_t;
        let ref mut c2rust_fresh51 = (*(src.offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh51 = *c2rust_fresh50;
        let ref mut c2rust_fresh52 = (*(src.offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh52 = *c2rust_fresh51;
        (*(src.offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh52;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_4x4_dc_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut dc: crate::src::common::common::pixel4 = ((*src.offset(
            (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_int
            + *src.offset(
                (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset(
                (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset(
                (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset(
                (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset(
                (1 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset(
                (2 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + *src.offset(
                (3 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int
            >> 3 as ::core::ffi::c_int)
            as crate::src::common::common::pixel4)
            .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let ref mut c2rust_fresh8 = (*(src.offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh8 = dc as crate::stdlib::uint32_t;
        let ref mut c2rust_fresh9 = (*(src.offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh9 = *c2rust_fresh8;
        let ref mut c2rust_fresh10 = (*(src.offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh10 = *c2rust_fresh9;
        (*(src.offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh10;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_4x4_h_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        (*(src.offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*src.offset(
            (-(1 as ::core::ffi::c_int) + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        (*(src.offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*src.offset(
            (-(1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        (*(src.offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*src.offset(
            (-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        (*(src.offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*src.offset(
            (-(1 as ::core::ffi::c_int) + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                as isize,
        ) as ::core::ffi::c_uint)
            .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_4x4_v_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let ref mut c2rust_fresh11 = (*(src.offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh11 = (*(src.offset(
            (0 as ::core::ffi::c_int + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        let ref mut c2rust_fresh12 = (*(src.offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh12 = *c2rust_fresh11;
        let ref mut c2rust_fresh13 = (*(src.offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh13 = *c2rust_fresh12;
        (*(src.offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh13;
    }
}

unsafe extern "C" fn predict_4x4_ddl_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut t0: ::core::ffi::c_int = *src.offset(
            (0 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t1: ::core::ffi::c_int = *src.offset(
            (1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t2: ::core::ffi::c_int = *src.offset(
            (2 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t3: ::core::ffi::c_int = *src.offset(
            (3 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t4: ::core::ffi::c_int = *src.offset(
            (4 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t5: ::core::ffi::c_int = *src.offset(
            (5 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t6: ::core::ffi::c_int = *src.offset(
            (6 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t7: ::core::ffi::c_int = *src.offset(
            (7 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        let ref mut c2rust_fresh92 = *src.offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh92 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh92;
        let ref mut c2rust_fresh93 = *src.offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh93 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh94 = *src.offset(
            (1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh94 = *c2rust_fresh93;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh94;
        let ref mut c2rust_fresh95 = *src.offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh95 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh96 = *src.offset(
            (1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh96 = *c2rust_fresh95;
        let ref mut c2rust_fresh97 = *src.offset(
            (2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh97 = *c2rust_fresh96;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh97;
        let ref mut c2rust_fresh98 = *src.offset(
            (1 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh98 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh99 = *src.offset(
            (2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh99 = *c2rust_fresh98;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh99;
        let ref mut c2rust_fresh100 = *src.offset(
            (2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh100 = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh100;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t6 + 2 as ::core::ffi::c_int * t7 + t7 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
    }
}

unsafe extern "C" fn predict_4x4_ddr_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut lt: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l0: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l1: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l2: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l3: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t0: ::core::ffi::c_int = *src.offset(
            (0 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t1: ::core::ffi::c_int = *src.offset(
            (1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t2: ::core::ffi::c_int = *src.offset(
            (2 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t3: ::core::ffi::c_int = *src.offset(
            (3 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t3 + 2 as ::core::ffi::c_int * t2 + t1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        let ref mut c2rust_fresh83 = *src.offset(
            (3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh83 = (t2 + 2 as ::core::ffi::c_int * t1 + t0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh83;
        let ref mut c2rust_fresh84 = *src.offset(
            (3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh84 = (t1 + 2 as ::core::ffi::c_int * t0 + lt + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh85 = *src.offset(
            (2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh85 = *c2rust_fresh84;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh85;
        let ref mut c2rust_fresh86 = *src.offset(
            (3 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh86 = (t0 + 2 as ::core::ffi::c_int * lt + l0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh87 = *src.offset(
            (2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh87 = *c2rust_fresh86;
        let ref mut c2rust_fresh88 = *src.offset(
            (1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh88 = *c2rust_fresh87;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh88;
        let ref mut c2rust_fresh89 = *src.offset(
            (2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh89 = (lt + 2 as ::core::ffi::c_int * l0 + l1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh90 = *src.offset(
            (1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh90 = *c2rust_fresh89;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh90;
        let ref mut c2rust_fresh91 = *src.offset(
            (1 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh91 = (l0 + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh91;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
    }
}

unsafe extern "C" fn predict_4x4_vr_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut lt: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l0: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l1: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l2: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut _l3: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t0: ::core::ffi::c_int = *src.offset(
            (0 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t1: ::core::ffi::c_int = *src.offset(
            (1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t2: ::core::ffi::c_int = *src.offset(
            (2 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t3: ::core::ffi::c_int = *src.offset(
            (3 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (l2 + 2 as ::core::ffi::c_int * l1 + l0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (l1 + 2 as ::core::ffi::c_int * l0 + lt + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        let ref mut c2rust_fresh77 = *src.offset(
            (1 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh77 = (l0 + 2 as ::core::ffi::c_int * lt + t0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh77;
        let ref mut c2rust_fresh78 = *src.offset(
            (1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh78 = (lt + t0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh78;
        let ref mut c2rust_fresh79 = *src.offset(
            (2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh79 = (lt + 2 as ::core::ffi::c_int * t0 + t1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh79;
        let ref mut c2rust_fresh80 = *src.offset(
            (2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh80 = (t0 + t1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh80;
        let ref mut c2rust_fresh81 = *src.offset(
            (3 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh81 = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh81;
        let ref mut c2rust_fresh82 = *src.offset(
            (3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh82 = (t1 + t2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh82;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t2 + t3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
    }
}

unsafe extern "C" fn predict_4x4_hd_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut lt: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l0: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l1: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l2: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l3: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t0: ::core::ffi::c_int = *src.offset(
            (0 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t1: ::core::ffi::c_int = *src.offset(
            (1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t2: ::core::ffi::c_int = *src.offset(
            (2 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut _t3: ::core::ffi::c_int = *src.offset(
            (3 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (l2 + l3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        let ref mut c2rust_fresh71 = *src.offset(
            (2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh71 = (l1 + l2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh71;
        let ref mut c2rust_fresh72 = *src.offset(
            (3 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh72 = (l0 + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh72;
        let ref mut c2rust_fresh73 = *src.offset(
            (2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh73 = (l0 + l1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh73;
        let ref mut c2rust_fresh74 = *src.offset(
            (3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh74 = (lt + 2 as ::core::ffi::c_int * l0 + l1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh74;
        let ref mut c2rust_fresh75 = *src.offset(
            (2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh75 = (lt + l0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh75;
        let ref mut c2rust_fresh76 = *src.offset(
            (3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh76 = (t0 + 2 as ::core::ffi::c_int * lt + l0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh76;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t1 + 2 as ::core::ffi::c_int * t0 + lt + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t2 + 2 as ::core::ffi::c_int * t1 + t0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
    }
}

unsafe extern "C" fn predict_4x4_vl_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut t0: ::core::ffi::c_int = *src.offset(
            (0 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t1: ::core::ffi::c_int = *src.offset(
            (1 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t2: ::core::ffi::c_int = *src.offset(
            (2 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t3: ::core::ffi::c_int = *src.offset(
            (3 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t4: ::core::ffi::c_int = *src.offset(
            (4 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t5: ::core::ffi::c_int = *src.offset(
            (5 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut t6: ::core::ffi::c_int = *src.offset(
            (6 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut _t7: ::core::ffi::c_int = *src.offset(
            (7 as ::core::ffi::c_int
                + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t0 + t1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        let ref mut c2rust_fresh65 = *src.offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh65 = (t1 + t2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh65;
        let ref mut c2rust_fresh66 = *src.offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh66 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh66;
        let ref mut c2rust_fresh67 = *src.offset(
            (1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh67 = (t2 + t3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh67;
        let ref mut c2rust_fresh68 = *src.offset(
            (1 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh68 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh68;
        let ref mut c2rust_fresh69 = *src.offset(
            (2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh69 = (t3 + t4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh69;
        let ref mut c2rust_fresh70 = *src.offset(
            (2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh70 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh70;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t4 + t5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
    }
}

unsafe extern "C" fn predict_4x4_hu_c(mut src: *mut crate::src::common::common::pixel) {
    unsafe {
        let mut l0: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l1: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l2: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        let mut l3: ::core::ffi::c_int = *src.offset(
            (-(1 as ::core::ffi::c_int)
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) as ::core::ffi::c_int;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (l0 + l1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (l0 + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        let ref mut c2rust_fresh56 = *src.offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh56 = (l1 + l2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh56;
        let ref mut c2rust_fresh57 = *src.offset(
            (1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh57 = (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh57;
        let ref mut c2rust_fresh58 = *src.offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh58 = (l2 + l3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh58;
        let ref mut c2rust_fresh59 = *src.offset(
            (1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh59 = (l2 + 2 as ::core::ffi::c_int * l3 + l3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh59;
        let ref mut c2rust_fresh60 = *src.offset(
            (3 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh60 = l3 as crate::src::common::common::pixel;
        let ref mut c2rust_fresh61 = *src.offset(
            (2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh61 = *c2rust_fresh60;
        let ref mut c2rust_fresh62 = *src.offset(
            (2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh62 = *c2rust_fresh61;
        let ref mut c2rust_fresh63 = *src.offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh63 = *c2rust_fresh62;
        let ref mut c2rust_fresh64 = *src.offset(
            (1 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh64 = *c2rust_fresh63;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh64;
    }
}

unsafe extern "C" fn predict_8x8_filter_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
    mut i_neighbor: ::core::ffi::c_int,
    mut i_filters: ::core::ffi::c_int,
) {
    unsafe {
        let mut have_lt: ::core::ffi::c_int =
            i_neighbor & crate::src::common::macroblock::MB_TOPLEFT as ::core::ffi::c_int;
        if i_filters & crate::src::common::macroblock::MB_LEFT as ::core::ffi::c_int != 0 {
            *edge.offset(15 as ::core::ffi::c_int as isize) = (*src.offset(
                (0 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *src.offset(
                        (-(1 as ::core::ffi::c_int)
                            + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int
                + *src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                as crate::src::common::common::pixel;
            *edge.offset(14 as ::core::ffi::c_int as isize) = ((if have_lt != 0 {
                *src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
            } else {
                *src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
            }) + 2 as ::core::ffi::c_int
                * *src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                + *src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                as crate::src::common::common::pixel;
            *edge.offset((14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (-(1 as ::core::ffi::c_int)
                                + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (-(1 as ::core::ffi::c_int)
                            + (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                * 32 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            *edge.offset((14 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (-(1 as ::core::ffi::c_int)
                                + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (-(1 as ::core::ffi::c_int)
                            + (2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                * 32 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            *edge.offset((14 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (-(1 as ::core::ffi::c_int)
                                + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (-(1 as ::core::ffi::c_int)
                            + (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                * 32 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            *edge.offset((14 as ::core::ffi::c_int - 4 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (-(1 as ::core::ffi::c_int)
                                + 4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (-(1 as ::core::ffi::c_int)
                            + (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                * 32 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            *edge.offset((14 as ::core::ffi::c_int - 5 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (-(1 as ::core::ffi::c_int)
                                + 5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (-(1 as ::core::ffi::c_int)
                            + (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                * 32 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            *edge.offset((14 as ::core::ffi::c_int - 6 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            * 32 as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (-(1 as ::core::ffi::c_int)
                                + 6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (-(1 as ::core::ffi::c_int)
                            + (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                                * 32 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            let ref mut c2rust_fresh113 = *edge.offset(7 as ::core::ffi::c_int as isize);
            *c2rust_fresh113 = (*src.offset(
                (-(1 as ::core::ffi::c_int)
                    + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int
                    * *src.offset(
                        (-(1 as ::core::ffi::c_int)
                            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                as crate::src::common::common::pixel;
            *edge.offset(6 as ::core::ffi::c_int as isize) = *c2rust_fresh113;
        }
        if i_filters & crate::src::common::macroblock::MB_TOP as ::core::ffi::c_int != 0 {
            let mut have_tr: ::core::ffi::c_int =
                i_neighbor & crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int;
            *edge.offset(16 as ::core::ffi::c_int as isize) = ((if have_lt != 0 {
                *src.offset(
                    (-(1 as ::core::ffi::c_int)
                        + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
            } else {
                *src.offset(
                    (0 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
            }) + 2 as ::core::ffi::c_int
                * *src.offset(
                    (0 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                + *src.offset(
                    (1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                as crate::src::common::common::pixel;
            *edge.offset((16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (1 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (1 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            *edge.offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (2 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            *edge.offset((16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (3 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (3 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            *edge.offset((16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (4 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (4 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (4 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            *edge.offset((16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (5 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (5 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (5 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            *edge.offset((16 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize) =
                (*src.offset(
                    (6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                        * *src.offset(
                            (6 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    + *src.offset(
                        (6 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int)
                    as crate::src::common::common::pixel;
            *edge.offset(23 as ::core::ffi::c_int as isize) = (*src.offset(
                (6 as ::core::ffi::c_int
                    + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                    as isize,
            ) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                    * *src.offset(
                        (7 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int
                + (if have_tr != 0 {
                    *src.offset(
                        (8 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int
                } else {
                    *src.offset(
                        (7 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int
                })
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int)
                as crate::src::common::common::pixel;
            if i_filters & crate::src::common::macroblock::MB_TOPRIGHT as ::core::ffi::c_int != 0 {
                if have_tr != 0 {
                    *edge.offset((16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize) =
                        (*src.offset(
                            (8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * *src.offset(
                                    (8 as ::core::ffi::c_int
                                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                        as isize,
                                ) as ::core::ffi::c_int
                            + *src.offset(
                                (8 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                            as crate::src::common::common::pixel;
                    *edge.offset((16 as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as isize) =
                        (*src.offset(
                            (9 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * *src.offset(
                                    (9 as ::core::ffi::c_int
                                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                        as isize,
                                ) as ::core::ffi::c_int
                            + *src.offset(
                                (9 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                            as crate::src::common::common::pixel;
                    *edge.offset((16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize) =
                        (*src.offset(
                            (10 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * *src.offset(
                                    (10 as ::core::ffi::c_int
                                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                        as isize,
                                ) as ::core::ffi::c_int
                            + *src.offset(
                                (10 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                            as crate::src::common::common::pixel;
                    *edge.offset((16 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as isize) =
                        (*src.offset(
                            (11 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * *src.offset(
                                    (11 as ::core::ffi::c_int
                                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                        as isize,
                                ) as ::core::ffi::c_int
                            + *src.offset(
                                (11 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                            as crate::src::common::common::pixel;
                    *edge.offset((16 as ::core::ffi::c_int + 12 as ::core::ffi::c_int) as isize) =
                        (*src.offset(
                            (12 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * *src.offset(
                                    (12 as ::core::ffi::c_int
                                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                        as isize,
                                ) as ::core::ffi::c_int
                            + *src.offset(
                                (12 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                            as crate::src::common::common::pixel;
                    *edge.offset((16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize) =
                        (*src.offset(
                            (13 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * *src.offset(
                                    (13 as ::core::ffi::c_int
                                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                        as isize,
                                ) as ::core::ffi::c_int
                            + *src.offset(
                                (13 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                            as crate::src::common::common::pixel;
                    *edge.offset((16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize) =
                        (*src.offset(
                            (14 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                                * *src.offset(
                                    (14 as ::core::ffi::c_int
                                        + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                        as isize,
                                ) as ::core::ffi::c_int
                            + *src.offset(
                                (14 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            >> 2 as ::core::ffi::c_int)
                            as crate::src::common::common::pixel;
                    let ref mut c2rust_fresh114 = *edge.offset(32 as ::core::ffi::c_int as isize);
                    *c2rust_fresh114 = (*src.offset(
                        (14 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    ) as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                            * *src.offset(
                                (15 as ::core::ffi::c_int
                                    + -(1 as ::core::ffi::c_int)
                                        * crate::src::common::common::FDEC_STRIDE)
                                    as isize,
                            ) as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        >> 2 as ::core::ffi::c_int)
                        as crate::src::common::common::pixel;
                    *edge.offset(31 as ::core::ffi::c_int as isize) = *c2rust_fresh114;
                } else {
                    (*(edge.offset(24 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*src.offset(
                        (7 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                        as crate::stdlib::uint32_t;
                    (*(edge.offset(28 as ::core::ffi::c_int as isize)
                        as *mut crate::src::common::base::x264_union32_t))
                        .i = (*src.offset(
                        (7 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * 32 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                        as crate::stdlib::uint32_t;
                    *edge.offset(32 as ::core::ffi::c_int as isize) = *src.offset(
                        (7 as ::core::ffi::c_int
                            + -(1 as ::core::ffi::c_int) * crate::src::common::common::FDEC_STRIDE)
                            as isize,
                    );
                }
            }
        }
    }
}

unsafe extern "C" fn predict_8x8_dc_128_c(
    mut src: *mut crate::src::common::common::pixel,
    mut _edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = (((1 as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
    }
}

unsafe extern "C" fn predict_8x8_dc_left_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
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
        let mut dc: crate::src::common::common::pixel4 =
            ((l0 + l1 + l2 + l3 + l4 + l5 + l6 + l7 + 4 as ::core::ffi::c_int
                >> 3 as ::core::ffi::c_int) as crate::src::common::common::pixel4)
                .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
    }
}

unsafe extern "C" fn predict_8x8_dc_top_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
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
        let mut dc: crate::src::common::common::pixel4 =
            ((t0 + t1 + t2 + t3 + t4 + t5 + t6 + t7 + 4 as ::core::ffi::c_int
                >> 3 as ::core::ffi::c_int) as crate::src::common::common::pixel4)
                .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x8_dc_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
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
        let mut dc: crate::src::common::common::pixel4 =
            ((l0 + l1
                + l2
                + l3
                + l4
                + l5
                + l6
                + l7
                + t0
                + t1
                + t2
                + t3
                + t4
                + t5
                + t6
                + t7
                + 8 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int) as crate::src::common::common::pixel4)
                .wrapping_mul(0x1010101 as crate::src::common::common::pixel4);
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            (*(src.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc as crate::stdlib::uint32_t;
            (*(src.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = dc as crate::stdlib::uint32_t;
            src = src.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x8_h_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
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
        let ref mut c2rust_fresh0 = (*(src
            .offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh0 = (l0 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        (*(src
            .offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh0;
        let ref mut c2rust_fresh1 = (*(src
            .offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh1 = (l1 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        (*(src
            .offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh1;
        let ref mut c2rust_fresh2 = (*(src
            .offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh2 = (l2 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        (*(src
            .offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh2;
        let ref mut c2rust_fresh3 = (*(src
            .offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh3 = (l3 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        (*(src
            .offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh3;
        let ref mut c2rust_fresh4 = (*(src
            .offset((4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh4 = (l4 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        (*(src
            .offset((4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh4;
        let ref mut c2rust_fresh5 = (*(src
            .offset((5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh5 = (l5 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        (*(src
            .offset((5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh5;
        let ref mut c2rust_fresh6 = (*(src
            .offset((6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh6 = (l6 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        (*(src
            .offset((6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh6;
        let ref mut c2rust_fresh7 = (*(src
            .offset((7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh7 = (l7 as ::core::ffi::c_uint).wrapping_mul(0x1010101 as ::core::ffi::c_uint)
            as crate::stdlib::uint32_t;
        (*(src
            .offset((7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh7;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x8_v_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut top: [crate::src::common::common::pixel4; 2] = [
            (*(edge.offset(16 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i,
            (*(edge.offset(20 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i,
        ];
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            (*(src
                .offset((y * 32 as ::core::ffi::c_int) as isize)
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = top[0 as ::core::ffi::c_int as usize] as crate::stdlib::uint32_t;
            (*(src
                .offset((y * 32 as ::core::ffi::c_int) as isize)
                .offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::base::x264_union32_t))
                .i = top[1 as ::core::ffi::c_int as usize] as crate::stdlib::uint32_t;
            y += 1;
        }
    }
}

unsafe extern "C" fn predict_8x8_ddl_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
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
        *src.offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        let ref mut c2rust_fresh262 = *src.offset(
            (1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh262 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh262;
        let ref mut c2rust_fresh263 = *src.offset(
            (2 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh263 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh264 = *src.offset(
            (1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh264 = *c2rust_fresh263;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh264;
        let ref mut c2rust_fresh265 = *src.offset(
            (3 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh265 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh266 = *src.offset(
            (2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh266 = *c2rust_fresh265;
        let ref mut c2rust_fresh267 = *src.offset(
            (1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh267 = *c2rust_fresh266;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh267;
        let ref mut c2rust_fresh268 = *src.offset(
            (4 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh268 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh269 = *src.offset(
            (3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh269 = *c2rust_fresh268;
        let ref mut c2rust_fresh270 = *src.offset(
            (2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh270 = *c2rust_fresh269;
        let ref mut c2rust_fresh271 = *src.offset(
            (1 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh271 = *c2rust_fresh270;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh271;
        let ref mut c2rust_fresh272 = *src.offset(
            (5 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh272 = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh273 = *src.offset(
            (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh273 = *c2rust_fresh272;
        let ref mut c2rust_fresh274 = *src.offset(
            (3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh274 = *c2rust_fresh273;
        let ref mut c2rust_fresh275 = *src.offset(
            (2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh275 = *c2rust_fresh274;
        let ref mut c2rust_fresh276 = *src.offset(
            (1 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh276 = *c2rust_fresh275;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh276;
        let ref mut c2rust_fresh277 = *src.offset(
            (6 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh277 = (t6 + 2 as ::core::ffi::c_int * t7 + t8 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh278 = *src.offset(
            (5 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh278 = *c2rust_fresh277;
        let ref mut c2rust_fresh279 = *src.offset(
            (4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh279 = *c2rust_fresh278;
        let ref mut c2rust_fresh280 = *src.offset(
            (3 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh280 = *c2rust_fresh279;
        let ref mut c2rust_fresh281 = *src.offset(
            (2 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh281 = *c2rust_fresh280;
        let ref mut c2rust_fresh282 = *src.offset(
            (1 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh282 = *c2rust_fresh281;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh282;
        let ref mut c2rust_fresh283 = *src.offset(
            (7 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh283 = (t7 + 2 as ::core::ffi::c_int * t8 + t9 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh284 = *src.offset(
            (6 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh284 = *c2rust_fresh283;
        let ref mut c2rust_fresh285 = *src.offset(
            (5 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh285 = *c2rust_fresh284;
        let ref mut c2rust_fresh286 = *src.offset(
            (4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh286 = *c2rust_fresh285;
        let ref mut c2rust_fresh287 = *src.offset(
            (3 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh287 = *c2rust_fresh286;
        let ref mut c2rust_fresh288 = *src.offset(
            (2 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh288 = *c2rust_fresh287;
        let ref mut c2rust_fresh289 = *src.offset(
            (1 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh289 = *c2rust_fresh288;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh289;
        let ref mut c2rust_fresh290 = *src.offset(
            (7 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh290 = (t8 + 2 as ::core::ffi::c_int * t9 + t10 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh291 = *src.offset(
            (6 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh291 = *c2rust_fresh290;
        let ref mut c2rust_fresh292 = *src.offset(
            (5 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh292 = *c2rust_fresh291;
        let ref mut c2rust_fresh293 = *src.offset(
            (4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh293 = *c2rust_fresh292;
        let ref mut c2rust_fresh294 = *src.offset(
            (3 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh294 = *c2rust_fresh293;
        let ref mut c2rust_fresh295 = *src.offset(
            (2 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh295 = *c2rust_fresh294;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh295;
        let ref mut c2rust_fresh296 = *src.offset(
            (7 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh296 = (t9 + 2 as ::core::ffi::c_int * t10 + t11 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh297 = *src.offset(
            (6 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh297 = *c2rust_fresh296;
        let ref mut c2rust_fresh298 = *src.offset(
            (5 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh298 = *c2rust_fresh297;
        let ref mut c2rust_fresh299 = *src.offset(
            (4 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh299 = *c2rust_fresh298;
        let ref mut c2rust_fresh300 = *src.offset(
            (3 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh300 = *c2rust_fresh299;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh300;
        let ref mut c2rust_fresh301 = *src.offset(
            (7 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh301 = (t10 + 2 as ::core::ffi::c_int * t11 + t12 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh302 = *src.offset(
            (6 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh302 = *c2rust_fresh301;
        let ref mut c2rust_fresh303 = *src.offset(
            (5 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh303 = *c2rust_fresh302;
        let ref mut c2rust_fresh304 = *src.offset(
            (4 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh304 = *c2rust_fresh303;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh304;
        let ref mut c2rust_fresh305 = *src.offset(
            (7 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh305 = (t11 + 2 as ::core::ffi::c_int * t12 + t13 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh306 = *src.offset(
            (6 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh306 = *c2rust_fresh305;
        let ref mut c2rust_fresh307 = *src.offset(
            (5 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh307 = *c2rust_fresh306;
        *src.offset(
            (4 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh307;
        let ref mut c2rust_fresh308 = *src.offset(
            (7 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh308 = (t12 + 2 as ::core::ffi::c_int * t13 + t14 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh309 = *src.offset(
            (6 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh309 = *c2rust_fresh308;
        *src.offset(
            (5 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh309;
        let ref mut c2rust_fresh310 = *src.offset(
            (7 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh310 = (t13 + 2 as ::core::ffi::c_int * t14 + t15 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (6 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh310;
        *src.offset(
            (7 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t14 + 2 as ::core::ffi::c_int * t15 + t15 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
    }
}

unsafe extern "C" fn predict_8x8_ddr_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
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
        let mut lt: ::core::ffi::c_int =
            *edge.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (l7 + 2 as ::core::ffi::c_int * l6 + l5 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        let ref mut c2rust_fresh213 = *src.offset(
            (1 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh213 = (l6 + 2 as ::core::ffi::c_int * l5 + l4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh213;
        let ref mut c2rust_fresh214 = *src.offset(
            (2 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh214 = (l5 + 2 as ::core::ffi::c_int * l4 + l3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh215 = *src.offset(
            (1 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh215 = *c2rust_fresh214;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh215;
        let ref mut c2rust_fresh216 = *src.offset(
            (3 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh216 = (l4 + 2 as ::core::ffi::c_int * l3 + l2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh217 = *src.offset(
            (2 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh217 = *c2rust_fresh216;
        let ref mut c2rust_fresh218 = *src.offset(
            (1 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh218 = *c2rust_fresh217;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh218;
        let ref mut c2rust_fresh219 = *src.offset(
            (4 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh219 = (l3 + 2 as ::core::ffi::c_int * l2 + l1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh220 = *src.offset(
            (3 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh220 = *c2rust_fresh219;
        let ref mut c2rust_fresh221 = *src.offset(
            (2 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh221 = *c2rust_fresh220;
        let ref mut c2rust_fresh222 = *src.offset(
            (1 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh222 = *c2rust_fresh221;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh222;
        let ref mut c2rust_fresh223 = *src.offset(
            (5 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh223 = (l2 + 2 as ::core::ffi::c_int * l1 + l0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh224 = *src.offset(
            (4 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh224 = *c2rust_fresh223;
        let ref mut c2rust_fresh225 = *src.offset(
            (3 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh225 = *c2rust_fresh224;
        let ref mut c2rust_fresh226 = *src.offset(
            (2 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh226 = *c2rust_fresh225;
        let ref mut c2rust_fresh227 = *src.offset(
            (1 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh227 = *c2rust_fresh226;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh227;
        let ref mut c2rust_fresh228 = *src.offset(
            (6 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh228 = (l1 + 2 as ::core::ffi::c_int * l0 + lt + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh229 = *src.offset(
            (5 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh229 = *c2rust_fresh228;
        let ref mut c2rust_fresh230 = *src.offset(
            (4 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh230 = *c2rust_fresh229;
        let ref mut c2rust_fresh231 = *src.offset(
            (3 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh231 = *c2rust_fresh230;
        let ref mut c2rust_fresh232 = *src.offset(
            (2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh232 = *c2rust_fresh231;
        let ref mut c2rust_fresh233 = *src.offset(
            (1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh233 = *c2rust_fresh232;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh233;
        let ref mut c2rust_fresh234 = *src.offset(
            (7 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh234 = (l0 + 2 as ::core::ffi::c_int * lt + t0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh235 = *src.offset(
            (6 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh235 = *c2rust_fresh234;
        let ref mut c2rust_fresh236 = *src.offset(
            (5 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh236 = *c2rust_fresh235;
        let ref mut c2rust_fresh237 = *src.offset(
            (4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh237 = *c2rust_fresh236;
        let ref mut c2rust_fresh238 = *src.offset(
            (3 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh238 = *c2rust_fresh237;
        let ref mut c2rust_fresh239 = *src.offset(
            (2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh239 = *c2rust_fresh238;
        let ref mut c2rust_fresh240 = *src.offset(
            (1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh240 = *c2rust_fresh239;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh240;
        let ref mut c2rust_fresh241 = *src.offset(
            (7 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh241 = (lt + 2 as ::core::ffi::c_int * t0 + t1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh242 = *src.offset(
            (6 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh242 = *c2rust_fresh241;
        let ref mut c2rust_fresh243 = *src.offset(
            (5 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh243 = *c2rust_fresh242;
        let ref mut c2rust_fresh244 = *src.offset(
            (4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh244 = *c2rust_fresh243;
        let ref mut c2rust_fresh245 = *src.offset(
            (3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh245 = *c2rust_fresh244;
        let ref mut c2rust_fresh246 = *src.offset(
            (2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh246 = *c2rust_fresh245;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh246;
        let ref mut c2rust_fresh247 = *src.offset(
            (7 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh247 = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh248 = *src.offset(
            (6 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh248 = *c2rust_fresh247;
        let ref mut c2rust_fresh249 = *src.offset(
            (5 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh249 = *c2rust_fresh248;
        let ref mut c2rust_fresh250 = *src.offset(
            (4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh250 = *c2rust_fresh249;
        let ref mut c2rust_fresh251 = *src.offset(
            (3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh251 = *c2rust_fresh250;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh251;
        let ref mut c2rust_fresh252 = *src.offset(
            (7 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh252 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh253 = *src.offset(
            (6 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh253 = *c2rust_fresh252;
        let ref mut c2rust_fresh254 = *src.offset(
            (5 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh254 = *c2rust_fresh253;
        let ref mut c2rust_fresh255 = *src.offset(
            (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh255 = *c2rust_fresh254;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh255;
        let ref mut c2rust_fresh256 = *src.offset(
            (7 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh256 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh257 = *src.offset(
            (6 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh257 = *c2rust_fresh256;
        let ref mut c2rust_fresh258 = *src.offset(
            (5 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh258 = *c2rust_fresh257;
        *src.offset(
            (4 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh258;
        let ref mut c2rust_fresh259 = *src.offset(
            (7 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh259 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh260 = *src.offset(
            (6 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh260 = *c2rust_fresh259;
        *src.offset(
            (5 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh260;
        let ref mut c2rust_fresh261 = *src.offset(
            (7 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh261 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (6 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh261;
        *src.offset(
            (7 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
    }
}

unsafe extern "C" fn predict_8x8_vr_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
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
        let mut _l7: ::core::ffi::c_int = *edge
            .offset((14 as ::core::ffi::c_int - 7 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        let mut lt: ::core::ffi::c_int =
            *edge.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (l5 + 2 as ::core::ffi::c_int * l4 + l3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (l6 + 2 as ::core::ffi::c_int * l5 + l4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        let ref mut c2rust_fresh171 = *src.offset(
            (1 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh171 = (l3 + 2 as ::core::ffi::c_int * l2 + l1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh171;
        let ref mut c2rust_fresh172 = *src.offset(
            (1 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh172 = (l4 + 2 as ::core::ffi::c_int * l3 + l2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh172;
        let ref mut c2rust_fresh173 = *src.offset(
            (2 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh173 = (l1 + 2 as ::core::ffi::c_int * l0 + lt + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh174 = *src.offset(
            (1 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh174 = *c2rust_fresh173;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh174;
        let ref mut c2rust_fresh175 = *src.offset(
            (2 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh175 = (l2 + 2 as ::core::ffi::c_int * l1 + l0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh176 = *src.offset(
            (1 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh176 = *c2rust_fresh175;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh176;
        let ref mut c2rust_fresh177 = *src.offset(
            (3 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh177 = (l0 + 2 as ::core::ffi::c_int * lt + t0 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh178 = *src.offset(
            (2 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh178 = *c2rust_fresh177;
        let ref mut c2rust_fresh179 = *src.offset(
            (1 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh179 = *c2rust_fresh178;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh179;
        let ref mut c2rust_fresh180 = *src.offset(
            (3 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh180 = (lt + t0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh181 = *src.offset(
            (2 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh181 = *c2rust_fresh180;
        let ref mut c2rust_fresh182 = *src.offset(
            (1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh182 = *c2rust_fresh181;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh182;
        let ref mut c2rust_fresh183 = *src.offset(
            (4 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh183 = (lt + 2 as ::core::ffi::c_int * t0 + t1 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh184 = *src.offset(
            (3 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh184 = *c2rust_fresh183;
        let ref mut c2rust_fresh185 = *src.offset(
            (2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh185 = *c2rust_fresh184;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh185;
        let ref mut c2rust_fresh186 = *src.offset(
            (4 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh186 = (t0 + t1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh187 = *src.offset(
            (3 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh187 = *c2rust_fresh186;
        let ref mut c2rust_fresh188 = *src.offset(
            (2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh188 = *c2rust_fresh187;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh188;
        let ref mut c2rust_fresh189 = *src.offset(
            (5 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh189 = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh190 = *src.offset(
            (4 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh190 = *c2rust_fresh189;
        let ref mut c2rust_fresh191 = *src.offset(
            (3 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh191 = *c2rust_fresh190;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh191;
        let ref mut c2rust_fresh192 = *src.offset(
            (5 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh192 = (t1 + t2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh193 = *src.offset(
            (4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh193 = *c2rust_fresh192;
        let ref mut c2rust_fresh194 = *src.offset(
            (3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh194 = *c2rust_fresh193;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh194;
        let ref mut c2rust_fresh195 = *src.offset(
            (6 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh195 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh196 = *src.offset(
            (5 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh196 = *c2rust_fresh195;
        let ref mut c2rust_fresh197 = *src.offset(
            (4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh197 = *c2rust_fresh196;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh197;
        let ref mut c2rust_fresh198 = *src.offset(
            (6 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh198 = (t2 + t3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh199 = *src.offset(
            (5 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh199 = *c2rust_fresh198;
        let ref mut c2rust_fresh200 = *src.offset(
            (4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh200 = *c2rust_fresh199;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh200;
        let ref mut c2rust_fresh201 = *src.offset(
            (7 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh201 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh202 = *src.offset(
            (6 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh202 = *c2rust_fresh201;
        let ref mut c2rust_fresh203 = *src.offset(
            (5 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh203 = *c2rust_fresh202;
        *src.offset(
            (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh203;
        let ref mut c2rust_fresh204 = *src.offset(
            (7 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh204 = (t3 + t4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh205 = *src.offset(
            (6 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh205 = *c2rust_fresh204;
        let ref mut c2rust_fresh206 = *src.offset(
            (5 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh206 = *c2rust_fresh205;
        *src.offset(
            (4 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh206;
        let ref mut c2rust_fresh207 = *src.offset(
            (7 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh207 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh208 = *src.offset(
            (6 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh208 = *c2rust_fresh207;
        *src.offset(
            (5 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh208;
        let ref mut c2rust_fresh209 = *src.offset(
            (7 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh209 = (t4 + t5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh210 = *src.offset(
            (6 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh210 = *c2rust_fresh209;
        *src.offset(
            (5 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh210;
        let ref mut c2rust_fresh211 = *src.offset(
            (7 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh211 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (6 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh211;
        let ref mut c2rust_fresh212 = *src.offset(
            (7 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh212 = (t5 + t6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (6 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh212;
        *src.offset(
            (7 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        *src.offset(
            (7 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t6 + t7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
    }
}

unsafe extern "C" fn predict_8x8_hd_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
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
        let mut _t7: ::core::ffi::c_int = *edge
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
        let mut lt: ::core::ffi::c_int =
            *edge.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut p1: ::core::ffi::c_int = pack8to16(
            (l6 + l7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l5 + 2 as ::core::ffi::c_int * l6 + l7 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p2: ::core::ffi::c_int = pack8to16(
            (l5 + l6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l4 + 2 as ::core::ffi::c_int * l5 + l6 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p3: ::core::ffi::c_int = pack8to16(
            (l4 + l5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l3 + 2 as ::core::ffi::c_int * l4 + l5 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p4: ::core::ffi::c_int = pack8to16(
            (l3 + l4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l2 + 2 as ::core::ffi::c_int * l3 + l4 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p5: ::core::ffi::c_int = pack8to16(
            (l2 + l3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p6: ::core::ffi::c_int = pack8to16(
            (l1 + l2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l0 + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p7: ::core::ffi::c_int = pack8to16(
            (l0 + l1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (lt + 2 as ::core::ffi::c_int * l0 + l1 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p8: ::core::ffi::c_int = pack8to16(
            (lt + l0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l0 + 2 as ::core::ffi::c_int * lt + t0 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p9: ::core::ffi::c_int = pack8to16(
            (t1 + 2 as ::core::ffi::c_int * t0 + lt + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
            (t2 + 2 as ::core::ffi::c_int * t1 + t0 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p10: ::core::ffi::c_int = pack8to16(
            (t3 + 2 as ::core::ffi::c_int * t2 + t1 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
            (t4 + 2 as ::core::ffi::c_int * t3 + t2 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p11: ::core::ffi::c_int = pack8to16(
            (t5 + 2 as ::core::ffi::c_int * t4 + t3 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
            (t6 + 2 as ::core::ffi::c_int * t5 + t4 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        (*(src.offset(
            (0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = pack16to32(p1 as crate::stdlib::uint32_t, p2 as crate::stdlib::uint32_t);
        (*(src.offset(
            (0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = pack16to32(p2 as crate::stdlib::uint32_t, p3 as crate::stdlib::uint32_t);
        let ref mut c2rust_fresh165 = (*(src.offset(
            (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh165 = pack16to32(p3 as crate::stdlib::uint32_t, p4 as crate::stdlib::uint32_t);
        (*(src.offset(
            (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh165;
        let ref mut c2rust_fresh166 = (*(src.offset(
            (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh166 = pack16to32(p4 as crate::stdlib::uint32_t, p5 as crate::stdlib::uint32_t);
        (*(src.offset(
            (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh166;
        let ref mut c2rust_fresh167 = (*(src.offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh167 = pack16to32(p5 as crate::stdlib::uint32_t, p6 as crate::stdlib::uint32_t);
        (*(src.offset(
            (4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh167;
        let ref mut c2rust_fresh168 = (*(src.offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh168 = pack16to32(p6 as crate::stdlib::uint32_t, p7 as crate::stdlib::uint32_t);
        (*(src.offset(
            (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh168;
        let ref mut c2rust_fresh169 = (*(src.offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh169 = pack16to32(p7 as crate::stdlib::uint32_t, p8 as crate::stdlib::uint32_t);
        (*(src.offset(
            (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh169;
        let ref mut c2rust_fresh170 = (*(src.offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh170 = pack16to32(p8 as crate::stdlib::uint32_t, p9 as crate::stdlib::uint32_t);
        (*(src.offset(
            (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh170;
        (*(src.offset(
            (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = pack16to32(
            p9 as crate::stdlib::uint32_t,
            p10 as crate::stdlib::uint32_t,
        );
        (*(src.offset(
            (4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = pack16to32(
            p10 as crate::stdlib::uint32_t,
            p11 as crate::stdlib::uint32_t,
        );
    }
}

unsafe extern "C" fn predict_8x8_vl_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
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
        let mut _t13: ::core::ffi::c_int = *edge
            .offset((16 as ::core::ffi::c_int + 13 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        let mut _t14: ::core::ffi::c_int = *edge
            .offset((16 as ::core::ffi::c_int + 14 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        let mut _t15: ::core::ffi::c_int = *edge
            .offset((16 as ::core::ffi::c_int + 15 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t0 + t1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t0 + 2 as ::core::ffi::c_int * t1 + t2 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
        let ref mut c2rust_fresh123 = *src.offset(
            (1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh123 = (t1 + t2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh123;
        let ref mut c2rust_fresh124 = *src.offset(
            (1 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh124 = (t1 + 2 as ::core::ffi::c_int * t2 + t3 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh124;
        let ref mut c2rust_fresh125 = *src.offset(
            (2 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh125 = (t2 + t3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh126 = *src.offset(
            (1 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh126 = *c2rust_fresh125;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh126;
        let ref mut c2rust_fresh127 = *src.offset(
            (2 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh127 = (t2 + 2 as ::core::ffi::c_int * t3 + t4 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh128 = *src.offset(
            (1 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh128 = *c2rust_fresh127;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh128;
        let ref mut c2rust_fresh129 = *src.offset(
            (3 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh129 = (t3 + t4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh130 = *src.offset(
            (2 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh130 = *c2rust_fresh129;
        let ref mut c2rust_fresh131 = *src.offset(
            (1 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh131 = *c2rust_fresh130;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh131;
        let ref mut c2rust_fresh132 = *src.offset(
            (3 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh132 = (t3 + 2 as ::core::ffi::c_int * t4 + t5 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh133 = *src.offset(
            (2 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh133 = *c2rust_fresh132;
        let ref mut c2rust_fresh134 = *src.offset(
            (1 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh134 = *c2rust_fresh133;
        *src.offset(
            (0 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh134;
        let ref mut c2rust_fresh135 = *src.offset(
            (4 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh135 = (t4 + t5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh136 = *src.offset(
            (3 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh136 = *c2rust_fresh135;
        let ref mut c2rust_fresh137 = *src.offset(
            (2 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh137 = *c2rust_fresh136;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh137;
        let ref mut c2rust_fresh138 = *src.offset(
            (4 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh138 = (t4 + 2 as ::core::ffi::c_int * t5 + t6 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh139 = *src.offset(
            (3 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh139 = *c2rust_fresh138;
        let ref mut c2rust_fresh140 = *src.offset(
            (2 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh140 = *c2rust_fresh139;
        *src.offset(
            (1 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh140;
        let ref mut c2rust_fresh141 = *src.offset(
            (5 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh141 = (t5 + t6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh142 = *src.offset(
            (4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh142 = *c2rust_fresh141;
        let ref mut c2rust_fresh143 = *src.offset(
            (3 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh143 = *c2rust_fresh142;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh143;
        let ref mut c2rust_fresh144 = *src.offset(
            (5 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh144 = (t5 + 2 as ::core::ffi::c_int * t6 + t7 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh145 = *src.offset(
            (4 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh145 = *c2rust_fresh144;
        let ref mut c2rust_fresh146 = *src.offset(
            (3 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh146 = *c2rust_fresh145;
        *src.offset(
            (2 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh146;
        let ref mut c2rust_fresh147 = *src.offset(
            (6 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh147 = (t6 + t7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh148 = *src.offset(
            (5 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh148 = *c2rust_fresh147;
        let ref mut c2rust_fresh149 = *src.offset(
            (4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh149 = *c2rust_fresh148;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh149;
        let ref mut c2rust_fresh150 = *src.offset(
            (6 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh150 = (t6 + 2 as ::core::ffi::c_int * t7 + t8 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh151 = *src.offset(
            (5 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh151 = *c2rust_fresh150;
        let ref mut c2rust_fresh152 = *src.offset(
            (4 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh152 = *c2rust_fresh151;
        *src.offset(
            (3 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh152;
        let ref mut c2rust_fresh153 = *src.offset(
            (7 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh153 = (t7 + t8 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh154 = *src.offset(
            (6 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh154 = *c2rust_fresh153;
        let ref mut c2rust_fresh155 = *src.offset(
            (5 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh155 = *c2rust_fresh154;
        *src.offset(
            (4 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh155;
        let ref mut c2rust_fresh156 = *src.offset(
            (7 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh156 = (t7 + 2 as ::core::ffi::c_int * t8 + t9 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh157 = *src.offset(
            (6 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh157 = *c2rust_fresh156;
        let ref mut c2rust_fresh158 = *src.offset(
            (5 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh158 = *c2rust_fresh157;
        *src.offset(
            (4 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh158;
        let ref mut c2rust_fresh159 = *src.offset(
            (7 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh159 = (t8 + t9 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh160 = *src.offset(
            (6 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh160 = *c2rust_fresh159;
        *src.offset(
            (5 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh160;
        let ref mut c2rust_fresh161 = *src.offset(
            (7 as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh161 = (t8 + 2 as ::core::ffi::c_int * t9 + t10 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        let ref mut c2rust_fresh162 = *src.offset(
            (6 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh162 = *c2rust_fresh161;
        *src.offset(
            (5 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh162;
        let ref mut c2rust_fresh163 = *src.offset(
            (7 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh163 = (t9 + t10 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (6 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh163;
        let ref mut c2rust_fresh164 = *src.offset(
            (7 as ::core::ffi::c_int
                + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        );
        *c2rust_fresh164 = (t9 + 2 as ::core::ffi::c_int * t10 + t11 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (6 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = *c2rust_fresh164;
        *src.offset(
            (7 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t10 + t11 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
            as crate::src::common::common::pixel;
        *src.offset(
            (7 as ::core::ffi::c_int
                + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                as isize,
        ) = (t10 + 2 as ::core::ffi::c_int * t11 + t12 + 2 as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) as crate::src::common::common::pixel;
    }
}

unsafe extern "C" fn predict_8x8_hu_c(
    mut src: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
) {
    unsafe {
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
        let mut p1: ::core::ffi::c_int = pack8to16(
            (l0 + l1 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l0 + 2 as ::core::ffi::c_int * l1 + l2 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p2: ::core::ffi::c_int = pack8to16(
            (l1 + l2 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l1 + 2 as ::core::ffi::c_int * l2 + l3 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p3: ::core::ffi::c_int = pack8to16(
            (l2 + l3 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l2 + 2 as ::core::ffi::c_int * l3 + l4 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p4: ::core::ffi::c_int = pack8to16(
            (l3 + l4 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l3 + 2 as ::core::ffi::c_int * l4 + l5 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p5: ::core::ffi::c_int = pack8to16(
            (l4 + l5 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l4 + 2 as ::core::ffi::c_int * l5 + l6 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p6: ::core::ffi::c_int = pack8to16(
            (l5 + l6 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l5 + 2 as ::core::ffi::c_int * l6 + l7 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p7: ::core::ffi::c_int = pack8to16(
            (l6 + l7 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                as crate::stdlib::uint32_t,
            (l6 + 2 as ::core::ffi::c_int * l7 + l7 + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int) as crate::stdlib::uint32_t,
        ) as ::core::ffi::c_int;
        let mut p8: ::core::ffi::c_int =
            pack8to16(l7 as crate::stdlib::uint32_t, l7 as crate::stdlib::uint32_t)
                as ::core::ffi::c_int;
        (*(src.offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = pack16to32(p1 as crate::stdlib::uint32_t, p2 as crate::stdlib::uint32_t);
        (*(src.offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = pack16to32(p2 as crate::stdlib::uint32_t, p3 as crate::stdlib::uint32_t);
        let ref mut c2rust_fresh115 = (*(src.offset(
            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh115 = pack16to32(p3 as crate::stdlib::uint32_t, p4 as crate::stdlib::uint32_t);
        (*(src.offset(
            (4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh115;
        let ref mut c2rust_fresh116 = (*(src.offset(
            (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh116 = pack16to32(p4 as crate::stdlib::uint32_t, p5 as crate::stdlib::uint32_t);
        (*(src.offset(
            (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh116;
        let ref mut c2rust_fresh117 = (*(src.offset(
            (0 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh117 = pack16to32(p5 as crate::stdlib::uint32_t, p6 as crate::stdlib::uint32_t);
        (*(src.offset(
            (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh117;
        let ref mut c2rust_fresh118 = (*(src.offset(
            (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh118 = pack16to32(p6 as crate::stdlib::uint32_t, p7 as crate::stdlib::uint32_t);
        (*(src.offset(
            (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh118;
        let ref mut c2rust_fresh119 = (*(src.offset(
            (0 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh119 = pack16to32(p7 as crate::stdlib::uint32_t, p8 as crate::stdlib::uint32_t);
        (*(src.offset(
            (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh119;
        let ref mut c2rust_fresh120 = (*(src.offset(
            (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh120 = pack16to32(p8 as crate::stdlib::uint32_t, p8 as crate::stdlib::uint32_t);
        let ref mut c2rust_fresh121 = (*(src.offset(
            (0 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh121 = *c2rust_fresh120;
        let ref mut c2rust_fresh122 = (*(src.offset(
            (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        *c2rust_fresh122 = *c2rust_fresh121;
        (*(src.offset(
            (4 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize,
        ) as *mut crate::src::common::common::pixel
            as *mut crate::src::common::base::x264_union32_t))
            .i = *c2rust_fresh122;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_16x16_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pf: *mut crate::src::common::predict::x264_predict_t,
) {
    unsafe {
        let ref mut c2rust_fresh14 =
            *pf.offset(crate::src::common::predict::I_PRED_16x16_V as ::core::ffi::c_int as isize);
        *c2rust_fresh14 = Some(
            x264_8_predict_16x16_v_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh15 =
            *pf.offset(crate::src::common::predict::I_PRED_16x16_H as ::core::ffi::c_int as isize);
        *c2rust_fresh15 = Some(
            x264_8_predict_16x16_h_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh16 =
            *pf.offset(crate::src::common::predict::I_PRED_16x16_DC as ::core::ffi::c_int as isize);
        *c2rust_fresh16 = Some(
            x264_8_predict_16x16_dc_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh17 =
            *pf.offset(crate::src::common::predict::I_PRED_16x16_P as ::core::ffi::c_int as isize);
        *c2rust_fresh17 = Some(
            x264_8_predict_16x16_p_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh18 = *pf.offset(
            crate::src::common::predict::I_PRED_16x16_DC_LEFT as ::core::ffi::c_int as isize,
        );
        *c2rust_fresh18 = Some(
            predict_16x16_dc_left_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh19 = *pf.offset(
            crate::src::common::predict::I_PRED_16x16_DC_TOP as ::core::ffi::c_int as isize,
        );
        *c2rust_fresh19 = Some(
            predict_16x16_dc_top_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh20 = *pf.offset(
            crate::src::common::predict::I_PRED_16x16_DC_128 as ::core::ffi::c_int as isize,
        );
        *c2rust_fresh20 = Some(
            predict_16x16_dc_128_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x8c_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pf: *mut crate::src::common::predict::x264_predict_t,
) {
    unsafe {
        let ref mut c2rust_fresh21 =
            *pf.offset(crate::src::common::predict::I_PRED_CHROMA_V as ::core::ffi::c_int as isize);
        *c2rust_fresh21 = Some(
            x264_8_predict_8x8c_v_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh22 =
            *pf.offset(crate::src::common::predict::I_PRED_CHROMA_H as ::core::ffi::c_int as isize);
        *c2rust_fresh22 = Some(
            x264_8_predict_8x8c_h_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh23 = *pf
            .offset(crate::src::common::predict::I_PRED_CHROMA_DC as ::core::ffi::c_int as isize);
        *c2rust_fresh23 = Some(
            x264_8_predict_8x8c_dc_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh24 =
            *pf.offset(crate::src::common::predict::I_PRED_CHROMA_P as ::core::ffi::c_int as isize);
        *c2rust_fresh24 = Some(
            x264_8_predict_8x8c_p_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh25 = *pf.offset(
            crate::src::common::predict::I_PRED_CHROMA_DC_LEFT as ::core::ffi::c_int as isize,
        );
        *c2rust_fresh25 = Some(
            predict_8x8c_dc_left_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh26 = *pf.offset(
            crate::src::common::predict::I_PRED_CHROMA_DC_TOP as ::core::ffi::c_int as isize,
        );
        *c2rust_fresh26 = Some(
            predict_8x8c_dc_top_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh27 = *pf.offset(
            crate::src::common::predict::I_PRED_CHROMA_DC_128 as ::core::ffi::c_int as isize,
        );
        *c2rust_fresh27 = Some(
            predict_8x8c_dc_128_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x16c_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pf: *mut crate::src::common::predict::x264_predict_t,
) {
    unsafe {
        let ref mut c2rust_fresh28 =
            *pf.offset(crate::src::common::predict::I_PRED_CHROMA_V as ::core::ffi::c_int as isize);
        *c2rust_fresh28 = Some(
            x264_8_predict_8x16c_v_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh29 =
            *pf.offset(crate::src::common::predict::I_PRED_CHROMA_H as ::core::ffi::c_int as isize);
        *c2rust_fresh29 = Some(
            x264_8_predict_8x16c_h_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh30 = *pf
            .offset(crate::src::common::predict::I_PRED_CHROMA_DC as ::core::ffi::c_int as isize);
        *c2rust_fresh30 = Some(
            x264_8_predict_8x16c_dc_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh31 =
            *pf.offset(crate::src::common::predict::I_PRED_CHROMA_P as ::core::ffi::c_int as isize);
        *c2rust_fresh31 = Some(
            x264_8_predict_8x16c_p_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh32 = *pf.offset(
            crate::src::common::predict::I_PRED_CHROMA_DC_LEFT as ::core::ffi::c_int as isize,
        );
        *c2rust_fresh32 = Some(
            predict_8x16c_dc_left_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh33 = *pf.offset(
            crate::src::common::predict::I_PRED_CHROMA_DC_TOP as ::core::ffi::c_int as isize,
        );
        *c2rust_fresh33 = Some(
            predict_8x16c_dc_top_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh34 = *pf.offset(
            crate::src::common::predict::I_PRED_CHROMA_DC_128 as ::core::ffi::c_int as isize,
        );
        *c2rust_fresh34 = Some(
            predict_8x16c_dc_128_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_8x8_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pf: *mut crate::src::common::predict::x264_predict8x8_t,
    mut predict_filter: *mut crate::src::common::predict::x264_predict_8x8_filter_t,
) {
    unsafe {
        let ref mut c2rust_fresh101 =
            *pf.offset(crate::src::common::predict::I_PRED_8x8_V as ::core::ffi::c_int as isize);
        *c2rust_fresh101 = Some(
            x264_8_predict_8x8_v_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        let ref mut c2rust_fresh102 =
            *pf.offset(crate::src::common::predict::I_PRED_8x8_H as ::core::ffi::c_int as isize);
        *c2rust_fresh102 = Some(
            x264_8_predict_8x8_h_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        let ref mut c2rust_fresh103 =
            *pf.offset(crate::src::common::predict::I_PRED_8x8_DC as ::core::ffi::c_int as isize);
        *c2rust_fresh103 = Some(
            x264_8_predict_8x8_dc_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        let ref mut c2rust_fresh104 =
            *pf.offset(crate::src::common::predict::I_PRED_8x8_DDL as ::core::ffi::c_int as isize);
        *c2rust_fresh104 = Some(
            predict_8x8_ddl_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        let ref mut c2rust_fresh105 =
            *pf.offset(crate::src::common::predict::I_PRED_8x8_DDR as ::core::ffi::c_int as isize);
        *c2rust_fresh105 = Some(
            predict_8x8_ddr_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        let ref mut c2rust_fresh106 =
            *pf.offset(crate::src::common::predict::I_PRED_8x8_VR as ::core::ffi::c_int as isize);
        *c2rust_fresh106 = Some(
            predict_8x8_vr_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        let ref mut c2rust_fresh107 =
            *pf.offset(crate::src::common::predict::I_PRED_8x8_HD as ::core::ffi::c_int as isize);
        *c2rust_fresh107 = Some(
            predict_8x8_hd_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        let ref mut c2rust_fresh108 =
            *pf.offset(crate::src::common::predict::I_PRED_8x8_VL as ::core::ffi::c_int as isize);
        *c2rust_fresh108 = Some(
            predict_8x8_vl_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        let ref mut c2rust_fresh109 =
            *pf.offset(crate::src::common::predict::I_PRED_8x8_HU as ::core::ffi::c_int as isize);
        *c2rust_fresh109 = Some(
            predict_8x8_hu_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        let ref mut c2rust_fresh110 = *pf
            .offset(crate::src::common::predict::I_PRED_8x8_DC_LEFT as ::core::ffi::c_int as isize);
        *c2rust_fresh110 = Some(
            predict_8x8_dc_left_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        let ref mut c2rust_fresh111 = *pf
            .offset(crate::src::common::predict::I_PRED_8x8_DC_TOP as ::core::ffi::c_int as isize);
        *c2rust_fresh111 = Some(
            predict_8x8_dc_top_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        let ref mut c2rust_fresh112 = *pf
            .offset(crate::src::common::predict::I_PRED_8x8_DC_128 as ::core::ffi::c_int as isize);
        *c2rust_fresh112 = Some(
            predict_8x8_dc_128_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        ) as crate::src::common::predict::x264_predict8x8_t;
        *predict_filter = Some(
            predict_8x8_filter_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        ) as crate::src::common::predict::x264_predict_8x8_filter_t;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_predict_4x4_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pf: *mut crate::src::common::predict::x264_predict_t,
) {
    unsafe {
        let ref mut c2rust_fresh35 =
            *pf.offset(crate::src::common::predict::I_PRED_4x4_V as ::core::ffi::c_int as isize);
        *c2rust_fresh35 = Some(
            x264_8_predict_4x4_v_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh36 =
            *pf.offset(crate::src::common::predict::I_PRED_4x4_H as ::core::ffi::c_int as isize);
        *c2rust_fresh36 = Some(
            x264_8_predict_4x4_h_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh37 =
            *pf.offset(crate::src::common::predict::I_PRED_4x4_DC as ::core::ffi::c_int as isize);
        *c2rust_fresh37 = Some(
            x264_8_predict_4x4_dc_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh38 =
            *pf.offset(crate::src::common::predict::I_PRED_4x4_DDL as ::core::ffi::c_int as isize);
        *c2rust_fresh38 = Some(
            predict_4x4_ddl_c as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh39 =
            *pf.offset(crate::src::common::predict::I_PRED_4x4_DDR as ::core::ffi::c_int as isize);
        *c2rust_fresh39 = Some(
            predict_4x4_ddr_c as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh40 =
            *pf.offset(crate::src::common::predict::I_PRED_4x4_VR as ::core::ffi::c_int as isize);
        *c2rust_fresh40 = Some(
            predict_4x4_vr_c as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh41 =
            *pf.offset(crate::src::common::predict::I_PRED_4x4_HD as ::core::ffi::c_int as isize);
        *c2rust_fresh41 = Some(
            predict_4x4_hd_c as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh42 =
            *pf.offset(crate::src::common::predict::I_PRED_4x4_VL as ::core::ffi::c_int as isize);
        *c2rust_fresh42 = Some(
            predict_4x4_vl_c as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh43 =
            *pf.offset(crate::src::common::predict::I_PRED_4x4_HU as ::core::ffi::c_int as isize);
        *c2rust_fresh43 = Some(
            predict_4x4_hu_c as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh44 = *pf
            .offset(crate::src::common::predict::I_PRED_4x4_DC_LEFT as ::core::ffi::c_int as isize);
        *c2rust_fresh44 = Some(
            predict_4x4_dc_left_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh45 = *pf
            .offset(crate::src::common::predict::I_PRED_4x4_DC_TOP as ::core::ffi::c_int as isize);
        *c2rust_fresh45 = Some(
            predict_4x4_dc_top_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
        let ref mut c2rust_fresh46 = *pf
            .offset(crate::src::common::predict::I_PRED_4x4_DC_128 as ::core::ffi::c_int as isize);
        *c2rust_fresh46 = Some(
            predict_4x4_dc_128_c
                as unsafe extern "C" fn(*mut crate::src::common::common::pixel) -> (),
        ) as crate::src::common::predict::x264_predict_t;
    }
}
