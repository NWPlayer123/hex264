pub mod common_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_clip_pixel(
        mut x: ::core::ffi::c_int,
    ) -> crate::src::common::common::pixel {
        return (if x & !crate::src::common::common::PIXEL_MAX != 0 {
            -x >> 31i32 & crate::src::common::common::PIXEL_MAX
        } else {
            x
        }) as crate::src::common::common::pixel;
    }
}
pub mod base_h {
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
use crate::src::common::deblock::base_h::x264_clip3;
use crate::src::common::deblock::common_h::x264_clip_pixel;
static mut i_alpha_table: [crate::stdlib::uint8_t; 88] = [
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 4u8, 4u8, 5u8, 6u8, 7u8, 8u8, 9u8, 10u8, 12u8, 13u8, 15u8, 17u8, 20u8, 22u8, 25u8,
    28u8, 32u8, 36u8, 40u8, 45u8, 50u8, 56u8, 63u8, 71u8, 80u8, 90u8, 101u8, 113u8, 127u8, 144u8,
    162u8, 182u8, 203u8, 226u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8, 255u8,
    255u8, 255u8, 255u8, 255u8, 255u8,
];
static mut i_beta_table: [crate::stdlib::uint8_t; 88] = [
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
    0u8, 0u8, 2u8, 2u8, 2u8, 3u8, 3u8, 3u8, 3u8, 4u8, 4u8, 4u8, 6u8, 6u8, 7u8, 7u8, 8u8, 8u8, 9u8,
    9u8, 10u8, 10u8, 11u8, 11u8, 12u8, 12u8, 13u8, 13u8, 14u8, 14u8, 15u8, 15u8, 16u8, 16u8, 17u8,
    17u8, 18u8, 18u8, 18u8, 18u8, 18u8, 18u8, 18u8, 18u8, 18u8, 18u8, 18u8, 18u8, 18u8, 18u8,
];
static mut i_tc0_table: [[crate::stdlib::int8_t; 4]; 88] = [
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 0i8],
    [-1i8, 0i8, 0i8, 1i8],
    [-1i8, 0i8, 0i8, 1i8],
    [-1i8, 0i8, 0i8, 1i8],
    [-1i8, 0i8, 0i8, 1i8],
    [-1i8, 0i8, 1i8, 1i8],
    [-1i8, 0i8, 1i8, 1i8],
    [-1i8, 1i8, 1i8, 1i8],
    [-1i8, 1i8, 1i8, 1i8],
    [-1i8, 1i8, 1i8, 1i8],
    [-1i8, 1i8, 1i8, 1i8],
    [-1i8, 1i8, 1i8, 2i8],
    [-1i8, 1i8, 1i8, 2i8],
    [-1i8, 1i8, 1i8, 2i8],
    [-1i8, 1i8, 1i8, 2i8],
    [-1i8, 1i8, 2i8, 3i8],
    [-1i8, 1i8, 2i8, 3i8],
    [-1i8, 2i8, 2i8, 3i8],
    [-1i8, 2i8, 2i8, 4i8],
    [-1i8, 2i8, 3i8, 4i8],
    [-1i8, 2i8, 3i8, 4i8],
    [-1i8, 3i8, 3i8, 5i8],
    [-1i8, 3i8, 4i8, 6i8],
    [-1i8, 3i8, 4i8, 6i8],
    [-1i8, 4i8, 5i8, 7i8],
    [-1i8, 4i8, 5i8, 8i8],
    [-1i8, 4i8, 6i8, 9i8],
    [-1i8, 5i8, 7i8, 10i8],
    [-1i8, 6i8, 8i8, 11i8],
    [-1i8, 6i8, 8i8, 13i8],
    [-1i8, 7i8, 10i8, 14i8],
    [-1i8, 8i8, 11i8, 16i8],
    [-1i8, 9i8, 12i8, 18i8],
    [-1i8, 10i8, 13i8, 20i8],
    [-1i8, 11i8, 15i8, 23i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
    [-1i8, 13i8, 17i8, 25i8],
];
#[inline(always)]
unsafe extern "C" fn deblock_edge_luma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: crate::stdlib::int8_t,
) {
    unsafe {
        let mut p2 = *pix.offset(-3isize * xstride) as ::core::ffi::c_int;
        let mut p1 = *pix.offset(-2isize * xstride) as ::core::ffi::c_int;
        let mut p0 = *pix.offset(-1isize * xstride) as ::core::ffi::c_int;
        let mut q0 = *pix.offset(0isize * xstride) as ::core::ffi::c_int;
        let mut q1 = *pix.offset(1isize * xstride) as ::core::ffi::c_int;
        let mut q2 = *pix.offset(2isize * xstride) as ::core::ffi::c_int;
        if crate::stdlib::abs(p0 - q0) < alpha
            && crate::stdlib::abs(p1 - p0) < beta
            && crate::stdlib::abs(q1 - q0) < beta
        {
            let mut tc = tc0 as ::core::ffi::c_int;
            if crate::stdlib::abs(p2 - p0) < beta {
                if tc0 != 0 {
                    *pix.offset(-2isize * xstride) =
                        (p1 + x264_clip3(
                            (p2 + (p0 + q0 + 1i32 >> 1i32) >> 1i32) - p1,
                            -(tc0 as ::core::ffi::c_int),
                            tc0 as ::core::ffi::c_int,
                        )) as crate::src::common::common::pixel;
                }
                tc += 1;
            }
            if crate::stdlib::abs(q2 - q0) < beta {
                if tc0 != 0 {
                    *pix.offset(1isize * xstride) =
                        (q1 + x264_clip3(
                            (q2 + (p0 + q0 + 1i32 >> 1i32) >> 1i32) - q1,
                            -(tc0 as ::core::ffi::c_int),
                            tc0 as ::core::ffi::c_int,
                        )) as crate::src::common::common::pixel;
                }
                tc += 1;
            }
            let mut delta = x264_clip3((q0 - p0) * 4i32 + (p1 - q1) + 4i32 >> 3i32, -tc, tc);
            *pix.offset(-1isize * xstride) = x264_clip_pixel(p0 + delta);
            *pix.offset(0isize * xstride) = x264_clip_pixel(q0 - delta);
        }
    }
}
#[inline]
unsafe extern "C" fn deblock_luma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut ystride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        let mut i = 0i32;
        while i < 4i32 {
            if (*tc0.offset(i as isize) as ::core::ffi::c_int) < 0i32 {
                pix = pix.offset(4isize * ystride);
            } else {
                let mut d = 0i32;
                while d < 4i32 {
                    deblock_edge_luma_c(pix, xstride, alpha, beta, *tc0.offset(i as isize));
                    d += 1;
                    pix = pix.offset(ystride);
                }
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn deblock_h_luma_mbaff_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        let mut d = 0i32;
        while d < 8i32 {
            deblock_edge_luma_c(pix, 1isize, alpha, beta, *tc0.offset((d >> 1i32) as isize));
            d += 1;
            pix = pix.offset(stride);
        }
    }
}
unsafe extern "C" fn deblock_v_luma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_luma_c(pix, stride, 1isize, alpha, beta, tc0);
    }
}
unsafe extern "C" fn deblock_h_luma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_luma_c(pix, 1isize, stride, alpha, beta, tc0);
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_chroma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc: crate::stdlib::int8_t,
) {
    unsafe {
        let mut p1 = *pix.offset(-2isize * xstride) as ::core::ffi::c_int;
        let mut p0 = *pix.offset(-1isize * xstride) as ::core::ffi::c_int;
        let mut q0 = *pix.offset(0isize * xstride) as ::core::ffi::c_int;
        let mut q1 = *pix.offset(1isize * xstride) as ::core::ffi::c_int;
        if crate::stdlib::abs(p0 - q0) < alpha
            && crate::stdlib::abs(p1 - p0) < beta
            && crate::stdlib::abs(q1 - q0) < beta
        {
            let mut delta = x264_clip3(
                (q0 - p0) * 4i32 + (p1 - q1) + 4i32 >> 3i32,
                -(tc as ::core::ffi::c_int),
                tc as ::core::ffi::c_int,
            );
            *pix.offset(-1isize * xstride) = x264_clip_pixel(p0 + delta);
            *pix.offset(0isize * xstride) = x264_clip_pixel(q0 - delta);
        }
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_chroma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut height: ::core::ffi::c_int,
    mut xstride: crate::stdlib::intptr_t,
    mut ystride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        let mut i = 0i32;
        while i < 4i32 {
            let mut tc = *tc0.offset(i as isize) as ::core::ffi::c_int;
            if tc <= 0i32 {
                pix = pix.offset(height as crate::stdlib::intptr_t * ystride);
            } else {
                let mut d = 0i32;
                while d < height {
                    let mut e = 0i32;
                    while e < 2i32 {
                        deblock_edge_chroma_c(pix, xstride, alpha, beta, *tc0.offset(i as isize));
                        e += 1;
                        pix = pix.offset(1);
                    }
                    d += 1;
                    pix = pix.offset(ystride - 2isize);
                }
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn deblock_h_chroma_mbaff_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_chroma_c(pix, 1i32, 2isize, stride, alpha, beta, tc0);
    }
}
unsafe extern "C" fn deblock_v_chroma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_chroma_c(pix, 2i32, stride, 2isize, alpha, beta, tc0);
    }
}
unsafe extern "C" fn deblock_h_chroma_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_chroma_c(pix, 2i32, 2isize, stride, alpha, beta, tc0);
    }
}
unsafe extern "C" fn deblock_h_chroma_422_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
    mut tc0: *mut crate::stdlib::int8_t,
) {
    unsafe {
        deblock_chroma_c(pix, 4i32, 2isize, stride, alpha, beta, tc0);
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_luma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        let mut p2 = *pix.offset(-3isize * xstride) as ::core::ffi::c_int;
        let mut p1 = *pix.offset(-2isize * xstride) as ::core::ffi::c_int;
        let mut p0 = *pix.offset(-1isize * xstride) as ::core::ffi::c_int;
        let mut q0 = *pix.offset(0isize * xstride) as ::core::ffi::c_int;
        let mut q1 = *pix.offset(1isize * xstride) as ::core::ffi::c_int;
        let mut q2 = *pix.offset(2isize * xstride) as ::core::ffi::c_int;
        if crate::stdlib::abs(p0 - q0) < alpha
            && crate::stdlib::abs(p1 - p0) < beta
            && crate::stdlib::abs(q1 - q0) < beta
        {
            if crate::stdlib::abs(p0 - q0) < (alpha >> 2i32) + 2i32 {
                if crate::stdlib::abs(p2 - p0) < beta {
                    let p3 = *pix.offset(-4isize * xstride) as ::core::ffi::c_int;
                    *pix.offset(-1isize * xstride) =
                        (p2 + 2i32 * p1 + 2i32 * p0 + 2i32 * q0 + q1 + 4i32 >> 3i32)
                            as crate::src::common::common::pixel;
                    *pix.offset(-2isize * xstride) =
                        (p2 + p1 + p0 + q0 + 2i32 >> 2i32) as crate::src::common::common::pixel;
                    *pix.offset(-3isize * xstride) = (2i32 * p3 + 3i32 * p2 + p1 + p0 + q0 + 4i32
                        >> 3i32)
                        as crate::src::common::common::pixel;
                } else {
                    *pix.offset(-1isize * xstride) =
                        (2i32 * p1 + p0 + q1 + 2i32 >> 2i32) as crate::src::common::common::pixel;
                }
                if crate::stdlib::abs(q2 - q0) < beta {
                    let q3 = *pix.offset(3isize * xstride) as ::core::ffi::c_int;
                    *pix.offset(0isize * xstride) =
                        (p1 + 2i32 * p0 + 2i32 * q0 + 2i32 * q1 + q2 + 4i32 >> 3i32)
                            as crate::src::common::common::pixel;
                    *pix.offset(1isize * xstride) =
                        (p0 + q0 + q1 + q2 + 2i32 >> 2i32) as crate::src::common::common::pixel;
                    *pix.offset(2isize * xstride) = (2i32 * q3 + 3i32 * q2 + q1 + q0 + p0 + 4i32
                        >> 3i32)
                        as crate::src::common::common::pixel;
                } else {
                    *pix.offset(0isize * xstride) =
                        (2i32 * q1 + q0 + p1 + 2i32 >> 2i32) as crate::src::common::common::pixel;
                }
            } else {
                *pix.offset(-1isize * xstride) =
                    (2i32 * p1 + p0 + q1 + 2i32 >> 2i32) as crate::src::common::common::pixel;
                *pix.offset(0isize * xstride) =
                    (2i32 * q1 + q0 + p1 + 2i32 >> 2i32) as crate::src::common::common::pixel;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn deblock_luma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut ystride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        let mut d = 0i32;
        while d < 16i32 {
            deblock_edge_luma_intra_c(pix, xstride, alpha, beta);
            d += 1;
            pix = pix.offset(ystride);
        }
    }
}
unsafe extern "C" fn deblock_h_luma_intra_mbaff_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut ystride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        let mut d = 0i32;
        while d < 8i32 {
            deblock_edge_luma_intra_c(pix, 1isize, alpha, beta);
            d += 1;
            pix = pix.offset(ystride);
        }
    }
}
unsafe extern "C" fn deblock_v_luma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_luma_intra_c(pix, stride, 1isize, alpha, beta);
    }
}
unsafe extern "C" fn deblock_h_luma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_luma_intra_c(pix, 1isize, stride, alpha, beta);
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_chroma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut xstride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        let mut p1 = *pix.offset(-2isize * xstride) as ::core::ffi::c_int;
        let mut p0 = *pix.offset(-1isize * xstride) as ::core::ffi::c_int;
        let mut q0 = *pix.offset(0isize * xstride) as ::core::ffi::c_int;
        let mut q1 = *pix.offset(1isize * xstride) as ::core::ffi::c_int;
        if crate::stdlib::abs(p0 - q0) < alpha
            && crate::stdlib::abs(p1 - p0) < beta
            && crate::stdlib::abs(q1 - q0) < beta
        {
            *pix.offset(-1isize * xstride) =
                (2i32 * p1 + p0 + q1 + 2i32 >> 2i32) as crate::src::common::common::pixel;
            *pix.offset(0isize * xstride) =
                (2i32 * q1 + q0 + p1 + 2i32 >> 2i32) as crate::src::common::common::pixel;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_chroma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut xstride: crate::stdlib::intptr_t,
    mut ystride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        let mut d = 0i32;
        while d < height {
            let mut e = 0i32;
            while e < width {
                deblock_edge_chroma_intra_c(pix, xstride, alpha, beta);
                e += 1;
                pix = pix.offset(1);
            }
            d += 1;
            pix = pix.offset(ystride - 2isize);
        }
    }
}
unsafe extern "C" fn deblock_h_chroma_intra_mbaff_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_chroma_intra_c(pix, 2i32, 4i32, 2isize, stride, alpha, beta);
    }
}
unsafe extern "C" fn deblock_v_chroma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_chroma_intra_c(pix, 1i32, 16i32, stride, 2isize, alpha, beta);
    }
}
unsafe extern "C" fn deblock_h_chroma_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_chroma_intra_c(pix, 2i32, 8i32, 2isize, stride, alpha, beta);
    }
}
unsafe extern "C" fn deblock_h_chroma_422_intra_c(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut alpha: ::core::ffi::c_int,
    mut beta: ::core::ffi::c_int,
) {
    unsafe {
        deblock_chroma_intra_c(pix, 2i32, 16i32, 2isize, stride, alpha, beta);
    }
}
unsafe extern "C" fn deblock_strength_c(
    mut nnz: *mut crate::stdlib::uint8_t,
    mut ref_0: *mut [crate::stdlib::int8_t; 40],
    mut mv: *mut [[crate::stdlib::int16_t; 2]; 40],
    mut bs: *mut [[crate::stdlib::uint8_t; 4]; 8],
    mut mvy_limit: ::core::ffi::c_int,
    mut bframe: ::core::ffi::c_int,
) {
    unsafe {
        let mut dir = 0i32;
        while dir < 2i32 {
            let mut s1 = if dir != 0 { 1i32 } else { 8i32 };
            let mut s2 = if dir != 0 { 8i32 } else { 1i32 };
            let mut edge = 0i32;
            while edge < 4i32 {
                let mut i = 0i32;
                let mut loc = crate::src::common::base::X264_SCAN8_0 + edge * s2;
                while i < 4i32 {
                    let mut locn = loc - s2;
                    if *nnz.offset(loc as isize) as ::core::ffi::c_int != 0
                        || *nnz.offset(locn as isize) as ::core::ffi::c_int != 0
                    {
                        (*bs.offset(dir as isize))[edge as usize][i as usize] = 2u8;
                    } else if (*ref_0.offset(0isize))[loc as usize] as ::core::ffi::c_int
                        != (*ref_0.offset(0isize))[locn as usize] as ::core::ffi::c_int
                        || crate::stdlib::abs(
                            (*mv.offset(0isize))[loc as usize][0usize] as ::core::ffi::c_int
                                - (*mv.offset(0isize))[locn as usize][0usize] as ::core::ffi::c_int,
                        ) >= 4i32
                        || crate::stdlib::abs(
                            (*mv.offset(0isize))[loc as usize][1usize] as ::core::ffi::c_int
                                - (*mv.offset(0isize))[locn as usize][1usize] as ::core::ffi::c_int,
                        ) >= mvy_limit
                        || bframe != 0
                            && ((*ref_0.offset(1isize))[loc as usize] as ::core::ffi::c_int
                                != (*ref_0.offset(1isize))[locn as usize] as ::core::ffi::c_int
                                || crate::stdlib::abs(
                                    (*mv.offset(1isize))[loc as usize][0usize]
                                        as ::core::ffi::c_int
                                        - (*mv.offset(1isize))[locn as usize][0usize]
                                            as ::core::ffi::c_int,
                                ) >= 4i32
                                || crate::stdlib::abs(
                                    (*mv.offset(1isize))[loc as usize][1usize]
                                        as ::core::ffi::c_int
                                        - (*mv.offset(1isize))[locn as usize][1usize]
                                            as ::core::ffi::c_int,
                                ) >= mvy_limit)
                    {
                        (*bs.offset(dir as isize))[edge as usize][i as usize] = 1u8;
                    } else {
                        (*bs.offset(dir as isize))[edge as usize][i as usize] = 0u8;
                    }
                    i += 1;
                    loc += s1;
                }
                edge += 1;
            }
            dir += 1;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_edge(
    mut _h: *mut crate::src::common::common::x264_t,
    mut pix: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut bS: *mut crate::stdlib::uint8_t,
    mut i_qp: ::core::ffi::c_int,
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut b_chroma: ::core::ffi::c_int,
    mut pf_inter: crate::src::common::frame::x264_deblock_inter_t,
) {
    unsafe {
        let mut index_a = i_qp + a;
        let mut index_b = i_qp + b;
        let mut alpha = (i_alpha_table[(index_a + 24i32) as usize] as ::core::ffi::c_int)
            << crate::internal::BIT_DEPTH - 8i32;
        let mut beta = (i_beta_table[(index_b + 24i32) as usize] as ::core::ffi::c_int)
            << crate::internal::BIT_DEPTH - 8i32;
        let mut tc = [0; 4];
        if (*(bS as *mut crate::src::common::base::x264_union32_t)).i == 0
            || alpha == 0
            || beta == 0
        {
            return;
        }
        tc[0usize] = (i_tc0_table[(index_a + 24i32) as usize][*bS.offset(0isize) as usize]
            as ::core::ffi::c_int
            * ((1i32) << crate::internal::BIT_DEPTH - 8i32)
            + b_chroma) as crate::stdlib::int8_t;
        tc[1usize] = (i_tc0_table[(index_a + 24i32) as usize][*bS.offset(1isize) as usize]
            as ::core::ffi::c_int
            * ((1i32) << crate::internal::BIT_DEPTH - 8i32)
            + b_chroma) as crate::stdlib::int8_t;
        tc[2usize] = (i_tc0_table[(index_a + 24i32) as usize][*bS.offset(2isize) as usize]
            as ::core::ffi::c_int
            * ((1i32) << crate::internal::BIT_DEPTH - 8i32)
            + b_chroma) as crate::stdlib::int8_t;
        tc[3usize] = (i_tc0_table[(index_a + 24i32) as usize][*bS.offset(3isize) as usize]
            as ::core::ffi::c_int
            * ((1i32) << crate::internal::BIT_DEPTH - 8i32)
            + b_chroma) as crate::stdlib::int8_t;
        pf_inter.expect("non-null function pointer")(
            pix,
            i_stride,
            alpha,
            beta,
            &raw mut tc as *mut crate::stdlib::int8_t,
        );
    }
}
#[inline(always)]
unsafe extern "C" fn deblock_edge_intra(
    mut _h: *mut crate::src::common::common::x264_t,
    mut pix: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut _bS: *mut crate::stdlib::uint8_t,
    mut i_qp: ::core::ffi::c_int,
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut _b_chroma: ::core::ffi::c_int,
    mut pf_intra: crate::src::common::frame::x264_deblock_intra_t,
) {
    unsafe {
        let mut index_a = i_qp + a;
        let mut index_b = i_qp + b;
        let mut alpha = (i_alpha_table[(index_a + 24i32) as usize] as ::core::ffi::c_int)
            << crate::internal::BIT_DEPTH - 8i32;
        let mut beta = (i_beta_table[(index_b + 24i32) as usize] as ::core::ffi::c_int)
            << crate::internal::BIT_DEPTH - 8i32;
        if alpha == 0 || beta == 0 {
            return;
        }
        pf_intra.expect("non-null function pointer")(pix, i_stride, alpha, beta);
    }
}
#[inline(always)]
unsafe extern "C" fn macroblock_cache_load_neighbours_deblock(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) {
    unsafe {
        let mut deblock_on_slice_edges =
            ((*h).sh.i_disable_deblocking_filter_idc != 2i32) as ::core::ffi::c_int;
        (*h).mb.i_neighbour = 0u32;
        (*h).mb.i_mb_xy = mb_y * (*h).mb.i_mb_stride + mb_x;
        (*h).mb.b_interlaced = ((*h).param.b_interlaced != 0
            && *(*h).mb.field.offset((*h).mb.i_mb_xy as isize) as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
        (*h).mb.i_mb_top_y = mb_y - ((1i32) << (*h).mb.b_interlaced);
        (*h).mb.i_mb_top_xy = mb_x + (*h).mb.i_mb_stride * (*h).mb.i_mb_top_y;
        (*h).mb.i_mb_left_xy[0usize] = (*h).mb.i_mb_xy - 1i32;
        (*h).mb.i_mb_left_xy[1usize] = (*h).mb.i_mb_left_xy[0usize];
        if (*h).sh.b_mbaff != 0 {
            if mb_y & 1i32 != 0 {
                if mb_x != 0
                    && *(*h).mb.field.offset(((*h).mb.i_mb_xy - 1i32) as isize)
                        as ::core::ffi::c_int
                        != (*h).mb.b_interlaced
                {
                    (*h).mb.i_mb_left_xy[0usize] -= (*h).mb.i_mb_stride;
                }
            } else {
                if (*h).mb.i_mb_top_xy >= 0i32
                    && (*h).mb.b_interlaced != 0
                    && *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) == 0
                {
                    (*h).mb.i_mb_top_xy += (*h).mb.i_mb_stride;
                    (*h).mb.i_mb_top_y += 1;
                }
                if mb_x != 0
                    && *(*h).mb.field.offset(((*h).mb.i_mb_xy - 1i32) as isize)
                        as ::core::ffi::c_int
                        != (*h).mb.b_interlaced
                {
                    (*h).mb.i_mb_left_xy[1usize] += (*h).mb.i_mb_stride;
                }
            }
        }
        if mb_x > 0i32
            && (deblock_on_slice_edges != 0
                || *(*h)
                    .mb
                    .slice_table
                    .offset((*h).mb.i_mb_left_xy[0usize] as isize)
                    == *(*h).mb.slice_table.offset((*h).mb.i_mb_xy as isize))
        {
            (*h).mb.i_neighbour |= crate::src::common::macroblock::MB_LEFT;
        }
        if mb_y > (*h).mb.b_interlaced
            && (deblock_on_slice_edges != 0
                || *(*h).mb.slice_table.offset((*h).mb.i_mb_top_xy as isize)
                    == *(*h).mb.slice_table.offset((*h).mb.i_mb_xy as isize))
        {
            (*h).mb.i_neighbour |= crate::src::common::macroblock::MB_TOP;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_frame_deblock_row(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_y: ::core::ffi::c_int,
) {
    unsafe {
        let mut b_interlaced = (*h).sh.b_mbaff;
        let mut a = (*h).sh.i_alpha_c0_offset - crate::src::common::common::QP_BD_OFFSET;
        let mut b = (*h).sh.i_beta_offset - crate::src::common::common::QP_BD_OFFSET;
        let mut qp_thresh = 15i32
            - (if a < b { a } else { b })
            - (if 0i32
                > (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                    .i_chroma_qp_index_offset
            {
                0i32
            } else {
                (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                    .i_chroma_qp_index_offset
            });
        let mut stridey = (*(*h).fdec).i_stride[0usize];
        let mut strideuv = (*(*h).fdec).i_stride[1usize];
        let mut chroma_format = crate::src::common::base::CHROMA_444 as ::core::ffi::c_int;
        let mut chroma444 = (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut chroma_height = 16i32
            >> (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        let mut uvdiff = if chroma444 != 0 {
            (*(*h).fdec).plane[2usize].offset_from((*(*h).fdec).plane[1usize])
        } else {
            1isize
        };
        let mut mb_x = 0i32;
        while mb_x < (*h).mb.i_mb_width {
            crate::src::common::macroblock::x264_8_prefetch_fenc(h, (*h).fdec, mb_x, mb_y);
            macroblock_cache_load_neighbours_deblock(h, mb_x, mb_y);
            let mut mb_xy = (*h).mb.i_mb_xy;
            let mut transform_8x8 =
                *(*h).mb.mb_transform_size.offset(mb_xy as isize) as ::core::ffi::c_int;
            let mut intra_cur = (*(*h).mb.type_0.offset(mb_xy as isize) as ::core::ffi::c_int
                == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                || *(*h).mb.type_0.offset(mb_xy as isize) as ::core::ffi::c_int
                    == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                || *(*h).mb.type_0.offset(mb_xy as isize) as ::core::ffi::c_int
                    == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                || *(*h).mb.type_0.offset(mb_xy as isize) as ::core::ffi::c_int
                    == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            let mut bs = &raw mut *(*(&raw mut (*h).deblock_strength
                as *mut *mut [[[crate::stdlib::uint8_t; 4]; 8]; 2])
                .offset((mb_y & 1i32) as isize))
            .offset(
                (if (*h).param.b_sliced_threads != 0 {
                    mb_xy
                } else {
                    mb_x
                }) as isize,
            ) as *mut [[crate::stdlib::uint8_t; 4]; 8];
            let mut pixy = (*(*h).fdec).plane[0usize]
                .offset((16i32 * mb_y * stridey) as isize)
                .offset((16i32 * mb_x) as isize);
            let mut pixuv = if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                (*(*h).fdec).plane[1usize]
                    .offset((chroma_height * mb_y * strideuv) as isize)
                    .offset((16i32 * mb_x) as isize)
            } else {
                ::core::ptr::null_mut::<crate::src::common::common::pixel>()
            };
            if mb_y & (*h).mb.b_interlaced != 0 {
                pixy = pixy.offset(-((15i32 * stridey) as isize));
                if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int != 0 {
                    pixuv = pixuv.offset(-(((chroma_height - 1i32) * strideuv) as isize));
                }
            }
            let mut stride2y = stridey << (*h).mb.b_interlaced;
            let mut stride2uv = strideuv << (*h).mb.b_interlaced;
            let mut qp = *(*h).mb.qp.offset(mb_xy as isize) as ::core::ffi::c_int;
            let mut qpc = *(*h).chroma_qp_table.offset(qp as isize) as ::core::ffi::c_int;
            let mut first_edge_only = (*(*h).mb.partition.offset(mb_xy as isize)
                as ::core::ffi::c_int
                == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
                && *(*h).mb.cbp.offset(mb_xy as isize) == 0
                && intra_cur == 0
                || qp <= qp_thresh) as ::core::ffi::c_int;
            if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_LEFT != 0 {
                if b_interlaced != 0
                    && *(*h).mb.field.offset((*h).mb.i_mb_left_xy[0usize] as isize)
                        as ::core::ffi::c_int
                        != (*h).mb.b_interlaced
                {
                    let mut luma_qp = [0; 2];
                    let mut chroma_qp = [0; 2];
                    let mut left_qp = [0; 2];
                    let mut luma_deblock = (*h).loopf.deblock_luma_mbaff;
                    let mut chroma_deblock = (*h).loopf.deblock_chroma_mbaff;
                    let mut luma_intra_deblock = (*h).loopf.deblock_luma_intra_mbaff;
                    let mut chroma_intra_deblock = (*h).loopf.deblock_chroma_intra_mbaff;
                    let mut c = if chroma444 != 0 { 0i32 } else { 1i32 };
                    left_qp[0usize] = *(*h).mb.qp.offset((*h).mb.i_mb_left_xy[0usize] as isize)
                        as ::core::ffi::c_int;
                    luma_qp[0usize] = qp + left_qp[0usize] + 1i32 >> 1i32;
                    chroma_qp[0usize] = qpc
                        + *(*h).chroma_qp_table.offset(left_qp[0usize] as isize)
                            as ::core::ffi::c_int
                        + 1i32
                        >> 1i32;
                    if intra_cur != 0
                        || (*(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[0usize] as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[0usize] as isize)
                                as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[0usize] as isize)
                                as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[0usize] as isize)
                                as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    {
                        deblock_edge_intra(
                            h,
                            pixy,
                            (2i32 * stridey) as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(0isize)
                                as *mut crate::stdlib::uint8_t,
                            luma_qp[0usize],
                            a,
                            b,
                            0i32,
                            luma_intra_deblock,
                        );
                        if chroma_format != 0 {
                            deblock_edge_intra(
                                h,
                                pixuv,
                                (2i32 * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0isize)
                                    as *mut crate::stdlib::uint8_t,
                                chroma_qp[0usize],
                                a,
                                b,
                                c,
                                chroma_intra_deblock,
                            );
                            if chroma444 != 0 {
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(uvdiff),
                                    (2i32 * strideuv) as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    chroma_qp[0usize],
                                    a,
                                    b,
                                    c,
                                    chroma_intra_deblock,
                                );
                            }
                        }
                    } else {
                        deblock_edge(
                            h,
                            pixy,
                            (2i32 * stridey) as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(0isize)
                                as *mut crate::stdlib::uint8_t,
                            luma_qp[0usize],
                            a,
                            b,
                            0i32,
                            luma_deblock,
                        );
                        if chroma_format != 0 {
                            deblock_edge(
                                h,
                                pixuv,
                                (2i32 * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0isize)
                                    as *mut crate::stdlib::uint8_t,
                                chroma_qp[0usize],
                                a,
                                b,
                                c,
                                chroma_deblock,
                            );
                            if chroma444 != 0 {
                                deblock_edge(
                                    h,
                                    pixuv.offset(uvdiff),
                                    (2i32 * strideuv) as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    chroma_qp[0usize],
                                    a,
                                    b,
                                    c,
                                    chroma_deblock,
                                );
                            }
                        }
                    }
                    let mut offy = if (*h).mb.b_interlaced != 0 {
                        4i32
                    } else {
                        0i32
                    };
                    let mut offuv = if (*h).mb.b_interlaced != 0 {
                        4i32 - (crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                    } else {
                        0i32
                    };
                    left_qp[1usize] = *(*h).mb.qp.offset((*h).mb.i_mb_left_xy[1usize] as isize)
                        as ::core::ffi::c_int;
                    luma_qp[1usize] = qp + left_qp[1usize] + 1i32 >> 1i32;
                    chroma_qp[1usize] = qpc
                        + *(*h).chroma_qp_table.offset(left_qp[1usize] as isize)
                            as ::core::ffi::c_int
                        + 1i32
                        >> 1i32;
                    if intra_cur != 0
                        || (*(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[1usize] as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[1usize] as isize)
                                as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[1usize] as isize)
                                as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset((*h).mb.i_mb_left_xy[1usize] as isize)
                                as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                    {
                        deblock_edge_intra(
                            h,
                            pixy.offset((stridey << offy) as isize),
                            (2i32 * stridey) as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(4isize)
                                as *mut crate::stdlib::uint8_t,
                            luma_qp[1usize],
                            a,
                            b,
                            0i32,
                            luma_intra_deblock,
                        );
                        if chroma_format != 0 {
                            deblock_edge_intra(
                                h,
                                pixuv.offset((strideuv << offuv) as isize),
                                (2i32 * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(4isize)
                                    as *mut crate::stdlib::uint8_t,
                                chroma_qp[1usize],
                                a,
                                b,
                                c,
                                chroma_intra_deblock,
                            );
                            if chroma444 != 0 {
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(uvdiff).offset((strideuv << offuv) as isize),
                                    (2i32 * strideuv) as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(4isize)
                                        as *mut crate::stdlib::uint8_t,
                                    chroma_qp[1usize],
                                    a,
                                    b,
                                    c,
                                    chroma_intra_deblock,
                                );
                            }
                        }
                    } else {
                        deblock_edge(
                            h,
                            pixy.offset((stridey << offy) as isize),
                            (2i32 * stridey) as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(4isize)
                                as *mut crate::stdlib::uint8_t,
                            luma_qp[1usize],
                            a,
                            b,
                            0i32,
                            luma_deblock,
                        );
                        if chroma_format != 0 {
                            deblock_edge(
                                h,
                                pixuv.offset((strideuv << offuv) as isize),
                                (2i32 * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(4isize)
                                    as *mut crate::stdlib::uint8_t,
                                chroma_qp[1usize],
                                a,
                                b,
                                c,
                                chroma_deblock,
                            );
                            if chroma444 != 0 {
                                deblock_edge(
                                    h,
                                    pixuv.offset(uvdiff).offset((strideuv << offuv) as isize),
                                    (2i32 * strideuv) as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(4isize)
                                        as *mut crate::stdlib::uint8_t,
                                    chroma_qp[1usize],
                                    a,
                                    b,
                                    c,
                                    chroma_deblock,
                                );
                            }
                        }
                    }
                } else {
                    let mut qpl =
                        *(*h).mb.qp.offset(((*h).mb.i_mb_xy - 1i32) as isize) as ::core::ffi::c_int;
                    let mut qp_left = qp + qpl + 1i32 >> 1i32;
                    let mut qpc_left = qpc
                        + *(*h).chroma_qp_table.offset(qpl as isize) as ::core::ffi::c_int
                        + 1i32
                        >> 1i32;
                    let mut intra_left = (*(*h).mb.type_0.offset(((*h).mb.i_mb_xy - 1i32) as isize)
                        as ::core::ffi::c_int
                        == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                        || *(*h).mb.type_0.offset(((*h).mb.i_mb_xy - 1i32) as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                        || *(*h).mb.type_0.offset(((*h).mb.i_mb_xy - 1i32) as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                        || *(*h).mb.type_0.offset(((*h).mb.i_mb_xy - 1i32) as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    let mut intra_deblock =
                        (intra_cur != 0 || intra_left != 0) as ::core::ffi::c_int;
                    if !(*(*h).fdec).mb_info.is_null()
                        && (*(&raw mut *(&raw mut *bs.offset(0isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(0isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i
                            != 0
                    {
                        let ref mut c2rust_fresh0 =
                            *(*(*h).fdec).effective_qp.offset(mb_xy as isize);
                        *c2rust_fresh0 = (*c2rust_fresh0 as ::core::ffi::c_int
                            | 0xffi32
                                * (*(*(*h).fdec).mb_info.offset(mb_xy as isize)
                                    as ::core::ffi::c_uint
                                    & crate::x264_h::X264_MBINFO_CONSTANT
                                    != 0) as ::core::ffi::c_int)
                            as crate::stdlib::uint8_t;
                        let ref mut c2rust_fresh1 = *(*(*h).fdec)
                            .effective_qp
                            .offset((*h).mb.i_mb_left_xy[0usize] as isize);
                        *c2rust_fresh1 = (*c2rust_fresh1 as ::core::ffi::c_int
                            | 0xffi32
                                * (*(*(*h).fdec)
                                    .mb_info
                                    .offset((*h).mb.i_mb_left_xy[0usize] as isize)
                                    as ::core::ffi::c_uint
                                    & crate::x264_h::X264_MBINFO_CONSTANT
                                    != 0) as ::core::ffi::c_int)
                            as crate::stdlib::uint8_t;
                    }
                    if intra_deblock != 0 {
                        if 0i32 & 1i32 == 0 || transform_8x8 == 0 {
                            deblock_edge_intra(
                                h,
                                pixy.offset(
                                    (4i32 * 0i32 * (if 0i32 != 0 { stride2y } else { 1i32 }))
                                        as isize,
                                ),
                                stride2y as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0isize)
                                    as *mut crate::stdlib::uint8_t,
                                qp_left,
                                a,
                                b,
                                0i32,
                                (*h).loopf.deblock_luma_intra[0usize],
                            );
                            if chroma_format
                                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            {
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(
                                        (4i32 * 0i32 * (if 0i32 != 0 { stride2uv } else { 1i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    0i32,
                                    (*h).loopf.deblock_luma_intra[0usize],
                                );
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(uvdiff).offset(
                                        (4i32 * 0i32 * (if 0i32 != 0 { stride2uv } else { 1i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    0i32,
                                    (*h).loopf.deblock_luma_intra[0usize],
                                );
                            } else if chroma_format
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                                && 0i32 & 1i32 == 0
                            {
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(
                                        (0i32 * (if 0i32 != 0 { 2i32 * stride2uv } else { 4i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    1i32,
                                    (*h).loopf.deblock_chroma_intra[0usize],
                                );
                            }
                        }
                        if chroma_format
                            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                            && (0i32 != 0 || 0i32 & 1i32 == 0)
                        {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (0i32 * (if 0i32 != 0 { 4i32 * stride2uv } else { 4i32 }))
                                        as isize,
                                ),
                                stride2uv as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_left,
                                a,
                                b,
                                1i32,
                                (*h).loopf.deblock_chroma_intra[0usize],
                            );
                        }
                    } else {
                        if 0i32 & 1i32 == 0 || transform_8x8 == 0 {
                            deblock_edge(
                                h,
                                pixy.offset(
                                    (4i32 * 0i32 * (if 0i32 != 0 { stride2y } else { 1i32 }))
                                        as isize,
                                ),
                                stride2y as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0isize)
                                    as *mut crate::stdlib::uint8_t,
                                qp_left,
                                a,
                                b,
                                0i32,
                                (*h).loopf.deblock_luma[0usize],
                            );
                            if chroma_format
                                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            {
                                deblock_edge(
                                    h,
                                    pixuv.offset(
                                        (4i32 * 0i32 * (if 0i32 != 0 { stride2uv } else { 1i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    0i32,
                                    (*h).loopf.deblock_luma[0usize],
                                );
                                deblock_edge(
                                    h,
                                    pixuv.offset(uvdiff).offset(
                                        (4i32 * 0i32 * (if 0i32 != 0 { stride2uv } else { 1i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    0i32,
                                    (*h).loopf.deblock_luma[0usize],
                                );
                            } else if chroma_format
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                                && 0i32 & 1i32 == 0
                            {
                                deblock_edge(
                                    h,
                                    pixuv.offset(
                                        (0i32 * (if 0i32 != 0 { 2i32 * stride2uv } else { 4i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(0isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_left,
                                    a,
                                    b,
                                    1i32,
                                    (*h).loopf.deblock_chroma[0usize],
                                );
                            }
                        }
                        if chroma_format
                            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                            && (0i32 != 0 || 0i32 & 1i32 == 0)
                        {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (0i32 * (if 0i32 != 0 { 4i32 * stride2uv } else { 4i32 }))
                                        as isize,
                                ),
                                stride2uv as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(0isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_left,
                                a,
                                b,
                                1i32,
                                (*h).loopf.deblock_chroma[0usize],
                            );
                        }
                    }
                }
            }
            if first_edge_only == 0 {
                if 1i32 & 1i32 == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4i32 * 1i32 * (if 0i32 != 0 { stride2y } else { 1i32 })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(1isize) as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0i32,
                        (*h).loopf.deblock_luma[0usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4i32 * 1i32 * (if 0i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[0usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff).offset(
                                (4i32 * 1i32 * (if 0i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[0usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 1i32 & 1i32 == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (1i32 * (if 0i32 != 0 { 2i32 * stride2uv } else { 4i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1i32,
                            (*h).loopf.deblock_chroma[0usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (0i32 != 0 || 1i32 & 1i32 == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (1i32 * (if 0i32 != 0 { 4i32 * stride2uv } else { 4i32 })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(1isize) as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1i32,
                        (*h).loopf.deblock_chroma[0usize],
                    );
                }
                if 2i32 & 1i32 == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4i32 * 2i32 * (if 0i32 != 0 { stride2y } else { 1i32 })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(2isize) as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0i32,
                        (*h).loopf.deblock_luma[0usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4i32 * 2i32 * (if 0i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[0usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff).offset(
                                (4i32 * 2i32 * (if 0i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[0usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 2i32 & 1i32 == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (2i32 * (if 0i32 != 0 { 2i32 * stride2uv } else { 4i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1i32,
                            (*h).loopf.deblock_chroma[0usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (0i32 != 0 || 2i32 & 1i32 == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (2i32 * (if 0i32 != 0 { 4i32 * stride2uv } else { 4i32 })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(2isize) as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1i32,
                        (*h).loopf.deblock_chroma[0usize],
                    );
                }
                if 3i32 & 1i32 == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4i32 * 3i32 * (if 0i32 != 0 { stride2y } else { 1i32 })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(3isize) as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0i32,
                        (*h).loopf.deblock_luma[0usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4i32 * 3i32 * (if 0i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[0usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff).offset(
                                (4i32 * 3i32 * (if 0i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[0usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 3i32 & 1i32 == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (3i32 * (if 0i32 != 0 { 2i32 * stride2uv } else { 4i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(0isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1i32,
                            (*h).loopf.deblock_chroma[0usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (0i32 != 0 || 3i32 & 1i32 == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (3i32 * (if 0i32 != 0 { 4i32 * stride2uv } else { 4i32 })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(3isize) as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1i32,
                        (*h).loopf.deblock_chroma[0usize],
                    );
                }
            }
            if (*h).mb.i_neighbour & crate::src::common::macroblock::MB_TOP != 0 {
                if b_interlaced != 0
                    && mb_y & 1i32 == 0
                    && (*h).mb.b_interlaced == 0
                    && *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
                        != 0
                {
                    let mut mbn_xy = mb_xy - 2i32 * (*h).mb.i_mb_stride;
                    let mut j = 0i32;
                    while j < 2i32 {
                        let mut qpt = *(*h).mb.qp.offset(mbn_xy as isize) as ::core::ffi::c_int;
                        let mut qp_top = qp + qpt + 1i32 >> 1i32;
                        let mut qpc_top = qpc
                            + *(*h).chroma_qp_table.offset(qpt as isize) as ::core::ffi::c_int
                            + 1i32
                            >> 1i32;
                        let mut intra_top = (*(*h).mb.type_0.offset(mbn_xy as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset(mbn_xy as isize) as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset(mbn_xy as isize) as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                            || *(*h).mb.type_0.offset(mbn_xy as isize) as ::core::ffi::c_int
                                == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                        if intra_cur != 0 || intra_top != 0 {
                            (*(&raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset((4i32 * j) as isize)
                                as *mut crate::src::common::base::x264_union32_t))
                                .i = 0x3030303u32;
                        }
                        deblock_edge(
                            h,
                            pixy.offset((j * stridey) as isize),
                            (2i32 * stridey) as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset((4i32 * j) as isize)
                                as *mut crate::stdlib::uint8_t,
                            qp_top,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[1usize],
                        );
                        if chroma444 != 0 {
                            deblock_edge(
                                h,
                                pixuv.offset((j * strideuv) as isize),
                                (2i32 * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset((4i32 * j) as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_top,
                                a,
                                b,
                                0i32,
                                (*h).loopf.deblock_luma[1usize],
                            );
                            deblock_edge(
                                h,
                                pixuv.offset(uvdiff).offset((j * strideuv) as isize),
                                (2i32 * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset((4i32 * j) as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_top,
                                a,
                                b,
                                0i32,
                                (*h).loopf.deblock_luma[1usize],
                            );
                        } else if chroma_format != 0 {
                            deblock_edge(
                                h,
                                pixuv.offset((j * strideuv) as isize),
                                (2i32 * strideuv) as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset((4i32 * j) as isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_top,
                                a,
                                b,
                                1i32,
                                (*h).loopf.deblock_chroma[1usize],
                            );
                        }
                        j += 1;
                        mbn_xy += (*h).mb.i_mb_stride;
                    }
                } else {
                    let mut qpt_0 =
                        *(*h).mb.qp.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int;
                    let mut qp_top_0 = qp + qpt_0 + 1i32 >> 1i32;
                    let mut qpc_top_0 = qpc
                        + *(*h).chroma_qp_table.offset(qpt_0 as isize) as ::core::ffi::c_int
                        + 1i32
                        >> 1i32;
                    let mut intra_top_0 = (*(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize)
                        as ::core::ffi::c_int
                        == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
                        || *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
                        || *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
                        || *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize)
                            as ::core::ffi::c_int
                            == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                    let mut intra_deblock_0 =
                        (intra_cur != 0 || intra_top_0 != 0) as ::core::ffi::c_int;
                    if !(*(*h).fdec).mb_info.is_null()
                        && (*(&raw mut *(&raw mut *bs.offset(1isize)
                            as *mut [crate::stdlib::uint8_t; 4])
                            .offset(0isize)
                            as *mut crate::src::common::base::x264_union32_t))
                            .i
                            != 0
                    {
                        let ref mut c2rust_fresh2 =
                            *(*(*h).fdec).effective_qp.offset(mb_xy as isize);
                        *c2rust_fresh2 = (*c2rust_fresh2 as ::core::ffi::c_int
                            | 0xffi32
                                * (*(*(*h).fdec).mb_info.offset(mb_xy as isize)
                                    as ::core::ffi::c_uint
                                    & crate::x264_h::X264_MBINFO_CONSTANT
                                    != 0) as ::core::ffi::c_int)
                            as crate::stdlib::uint8_t;
                        let ref mut c2rust_fresh3 = *(*(*h).fdec)
                            .effective_qp
                            .offset((*h).mb.i_mb_top_xy as isize);
                        *c2rust_fresh3 = (*c2rust_fresh3 as ::core::ffi::c_int
                            | 0xffi32
                                * (*(*(*h).fdec).mb_info.offset((*h).mb.i_mb_top_xy as isize)
                                    as ::core::ffi::c_uint
                                    & crate::x264_h::X264_MBINFO_CONSTANT
                                    != 0) as ::core::ffi::c_int)
                            as crate::stdlib::uint8_t;
                    }
                    if (b_interlaced == 0
                        || (*h).mb.b_interlaced == 0
                            && *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) == 0)
                        && intra_deblock_0 != 0
                    {
                        if 0i32 & 1i32 == 0 || transform_8x8 == 0 {
                            deblock_edge_intra(
                                h,
                                pixy.offset(
                                    (4i32 * 0i32 * (if 1i32 != 0 { stride2y } else { 1i32 }))
                                        as isize,
                                ),
                                stride2y as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0isize)
                                    as *mut crate::stdlib::uint8_t,
                                qp_top_0,
                                a,
                                b,
                                0i32,
                                (*h).loopf.deblock_luma_intra[1usize],
                            );
                            if chroma_format
                                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            {
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(
                                        (4i32 * 0i32 * (if 1i32 != 0 { stride2uv } else { 1i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    0i32,
                                    (*h).loopf.deblock_luma_intra[1usize],
                                );
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(uvdiff).offset(
                                        (4i32 * 0i32 * (if 1i32 != 0 { stride2uv } else { 1i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    0i32,
                                    (*h).loopf.deblock_luma_intra[1usize],
                                );
                            } else if chroma_format
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                                && 0i32 & 1i32 == 0
                            {
                                deblock_edge_intra(
                                    h,
                                    pixuv.offset(
                                        (0i32 * (if 1i32 != 0 { 2i32 * stride2uv } else { 4i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    1i32,
                                    (*h).loopf.deblock_chroma_intra[1usize],
                                );
                            }
                        }
                        if chroma_format
                            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                            && (1i32 != 0 || 0i32 & 1i32 == 0)
                        {
                            deblock_edge_intra(
                                h,
                                pixuv.offset(
                                    (0i32 * (if 1i32 != 0 { 4i32 * stride2uv } else { 4i32 }))
                                        as isize,
                                ),
                                stride2uv as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_top_0,
                                a,
                                b,
                                1i32,
                                (*h).loopf.deblock_chroma_intra[1usize],
                            );
                        }
                    } else {
                        if intra_deblock_0 != 0 {
                            (*(&raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(0isize)
                                as *mut crate::src::common::base::x264_union32_t))
                                .i = 0x3030303u32;
                        }
                        if 0i32 & 1i32 == 0 || transform_8x8 == 0 {
                            deblock_edge(
                                h,
                                pixy.offset(
                                    (4i32 * 0i32 * (if 1i32 != 0 { stride2y } else { 1i32 }))
                                        as isize,
                                ),
                                stride2y as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0isize)
                                    as *mut crate::stdlib::uint8_t,
                                qp_top_0,
                                a,
                                b,
                                0i32,
                                (*h).loopf.deblock_luma[1usize],
                            );
                            if chroma_format
                                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                            {
                                deblock_edge(
                                    h,
                                    pixuv.offset(
                                        (4i32 * 0i32 * (if 1i32 != 0 { stride2uv } else { 1i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    0i32,
                                    (*h).loopf.deblock_luma[1usize],
                                );
                                deblock_edge(
                                    h,
                                    pixuv.offset(uvdiff).offset(
                                        (4i32 * 0i32 * (if 1i32 != 0 { stride2uv } else { 1i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    0i32,
                                    (*h).loopf.deblock_luma[1usize],
                                );
                            } else if chroma_format
                                == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                                && 0i32 & 1i32 == 0
                            {
                                deblock_edge(
                                    h,
                                    pixuv.offset(
                                        (0i32 * (if 1i32 != 0 { 2i32 * stride2uv } else { 4i32 }))
                                            as isize,
                                    ),
                                    stride2uv as crate::stdlib::intptr_t,
                                    &raw mut *(&raw mut *bs.offset(1isize)
                                        as *mut [crate::stdlib::uint8_t; 4])
                                        .offset(0isize)
                                        as *mut crate::stdlib::uint8_t,
                                    qpc_top_0,
                                    a,
                                    b,
                                    1i32,
                                    (*h).loopf.deblock_chroma[1usize],
                                );
                            }
                        }
                        if chroma_format
                            == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                            && (1i32 != 0 || 0i32 & 1i32 == 0)
                        {
                            deblock_edge(
                                h,
                                pixuv.offset(
                                    (0i32 * (if 1i32 != 0 { 4i32 * stride2uv } else { 4i32 }))
                                        as isize,
                                ),
                                stride2uv as crate::stdlib::intptr_t,
                                &raw mut *(&raw mut *bs.offset(1isize)
                                    as *mut [crate::stdlib::uint8_t; 4])
                                    .offset(0isize)
                                    as *mut crate::stdlib::uint8_t,
                                qpc_top_0,
                                a,
                                b,
                                1i32,
                                (*h).loopf.deblock_chroma[1usize],
                            );
                        }
                    }
                }
            }
            if first_edge_only == 0 {
                if 1i32 & 1i32 == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4i32 * 1i32 * (if 1i32 != 0 { stride2y } else { 1i32 })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(1isize) as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0i32,
                        (*h).loopf.deblock_luma[1usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4i32 * 1i32 * (if 1i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[1usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff).offset(
                                (4i32 * 1i32 * (if 1i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[1usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 1i32 & 1i32 == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (1i32 * (if 1i32 != 0 { 2i32 * stride2uv } else { 4i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(1isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1i32,
                            (*h).loopf.deblock_chroma[1usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (1i32 != 0 || 1i32 & 1i32 == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (1i32 * (if 1i32 != 0 { 4i32 * stride2uv } else { 4i32 })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(1isize) as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1i32,
                        (*h).loopf.deblock_chroma[1usize],
                    );
                }
                if 2i32 & 1i32 == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4i32 * 2i32 * (if 1i32 != 0 { stride2y } else { 1i32 })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(2isize) as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0i32,
                        (*h).loopf.deblock_luma[1usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4i32 * 2i32 * (if 1i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[1usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff).offset(
                                (4i32 * 2i32 * (if 1i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[1usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 2i32 & 1i32 == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (2i32 * (if 1i32 != 0 { 2i32 * stride2uv } else { 4i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(2isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1i32,
                            (*h).loopf.deblock_chroma[1usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (1i32 != 0 || 2i32 & 1i32 == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (2i32 * (if 1i32 != 0 { 4i32 * stride2uv } else { 4i32 })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(2isize) as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1i32,
                        (*h).loopf.deblock_chroma[1usize],
                    );
                }
                if 3i32 & 1i32 == 0 || transform_8x8 == 0 {
                    deblock_edge(
                        h,
                        pixy.offset(
                            (4i32 * 3i32 * (if 1i32 != 0 { stride2y } else { 1i32 })) as isize,
                        ),
                        stride2y as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(3isize) as *mut crate::stdlib::uint8_t,
                        qp,
                        a,
                        b,
                        0i32,
                        (*h).loopf.deblock_luma[1usize],
                    );
                    if chroma_format == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (4i32 * 3i32 * (if 1i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[1usize],
                        );
                        deblock_edge(
                            h,
                            pixuv.offset(uvdiff).offset(
                                (4i32 * 3i32 * (if 1i32 != 0 { stride2uv } else { 1i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            0i32,
                            (*h).loopf.deblock_luma[1usize],
                        );
                    } else if chroma_format
                        == crate::src::common::base::CHROMA_420 as ::core::ffi::c_int
                        && 3i32 & 1i32 == 0
                    {
                        deblock_edge(
                            h,
                            pixuv.offset(
                                (3i32 * (if 1i32 != 0 { 2i32 * stride2uv } else { 4i32 })) as isize,
                            ),
                            stride2uv as crate::stdlib::intptr_t,
                            &raw mut *(&raw mut *bs.offset(1isize)
                                as *mut [crate::stdlib::uint8_t; 4])
                                .offset(3isize)
                                as *mut crate::stdlib::uint8_t,
                            qpc,
                            a,
                            b,
                            1i32,
                            (*h).loopf.deblock_chroma[1usize],
                        );
                    }
                }
                if chroma_format == crate::src::common::base::CHROMA_422 as ::core::ffi::c_int
                    && (1i32 != 0 || 3i32 & 1i32 == 0)
                {
                    deblock_edge(
                        h,
                        pixuv.offset(
                            (3i32 * (if 1i32 != 0 { 4i32 * stride2uv } else { 4i32 })) as isize,
                        ),
                        stride2uv as crate::stdlib::intptr_t,
                        &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                            .offset(3isize) as *mut crate::stdlib::uint8_t,
                        qpc,
                        a,
                        b,
                        1i32,
                        (*h).loopf.deblock_chroma[1usize],
                    );
                }
            }
            mb_x += (!b_interlaced | mb_y) & 1i32;
            mb_y ^= b_interlaced;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_macroblock_deblock(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut a = (*h).sh.i_alpha_c0_offset - crate::src::common::common::QP_BD_OFFSET;
        let mut b = (*h).sh.i_beta_offset - crate::src::common::common::QP_BD_OFFSET;
        let mut qp_thresh = 15i32
            - (if a < b { a } else { b })
            - (if 0i32
                > (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                    .i_chroma_qp_index_offset
            {
                0i32
            } else {
                (*(&raw mut (*h).pps as *mut crate::src::common::set::x264_pps_t))
                    .i_chroma_qp_index_offset
            });
        let mut intra_cur = ((*h).mb.i_type
            == crate::src::common::macroblock::I_4x4 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_8x8 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_16x16 as ::core::ffi::c_int
            || (*h).mb.i_type == crate::src::common::macroblock::I_PCM as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut qp = (*h).mb.i_qp;
        let mut qpc = (*h).mb.i_chroma_qp;
        if (*h).mb.i_partition == crate::src::common::macroblock::D_16x16 as ::core::ffi::c_int
            && (*h).mb.i_cbp_luma == 0
            && intra_cur == 0
            || qp <= qp_thresh
        {
            return;
        }
        let mut bs = (*h).mb.cache.deblock_strength;
        if intra_cur != 0 {
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
        } else {
            (*h).loopf
                .deblock_strength
                .expect("non-null function pointer")(
                &raw mut (*h).mb.cache.non_zero_count as *mut crate::stdlib::uint8_t,
                &raw mut (*h).mb.cache.ref_0 as *mut [crate::stdlib::int8_t; 40],
                &raw mut (*h).mb.cache.mv as *mut [[crate::stdlib::int16_t; 2]; 40],
                bs,
                4i32 >> (*h).mb.b_interlaced,
                ((*h).sh.i_type == crate::src::common::base::SLICE_TYPE_B as ::core::ffi::c_int)
                    as ::core::ffi::c_int,
            );
        }
        let mut transform_8x8 = (*h).mb.b_transform_8x8;
        if transform_8x8 == 0 {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[0usize].offset(
                    (4i32
                        * 1i32
                        * (if 0i32 != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1i32
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(1isize) as *mut crate::stdlib::uint8_t,
                qp,
                a,
                b,
                0i32,
                (*h).loopf.deblock_luma[0usize],
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[1usize].offset(
                        (4i32
                            * 1i32
                            * (if 0i32 != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1i32
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                        .offset(1isize) as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0i32,
                    (*h).loopf.deblock_luma[0usize],
                );
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[2usize].offset(
                        (4i32
                            * 1i32
                            * (if 0i32 != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1i32
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                        .offset(1isize) as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0i32,
                    (*h).loopf.deblock_luma[0usize],
                );
            }
        }
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[0usize].offset(
                (4i32
                    * 2i32
                    * (if 0i32 != 0 {
                        crate::src::common::common::FDEC_STRIDE
                    } else {
                        1i32
                    })) as isize,
            ),
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                .offset(2isize) as *mut crate::stdlib::uint8_t,
            qp,
            a,
            b,
            0i32,
            (*h).loopf.deblock_luma[0usize],
        );
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[1usize].offset(
                    (4i32
                        * 2i32
                        * (if 0i32 != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1i32
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2isize) as *mut crate::stdlib::uint8_t,
                qpc,
                a,
                b,
                0i32,
                (*h).loopf.deblock_luma[0usize],
            );
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[2usize].offset(
                    (4i32
                        * 2i32
                        * (if 0i32 != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1i32
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2isize) as *mut crate::stdlib::uint8_t,
                qpc,
                a,
                b,
                0i32,
                (*h).loopf.deblock_luma[0usize],
            );
        }
        if transform_8x8 == 0 {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[0usize].offset(
                    (4i32
                        * 3i32
                        * (if 0i32 != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1i32
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(3isize) as *mut crate::stdlib::uint8_t,
                qp,
                a,
                b,
                0i32,
                (*h).loopf.deblock_luma[0usize],
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[1usize].offset(
                        (4i32
                            * 3i32
                            * (if 0i32 != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1i32
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                        .offset(3isize) as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0i32,
                    (*h).loopf.deblock_luma[0usize],
                );
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[2usize].offset(
                        (4i32
                            * 3i32
                            * (if 0i32 != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1i32
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(0isize) as *mut [crate::stdlib::uint8_t; 4])
                        .offset(3isize) as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0i32,
                    (*h).loopf.deblock_luma[0usize],
                );
            }
        }
        if transform_8x8 == 0 {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[0usize].offset(
                    (4i32
                        * 1i32
                        * (if 1i32 != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1i32
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(1isize) as *mut crate::stdlib::uint8_t,
                qp,
                a,
                b,
                0i32,
                (*h).loopf.deblock_luma[1usize],
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[1usize].offset(
                        (4i32
                            * 1i32
                            * (if 1i32 != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1i32
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                        .offset(1isize) as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0i32,
                    (*h).loopf.deblock_luma[1usize],
                );
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[2usize].offset(
                        (4i32
                            * 1i32
                            * (if 1i32 != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1i32
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                        .offset(1isize) as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0i32,
                    (*h).loopf.deblock_luma[1usize],
                );
            }
        }
        deblock_edge(
            h,
            (*h).mb.pic.p_fdec[0usize].offset(
                (4i32
                    * 2i32
                    * (if 1i32 != 0 {
                        crate::src::common::common::FDEC_STRIDE
                    } else {
                        1i32
                    })) as isize,
            ),
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                .offset(2isize) as *mut crate::stdlib::uint8_t,
            qp,
            a,
            b,
            0i32,
            (*h).loopf.deblock_luma[1usize],
        );
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[1usize].offset(
                    (4i32
                        * 2i32
                        * (if 1i32 != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1i32
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2isize) as *mut crate::stdlib::uint8_t,
                qpc,
                a,
                b,
                0i32,
                (*h).loopf.deblock_luma[1usize],
            );
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[2usize].offset(
                    (4i32
                        * 2i32
                        * (if 1i32 != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1i32
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(2isize) as *mut crate::stdlib::uint8_t,
                qpc,
                a,
                b,
                0i32,
                (*h).loopf.deblock_luma[1usize],
            );
        }
        if transform_8x8 == 0 {
            deblock_edge(
                h,
                (*h).mb.pic.p_fdec[0usize].offset(
                    (4i32
                        * 3i32
                        * (if 1i32 != 0 {
                            crate::src::common::common::FDEC_STRIDE
                        } else {
                            1i32
                        })) as isize,
                ),
                crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                    .offset(3isize) as *mut crate::stdlib::uint8_t,
                qp,
                a,
                b,
                0i32,
                (*h).loopf.deblock_luma[1usize],
            );
            if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[1usize].offset(
                        (4i32
                            * 3i32
                            * (if 1i32 != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1i32
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                        .offset(3isize) as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0i32,
                    (*h).loopf.deblock_luma[1usize],
                );
                deblock_edge(
                    h,
                    (*h).mb.pic.p_fdec[2usize].offset(
                        (4i32
                            * 3i32
                            * (if 1i32 != 0 {
                                crate::src::common::common::FDEC_STRIDE
                            } else {
                                1i32
                            })) as isize,
                    ),
                    crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
                    &raw mut *(&raw mut *bs.offset(1isize) as *mut [crate::stdlib::uint8_t; 4])
                        .offset(3isize) as *mut crate::stdlib::uint8_t,
                    qpc,
                    a,
                    b,
                    0i32,
                    (*h).loopf.deblock_luma[1usize],
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_deblock_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pf: *mut crate::src::common::frame::x264_deblock_function_t,
    mut _b_mbaff: ::core::ffi::c_int,
) {
    unsafe {
        (*pf).deblock_luma[1usize] = Some(
            deblock_v_luma_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        );
        (*pf).deblock_luma[0usize] = Some(
            deblock_h_luma_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        );
        (*pf).deblock_chroma[1usize] = Some(
            deblock_v_chroma_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        );
        (*pf).deblock_h_chroma_420 = Some(
            deblock_h_chroma_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        );
        (*pf).deblock_h_chroma_422 = Some(
            deblock_h_chroma_422_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        );
        (*pf).deblock_luma_intra[1usize] = Some(
            deblock_v_luma_intra_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).deblock_luma_intra[0usize] = Some(
            deblock_h_luma_intra_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).deblock_chroma_intra[1usize] = Some(
            deblock_v_chroma_intra_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).deblock_h_chroma_420_intra = Some(
            deblock_h_chroma_intra_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).deblock_h_chroma_422_intra = Some(
            deblock_h_chroma_422_intra_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).deblock_luma_mbaff = Some(
            deblock_h_luma_mbaff_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        );
        (*pf).deblock_chroma_420_mbaff = Some(
            deblock_h_chroma_mbaff_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::int8_t,
                ) -> (),
        );
        (*pf).deblock_luma_intra_mbaff = Some(
            deblock_h_luma_intra_mbaff_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).deblock_chroma_420_intra_mbaff = Some(
            deblock_h_chroma_intra_mbaff_c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).deblock_strength = Some(
            deblock_strength_c
                as unsafe extern "C" fn(
                    *mut crate::stdlib::uint8_t,
                    *mut [crate::stdlib::int8_t; 40],
                    *mut [[crate::stdlib::int16_t; 2]; 40],
                    *mut [[crate::stdlib::uint8_t; 4]; 8],
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> (),
        );
        (*pf).deblock_chroma_422_mbaff = (*pf).deblock_h_chroma_420;
        (*pf).deblock_chroma_422_intra_mbaff = (*pf).deblock_h_chroma_420_intra;
    }
}
