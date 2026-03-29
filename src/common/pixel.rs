// =============== BEGIN pixel_h ================
pub type x264_pixel_cmp_t = Option<
    unsafe extern "C" fn(
        *mut crate::src::common::common::pixel,
        crate::stdlib::intptr_t,
        *mut crate::src::common::common::pixel,
        crate::stdlib::intptr_t,
    ) -> ::core::ffi::c_int,
>;
pub type x264_pixel_cmp_x3_t = Option<
    unsafe extern "C" fn(
        *mut crate::src::common::common::pixel,
        *mut crate::src::common::common::pixel,
        *mut crate::src::common::common::pixel,
        *mut crate::src::common::common::pixel,
        crate::stdlib::intptr_t,
        *mut ::core::ffi::c_int,
    ) -> (),
>;
pub type x264_pixel_cmp_x4_t = Option<
    unsafe extern "C" fn(
        *mut crate::src::common::common::pixel,
        *mut crate::src::common::common::pixel,
        *mut crate::src::common::common::pixel,
        *mut crate::src::common::common::pixel,
        *mut crate::src::common::common::pixel,
        crate::stdlib::intptr_t,
        *mut ::core::ffi::c_int,
    ) -> (),
>;
pub const PIXEL_16x16: crate::stdlib::C2Rust_Unnamed_7 = 0;
pub const PIXEL_16x8: crate::stdlib::C2Rust_Unnamed_7 = 1;
pub const PIXEL_8x16: crate::stdlib::C2Rust_Unnamed_7 = 2;
pub const PIXEL_8x8: crate::stdlib::C2Rust_Unnamed_7 = 3;
pub const PIXEL_8x4: crate::stdlib::C2Rust_Unnamed_7 = 4;
pub const PIXEL_4x8: crate::stdlib::C2Rust_Unnamed_7 = 5;
pub const PIXEL_4x4: crate::stdlib::C2Rust_Unnamed_7 = 6;
pub const PIXEL_4x16: crate::stdlib::C2Rust_Unnamed_7 = 7;
pub const PIXEL_4x2: crate::stdlib::C2Rust_Unnamed_7 = 8;
pub const PIXEL_2x8: crate::stdlib::C2Rust_Unnamed_7 = 9;
pub const PIXEL_2x4: crate::stdlib::C2Rust_Unnamed_7 = 10;
pub const PIXEL_2x2: crate::stdlib::C2Rust_Unnamed_7 = 11;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub w: crate::stdlib::uint8_t,
    pub h: crate::stdlib::uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_pixel_function_t {
    pub sad: [crate::src::common::pixel::x264_pixel_cmp_t; 8],
    pub ssd: [crate::src::common::pixel::x264_pixel_cmp_t; 8],
    pub satd: [crate::src::common::pixel::x264_pixel_cmp_t; 8],
    pub ssim: [crate::src::common::pixel::x264_pixel_cmp_t; 7],
    pub sa8d: [crate::src::common::pixel::x264_pixel_cmp_t; 4],
    pub mbcmp: [crate::src::common::pixel::x264_pixel_cmp_t; 8],
    pub mbcmp_unaligned: [crate::src::common::pixel::x264_pixel_cmp_t; 8],
    pub fpelcmp: [crate::src::common::pixel::x264_pixel_cmp_t; 8],
    pub fpelcmp_x3: [crate::src::common::pixel::x264_pixel_cmp_x3_t; 7],
    pub fpelcmp_x4: [crate::src::common::pixel::x264_pixel_cmp_x4_t; 7],
    pub sad_aligned: [crate::src::common::pixel::x264_pixel_cmp_t; 8],
    pub vsad: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub asd8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub sa8d_satd: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> crate::stdlib::uint64_t,
    >; 1],
    pub var: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> crate::stdlib::uint64_t,
    >; 4],
    pub var2: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >; 4],
    pub hadamard_ac: [Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
        ) -> crate::stdlib::uint64_t,
    >; 4],
    pub ssd_nv12_core: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut crate::stdlib::uint64_t,
            *mut crate::stdlib::uint64_t,
        ) -> (),
    >,
    pub ssim_4x4x2_core: Option<
        unsafe extern "C" fn(
            *const crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *const crate::src::common::common::pixel,
            crate::stdlib::intptr_t,
            *mut [::core::ffi::c_int; 4],
        ) -> (),
    >,
    pub ssim_end4: Option<
        unsafe extern "C" fn(
            *mut [::core::ffi::c_int; 4],
            *mut [::core::ffi::c_int; 4],
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_float,
    >,
    pub sad_x3: [crate::src::common::pixel::x264_pixel_cmp_x3_t; 7],
    pub sad_x4: [crate::src::common::pixel::x264_pixel_cmp_x4_t; 7],
    pub satd_x3: [crate::src::common::pixel::x264_pixel_cmp_x3_t; 7],
    pub satd_x4: [crate::src::common::pixel::x264_pixel_cmp_x4_t; 7],
    pub ads: [Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_int,
            *mut crate::stdlib::uint16_t,
            ::core::ffi::c_int,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::int16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >; 7],
    pub intra_mbcmp_x3_16x16: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_satd_x3_16x16: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_sad_x3_16x16: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_mbcmp_x3_4x4: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_satd_x3_4x4: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_sad_x3_4x4: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_mbcmp_x3_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_satd_x3_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_sad_x3_chroma: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_mbcmp_x3_8x16c: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_satd_x3_8x16c: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_sad_x3_8x16c: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_mbcmp_x3_8x8c: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_satd_x3_8x8c: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_sad_x3_8x8c: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_mbcmp_x3_8x8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_sa8d_x3_8x8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_sad_x3_8x8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut ::core::ffi::c_int,
        ) -> (),
    >,
    pub intra_mbcmp_x9_4x4: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::uint16_t,
        ) -> ::core::ffi::c_int,
    >,
    pub intra_satd_x9_4x4: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::uint16_t,
        ) -> ::core::ffi::c_int,
    >,
    pub intra_sad_x9_4x4: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::uint16_t,
        ) -> ::core::ffi::c_int,
    >,
    pub intra_mbcmp_x9_8x8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
        ) -> ::core::ffi::c_int,
    >,
    pub intra_sa8d_x9_8x8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
        ) -> ::core::ffi::c_int,
    >,
    pub intra_sad_x9_8x8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::stdlib::uint16_t,
            *mut crate::stdlib::uint16_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type sum2_t = crate::stdlib::uint32_t;
