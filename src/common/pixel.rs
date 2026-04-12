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
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 16i32 {
            let mut x = 0i32;
            while x < 16i32 {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_sad_16x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 8i32 {
            let mut x = 0i32;
            while x < 16i32 {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_sad_8x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 16i32 {
            let mut x = 0i32;
            while x < 8i32 {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_sad_8x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 8i32 {
            let mut x = 0i32;
            while x < 8i32 {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_sad_8x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 4i32 {
            let mut x = 0i32;
            while x < 8i32 {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_sad_4x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 16i32 {
            let mut x = 0i32;
            while x < 4i32 {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_sad_4x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 8i32 {
            let mut x = 0i32;
            while x < 4i32 {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_sad_4x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 4i32 {
            let mut x = 0i32;
            while x < 4i32 {
                i_sum += crate::stdlib::abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_ssd_16x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 16i32 {
            let mut x = 0i32;
            while x < 16i32 {
                let mut d = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_ssd_16x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 8i32 {
            let mut x = 0i32;
            while x < 16i32 {
                let mut d = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_ssd_8x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 16i32 {
            let mut x = 0i32;
            while x < 8i32 {
                let mut d = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_ssd_8x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 8i32 {
            let mut x = 0i32;
            while x < 8i32 {
                let mut d = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_ssd_8x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 4i32 {
            let mut x = 0i32;
            while x < 8i32 {
                let mut d = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_ssd_4x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 16i32 {
            let mut x = 0i32;
            while x < 4i32 {
                let mut d = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_ssd_4x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 8i32 {
            let mut x = 0i32;
            while x < 4i32 {
                let mut d = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
unsafe extern "C" fn x264_pixel_ssd_4x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_stride_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_stride_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i_sum = 0i32;
        let mut y = 0i32;
        while y < 4i32 {
            let mut x = 0i32;
            while x < 4i32 {
                let mut d = *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                i_sum += d * d;
                x += 1;
            }
            pix1 = pix1.offset(i_stride_pix1);
            pix2 = pix2.offset(i_stride_pix2);
            y += 1;
        }
        i_sum
    }
}
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
        let mut i_ssd = 0u64;
        let mut y = 0i32;
        let mut align =
            ((pix1 as crate::stdlib::intptr_t | pix2 as crate::stdlib::intptr_t | i_pix1 | i_pix2)
                & 15isize
                == 0) as ::core::ffi::c_int;
        while y < i_height - 15i32 {
            let mut x = 0i32;
            if align != 0 {
                while x < i_width - 15i32 {
                    i_ssd = i_ssd.wrapping_add((*pf).ssd
                        [crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize]
                        .expect("non-null function pointer")(
                        pix1.offset(y as crate::stdlib::intptr_t * i_pix1)
                            .offset(x as isize),
                        i_pix1,
                        pix2.offset(y as crate::stdlib::intptr_t * i_pix2)
                            .offset(x as isize),
                        i_pix2,
                    ) as crate::stdlib::uint64_t);
                    x += 16i32;
                }
            }
            while x < i_width - 7i32 {
                i_ssd = i_ssd.wrapping_add((*pf).ssd
                    [crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(
                    pix1.offset(y as crate::stdlib::intptr_t * i_pix1)
                        .offset(x as isize),
                    i_pix1,
                    pix2.offset(y as crate::stdlib::intptr_t * i_pix2)
                        .offset(x as isize),
                    i_pix2,
                ) as crate::stdlib::uint64_t);
                x += 8i32;
            }
            y += 16i32;
        }
        if y < i_height - 7i32 {
            let mut x_0 = 0i32;
            while x_0 < i_width - 7i32 {
                i_ssd = i_ssd.wrapping_add((*pf).ssd
                    [crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize]
                    .expect("non-null function pointer")(
                    pix1.offset(y as crate::stdlib::intptr_t * i_pix1)
                        .offset(x_0 as isize),
                    i_pix1,
                    pix2.offset(y as crate::stdlib::intptr_t * i_pix2)
                        .offset(x_0 as isize),
                    i_pix2,
                ) as crate::stdlib::uint64_t);
                x_0 += 8i32;
            }
        }
        if i_width & 7i32 != 0 {
            y = 0i32;
            while y < i_height & !(7i32) {
                let mut x_1 = i_width & !(7i32);
                while x_1 < i_width {
                    let mut d = *pix1.offset(
                        y as crate::stdlib::intptr_t * i_pix1 + x_1 as crate::stdlib::intptr_t,
                    ) as ::core::ffi::c_int
                        - *pix2.offset(
                            y as crate::stdlib::intptr_t * i_pix2 + x_1 as crate::stdlib::intptr_t,
                        ) as ::core::ffi::c_int;
                    i_ssd = i_ssd.wrapping_add((d * d) as crate::stdlib::uint64_t);
                    x_1 += 1;
                }
                y += 1;
            }
        }
        if i_height & 7i32 != 0 {
            y = i_height & !(7i32);
            while y < i_height {
                let mut x_2 = 0i32;
                while x_2 < i_width {
                    let mut d_0 = *pix1.offset(
                        y as crate::stdlib::intptr_t * i_pix1 + x_2 as crate::stdlib::intptr_t,
                    ) as ::core::ffi::c_int
                        - *pix2.offset(
                            y as crate::stdlib::intptr_t * i_pix2 + x_2 as crate::stdlib::intptr_t,
                        ) as ::core::ffi::c_int;
                    i_ssd = i_ssd.wrapping_add((d_0 * d_0) as crate::stdlib::uint64_t);
                    x_2 += 1;
                }
                y += 1;
            }
        }
        i_ssd
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
        let mut y = 0i32;
        *ssd_u = 0u64;
        *ssd_v = 0u64;
        while y < height {
            let mut x = 0i32;
            while x < width {
                let mut du = *pixuv1.offset((2i32 * x) as isize) as ::core::ffi::c_int
                    - *pixuv2.offset((2i32 * x) as isize) as ::core::ffi::c_int;
                let mut dv = *pixuv1.offset((2i32 * x + 1i32) as isize) as ::core::ffi::c_int
                    - *pixuv2.offset((2i32 * x + 1i32) as isize) as ::core::ffi::c_int;
                *ssd_u = (*ssd_u).wrapping_add((du * du) as crate::stdlib::uint64_t);
                *ssd_v = (*ssd_v).wrapping_add((dv * dv) as crate::stdlib::uint64_t);
                x += 1;
            }
            y += 1;
            pixuv1 = pixuv1.offset(stride1);
            pixuv2 = pixuv2.offset(stride2);
        }
    }
}
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
            i_width & !(7i32),
            i_height,
            ssd_u,
            ssd_v,
        );
        if i_width & 7i32 != 0 {
            let mut tmp = [0; 2];
            pixel_ssd_nv12_core(
                pix1.offset((i_width & !(7i32)) as isize),
                i_pix1,
                pix2.offset((i_width & !(7i32)) as isize),
                i_pix2,
                i_width & 7i32,
                i_height,
                (&raw mut tmp as *mut crate::stdlib::uint64_t).offset(0isize),
                (&raw mut tmp as *mut crate::stdlib::uint64_t).offset(1isize),
            );
            *ssd_u = (*ssd_u).wrapping_add(tmp[0usize]);
            *ssd_v = (*ssd_v).wrapping_add(tmp[1usize]);
        }
    }
}
unsafe extern "C" fn pixel_var_16x16(
    mut pix: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum = 0u32;
        let mut sqr = 0u32;
        let mut y = 0i32;
        while y < 16i32 {
            let mut x = 0i32;
            while x < 16i32 {
                sum = sum.wrapping_add(*pix.offset(x as isize) as crate::stdlib::uint32_t);
                sqr = sqr.wrapping_add(
                    (*pix.offset(x as isize) as ::core::ffi::c_int
                        * *pix.offset(x as isize) as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                );
                x += 1;
            }
            pix = pix.offset(i_stride);
            y += 1;
        }
        (sum as crate::stdlib::uint64_t).wrapping_add((sqr as crate::stdlib::uint64_t) << 32i32)
    }
}
unsafe extern "C" fn pixel_var_8x16(
    mut pix: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum = 0u32;
        let mut sqr = 0u32;
        let mut y = 0i32;
        while y < 16i32 {
            let mut x = 0i32;
            while x < 8i32 {
                sum = sum.wrapping_add(*pix.offset(x as isize) as crate::stdlib::uint32_t);
                sqr = sqr.wrapping_add(
                    (*pix.offset(x as isize) as ::core::ffi::c_int
                        * *pix.offset(x as isize) as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                );
                x += 1;
            }
            pix = pix.offset(i_stride);
            y += 1;
        }
        (sum as crate::stdlib::uint64_t).wrapping_add((sqr as crate::stdlib::uint64_t) << 32i32)
    }
}
unsafe extern "C" fn pixel_var_8x8(
    mut pix: *mut crate::src::common::common::pixel,
    mut i_stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum = 0u32;
        let mut sqr = 0u32;
        let mut y = 0i32;
        while y < 8i32 {
            let mut x = 0i32;
            while x < 8i32 {
                sum = sum.wrapping_add(*pix.offset(x as isize) as crate::stdlib::uint32_t);
                sqr = sqr.wrapping_add(
                    (*pix.offset(x as isize) as ::core::ffi::c_int
                        * *pix.offset(x as isize) as ::core::ffi::c_int)
                        as crate::stdlib::uint32_t,
                );
                x += 1;
            }
            pix = pix.offset(i_stride);
            y += 1;
        }
        (sum as crate::stdlib::uint64_t).wrapping_add((sqr as crate::stdlib::uint64_t) << 32i32)
    }
}
unsafe extern "C" fn pixel_var2_8x16(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut ssd: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum_u = 0i32;
        let mut sum_v = 0i32;
        let mut sqr_u = 0i32;
        let mut sqr_v = 0i32;
        let mut y = 0i32;
        while y < 16i32 {
            let mut x = 0i32;
            while x < 8i32 {
                let mut diff_u = *fenc.offset(x as isize) as ::core::ffi::c_int
                    - *fdec.offset(x as isize) as ::core::ffi::c_int;
                let mut diff_v = *fenc
                    .offset((x + crate::src::common::common::FENC_STRIDE / 2i32) as isize)
                    as ::core::ffi::c_int
                    - *fdec.offset((x + crate::src::common::common::FDEC_STRIDE / 2i32) as isize)
                        as ::core::ffi::c_int;
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
        *ssd.offset(0isize) = sqr_u;
        *ssd.offset(1isize) = sqr_v;
        (sqr_u as crate::stdlib::int64_t
            - ((sum_u as crate::stdlib::int64_t * sum_u as crate::stdlib::int64_t) >> 7i32)
            + sqr_v as crate::stdlib::int64_t
            - ((sum_v as crate::stdlib::int64_t * sum_v as crate::stdlib::int64_t) >> 7i32))
            as ::core::ffi::c_int
    }
}
unsafe extern "C" fn pixel_var2_8x8(
    mut fenc: *mut crate::src::common::common::pixel,
    mut fdec: *mut crate::src::common::common::pixel,
    mut ssd: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum_u = 0i32;
        let mut sum_v = 0i32;
        let mut sqr_u = 0i32;
        let mut sqr_v = 0i32;
        let mut y = 0i32;
        while y < 8i32 {
            let mut x = 0i32;
            while x < 8i32 {
                let mut diff_u = *fenc.offset(x as isize) as ::core::ffi::c_int
                    - *fdec.offset(x as isize) as ::core::ffi::c_int;
                let mut diff_v = *fenc
                    .offset((x + crate::src::common::common::FENC_STRIDE / 2i32) as isize)
                    as ::core::ffi::c_int
                    - *fdec.offset((x + crate::src::common::common::FDEC_STRIDE / 2i32) as isize)
                        as ::core::ffi::c_int;
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
        *ssd.offset(0isize) = sqr_u;
        *ssd.offset(1isize) = sqr_v;
        (sqr_u as crate::stdlib::int64_t
            - ((sum_u as crate::stdlib::int64_t * sum_u as crate::stdlib::int64_t) >> 6i32)
            + sqr_v as crate::stdlib::int64_t
            - ((sum_v as crate::stdlib::int64_t * sum_v as crate::stdlib::int64_t) >> 6i32))
            as ::core::ffi::c_int
    }
}
pub const BITS_PER_SUM: usize = (8usize).wrapping_mul(::core::mem::size_of::<sum_t>());
#[inline(always)]
extern "C" fn abs2(mut a: sum2_t) -> sum2_t {
    let mut s = (a >> BITS_PER_SUM.wrapping_sub(1usize)
        & ((1u32) << BITS_PER_SUM).wrapping_add(1u32))
    .wrapping_mul(-(1i32) as sum_t as sum2_t);
    a.wrapping_add(s) ^ s
}
#[inline(never)]
unsafe extern "C" fn x264_pixel_satd_4x4(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut tmp = [[0; 2]; 4];
        let mut a0 = 0;
        let mut a1 = 0;
        let mut a2 = 0;
        let mut a3 = 0;
        let mut sum = 0u32;
        let mut i = 0i32;
        let mut i_0 = 0i32;
        while i < 4i32 {
            let mut b0 = 0;
            let mut b1 = 0;
            a0 = (*pix1.offset(0isize) as ::core::ffi::c_int
                - *pix2.offset(0isize) as ::core::ffi::c_int) as sum2_t;
            a1 = (*pix1.offset(1isize) as ::core::ffi::c_int
                - *pix2.offset(1isize) as ::core::ffi::c_int) as sum2_t;
            b0 = a0
                .wrapping_add(a1)
                .wrapping_add(a0.wrapping_sub(a1) << BITS_PER_SUM);
            a2 = (*pix1.offset(2isize) as ::core::ffi::c_int
                - *pix2.offset(2isize) as ::core::ffi::c_int) as sum2_t;
            a3 = (*pix1.offset(3isize) as ::core::ffi::c_int
                - *pix2.offset(3isize) as ::core::ffi::c_int) as sum2_t;
            b1 = a2
                .wrapping_add(a3)
                .wrapping_add(a2.wrapping_sub(a3) << BITS_PER_SUM);
            tmp[i as usize][0usize] = b0.wrapping_add(b1);
            tmp[i as usize][1usize] = b0.wrapping_sub(b1);
            i += 1;
            pix1 = pix1.offset(i_pix1);
            pix2 = pix2.offset(i_pix2);
        }
        while i_0 < 2i32 {
            let mut t0 = tmp[0usize][i_0 as usize].wrapping_add(tmp[1usize][i_0 as usize]);
            let mut t1 = tmp[0usize][i_0 as usize].wrapping_sub(tmp[1usize][i_0 as usize]);
            let mut t2 = tmp[2usize][i_0 as usize].wrapping_add(tmp[3usize][i_0 as usize]);
            let mut t3 = tmp[2usize][i_0 as usize].wrapping_sub(tmp[3usize][i_0 as usize]);
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
        (sum >> 1i32) as ::core::ffi::c_int
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
        let mut tmp = [[0; 4]; 4];
        let mut a0 = 0;
        let mut a1 = 0;
        let mut a2 = 0;
        let mut a3 = 0;
        let mut sum = 0u32;
        let mut i = 0i32;
        let mut i_0 = 0i32;
        while i < 4i32 {
            a0 = ((*pix1.offset(0isize) as ::core::ffi::c_int
                - *pix2.offset(0isize) as ::core::ffi::c_int) as sum2_t)
                .wrapping_add(
                    ((*pix1.offset(4isize) as ::core::ffi::c_int
                        - *pix2.offset(4isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            a1 = ((*pix1.offset(1isize) as ::core::ffi::c_int
                - *pix2.offset(1isize) as ::core::ffi::c_int) as sum2_t)
                .wrapping_add(
                    ((*pix1.offset(5isize) as ::core::ffi::c_int
                        - *pix2.offset(5isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            a2 = ((*pix1.offset(2isize) as ::core::ffi::c_int
                - *pix2.offset(2isize) as ::core::ffi::c_int) as sum2_t)
                .wrapping_add(
                    ((*pix1.offset(6isize) as ::core::ffi::c_int
                        - *pix2.offset(6isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            a3 = ((*pix1.offset(3isize) as ::core::ffi::c_int
                - *pix2.offset(3isize) as ::core::ffi::c_int) as sum2_t)
                .wrapping_add(
                    ((*pix1.offset(7isize) as ::core::ffi::c_int
                        - *pix2.offset(7isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            let mut t0 = a0.wrapping_add(a1);
            let mut t1 = a0.wrapping_sub(a1);
            let mut t2 = a2.wrapping_add(a3);
            let mut t3 = a2.wrapping_sub(a3);
            tmp[i as usize][0usize] = t0.wrapping_add(t2);
            tmp[i as usize][2usize] = t0.wrapping_sub(t2);
            tmp[i as usize][1usize] = t1.wrapping_add(t3);
            tmp[i as usize][3usize] = t1.wrapping_sub(t3);
            i += 1;
            pix1 = pix1.offset(i_pix1);
            pix2 = pix2.offset(i_pix2);
        }
        while i_0 < 4i32 {
            let mut t0_0 = tmp[0usize][i_0 as usize].wrapping_add(tmp[1usize][i_0 as usize]);
            let mut t1_0 = tmp[0usize][i_0 as usize].wrapping_sub(tmp[1usize][i_0 as usize]);
            let mut t2_0 = tmp[2usize][i_0 as usize].wrapping_add(tmp[3usize][i_0 as usize]);
            let mut t3_0 = tmp[2usize][i_0 as usize].wrapping_sub(tmp[3usize][i_0 as usize]);
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
        ((sum as sum_t as sum2_t).wrapping_add(sum >> BITS_PER_SUM) >> 1i32) as ::core::ffi::c_int
    }
}
unsafe extern "C" fn x264_pixel_satd_16x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_8x4(
                pix1.offset(4isize * i_pix1),
                i_pix1,
                pix2.offset(4isize * i_pix2),
                i_pix2,
            );
        if 16i32 == 16i32 {
            sum += x264_pixel_satd_8x4(pix1.offset(8isize), i_pix1, pix2.offset(8isize), i_pix2)
                + x264_pixel_satd_8x4(
                    pix1.offset(8isize).offset(4isize * i_pix1),
                    i_pix1,
                    pix2.offset(8isize).offset(4isize * i_pix2),
                    i_pix2,
                );
        }
        if 16i32 == 16i32 {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(12isize * i_pix2),
                i_pix2,
            );
        }
        if 16i32 == 16i32 && 16i32 == 16i32 {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8isize).offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8isize).offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(12isize * i_pix2),
                i_pix2,
            );
        }
        sum
    }
}
unsafe extern "C" fn x264_pixel_satd_16x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_8x4(
                pix1.offset(4isize * i_pix1),
                i_pix1,
                pix2.offset(4isize * i_pix2),
                i_pix2,
            );
        if 16i32 == 16i32 {
            sum += x264_pixel_satd_8x4(pix1.offset(8isize), i_pix1, pix2.offset(8isize), i_pix2)
                + x264_pixel_satd_8x4(
                    pix1.offset(8isize).offset(4isize * i_pix1),
                    i_pix1,
                    pix2.offset(8isize).offset(4isize * i_pix2),
                    i_pix2,
                );
        }
        if 8i32 == 16i32 {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(12isize * i_pix2),
                i_pix2,
            );
        }
        if 16i32 == 16i32 && 8i32 == 16i32 {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8isize).offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8isize).offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(12isize * i_pix2),
                i_pix2,
            );
        }
        sum
    }
}
unsafe extern "C" fn x264_pixel_satd_8x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_8x4(
                pix1.offset(4isize * i_pix1),
                i_pix1,
                pix2.offset(4isize * i_pix2),
                i_pix2,
            );
        if 8i32 == 16i32 {
            sum += x264_pixel_satd_8x4(pix1.offset(8isize), i_pix1, pix2.offset(8isize), i_pix2)
                + x264_pixel_satd_8x4(
                    pix1.offset(8isize).offset(4isize * i_pix1),
                    i_pix1,
                    pix2.offset(8isize).offset(4isize * i_pix2),
                    i_pix2,
                );
        }
        if 16i32 == 16i32 {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(12isize * i_pix2),
                i_pix2,
            );
        }
        if 8i32 == 16i32 && 16i32 == 16i32 {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8isize).offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8isize).offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(12isize * i_pix2),
                i_pix2,
            );
        }
        sum
    }
}
unsafe extern "C" fn x264_pixel_satd_8x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_8x4(
                pix1.offset(4isize * i_pix1),
                i_pix1,
                pix2.offset(4isize * i_pix2),
                i_pix2,
            );
        if 8i32 == 16i32 {
            sum += x264_pixel_satd_8x4(pix1.offset(8isize), i_pix1, pix2.offset(8isize), i_pix2)
                + x264_pixel_satd_8x4(
                    pix1.offset(8isize).offset(4isize * i_pix1),
                    i_pix1,
                    pix2.offset(8isize).offset(4isize * i_pix2),
                    i_pix2,
                );
        }
        if 8i32 == 16i32 {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(12isize * i_pix2),
                i_pix2,
            );
        }
        if 8i32 == 16i32 && 8i32 == 16i32 {
            sum += x264_pixel_satd_8x4(
                pix1.offset(8isize).offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_8x4(
                pix1.offset(8isize).offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(12isize * i_pix2),
                i_pix2,
            );
        }
        sum
    }
}
unsafe extern "C" fn x264_pixel_satd_4x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum = x264_pixel_satd_4x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_4x4(
                pix1.offset(4isize * i_pix1),
                i_pix1,
                pix2.offset(4isize * i_pix2),
                i_pix2,
            );
        if 4i32 == 16i32 {
            sum += x264_pixel_satd_4x4(pix1.offset(8isize), i_pix1, pix2.offset(8isize), i_pix2)
                + x264_pixel_satd_4x4(
                    pix1.offset(8isize).offset(4isize * i_pix1),
                    i_pix1,
                    pix2.offset(8isize).offset(4isize * i_pix2),
                    i_pix2,
                );
        }
        if 16i32 == 16i32 {
            sum += x264_pixel_satd_4x4(
                pix1.offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_4x4(
                pix1.offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(12isize * i_pix2),
                i_pix2,
            );
        }
        if 4i32 == 16i32 && 16i32 == 16i32 {
            sum += x264_pixel_satd_4x4(
                pix1.offset(8isize).offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_4x4(
                pix1.offset(8isize).offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(12isize * i_pix2),
                i_pix2,
            );
        }
        sum
    }
}
unsafe extern "C" fn x264_pixel_satd_4x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum = x264_pixel_satd_4x4(pix1, i_pix1, pix2, i_pix2)
            + x264_pixel_satd_4x4(
                pix1.offset(4isize * i_pix1),
                i_pix1,
                pix2.offset(4isize * i_pix2),
                i_pix2,
            );
        if 4i32 == 16i32 {
            sum += x264_pixel_satd_4x4(pix1.offset(8isize), i_pix1, pix2.offset(8isize), i_pix2)
                + x264_pixel_satd_4x4(
                    pix1.offset(8isize).offset(4isize * i_pix1),
                    i_pix1,
                    pix2.offset(8isize).offset(4isize * i_pix2),
                    i_pix2,
                );
        }
        if 8i32 == 16i32 {
            sum += x264_pixel_satd_4x4(
                pix1.offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_4x4(
                pix1.offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(12isize * i_pix2),
                i_pix2,
            );
        }
        if 4i32 == 16i32 && 8i32 == 16i32 {
            sum += x264_pixel_satd_4x4(
                pix1.offset(8isize).offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(8isize * i_pix2),
                i_pix2,
            ) + x264_pixel_satd_4x4(
                pix1.offset(8isize).offset(12isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(12isize * i_pix2),
                i_pix2,
            );
        }
        sum
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
        let mut tmp = [[0; 4]; 8];
        let mut a0 = 0;
        let mut a1 = 0;
        let mut a2 = 0;
        let mut a3 = 0;
        let mut a4 = 0;
        let mut a5 = 0;
        let mut a6 = 0;
        let mut a7 = 0;
        let mut b0 = 0;
        let mut sum = 0u32;
        let mut i = 0i32;
        let mut i_0 = 0i32;
        while i < 8i32 {
            let mut b1 = 0;
            let mut b2 = 0;
            let mut b3 = 0;
            a0 = (*pix1.offset(0isize) as ::core::ffi::c_int
                - *pix2.offset(0isize) as ::core::ffi::c_int) as sum2_t;
            a1 = (*pix1.offset(1isize) as ::core::ffi::c_int
                - *pix2.offset(1isize) as ::core::ffi::c_int) as sum2_t;
            b0 = a0
                .wrapping_add(a1)
                .wrapping_add(a0.wrapping_sub(a1) << BITS_PER_SUM);
            a2 = (*pix1.offset(2isize) as ::core::ffi::c_int
                - *pix2.offset(2isize) as ::core::ffi::c_int) as sum2_t;
            a3 = (*pix1.offset(3isize) as ::core::ffi::c_int
                - *pix2.offset(3isize) as ::core::ffi::c_int) as sum2_t;
            b1 = a2
                .wrapping_add(a3)
                .wrapping_add(a2.wrapping_sub(a3) << BITS_PER_SUM);
            a4 = (*pix1.offset(4isize) as ::core::ffi::c_int
                - *pix2.offset(4isize) as ::core::ffi::c_int) as sum2_t;
            a5 = (*pix1.offset(5isize) as ::core::ffi::c_int
                - *pix2.offset(5isize) as ::core::ffi::c_int) as sum2_t;
            b2 = a4
                .wrapping_add(a5)
                .wrapping_add(a4.wrapping_sub(a5) << BITS_PER_SUM);
            a6 = (*pix1.offset(6isize) as ::core::ffi::c_int
                - *pix2.offset(6isize) as ::core::ffi::c_int) as sum2_t;
            a7 = (*pix1.offset(7isize) as ::core::ffi::c_int
                - *pix2.offset(7isize) as ::core::ffi::c_int) as sum2_t;
            b3 = a6
                .wrapping_add(a7)
                .wrapping_add(a6.wrapping_sub(a7) << BITS_PER_SUM);
            let mut t0 = b0.wrapping_add(b1);
            let mut t1 = b0.wrapping_sub(b1);
            let mut t2 = b2.wrapping_add(b3);
            let mut t3 = b2.wrapping_sub(b3);
            tmp[i as usize][0usize] = t0.wrapping_add(t2);
            tmp[i as usize][2usize] = t0.wrapping_sub(t2);
            tmp[i as usize][1usize] = t1.wrapping_add(t3);
            tmp[i as usize][3usize] = t1.wrapping_sub(t3);
            i += 1;
            pix1 = pix1.offset(i_pix1);
            pix2 = pix2.offset(i_pix2);
        }
        while i_0 < 4i32 {
            let mut t0_0 = tmp[0usize][i_0 as usize].wrapping_add(tmp[1usize][i_0 as usize]);
            let mut t1_0 = tmp[0usize][i_0 as usize].wrapping_sub(tmp[1usize][i_0 as usize]);
            let mut t2_0 = tmp[2usize][i_0 as usize].wrapping_add(tmp[3usize][i_0 as usize]);
            let mut t3_0 = tmp[2usize][i_0 as usize].wrapping_sub(tmp[3usize][i_0 as usize]);
            a0 = t0_0.wrapping_add(t2_0);
            a2 = t0_0.wrapping_sub(t2_0);
            a1 = t1_0.wrapping_add(t3_0);
            a3 = t1_0.wrapping_sub(t3_0);
            let mut t0_1 = tmp[4usize][i_0 as usize].wrapping_add(tmp[5usize][i_0 as usize]);
            let mut t1_1 = tmp[4usize][i_0 as usize].wrapping_sub(tmp[5usize][i_0 as usize]);
            let mut t2_1 = tmp[6usize][i_0 as usize].wrapping_add(tmp[7usize][i_0 as usize]);
            let mut t3_1 = tmp[6usize][i_0 as usize].wrapping_sub(tmp[7usize][i_0 as usize]);
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
        sum as ::core::ffi::c_int
    }
}
unsafe extern "C" fn x264_pixel_sa8d_8x8(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum = sa8d_8x8(pix1, i_pix1, pix2, i_pix2);
        (sum + 2i32) >> 2i32
    }
}
unsafe extern "C" fn x264_pixel_sa8d_16x16(
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: crate::stdlib::intptr_t,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: crate::stdlib::intptr_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum = sa8d_8x8(pix1, i_pix1, pix2, i_pix2)
            + sa8d_8x8(pix1.offset(8isize), i_pix1, pix2.offset(8isize), i_pix2)
            + sa8d_8x8(
                pix1.offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize * i_pix2),
                i_pix2,
            )
            + sa8d_8x8(
                pix1.offset(8isize).offset(8isize * i_pix1),
                i_pix1,
                pix2.offset(8isize).offset(8isize * i_pix2),
                i_pix2,
            );
        (sum + 2i32) >> 2i32
    }
}
#[inline(never)]
unsafe extern "C" fn pixel_hadamard_ac(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut tmp = [0; 32];
        let mut a0 = 0;
        let mut a1 = 0;
        let mut a2 = 0;
        let mut a3 = 0;
        let mut sum4 = 0u32;
        let mut sum8 = 0u32;
        let mut i = 0i32;
        let mut i_0 = 0i32;
        let mut i_1 = 0i32;
        while i < 8i32 {
            let mut t = (&raw mut tmp as *mut sum2_t)
                .offset((i & 3i32) as isize)
                .offset(((i & 4i32) * 4i32) as isize);
            a0 = ((*pix.offset(0isize) as ::core::ffi::c_int
                + *pix.offset(1isize) as ::core::ffi::c_int) as sum2_t)
                .wrapping_add(
                    ((*pix.offset(0isize) as ::core::ffi::c_int
                        - *pix.offset(1isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            a1 = ((*pix.offset(2isize) as ::core::ffi::c_int
                + *pix.offset(3isize) as ::core::ffi::c_int) as sum2_t)
                .wrapping_add(
                    ((*pix.offset(2isize) as ::core::ffi::c_int
                        - *pix.offset(3isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            *t.offset(0isize) = a0.wrapping_add(a1);
            *t.offset(4isize) = a0.wrapping_sub(a1);
            a2 = ((*pix.offset(4isize) as ::core::ffi::c_int
                + *pix.offset(5isize) as ::core::ffi::c_int) as sum2_t)
                .wrapping_add(
                    ((*pix.offset(4isize) as ::core::ffi::c_int
                        - *pix.offset(5isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            a3 = ((*pix.offset(6isize) as ::core::ffi::c_int
                + *pix.offset(7isize) as ::core::ffi::c_int) as sum2_t)
                .wrapping_add(
                    ((*pix.offset(6isize) as ::core::ffi::c_int
                        - *pix.offset(7isize) as ::core::ffi::c_int)
                        as sum2_t)
                        << BITS_PER_SUM,
                );
            *t.offset(8isize) = a2.wrapping_add(a3);
            *t.offset(12isize) = a2.wrapping_sub(a3);
            i += 1;
            pix = pix.offset(stride);
        }
        while i_0 < 8i32 {
            let mut t0 = sum2_t::wrapping_add(
                tmp[(i_0 * 4i32 + 0i32) as usize],
                tmp[(i_0 * 4i32 + 1i32) as usize],
            );
            let mut t1 = sum2_t::wrapping_sub(
                tmp[(i_0 * 4i32 + 0i32) as usize],
                tmp[(i_0 * 4i32 + 1i32) as usize],
            );
            let mut t2 = sum2_t::wrapping_add(
                tmp[(i_0 * 4i32 + 2i32) as usize],
                tmp[(i_0 * 4i32 + 3i32) as usize],
            );
            let mut t3 = sum2_t::wrapping_sub(
                tmp[(i_0 * 4i32 + 2i32) as usize],
                tmp[(i_0 * 4i32 + 3i32) as usize],
            );
            a0 = t0.wrapping_add(t2);
            a2 = t0.wrapping_sub(t2);
            a1 = t1.wrapping_add(t3);
            a3 = t1.wrapping_sub(t3);
            tmp[(i_0 * 4i32 + 0i32) as usize] = a0;
            tmp[(i_0 * 4i32 + 1i32) as usize] = a1;
            tmp[(i_0 * 4i32 + 2i32) as usize] = a2;
            tmp[(i_0 * 4i32 + 3i32) as usize] = a3;
            sum4 = sum4.wrapping_add(
                abs2(a0)
                    .wrapping_add(abs2(a1))
                    .wrapping_add(abs2(a2))
                    .wrapping_add(abs2(a3)),
            );
            i_0 += 1;
        }
        while i_1 < 8i32 {
            let mut t0_0 = tmp[i_1 as usize].wrapping_add(tmp[(8i32 + i_1) as usize]);
            let mut t1_0 = tmp[i_1 as usize].wrapping_sub(tmp[(8i32 + i_1) as usize]);
            let mut t2_0 = tmp[(16i32 + i_1) as usize].wrapping_add(tmp[(24i32 + i_1) as usize]);
            let mut t3_0 = tmp[(16i32 + i_1) as usize].wrapping_sub(tmp[(24i32 + i_1) as usize]);
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
        let mut dc = tmp[0usize]
            .wrapping_add(tmp[8usize])
            .wrapping_add(tmp[16usize])
            .wrapping_add(tmp[24usize]) as sum_t as sum2_t;
        sum4 = (sum4 as sum_t as sum2_t)
            .wrapping_add(sum4 >> BITS_PER_SUM)
            .wrapping_sub(dc);
        sum8 = (sum8 as sum_t as sum2_t)
            .wrapping_add(sum8 >> BITS_PER_SUM)
            .wrapping_sub(dc);
        ((sum8 as crate::stdlib::uint64_t) << 32i32).wrapping_add(sum4 as crate::stdlib::uint64_t)
    }
}
unsafe extern "C" fn x264_pixel_hadamard_ac_16x16(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum = pixel_hadamard_ac(pix, stride);
        if 16i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8isize), stride));
        }
        if 16i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8isize * stride), stride));
        }
        if 16i32 == 16i32 && 16i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset(8isize * stride).offset(8isize),
                stride,
            ));
        }
        ((sum >> 34i32) << 32i32)
            .wrapping_add((sum as crate::stdlib::uint32_t >> 1i32) as crate::stdlib::uint64_t)
    }
}
unsafe extern "C" fn x264_pixel_hadamard_ac_16x8(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum = pixel_hadamard_ac(pix, stride);
        if 16i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8isize), stride));
        }
        if 8i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8isize * stride), stride));
        }
        if 16i32 == 16i32 && 8i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset(8isize * stride).offset(8isize),
                stride,
            ));
        }
        ((sum >> 34i32) << 32i32)
            .wrapping_add((sum as crate::stdlib::uint32_t >> 1i32) as crate::stdlib::uint64_t)
    }
}
unsafe extern "C" fn x264_pixel_hadamard_ac_8x16(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum = pixel_hadamard_ac(pix, stride);
        if 8i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8isize), stride));
        }
        if 16i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8isize * stride), stride));
        }
        if 8i32 == 16i32 && 16i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset(8isize * stride).offset(8isize),
                stride,
            ));
        }
        ((sum >> 34i32) << 32i32)
            .wrapping_add((sum as crate::stdlib::uint32_t >> 1i32) as crate::stdlib::uint64_t)
    }
}
unsafe extern "C" fn x264_pixel_hadamard_ac_8x8(
    mut pix: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
) -> crate::stdlib::uint64_t {
    unsafe {
        let mut sum = pixel_hadamard_ac(pix, stride);
        if 8i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8isize), stride));
        }
        if 8i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8isize * stride), stride));
        }
        if 8i32 == 16i32 && 8i32 == 16i32 {
            sum = sum.wrapping_add(pixel_hadamard_ac(
                pix.offset(8isize * stride).offset(8isize),
                stride,
            ));
        }
        ((sum >> 34i32) << 32i32)
            .wrapping_add((sum as crate::stdlib::uint32_t >> 1i32) as crate::stdlib::uint64_t)
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
        *scores.offset(0isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_16x16(
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
        *scores.offset(0isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_sad_16x16(
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
        *scores.offset(0isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_16x8(
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
        *scores.offset(0isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_sad_16x8(
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
        *scores.offset(0isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_8x16(
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
        *scores.offset(0isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_sad_8x16(
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
        *scores.offset(0isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_8x8(
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
        *scores.offset(0isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_sad_8x8(
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
        *scores.offset(0isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_sad_8x4(
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
        *scores.offset(0isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_8x4(
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
        *scores.offset(0isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_sad_4x8(
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
        *scores.offset(0isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_4x8(
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
        *scores.offset(0isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_4x4(
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
        *scores.offset(0isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_sad_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_sad_4x4(
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
        *scores.offset(0isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_satd_8x8(
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
        *scores.offset(0isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_4x8(
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
        *scores.offset(0isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_8x4(
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
        *scores.offset(0isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_8x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_8x8(
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
        *scores.offset(0isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_8x16(
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
        *scores.offset(0isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_16x8(
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
        *scores.offset(0isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_16x16(
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
        *scores.offset(0isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_8x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_satd_8x16(
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
        *scores.offset(0isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_4x4(
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
        *scores.offset(0isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_8x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_satd_8x4(
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
        *scores.offset(0isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_4x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_satd_4x8(
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
        *scores.offset(0isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_4x4(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_satd_4x4(
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
        *scores.offset(0isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_16x8(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_satd_16x8(
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
        *scores.offset(0isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix0,
            i_stride,
        );
        *scores.offset(1isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix1,
            i_stride,
        );
        *scores.offset(2isize) = x264_pixel_satd_16x16(
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
            pix2,
            i_stride,
        );
        *scores.offset(3isize) = x264_pixel_satd_16x16(
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
        let mut pix = [0; 256];
        crate::src::common::predict::x264_8_predict_8x8_v_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(0isize) = x264_pixel_sad_8x8(
            &raw mut pix as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8_h_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(1isize) = x264_pixel_sad_8x8(
            &raw mut pix as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8_dc_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(2isize) = x264_pixel_sad_8x8(
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
        let mut pix = [0; 256];
        crate::src::common::predict::x264_8_predict_8x8_v_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(0isize) = x264_pixel_sa8d_8x8(
            &raw mut pix as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8_h_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(1isize) = x264_pixel_sa8d_8x8(
            &raw mut pix as *mut crate::src::common::common::pixel,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8_dc_c(
            &raw mut pix as *mut crate::src::common::common::pixel,
            edge,
        );
        *res.offset(2isize) = x264_pixel_sa8d_8x8(
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
        *res.offset(0isize) = x264_pixel_sad_4x4(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_4x4_h_c(fdec);
        *res.offset(1isize) = x264_pixel_sad_4x4(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_4x4_dc_c(fdec);
        *res.offset(2isize) = x264_pixel_sad_4x4(
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
        *res.offset(0isize) = x264_pixel_satd_4x4(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_4x4_h_c(fdec);
        *res.offset(1isize) = x264_pixel_satd_4x4(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_4x4_dc_c(fdec);
        *res.offset(2isize) = x264_pixel_satd_4x4(
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
        *res.offset(0isize) = x264_pixel_sad_8x8(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8c_h_c(fdec);
        *res.offset(1isize) = x264_pixel_sad_8x8(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8c_v_c(fdec);
        *res.offset(2isize) = x264_pixel_sad_8x8(
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
        *res.offset(0isize) = x264_pixel_satd_8x8(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8c_h_c(fdec);
        *res.offset(1isize) = x264_pixel_satd_8x8(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x8c_v_c(fdec);
        *res.offset(2isize) = x264_pixel_satd_8x8(
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
        *res.offset(0isize) = x264_pixel_sad_8x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x16c_h_c(fdec);
        *res.offset(1isize) = x264_pixel_sad_8x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x16c_v_c(fdec);
        *res.offset(2isize) = x264_pixel_sad_8x16(
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
        *res.offset(0isize) = x264_pixel_satd_8x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x16c_h_c(fdec);
        *res.offset(1isize) = x264_pixel_satd_8x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_8x16c_v_c(fdec);
        *res.offset(2isize) = x264_pixel_satd_8x16(
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
        *res.offset(0isize) = x264_pixel_sad_16x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_16x16_h_c(fdec);
        *res.offset(1isize) = x264_pixel_sad_16x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_16x16_dc_c(fdec);
        *res.offset(2isize) = x264_pixel_sad_16x16(
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
        *res.offset(0isize) = x264_pixel_satd_16x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_16x16_h_c(fdec);
        *res.offset(1isize) = x264_pixel_satd_16x16(
            fdec,
            crate::src::common::common::FDEC_STRIDE as crate::stdlib::intptr_t,
            fenc,
            crate::src::common::common::FENC_STRIDE as crate::stdlib::intptr_t,
        );
        crate::src::common::predict::x264_8_predict_16x16_dc_c(fdec);
        *res.offset(2isize) = x264_pixel_satd_16x16(
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
        let mut z = 0i32;
        while z < 2i32 {
            let mut s1 = 0u32;
            let mut s2 = 0u32;
            let mut ss = 0u32;
            let mut s12 = 0u32;
            let mut y = 0i32;
            while y < 4i32 {
                let mut x = 0i32;
                while x < 4i32 {
                    let mut a = *pix1.offset(
                        x as crate::stdlib::intptr_t + y as crate::stdlib::intptr_t * stride1,
                    ) as ::core::ffi::c_int;
                    let mut b = *pix2.offset(
                        x as crate::stdlib::intptr_t + y as crate::stdlib::intptr_t * stride2,
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
            (*sums.offset(z as isize))[0usize] = s1 as ::core::ffi::c_int;
            (*sums.offset(z as isize))[1usize] = s2 as ::core::ffi::c_int;
            (*sums.offset(z as isize))[2usize] = ss as ::core::ffi::c_int;
            (*sums.offset(z as isize))[3usize] = s12 as ::core::ffi::c_int;
            pix1 = pix1.offset(4isize);
            pix2 = pix2.offset(4isize);
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
        static mut ssim_c1: ::core::ffi::c_int = (0.01
            * 0.01
            * crate::src::common::common::PIXEL_MAX as ::core::ffi::c_double
            * crate::src::common::common::PIXEL_MAX as ::core::ffi::c_double
            * 64f64
            + 0.5) as ::core::ffi::c_int;
        static mut ssim_c2: ::core::ffi::c_int = (0.03
            * 0.03
            * crate::src::common::common::PIXEL_MAX as ::core::ffi::c_double
            * crate::src::common::common::PIXEL_MAX as ::core::ffi::c_double
            * 64f64
            * 63f64
            + 0.5) as ::core::ffi::c_int;
        let mut fs1 = s1;
        let mut fs2 = s2;
        let mut fss = ss;
        let mut fs12 = s12;
        let mut vars = fss * 64i32 - fs1 * fs1 - fs2 * fs2;
        let mut covar = fs12 * 64i32 - fs1 * fs2;
        (2i32 * fs1 * fs2 + ssim_c1) as ::core::ffi::c_float
            * (2i32 * covar + ssim_c2) as ::core::ffi::c_float
            / ((fs1 * fs1 + fs2 * fs2 + ssim_c1) as ::core::ffi::c_float
                * (vars + ssim_c2) as ::core::ffi::c_float)
    }
}
unsafe extern "C" fn ssim_end4(
    mut sum0: *mut [::core::ffi::c_int; 4],
    mut sum1: *mut [::core::ffi::c_int; 4],
    mut width: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    unsafe {
        let mut ssim = 0.0;
        let mut i = 0i32;
        while i < width {
            ssim += ssim_end1(
                (*sum0.offset(i as isize))[0usize]
                    + (*sum0.offset((i + 1i32) as isize))[0usize]
                    + (*sum1.offset(i as isize))[0usize]
                    + (*sum1.offset((i + 1i32) as isize))[0usize],
                (*sum0.offset(i as isize))[1usize]
                    + (*sum0.offset((i + 1i32) as isize))[1usize]
                    + (*sum1.offset(i as isize))[1usize]
                    + (*sum1.offset((i + 1i32) as isize))[1usize],
                (*sum0.offset(i as isize))[2usize]
                    + (*sum0.offset((i + 1i32) as isize))[2usize]
                    + (*sum1.offset(i as isize))[2usize]
                    + (*sum1.offset((i + 1i32) as isize))[2usize],
                (*sum0.offset(i as isize))[3usize]
                    + (*sum0.offset((i + 1i32) as isize))[3usize]
                    + (*sum1.offset(i as isize))[3usize]
                    + (*sum1.offset((i + 1i32) as isize))[3usize],
            );
            i += 1;
        }
        ssim
    }
}
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
        let mut ssim = 0.0;
        let mut y = 1i32;
        let mut sum0 = buf as *mut [::core::ffi::c_int; 4];
        let mut sum1 = sum0.offset((width >> 2i32) as isize).offset(3isize);
        width >>= 2i32;
        height >>= 2i32;
        while y < height {
            let mut z = 0i32;
            let mut x_0 = 0i32;
            while z <= y {
                let mut x = 0i32;
                let mut t = sum0 as *mut ::core::ffi::c_void;
                sum0 = sum1;
                sum1 = t as *mut [::core::ffi::c_int; 4];
                while x < width {
                    (*pf).ssim_4x4x2_core.expect("non-null function pointer")(
                        pix1.offset(
                            4isize
                                * (x as crate::stdlib::intptr_t
                                    + z as crate::stdlib::intptr_t * stride1),
                        ),
                        stride1,
                        pix2.offset(
                            4isize
                                * (x as crate::stdlib::intptr_t
                                    + z as crate::stdlib::intptr_t * stride2),
                        ),
                        stride2,
                        sum0.offset(x as isize),
                    );
                    x += 2i32;
                }
                z += 1;
            }
            while x_0 < width - 1i32 {
                ssim += (*pf).ssim_end4.expect("non-null function pointer")(
                    sum0.offset(x_0 as isize),
                    sum1.offset(x_0 as isize),
                    if (4i32) < width - x_0 - 1i32 {
                        4i32
                    } else {
                        width - x_0 - 1i32
                    },
                );
                x_0 += 4i32;
            }
            y += 1;
        }
        *cnt = (height - 1i32) * (width - 1i32);
        ssim
    }
}
unsafe extern "C" fn pixel_vsad(
    mut src: *mut crate::src::common::common::pixel,
    mut stride: crate::stdlib::intptr_t,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut score = 0i32;
        let mut i = 1i32;
        while i < height {
            let mut j = 0i32;
            while j < 16i32 {
                score += crate::stdlib::abs(
                    *src.offset(j as isize) as ::core::ffi::c_int
                        - *src.offset(j as crate::stdlib::intptr_t + stride) as ::core::ffi::c_int,
                );
                j += 1;
            }
            i += 1;
            src = src.offset(stride);
        }
        score
    }
}
pub unsafe extern "C" fn x264_8_field_vsad(
    mut h: *mut crate::src::common::common::x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) -> bool {
    unsafe {
        let mut stride = (*(*h).fenc).i_stride[0usize];
        let mut mb_stride = (*h).mb.i_mb_stride;
        let mut fenc = (*(*h).fenc).plane[0usize].offset((16i32 * (mb_x + mb_y * stride)) as isize);
        let mut mb_xy = mb_x + mb_y * mb_stride;
        let mut mbpair_height = if ((*h).param.i_height - mb_y * 16i32) < 32i32 {
            (*h).param.i_height - mb_y * 16i32
        } else {
            32i32
        };
        let mut score_frame = (*h).pixf.vsad.expect("non-null function pointer")(
            fenc,
            stride as crate::stdlib::intptr_t,
            mbpair_height,
        );
        let mut score_field = (*h).pixf.vsad.expect("non-null function pointer")(
            fenc,
            (stride * 2i32) as crate::stdlib::intptr_t,
            mbpair_height >> 1i32,
        );
        score_field += (*h).pixf.vsad.expect("non-null function pointer")(
            fenc.offset(stride as isize),
            (stride * 2i32) as crate::stdlib::intptr_t,
            mbpair_height >> 1i32,
        );
        if mb_x > 0i32 {
            score_field += 512i32
                - *(*h).mb.field.offset((mb_xy - 1i32) as isize) as ::core::ffi::c_int * 1024i32;
        }
        if mb_y > 0i32 {
            score_field += 512i32
                - *(*h).mb.field.offset((mb_xy - mb_stride) as isize) as ::core::ffi::c_int
                    * 1024i32;
        }
        score_field < score_frame
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
        let mut sum = 0i32;
        let mut y = 0i32;
        while y < height {
            let mut x = 0i32;
            while x < 8i32 {
                sum += *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
                x += 1;
            }
            y += 1;
            pix1 = pix1.offset(stride1);
            pix2 = pix2.offset(stride2);
        }
        crate::stdlib::abs(sum)
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
        let mut nmv = 0i32;
        let mut i = 0i32;
        while i < width {
            let mut ads = crate::stdlib::abs(
                *enc_dc.offset(0isize) - *sums.offset(0isize) as ::core::ffi::c_int,
            ) + crate::stdlib::abs(
                *enc_dc.offset(1isize) - *sums.offset(8isize) as ::core::ffi::c_int,
            ) + crate::stdlib::abs(
                *enc_dc.offset(2isize) - *sums.offset(delta as isize) as ::core::ffi::c_int,
            ) + crate::stdlib::abs(
                *enc_dc.offset(3isize)
                    - *sums.offset((delta + 8i32) as isize) as ::core::ffi::c_int,
            ) + *cost_mvx.offset(i as isize) as ::core::ffi::c_int;
            if ads < thresh {
                let c2rust_fresh2 = nmv;
                nmv += 1;
                *mvs.offset(c2rust_fresh2 as isize) = i as crate::stdlib::int16_t;
            }
            i += 1;
            sums = sums.offset(1);
        }
        nmv
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
        let mut nmv = 0i32;
        let mut i = 0i32;
        while i < width {
            let mut ads = crate::stdlib::abs(
                *enc_dc.offset(0isize) - *sums.offset(0isize) as ::core::ffi::c_int,
            ) + crate::stdlib::abs(
                *enc_dc.offset(1isize) - *sums.offset(delta as isize) as ::core::ffi::c_int,
            ) + *cost_mvx.offset(i as isize) as ::core::ffi::c_int;
            if ads < thresh {
                let c2rust_fresh1 = nmv;
                nmv += 1;
                *mvs.offset(c2rust_fresh1 as isize) = i as crate::stdlib::int16_t;
            }
            i += 1;
            sums = sums.offset(1);
        }
        nmv
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
        let mut nmv = 0i32;
        let mut i = 0i32;
        while i < width {
            let mut ads = crate::stdlib::abs(
                *enc_dc.offset(0isize) - *sums.offset(0isize) as ::core::ffi::c_int,
            ) + *cost_mvx.offset(i as isize) as ::core::ffi::c_int;
            if ads < thresh {
                let c2rust_fresh0 = nmv;
                nmv += 1;
                *mvs.offset(c2rust_fresh0 as isize) = i as crate::stdlib::int16_t;
            }
            i += 1;
            sums = sums.offset(1);
        }
        nmv
    }
}
pub unsafe extern "C" fn x264_8_pixel_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pixf: *mut crate::src::common::pixel::x264_pixel_function_t,
) {
    unsafe {
        crate::stdlib::memset(
            pixf as *mut ::core::ffi::c_void,
            0i32,
            ::core::mem::size_of::<crate::src::common::pixel::x264_pixel_function_t>(),
        );
        (*pixf).sad[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).sad[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_16x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).sad[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).sad[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).sad[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_8x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).sad[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_4x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).sad[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).sad[crate::src::common::pixel::PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_4x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).sad_aligned
            [crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sad_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_16x8
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            );
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_8x16
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            );
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_8x8
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            );
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_8x4
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            );
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_4x8
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            );
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_4x4
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            );
        (*pixf).sad_aligned[crate::src::common::pixel::PIXEL_4x16 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_4x16
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> ::core::ffi::c_int,
            );
        (*pixf).sad_x3[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_sad_x3_16x16
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut ::core::ffi::c_int,
                    ) -> (),
            );
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
        );
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
        );
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
        );
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
        );
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
        );
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
        );
        (*pixf).sad_x4[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] =
            Some(
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
            );
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
        );
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
        );
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
        );
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
        );
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
        );
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
        );
        (*pixf).ssd[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).ssd[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_16x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).ssd[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).ssd[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).ssd[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_8x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).ssd[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_4x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).ssd[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).ssd[crate::src::common::pixel::PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_ssd_4x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).satd[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).satd[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_16x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).satd[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).satd[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).satd[crate::src::common::pixel::PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_8x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).satd[crate::src::common::pixel::PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_4x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).satd[crate::src::common::pixel::PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).satd[crate::src::common::pixel::PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_satd_4x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
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
            );
        (*pixf).satd_x3[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_satd_x3_16x8
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut ::core::ffi::c_int,
                    ) -> (),
            );
        (*pixf).satd_x3[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_satd_x3_8x16
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                        *mut ::core::ffi::c_int,
                    ) -> (),
            );
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
        );
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
        );
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
        );
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
        );
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
            );
        (*pixf).satd_x4[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] =
            Some(
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
            );
        (*pixf).satd_x4[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] =
            Some(
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
            );
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
        );
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
        );
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
        );
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
        );
        (*pixf).hadamard_ac
            [crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_hadamard_ac_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
        );
        (*pixf).hadamard_ac[crate::src::common::pixel::PIXEL_16x8 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_hadamard_ac_16x8
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> crate::stdlib::uint64_t,
            );
        (*pixf).hadamard_ac[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_hadamard_ac_8x16
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> crate::stdlib::uint64_t,
            );
        (*pixf).hadamard_ac[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] =
            Some(
                x264_pixel_hadamard_ac_8x8
                    as unsafe extern "C" fn(
                        *mut crate::src::common::common::pixel,
                        crate::stdlib::intptr_t,
                    ) -> crate::stdlib::uint64_t,
            );
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
        );
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
        );
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
        );
        (*pixf).sa8d[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sa8d_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).sa8d[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            x264_pixel_sa8d_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).var[crate::src::common::pixel::PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
            pixel_var_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
        );
        (*pixf).var[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            pixel_var_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
        );
        (*pixf).var[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            pixel_var_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                ) -> crate::stdlib::uint64_t,
        );
        (*pixf).var2[crate::src::common::pixel::PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
            pixel_var2_8x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).var2[crate::src::common::pixel::PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
            pixel_var2_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
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
        );
        (*pixf).ssim_4x4x2_core = Some(
            ssim_4x4x2_core
                as unsafe extern "C" fn(
                    *const crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *const crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut [::core::ffi::c_int; 4],
                ) -> (),
        );
        (*pixf).ssim_end4 = Some(
            ssim_end4
                as unsafe extern "C" fn(
                    *mut [::core::ffi::c_int; 4],
                    *mut [::core::ffi::c_int; 4],
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_float,
        );
        (*pixf).vsad = Some(
            pixel_vsad
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).asd8 = Some(
            pixel_asd8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    *mut crate::src::common::common::pixel,
                    crate::stdlib::intptr_t,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        );
        (*pixf).intra_sad_x3_4x4 = Some(
            intra_sad_x3_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        );
        (*pixf).intra_satd_x3_4x4 = Some(
            intra_satd_x3_4x4
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        );
        (*pixf).intra_sad_x3_8x8 = Some(
            intra_sad_x3_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        );
        (*pixf).intra_sa8d_x3_8x8 = Some(
            intra_sa8d_x3_8x8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        );
        (*pixf).intra_sad_x3_8x8c = Some(
            intra_sad_x3_8x8c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        );
        (*pixf).intra_satd_x3_8x8c = Some(
            intra_satd_x3_8x8c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        );
        (*pixf).intra_sad_x3_8x16c = Some(
            intra_sad_x3_8x16c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        );
        (*pixf).intra_satd_x3_8x16c = Some(
            intra_satd_x3_8x16c
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        );
        (*pixf).intra_sad_x3_16x16 = Some(
            intra_sad_x3_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        );
        (*pixf).intra_satd_x3_16x16 = Some(
            intra_satd_x3_16x16
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut ::core::ffi::c_int,
                ) -> (),
        );
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
