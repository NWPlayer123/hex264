// =============== BEGIN dct_h ================
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_dct_function_t {
    pub sub4x4_dct: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
        ) -> (),
    >,
    pub add4x4_idct: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::dctcoef,
        ) -> (),
    >,
    pub sub8x8_dct: Option<
        unsafe extern "C" fn(
            *mut [crate::src::common::common::dctcoef; 16],
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
        ) -> (),
    >,
    pub sub8x8_dct_dc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
        ) -> (),
    >,
    pub add8x8_idct: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut [crate::src::common::common::dctcoef; 16],
        ) -> (),
    >,
    pub add8x8_idct_dc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::dctcoef,
        ) -> (),
    >,
    pub sub8x16_dct_dc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
        ) -> (),
    >,
    pub sub16x16_dct: Option<
        unsafe extern "C" fn(
            *mut [crate::src::common::common::dctcoef; 16],
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
        ) -> (),
    >,
    pub add16x16_idct: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut [crate::src::common::common::dctcoef; 16],
        ) -> (),
    >,
    pub add16x16_idct_dc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::dctcoef,
        ) -> (),
    >,
    pub sub8x8_dct8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
        ) -> (),
    >,
    pub add8x8_idct8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::dctcoef,
        ) -> (),
    >,
    pub sub16x16_dct8: Option<
        unsafe extern "C" fn(
            *mut [crate::src::common::common::dctcoef; 64],
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
        ) -> (),
    >,
    pub add16x16_idct8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::pixel,
            *mut [crate::src::common::common::dctcoef; 64],
        ) -> (),
    >,
    pub dct4x4dc: Option<unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ()>,
    pub idct4x4dc: Option<unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ()>,
    pub dct2x4dc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut [crate::src::common::common::dctcoef; 16],
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct x264_zigzag_function_t {
    pub scan_8x8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
        ) -> (),
    >,
    pub scan_4x4: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
        ) -> (),
    >,
    pub sub_8x8: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *const crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
        ) -> ::core::ffi::c_int,
    >,
    pub sub_4x4: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *const crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
        ) -> ::core::ffi::c_int,
    >,
    pub sub_4x4ac: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *const crate::src::common::common::pixel,
            *mut crate::src::common::common::pixel,
            *mut crate::src::common::common::dctcoef,
        ) -> ::core::ffi::c_int,
    >,
    pub interleave_8x8_cavlc: Option<
        unsafe extern "C" fn(
            *mut crate::src::common::common::dctcoef,
            *mut crate::src::common::common::dctcoef,
            *mut crate::stdlib::uint8_t,
        ) -> (),
    >,
}
pub mod common_h {
    #[inline(always)]
    pub unsafe extern "C" fn x264_clip_pixel(mut x: ::core::ffi::c_int) -> crate::src::common::common::pixel {
        (if x & !crate::src::common::common::PIXEL_MAX != 0 {
            -x >> 31i32 & crate::src::common::common::PIXEL_MAX
        } else {
            x
        }) as crate::src::common::common::pixel
    }
}
use crate::src::common::dct::common_h::x264_clip_pixel;
unsafe extern "C" fn dct4x4dc(mut d: *mut crate::src::common::common::dctcoef) {
    unsafe {
        let mut tmp = [0; 16];
        let mut i = 0i32;
        let mut i_0 = 0i32;
        while i < 4i32 {
            let mut s01 = *d.offset((i * 4i32 + 0i32) as isize) as ::core::ffi::c_int
                + *d.offset((i * 4i32 + 1i32) as isize) as ::core::ffi::c_int;
            let mut d01 = *d.offset((i * 4i32 + 0i32) as isize) as ::core::ffi::c_int
                - *d.offset((i * 4i32 + 1i32) as isize) as ::core::ffi::c_int;
            let mut s23 = *d.offset((i * 4i32 + 2i32) as isize) as ::core::ffi::c_int
                + *d.offset((i * 4i32 + 3i32) as isize) as ::core::ffi::c_int;
            let mut d23 = *d.offset((i * 4i32 + 2i32) as isize) as ::core::ffi::c_int
                - *d.offset((i * 4i32 + 3i32) as isize) as ::core::ffi::c_int;
            tmp[(0i32 * 4i32 + i) as usize] = (s01 + s23) as crate::src::common::common::dctcoef;
            tmp[(1i32 * 4i32 + i) as usize] = (s01 - s23) as crate::src::common::common::dctcoef;
            tmp[(2i32 * 4i32 + i) as usize] = (d01 - d23) as crate::src::common::common::dctcoef;
            tmp[(3i32 * 4i32 + i) as usize] = (d01 + d23) as crate::src::common::common::dctcoef;
            i += 1;
        }
        while i_0 < 4i32 {
            let mut s01_0 = tmp[(i_0 * 4i32 + 0i32) as usize] as ::core::ffi::c_int
                + tmp[(i_0 * 4i32 + 1i32) as usize] as ::core::ffi::c_int;
            let mut d01_0 = tmp[(i_0 * 4i32 + 0i32) as usize] as ::core::ffi::c_int
                - tmp[(i_0 * 4i32 + 1i32) as usize] as ::core::ffi::c_int;
            let mut s23_0 = tmp[(i_0 * 4i32 + 2i32) as usize] as ::core::ffi::c_int
                + tmp[(i_0 * 4i32 + 3i32) as usize] as ::core::ffi::c_int;
            let mut d23_0 = tmp[(i_0 * 4i32 + 2i32) as usize] as ::core::ffi::c_int
                - tmp[(i_0 * 4i32 + 3i32) as usize] as ::core::ffi::c_int;
            *d.offset((i_0 * 4i32 + 0i32) as isize) =
                ((s01_0 + s23_0 + 1i32) >> 1i32) as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4i32 + 1i32) as isize) =
                ((s01_0 - s23_0 + 1i32) >> 1i32) as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4i32 + 2i32) as isize) =
                ((d01_0 - d23_0 + 1i32) >> 1i32) as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4i32 + 3i32) as isize) =
                ((d01_0 + d23_0 + 1i32) >> 1i32) as crate::src::common::common::dctcoef;
            i_0 += 1;
        }
    }
}
unsafe extern "C" fn idct4x4dc(mut d: *mut crate::src::common::common::dctcoef) {
    unsafe {
        let mut tmp = [0; 16];
        let mut i = 0i32;
        let mut i_0 = 0i32;
        while i < 4i32 {
            let mut s01 = *d.offset((i * 4i32 + 0i32) as isize) as ::core::ffi::c_int
                + *d.offset((i * 4i32 + 1i32) as isize) as ::core::ffi::c_int;
            let mut d01 = *d.offset((i * 4i32 + 0i32) as isize) as ::core::ffi::c_int
                - *d.offset((i * 4i32 + 1i32) as isize) as ::core::ffi::c_int;
            let mut s23 = *d.offset((i * 4i32 + 2i32) as isize) as ::core::ffi::c_int
                + *d.offset((i * 4i32 + 3i32) as isize) as ::core::ffi::c_int;
            let mut d23 = *d.offset((i * 4i32 + 2i32) as isize) as ::core::ffi::c_int
                - *d.offset((i * 4i32 + 3i32) as isize) as ::core::ffi::c_int;
            tmp[(0i32 * 4i32 + i) as usize] = (s01 + s23) as crate::src::common::common::dctcoef;
            tmp[(1i32 * 4i32 + i) as usize] = (s01 - s23) as crate::src::common::common::dctcoef;
            tmp[(2i32 * 4i32 + i) as usize] = (d01 - d23) as crate::src::common::common::dctcoef;
            tmp[(3i32 * 4i32 + i) as usize] = (d01 + d23) as crate::src::common::common::dctcoef;
            i += 1;
        }
        while i_0 < 4i32 {
            let mut s01_0 = tmp[(i_0 * 4i32 + 0i32) as usize] as ::core::ffi::c_int
                + tmp[(i_0 * 4i32 + 1i32) as usize] as ::core::ffi::c_int;
            let mut d01_0 = tmp[(i_0 * 4i32 + 0i32) as usize] as ::core::ffi::c_int
                - tmp[(i_0 * 4i32 + 1i32) as usize] as ::core::ffi::c_int;
            let mut s23_0 = tmp[(i_0 * 4i32 + 2i32) as usize] as ::core::ffi::c_int
                + tmp[(i_0 * 4i32 + 3i32) as usize] as ::core::ffi::c_int;
            let mut d23_0 = tmp[(i_0 * 4i32 + 2i32) as usize] as ::core::ffi::c_int
                - tmp[(i_0 * 4i32 + 3i32) as usize] as ::core::ffi::c_int;
            *d.offset((i_0 * 4i32 + 0i32) as isize) = (s01_0 + s23_0) as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4i32 + 1i32) as isize) = (s01_0 - s23_0) as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4i32 + 2i32) as isize) = (d01_0 - d23_0) as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4i32 + 3i32) as isize) = (d01_0 + d23_0) as crate::src::common::common::dctcoef;
            i_0 += 1;
        }
    }
}
unsafe extern "C" fn dct2x4dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dct4x4: *mut [crate::src::common::common::dctcoef; 16],
) {
    unsafe {
        let mut a0 = (*dct4x4.offset(0isize))[0usize] as ::core::ffi::c_int
            + (*dct4x4.offset(1isize))[0usize] as ::core::ffi::c_int;
        let mut a1 = (*dct4x4.offset(2isize))[0usize] as ::core::ffi::c_int
            + (*dct4x4.offset(3isize))[0usize] as ::core::ffi::c_int;
        let mut a2 = (*dct4x4.offset(4isize))[0usize] as ::core::ffi::c_int
            + (*dct4x4.offset(5isize))[0usize] as ::core::ffi::c_int;
        let mut a3 = (*dct4x4.offset(6isize))[0usize] as ::core::ffi::c_int
            + (*dct4x4.offset(7isize))[0usize] as ::core::ffi::c_int;
        let mut a4 = (*dct4x4.offset(0isize))[0usize] as ::core::ffi::c_int
            - (*dct4x4.offset(1isize))[0usize] as ::core::ffi::c_int;
        let mut a5 = (*dct4x4.offset(2isize))[0usize] as ::core::ffi::c_int
            - (*dct4x4.offset(3isize))[0usize] as ::core::ffi::c_int;
        let mut a6 = (*dct4x4.offset(4isize))[0usize] as ::core::ffi::c_int
            - (*dct4x4.offset(5isize))[0usize] as ::core::ffi::c_int;
        let mut a7 = (*dct4x4.offset(6isize))[0usize] as ::core::ffi::c_int
            - (*dct4x4.offset(7isize))[0usize] as ::core::ffi::c_int;
        let mut b0 = a0 + a1;
        let mut b1 = a2 + a3;
        let mut b2 = a4 + a5;
        let mut b3 = a6 + a7;
        let mut b4 = a0 - a1;
        let mut b5 = a2 - a3;
        let mut b6 = a4 - a5;
        let mut b7 = a6 - a7;
        *dct.offset(0isize) = (b0 + b1) as crate::src::common::common::dctcoef;
        *dct.offset(1isize) = (b2 + b3) as crate::src::common::common::dctcoef;
        *dct.offset(2isize) = (b0 - b1) as crate::src::common::common::dctcoef;
        *dct.offset(3isize) = (b2 - b3) as crate::src::common::common::dctcoef;
        *dct.offset(4isize) = (b4 - b5) as crate::src::common::common::dctcoef;
        *dct.offset(5isize) = (b6 - b7) as crate::src::common::common::dctcoef;
        *dct.offset(6isize) = (b4 + b5) as crate::src::common::common::dctcoef;
        *dct.offset(7isize) = (b6 + b7) as crate::src::common::common::dctcoef;
        (*dct4x4.offset(0isize))[0usize] = 0i16;
        (*dct4x4.offset(1isize))[0usize] = 0i16;
        (*dct4x4.offset(2isize))[0usize] = 0i16;
        (*dct4x4.offset(3isize))[0usize] = 0i16;
        (*dct4x4.offset(4isize))[0usize] = 0i16;
        (*dct4x4.offset(5isize))[0usize] = 0i16;
        (*dct4x4.offset(6isize))[0usize] = 0i16;
        (*dct4x4.offset(7isize))[0usize] = 0i16;
    }
}
#[inline]
unsafe extern "C" fn pixel_sub_wxh(
    mut diff: *mut crate::src::common::common::dctcoef,
    mut i_size: ::core::ffi::c_int,
    mut pix1: *mut crate::src::common::common::pixel,
    mut i_pix1: ::core::ffi::c_int,
    mut pix2: *mut crate::src::common::common::pixel,
    mut i_pix2: ::core::ffi::c_int,
) {
    unsafe {
        let mut y = 0i32;
        while y < i_size {
            let mut x = 0i32;
            while x < i_size {
                *diff.offset((x + y * i_size) as isize) = (*pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
                x += 1;
            }
            pix1 = pix1.offset(i_pix1 as isize);
            pix2 = pix2.offset(i_pix2 as isize);
            y += 1;
        }
    }
}
unsafe extern "C" fn sub4x4_dct(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut d = [0; 16];
        let mut tmp = [0; 16];
        let mut i = 0i32;
        let mut i_0 = 0i32;
        pixel_sub_wxh(
            &raw mut d as *mut crate::src::common::common::dctcoef,
            4i32,
            pix1,
            crate::src::common::common::FENC_STRIDE,
            pix2,
            crate::src::common::common::FDEC_STRIDE,
        );
        while i < 4i32 {
            let mut s03 = d[(i * 4i32 + 0i32) as usize] + d[(i * 4i32 + 3i32) as usize];
            let mut s12 = d[(i * 4i32 + 1i32) as usize] + d[(i * 4i32 + 2i32) as usize];
            let mut d03 = d[(i * 4i32 + 0i32) as usize] - d[(i * 4i32 + 3i32) as usize];
            let mut d12 = d[(i * 4i32 + 1i32) as usize] - d[(i * 4i32 + 2i32) as usize];
            tmp[(0i32 * 4i32 + i) as usize] = (s03 + s12) as crate::src::common::common::dctcoef;
            tmp[(1i32 * 4i32 + i) as usize] = (2i32 * d03 + d12) as crate::src::common::common::dctcoef;
            tmp[(2i32 * 4i32 + i) as usize] = (s03 - s12) as crate::src::common::common::dctcoef;
            tmp[(3i32 * 4i32 + i) as usize] = (d03 - 2i32 * d12) as crate::src::common::common::dctcoef;
            i += 1;
        }
        while i_0 < 4i32 {
            let mut s03_0 = tmp[(i_0 * 4i32 + 0i32) as usize] as ::core::ffi::c_int
                + tmp[(i_0 * 4i32 + 3i32) as usize] as ::core::ffi::c_int;
            let mut s12_0 = tmp[(i_0 * 4i32 + 1i32) as usize] as ::core::ffi::c_int
                + tmp[(i_0 * 4i32 + 2i32) as usize] as ::core::ffi::c_int;
            let mut d03_0 = tmp[(i_0 * 4i32 + 0i32) as usize] as ::core::ffi::c_int
                - tmp[(i_0 * 4i32 + 3i32) as usize] as ::core::ffi::c_int;
            let mut d12_0 = tmp[(i_0 * 4i32 + 1i32) as usize] as ::core::ffi::c_int
                - tmp[(i_0 * 4i32 + 2i32) as usize] as ::core::ffi::c_int;
            *dct.offset((i_0 * 4i32 + 0i32) as isize) =
                (s03_0 + s12_0) as crate::src::common::common::dctcoef;
            *dct.offset((i_0 * 4i32 + 1i32) as isize) =
                (2i32 * d03_0 + d12_0) as crate::src::common::common::dctcoef;
            *dct.offset((i_0 * 4i32 + 2i32) as isize) =
                (s03_0 - s12_0) as crate::src::common::common::dctcoef;
            *dct.offset((i_0 * 4i32 + 3i32) as isize) =
                (d03_0 - 2i32 * d12_0) as crate::src::common::common::dctcoef;
            i_0 += 1;
        }
    }
}
unsafe extern "C" fn sub8x8_dct(
    mut dct: *mut [crate::src::common::common::dctcoef; 16],
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) {
    unsafe {
        sub4x4_dct(
            &raw mut *dct.offset(0isize) as *mut crate::src::common::common::dctcoef,
            pix1.offset(0isize),
            pix2.offset(0isize),
        );
        sub4x4_dct(
            &raw mut *dct.offset(1isize) as *mut crate::src::common::common::dctcoef,
            pix1.offset(4isize),
            pix2.offset(4isize),
        );
        sub4x4_dct(
            &raw mut *dct.offset(2isize) as *mut crate::src::common::common::dctcoef,
            pix1.offset((4i32 * crate::src::common::common::FENC_STRIDE + 0i32) as isize),
            pix2.offset((4i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
        );
        sub4x4_dct(
            &raw mut *dct.offset(3isize) as *mut crate::src::common::common::dctcoef,
            pix1.offset((4i32 * crate::src::common::common::FENC_STRIDE + 4i32) as isize),
            pix2.offset((4i32 * crate::src::common::common::FDEC_STRIDE + 4i32) as isize),
        );
    }
}
unsafe extern "C" fn sub16x16_dct(
    mut dct: *mut [crate::src::common::common::dctcoef; 16],
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) {
    unsafe {
        sub8x8_dct(dct.offset(0isize), pix1.offset(0isize), pix2.offset(0isize));
        sub8x8_dct(dct.offset(4isize), pix1.offset(8isize), pix2.offset(8isize));
        sub8x8_dct(
            dct.offset(8isize),
            pix1.offset((8i32 * crate::src::common::common::FENC_STRIDE + 0i32) as isize),
            pix2.offset((8i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
        );
        sub8x8_dct(
            dct.offset(12isize),
            pix1.offset((8i32 * crate::src::common::common::FENC_STRIDE + 8i32) as isize),
            pix2.offset((8i32 * crate::src::common::common::FDEC_STRIDE + 8i32) as isize),
        );
    }
}
unsafe extern "C" fn sub4x4_dct_dc(
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum = 0i32;
        let mut i = 0i32;
        while i < 4i32 {
            sum += *pix1.offset(0isize) as ::core::ffi::c_int
                + *pix1.offset(1isize) as ::core::ffi::c_int
                + *pix1.offset(2isize) as ::core::ffi::c_int
                + *pix1.offset(3isize) as ::core::ffi::c_int
                - *pix2.offset(0isize) as ::core::ffi::c_int
                - *pix2.offset(1isize) as ::core::ffi::c_int
                - *pix2.offset(2isize) as ::core::ffi::c_int
                - *pix2.offset(3isize) as ::core::ffi::c_int;
            i += 1;
            pix1 = pix1.offset(crate::src::common::common::FENC_STRIDE as isize);
            pix2 = pix2.offset(crate::src::common::common::FDEC_STRIDE as isize);
        }
        sum
    }
}
unsafe extern "C" fn sub8x8_dct_dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) {
    unsafe {
        *dct.offset(0isize) =
            sub4x4_dct_dc(pix1.offset(0isize), pix2.offset(0isize)) as crate::src::common::common::dctcoef;
        *dct.offset(1isize) =
            sub4x4_dct_dc(pix1.offset(4isize), pix2.offset(4isize)) as crate::src::common::common::dctcoef;
        *dct.offset(2isize) = sub4x4_dct_dc(
            pix1.offset((4i32 * crate::src::common::common::FENC_STRIDE + 0i32) as isize),
            pix2.offset((4i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
        ) as crate::src::common::common::dctcoef;
        *dct.offset(3isize) = sub4x4_dct_dc(
            pix1.offset((4i32 * crate::src::common::common::FENC_STRIDE + 4i32) as isize),
            pix2.offset((4i32 * crate::src::common::common::FDEC_STRIDE + 4i32) as isize),
        ) as crate::src::common::common::dctcoef;
        let mut d0 = *dct.offset(0isize) as ::core::ffi::c_int + *dct.offset(1isize) as ::core::ffi::c_int;
        let mut d1 = *dct.offset(2isize) as ::core::ffi::c_int + *dct.offset(3isize) as ::core::ffi::c_int;
        let mut d2 = *dct.offset(0isize) as ::core::ffi::c_int - *dct.offset(1isize) as ::core::ffi::c_int;
        let mut d3 = *dct.offset(2isize) as ::core::ffi::c_int - *dct.offset(3isize) as ::core::ffi::c_int;
        *dct.offset(0isize) = (d0 + d1) as crate::src::common::common::dctcoef;
        *dct.offset(1isize) = (d0 - d1) as crate::src::common::common::dctcoef;
        *dct.offset(2isize) = (d2 + d3) as crate::src::common::common::dctcoef;
        *dct.offset(3isize) = (d2 - d3) as crate::src::common::common::dctcoef;
    }
}
unsafe extern "C" fn sub8x16_dct_dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut a0 = sub4x4_dct_dc(
            pix1.offset((0i32 * crate::src::common::common::FENC_STRIDE + 0i32) as isize),
            pix2.offset((0i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
        );
        let mut a1 = sub4x4_dct_dc(
            pix1.offset((0i32 * crate::src::common::common::FENC_STRIDE + 4i32) as isize),
            pix2.offset((0i32 * crate::src::common::common::FDEC_STRIDE + 4i32) as isize),
        );
        let mut a2 = sub4x4_dct_dc(
            pix1.offset((4i32 * crate::src::common::common::FENC_STRIDE + 0i32) as isize),
            pix2.offset((4i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
        );
        let mut a3 = sub4x4_dct_dc(
            pix1.offset((4i32 * crate::src::common::common::FENC_STRIDE + 4i32) as isize),
            pix2.offset((4i32 * crate::src::common::common::FDEC_STRIDE + 4i32) as isize),
        );
        let mut a4 = sub4x4_dct_dc(
            pix1.offset((8i32 * crate::src::common::common::FENC_STRIDE + 0i32) as isize),
            pix2.offset((8i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
        );
        let mut a5 = sub4x4_dct_dc(
            pix1.offset((8i32 * crate::src::common::common::FENC_STRIDE + 4i32) as isize),
            pix2.offset((8i32 * crate::src::common::common::FDEC_STRIDE + 4i32) as isize),
        );
        let mut a6 = sub4x4_dct_dc(
            pix1.offset((12i32 * crate::src::common::common::FENC_STRIDE + 0i32) as isize),
            pix2.offset((12i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
        );
        let mut a7 = sub4x4_dct_dc(
            pix1.offset((12i32 * crate::src::common::common::FENC_STRIDE + 4i32) as isize),
            pix2.offset((12i32 * crate::src::common::common::FDEC_STRIDE + 4i32) as isize),
        );
        let mut b0 = a0 + a1;
        let mut b1 = a2 + a3;
        let mut b2 = a4 + a5;
        let mut b3 = a6 + a7;
        let mut b4 = a0 - a1;
        let mut b5 = a2 - a3;
        let mut b6 = a4 - a5;
        let mut b7 = a6 - a7;
        a0 = b0 + b1;
        a1 = b2 + b3;
        a2 = b4 + b5;
        a3 = b6 + b7;
        a4 = b0 - b1;
        a5 = b2 - b3;
        a6 = b4 - b5;
        a7 = b6 - b7;
        *dct.offset(0isize) = (a0 + a1) as crate::src::common::common::dctcoef;
        *dct.offset(1isize) = (a2 + a3) as crate::src::common::common::dctcoef;
        *dct.offset(2isize) = (a0 - a1) as crate::src::common::common::dctcoef;
        *dct.offset(3isize) = (a2 - a3) as crate::src::common::common::dctcoef;
        *dct.offset(4isize) = (a4 - a5) as crate::src::common::common::dctcoef;
        *dct.offset(5isize) = (a6 - a7) as crate::src::common::common::dctcoef;
        *dct.offset(6isize) = (a4 + a5) as crate::src::common::common::dctcoef;
        *dct.offset(7isize) = (a6 + a7) as crate::src::common::common::dctcoef;
    }
}
unsafe extern "C" fn add4x4_idct(
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        let mut d = [0; 16];
        let mut tmp = [0; 16];
        let mut i = 0i32;
        let mut i_0 = 0i32;
        let mut y = 0i32;
        while i < 4i32 {
            let mut s02 = *dct.offset((0i32 * 4i32 + i) as isize) as ::core::ffi::c_int
                + *dct.offset((2i32 * 4i32 + i) as isize) as ::core::ffi::c_int;
            let mut d02 = *dct.offset((0i32 * 4i32 + i) as isize) as ::core::ffi::c_int
                - *dct.offset((2i32 * 4i32 + i) as isize) as ::core::ffi::c_int;
            let mut s13 = *dct.offset((1i32 * 4i32 + i) as isize) as ::core::ffi::c_int
                + (*dct.offset((3i32 * 4i32 + i) as isize) as ::core::ffi::c_int >> 1i32);
            let mut d13 = (*dct.offset((1i32 * 4i32 + i) as isize) as ::core::ffi::c_int >> 1i32)
                - *dct.offset((3i32 * 4i32 + i) as isize) as ::core::ffi::c_int;
            tmp[(i * 4i32 + 0i32) as usize] = (s02 + s13) as crate::src::common::common::dctcoef;
            tmp[(i * 4i32 + 1i32) as usize] = (d02 + d13) as crate::src::common::common::dctcoef;
            tmp[(i * 4i32 + 2i32) as usize] = (d02 - d13) as crate::src::common::common::dctcoef;
            tmp[(i * 4i32 + 3i32) as usize] = (s02 - s13) as crate::src::common::common::dctcoef;
            i += 1;
        }
        while i_0 < 4i32 {
            let mut s02_0 = tmp[(0i32 * 4i32 + i_0) as usize] as ::core::ffi::c_int
                + tmp[(2i32 * 4i32 + i_0) as usize] as ::core::ffi::c_int;
            let mut d02_0 = tmp[(0i32 * 4i32 + i_0) as usize] as ::core::ffi::c_int
                - tmp[(2i32 * 4i32 + i_0) as usize] as ::core::ffi::c_int;
            let mut s13_0 = tmp[(1i32 * 4i32 + i_0) as usize] as ::core::ffi::c_int
                + (tmp[(3i32 * 4i32 + i_0) as usize] as ::core::ffi::c_int >> 1i32);
            let mut d13_0 = (tmp[(1i32 * 4i32 + i_0) as usize] as ::core::ffi::c_int >> 1i32)
                - tmp[(3i32 * 4i32 + i_0) as usize] as ::core::ffi::c_int;
            d[(0i32 * 4i32 + i_0) as usize] =
                ((s02_0 + s13_0 + 32i32) >> 6i32) as crate::src::common::common::dctcoef;
            d[(1i32 * 4i32 + i_0) as usize] =
                ((d02_0 + d13_0 + 32i32) >> 6i32) as crate::src::common::common::dctcoef;
            d[(2i32 * 4i32 + i_0) as usize] =
                ((d02_0 - d13_0 + 32i32) >> 6i32) as crate::src::common::common::dctcoef;
            d[(3i32 * 4i32 + i_0) as usize] =
                ((s02_0 - s13_0 + 32i32) >> 6i32) as crate::src::common::common::dctcoef;
            i_0 += 1;
        }
        while y < 4i32 {
            let mut x = 0i32;
            while x < 4i32 {
                *p_dst.offset(x as isize) = x264_clip_pixel(
                    *p_dst.offset(x as isize) as ::core::ffi::c_int
                        + d[(y * 4i32 + x) as usize] as ::core::ffi::c_int,
                );
                x += 1;
            }
            p_dst = p_dst.offset(crate::src::common::common::FDEC_STRIDE as isize);
            y += 1;
        }
    }
}
unsafe extern "C" fn add8x8_idct(
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dct: *mut [crate::src::common::common::dctcoef; 16],
) {
    unsafe {
        add4x4_idct(
            p_dst.offset(0isize),
            &raw mut *dct.offset(0isize) as *mut crate::src::common::common::dctcoef,
        );
        add4x4_idct(
            p_dst.offset(4isize),
            &raw mut *dct.offset(1isize) as *mut crate::src::common::common::dctcoef,
        );
        add4x4_idct(
            p_dst.offset((4i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
            &raw mut *dct.offset(2isize) as *mut crate::src::common::common::dctcoef,
        );
        add4x4_idct(
            p_dst.offset((4i32 * crate::src::common::common::FDEC_STRIDE + 4i32) as isize),
            &raw mut *dct.offset(3isize) as *mut crate::src::common::common::dctcoef,
        );
    }
}
unsafe extern "C" fn add16x16_idct(
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dct: *mut [crate::src::common::common::dctcoef; 16],
) {
    unsafe {
        add8x8_idct(p_dst.offset(0isize), dct.offset(0isize));
        add8x8_idct(p_dst.offset(8isize), dct.offset(4isize));
        add8x8_idct(
            p_dst.offset((8i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
            dct.offset(8isize),
        );
        add8x8_idct(
            p_dst.offset((8i32 * crate::src::common::common::FDEC_STRIDE + 8i32) as isize),
            dct.offset(12isize),
        );
    }
}
unsafe extern "C" fn sub8x8_dct8(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut tmp = [0; 64];
        let mut i = 0i32;
        let mut i_0 = 0i32;
        pixel_sub_wxh(
            &raw mut tmp as *mut crate::src::common::common::dctcoef,
            8i32,
            pix1,
            crate::src::common::common::FENC_STRIDE,
            pix2,
            crate::src::common::common::FDEC_STRIDE,
        );
        while i < 8i32 {
            let mut s07 = tmp[(0i32 * 8i32 + i) as usize] as ::core::ffi::c_int
                + tmp[(7i32 * 8i32 + i) as usize] as ::core::ffi::c_int;
            let mut s16 = tmp[(1i32 * 8i32 + i) as usize] as ::core::ffi::c_int
                + tmp[(6i32 * 8i32 + i) as usize] as ::core::ffi::c_int;
            let mut s25 = tmp[(2i32 * 8i32 + i) as usize] as ::core::ffi::c_int
                + tmp[(5i32 * 8i32 + i) as usize] as ::core::ffi::c_int;
            let mut s34 = tmp[(3i32 * 8i32 + i) as usize] as ::core::ffi::c_int
                + tmp[(4i32 * 8i32 + i) as usize] as ::core::ffi::c_int;
            let mut a0 = s07 + s34;
            let mut a1 = s16 + s25;
            let mut a2 = s07 - s34;
            let mut a3 = s16 - s25;
            let mut d07 = tmp[(0i32 * 8i32 + i) as usize] as ::core::ffi::c_int
                - tmp[(7i32 * 8i32 + i) as usize] as ::core::ffi::c_int;
            let mut d16 = tmp[(1i32 * 8i32 + i) as usize] as ::core::ffi::c_int
                - tmp[(6i32 * 8i32 + i) as usize] as ::core::ffi::c_int;
            let mut d25 = tmp[(2i32 * 8i32 + i) as usize] as ::core::ffi::c_int
                - tmp[(5i32 * 8i32 + i) as usize] as ::core::ffi::c_int;
            let mut d34 = tmp[(3i32 * 8i32 + i) as usize] as ::core::ffi::c_int
                - tmp[(4i32 * 8i32 + i) as usize] as ::core::ffi::c_int;
            let mut a4 = d16 + d25 + (d07 + (d07 >> 1i32));
            let mut a5 = d07 - d34 - (d25 + (d25 >> 1i32));
            let mut a6 = d07 + d34 - (d16 + (d16 >> 1i32));
            let mut a7 = d16 - d25 + (d34 + (d34 >> 1i32));
            tmp[(0i32 * 8i32 + i) as usize] = (a0 + a1) as crate::src::common::common::dctcoef;
            tmp[(1i32 * 8i32 + i) as usize] = (a4 + (a7 >> 2i32)) as crate::src::common::common::dctcoef;
            tmp[(2i32 * 8i32 + i) as usize] = (a2 + (a3 >> 1i32)) as crate::src::common::common::dctcoef;
            tmp[(3i32 * 8i32 + i) as usize] = (a5 + (a6 >> 2i32)) as crate::src::common::common::dctcoef;
            tmp[(4i32 * 8i32 + i) as usize] = (a0 - a1) as crate::src::common::common::dctcoef;
            tmp[(5i32 * 8i32 + i) as usize] = (a6 - (a5 >> 2i32)) as crate::src::common::common::dctcoef;
            tmp[(6i32 * 8i32 + i) as usize] = ((a2 >> 1i32) - a3) as crate::src::common::common::dctcoef;
            tmp[(7i32 * 8i32 + i) as usize] = ((a4 >> 2i32) - a7) as crate::src::common::common::dctcoef;
            i += 1;
        }
        while i_0 < 8i32 {
            let mut s07_0 = tmp[(i_0 * 8i32 + 0i32) as usize] as ::core::ffi::c_int
                + tmp[(i_0 * 8i32 + 7i32) as usize] as ::core::ffi::c_int;
            let mut s16_0 = tmp[(i_0 * 8i32 + 1i32) as usize] as ::core::ffi::c_int
                + tmp[(i_0 * 8i32 + 6i32) as usize] as ::core::ffi::c_int;
            let mut s25_0 = tmp[(i_0 * 8i32 + 2i32) as usize] as ::core::ffi::c_int
                + tmp[(i_0 * 8i32 + 5i32) as usize] as ::core::ffi::c_int;
            let mut s34_0 = tmp[(i_0 * 8i32 + 3i32) as usize] as ::core::ffi::c_int
                + tmp[(i_0 * 8i32 + 4i32) as usize] as ::core::ffi::c_int;
            let mut a0_0 = s07_0 + s34_0;
            let mut a1_0 = s16_0 + s25_0;
            let mut a2_0 = s07_0 - s34_0;
            let mut a3_0 = s16_0 - s25_0;
            let mut d07_0 = tmp[(i_0 * 8i32 + 0i32) as usize] as ::core::ffi::c_int
                - tmp[(i_0 * 8i32 + 7i32) as usize] as ::core::ffi::c_int;
            let mut d16_0 = tmp[(i_0 * 8i32 + 1i32) as usize] as ::core::ffi::c_int
                - tmp[(i_0 * 8i32 + 6i32) as usize] as ::core::ffi::c_int;
            let mut d25_0 = tmp[(i_0 * 8i32 + 2i32) as usize] as ::core::ffi::c_int
                - tmp[(i_0 * 8i32 + 5i32) as usize] as ::core::ffi::c_int;
            let mut d34_0 = tmp[(i_0 * 8i32 + 3i32) as usize] as ::core::ffi::c_int
                - tmp[(i_0 * 8i32 + 4i32) as usize] as ::core::ffi::c_int;
            let mut a4_0 = d16_0 + d25_0 + (d07_0 + (d07_0 >> 1i32));
            let mut a5_0 = d07_0 - d34_0 - (d25_0 + (d25_0 >> 1i32));
            let mut a6_0 = d07_0 + d34_0 - (d16_0 + (d16_0 >> 1i32));
            let mut a7_0 = d16_0 - d25_0 + (d34_0 + (d34_0 >> 1i32));
            *dct.offset((0i32 * 8i32 + i_0) as isize) = (a0_0 + a1_0) as crate::src::common::common::dctcoef;
            *dct.offset((1i32 * 8i32 + i_0) as isize) =
                (a4_0 + (a7_0 >> 2i32)) as crate::src::common::common::dctcoef;
            *dct.offset((2i32 * 8i32 + i_0) as isize) =
                (a2_0 + (a3_0 >> 1i32)) as crate::src::common::common::dctcoef;
            *dct.offset((3i32 * 8i32 + i_0) as isize) =
                (a5_0 + (a6_0 >> 2i32)) as crate::src::common::common::dctcoef;
            *dct.offset((4i32 * 8i32 + i_0) as isize) = (a0_0 - a1_0) as crate::src::common::common::dctcoef;
            *dct.offset((5i32 * 8i32 + i_0) as isize) =
                (a6_0 - (a5_0 >> 2i32)) as crate::src::common::common::dctcoef;
            *dct.offset((6i32 * 8i32 + i_0) as isize) =
                ((a2_0 >> 1i32) - a3_0) as crate::src::common::common::dctcoef;
            *dct.offset((7i32 * 8i32 + i_0) as isize) =
                ((a4_0 >> 2i32) - a7_0) as crate::src::common::common::dctcoef;
            i_0 += 1;
        }
    }
}
unsafe extern "C" fn sub16x16_dct8(
    mut dct: *mut [crate::src::common::common::dctcoef; 64],
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) {
    unsafe {
        sub8x8_dct8(
            &raw mut *dct.offset(0isize) as *mut crate::src::common::common::dctcoef,
            pix1.offset(0isize),
            pix2.offset(0isize),
        );
        sub8x8_dct8(
            &raw mut *dct.offset(1isize) as *mut crate::src::common::common::dctcoef,
            pix1.offset(8isize),
            pix2.offset(8isize),
        );
        sub8x8_dct8(
            &raw mut *dct.offset(2isize) as *mut crate::src::common::common::dctcoef,
            pix1.offset((8i32 * crate::src::common::common::FENC_STRIDE + 0i32) as isize),
            pix2.offset((8i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
        );
        sub8x8_dct8(
            &raw mut *dct.offset(3isize) as *mut crate::src::common::common::dctcoef,
            pix1.offset((8i32 * crate::src::common::common::FENC_STRIDE + 8i32) as isize),
            pix2.offset((8i32 * crate::src::common::common::FDEC_STRIDE + 8i32) as isize),
        );
    }
}
unsafe extern "C" fn add8x8_idct8(
    mut dst: *mut crate::src::common::common::pixel,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        let mut i = 0i32;
        let mut i_0 = 0i32;
        let ref mut c2rust_fresh0 = *dct.offset(0isize);
        *c2rust_fresh0 =
            (*c2rust_fresh0 as ::core::ffi::c_int + 32i32) as crate::src::common::common::dctcoef;
        while i < 8i32 {
            let mut a0 = *dct.offset((0i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                + *dct.offset((4i32 * 8i32 + i) as isize) as ::core::ffi::c_int;
            let mut a2 = *dct.offset((0i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                - *dct.offset((4i32 * 8i32 + i) as isize) as ::core::ffi::c_int;
            let mut a4 = (*dct.offset((2i32 * 8i32 + i) as isize) as ::core::ffi::c_int >> 1i32)
                - *dct.offset((6i32 * 8i32 + i) as isize) as ::core::ffi::c_int;
            let mut a6 = (*dct.offset((6i32 * 8i32 + i) as isize) as ::core::ffi::c_int >> 1i32)
                + *dct.offset((2i32 * 8i32 + i) as isize) as ::core::ffi::c_int;
            let mut b0 = a0 + a6;
            let mut b2 = a2 + a4;
            let mut b4 = a2 - a4;
            let mut b6 = a0 - a6;
            let mut a1 = -(*dct.offset((3i32 * 8i32 + i) as isize) as ::core::ffi::c_int)
                + *dct.offset((5i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                - *dct.offset((7i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                - (*dct.offset((7i32 * 8i32 + i) as isize) as ::core::ffi::c_int >> 1i32);
            let mut a3 = *dct.offset((1i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                + *dct.offset((7i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                - *dct.offset((3i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                - (*dct.offset((3i32 * 8i32 + i) as isize) as ::core::ffi::c_int >> 1i32);
            let mut a5 = -(*dct.offset((1i32 * 8i32 + i) as isize) as ::core::ffi::c_int)
                + *dct.offset((7i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                + *dct.offset((5i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                + (*dct.offset((5i32 * 8i32 + i) as isize) as ::core::ffi::c_int >> 1i32);
            let mut a7 = *dct.offset((3i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                + *dct.offset((5i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                + *dct.offset((1i32 * 8i32 + i) as isize) as ::core::ffi::c_int
                + (*dct.offset((1i32 * 8i32 + i) as isize) as ::core::ffi::c_int >> 1i32);
            let mut b1 = (a7 >> 2i32) + a1;
            let mut b3 = a3 + (a5 >> 2i32);
            let mut b5 = (a3 >> 2i32) - a5;
            let mut b7 = a7 - (a1 >> 2i32);
            *dct.offset((0i32 * 8i32 + i) as isize) = (b0 + b7) as crate::src::common::common::dctcoef;
            *dct.offset((1i32 * 8i32 + i) as isize) = (b2 + b5) as crate::src::common::common::dctcoef;
            *dct.offset((2i32 * 8i32 + i) as isize) = (b4 + b3) as crate::src::common::common::dctcoef;
            *dct.offset((3i32 * 8i32 + i) as isize) = (b6 + b1) as crate::src::common::common::dctcoef;
            *dct.offset((4i32 * 8i32 + i) as isize) = (b6 - b1) as crate::src::common::common::dctcoef;
            *dct.offset((5i32 * 8i32 + i) as isize) = (b4 - b3) as crate::src::common::common::dctcoef;
            *dct.offset((6i32 * 8i32 + i) as isize) = (b2 - b5) as crate::src::common::common::dctcoef;
            *dct.offset((7i32 * 8i32 + i) as isize) = (b0 - b7) as crate::src::common::common::dctcoef;
            i += 1;
        }
        while i_0 < 8i32 {
            let mut a0_0 = *dct.offset((i_0 * 8i32 + 0i32) as isize) as ::core::ffi::c_int
                + *dct.offset((i_0 * 8i32 + 4i32) as isize) as ::core::ffi::c_int;
            let mut a2_0 = *dct.offset((i_0 * 8i32 + 0i32) as isize) as ::core::ffi::c_int
                - *dct.offset((i_0 * 8i32 + 4i32) as isize) as ::core::ffi::c_int;
            let mut a4_0 = (*dct.offset((i_0 * 8i32 + 2i32) as isize) as ::core::ffi::c_int >> 1i32)
                - *dct.offset((i_0 * 8i32 + 6i32) as isize) as ::core::ffi::c_int;
            let mut a6_0 = (*dct.offset((i_0 * 8i32 + 6i32) as isize) as ::core::ffi::c_int >> 1i32)
                + *dct.offset((i_0 * 8i32 + 2i32) as isize) as ::core::ffi::c_int;
            let mut b0_0 = a0_0 + a6_0;
            let mut b2_0 = a2_0 + a4_0;
            let mut b4_0 = a2_0 - a4_0;
            let mut b6_0 = a0_0 - a6_0;
            let mut a1_0 = -(*dct.offset((i_0 * 8i32 + 3i32) as isize) as ::core::ffi::c_int)
                + *dct.offset((i_0 * 8i32 + 5i32) as isize) as ::core::ffi::c_int
                - *dct.offset((i_0 * 8i32 + 7i32) as isize) as ::core::ffi::c_int
                - (*dct.offset((i_0 * 8i32 + 7i32) as isize) as ::core::ffi::c_int >> 1i32);
            let mut a3_0 = *dct.offset((i_0 * 8i32 + 1i32) as isize) as ::core::ffi::c_int
                + *dct.offset((i_0 * 8i32 + 7i32) as isize) as ::core::ffi::c_int
                - *dct.offset((i_0 * 8i32 + 3i32) as isize) as ::core::ffi::c_int
                - (*dct.offset((i_0 * 8i32 + 3i32) as isize) as ::core::ffi::c_int >> 1i32);
            let mut a5_0 = -(*dct.offset((i_0 * 8i32 + 1i32) as isize) as ::core::ffi::c_int)
                + *dct.offset((i_0 * 8i32 + 7i32) as isize) as ::core::ffi::c_int
                + *dct.offset((i_0 * 8i32 + 5i32) as isize) as ::core::ffi::c_int
                + (*dct.offset((i_0 * 8i32 + 5i32) as isize) as ::core::ffi::c_int >> 1i32);
            let mut a7_0 = *dct.offset((i_0 * 8i32 + 3i32) as isize) as ::core::ffi::c_int
                + *dct.offset((i_0 * 8i32 + 5i32) as isize) as ::core::ffi::c_int
                + *dct.offset((i_0 * 8i32 + 1i32) as isize) as ::core::ffi::c_int
                + (*dct.offset((i_0 * 8i32 + 1i32) as isize) as ::core::ffi::c_int >> 1i32);
            let mut b1_0 = (a7_0 >> 2i32) + a1_0;
            let mut b3_0 = a3_0 + (a5_0 >> 2i32);
            let mut b5_0 = (a3_0 >> 2i32) - a5_0;
            let mut b7_0 = a7_0 - (a1_0 >> 2i32);
            *dst.offset((i_0 + 0i32 * crate::src::common::common::FDEC_STRIDE) as isize) = x264_clip_pixel(
                *dst.offset((i_0 + 0i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                    as ::core::ffi::c_int
                    + ((b0_0 + b7_0) >> 6i32),
            );
            *dst.offset((i_0 + 1i32 * crate::src::common::common::FDEC_STRIDE) as isize) = x264_clip_pixel(
                *dst.offset((i_0 + 1i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                    as ::core::ffi::c_int
                    + ((b2_0 + b5_0) >> 6i32),
            );
            *dst.offset((i_0 + 2i32 * crate::src::common::common::FDEC_STRIDE) as isize) = x264_clip_pixel(
                *dst.offset((i_0 + 2i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                    as ::core::ffi::c_int
                    + ((b4_0 + b3_0) >> 6i32),
            );
            *dst.offset((i_0 + 3i32 * crate::src::common::common::FDEC_STRIDE) as isize) = x264_clip_pixel(
                *dst.offset((i_0 + 3i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                    as ::core::ffi::c_int
                    + ((b6_0 + b1_0) >> 6i32),
            );
            *dst.offset((i_0 + 4i32 * crate::src::common::common::FDEC_STRIDE) as isize) = x264_clip_pixel(
                *dst.offset((i_0 + 4i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                    as ::core::ffi::c_int
                    + ((b6_0 - b1_0) >> 6i32),
            );
            *dst.offset((i_0 + 5i32 * crate::src::common::common::FDEC_STRIDE) as isize) = x264_clip_pixel(
                *dst.offset((i_0 + 5i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                    as ::core::ffi::c_int
                    + ((b4_0 - b3_0) >> 6i32),
            );
            *dst.offset((i_0 + 6i32 * crate::src::common::common::FDEC_STRIDE) as isize) = x264_clip_pixel(
                *dst.offset((i_0 + 6i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                    as ::core::ffi::c_int
                    + ((b2_0 - b5_0) >> 6i32),
            );
            *dst.offset((i_0 + 7i32 * crate::src::common::common::FDEC_STRIDE) as isize) = x264_clip_pixel(
                *dst.offset((i_0 + 7i32 * crate::src::common::common::FDEC_STRIDE) as isize)
                    as ::core::ffi::c_int
                    + ((b0_0 - b7_0) >> 6i32),
            );
            i_0 += 1;
        }
    }
}
unsafe extern "C" fn add16x16_idct8(
    mut dst: *mut crate::src::common::common::pixel,
    mut dct: *mut [crate::src::common::common::dctcoef; 64],
) {
    unsafe {
        add8x8_idct8(
            dst.offset(0isize),
            &raw mut *dct.offset(0isize) as *mut crate::src::common::common::dctcoef,
        );
        add8x8_idct8(
            dst.offset(8isize),
            &raw mut *dct.offset(1isize) as *mut crate::src::common::common::dctcoef,
        );
        add8x8_idct8(
            dst.offset((8i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
            &raw mut *dct.offset(2isize) as *mut crate::src::common::common::dctcoef,
        );
        add8x8_idct8(
            dst.offset((8i32 * crate::src::common::common::FDEC_STRIDE + 8i32) as isize),
            &raw mut *dct.offset(3isize) as *mut crate::src::common::common::dctcoef,
        );
    }
}
#[inline]
unsafe extern "C" fn add4x4_idct_dc(
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dc: crate::src::common::common::dctcoef,
) {
    unsafe {
        let mut i = 0i32;
        dc = ((dc as ::core::ffi::c_int + 32i32) >> 6i32) as crate::src::common::common::dctcoef;
        while i < 4i32 {
            *p_dst.offset(0isize) =
                x264_clip_pixel(*p_dst.offset(0isize) as ::core::ffi::c_int + dc as ::core::ffi::c_int);
            *p_dst.offset(1isize) =
                x264_clip_pixel(*p_dst.offset(1isize) as ::core::ffi::c_int + dc as ::core::ffi::c_int);
            *p_dst.offset(2isize) =
                x264_clip_pixel(*p_dst.offset(2isize) as ::core::ffi::c_int + dc as ::core::ffi::c_int);
            *p_dst.offset(3isize) =
                x264_clip_pixel(*p_dst.offset(3isize) as ::core::ffi::c_int + dc as ::core::ffi::c_int);
            i += 1;
            p_dst = p_dst.offset(crate::src::common::common::FDEC_STRIDE as isize);
        }
    }
}
unsafe extern "C" fn add8x8_idct_dc(
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        add4x4_idct_dc(p_dst.offset(0isize), *dct.offset(0isize));
        add4x4_idct_dc(p_dst.offset(4isize), *dct.offset(1isize));
        add4x4_idct_dc(
            p_dst.offset((4i32 * crate::src::common::common::FDEC_STRIDE + 0i32) as isize),
            *dct.offset(2isize),
        );
        add4x4_idct_dc(
            p_dst.offset((4i32 * crate::src::common::common::FDEC_STRIDE + 4i32) as isize),
            *dct.offset(3isize),
        );
    }
}
unsafe extern "C" fn add16x16_idct_dc(
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        let mut i = 0i32;
        while i < 4i32 {
            add4x4_idct_dc(p_dst.offset(0isize), *dct.offset(0isize));
            add4x4_idct_dc(p_dst.offset(4isize), *dct.offset(1isize));
            add4x4_idct_dc(p_dst.offset(8isize), *dct.offset(2isize));
            add4x4_idct_dc(p_dst.offset(12isize), *dct.offset(3isize));
            i += 1;
            dct = dct.offset(4isize);
            p_dst = p_dst.offset((4i32 * crate::src::common::common::FDEC_STRIDE) as isize);
        }
    }
}
pub unsafe extern "C" fn x264_8_dct_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut dctf: *mut crate::src::common::dct::x264_dct_function_t,
) {
    unsafe {
        (*dctf).sub4x4_dct = Some(
            sub4x4_dct
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        );
        (*dctf).add4x4_idct = Some(
            add4x4_idct
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        );
        (*dctf).sub8x8_dct = Some(
            sub8x8_dct
                as unsafe extern "C" fn(
                    *mut [crate::src::common::common::dctcoef; 16],
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        );
        (*dctf).sub8x8_dct_dc = Some(
            sub8x8_dct_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        );
        (*dctf).add8x8_idct = Some(
            add8x8_idct
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut [crate::src::common::common::dctcoef; 16],
                ) -> (),
        );
        (*dctf).add8x8_idct_dc = Some(
            add8x8_idct_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        );
        (*dctf).sub8x16_dct_dc = Some(
            sub8x16_dct_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        );
        (*dctf).sub16x16_dct = Some(
            sub16x16_dct
                as unsafe extern "C" fn(
                    *mut [crate::src::common::common::dctcoef; 16],
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        );
        (*dctf).add16x16_idct = Some(
            add16x16_idct
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut [crate::src::common::common::dctcoef; 16],
                ) -> (),
        );
        (*dctf).add16x16_idct_dc = Some(
            add16x16_idct_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        );
        (*dctf).sub8x8_dct8 = Some(
            sub8x8_dct8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        );
        (*dctf).add8x8_idct8 = Some(
            add8x8_idct8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        );
        (*dctf).sub16x16_dct8 = Some(
            sub16x16_dct8
                as unsafe extern "C" fn(
                    *mut [crate::src::common::common::dctcoef; 64],
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        );
        (*dctf).add16x16_idct8 = Some(
            add16x16_idct8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut [crate::src::common::common::dctcoef; 64],
                ) -> (),
        );
        (*dctf).dct4x4dc =
            Some(dct4x4dc as unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ());
        (*dctf).idct4x4dc =
            Some(idct4x4dc as unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ());
        (*dctf).dct2x4dc = Some(
            dct2x4dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [crate::src::common::common::dctcoef; 16],
                ) -> (),
        );
    }
}
unsafe extern "C" fn zigzag_scan_8x8_frame(
    mut level: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        *level.offset(0isize) = *dct.offset((0i32 * 8i32 + 0i32) as isize);
        *level.offset(1isize) = *dct.offset((1i32 * 8i32 + 0i32) as isize);
        *level.offset(2isize) = *dct.offset((0i32 * 8i32 + 1i32) as isize);
        *level.offset(3isize) = *dct.offset((0i32 * 8i32 + 2i32) as isize);
        *level.offset(4isize) = *dct.offset((1i32 * 8i32 + 1i32) as isize);
        *level.offset(5isize) = *dct.offset((2i32 * 8i32 + 0i32) as isize);
        *level.offset(6isize) = *dct.offset((3i32 * 8i32 + 0i32) as isize);
        *level.offset(7isize) = *dct.offset((2i32 * 8i32 + 1i32) as isize);
        *level.offset(8isize) = *dct.offset((1i32 * 8i32 + 2i32) as isize);
        *level.offset(9isize) = *dct.offset((0i32 * 8i32 + 3i32) as isize);
        *level.offset(10isize) = *dct.offset((0i32 * 8i32 + 4i32) as isize);
        *level.offset(11isize) = *dct.offset((1i32 * 8i32 + 3i32) as isize);
        *level.offset(12isize) = *dct.offset((2i32 * 8i32 + 2i32) as isize);
        *level.offset(13isize) = *dct.offset((3i32 * 8i32 + 1i32) as isize);
        *level.offset(14isize) = *dct.offset((4i32 * 8i32 + 0i32) as isize);
        *level.offset(15isize) = *dct.offset((5i32 * 8i32 + 0i32) as isize);
        *level.offset(16isize) = *dct.offset((4i32 * 8i32 + 1i32) as isize);
        *level.offset(17isize) = *dct.offset((3i32 * 8i32 + 2i32) as isize);
        *level.offset(18isize) = *dct.offset((2i32 * 8i32 + 3i32) as isize);
        *level.offset(19isize) = *dct.offset((1i32 * 8i32 + 4i32) as isize);
        *level.offset(20isize) = *dct.offset((0i32 * 8i32 + 5i32) as isize);
        *level.offset(21isize) = *dct.offset((0i32 * 8i32 + 6i32) as isize);
        *level.offset(22isize) = *dct.offset((1i32 * 8i32 + 5i32) as isize);
        *level.offset(23isize) = *dct.offset((2i32 * 8i32 + 4i32) as isize);
        *level.offset(24isize) = *dct.offset((3i32 * 8i32 + 3i32) as isize);
        *level.offset(25isize) = *dct.offset((4i32 * 8i32 + 2i32) as isize);
        *level.offset(26isize) = *dct.offset((5i32 * 8i32 + 1i32) as isize);
        *level.offset(27isize) = *dct.offset((6i32 * 8i32 + 0i32) as isize);
        *level.offset(28isize) = *dct.offset((7i32 * 8i32 + 0i32) as isize);
        *level.offset(29isize) = *dct.offset((6i32 * 8i32 + 1i32) as isize);
        *level.offset(30isize) = *dct.offset((5i32 * 8i32 + 2i32) as isize);
        *level.offset(31isize) = *dct.offset((4i32 * 8i32 + 3i32) as isize);
        *level.offset(32isize) = *dct.offset((3i32 * 8i32 + 4i32) as isize);
        *level.offset(33isize) = *dct.offset((2i32 * 8i32 + 5i32) as isize);
        *level.offset(34isize) = *dct.offset((1i32 * 8i32 + 6i32) as isize);
        *level.offset(35isize) = *dct.offset((0i32 * 8i32 + 7i32) as isize);
        *level.offset(36isize) = *dct.offset((1i32 * 8i32 + 7i32) as isize);
        *level.offset(37isize) = *dct.offset((2i32 * 8i32 + 6i32) as isize);
        *level.offset(38isize) = *dct.offset((3i32 * 8i32 + 5i32) as isize);
        *level.offset(39isize) = *dct.offset((4i32 * 8i32 + 4i32) as isize);
        *level.offset(40isize) = *dct.offset((5i32 * 8i32 + 3i32) as isize);
        *level.offset(41isize) = *dct.offset((6i32 * 8i32 + 2i32) as isize);
        *level.offset(42isize) = *dct.offset((7i32 * 8i32 + 1i32) as isize);
        *level.offset(43isize) = *dct.offset((7i32 * 8i32 + 2i32) as isize);
        *level.offset(44isize) = *dct.offset((6i32 * 8i32 + 3i32) as isize);
        *level.offset(45isize) = *dct.offset((5i32 * 8i32 + 4i32) as isize);
        *level.offset(46isize) = *dct.offset((4i32 * 8i32 + 5i32) as isize);
        *level.offset(47isize) = *dct.offset((3i32 * 8i32 + 6i32) as isize);
        *level.offset(48isize) = *dct.offset((2i32 * 8i32 + 7i32) as isize);
        *level.offset(49isize) = *dct.offset((3i32 * 8i32 + 7i32) as isize);
        *level.offset(50isize) = *dct.offset((4i32 * 8i32 + 6i32) as isize);
        *level.offset(51isize) = *dct.offset((5i32 * 8i32 + 5i32) as isize);
        *level.offset(52isize) = *dct.offset((6i32 * 8i32 + 4i32) as isize);
        *level.offset(53isize) = *dct.offset((7i32 * 8i32 + 3i32) as isize);
        *level.offset(54isize) = *dct.offset((7i32 * 8i32 + 4i32) as isize);
        *level.offset(55isize) = *dct.offset((6i32 * 8i32 + 5i32) as isize);
        *level.offset(56isize) = *dct.offset((5i32 * 8i32 + 6i32) as isize);
        *level.offset(57isize) = *dct.offset((4i32 * 8i32 + 7i32) as isize);
        *level.offset(58isize) = *dct.offset((5i32 * 8i32 + 7i32) as isize);
        *level.offset(59isize) = *dct.offset((6i32 * 8i32 + 6i32) as isize);
        *level.offset(60isize) = *dct.offset((7i32 * 8i32 + 5i32) as isize);
        *level.offset(61isize) = *dct.offset((7i32 * 8i32 + 6i32) as isize);
        *level.offset(62isize) = *dct.offset((6i32 * 8i32 + 7i32) as isize);
        *level.offset(63isize) = *dct.offset((7i32 * 8i32 + 7i32) as isize);
    }
}
unsafe extern "C" fn zigzag_scan_8x8_field(
    mut level: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        *level.offset(0isize) = *dct.offset((0i32 * 8i32 + 0i32) as isize);
        *level.offset(1isize) = *dct.offset((0i32 * 8i32 + 1i32) as isize);
        *level.offset(2isize) = *dct.offset((0i32 * 8i32 + 2i32) as isize);
        *level.offset(3isize) = *dct.offset((1i32 * 8i32 + 0i32) as isize);
        *level.offset(4isize) = *dct.offset((1i32 * 8i32 + 1i32) as isize);
        *level.offset(5isize) = *dct.offset((0i32 * 8i32 + 3i32) as isize);
        *level.offset(6isize) = *dct.offset((0i32 * 8i32 + 4i32) as isize);
        *level.offset(7isize) = *dct.offset((1i32 * 8i32 + 2i32) as isize);
        *level.offset(8isize) = *dct.offset((2i32 * 8i32 + 0i32) as isize);
        *level.offset(9isize) = *dct.offset((1i32 * 8i32 + 3i32) as isize);
        *level.offset(10isize) = *dct.offset((0i32 * 8i32 + 5i32) as isize);
        *level.offset(11isize) = *dct.offset((0i32 * 8i32 + 6i32) as isize);
        *level.offset(12isize) = *dct.offset((0i32 * 8i32 + 7i32) as isize);
        *level.offset(13isize) = *dct.offset((1i32 * 8i32 + 4i32) as isize);
        *level.offset(14isize) = *dct.offset((2i32 * 8i32 + 1i32) as isize);
        *level.offset(15isize) = *dct.offset((3i32 * 8i32 + 0i32) as isize);
        *level.offset(16isize) = *dct.offset((2i32 * 8i32 + 2i32) as isize);
        *level.offset(17isize) = *dct.offset((1i32 * 8i32 + 5i32) as isize);
        *level.offset(18isize) = *dct.offset((1i32 * 8i32 + 6i32) as isize);
        *level.offset(19isize) = *dct.offset((1i32 * 8i32 + 7i32) as isize);
        *level.offset(20isize) = *dct.offset((2i32 * 8i32 + 3i32) as isize);
        *level.offset(21isize) = *dct.offset((3i32 * 8i32 + 1i32) as isize);
        *level.offset(22isize) = *dct.offset((4i32 * 8i32 + 0i32) as isize);
        *level.offset(23isize) = *dct.offset((3i32 * 8i32 + 2i32) as isize);
        *level.offset(24isize) = *dct.offset((2i32 * 8i32 + 4i32) as isize);
        *level.offset(25isize) = *dct.offset((2i32 * 8i32 + 5i32) as isize);
        *level.offset(26isize) = *dct.offset((2i32 * 8i32 + 6i32) as isize);
        *level.offset(27isize) = *dct.offset((2i32 * 8i32 + 7i32) as isize);
        *level.offset(28isize) = *dct.offset((3i32 * 8i32 + 3i32) as isize);
        *level.offset(29isize) = *dct.offset((4i32 * 8i32 + 1i32) as isize);
        *level.offset(30isize) = *dct.offset((5i32 * 8i32 + 0i32) as isize);
        *level.offset(31isize) = *dct.offset((4i32 * 8i32 + 2i32) as isize);
        *level.offset(32isize) = *dct.offset((3i32 * 8i32 + 4i32) as isize);
        *level.offset(33isize) = *dct.offset((3i32 * 8i32 + 5i32) as isize);
        *level.offset(34isize) = *dct.offset((3i32 * 8i32 + 6i32) as isize);
        *level.offset(35isize) = *dct.offset((3i32 * 8i32 + 7i32) as isize);
        *level.offset(36isize) = *dct.offset((4i32 * 8i32 + 3i32) as isize);
        *level.offset(37isize) = *dct.offset((5i32 * 8i32 + 1i32) as isize);
        *level.offset(38isize) = *dct.offset((6i32 * 8i32 + 0i32) as isize);
        *level.offset(39isize) = *dct.offset((5i32 * 8i32 + 2i32) as isize);
        *level.offset(40isize) = *dct.offset((4i32 * 8i32 + 4i32) as isize);
        *level.offset(41isize) = *dct.offset((4i32 * 8i32 + 5i32) as isize);
        *level.offset(42isize) = *dct.offset((4i32 * 8i32 + 6i32) as isize);
        *level.offset(43isize) = *dct.offset((4i32 * 8i32 + 7i32) as isize);
        *level.offset(44isize) = *dct.offset((5i32 * 8i32 + 3i32) as isize);
        *level.offset(45isize) = *dct.offset((6i32 * 8i32 + 1i32) as isize);
        *level.offset(46isize) = *dct.offset((6i32 * 8i32 + 2i32) as isize);
        *level.offset(47isize) = *dct.offset((5i32 * 8i32 + 4i32) as isize);
        *level.offset(48isize) = *dct.offset((5i32 * 8i32 + 5i32) as isize);
        *level.offset(49isize) = *dct.offset((5i32 * 8i32 + 6i32) as isize);
        *level.offset(50isize) = *dct.offset((5i32 * 8i32 + 7i32) as isize);
        *level.offset(51isize) = *dct.offset((6i32 * 8i32 + 3i32) as isize);
        *level.offset(52isize) = *dct.offset((7i32 * 8i32 + 0i32) as isize);
        *level.offset(53isize) = *dct.offset((7i32 * 8i32 + 1i32) as isize);
        *level.offset(54isize) = *dct.offset((6i32 * 8i32 + 4i32) as isize);
        *level.offset(55isize) = *dct.offset((6i32 * 8i32 + 5i32) as isize);
        *level.offset(56isize) = *dct.offset((6i32 * 8i32 + 6i32) as isize);
        *level.offset(57isize) = *dct.offset((6i32 * 8i32 + 7i32) as isize);
        *level.offset(58isize) = *dct.offset((7i32 * 8i32 + 2i32) as isize);
        *level.offset(59isize) = *dct.offset((7i32 * 8i32 + 3i32) as isize);
        *level.offset(60isize) = *dct.offset((7i32 * 8i32 + 4i32) as isize);
        *level.offset(61isize) = *dct.offset((7i32 * 8i32 + 5i32) as isize);
        *level.offset(62isize) = *dct.offset((7i32 * 8i32 + 6i32) as isize);
        *level.offset(63isize) = *dct.offset((7i32 * 8i32 + 7i32) as isize);
    }
}
unsafe extern "C" fn zigzag_scan_4x4_frame(
    mut level: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        *level.offset(0isize) = *dct.offset((0i32 * 4i32 + 0i32) as isize);
        *level.offset(1isize) = *dct.offset((1i32 * 4i32 + 0i32) as isize);
        *level.offset(2isize) = *dct.offset((0i32 * 4i32 + 1i32) as isize);
        *level.offset(3isize) = *dct.offset((0i32 * 4i32 + 2i32) as isize);
        *level.offset(4isize) = *dct.offset((1i32 * 4i32 + 1i32) as isize);
        *level.offset(5isize) = *dct.offset((2i32 * 4i32 + 0i32) as isize);
        *level.offset(6isize) = *dct.offset((3i32 * 4i32 + 0i32) as isize);
        *level.offset(7isize) = *dct.offset((2i32 * 4i32 + 1i32) as isize);
        *level.offset(8isize) = *dct.offset((1i32 * 4i32 + 2i32) as isize);
        *level.offset(9isize) = *dct.offset((0i32 * 4i32 + 3i32) as isize);
        *level.offset(10isize) = *dct.offset((1i32 * 4i32 + 3i32) as isize);
        *level.offset(11isize) = *dct.offset((2i32 * 4i32 + 2i32) as isize);
        *level.offset(12isize) = *dct.offset((3i32 * 4i32 + 1i32) as isize);
        *level.offset(13isize) = *dct.offset((3i32 * 4i32 + 2i32) as isize);
        *level.offset(14isize) = *dct.offset((2i32 * 4i32 + 3i32) as isize);
        *level.offset(15isize) = *dct.offset((3i32 * 4i32 + 3i32) as isize);
    }
}
unsafe extern "C" fn zigzag_scan_4x4_field(
    mut level: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        crate::stdlib::memcpy(
            level as *mut ::core::ffi::c_void,
            dct as *const ::core::ffi::c_void,
            (2usize).wrapping_mul(::core::mem::size_of::<crate::src::common::common::dctcoef>()),
        );
        *level.offset(2isize) = *dct.offset((1i32 * 4i32 + 0i32) as isize);
        *level.offset(3isize) = *dct.offset((0i32 * 4i32 + 2i32) as isize);
        *level.offset(4isize) = *dct.offset((0i32 * 4i32 + 3i32) as isize);
        *level.offset(5isize) = *dct.offset((1i32 * 4i32 + 1i32) as isize);
        crate::stdlib::memcpy(
            level.offset(6isize) as *mut ::core::ffi::c_void,
            dct.offset(6isize) as *const ::core::ffi::c_void,
            (10usize).wrapping_mul(::core::mem::size_of::<crate::src::common::common::dctcoef>()),
        );
    }
}
unsafe extern "C" fn zigzag_sub_4x4_frame(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz = 0i32;
        let mut oe = 0i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od = 0i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(0isize) = (*p_src.offset(oe as isize) as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(0isize) as ::core::ffi::c_int;
        let mut oe_0 = 1i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_0 = 1i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1isize) = (*p_src.offset(oe_0 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1isize) as ::core::ffi::c_int;
        let mut oe_1 = 0i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_1 = 0i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2isize) = (*p_src.offset(oe_1 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2isize) as ::core::ffi::c_int;
        let mut oe_2 = 0i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_2 = 0i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3isize) = (*p_src.offset(oe_2 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3isize) as ::core::ffi::c_int;
        let mut oe_3 = 1i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_3 = 1i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4isize) = (*p_src.offset(oe_3 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4isize) as ::core::ffi::c_int;
        let mut oe_4 = 2i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_4 = 2i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5isize) = (*p_src.offset(oe_4 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5isize) as ::core::ffi::c_int;
        let mut oe_5 = 3i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_5 = 3i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6isize) = (*p_src.offset(oe_5 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6isize) as ::core::ffi::c_int;
        let mut oe_6 = 2i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_6 = 2i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7isize) = (*p_src.offset(oe_6 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7isize) as ::core::ffi::c_int;
        let mut oe_7 = 1i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_7 = 1i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8isize) = (*p_src.offset(oe_7 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8isize) as ::core::ffi::c_int;
        let mut oe_8 = 0i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_8 = 0i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9isize) = (*p_src.offset(oe_8 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9isize) as ::core::ffi::c_int;
        let mut oe_9 = 1i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_9 = 1i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10isize) = (*p_src.offset(oe_9 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10isize) as ::core::ffi::c_int;
        let mut oe_10 = 2i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_10 = 2i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11isize) = (*p_src.offset(oe_10 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11isize) as ::core::ffi::c_int;
        let mut oe_11 = 3i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_11 = 3i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12isize) = (*p_src.offset(oe_11 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12isize) as ::core::ffi::c_int;
        let mut oe_12 = 3i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_12 = 3i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13isize) = (*p_src.offset(oe_12 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13isize) as ::core::ffi::c_int;
        let mut oe_13 = 2i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_13 = 2i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14isize) = (*p_src.offset(oe_13 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14isize) as ::core::ffi::c_int;
        let mut oe_14 = 3i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_14 = 3i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15isize) = (*p_src.offset(oe_14 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((0i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((1i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((1i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((2i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((2i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((3i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((3i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (nz != 0) as ::core::ffi::c_int
    }
}
unsafe extern "C" fn zigzag_sub_4x4_field(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz = 0i32;
        let mut oe = 0i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od = 0i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(0isize) = (*p_src.offset(oe as isize) as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(0isize) as ::core::ffi::c_int;
        let mut oe_0 = 0i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_0 = 0i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1isize) = (*p_src.offset(oe_0 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1isize) as ::core::ffi::c_int;
        let mut oe_1 = 1i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_1 = 1i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2isize) = (*p_src.offset(oe_1 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2isize) as ::core::ffi::c_int;
        let mut oe_2 = 0i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_2 = 0i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3isize) = (*p_src.offset(oe_2 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3isize) as ::core::ffi::c_int;
        let mut oe_3 = 0i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_3 = 0i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4isize) = (*p_src.offset(oe_3 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4isize) as ::core::ffi::c_int;
        let mut oe_4 = 1i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_4 = 1i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5isize) = (*p_src.offset(oe_4 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5isize) as ::core::ffi::c_int;
        let mut oe_5 = 1i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_5 = 1i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6isize) = (*p_src.offset(oe_5 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6isize) as ::core::ffi::c_int;
        let mut oe_6 = 1i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_6 = 1i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7isize) = (*p_src.offset(oe_6 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7isize) as ::core::ffi::c_int;
        let mut oe_7 = 2i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_7 = 2i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8isize) = (*p_src.offset(oe_7 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8isize) as ::core::ffi::c_int;
        let mut oe_8 = 2i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_8 = 2i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9isize) = (*p_src.offset(oe_8 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9isize) as ::core::ffi::c_int;
        let mut oe_9 = 2i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_9 = 2i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10isize) = (*p_src.offset(oe_9 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10isize) as ::core::ffi::c_int;
        let mut oe_10 = 2i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_10 = 2i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11isize) = (*p_src.offset(oe_10 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11isize) as ::core::ffi::c_int;
        let mut oe_11 = 3i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_11 = 3i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12isize) = (*p_src.offset(oe_11 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12isize) as ::core::ffi::c_int;
        let mut oe_12 = 3i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_12 = 3i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13isize) = (*p_src.offset(oe_12 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13isize) as ::core::ffi::c_int;
        let mut oe_13 = 3i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_13 = 3i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14isize) = (*p_src.offset(oe_13 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14isize) as ::core::ffi::c_int;
        let mut oe_14 = 3i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_14 = 3i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15isize) = (*p_src.offset(oe_14 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((0i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((1i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((1i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((2i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((2i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((3i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((3i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (nz != 0) as ::core::ffi::c_int
    }
}
unsafe extern "C" fn zigzag_sub_4x4ac_frame(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dc: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz = 0i32;
        let mut oe = 0i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od = 0i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *dc = (*p_src.offset(oe as isize) as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        *level.offset(0isize) = 0i16;
        let mut oe_0 = 1i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_0 = 1i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1isize) = (*p_src.offset(oe_0 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1isize) as ::core::ffi::c_int;
        let mut oe_1 = 0i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_1 = 0i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2isize) = (*p_src.offset(oe_1 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2isize) as ::core::ffi::c_int;
        let mut oe_2 = 0i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_2 = 0i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3isize) = (*p_src.offset(oe_2 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3isize) as ::core::ffi::c_int;
        let mut oe_3 = 1i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_3 = 1i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4isize) = (*p_src.offset(oe_3 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4isize) as ::core::ffi::c_int;
        let mut oe_4 = 2i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_4 = 2i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5isize) = (*p_src.offset(oe_4 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5isize) as ::core::ffi::c_int;
        let mut oe_5 = 3i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_5 = 3i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6isize) = (*p_src.offset(oe_5 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6isize) as ::core::ffi::c_int;
        let mut oe_6 = 2i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_6 = 2i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7isize) = (*p_src.offset(oe_6 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7isize) as ::core::ffi::c_int;
        let mut oe_7 = 1i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_7 = 1i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8isize) = (*p_src.offset(oe_7 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8isize) as ::core::ffi::c_int;
        let mut oe_8 = 0i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_8 = 0i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9isize) = (*p_src.offset(oe_8 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9isize) as ::core::ffi::c_int;
        let mut oe_9 = 1i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_9 = 1i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10isize) = (*p_src.offset(oe_9 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10isize) as ::core::ffi::c_int;
        let mut oe_10 = 2i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_10 = 2i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11isize) = (*p_src.offset(oe_10 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11isize) as ::core::ffi::c_int;
        let mut oe_11 = 3i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_11 = 3i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12isize) = (*p_src.offset(oe_11 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12isize) as ::core::ffi::c_int;
        let mut oe_12 = 3i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_12 = 3i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13isize) = (*p_src.offset(oe_12 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13isize) as ::core::ffi::c_int;
        let mut oe_13 = 2i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_13 = 2i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14isize) = (*p_src.offset(oe_13 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14isize) as ::core::ffi::c_int;
        let mut oe_14 = 3i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_14 = 3i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15isize) = (*p_src.offset(oe_14 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((0i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((1i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((1i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((2i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((2i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((3i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((3i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (nz != 0) as ::core::ffi::c_int
    }
}
unsafe extern "C" fn zigzag_sub_4x4ac_field(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dc: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz = 0i32;
        let mut oe = 0i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od = 0i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *dc = (*p_src.offset(oe as isize) as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        *level.offset(0isize) = 0i16;
        let mut oe_0 = 0i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_0 = 0i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1isize) = (*p_src.offset(oe_0 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1isize) as ::core::ffi::c_int;
        let mut oe_1 = 1i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_1 = 1i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2isize) = (*p_src.offset(oe_1 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2isize) as ::core::ffi::c_int;
        let mut oe_2 = 0i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_2 = 0i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3isize) = (*p_src.offset(oe_2 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3isize) as ::core::ffi::c_int;
        let mut oe_3 = 0i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_3 = 0i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4isize) = (*p_src.offset(oe_3 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4isize) as ::core::ffi::c_int;
        let mut oe_4 = 1i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_4 = 1i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5isize) = (*p_src.offset(oe_4 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5isize) as ::core::ffi::c_int;
        let mut oe_5 = 1i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_5 = 1i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6isize) = (*p_src.offset(oe_5 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6isize) as ::core::ffi::c_int;
        let mut oe_6 = 1i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_6 = 1i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7isize) = (*p_src.offset(oe_6 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7isize) as ::core::ffi::c_int;
        let mut oe_7 = 2i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_7 = 2i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8isize) = (*p_src.offset(oe_7 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8isize) as ::core::ffi::c_int;
        let mut oe_8 = 2i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_8 = 2i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9isize) = (*p_src.offset(oe_8 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9isize) as ::core::ffi::c_int;
        let mut oe_9 = 2i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_9 = 2i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10isize) = (*p_src.offset(oe_9 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10isize) as ::core::ffi::c_int;
        let mut oe_10 = 2i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_10 = 2i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11isize) = (*p_src.offset(oe_10 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11isize) as ::core::ffi::c_int;
        let mut oe_11 = 3i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_11 = 3i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12isize) = (*p_src.offset(oe_11 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12isize) as ::core::ffi::c_int;
        let mut oe_12 = 3i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_12 = 3i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13isize) = (*p_src.offset(oe_12 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13isize) as ::core::ffi::c_int;
        let mut oe_13 = 3i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_13 = 3i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14isize) = (*p_src.offset(oe_13 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14isize) as ::core::ffi::c_int;
        let mut oe_14 = 3i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_14 = 3i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15isize) = (*p_src.offset(oe_14 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((0i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((1i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((1i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((2i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((2i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((3i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((3i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (nz != 0) as ::core::ffi::c_int
    }
}
unsafe extern "C" fn zigzag_sub_8x8_frame(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz = 0i32;
        let mut oe = 0i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od = 0i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(0isize) = (*p_src.offset(oe as isize) as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(0isize) as ::core::ffi::c_int;
        let mut oe_0 = 1i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_0 = 1i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1isize) = (*p_src.offset(oe_0 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1isize) as ::core::ffi::c_int;
        let mut oe_1 = 0i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_1 = 0i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2isize) = (*p_src.offset(oe_1 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2isize) as ::core::ffi::c_int;
        let mut oe_2 = 0i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_2 = 0i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3isize) = (*p_src.offset(oe_2 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3isize) as ::core::ffi::c_int;
        let mut oe_3 = 1i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_3 = 1i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4isize) = (*p_src.offset(oe_3 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4isize) as ::core::ffi::c_int;
        let mut oe_4 = 2i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_4 = 2i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5isize) = (*p_src.offset(oe_4 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5isize) as ::core::ffi::c_int;
        let mut oe_5 = 3i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_5 = 3i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6isize) = (*p_src.offset(oe_5 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6isize) as ::core::ffi::c_int;
        let mut oe_6 = 2i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_6 = 2i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7isize) = (*p_src.offset(oe_6 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7isize) as ::core::ffi::c_int;
        let mut oe_7 = 1i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_7 = 1i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8isize) = (*p_src.offset(oe_7 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8isize) as ::core::ffi::c_int;
        let mut oe_8 = 0i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_8 = 0i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9isize) = (*p_src.offset(oe_8 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9isize) as ::core::ffi::c_int;
        let mut oe_9 = 0i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_9 = 0i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10isize) = (*p_src.offset(oe_9 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10isize) as ::core::ffi::c_int;
        let mut oe_10 = 1i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_10 = 1i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11isize) = (*p_src.offset(oe_10 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11isize) as ::core::ffi::c_int;
        let mut oe_11 = 2i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_11 = 2i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12isize) = (*p_src.offset(oe_11 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12isize) as ::core::ffi::c_int;
        let mut oe_12 = 3i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_12 = 3i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13isize) = (*p_src.offset(oe_12 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13isize) as ::core::ffi::c_int;
        let mut oe_13 = 4i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_13 = 4i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14isize) = (*p_src.offset(oe_13 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14isize) as ::core::ffi::c_int;
        let mut oe_14 = 5i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_14 = 5i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15isize) = (*p_src.offset(oe_14 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15isize) as ::core::ffi::c_int;
        let mut oe_15 = 4i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_15 = 4i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(16isize) = (*p_src.offset(oe_15 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_15 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(16isize) as ::core::ffi::c_int;
        let mut oe_16 = 3i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_16 = 3i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(17isize) = (*p_src.offset(oe_16 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_16 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(17isize) as ::core::ffi::c_int;
        let mut oe_17 = 2i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_17 = 2i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(18isize) = (*p_src.offset(oe_17 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_17 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(18isize) as ::core::ffi::c_int;
        let mut oe_18 = 1i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_18 = 1i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(19isize) = (*p_src.offset(oe_18 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_18 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(19isize) as ::core::ffi::c_int;
        let mut oe_19 = 0i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_19 = 0i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(20isize) = (*p_src.offset(oe_19 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_19 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(20isize) as ::core::ffi::c_int;
        let mut oe_20 = 0i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_20 = 0i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(21isize) = (*p_src.offset(oe_20 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_20 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(21isize) as ::core::ffi::c_int;
        let mut oe_21 = 1i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_21 = 1i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(22isize) = (*p_src.offset(oe_21 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_21 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(22isize) as ::core::ffi::c_int;
        let mut oe_22 = 2i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_22 = 2i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(23isize) = (*p_src.offset(oe_22 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_22 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(23isize) as ::core::ffi::c_int;
        let mut oe_23 = 3i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_23 = 3i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(24isize) = (*p_src.offset(oe_23 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_23 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(24isize) as ::core::ffi::c_int;
        let mut oe_24 = 4i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_24 = 4i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(25isize) = (*p_src.offset(oe_24 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_24 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(25isize) as ::core::ffi::c_int;
        let mut oe_25 = 5i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_25 = 5i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(26isize) = (*p_src.offset(oe_25 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_25 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(26isize) as ::core::ffi::c_int;
        let mut oe_26 = 6i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_26 = 6i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(27isize) = (*p_src.offset(oe_26 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_26 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(27isize) as ::core::ffi::c_int;
        let mut oe_27 = 7i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_27 = 7i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(28isize) = (*p_src.offset(oe_27 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_27 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(28isize) as ::core::ffi::c_int;
        let mut oe_28 = 6i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_28 = 6i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(29isize) = (*p_src.offset(oe_28 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_28 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(29isize) as ::core::ffi::c_int;
        let mut oe_29 = 5i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_29 = 5i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(30isize) = (*p_src.offset(oe_29 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_29 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(30isize) as ::core::ffi::c_int;
        let mut oe_30 = 4i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_30 = 4i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(31isize) = (*p_src.offset(oe_30 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_30 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(31isize) as ::core::ffi::c_int;
        let mut oe_31 = 3i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_31 = 3i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(32isize) = (*p_src.offset(oe_31 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_31 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(32isize) as ::core::ffi::c_int;
        let mut oe_32 = 2i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_32 = 2i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(33isize) = (*p_src.offset(oe_32 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_32 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(33isize) as ::core::ffi::c_int;
        let mut oe_33 = 1i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_33 = 1i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(34isize) = (*p_src.offset(oe_33 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_33 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(34isize) as ::core::ffi::c_int;
        let mut oe_34 = 0i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_34 = 0i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(35isize) = (*p_src.offset(oe_34 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_34 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(35isize) as ::core::ffi::c_int;
        let mut oe_35 = 1i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_35 = 1i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(36isize) = (*p_src.offset(oe_35 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_35 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(36isize) as ::core::ffi::c_int;
        let mut oe_36 = 2i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_36 = 2i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(37isize) = (*p_src.offset(oe_36 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_36 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(37isize) as ::core::ffi::c_int;
        let mut oe_37 = 3i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_37 = 3i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(38isize) = (*p_src.offset(oe_37 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_37 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(38isize) as ::core::ffi::c_int;
        let mut oe_38 = 4i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_38 = 4i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(39isize) = (*p_src.offset(oe_38 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_38 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(39isize) as ::core::ffi::c_int;
        let mut oe_39 = 5i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_39 = 5i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(40isize) = (*p_src.offset(oe_39 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_39 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(40isize) as ::core::ffi::c_int;
        let mut oe_40 = 6i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_40 = 6i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(41isize) = (*p_src.offset(oe_40 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_40 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(41isize) as ::core::ffi::c_int;
        let mut oe_41 = 7i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_41 = 7i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(42isize) = (*p_src.offset(oe_41 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_41 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(42isize) as ::core::ffi::c_int;
        let mut oe_42 = 7i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_42 = 7i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(43isize) = (*p_src.offset(oe_42 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_42 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(43isize) as ::core::ffi::c_int;
        let mut oe_43 = 6i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_43 = 6i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(44isize) = (*p_src.offset(oe_43 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_43 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(44isize) as ::core::ffi::c_int;
        let mut oe_44 = 5i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_44 = 5i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(45isize) = (*p_src.offset(oe_44 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_44 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(45isize) as ::core::ffi::c_int;
        let mut oe_45 = 4i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_45 = 4i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(46isize) = (*p_src.offset(oe_45 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_45 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(46isize) as ::core::ffi::c_int;
        let mut oe_46 = 3i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_46 = 3i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(47isize) = (*p_src.offset(oe_46 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_46 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(47isize) as ::core::ffi::c_int;
        let mut oe_47 = 2i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_47 = 2i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(48isize) = (*p_src.offset(oe_47 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_47 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(48isize) as ::core::ffi::c_int;
        let mut oe_48 = 3i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_48 = 3i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(49isize) = (*p_src.offset(oe_48 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_48 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(49isize) as ::core::ffi::c_int;
        let mut oe_49 = 4i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_49 = 4i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(50isize) = (*p_src.offset(oe_49 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_49 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(50isize) as ::core::ffi::c_int;
        let mut oe_50 = 5i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_50 = 5i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(51isize) = (*p_src.offset(oe_50 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_50 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(51isize) as ::core::ffi::c_int;
        let mut oe_51 = 6i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_51 = 6i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(52isize) = (*p_src.offset(oe_51 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_51 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(52isize) as ::core::ffi::c_int;
        let mut oe_52 = 7i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_52 = 7i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(53isize) = (*p_src.offset(oe_52 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_52 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(53isize) as ::core::ffi::c_int;
        let mut oe_53 = 7i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_53 = 7i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(54isize) = (*p_src.offset(oe_53 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_53 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(54isize) as ::core::ffi::c_int;
        let mut oe_54 = 6i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_54 = 6i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(55isize) = (*p_src.offset(oe_54 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_54 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(55isize) as ::core::ffi::c_int;
        let mut oe_55 = 5i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_55 = 5i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(56isize) = (*p_src.offset(oe_55 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_55 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(56isize) as ::core::ffi::c_int;
        let mut oe_56 = 4i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_56 = 4i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(57isize) = (*p_src.offset(oe_56 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_56 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(57isize) as ::core::ffi::c_int;
        let mut oe_57 = 5i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_57 = 5i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(58isize) = (*p_src.offset(oe_57 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_57 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(58isize) as ::core::ffi::c_int;
        let mut oe_58 = 6i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_58 = 6i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(59isize) = (*p_src.offset(oe_58 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_58 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(59isize) as ::core::ffi::c_int;
        let mut oe_59 = 7i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_59 = 7i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(60isize) = (*p_src.offset(oe_59 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_59 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(60isize) as ::core::ffi::c_int;
        let mut oe_60 = 7i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_60 = 7i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(61isize) = (*p_src.offset(oe_60 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_60 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(61isize) as ::core::ffi::c_int;
        let mut oe_61 = 6i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_61 = 6i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(62isize) = (*p_src.offset(oe_61 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_61 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(62isize) as ::core::ffi::c_int;
        let mut oe_62 = 7i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_62 = 7i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(63isize) = (*p_src.offset(oe_62 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_62 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(63isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((0i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((0i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((0i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((1i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((1i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((1i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((1i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((2i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((2i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((2i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((2i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((3i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((3i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((3i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((3i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((4i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((4i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((4i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((4i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((5i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((5i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((5i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((5i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((6i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((6i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((6i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((6i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((7i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((7i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((7i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((7i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (nz != 0) as ::core::ffi::c_int
    }
}
unsafe extern "C" fn zigzag_sub_8x8_field(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz = 0i32;
        let mut oe = 0i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od = 0i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(0isize) = (*p_src.offset(oe as isize) as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(0isize) as ::core::ffi::c_int;
        let mut oe_0 = 0i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_0 = 0i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1isize) = (*p_src.offset(oe_0 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1isize) as ::core::ffi::c_int;
        let mut oe_1 = 0i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_1 = 0i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2isize) = (*p_src.offset(oe_1 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2isize) as ::core::ffi::c_int;
        let mut oe_2 = 1i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_2 = 1i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3isize) = (*p_src.offset(oe_2 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3isize) as ::core::ffi::c_int;
        let mut oe_3 = 1i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_3 = 1i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4isize) = (*p_src.offset(oe_3 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4isize) as ::core::ffi::c_int;
        let mut oe_4 = 0i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_4 = 0i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5isize) = (*p_src.offset(oe_4 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5isize) as ::core::ffi::c_int;
        let mut oe_5 = 0i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_5 = 0i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6isize) = (*p_src.offset(oe_5 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6isize) as ::core::ffi::c_int;
        let mut oe_6 = 1i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_6 = 1i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7isize) = (*p_src.offset(oe_6 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7isize) as ::core::ffi::c_int;
        let mut oe_7 = 2i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_7 = 2i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8isize) = (*p_src.offset(oe_7 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8isize) as ::core::ffi::c_int;
        let mut oe_8 = 1i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_8 = 1i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9isize) = (*p_src.offset(oe_8 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9isize) as ::core::ffi::c_int;
        let mut oe_9 = 0i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_9 = 0i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10isize) = (*p_src.offset(oe_9 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10isize) as ::core::ffi::c_int;
        let mut oe_10 = 0i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_10 = 0i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11isize) = (*p_src.offset(oe_10 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11isize) as ::core::ffi::c_int;
        let mut oe_11 = 0i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_11 = 0i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12isize) = (*p_src.offset(oe_11 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12isize) as ::core::ffi::c_int;
        let mut oe_12 = 1i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_12 = 1i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13isize) = (*p_src.offset(oe_12 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13isize) as ::core::ffi::c_int;
        let mut oe_13 = 2i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_13 = 2i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14isize) = (*p_src.offset(oe_13 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14isize) as ::core::ffi::c_int;
        let mut oe_14 = 3i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_14 = 3i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15isize) = (*p_src.offset(oe_14 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15isize) as ::core::ffi::c_int;
        let mut oe_15 = 2i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_15 = 2i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(16isize) = (*p_src.offset(oe_15 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_15 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(16isize) as ::core::ffi::c_int;
        let mut oe_16 = 1i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_16 = 1i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(17isize) = (*p_src.offset(oe_16 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_16 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(17isize) as ::core::ffi::c_int;
        let mut oe_17 = 1i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_17 = 1i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(18isize) = (*p_src.offset(oe_17 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_17 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(18isize) as ::core::ffi::c_int;
        let mut oe_18 = 1i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_18 = 1i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(19isize) = (*p_src.offset(oe_18 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_18 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(19isize) as ::core::ffi::c_int;
        let mut oe_19 = 2i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_19 = 2i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(20isize) = (*p_src.offset(oe_19 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_19 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(20isize) as ::core::ffi::c_int;
        let mut oe_20 = 3i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_20 = 3i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(21isize) = (*p_src.offset(oe_20 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_20 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(21isize) as ::core::ffi::c_int;
        let mut oe_21 = 4i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_21 = 4i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(22isize) = (*p_src.offset(oe_21 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_21 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(22isize) as ::core::ffi::c_int;
        let mut oe_22 = 3i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_22 = 3i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(23isize) = (*p_src.offset(oe_22 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_22 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(23isize) as ::core::ffi::c_int;
        let mut oe_23 = 2i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_23 = 2i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(24isize) = (*p_src.offset(oe_23 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_23 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(24isize) as ::core::ffi::c_int;
        let mut oe_24 = 2i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_24 = 2i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(25isize) = (*p_src.offset(oe_24 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_24 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(25isize) as ::core::ffi::c_int;
        let mut oe_25 = 2i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_25 = 2i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(26isize) = (*p_src.offset(oe_25 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_25 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(26isize) as ::core::ffi::c_int;
        let mut oe_26 = 2i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_26 = 2i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(27isize) = (*p_src.offset(oe_26 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_26 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(27isize) as ::core::ffi::c_int;
        let mut oe_27 = 3i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_27 = 3i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(28isize) = (*p_src.offset(oe_27 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_27 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(28isize) as ::core::ffi::c_int;
        let mut oe_28 = 4i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_28 = 4i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(29isize) = (*p_src.offset(oe_28 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_28 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(29isize) as ::core::ffi::c_int;
        let mut oe_29 = 5i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_29 = 5i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(30isize) = (*p_src.offset(oe_29 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_29 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(30isize) as ::core::ffi::c_int;
        let mut oe_30 = 4i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_30 = 4i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(31isize) = (*p_src.offset(oe_30 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_30 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(31isize) as ::core::ffi::c_int;
        let mut oe_31 = 3i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_31 = 3i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(32isize) = (*p_src.offset(oe_31 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_31 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(32isize) as ::core::ffi::c_int;
        let mut oe_32 = 3i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_32 = 3i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(33isize) = (*p_src.offset(oe_32 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_32 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(33isize) as ::core::ffi::c_int;
        let mut oe_33 = 3i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_33 = 3i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(34isize) = (*p_src.offset(oe_33 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_33 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(34isize) as ::core::ffi::c_int;
        let mut oe_34 = 3i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_34 = 3i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(35isize) = (*p_src.offset(oe_34 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_34 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(35isize) as ::core::ffi::c_int;
        let mut oe_35 = 4i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_35 = 4i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(36isize) = (*p_src.offset(oe_35 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_35 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(36isize) as ::core::ffi::c_int;
        let mut oe_36 = 5i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_36 = 5i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(37isize) = (*p_src.offset(oe_36 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_36 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(37isize) as ::core::ffi::c_int;
        let mut oe_37 = 6i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_37 = 6i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(38isize) = (*p_src.offset(oe_37 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_37 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(38isize) as ::core::ffi::c_int;
        let mut oe_38 = 5i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_38 = 5i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(39isize) = (*p_src.offset(oe_38 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_38 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(39isize) as ::core::ffi::c_int;
        let mut oe_39 = 4i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_39 = 4i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(40isize) = (*p_src.offset(oe_39 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_39 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(40isize) as ::core::ffi::c_int;
        let mut oe_40 = 4i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_40 = 4i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(41isize) = (*p_src.offset(oe_40 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_40 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(41isize) as ::core::ffi::c_int;
        let mut oe_41 = 4i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_41 = 4i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(42isize) = (*p_src.offset(oe_41 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_41 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(42isize) as ::core::ffi::c_int;
        let mut oe_42 = 4i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_42 = 4i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(43isize) = (*p_src.offset(oe_42 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_42 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(43isize) as ::core::ffi::c_int;
        let mut oe_43 = 5i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_43 = 5i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(44isize) = (*p_src.offset(oe_43 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_43 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(44isize) as ::core::ffi::c_int;
        let mut oe_44 = 6i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_44 = 6i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(45isize) = (*p_src.offset(oe_44 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_44 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(45isize) as ::core::ffi::c_int;
        let mut oe_45 = 6i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_45 = 6i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(46isize) = (*p_src.offset(oe_45 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_45 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(46isize) as ::core::ffi::c_int;
        let mut oe_46 = 5i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_46 = 5i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(47isize) = (*p_src.offset(oe_46 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_46 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(47isize) as ::core::ffi::c_int;
        let mut oe_47 = 5i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_47 = 5i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(48isize) = (*p_src.offset(oe_47 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_47 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(48isize) as ::core::ffi::c_int;
        let mut oe_48 = 5i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_48 = 5i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(49isize) = (*p_src.offset(oe_48 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_48 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(49isize) as ::core::ffi::c_int;
        let mut oe_49 = 5i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_49 = 5i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(50isize) = (*p_src.offset(oe_49 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_49 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(50isize) as ::core::ffi::c_int;
        let mut oe_50 = 6i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_50 = 6i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(51isize) = (*p_src.offset(oe_50 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_50 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(51isize) as ::core::ffi::c_int;
        let mut oe_51 = 7i32 + 0i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_51 = 7i32 + 0i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(52isize) = (*p_src.offset(oe_51 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_51 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(52isize) as ::core::ffi::c_int;
        let mut oe_52 = 7i32 + 1i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_52 = 7i32 + 1i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(53isize) = (*p_src.offset(oe_52 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_52 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(53isize) as ::core::ffi::c_int;
        let mut oe_53 = 6i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_53 = 6i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(54isize) = (*p_src.offset(oe_53 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_53 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(54isize) as ::core::ffi::c_int;
        let mut oe_54 = 6i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_54 = 6i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(55isize) = (*p_src.offset(oe_54 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_54 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(55isize) as ::core::ffi::c_int;
        let mut oe_55 = 6i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_55 = 6i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(56isize) = (*p_src.offset(oe_55 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_55 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(56isize) as ::core::ffi::c_int;
        let mut oe_56 = 6i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_56 = 6i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(57isize) = (*p_src.offset(oe_56 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_56 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(57isize) as ::core::ffi::c_int;
        let mut oe_57 = 7i32 + 2i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_57 = 7i32 + 2i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(58isize) = (*p_src.offset(oe_57 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_57 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(58isize) as ::core::ffi::c_int;
        let mut oe_58 = 7i32 + 3i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_58 = 7i32 + 3i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(59isize) = (*p_src.offset(oe_58 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_58 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(59isize) as ::core::ffi::c_int;
        let mut oe_59 = 7i32 + 4i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_59 = 7i32 + 4i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(60isize) = (*p_src.offset(oe_59 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_59 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(60isize) as ::core::ffi::c_int;
        let mut oe_60 = 7i32 + 5i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_60 = 7i32 + 5i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(61isize) = (*p_src.offset(oe_60 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_60 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(61isize) as ::core::ffi::c_int;
        let mut oe_61 = 7i32 + 6i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_61 = 7i32 + 6i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(62isize) = (*p_src.offset(oe_61 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_61 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(62isize) as ::core::ffi::c_int;
        let mut oe_62 = 7i32 + 7i32 * crate::src::common::common::FENC_STRIDE;
        let mut od_62 = 7i32 + 7i32 * crate::src::common::common::FDEC_STRIDE;
        *level.offset(63isize) = (*p_src.offset(oe_62 as isize) as ::core::ffi::c_int
            - *p_dst.offset(od_62 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(63isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((0i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((0i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((0i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((1i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((1i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((1i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((1i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((2i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((2i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((2i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((2i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((3i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((3i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((3i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((3i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((4i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((4i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((4i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((4i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((5i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((5i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((5i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((5i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((6i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((6i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((6i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((6i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((7i32 * 32i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i =
            (*(p_src.offset((7i32 * 16i32) as isize) as *mut crate::src::common::base::x264_union32_t)).i;
        (*(p_dst.offset((7i32 * 32i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((7i32 * 16i32) as isize).offset(4isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (nz != 0) as ::core::ffi::c_int
    }
}
unsafe extern "C" fn zigzag_interleave_8x8_cavlc(
    mut dst: *mut crate::src::common::common::dctcoef,
    mut src: *mut crate::src::common::common::dctcoef,
    mut nnz: *mut crate::stdlib::uint8_t,
) {
    unsafe {
        let mut i = 0i32;
        while i < 4i32 {
            let mut nz = 0i32;
            let mut j = 0i32;
            while j < 16i32 {
                nz |= *src.offset((i + j * 4i32) as isize) as ::core::ffi::c_int;
                *dst.offset((i * 16i32 + j) as isize) = *src.offset((i + j * 4i32) as isize);
                j += 1;
            }
            *nnz.offset(((i & 1i32) + (i >> 1i32) * 8i32) as isize) = (nz != 0) as crate::stdlib::uint8_t;
            i += 1;
        }
    }
}
pub unsafe extern "C" fn x264_8_zigzag_init(
    mut _cpu: crate::stdlib::uint32_t,
    mut pf_progressive: *mut crate::src::common::dct::x264_zigzag_function_t,
    mut pf_interlaced: *mut crate::src::common::dct::x264_zigzag_function_t,
) {
    unsafe {
        (*pf_interlaced).scan_8x8 = Some(
            zigzag_scan_8x8_field
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        );
        (*pf_progressive).scan_8x8 = Some(
            zigzag_scan_8x8_frame
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        );
        (*pf_interlaced).scan_4x4 = Some(
            zigzag_scan_4x4_field
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        );
        (*pf_progressive).scan_4x4 = Some(
            zigzag_scan_4x4_frame
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        );
        (*pf_interlaced).sub_8x8 = Some(
            zigzag_sub_8x8_field
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
        );
        (*pf_progressive).sub_8x8 = Some(
            zigzag_sub_8x8_frame
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
        );
        (*pf_interlaced).sub_4x4 = Some(
            zigzag_sub_4x4_field
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
        );
        (*pf_progressive).sub_4x4 = Some(
            zigzag_sub_4x4_frame
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
        );
        (*pf_interlaced).sub_4x4ac = Some(
            zigzag_sub_4x4ac_field
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        );
        (*pf_progressive).sub_4x4ac = Some(
            zigzag_sub_4x4ac_frame
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        );
        (*pf_progressive).interleave_8x8_cavlc = Some(
            zigzag_interleave_8x8_cavlc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::stdlib::uint8_t,
                ) -> (),
        );
        (*pf_interlaced).interleave_8x8_cavlc = (*pf_progressive).interleave_8x8_cavlc;
    }
}