pub type sum_t = crate::stdlib::uint16_t;
unsafe extern "C" fn x264_pixel_sad_16x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 16 as ::core::ffi::c_int {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_sad_16x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 16 as ::core::ffi::c_int {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_sad_8x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_sad_8x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_sad_8x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 4 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_sad_4x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 4 as ::core::ffi::c_int {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_sad_4x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 4 as ::core::ffi::c_int {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_sad_4x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 4 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 4 as ::core::ffi::c_int {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_ssd_16x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 16 as ::core::ffi::c_int {
                let mut d: ::core::ffi::c_int = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_ssd_16x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 16 as ::core::ffi::c_int {
                let mut d: ::core::ffi::c_int = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_ssd_8x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                let mut d: ::core::ffi::c_int = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_ssd_8x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                let mut d: ::core::ffi::c_int = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_ssd_8x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 4 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                let mut d: ::core::ffi::c_int = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_ssd_4x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 4 as ::core::ffi::c_int {
                let mut d: ::core::ffi::c_int = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_ssd_4x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 4 as ::core::ffi::c_int {
                let mut d: ::core::ffi::c_int = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
unsafe extern "C" fn x264_pixel_ssd_4x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 4 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 4 as ::core::ffi::c_int {
                let mut d: ::core::ffi::c_int = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1 as isize);
            pix2 = pix2.offset(i_stride_pix2 as isize);
            y += 1;
        }
        return i_sum;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pixel_ssd_wxh(
    mut pf: *mut crate::src::common::pixel::x264_pixel_function_t,
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut i_ssd: crate::stdlib::uint64_t = 0 as crate::stdlib::uint64_t;
        let mut y: ::core::ffi::c_int = 0;
        let mut align: ::core::ffi::c_int =
            ((pix1 as crate::stdlib::intptr_t | pix2 as crate::stdlib::intptr_t | i_pix1 | i_pix2)
                & 15 as ::core::ffi::c_int as crate::stdlib::intptr_t
                == 0) as ::core::ffi::c_int;
        y = 0 as ::core::ffi::c_int;
        while y < i_height - 15 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if align != 0 {
                while x < i_width - 15 as ::core::ffi::c_int {
                    i_ssd = i_ssd.wrapping_add((*pf).ssd
                        [crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        pix1.offset((y as crate::stdlib::intptr_t * i_pix1) as isize)
                            .offset(x as isize),
                        i_pix1,
                        pix2.offset((y as crate::stdlib::intptr_t * i_pix2) as isize)
                            .offset(x as isize),
                        i_pix2,
                    ) as crate::stdlib::uint64_t);
                    x += 16 as ::core::ffi::c_int;
                }
            }
            while x < i_width - 7 as ::core::ffi::c_int {
                i_ssd = i_ssd.wrapping_add((*pf).ssd
                    [crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(
                    pix1.offset((y as crate::stdlib::intptr_t * i_pix1) as isize)
                        .offset(x as isize),
                    i_pix1,
                    pix2.offset((y as crate::stdlib::intptr_t * i_pix2) as isize)
                        .offset(x as isize),
                    i_pix2,
                ) as crate::stdlib::uint64_t);
                x += 8 as ::core::ffi::c_int;
            }
            y += 16 as ::core::ffi::c_int;
        }
        if y < i_height - 7 as ::core::ffi::c_int {
            let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_0 < i_width - 7 as ::core::ffi::c_int {
                i_ssd = i_ssd.wrapping_add((*pf).ssd
                    [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(
                    pix1.offset((y as crate::stdlib::intptr_t * i_pix1) as isize)
                        .offset(x_0 as isize),
                    i_pix1,
                    pix2.offset((y as crate::stdlib::intptr_t * i_pix2) as isize)
                        .offset(x_0 as isize),
                    i_pix2,
                ) as crate::stdlib::uint64_t);
                x_0 += 8 as ::core::ffi::c_int;
            }
        }
        if i_width & 7 as ::core::ffi::c_int != 0 {
            y = 0 as ::core::ffi::c_int;
            while y < i_height & !(7 as ::core::ffi::c_int) {
                let mut x_1: ::core::ffi::c_int = i_width & !(7 as ::core::ffi::c_int);
                while x_1 < i_width {
                    let mut d: ::core::ffi::c_int = *pix1.offset(
                        (y as crate::stdlib::intptr_t * i_pix1 + x_1 as crate::stdlib::intptr_t)
                            as isize,
                    ) as ::core::ffi::c_int
                        - *pix2.offset(
                            (y as crate::stdlib::intptr_t * i_pix2 + x_1 as crate::stdlib::intptr_t)
                                as isize,
                        ) as ::core::ffi::c_int;
                    i_ssd = i_ssd.wrapping_add((d * d) as crate::stdlib::uint64_t);
                    x_1 += 1;
                }
                y += 1;
            }
        }
        if i_height & 7 as ::core::ffi::c_int != 0 {
            y = i_height & !(7 as ::core::ffi::c_int);
            while y < i_height {
                let mut x_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x_2 < i_width {
                    let mut d_0: ::core::ffi::c_int = *pix1.offset(
                        (y as crate::stdlib::intptr_t * i_pix1 + x_2 as crate::stdlib::intptr_t)
                            as isize,
                    ) as ::core::ffi::c_int
                        - *pix2.offset(
                            (y as crate::stdlib::intptr_t * i_pix2 + x_2 as crate::stdlib::intptr_t)
                                as isize,
                        ) as ::core::ffi::c_int;
                    i_ssd = i_ssd.wrapping_add((d_0 * d_0) as crate::stdlib::uint64_t);
                    x_2 += 1;
                }
                y += 1;
            }
        }
        return i_ssd;
    }
}
unsafe extern "C" fn pixel_ssd_nv12_core(
    mut pixuv1: *mut crate::src::common::common::pixel,
    mut stride1: crate::stdlib::intptr_t,
    mut pixuv2: *mut crate::src::common::common::pixel,
    mut stride2: crate::stdlib::intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut ssd_u: *mut crate::stdlib::uint64_t,
    mut ssd_v: *mut crate::stdlib::uint64_t,
) {
    unsafe {
        *ssd_u = 0 as crate::stdlib::uint64_t;
        *ssd_v = 0 as crate::stdlib::uint64_t;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < height {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < width {
                let mut du: ::core::ffi::c_int = *pixuv1
                    .offset((2 as ::core::ffi::c_int * x) as isize)
                    as ::core::ffi::c_int
                    - *pixuv2.offset((2 as ::core::ffi::c_int * x) as isize) as ::core::ffi::c_int;
                let mut dv: ::core::ffi::c_int = *pixuv1
                    .offset((2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    - *pixuv2
                        .offset((2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int;
                *ssd_u = (*ssd_u).wrapping_add((du * du) as crate::stdlib::uint64_t);
                *ssd_v = (*ssd_v).wrapping_add((dv * dv) as crate::stdlib::uint64_t);
                x += 1;
            }
            y += 1;
            pixuv1 = pixuv1.offset(stride1 as isize);
            pixuv2 = pixuv2.offset(stride2 as isize);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pixel_ssd_nv12(
    mut pf: *mut crate::src::common::pixel::x264_pixel_function_t,
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
    mut ssd_u: *mut crate::stdlib::uint64_t,
    mut ssd_v: *mut crate::stdlib::uint64_t,
) {
    unsafe {
        (*pf).ssd_nv12_core.expect("non-null function pointer")(
            pix1,
            i_pix1,
            pix2,
            i_pix2,
            i_width & !(7 as ::core::ffi::c_int),
            i_height,
            ssd_u,
            ssd_v,
        );
        if i_width & 7 as ::core::ffi::c_int != 0 {
            let mut tmp: [crate::stdlib::uint64_t; 2] = [0; 2];
            pixel_ssd_nv12_core(
                pix1.offset((i_width & !(7 as ::core::ffi::c_int)) as isize),
                i_pix1,
                pix2.offset((i_width & !(7 as ::core::ffi::c_int)) as isize),
                i_pix2,
                i_width & 7 as ::core::ffi::c_int,
                i_height,
                (&raw mut tmp as *mut crate::stdlib::uint64_t)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint64_t,
                (&raw mut tmp as *mut crate::stdlib::uint64_t)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut crate::stdlib::uint64_t,
            );
            *ssd_u = (*ssd_u).wrapping_add(tmp[0 as ::core::ffi::c_int as usize]);
            *ssd_v = (*ssd_v).wrapping_add(tmp[1 as ::core::ffi::c_int as usize]);
        }
    }
}
unsafe extern "C" fn pixel_var_16x16(
    mut pix: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum: crate::stdlib::uint32_t = 0 as crate::stdlib::uint32_t;
        let mut sqr: crate::stdlib::uint32_t = 0 as crate::stdlib::uint32_t;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 16 as ::core::ffi::c_int {
                sum = sum.wrapping_add(*pix.offset(x as isize) as crate::stdlib::uint32_t);
                sqr = sqr.wrapping_add(
                    (*pix.offset(x as isize) as ::core::ffi::c_int
                        * *pix.offset(x as isize) as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                );
                x += 1;
            }
            pix = pix.offset(i_stride as isize);
            y += 1;
        }
        return (sum as crate::stdlib::uint64_t)
            .wrapping_add((sqr as crate::stdlib::uint64_t) << 32 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn pixel_var_8x16(
    mut pix: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum: crate::stdlib::uint32_t = 0 as crate::stdlib::uint32_t;
        let mut sqr: crate::stdlib::uint32_t = 0 as crate::stdlib::uint32_t;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                sum = sum.wrapping_add(*pix.offset(x as isize) as crate::stdlib::uint32_t);
                sqr = sqr.wrapping_add(
                    (*pix.offset(x as isize) as ::core::ffi::c_int
                        * *pix.offset(x as isize) as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                );
                x += 1;
            }
            pix = pix.offset(i_stride as isize);
            y += 1;
        }
        return (sum as crate::stdlib::uint64_t)
            .wrapping_add((sqr as crate::stdlib::uint64_t) << 32 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn pixel_var_8x8(
    mut pix: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum: crate::stdlib::uint32_t = 0 as crate::stdlib::uint32_t;
        let mut sqr: crate::stdlib::uint32_t = 0 as crate::stdlib::uint32_t;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                sum = sum.wrapping_add(*pix.offset(x as isize) as crate::stdlib::uint32_t);
                sqr = sqr.wrapping_add(
                    (*pix.offset(x as isize) as ::core::ffi::c_int
                        * *pix.offset(x as isize) as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                );
                x += 1;
            }
            pix = pix.offset(i_stride as isize);
            y += 1;
        }
        return (sum as crate::stdlib::uint64_t)
            .wrapping_add((sqr as crate::stdlib::uint64_t) << 32 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn pixel_var2_8x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut ssd: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum_u: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut sum_v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut sqr_u: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut sqr_v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 16 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                let mut diff_u: ::core::ffi::c_int = *fenc.offset(x as isize) as ::core::ffi::c_int
                    - *fdec.offset(x as isize) as ::core::ffi::c_int;
                let mut diff_v: ::core::ffi::c_int = *fenc.offset(
                    (x + crate::src::common::common::FENC_STRIDE / 2 as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_int
                    - *fdec.offset(
                        (x + crate::src::common::common::FDEC_STRIDE / 2 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int;
                sum_u += diff_u;
                sum_v += diff_v;
                sqr_u += diff_u * diff_u;
                sqr_v += diff_v * diff_v;
                x += 1;
            }
            fenc = fenc.offset(crate::src::common::common::FENC_STRIDE as isize);
            fdec = fdec.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
        *ssd.offset(0 as ::core::ffi::c_int as isize) = sqr_u;
        *ssd.offset(1 as ::core::ffi::c_int as isize) = sqr_v;
        return (sqr_u as crate::stdlib::int64_t
            - (sum_u as crate::stdlib::int64_t * sum_u as crate::stdlib::int64_t
                >> 7 as ::core::ffi::c_int)
            + sqr_v as crate::stdlib::int64_t
            - (sum_v as crate::stdlib::int64_t * sum_v as crate::stdlib::int64_t
                >> 7 as ::core::ffi::c_int)) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn pixel_var2_8x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut ssd: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum_u: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut sum_v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut sqr_u: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut sqr_v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                let mut diff_u: ::core::ffi::c_int = *fenc.offset(x as isize) as ::core::ffi::c_int
                    - *fdec.offset(x as isize) as ::core::ffi::c_int;
                let mut diff_v: ::core::ffi::c_int = *fenc.offset(
                    (x + crate::src::common::common::FENC_STRIDE / 2 as ::core::ffi::c_int)
                        as isize,
                ) as ::core::ffi::c_int
                    - *fdec.offset(
                        (x + crate::src::common::common::FDEC_STRIDE / 2 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int;
                sum_u += diff_u;
                sum_v += diff_v;
                sqr_u += diff_u * diff_u;
                sqr_v += diff_v * diff_v;
                x += 1;
            }
            fenc = fenc.offset(crate::src::common::common::FENC_STRIDE as isize);
            fdec = fdec.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
        *ssd.offset(0 as ::core::ffi::c_int as isize) = sqr_u;
        *ssd.offset(1 as ::core::ffi::c_int as isize) = sqr_v;
        return (sqr_u as crate::stdlib::int64_t
            - (sum_u as crate::stdlib::int64_t * sum_u as crate::stdlib::int64_t
                >> 6 as ::core::ffi::c_int)
            + sqr_v as crate::stdlib::int64_t
            - (sum_v as crate::stdlib::int64_t * sum_v as crate::stdlib::int64_t
                >> 6 as ::core::ffi::c_int)) as ::core::ffi::c_int;
    }
}
pub const BITS_PER_SUM: usize = (8 as usize).wrapping_mul(::core::mem::size_of::<sum_t>() as usize);
#[inline(always)]
extern "C" fn abs2(mut a: sum2_t) -> sum2_t {
    let mut s: sum2_t = (a >> BITS_PER_SUM.wrapping_sub(1 as usize)
        & ((1 as ::core::ffi::c_int as sum2_t) << BITS_PER_SUM).wrapping_add(1 as sum2_t))
    .wrapping_mul(-(1 as ::core::ffi::c_int) as sum_t as sum2_t);
    return a.wrapping_add(s) ^ s;
}
#[inline(never)]
unsafe extern "C" fn x264_pixel_satd_4x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut tmp: [[sum2_t; 2]; 4] = [[0; 2]; 4];
        let mut a0: sum2_t = 0;
        let mut a1: sum2_t = 0;
        let mut a2: sum2_t = 0;
        let mut a3: sum2_t = 0;
        let mut b0: sum2_t = 0;
        let mut b1: sum2_t = 0;
        let mut sum: sum2_t = 0 as sum2_t;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            a0 = (*pix1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            a1 = (*pix1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            b0 = a0
                .wrapping_add(a1)
                .wrapping_add(a0.wrapping_sub(a1) << BITS_PER_SUM);
            a2 = (*pix1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            a3 = (*pix1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            b1 = a2
                .wrapping_add(a3)
                .wrapping_add(a2.wrapping_sub(a3) << BITS_PER_SUM);
            tmp[i as usize][0 as ::core::ffi::c_int as usize] = b0.wrapping_add(b1);
            tmp[i as usize][1 as ::core::ffi::c_int as usize] = b0.wrapping_sub(b1);
            i += 1;
            pix1 = pix1.offset(i_pix1 as isize);
            pix2 = pix2.offset(i_pix2 as isize);
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 2 as ::core::ffi::c_int {
            let mut t0: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_add(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t1: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_sub(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t2: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_add(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t3: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_sub(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
            a0 = t0.wrapping_add(t2);
            a2 = t0.wrapping_sub(t2);
            a1 = t1.wrapping_add(t3);
            a3 = t1.wrapping_sub(t3);
            a0 = abs2(a0)
                .wrapping_add(abs2(a1))
                .wrapping_add(abs2(a2))
                .wrapping_add(abs2(a3));
            sum = sum.wrapping_add((a0 as sum_t as sum2_t).wrapping_add(a0 >> BITS_PER_SUM));
            i_0 += 1;
        }
        return (sum >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn x264_pixel_satd_8x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut tmp: [[sum2_t; 4]; 4] = [[0; 4]; 4];
        let mut a0: sum2_t = 0;
        let mut a1: sum2_t = 0;
        let mut a2: sum2_t = 0;
        let mut a3: sum2_t = 0;
        let mut sum: sum2_t = 0 as sum2_t;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            a0 = ((*pix1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t)
                .wrapping_add(
                    ((*pix1.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - *pix2.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            a1 = ((*pix1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t)
                .wrapping_add(
                    ((*pix1.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - *pix2.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            a2 = ((*pix1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t)
                .wrapping_add(
                    ((*pix1.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - *pix2.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            a3 = ((*pix1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t)
                .wrapping_add(
                    ((*pix1.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - *pix2.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            let mut t0: sum2_t = a0.wrapping_add(a1);
            let mut t1: sum2_t = a0.wrapping_sub(a1);
            let mut t2: sum2_t = a2.wrapping_add(a3);
            let mut t3: sum2_t = a2.wrapping_sub(a3);
            tmp[i as usize][0 as ::core::ffi::c_int as usize] = t0.wrapping_add(t2);
            tmp[i as usize][2 as ::core::ffi::c_int as usize] = t0.wrapping_sub(t2);
            tmp[i as usize][1 as ::core::ffi::c_int as usize] = t1.wrapping_add(t3);
            tmp[i as usize][3 as ::core::ffi::c_int as usize] = t1.wrapping_sub(t3);
            i += 1;
            pix1 = pix1.offset(i_pix1 as isize);
            pix2 = pix2.offset(i_pix2 as isize);
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 4 as ::core::ffi::c_int {
            let mut t0_0: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_add(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t1_0: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_sub(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t2_0: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_add(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t3_0: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_sub(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
            a0 = t0_0.wrapping_add(t2_0);
            a2 = t0_0.wrapping_sub(t2_0);
            a1 = t1_0.wrapping_add(t3_0);
            a3 = t1_0.wrapping_sub(t3_0);
            sum = sum.wrapping_add(
                abs2(a0)
                    .wrapping_add(abs2(a1))
                    .wrapping_add(abs2(a2))
                    .wrapping_add(abs2(a3)),
            );
            i_0 += 1;
        }
        return ((sum as sum_t as sum2_t).wrapping_add(sum >> BITS_PER_SUM)
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn x264_pixel_satd_16x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum: ::core::ffi::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_8x4(
                pix1.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        }
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_8x4(
                pix1.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
            && 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        return sum;
    }
}
unsafe extern "C" fn x264_pixel_satd_16x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum: ::core::ffi::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_8x4(
                pix1.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        }
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_8x4(
                pix1.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
            && 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        return sum;
    }
}
unsafe extern "C" fn x264_pixel_satd_8x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum: ::core::ffi::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_8x4(
                pix1.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        }
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_8x4(
                pix1.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
            && 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        return sum;
    }
}
unsafe extern "C" fn x264_pixel_satd_8x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum: ::core::ffi::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_8x4(
                pix1.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        }
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_8x4(
                pix1.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
            && 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        return sum;
    }
}
unsafe extern "C" fn x264_pixel_satd_4x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum: ::core::ffi::c_int = x264_pixel_satd_4x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_4x4(
                pix1.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        if 4 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_4x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            ) + x264_pixel_satd_4x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        }
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_4x4(
                pix1.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_4x4(
                pix1.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        if 4 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
            && 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        {
            sum += x264_pixel_satd_4x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_4x4(
                pix1.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        return sum;
    }
}
unsafe extern "C" fn x264_pixel_satd_4x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum: ::core::ffi::c_int = x264_pixel_satd_4x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_4x4(
                pix1.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        if 4 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_4x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            ) + x264_pixel_satd_4x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        }
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum += x264_pixel_satd_4x4(
                pix1.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_4x4(
                pix1.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        if 4 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
            && 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        {
            sum += x264_pixel_satd_4x4(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            ) + x264_pixel_satd_4x4(
                pix1.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize,
                ),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize).offset(
                    (12 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize,
                ),
                i_pix2,
            );
        }
        return sum;
    }
}
#[inline(never)]
unsafe extern "C" fn sa8d_8x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut tmp: [[sum2_t; 4]; 8] = [[0; 4]; 8];
        let mut a0: sum2_t = 0;
        let mut a1: sum2_t = 0;
        let mut a2: sum2_t = 0;
        let mut a3: sum2_t = 0;
        let mut a4: sum2_t = 0;
        let mut a5: sum2_t = 0;
        let mut a6: sum2_t = 0;
        let mut a7: sum2_t = 0;
        let mut b0: sum2_t = 0;
        let mut b1: sum2_t = 0;
        let mut b2: sum2_t = 0;
        let mut b3: sum2_t = 0;
        let mut sum: sum2_t = 0 as sum2_t;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            a0 = (*pix1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            a1 = (*pix1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            b0 = a0
                .wrapping_add(a1)
                .wrapping_add(a0.wrapping_sub(a1) << BITS_PER_SUM);
            a2 = (*pix1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            a3 = (*pix1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            b1 = a2
                .wrapping_add(a3)
                .wrapping_add(a2.wrapping_sub(a3) << BITS_PER_SUM);
            a4 = (*pix1.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            a5 = (*pix1.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            b2 = a4
                .wrapping_add(a5)
                .wrapping_add(a4.wrapping_sub(a5) << BITS_PER_SUM);
            a6 = (*pix1.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            a7 = (*pix1.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t;
            b3 = a6
                .wrapping_add(a7)
                .wrapping_add(a6.wrapping_sub(a7) << BITS_PER_SUM);
            let mut t0: sum2_t = b0.wrapping_add(b1);
            let mut t1: sum2_t = b0.wrapping_sub(b1);
            let mut t2: sum2_t = b2.wrapping_add(b3);
            let mut t3: sum2_t = b2.wrapping_sub(b3);
            tmp[i as usize][0 as ::core::ffi::c_int as usize] = t0.wrapping_add(t2);
            tmp[i as usize][2 as ::core::ffi::c_int as usize] = t0.wrapping_sub(t2);
            tmp[i as usize][1 as ::core::ffi::c_int as usize] = t1.wrapping_add(t3);
            tmp[i as usize][3 as ::core::ffi::c_int as usize] = t1.wrapping_sub(t3);
            i += 1;
            pix1 = pix1.offset(i_pix1 as isize);
            pix2 = pix2.offset(i_pix2 as isize);
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 4 as ::core::ffi::c_int {
            let mut t0_0: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_add(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t1_0: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_sub(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t2_0: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_add(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t3_0: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_sub(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
            a0 = t0_0.wrapping_add(t2_0);
            a2 = t0_0.wrapping_sub(t2_0);
            a1 = t1_0.wrapping_add(t3_0);
            a3 = t1_0.wrapping_sub(t3_0);
            let mut t0_1: sum2_t = tmp[4 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_add(tmp[5 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t1_1: sum2_t = tmp[4 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_sub(tmp[5 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t2_1: sum2_t = tmp[6 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_add(tmp[7 as ::core::ffi::c_int as usize][i_0 as usize]);
            let mut t3_1: sum2_t = tmp[6 as ::core::ffi::c_int as usize][i_0 as usize]
                .wrapping_sub(tmp[7 as ::core::ffi::c_int as usize][i_0 as usize]);
            a4 = t0_1.wrapping_add(t2_1);
            a6 = t0_1.wrapping_sub(t2_1);
            a5 = t1_1.wrapping_add(t3_1);
            a7 = t1_1.wrapping_sub(t3_1);
            b0 = abs2(a0.wrapping_add(a4)).wrapping_add(abs2(a0.wrapping_sub(a4)));
            b0 = b0.wrapping_add(abs2(a1.wrapping_add(a5)).wrapping_add(abs2(a1.wrapping_sub(a5))));
            b0 = b0.wrapping_add(abs2(a2.wrapping_add(a6)).wrapping_add(abs2(a2.wrapping_sub(a6))));
            b0 = b0.wrapping_add(abs2(a3.wrapping_add(a7)).wrapping_add(abs2(a3.wrapping_sub(a7))));
            sum = sum.wrapping_add((b0 as sum_t as sum2_t).wrapping_add(b0 >> BITS_PER_SUM));
            i_0 += 1;
        }
        return sum as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn x264_pixel_sa8d_8x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum: ::core::ffi::c_int = sa8d_8x8(pix1, i_pix1, pix2, i_pix2);
        return sum + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn x264_pixel_sa8d_16x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum: ::core::ffi::c_int = sa8d_8x8(pix1, i_pix1, pix2, i_pix2)
            + sa8d_8x8(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            )
            + sa8d_8x8(
                pix1.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            )
            + sa8d_8x8(
                pix1.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * i_pix2) as isize),
                i_pix2,
            );
        return sum + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
    }
}
#[inline(never)]
unsafe extern "C" fn pixel_hadamard_ac(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut tmp: [sum2_t; 32] = [0; 32];
        let mut a0: sum2_t = 0;
        let mut a1: sum2_t = 0;
        let mut a2: sum2_t = 0;
        let mut a3: sum2_t = 0;
        let mut dc: sum2_t = 0;
        let mut sum4: sum2_t = 0 as sum2_t;
        let mut sum8: sum2_t = 0 as sum2_t;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let mut t: *mut sum2_t = (&raw mut tmp as *mut sum2_t)
                .offset((i & 3 as ::core::ffi::c_int) as isize)
                .offset(((i & 4 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize);
            a0 = ((*pix.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *pix.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t)
                .wrapping_add(
                    ((*pix.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - *pix.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            a1 = ((*pix.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *pix.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t)
                .wrapping_add(
                    ((*pix.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - *pix.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            *t.offset(0 as ::core::ffi::c_int as isize) = a0.wrapping_add(a1);
            *t.offset(4 as ::core::ffi::c_int as isize) = a0.wrapping_sub(a1);
            a2 = ((*pix.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *pix.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t)
                .wrapping_add(
                    ((*pix.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - *pix.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            a3 = ((*pix.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *pix.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                as sum2_t)
                .wrapping_add(
                    ((*pix.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - *pix.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            *t.offset(8 as ::core::ffi::c_int as isize) = a2.wrapping_add(a3);
            *t.offset(12 as ::core::ffi::c_int as isize) = a2.wrapping_sub(a3);
            i += 1;
            pix = pix.offset(stride as isize);
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 8 as ::core::ffi::c_int {
            let mut t0: sum2_t = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                .wrapping_add(
                    tmp[(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize],
                );
            let mut t1: sum2_t = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                .wrapping_sub(
                    tmp[(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize],
                );
            let mut t2: sum2_t = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                .wrapping_add(
                    tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize],
                );
            let mut t3: sum2_t = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                .wrapping_sub(
                    tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize],
                );
            a0 = t0.wrapping_add(t2);
            a2 = t0.wrapping_sub(t2);
            a1 = t1.wrapping_add(t3);
            a3 = t1.wrapping_sub(t3);
            tmp[(i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize] = a0;
            tmp[(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] = a1;
            tmp[(i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] = a2;
            tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] = a3;
            sum4 = sum4.wrapping_add(
                abs2(a0)
                    .wrapping_add(abs2(a1))
                    .wrapping_add(abs2(a2))
                    .wrapping_add(abs2(a3)),
            );
            i_0 += 1;
        }
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < 8 as ::core::ffi::c_int {
            let mut t0_0: sum2_t =
                tmp[i_1 as usize].wrapping_add(tmp[(8 as ::core::ffi::c_int + i_1) as usize]);
            let mut t1_0: sum2_t =
                tmp[i_1 as usize].wrapping_sub(tmp[(8 as ::core::ffi::c_int + i_1) as usize]);
            let mut t2_0: sum2_t = tmp[(16 as ::core::ffi::c_int + i_1) as usize]
                .wrapping_add(tmp[(24 as ::core::ffi::c_int + i_1) as usize]);
            let mut t3_0: sum2_t = tmp[(16 as ::core::ffi::c_int + i_1) as usize]
                .wrapping_sub(tmp[(24 as ::core::ffi::c_int + i_1) as usize]);
            a0 = t0_0.wrapping_add(t2_0);
            a2 = t0_0.wrapping_sub(t2_0);
            a1 = t1_0.wrapping_add(t3_0);
            a3 = t1_0.wrapping_sub(t3_0);
            sum8 = sum8.wrapping_add(
                abs2(a0)
                    .wrapping_add(abs2(a1))
                    .wrapping_add(abs2(a2))
                    .wrapping_add(abs2(a3)),
            );
            i_1 += 1;
        }
        dc = tmp[0 as ::core::ffi::c_int as usize]
            .wrapping_add(tmp[8 as ::core::ffi::c_int as usize])
            .wrapping_add(tmp[16 as ::core::ffi::c_int as usize])
            .wrapping_add(tmp[24 as ::core::ffi::c_int as usize]) as sum_t as sum2_t;
        sum4 = (sum4 as sum_t as sum2_t)
            .wrapping_add(sum4 >> BITS_PER_SUM)
            .wrapping_sub(dc);
        sum8 = (sum8 as sum_t as sum2_t)
            .wrapping_add(sum8 >> BITS_PER_SUM)
            .wrapping_sub(dc);
        return ((sum8 as crate::stdlib::uint64_t) << 32 as ::core::ffi::c_int)
            .wrapping_add(sum4 as crate::stdlib::uint64_t);
    }
}
unsafe extern "C" fn x264_pixel_hadamard_ac_16x16(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum: crate::stdlib::uint64_t = pixel_hadamard_ac(pix, stride);
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset(8 as ::core::ffi::c_int as isize),
                stride,
            ));
        }
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * stride) as isize),
                stride,
            ));
        }
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
            && 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * stride) as isize)
                    .offset(8 as ::core::ffi::c_int as isize),
                stride,
            ));
        }
        return ((sum >> 34 as ::core::ffi::c_int) << 32 as ::core::ffi::c_int).wrapping_add(
            (sum as crate::stdlib::uint32_t >> 1 as ::core::ffi::c_int) as crate::stdlib::uint64_t,
        );
    }
}
unsafe extern "C" fn x264_pixel_hadamard_ac_16x8(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum: crate::stdlib::uint64_t = pixel_hadamard_ac(pix, stride);
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset(8 as ::core::ffi::c_int as isize),
                stride,
            ));
        }
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * stride) as isize),
                stride,
            ));
        }
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
            && 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * stride) as isize)
                    .offset(8 as ::core::ffi::c_int as isize),
                stride,
            ));
        }
        return ((sum >> 34 as ::core::ffi::c_int) << 32 as ::core::ffi::c_int).wrapping_add(
            (sum as crate::stdlib::uint32_t >> 1 as ::core::ffi::c_int) as crate::stdlib::uint64_t,
        );
    }
}
unsafe extern "C" fn x264_pixel_hadamard_ac_8x16(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum: crate::stdlib::uint64_t = pixel_hadamard_ac(pix, stride);
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset(8 as ::core::ffi::c_int as isize),
                stride,
            ));
        }
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * stride) as isize),
                stride,
            ));
        }
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
            && 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * stride) as isize)
                    .offset(8 as ::core::ffi::c_int as isize),
                stride,
            ));
        }
        return ((sum >> 34 as ::core::ffi::c_int) << 32 as ::core::ffi::c_int).wrapping_add(
            (sum as crate::stdlib::uint32_t >> 1 as ::core::ffi::c_int) as crate::stdlib::uint64_t,
        );
    }
}
unsafe extern "C" fn x264_pixel_hadamard_ac_8x8(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum: crate::stdlib::uint64_t = pixel_hadamard_ac(pix, stride);
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset(8 as ::core::ffi::c_int as isize),
                stride,
            ));
        }
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * stride) as isize),
                stride,
            ));
        }
        if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
            && 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset((8 as ::core::ffi::c_int as crate::stdlib::intptr_t * stride) as isize)
                    .offset(8 as ::core::ffi::c_int as isize),
                stride,
            ));
        }
        return ((sum >> 34 as ::core::ffi::c_int) << 32 as ::core::ffi::c_int).wrapping_add(
            (sum as crate::stdlib::uint32_t >> 1 as ::core::ffi::c_int) as crate::stdlib::uint64_t,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x3_16x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x4_16x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x3_16x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x4_16x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x3_8x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x4_8x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x3_8x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x4_8x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x4_8x4(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x3_8x4(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x4_4x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x3_4x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x3_4x4(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_sad_x4_4x4(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x4_8x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x3_4x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x3_8x4(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x3_8x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x3_8x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x3_16x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x3_16x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x4_8x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x3_4x4(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x4_8x4(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x4_4x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x4_4x4(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x4_16x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn x264_pixel_satd_x4_16x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut pix0: *mut crate::src::common::common::pixel,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
    mut pix3: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    unsafe {
        *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix3,
            i_stride,
        );
    }
}
unsafe extern "C" fn intra_sad_x3_8x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut pix: [crate::src::common::common::pixel; 256] = [0; 256];
        crate::src::common::predict::x264_8_predict_8x8_v_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            &raw mut pix as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8_h_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            &raw mut pix as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8_dc_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            &raw mut pix as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
    }
}
unsafe extern "C" fn intra_sa8d_x3_8x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut edge: *mut crate::src::common::common::pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    unsafe {
        let mut pix: [crate::src::common::common::pixel; 256] = [0; 256];
        crate::src::common::predict::x264_8_predict_8x8_v_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sa8d_8x8(
            &raw mut pix as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8_h_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sa8d_8x8(
            &raw mut pix as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8_dc_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sa8d_8x8(
            &raw mut pix as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
    }
}
unsafe extern "C" fn intra_sad_x3_4x4(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    unsafe {
        crate::src::common::predict::x264_8_predict_4x4_v_c(fdec);
        *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_4x4_h_c(fdec);
        *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_4x4_dc_c(fdec);
        *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
    }
}
unsafe extern "C" fn intra_satd_x3_4x4(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    unsafe {
        crate::src::common::predict::x264_8_predict_4x4_v_c(fdec);
        *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_4x4_h_c(fdec);
        *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_4x4_dc_c(fdec);
        *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
    }
}
unsafe extern "C" fn intra_sad_x3_8x8c(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    unsafe {
        crate::src::common::predict::x264_8_predict_8x8c_dc_c(fdec);
        *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8c_h_c(fdec);
        *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8c_v_c(fdec);
        *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
    }
}
unsafe extern "C" fn intra_satd_x3_8x8c(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    unsafe {
        crate::src::common::predict::x264_8_predict_8x8c_dc_c(fdec);
        *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8c_h_c(fdec);
        *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8c_v_c(fdec);
        *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
    }
}
unsafe extern "C" fn intra_sad_x3_8x16c(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    unsafe {
        crate::src::common::predict::x264_8_predict_8x16c_dc_c(fdec);
        *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x16c_h_c(fdec);
        *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x16c_v_c(fdec);
        *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
    }
}
unsafe extern "C" fn intra_satd_x3_8x16c(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    unsafe {
        crate::src::common::predict::x264_8_predict_8x16c_dc_c(fdec);
        *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x16c_h_c(fdec);
        *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x16c_v_c(fdec);
        *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
    }
}
unsafe extern "C" fn intra_sad_x3_16x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    unsafe {
        crate::src::common::predict::x264_8_predict_16x16_v_c(fdec);
        *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_16x16_h_c(fdec);
        *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_16x16_dc_c(fdec);
        *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
    }
}
unsafe extern "C" fn intra_satd_x3_16x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    unsafe {
        crate::src::common::predict::x264_8_predict_16x16_v_c(fdec);
        *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_16x16_h_c(fdec);
        *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_16x16_dc_c(fdec);
        *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
    }
}
unsafe extern "C" fn ssim_4x4x2_core(
    mut pix1: *const crate::src::common::common::pixel,
    mut stride1: crate::stdlib::intptr_t,
    mut pix2: *const crate::src::common::common::pixel,
    mut stride2: crate::stdlib::intptr_t,
    mut sums: *mut [::core::ffi::c_int; 4],
) {
    unsafe {
        let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while z < 2 as ::core::ffi::c_int {
            let mut s1: crate::stdlib::uint32_t = 0 as crate::stdlib::uint32_t;
            let mut s2: crate::stdlib::uint32_t = 0 as crate::stdlib::uint32_t;
            let mut ss: crate::stdlib::uint32_t = 0 as crate::stdlib::uint32_t;
            let mut s12: crate::stdlib::uint32_t = 0 as crate::stdlib::uint32_t;
            let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y < 4 as ::core::ffi::c_int {
                let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x < 4 as ::core::ffi::c_int {
                    let mut a: ::core::ffi::c_int = *pix1.offset(
                        (x as crate::stdlib::intptr_t + y as crate::stdlib::intptr_t * stride1)
                            as isize,
                    ) as ::core::ffi::c_int;
                    let mut b: ::core::ffi::c_int = *pix2.offset(
                        (x as crate::stdlib::intptr_t + y as crate::stdlib::intptr_t * stride2)
                            as isize,
                    ) as ::core::ffi::c_int;
                    s1 = s1.wrapping_add(a as crate::stdlib::uint32_t);
                    s2 = s2.wrapping_add(b as crate::stdlib::uint32_t);
                    ss = ss.wrapping_add((a * a) as crate::stdlib::uint32_t);
                    ss = ss.wrapping_add((b * b) as crate::stdlib::uint32_t);
                    s12 = s12.wrapping_add((a * b) as crate::stdlib::uint32_t);
                    x += 1;
                }
                y += 1;
            }
            (*sums.offset(z as isize))[0 as ::core::ffi::c_int as usize] = s1 as ::core::ffi::c_int;
            (*sums.offset(z as isize))[1 as ::core::ffi::c_int as usize] = s2 as ::core::ffi::c_int;
            (*sums.offset(z as isize))[2 as ::core::ffi::c_int as usize] = ss as ::core::ffi::c_int;
            (*sums.offset(z as isize))[3 as ::core::ffi::c_int as usize] =
                s12 as ::core::ffi::c_int;
            pix1 = pix1.offset(4 as ::core::ffi::c_int as isize);
            pix2 = pix2.offset(4 as ::core::ffi::c_int as isize);
            z += 1;
        }
    }
}
unsafe extern "C" fn ssim_end1(
    mut s1: ::core::ffi::c_int,
    mut s2: ::core::ffi::c_int,
    mut ss: ::core::ffi::c_int,
    mut s12: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    unsafe {
        static mut ssim_c1: ::core::ffi::c_int = (0.01f64
            * 0.01f64
            * crate::src::common::common::PIXEL_MAX as ::core::ffi::c_double
            * crate::src::common::common::PIXEL_MAX as ::core::ffi::c_double
            * 64 as ::core::ffi::c_int as ::core::ffi::c_double
            + 0.5f64) as ::core::ffi::c_int;
        static mut ssim_c2: ::core::ffi::c_int = (0.03f64
            * 0.03f64
            * crate::src::common::common::PIXEL_MAX as ::core::ffi::c_double
            * crate::src::common::common::PIXEL_MAX as ::core::ffi::c_double
            * 64 as ::core::ffi::c_int as ::core::ffi::c_double
            * 63 as ::core::ffi::c_int as ::core::ffi::c_double
            + 0.5f64) as ::core::ffi::c_int;
        let mut fs1: ::core::ffi::c_int = s1;
        let mut fs2: ::core::ffi::c_int = s2;
        let mut fss: ::core::ffi::c_int = ss;
        let mut fs12: ::core::ffi::c_int = s12;
        let mut vars: ::core::ffi::c_int = fss * 64 as ::core::ffi::c_int - fs1 * fs1 - fs2 * fs2;
        let mut covar: ::core::ffi::c_int = fs12 * 64 as ::core::ffi::c_int - fs1 * fs2;
        return (2 as ::core::ffi::c_int * fs1 * fs2 + ssim_c1) as ::core::ffi::c_float
            * (2 as ::core::ffi::c_int * covar + ssim_c2) as ::core::ffi::c_float
            / ((fs1 * fs1 + fs2 * fs2 + ssim_c1) as ::core::ffi::c_float
                * (vars + ssim_c2) as ::core::ffi::c_float);
    }
}
unsafe extern "C" fn ssim_end4(
    mut sum0: *mut [::core::ffi::c_int; 4],
    mut sum1: *mut [::core::ffi::c_int; 4],
    mut width: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    unsafe {
        let mut ssim: ::core::ffi::c_float = 0.0f32;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < width {
            ssim += ssim_end1(
                (*sum0.offset(i as isize))[0 as ::core::ffi::c_int as usize]
                    + (*sum0.offset((i + 1 as ::core::ffi::c_int) as isize))
                        [0 as ::core::ffi::c_int as usize]
                    + (*sum1.offset(i as isize))[0 as ::core::ffi::c_int as usize]
                    + (*sum1.offset((i + 1 as ::core::ffi::c_int) as isize))
                        [0 as ::core::ffi::c_int as usize],
                (*sum0.offset(i as isize))[1 as ::core::ffi::c_int as usize]
                    + (*sum0.offset((i + 1 as ::core::ffi::c_int) as isize))
                        [1 as ::core::ffi::c_int as usize]
                    + (*sum1.offset(i as isize))[1 as ::core::ffi::c_int as usize]
                    + (*sum1.offset((i + 1 as ::core::ffi::c_int) as isize))
                        [1 as ::core::ffi::c_int as usize],
                (*sum0.offset(i as isize))[2 as ::core::ffi::c_int as usize]
                    + (*sum0.offset((i + 1 as ::core::ffi::c_int) as isize))
                        [2 as ::core::ffi::c_int as usize]
                    + (*sum1.offset(i as isize))[2 as ::core::ffi::c_int as usize]
                    + (*sum1.offset((i + 1 as ::core::ffi::c_int) as isize))
                        [2 as ::core::ffi::c_int as usize],
                (*sum0.offset(i as isize))[3 as ::core::ffi::c_int as usize]
                    + (*sum0.offset((i + 1 as ::core::ffi::c_int) as isize))
                        [3 as ::core::ffi::c_int as usize]
                    + (*sum1.offset(i as isize))[3 as ::core::ffi::c_int as usize]
                    + (*sum1.offset((i + 1 as ::core::ffi::c_int) as isize))
                        [3 as ::core::ffi::c_int as usize],
            );
            i += 1;
        }
        return ssim;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pixel_ssim_wxh(
    mut pf: *mut crate::src::common::pixel::x264_pixel_function_t,
    mut pix1: *mut crate::src::common::common::pixel,
    mut stride1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut stride2: crate::stdlib::intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_void,
    mut cnt: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    unsafe {
        let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ssim: ::core::ffi::c_float = 0.0f32;
        let mut sum0: *mut [::core::ffi::c_int; 4] = buf as *mut [::core::ffi::c_int; 4];
        let mut sum1: *mut [::core::ffi::c_int; 4] = sum0
            .offset((width >> 2 as ::core::ffi::c_int) as isize)
            .offset(3 as ::core::ffi::c_int as isize);
        width >>= 2 as ::core::ffi::c_int;
        height >>= 2 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while y < height {
            while z <= y {
                let mut t: *mut ::core::ffi::c_void = sum0 as *mut ::core::ffi::c_void;
                sum0 = sum1;
                sum1 = t as *mut [::core::ffi::c_int; 4];
                let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while x < width {
                    (*pf).ssim_4x4x2_core.expect("non-null function pointer")(
                        pix1.offset(
                            (4 as ::core::ffi::c_int as crate::stdlib::intptr_t
                                * (x as crate::stdlib::intptr_t
                                    + z as crate::stdlib::intptr_t * stride1))
                                as isize,
                        ) as *mut crate::src::common::common::pixel,
                        stride1,
                        pix2.offset(
                            (4 as ::core::ffi::c_int as crate::stdlib::intptr_t
                                * (x as crate::stdlib::intptr_t
                                    + z as crate::stdlib::intptr_t * stride2))
                                as isize,
                        ) as *mut crate::src::common::common::pixel,
                        stride2,
                        sum0.offset(x as isize) as *mut [::core::ffi::c_int; 4],
                    );
                    x += 2 as ::core::ffi::c_int;
                }
                z += 1;
            }
            let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_0 < width - 1 as ::core::ffi::c_int {
                ssim += (*pf).ssim_end4.expect("non-null function pointer")(
                    sum0.offset(x_0 as isize),
                    sum1.offset(x_0 as isize),
                    if (4 as ::core::ffi::c_int) < width - x_0 - 1 as ::core::ffi::c_int {
                        4 as ::core::ffi::c_int
                    } else {
                        width - x_0 - 1 as ::core::ffi::c_int
                    },
                );
                x_0 += 4 as ::core::ffi::c_int;
            }
            y += 1;
        }
        *cnt = (height - 1 as ::core::ffi::c_int) * (width - 1 as ::core::ffi::c_int);
        return ssim;
    }
}
unsafe extern "C" fn pixel_vsad(
    mut src: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut score: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i < height {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < 16 as ::core::ffi::c_int {
                score += crate::stdlib::abs(
                    *src.offset(j as isize) as ::core::ffi::c_int
                        - *src.offset((j as crate::stdlib::intptr_t + stride) as isize)
                            as ::core::ffi::c_int,
                );
                j += 1;
            }
            i += 1;
            src = src.offset(stride as isize);
        }
        return score;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_field_vsad(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut score_field: ::core::ffi::c_int = 0;
        let mut score_frame: ::core::ffi::c_int = 0;
        let mut stride: ::core::ffi::c_int =
            (*(*h).fenc).i_stride[0 as ::core::ffi::c_int as usize];
        let mut mb_stride: ::core::ffi::c_int = (*h).mb.i_mb_stride;
        let mut fenc: *mut crate::src::common::common::pixel = (*(*h).fenc).plane
            [0 as ::core::ffi::c_int as usize]
            .offset((16 as ::core::ffi::c_int * (mb_x + mb_y * stride)) as isize);
        let mut mb_xy: ::core::ffi::c_int = mb_x + mb_y * mb_stride;
        let mut mbpair_height: ::core::ffi::c_int =
            if ((*h).param.i_height - mb_y * 16 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
                (*h).param.i_height - mb_y * 16 as ::core::ffi::c_int
            } else {
                32 as ::core::ffi::c_int
            };
        score_frame = (*h).pixf.vsad.expect("non-null function pointer")(
            fenc,
            stride as crate::stdlib::intptr_t,
            mbpair_height,
        );
        score_field = (*h).pixf.vsad.expect("non-null function pointer")(
            fenc,
            (stride * 2 as ::core::ffi::c_int) as crate::stdlib::intptr_t,
            mbpair_height >> 1 as ::core::ffi::c_int,
        );
        score_field += (*h).pixf.vsad.expect("non-null function pointer")(
            fenc.offset(stride as isize),
            (stride * 2 as ::core::ffi::c_int) as crate::stdlib::intptr_t,
            mbpair_height >> 1 as ::core::ffi::c_int,
        );
        if mb_x > 0 as ::core::ffi::c_int {
            score_field += 512 as ::core::ffi::c_int
                - *(*h)
                    .mb
                    .field
                    .offset((mb_xy - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    * 1024 as ::core::ffi::c_int;
        }
        if mb_y > 0 as ::core::ffi::c_int {
            score_field += 512 as ::core::ffi::c_int
                - *(*h).mb.field.offset((mb_xy - mb_stride) as isize) as ::core::ffi::c_int
                    * 1024 as ::core::ffi::c_int;
        }
        return (score_field < score_frame) as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn pixel_asd8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut stride1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut stride2: crate::stdlib::intptr_t,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < height {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                sum += *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                x += 1;
            }
            y += 1;
            pix1 = pix1.offset(stride1 as isize);
            pix2 = pix2.offset(stride2 as isize);
        }
        return crate::stdlib::abs(sum);
    }
}
unsafe extern "C" fn x264_pixel_ads4(
    mut enc_dc: *mut ::core::ffi::c_int,
    mut sums: *mut crate::stdlib::uint16_t,
    mut delta: ::core::ffi::c_int,
    mut cost_mvx: *mut crate::stdlib::uint16_t,
    mut mvs: *mut crate::stdlib::int16_t,
    mut width: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nmv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < width {
            let mut ads: ::core::ffi::c_int = crate::stdlib::abs(
                *enc_dc.offset(0 as ::core::ffi::c_int as isize)
                    - *sums.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) + crate::stdlib::abs(
                *enc_dc.offset(1 as ::core::ffi::c_int as isize)
                    - *sums.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) + crate::stdlib::abs(
                *enc_dc.offset(2 as ::core::ffi::c_int as isize)
                    - *sums.offset(delta as isize) as ::core::ffi::c_int,
            ) + crate::stdlib::abs(
                *enc_dc.offset(3 as ::core::ffi::c_int as isize)
                    - *sums.offset((delta + 8 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int,
            ) + *cost_mvx.offset(i as isize)
                as ::core::ffi::c_int;
            if ads < thresh {
                let c2rust_fresh2 = nmv;
                nmv = nmv + 1;
                *mvs.offset(c2rust_fresh2 as isize) = i as crate::stdlib::int16_t;
            }
            i += 1;
            sums = sums.offset(1);
        }
        return nmv;
    }
}
unsafe extern "C" fn x264_pixel_ads2(
    mut enc_dc: *mut ::core::ffi::c_int,
    mut sums: *mut crate::stdlib::uint16_t,
    mut delta: ::core::ffi::c_int,
    mut cost_mvx: *mut crate::stdlib::uint16_t,
    mut mvs: *mut crate::stdlib::int16_t,
    mut width: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nmv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < width {
            let mut ads: ::core::ffi::c_int = crate::stdlib::abs(
                *enc_dc.offset(0 as ::core::ffi::c_int as isize)
                    - *sums.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) + crate::stdlib::abs(
                *enc_dc.offset(1 as ::core::ffi::c_int as isize)
                    - *sums.offset(delta as isize) as ::core::ffi::c_int,
            ) + *cost_mvx.offset(i as isize)
                as ::core::ffi::c_int;
            if ads < thresh {
                let c2rust_fresh1 = nmv;
                nmv = nmv + 1;
                *mvs.offset(c2rust_fresh1 as isize) = i as crate::stdlib::int16_t;
            }
            i += 1;
            sums = sums.offset(1);
        }
        return nmv;
    }
}
unsafe extern "C" fn x264_pixel_ads1(
    mut enc_dc: *mut ::core::ffi::c_int,
    mut sums: *mut crate::stdlib::uint16_t,
    mut _delta: ::core::ffi::c_int,
    mut cost_mvx: *mut crate::stdlib::uint16_t,
    mut mvs: *mut crate::stdlib::int16_t,
    mut width: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nmv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < width {
            let mut ads: ::core::ffi::c_int = crate::stdlib::abs(
                *enc_dc.offset(0 as ::core::ffi::c_int as isize)
                    - *sums.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            ) + *cost_mvx.offset(i as isize)
                as ::core::ffi::c_int;
            if ads < thresh {
                let c2rust_fresh0 = nmv;
                nmv = nmv + 1;
                *mvs.offset(c2rust_fresh0 as isize) = i as crate::stdlib::int16_t;
            }
            i += 1;
            sums = sums.offset(1);
        }
        return nmv;
    }
}
#[no_mangle]
pub unsafe extern "C" fn x264_8_pixel_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pixf: *mut crate::src::common::pixel::x264_pixel_function_t,
) {
    unsafe {
        crate::stdlib::memset(
            pixf as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<crate::src::common::pixel::x264_pixel_function_t>()
                as crate::__stddef_size_t_h::size_t,
        );
        (*pixf).sad[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_16x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_8x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_4x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad[crate::src::common::pixel::PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_4x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad_aligned
            [crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_16x8
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            ) as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_8x16
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            ) as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_8x8
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            ) as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_8x4
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            ) as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_4x8
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            ) as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_4x4
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            ) as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_4x16 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_4x16
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            ) as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sad_x3[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x3_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).sad_x3[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x3_16x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).sad_x3[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x3_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).sad_x3[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x3_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).sad_x3[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x3_8x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).sad_x3[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x3_4x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).sad_x3[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x3_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).sad_x4[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x4_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).sad_x4[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x4_16x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).sad_x4[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x4_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).sad_x4[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x4_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).sad_x4[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x4_8x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).sad_x4[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x4_4x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).sad_x4[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_x4_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).ssd[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).ssd[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_16x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).ssd[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).ssd[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).ssd[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_8x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).ssd[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_4x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).ssd[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).ssd[crate::src::common::pixel::PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_4x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).satd[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).satd[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_16x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).satd[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).satd[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).satd[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_8x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).satd[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_4x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).satd[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).satd[crate::src::common::pixel::PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_4x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).satd_x3[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_satd_x3_16x16
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut ::core::ffi::c_int,
                    ) -> (),
            ) as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).satd_x3[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x3_16x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).satd_x3[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x3_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).satd_x3[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x3_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).satd_x3[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x3_8x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).satd_x3[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x3_4x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).satd_x3[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x3_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x3_t;
        (*pixf).satd_x4[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_satd_x4_16x16
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut ::core::ffi::c_int,
                    ) -> (),
            ) as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).satd_x4[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x4_16x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).satd_x4[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x4_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).satd_x4[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x4_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).satd_x4[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x4_8x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).satd_x4[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x4_4x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).satd_x4[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_x4_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as crate::src::common::pixel::x264_pixel_cmp_x4_t;
        (*pixf).hadamard_ac
            [crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_hadamard_ac_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
            >;
        (*pixf).hadamard_ac[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_hadamard_ac_16x8
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> crate::stdlib::uint64_t,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> crate::stdlib::uint64_t,
                >;
        (*pixf).hadamard_ac[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_hadamard_ac_8x16
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> crate::stdlib::uint64_t,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> crate::stdlib::uint64_t,
                >;
        (*pixf).hadamard_ac[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_hadamard_ac_8x8
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> crate::stdlib::uint64_t,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> crate::stdlib::uint64_t,
                >;
        (*pixf).ads[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ads4
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    *mut crate::stdlib::int16_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    *mut crate::stdlib::int16_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*pixf).ads[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ads2
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    *mut crate::stdlib::int16_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    *mut crate::stdlib::int16_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*pixf).ads[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ads1
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    *mut crate::stdlib::int16_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::uint16_t,
                    *mut crate::stdlib::int16_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*pixf).sa8d[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sa8d_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).sa8d[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sa8d_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        )
            as crate::src::common::pixel::x264_pixel_cmp_t;
        (*pixf).var[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            pixel_var_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
            >;
        (*pixf).var[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            pixel_var_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
            >;
        (*pixf).var[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            pixel_var_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
            >;
        (*pixf).var2[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            pixel_var2_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*pixf).var2[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            pixel_var2_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*pixf).ssd_nv12_core = Some(
            pixel_ssd_nv12_core
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::uint64_t,
                    *mut crate::stdlib::uint64_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                    *mut crate::stdlib::uint64_t,
                    *mut crate::stdlib::uint64_t,
                ) -> (),
            >;
        (*pixf).ssim_4x4x2_core = Some(
            ssim_4x4x2_core
                as unsafe extern "C" fn(
                    *const crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *const crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut [::core::ffi::c_int; 4],
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *const crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *const crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut [::core::ffi::c_int; 4],
                ) -> (),
            >;
        (*pixf).ssim_end4 = Some(
            ssim_end4
                as unsafe extern "C" fn(
                    *mut [::core::ffi::c_int; 4],
                    *mut [::core::ffi::c_int; 4],
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_float,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut [::core::ffi::c_int; 4],
                    *mut [::core::ffi::c_int; 4],
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_float,
            >;
        (*pixf).vsad = Some(
            pixel_vsad
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*pixf).asd8 = Some(
            pixel_asd8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*pixf).intra_sad_x3_4x4 = Some(
            intra_sad_x3_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
            >;
        (*pixf).intra_satd_x3_4x4 = Some(
            intra_satd_x3_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
            >;
        (*pixf).intra_sad_x3_8x8 = Some(
            intra_sad_x3_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
            >;
        (*pixf).intra_sa8d_x3_8x8 = Some(
            intra_sa8d_x3_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
            >;
        (*pixf).intra_sad_x3_8x8c = Some(
            intra_sad_x3_8x8c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
            >;
        (*pixf).intra_satd_x3_8x8c = Some(
            intra_satd_x3_8x8c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
            >;
        (*pixf).intra_sad_x3_8x16c = Some(
            intra_sad_x3_8x16c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
            >;
        (*pixf).intra_satd_x3_8x16c = Some(
            intra_satd_x3_8x16c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
            >;
        (*pixf).intra_sad_x3_16x16 = Some(
            intra_sad_x3_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
            >;
        (*pixf).intra_satd_x3_16x16 = Some(
            intra_satd_x3_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
            >;
        (*pixf).ads[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] =
            (*pixf).ads[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize];
        (*pixf).ads[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] =
            (*pixf).ads[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize];
        (*pixf).ads[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] =
            (*pixf).ads[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize];
        (*pixf).ads[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] =
            (*pixf).ads[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize];
    }
}
