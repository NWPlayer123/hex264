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

use crate::src::common::dct::common_h::x264_clip_pixel;

unsafe extern "C" fn dct4x4dc(mut d: *mut crate::src::common::common::dctcoef) {
    unsafe {
        let mut tmp: [crate::src::common::common::dctcoef; 16] = [0; 16];
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let mut s01: ::core::ffi::c_int = *d
                .offset((i * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *d.offset((i * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            let mut d01: ::core::ffi::c_int = *d
                .offset((i * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - *d.offset((i * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            let mut s23: ::core::ffi::c_int = *d
                .offset((i * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *d.offset((i * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            let mut d23: ::core::ffi::c_int = *d
                .offset((i * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - *d.offset((i * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            tmp[(0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (s01 + s23) as crate::src::common::common::dctcoef;
            tmp[(1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (s01 - s23) as crate::src::common::common::dctcoef;
            tmp[(2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (d01 - d23) as crate::src::common::common::dctcoef;
            tmp[(3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (d01 + d23) as crate::src::common::common::dctcoef;
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 4 as ::core::ffi::c_int {
            let mut s01_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + tmp[(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut d01_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - tmp[(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut s23_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut d23_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            *d.offset((i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
                (s01_0 + s23_0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (s01_0 - s23_0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                (d01_0 - d23_0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) =
                (d01_0 + d23_0 + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
            i_0 += 1;
        }
    }
}

unsafe extern "C" fn idct4x4dc(mut d: *mut crate::src::common::common::dctcoef) {
    unsafe {
        let mut tmp: [crate::src::common::common::dctcoef; 16] = [0; 16];
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let mut s01: ::core::ffi::c_int = *d
                .offset((i * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *d.offset((i * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            let mut d01: ::core::ffi::c_int = *d
                .offset((i * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - *d.offset((i * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            let mut s23: ::core::ffi::c_int = *d
                .offset((i * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *d.offset((i * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            let mut d23: ::core::ffi::c_int = *d
                .offset((i * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - *d.offset((i * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            tmp[(0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (s01 + s23) as crate::src::common::common::dctcoef;
            tmp[(1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (s01 - s23) as crate::src::common::common::dctcoef;
            tmp[(2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (d01 - d23) as crate::src::common::common::dctcoef;
            tmp[(3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (d01 + d23) as crate::src::common::common::dctcoef;
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 4 as ::core::ffi::c_int {
            let mut s01_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + tmp[(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut d01_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - tmp[(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut s23_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut d23_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            *d.offset((i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
                (s01_0 + s23_0) as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (s01_0 - s23_0) as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                (d01_0 - d23_0) as crate::src::common::common::dctcoef;
            *d.offset((i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) =
                (d01_0 + d23_0) as crate::src::common::common::dctcoef;
            i_0 += 1;
        }
    }
}

unsafe extern "C" fn dct2x4dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut dct4x4: *mut [crate::src::common::common::dctcoef; 16],
) {
    unsafe {
        let mut a0: ::core::ffi::c_int = (*dct4x4.offset(0 as ::core::ffi::c_int as isize))
            [0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            + (*dct4x4.offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int;
        let mut a1: ::core::ffi::c_int = (*dct4x4.offset(2 as ::core::ffi::c_int as isize))
            [0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            + (*dct4x4.offset(3 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int;
        let mut a2: ::core::ffi::c_int = (*dct4x4.offset(4 as ::core::ffi::c_int as isize))
            [0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            + (*dct4x4.offset(5 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int;
        let mut a3: ::core::ffi::c_int = (*dct4x4.offset(6 as ::core::ffi::c_int as isize))
            [0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            + (*dct4x4.offset(7 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int;
        let mut a4: ::core::ffi::c_int = (*dct4x4.offset(0 as ::core::ffi::c_int as isize))
            [0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - (*dct4x4.offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int;
        let mut a5: ::core::ffi::c_int = (*dct4x4.offset(2 as ::core::ffi::c_int as isize))
            [0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - (*dct4x4.offset(3 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int;
        let mut a6: ::core::ffi::c_int = (*dct4x4.offset(4 as ::core::ffi::c_int as isize))
            [0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - (*dct4x4.offset(5 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int;
        let mut a7: ::core::ffi::c_int = (*dct4x4.offset(6 as ::core::ffi::c_int as isize))
            [0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - (*dct4x4.offset(7 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int;
        let mut b0: ::core::ffi::c_int = a0 + a1;
        let mut b1: ::core::ffi::c_int = a2 + a3;
        let mut b2: ::core::ffi::c_int = a4 + a5;
        let mut b3: ::core::ffi::c_int = a6 + a7;
        let mut b4: ::core::ffi::c_int = a0 - a1;
        let mut b5: ::core::ffi::c_int = a2 - a3;
        let mut b6: ::core::ffi::c_int = a4 - a5;
        let mut b7: ::core::ffi::c_int = a6 - a7;
        *dct.offset(0 as ::core::ffi::c_int as isize) =
            (b0 + b1) as crate::src::common::common::dctcoef;
        *dct.offset(1 as ::core::ffi::c_int as isize) =
            (b2 + b3) as crate::src::common::common::dctcoef;
        *dct.offset(2 as ::core::ffi::c_int as isize) =
            (b0 - b1) as crate::src::common::common::dctcoef;
        *dct.offset(3 as ::core::ffi::c_int as isize) =
            (b2 - b3) as crate::src::common::common::dctcoef;
        *dct.offset(4 as ::core::ffi::c_int as isize) =
            (b4 - b5) as crate::src::common::common::dctcoef;
        *dct.offset(5 as ::core::ffi::c_int as isize) =
            (b6 - b7) as crate::src::common::common::dctcoef;
        *dct.offset(6 as ::core::ffi::c_int as isize) =
            (b4 + b5) as crate::src::common::common::dctcoef;
        *dct.offset(7 as ::core::ffi::c_int as isize) =
            (b6 + b7) as crate::src::common::common::dctcoef;
        (*dct4x4.offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            0 as crate::src::common::common::dctcoef;
        (*dct4x4.offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            0 as crate::src::common::common::dctcoef;
        (*dct4x4.offset(2 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            0 as crate::src::common::common::dctcoef;
        (*dct4x4.offset(3 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            0 as crate::src::common::common::dctcoef;
        (*dct4x4.offset(4 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            0 as crate::src::common::common::dctcoef;
        (*dct4x4.offset(5 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            0 as crate::src::common::common::dctcoef;
        (*dct4x4.offset(6 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            0 as crate::src::common::common::dctcoef;
        (*dct4x4.offset(7 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] =
            0 as crate::src::common::common::dctcoef;
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
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < i_size {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < i_size {
                *diff.offset((x + y * i_size) as isize) = (*pix1.offset(x as isize)
                    as ::core::ffi::c_int
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
        let mut d: [crate::src::common::common::dctcoef; 16] = [0; 16];
        let mut tmp: [crate::src::common::common::dctcoef; 16] = [0; 16];
        pixel_sub_wxh(
            &raw mut d as *mut crate::src::common::common::dctcoef,
            4 as ::core::ffi::c_int,
            pix1,
            crate::src::common::common::FENC_STRIDE,
            pix2,
            crate::src::common::common::FDEC_STRIDE,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let mut s03: ::core::ffi::c_int = d
                [(i * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + d[(i * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut s12: ::core::ffi::c_int = d
                [(i * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + d[(i * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut d03: ::core::ffi::c_int = d
                [(i * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - d[(i * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut d12: ::core::ffi::c_int = d
                [(i * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - d[(i * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            tmp[(0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (s03 + s12) as crate::src::common::common::dctcoef;
            tmp[(1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (2 as ::core::ffi::c_int * d03 + d12) as crate::src::common::common::dctcoef;
            tmp[(2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (s03 - s12) as crate::src::common::common::dctcoef;
            tmp[(3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as usize] =
                (d03 - 2 as ::core::ffi::c_int * d12) as crate::src::common::common::dctcoef;
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 4 as ::core::ffi::c_int {
            let mut s03_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut s12_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + tmp[(i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut d03_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut d12_0: ::core::ffi::c_int = tmp
                [(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - tmp[(i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            *dct.offset((i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) =
                (s03_0 + s12_0) as crate::src::common::common::dctcoef;
            *dct.offset((i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
                (2 as ::core::ffi::c_int * d03_0 + d12_0) as crate::src::common::common::dctcoef;
            *dct.offset((i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) =
                (s03_0 - s12_0) as crate::src::common::common::dctcoef;
            *dct.offset((i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize) =
                (d03_0 - 2 as ::core::ffi::c_int * d12_0) as crate::src::common::common::dctcoef;
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
            &raw mut *dct.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
            pix1.offset(0 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
            pix2.offset(0 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
        );
        sub4x4_dct(
            &raw mut *dct.offset(1 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
            pix1.offset(4 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
            pix2.offset(4 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
        );
        sub4x4_dct(
            &raw mut *dct.offset(2 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
            pix1.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
        sub4x4_dct(
            &raw mut *dct.offset(3 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
            pix1.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
    }
}

unsafe extern "C" fn sub16x16_dct(
    mut dct: *mut [crate::src::common::common::dctcoef; 16],
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) {
    unsafe {
        sub8x8_dct(
            dct.offset(0 as ::core::ffi::c_int as isize)
                as *mut [crate::src::common::common::dctcoef; 16],
            pix1.offset(0 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
            pix2.offset(0 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
        );
        sub8x8_dct(
            dct.offset(4 as ::core::ffi::c_int as isize)
                as *mut [crate::src::common::common::dctcoef; 16],
            pix1.offset(8 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
            pix2.offset(8 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
        );
        sub8x8_dct(
            dct.offset(8 as ::core::ffi::c_int as isize)
                as *mut [crate::src::common::common::dctcoef; 16],
            pix1.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
        sub8x8_dct(
            dct.offset(12 as ::core::ffi::c_int as isize)
                as *mut [crate::src::common::common::dctcoef; 16],
            pix1.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
    }
}

unsafe extern "C" fn sub4x4_dct_dc(
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) -> ::core::ffi::c_int {
    unsafe {
        let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            sum += *pix1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *pix1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *pix1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + *pix1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - *pix2.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            i += 1;
            pix1 = pix1.offset(crate::src::common::common::FENC_STRIDE as isize);
            pix2 = pix2.offset(crate::src::common::common::FDEC_STRIDE as isize);
        }
        return sum;
    }
}

unsafe extern "C" fn sub8x8_dct_dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) {
    unsafe {
        *dct.offset(0 as ::core::ffi::c_int as isize) = sub4x4_dct_dc(
            pix1.offset(0 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
            pix2.offset(0 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
        )
            as crate::src::common::common::dctcoef;
        *dct.offset(1 as ::core::ffi::c_int as isize) = sub4x4_dct_dc(
            pix1.offset(4 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
            pix2.offset(4 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
        )
            as crate::src::common::common::dctcoef;
        *dct.offset(2 as ::core::ffi::c_int as isize) = sub4x4_dct_dc(
            pix1.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        )
            as crate::src::common::common::dctcoef;
        *dct.offset(3 as ::core::ffi::c_int as isize) = sub4x4_dct_dc(
            pix1.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        )
            as crate::src::common::common::dctcoef;
        let mut d0: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut d1: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut d2: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut d3: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            - *dct.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *dct.offset(0 as ::core::ffi::c_int as isize) =
            (d0 + d1) as crate::src::common::common::dctcoef;
        *dct.offset(1 as ::core::ffi::c_int as isize) =
            (d0 - d1) as crate::src::common::common::dctcoef;
        *dct.offset(2 as ::core::ffi::c_int as isize) =
            (d2 + d3) as crate::src::common::common::dctcoef;
        *dct.offset(3 as ::core::ffi::c_int as isize) =
            (d2 - d3) as crate::src::common::common::dctcoef;
    }
}

unsafe extern "C" fn sub8x16_dct_dc(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut a0: ::core::ffi::c_int = sub4x4_dct_dc(
            pix1.offset(
                (0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
        let mut a1: ::core::ffi::c_int = sub4x4_dct_dc(
            pix1.offset(
                (0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
        let mut a2: ::core::ffi::c_int = sub4x4_dct_dc(
            pix1.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
        let mut a3: ::core::ffi::c_int = sub4x4_dct_dc(
            pix1.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
        let mut a4: ::core::ffi::c_int = sub4x4_dct_dc(
            pix1.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
        let mut a5: ::core::ffi::c_int = sub4x4_dct_dc(
            pix1.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
        let mut a6: ::core::ffi::c_int = sub4x4_dct_dc(
            pix1.offset(
                (12 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
        let mut a7: ::core::ffi::c_int = sub4x4_dct_dc(
            pix1.offset(
                (12 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (12 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
        let mut b0: ::core::ffi::c_int = a0 + a1;
        let mut b1: ::core::ffi::c_int = a2 + a3;
        let mut b2: ::core::ffi::c_int = a4 + a5;
        let mut b3: ::core::ffi::c_int = a6 + a7;
        let mut b4: ::core::ffi::c_int = a0 - a1;
        let mut b5: ::core::ffi::c_int = a2 - a3;
        let mut b6: ::core::ffi::c_int = a4 - a5;
        let mut b7: ::core::ffi::c_int = a6 - a7;
        a0 = b0 + b1;
        a1 = b2 + b3;
        a2 = b4 + b5;
        a3 = b6 + b7;
        a4 = b0 - b1;
        a5 = b2 - b3;
        a6 = b4 - b5;
        a7 = b6 - b7;
        *dct.offset(0 as ::core::ffi::c_int as isize) =
            (a0 + a1) as crate::src::common::common::dctcoef;
        *dct.offset(1 as ::core::ffi::c_int as isize) =
            (a2 + a3) as crate::src::common::common::dctcoef;
        *dct.offset(2 as ::core::ffi::c_int as isize) =
            (a0 - a1) as crate::src::common::common::dctcoef;
        *dct.offset(3 as ::core::ffi::c_int as isize) =
            (a2 - a3) as crate::src::common::common::dctcoef;
        *dct.offset(4 as ::core::ffi::c_int as isize) =
            (a4 - a5) as crate::src::common::common::dctcoef;
        *dct.offset(5 as ::core::ffi::c_int as isize) =
            (a6 - a7) as crate::src::common::common::dctcoef;
        *dct.offset(6 as ::core::ffi::c_int as isize) =
            (a4 + a5) as crate::src::common::common::dctcoef;
        *dct.offset(7 as ::core::ffi::c_int as isize) =
            (a6 + a7) as crate::src::common::common::dctcoef;
    }
}

unsafe extern "C" fn add4x4_idct(
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        let mut d: [crate::src::common::common::dctcoef; 16] = [0; 16];
        let mut tmp: [crate::src::common::common::dctcoef; 16] = [0; 16];
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let mut s02: ::core::ffi::c_int = *dct
                .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int
                + *dct.offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int;
            let mut d02: ::core::ffi::c_int = *dct
                .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int
                - *dct.offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int;
            let mut s13: ::core::ffi::c_int = *dct
                .offset((1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int
                + (*dct.offset((3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int);
            let mut d13: ::core::ffi::c_int = (*dct
                .offset((1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                - *dct.offset((3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int;
            tmp[(i * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize] =
                (s02 + s13) as crate::src::common::common::dctcoef;
            tmp[(i * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] =
                (d02 + d13) as crate::src::common::common::dctcoef;
            tmp[(i * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] =
                (d02 - d13) as crate::src::common::common::dctcoef;
            tmp[(i * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] =
                (s02 - s13) as crate::src::common::common::dctcoef;
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 4 as ::core::ffi::c_int {
            let mut s02_0: ::core::ffi::c_int = tmp
                [(0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize]
                as ::core::ffi::c_int
                + tmp[(2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize]
                    as ::core::ffi::c_int;
            let mut d02_0: ::core::ffi::c_int = tmp
                [(0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize]
                as ::core::ffi::c_int
                - tmp[(2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize]
                    as ::core::ffi::c_int;
            let mut s13_0: ::core::ffi::c_int = tmp
                [(1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize]
                as ::core::ffi::c_int
                + (tmp[(3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize]
                    as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int);
            let mut d13_0: ::core::ffi::c_int = (tmp
                [(1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize]
                as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                - tmp[(3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize]
                    as ::core::ffi::c_int;
            d[(0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize] =
                (s02_0 + s13_0 + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
            d[(1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize] =
                (d02_0 + d13_0 + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
            d[(2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize] =
                (d02_0 - d13_0 + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
            d[(3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + i_0) as usize] =
                (s02_0 - s13_0 + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
                    as crate::src::common::common::dctcoef;
            i_0 += 1;
        }
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 4 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 4 as ::core::ffi::c_int {
                *p_dst.offset(x as isize) = x264_clip_pixel(
                    *p_dst.offset(x as isize) as ::core::ffi::c_int
                        + d[(y * 4 as ::core::ffi::c_int + x) as usize] as ::core::ffi::c_int,
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
            p_dst.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::pixel,
            &raw mut *dct.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
        );
        add4x4_idct(
            p_dst.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::pixel,
            &raw mut *dct.offset(1 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
        );
        add4x4_idct(
            p_dst.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            &raw mut *dct.offset(2 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
        );
        add4x4_idct(
            p_dst.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            &raw mut *dct.offset(3 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
        );
    }
}

unsafe extern "C" fn add16x16_idct(
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dct: *mut [crate::src::common::common::dctcoef; 16],
) {
    unsafe {
        add8x8_idct(
            p_dst.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::pixel,
            dct.offset(0 as ::core::ffi::c_int as isize)
                as *mut [crate::src::common::common::dctcoef; 16],
        );
        add8x8_idct(
            p_dst.offset(8 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::pixel,
            dct.offset(4 as ::core::ffi::c_int as isize)
                as *mut [crate::src::common::common::dctcoef; 16],
        );
        add8x8_idct(
            p_dst.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            dct.offset(8 as ::core::ffi::c_int as isize)
                as *mut [crate::src::common::common::dctcoef; 16],
        );
        add8x8_idct(
            p_dst.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            dct.offset(12 as ::core::ffi::c_int as isize)
                as *mut [crate::src::common::common::dctcoef; 16],
        );
    }
}

unsafe extern "C" fn sub8x8_dct8(
    mut dct: *mut crate::src::common::common::dctcoef,
    mut pix1: *mut crate::src::common::common::pixel,
    mut pix2: *mut crate::src::common::common::pixel,
) {
    unsafe {
        let mut tmp: [crate::src::common::common::dctcoef; 64] = [0; 64];
        pixel_sub_wxh(
            &raw mut tmp as *mut crate::src::common::common::dctcoef,
            8 as ::core::ffi::c_int,
            pix1,
            crate::src::common::common::FENC_STRIDE,
            pix2,
            crate::src::common::common::FDEC_STRIDE,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let mut s07: ::core::ffi::c_int = tmp
                [(0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                as ::core::ffi::c_int
                + tmp[(7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                    as ::core::ffi::c_int;
            let mut s16: ::core::ffi::c_int = tmp
                [(1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                as ::core::ffi::c_int
                + tmp[(6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                    as ::core::ffi::c_int;
            let mut s25: ::core::ffi::c_int = tmp
                [(2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                as ::core::ffi::c_int
                + tmp[(5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                    as ::core::ffi::c_int;
            let mut s34: ::core::ffi::c_int = tmp
                [(3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                as ::core::ffi::c_int
                + tmp[(4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                    as ::core::ffi::c_int;
            let mut a0: ::core::ffi::c_int = s07 + s34;
            let mut a1: ::core::ffi::c_int = s16 + s25;
            let mut a2: ::core::ffi::c_int = s07 - s34;
            let mut a3: ::core::ffi::c_int = s16 - s25;
            let mut d07: ::core::ffi::c_int = tmp
                [(0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                as ::core::ffi::c_int
                - tmp[(7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                    as ::core::ffi::c_int;
            let mut d16: ::core::ffi::c_int = tmp
                [(1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                as ::core::ffi::c_int
                - tmp[(6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                    as ::core::ffi::c_int;
            let mut d25: ::core::ffi::c_int = tmp
                [(2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                as ::core::ffi::c_int
                - tmp[(5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                    as ::core::ffi::c_int;
            let mut d34: ::core::ffi::c_int = tmp
                [(3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                as ::core::ffi::c_int
                - tmp[(4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize]
                    as ::core::ffi::c_int;
            let mut a4: ::core::ffi::c_int = d16 + d25 + (d07 + (d07 >> 1 as ::core::ffi::c_int));
            let mut a5: ::core::ffi::c_int = d07 - d34 - (d25 + (d25 >> 1 as ::core::ffi::c_int));
            let mut a6: ::core::ffi::c_int = d07 + d34 - (d16 + (d16 >> 1 as ::core::ffi::c_int));
            let mut a7: ::core::ffi::c_int = d16 - d25 + (d34 + (d34 >> 1 as ::core::ffi::c_int));
            tmp[(0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize] =
                (a0 + a1) as crate::src::common::common::dctcoef;
            tmp[(1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize] =
                (a4 + (a7 >> 2 as ::core::ffi::c_int)) as crate::src::common::common::dctcoef;
            tmp[(2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize] =
                (a2 + (a3 >> 1 as ::core::ffi::c_int)) as crate::src::common::common::dctcoef;
            tmp[(3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize] =
                (a5 + (a6 >> 2 as ::core::ffi::c_int)) as crate::src::common::common::dctcoef;
            tmp[(4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize] =
                (a0 - a1) as crate::src::common::common::dctcoef;
            tmp[(5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize] =
                (a6 - (a5 >> 2 as ::core::ffi::c_int)) as crate::src::common::common::dctcoef;
            tmp[(6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize] =
                ((a2 >> 1 as ::core::ffi::c_int) - a3) as crate::src::common::common::dctcoef;
            tmp[(7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as usize] =
                ((a4 >> 2 as ::core::ffi::c_int) - a7) as crate::src::common::common::dctcoef;
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 8 as ::core::ffi::c_int {
            let mut s07_0: ::core::ffi::c_int = tmp
                [(i_0 * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + tmp[(i_0 * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut s16_0: ::core::ffi::c_int = tmp
                [(i_0 * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + tmp[(i_0 * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut s25_0: ::core::ffi::c_int = tmp
                [(i_0 * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + tmp[(i_0 * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut s34_0: ::core::ffi::c_int = tmp
                [(i_0 * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + tmp[(i_0 * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut a0_0: ::core::ffi::c_int = s07_0 + s34_0;
            let mut a1_0: ::core::ffi::c_int = s16_0 + s25_0;
            let mut a2_0: ::core::ffi::c_int = s07_0 - s34_0;
            let mut a3_0: ::core::ffi::c_int = s16_0 - s25_0;
            let mut d07_0: ::core::ffi::c_int = tmp
                [(i_0 * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - tmp[(i_0 * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut d16_0: ::core::ffi::c_int = tmp
                [(i_0 * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - tmp[(i_0 * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut d25_0: ::core::ffi::c_int = tmp
                [(i_0 * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - tmp[(i_0 * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut d34_0: ::core::ffi::c_int = tmp
                [(i_0 * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - tmp[(i_0 * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int;
            let mut a4_0: ::core::ffi::c_int =
                d16_0 + d25_0 + (d07_0 + (d07_0 >> 1 as ::core::ffi::c_int));
            let mut a5_0: ::core::ffi::c_int =
                d07_0 - d34_0 - (d25_0 + (d25_0 >> 1 as ::core::ffi::c_int));
            let mut a6_0: ::core::ffi::c_int =
                d07_0 + d34_0 - (d16_0 + (d16_0 >> 1 as ::core::ffi::c_int));
            let mut a7_0: ::core::ffi::c_int =
                d16_0 - d25_0 + (d34_0 + (d34_0 >> 1 as ::core::ffi::c_int));
            *dct.offset((0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i_0) as isize) =
                (a0_0 + a1_0) as crate::src::common::common::dctcoef;
            *dct.offset((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i_0) as isize) =
                (a4_0 + (a7_0 >> 2 as ::core::ffi::c_int)) as crate::src::common::common::dctcoef;
            *dct.offset((2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i_0) as isize) =
                (a2_0 + (a3_0 >> 1 as ::core::ffi::c_int)) as crate::src::common::common::dctcoef;
            *dct.offset((3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i_0) as isize) =
                (a5_0 + (a6_0 >> 2 as ::core::ffi::c_int)) as crate::src::common::common::dctcoef;
            *dct.offset((4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i_0) as isize) =
                (a0_0 - a1_0) as crate::src::common::common::dctcoef;
            *dct.offset((5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i_0) as isize) =
                (a6_0 - (a5_0 >> 2 as ::core::ffi::c_int)) as crate::src::common::common::dctcoef;
            *dct.offset((6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i_0) as isize) =
                ((a2_0 >> 1 as ::core::ffi::c_int) - a3_0) as crate::src::common::common::dctcoef;
            *dct.offset((7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i_0) as isize) =
                ((a4_0 >> 2 as ::core::ffi::c_int) - a7_0) as crate::src::common::common::dctcoef;
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
            &raw mut *dct.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
            pix1.offset(0 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
            pix2.offset(0 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
        );
        sub8x8_dct8(
            &raw mut *dct.offset(1 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
            pix1.offset(8 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
            pix2.offset(8 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
        );
        sub8x8_dct8(
            &raw mut *dct.offset(2 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
            pix1.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
        sub8x8_dct8(
            &raw mut *dct.offset(3 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
            pix1.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE
                    + 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            pix2.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
        );
    }
}

unsafe extern "C" fn add8x8_idct8(
    mut dst: *mut crate::src::common::common::pixel,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        let ref mut c2rust_fresh0 = *dct.offset(0 as ::core::ffi::c_int as isize);
        *c2rust_fresh0 = (*c2rust_fresh0 as ::core::ffi::c_int + 32 as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let mut a0: ::core::ffi::c_int = *dct
                .offset((0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int
                + *dct.offset((4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int;
            let mut a2: ::core::ffi::c_int = *dct
                .offset((0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int
                - *dct.offset((4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int;
            let mut a4: ::core::ffi::c_int = (*dct
                .offset((2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                - *dct.offset((6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int;
            let mut a6: ::core::ffi::c_int = (*dct
                .offset((6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                + *dct.offset((2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int;
            let mut b0: ::core::ffi::c_int = a0 + a6;
            let mut b2: ::core::ffi::c_int = a2 + a4;
            let mut b4: ::core::ffi::c_int = a2 - a4;
            let mut b6: ::core::ffi::c_int = a0 - a6;
            let mut a1: ::core::ffi::c_int = -(*dct
                .offset((3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int)
                + *dct.offset((5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                - *dct.offset((7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                - (*dct.offset((7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int);
            let mut a3: ::core::ffi::c_int = *dct
                .offset((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int
                + *dct.offset((7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                - *dct.offset((3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                - (*dct.offset((3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int);
            let mut a5: ::core::ffi::c_int = -(*dct
                .offset((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int)
                + *dct.offset((7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                + *dct.offset((5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                + (*dct.offset((5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int);
            let mut a7: ::core::ffi::c_int = *dct
                .offset((3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                as ::core::ffi::c_int
                + *dct.offset((5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                + *dct.offset((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                + (*dct.offset((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize)
                    as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int);
            let mut b1: ::core::ffi::c_int = (a7 >> 2 as ::core::ffi::c_int) + a1;
            let mut b3: ::core::ffi::c_int = a3 + (a5 >> 2 as ::core::ffi::c_int);
            let mut b5: ::core::ffi::c_int = (a3 >> 2 as ::core::ffi::c_int) - a5;
            let mut b7: ::core::ffi::c_int = a7 - (a1 >> 2 as ::core::ffi::c_int);
            *dct.offset((0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) =
                (b0 + b7) as crate::src::common::common::dctcoef;
            *dct.offset((1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) =
                (b2 + b5) as crate::src::common::common::dctcoef;
            *dct.offset((2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) =
                (b4 + b3) as crate::src::common::common::dctcoef;
            *dct.offset((3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) =
                (b6 + b1) as crate::src::common::common::dctcoef;
            *dct.offset((4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) =
                (b6 - b1) as crate::src::common::common::dctcoef;
            *dct.offset((5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) =
                (b4 - b3) as crate::src::common::common::dctcoef;
            *dct.offset((6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) =
                (b2 - b5) as crate::src::common::common::dctcoef;
            *dct.offset((7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + i) as isize) =
                (b0 - b7) as crate::src::common::common::dctcoef;
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 8 as ::core::ffi::c_int {
            let mut a0_0: ::core::ffi::c_int = *dct
                .offset((i_0 * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *dct.offset((i_0 * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            let mut a2_0: ::core::ffi::c_int = *dct
                .offset((i_0 * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - *dct.offset((i_0 * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            let mut a4_0: ::core::ffi::c_int = (*dct
                .offset((i_0 * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                - *dct.offset((i_0 * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            let mut a6_0: ::core::ffi::c_int = (*dct
                .offset((i_0 * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                + *dct.offset((i_0 * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            let mut b0_0: ::core::ffi::c_int = a0_0 + a6_0;
            let mut b2_0: ::core::ffi::c_int = a2_0 + a4_0;
            let mut b4_0: ::core::ffi::c_int = a2_0 - a4_0;
            let mut b6_0: ::core::ffi::c_int = a0_0 - a6_0;
            let mut a1_0: ::core::ffi::c_int = -(*dct
                .offset((i_0 * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int)
                + *dct.offset((i_0 * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                - *dct.offset((i_0 * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                - (*dct.offset((i_0 * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int);
            let mut a3_0: ::core::ffi::c_int = *dct
                .offset((i_0 * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *dct.offset((i_0 * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                - *dct.offset((i_0 * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                - (*dct.offset((i_0 * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int);
            let mut a5_0: ::core::ffi::c_int = -(*dct
                .offset((i_0 * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int)
                + *dct.offset((i_0 * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                + *dct.offset((i_0 * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                + (*dct.offset((i_0 * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int);
            let mut a7_0: ::core::ffi::c_int = *dct
                .offset((i_0 * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *dct.offset((i_0 * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                + *dct.offset((i_0 * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                + (*dct.offset((i_0 * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int);
            let mut b1_0: ::core::ffi::c_int = (a7_0 >> 2 as ::core::ffi::c_int) + a1_0;
            let mut b3_0: ::core::ffi::c_int = a3_0 + (a5_0 >> 2 as ::core::ffi::c_int);
            let mut b5_0: ::core::ffi::c_int = (a3_0 >> 2 as ::core::ffi::c_int) - a5_0;
            let mut b7_0: ::core::ffi::c_int = a7_0 - (a1_0 >> 2 as ::core::ffi::c_int);
            *dst.offset(
                (i_0 + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE) as isize,
            ) = x264_clip_pixel(
                *dst.offset(
                    (i_0 + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    + (b0_0 + b7_0 >> 6 as ::core::ffi::c_int),
            );
            *dst.offset(
                (i_0 + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE) as isize,
            ) = x264_clip_pixel(
                *dst.offset(
                    (i_0 + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    + (b2_0 + b5_0 >> 6 as ::core::ffi::c_int),
            );
            *dst.offset(
                (i_0 + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE) as isize,
            ) = x264_clip_pixel(
                *dst.offset(
                    (i_0 + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    + (b4_0 + b3_0 >> 6 as ::core::ffi::c_int),
            );
            *dst.offset(
                (i_0 + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE) as isize,
            ) = x264_clip_pixel(
                *dst.offset(
                    (i_0 + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    + (b6_0 + b1_0 >> 6 as ::core::ffi::c_int),
            );
            *dst.offset(
                (i_0 + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE) as isize,
            ) = x264_clip_pixel(
                *dst.offset(
                    (i_0 + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    + (b6_0 - b1_0 >> 6 as ::core::ffi::c_int),
            );
            *dst.offset(
                (i_0 + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE) as isize,
            ) = x264_clip_pixel(
                *dst.offset(
                    (i_0 + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    + (b4_0 - b3_0 >> 6 as ::core::ffi::c_int),
            );
            *dst.offset(
                (i_0 + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE) as isize,
            ) = x264_clip_pixel(
                *dst.offset(
                    (i_0 + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    + (b2_0 - b5_0 >> 6 as ::core::ffi::c_int),
            );
            *dst.offset(
                (i_0 + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE) as isize,
            ) = x264_clip_pixel(
                *dst.offset(
                    (i_0 + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE)
                        as isize,
                ) as ::core::ffi::c_int
                    + (b0_0 - b7_0 >> 6 as ::core::ffi::c_int),
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
            dst.offset(0 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
            &raw mut *dct.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
        );
        add8x8_idct8(
            dst.offset(8 as ::core::ffi::c_int as isize) as *mut crate::src::common::common::pixel,
            &raw mut *dct.offset(1 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
        );
        add8x8_idct8(
            dst.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            &raw mut *dct.offset(2 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
        );
        add8x8_idct8(
            dst.offset(
                (8 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 8 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            &raw mut *dct.offset(3 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::dctcoef,
        );
    }
}
#[inline]

unsafe extern "C" fn add4x4_idct_dc(
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dc: crate::src::common::common::dctcoef,
) {
    unsafe {
        dc = (dc as ::core::ffi::c_int + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            *p_dst.offset(0 as ::core::ffi::c_int as isize) = x264_clip_pixel(
                *p_dst.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + dc as ::core::ffi::c_int,
            );
            *p_dst.offset(1 as ::core::ffi::c_int as isize) = x264_clip_pixel(
                *p_dst.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + dc as ::core::ffi::c_int,
            );
            *p_dst.offset(2 as ::core::ffi::c_int as isize) = x264_clip_pixel(
                *p_dst.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + dc as ::core::ffi::c_int,
            );
            *p_dst.offset(3 as ::core::ffi::c_int as isize) = x264_clip_pixel(
                *p_dst.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + dc as ::core::ffi::c_int,
            );
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
        add4x4_idct_dc(
            p_dst.offset(0 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::pixel,
            *dct.offset(0 as ::core::ffi::c_int as isize),
        );
        add4x4_idct_dc(
            p_dst.offset(4 as ::core::ffi::c_int as isize)
                as *mut crate::src::common::common::pixel,
            *dct.offset(1 as ::core::ffi::c_int as isize),
        );
        add4x4_idct_dc(
            p_dst.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 0 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            *dct.offset(2 as ::core::ffi::c_int as isize),
        );
        add4x4_idct_dc(
            p_dst.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE
                    + 4 as ::core::ffi::c_int) as isize,
            ) as *mut crate::src::common::common::pixel,
            *dct.offset(3 as ::core::ffi::c_int as isize),
        );
    }
}

unsafe extern "C" fn add16x16_idct_dc(
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            add4x4_idct_dc(
                p_dst.offset(0 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::common::pixel,
                *dct.offset(0 as ::core::ffi::c_int as isize),
            );
            add4x4_idct_dc(
                p_dst.offset(4 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::common::pixel,
                *dct.offset(1 as ::core::ffi::c_int as isize),
            );
            add4x4_idct_dc(
                p_dst.offset(8 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::common::pixel,
                *dct.offset(2 as ::core::ffi::c_int as isize),
            );
            add4x4_idct_dc(
                p_dst.offset(12 as ::core::ffi::c_int as isize)
                    as *mut crate::src::common::common::pixel,
                *dct.offset(3 as ::core::ffi::c_int as isize),
            );
            i += 1;
            dct = dct.offset(4 as ::core::ffi::c_int as isize);
            p_dst = p_dst.offset(
                (4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE) as isize,
            );
        }
    }
}
#[no_mangle]

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
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
            >;
        (*dctf).add4x4_idct = Some(
            add4x4_idct
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
            >;
        (*dctf).sub8x8_dct = Some(
            sub8x8_dct
                as unsafe extern "C" fn(
                    *mut [crate::src::common::common::dctcoef; 16],
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut [crate::src::common::common::dctcoef; 16],
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
            >;
        (*dctf).sub8x8_dct_dc = Some(
            sub8x8_dct_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
            >;
        (*dctf).add8x8_idct = Some(
            add8x8_idct
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut [crate::src::common::common::dctcoef; 16],
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut [crate::src::common::common::dctcoef; 16],
                ) -> (),
            >;
        (*dctf).add8x8_idct_dc = Some(
            add8x8_idct_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
            >;
        (*dctf).sub8x16_dct_dc = Some(
            sub8x16_dct_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
            >;
        (*dctf).sub16x16_dct = Some(
            sub16x16_dct
                as unsafe extern "C" fn(
                    *mut [crate::src::common::common::dctcoef; 16],
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut [crate::src::common::common::dctcoef; 16],
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
            >;
        (*dctf).add16x16_idct = Some(
            add16x16_idct
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut [crate::src::common::common::dctcoef; 16],
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut [crate::src::common::common::dctcoef; 16],
                ) -> (),
            >;
        (*dctf).add16x16_idct_dc = Some(
            add16x16_idct_dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
            >;
        (*dctf).sub8x8_dct8 = Some(
            sub8x8_dct8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
            >;
        (*dctf).add8x8_idct8 = Some(
            add8x8_idct8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
            >;
        (*dctf).sub16x16_dct8 = Some(
            sub16x16_dct8
                as unsafe extern "C" fn(
                    *mut [crate::src::common::common::dctcoef; 64],
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut [crate::src::common::common::dctcoef; 64],
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> (),
            >;
        (*dctf).add16x16_idct8 = Some(
            add16x16_idct8
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut [crate::src::common::common::dctcoef; 64],
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::pixel,
                    *mut [crate::src::common::common::dctcoef; 64],
                ) -> (),
            >;
        (*dctf).dct4x4dc =
            Some(dct4x4dc as unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ())
                as Option<unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ()>;
        (*dctf).idct4x4dc =
            Some(idct4x4dc as unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ())
                as Option<unsafe extern "C" fn(*mut crate::src::common::common::dctcoef) -> ()>;
        (*dctf).dct2x4dc = Some(
            dct2x4dc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [crate::src::common::common::dctcoef; 16],
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut [crate::src::common::common::dctcoef; 16],
                ) -> (),
            >;
    }
}

unsafe extern "C" fn zigzag_scan_8x8_frame(
    mut level: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        *level.offset(0 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(1 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(2 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(3 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(4 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(5 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(6 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(7 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(8 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(9 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(10 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(11 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(12 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(13 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(14 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(15 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(16 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(17 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(18 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(19 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(20 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(21 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(22 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(23 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(24 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(25 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(26 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(27 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(28 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(29 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(30 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(31 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(32 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(33 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(34 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(35 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(36 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(37 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(38 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(39 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(40 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(41 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(42 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(43 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(44 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(45 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(46 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(47 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(48 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(49 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(50 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(51 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(52 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(53 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(54 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(55 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(56 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(57 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(58 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(59 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(60 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(61 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(62 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(63 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
    }
}

unsafe extern "C" fn zigzag_scan_8x8_field(
    mut level: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        *level.offset(0 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(1 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(2 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(3 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(4 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(5 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(6 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(7 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(8 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(9 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(10 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(11 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(12 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(13 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(14 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(15 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(16 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(17 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(18 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(19 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(20 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(21 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(22 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(23 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(24 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(25 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(26 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(27 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(28 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(29 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(30 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(31 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(32 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(33 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(34 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(35 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(36 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(37 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(38 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(39 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(40 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(41 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(42 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(43 as ::core::ffi::c_int as isize) = *dct.offset(
            (4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(44 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(45 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(46 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(47 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(48 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(49 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(50 as ::core::ffi::c_int as isize) = *dct.offset(
            (5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(51 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(52 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(53 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(54 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(55 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(56 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(57 as ::core::ffi::c_int as isize) = *dct.offset(
            (6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
        *level.offset(58 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(59 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(60 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
        );
        *level.offset(61 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
        );
        *level.offset(62 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 6 as ::core::ffi::c_int) as isize,
        );
        *level.offset(63 as ::core::ffi::c_int as isize) = *dct.offset(
            (7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) as isize,
        );
    }
}

unsafe extern "C" fn zigzag_scan_4x4_frame(
    mut level: *mut crate::src::common::common::dctcoef,
    mut dct: *mut crate::src::common::common::dctcoef,
) {
    unsafe {
        *level.offset(0 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(1 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(2 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(3 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(4 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(5 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(6 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(7 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(8 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(9 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(10 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(11 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(12 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        *level.offset(13 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(14 as ::core::ffi::c_int as isize) = *dct.offset(
            (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(15 as ::core::ffi::c_int as isize) = *dct.offset(
            (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
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
            (2 as crate::__stddef_size_t_h::size_t).wrapping_mul(::core::mem::size_of::<
                crate::src::common::common::dctcoef,
            >()
                as crate::__stddef_size_t_h::size_t),
        );
        *level.offset(2 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
        );
        *level.offset(3 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize,
        );
        *level.offset(4 as ::core::ffi::c_int as isize) = *dct.offset(
            (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize,
        );
        *level.offset(5 as ::core::ffi::c_int as isize) = *dct.offset(
            (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize,
        );
        crate::stdlib::memcpy(
            level.offset(6 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            dct.offset(6 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (10 as crate::__stddef_size_t_h::size_t).wrapping_mul(::core::mem::size_of::<
                crate::src::common::common::dctcoef,
            >()
                as crate::__stddef_size_t_h::size_t),
        );
    }
}

unsafe extern "C" fn zigzag_sub_4x4_frame(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut oe: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(0 as ::core::ffi::c_int as isize) = (*p_src.offset(oe as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_0 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_1 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_2 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_3: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_3: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_3 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_4: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_4: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_4 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_5: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_5: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_5 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_6: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_6: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_6 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_7: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_7: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_7 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_8 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_9: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_9: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_9 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_10: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_10: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_10 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_11: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_11: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_11 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_12: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_12: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_12 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_13: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_13: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_13 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_14: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_14: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_14 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        return (nz != 0) as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn zigzag_sub_4x4_field(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut oe: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(0 as ::core::ffi::c_int as isize) = (*p_src.offset(oe as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_0 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_1 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_2 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_3 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_4: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_4: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_4 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_5: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_5: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_5 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_6: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_6: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_6 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_7: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_7: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_7 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_8: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_8: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_8 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_9: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_9: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_9 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_10: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_10: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_10 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_11: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_11: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_11 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_12: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_12: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_12 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_13: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_13: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_13 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_14: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_14: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_14 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        return (nz != 0) as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn zigzag_sub_4x4ac_frame(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dc: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut oe: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *dc = (*p_src.offset(oe as isize) as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        *level.offset(0 as ::core::ffi::c_int as isize) = 0 as crate::src::common::common::dctcoef;
        let mut oe_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_0 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_1 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_2 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_3: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_3: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_3 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_4: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_4: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_4 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_5: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_5: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_5 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_6: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_6: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_6 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_7: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_7: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_7 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_8 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_9: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_9: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_9 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_10: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_10: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_10 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_11: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_11: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_11 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_12: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_12: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_12 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_13: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_13: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_13 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_14: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_14: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_14 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        return (nz != 0) as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn zigzag_sub_4x4ac_field(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
    mut dc: *mut crate::src::common::common::dctcoef,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut oe: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *dc = (*p_src.offset(oe as isize) as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        *level.offset(0 as ::core::ffi::c_int as isize) = 0 as crate::src::common::common::dctcoef;
        let mut oe_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_0 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_1 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_2 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_3 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_4: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_4: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_4 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_5: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_5: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_5 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_6: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_6: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_6 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_7: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_7: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_7 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_8: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_8: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_8 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_9: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_9: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_9 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_10: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_10: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_10 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_11: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_11: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_11 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_12: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_12: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_12 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_13: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_13: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_13 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_14: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_14: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_14 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        return (nz != 0) as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn zigzag_sub_8x8_frame(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut oe: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(0 as ::core::ffi::c_int as isize) = (*p_src.offset(oe as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_0 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_1 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_2 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_3: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_3: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_3 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_4: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_4: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_4 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_5: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_5: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_5 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_6: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_6: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_6 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_7: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_7: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_7 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_8 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_9 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_10: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_10: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_10 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_11: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_11: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_11 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_12: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_12: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_12 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_13: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_13: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_13 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_14: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_14: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_14 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_15: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_15: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(16 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_15 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_15 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(16 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_16: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_16: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(17 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_16 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_16 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(17 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_17: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_17: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(18 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_17 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_17 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(18 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_18: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_18: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(19 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_18 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_18 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(19 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_19: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_19: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(20 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_19 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_19 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(20 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_20: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_20: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(21 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_20 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_20 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(21 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_21: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_21: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(22 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_21 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_21 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(22 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_22: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_22: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(23 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_22 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_22 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(23 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_23: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_23: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(24 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_23 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_23 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(24 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_24: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_24: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(25 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_24 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_24 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(25 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_25: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_25: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(26 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_25 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_25 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(26 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_26: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_26: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(27 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_26 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_26 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(27 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_27: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_27: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(28 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_27 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_27 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(28 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_28: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_28: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(29 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_28 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_28 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(29 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_29: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_29: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(30 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_29 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_29 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(30 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_30: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_30: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(31 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_30 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_30 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(31 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_31: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_31: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(32 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_31 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_31 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(32 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_32: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_32: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(33 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_32 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_32 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(33 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_33: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_33: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(34 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_33 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_33 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(34 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_34: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_34: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(35 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_34 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_34 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(35 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_35: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_35: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(36 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_35 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_35 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(36 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_36: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_36: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(37 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_36 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_36 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(37 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_37: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_37: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(38 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_37 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_37 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(38 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_38: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_38: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(39 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_38 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_38 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(39 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_39: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_39: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(40 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_39 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_39 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(40 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_40: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_40: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(41 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_40 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_40 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(41 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_41: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_41: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(42 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_41 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_41 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(42 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_42: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_42: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(43 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_42 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_42 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(43 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_43: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_43: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(44 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_43 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_43 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(44 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_44: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_44: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(45 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_44 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_44 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(45 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_45: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_45: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(46 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_45 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_45 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(46 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_46: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_46: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(47 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_46 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_46 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(47 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_47: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_47: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(48 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_47 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_47 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(48 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_48: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_48: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(49 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_48 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_48 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(49 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_49: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_49: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(50 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_49 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_49 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(50 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_50: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_50: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(51 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_50 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_50 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(51 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_51: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_51: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(52 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_51 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_51 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(52 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_52: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_52: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(53 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_52 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_52 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(53 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_53: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_53: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(54 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_53 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_53 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(54 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_54: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_54: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(55 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_54 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_54 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(55 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_55: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_55: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(56 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_55 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_55 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(56 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_56: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_56: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(57 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_56 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_56 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(57 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_57: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_57: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(58 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_57 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_57 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(58 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_58: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_58: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(59 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_58 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_58 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(59 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_59: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_59: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(60 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_59 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_59 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(60 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_60: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_60: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(61 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_60 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_60 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(61 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_61: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_61: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(62 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_61 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_61 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(62 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_62: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_62: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(63 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_62 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_62 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(63 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((5 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((5 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((6 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((6 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((7 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((7 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        return (nz != 0) as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn zigzag_sub_8x8_field(
    mut level: *mut crate::src::common::common::dctcoef,
    mut p_src: *const crate::src::common::common::pixel,
    mut p_dst: *mut crate::src::common::common::pixel,
) -> ::core::ffi::c_int {
    unsafe {
        let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut oe: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(0 as ::core::ffi::c_int as isize) = (*p_src.offset(oe as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(1 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_0 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_0 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(2 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_1 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_1 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_2: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_2: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(3 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_2 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_2 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_3: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_3: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(4 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_3 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_3 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(5 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_4 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_4 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(6 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_5 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_5 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_6: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_6: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(7 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_6 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_6 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_7: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_7: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(8 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_7 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_7 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_8: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_8: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(9 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_8 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_8 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(10 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_9 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_9 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_10: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_10: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(11 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_10 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_10 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_11: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_11: ::core::ffi::c_int = 0 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(12 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_11 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_11 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_12: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_12: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(13 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_12 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_12 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_13: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_13: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(14 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_13 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_13 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_14: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_14: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(15 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_14 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_14 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_15: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_15: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(16 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_15 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_15 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(16 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_16: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_16: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(17 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_16 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_16 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(17 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_17: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_17: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(18 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_17 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_17 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(18 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_18: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_18: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(19 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_18 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_18 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(19 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_19: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_19: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(20 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_19 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_19 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(20 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_20: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_20: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(21 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_20 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_20 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(21 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_21: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_21: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(22 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_21 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_21 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(22 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_22: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_22: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(23 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_22 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_22 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(23 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_23: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_23: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(24 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_23 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_23 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(24 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_24: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_24: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(25 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_24 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_24 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(25 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_25: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_25: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(26 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_25 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_25 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(26 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_26: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_26: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(27 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_26 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_26 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(27 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_27: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_27: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(28 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_27 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_27 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(28 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_28: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_28: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(29 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_28 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_28 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(29 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_29: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_29: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(30 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_29 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_29 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(30 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_30: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_30: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(31 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_30 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_30 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(31 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_31: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_31: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(32 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_31 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_31 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(32 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_32: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_32: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(33 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_32 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_32 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(33 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_33: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_33: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(34 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_33 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_33 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(34 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_34: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_34: ::core::ffi::c_int = 3 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(35 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_34 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_34 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(35 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_35: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_35: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(36 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_35 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_35 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(36 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_36: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_36: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(37 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_36 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_36 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(37 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_37: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_37: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(38 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_37 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_37 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(38 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_38: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_38: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(39 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_38 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_38 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(39 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_39: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_39: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(40 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_39 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_39 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(40 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_40: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_40: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(41 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_40 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_40 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(41 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_41: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_41: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(42 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_41 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_41 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(42 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_42: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_42: ::core::ffi::c_int = 4 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(43 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_42 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_42 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(43 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_43: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_43: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(44 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_43 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_43 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(44 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_44: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_44: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(45 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_44 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_44 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(45 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_45: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_45: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(46 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_45 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_45 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(46 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_46: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_46: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(47 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_46 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_46 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(47 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_47: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_47: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(48 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_47 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_47 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(48 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_48: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_48: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(49 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_48 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_48 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(49 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_49: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_49: ::core::ffi::c_int = 5 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(50 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_49 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_49 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(50 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_50: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_50: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(51 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_50 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_50 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(51 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_51: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_51: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(52 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_51 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_51 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(52 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_52: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_52: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(53 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_52 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_52 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(53 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_53: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_53: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(54 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_53 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_53 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(54 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_54: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_54: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(55 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_54 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_54 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(55 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_55: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_55: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(56 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_55 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_55 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(56 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_56: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_56: ::core::ffi::c_int = 6 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(57 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_56 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_56 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(57 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_57: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_57: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(58 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_57 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_57 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(58 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_58: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_58: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(59 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_58 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_58 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(59 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_59: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_59: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(60 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_59 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_59 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(60 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_60: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_60: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 5 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(61 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_60 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_60 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(61 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_61: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_61: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(62 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_61 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_61 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(62 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut oe_62: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FENC_STRIDE;
        let mut od_62: ::core::ffi::c_int = 7 as ::core::ffi::c_int
            + 7 as ::core::ffi::c_int * crate::src::common::common::FDEC_STRIDE;
        *level.offset(63 as ::core::ffi::c_int as isize) = (*p_src.offset(oe_62 as isize)
            as ::core::ffi::c_int
            - *p_dst.offset(od_62 as isize) as ::core::ffi::c_int)
            as crate::src::common::common::dctcoef;
        nz |= *level.offset(63 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        (*(p_dst.offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((0 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((1 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((2 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((3 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((4 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((5 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((5 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((5 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((6 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((6 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((6 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst.offset((7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src.offset((7 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        (*(p_dst
            .offset((7 as ::core::ffi::c_int * 32 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i = (*(p_src
            .offset((7 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as isize)
            .offset(4 as ::core::ffi::c_int as isize)
            as *mut crate::src::common::base::x264_union32_t))
            .i;
        return (nz != 0) as ::core::ffi::c_int;
    }
}

unsafe extern "C" fn zigzag_interleave_8x8_cavlc(
    mut dst: *mut crate::src::common::common::dctcoef,
    mut src: *mut crate::src::common::common::dctcoef,
    mut nnz: *mut crate::stdlib::uint8_t,
) {
    unsafe {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let mut nz: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < 16 as ::core::ffi::c_int {
                nz |= *src.offset((i + j * 4 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
                *dst.offset((i * 16 as ::core::ffi::c_int + j) as isize) =
                    *src.offset((i + j * 4 as ::core::ffi::c_int) as isize);
                j += 1;
            }
            *nnz.offset(
                ((i & 1 as ::core::ffi::c_int)
                    + (i >> 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                    as isize,
            ) = (nz != 0) as ::core::ffi::c_int as crate::stdlib::uint8_t;
            i += 1;
        }
    }
}
#[no_mangle]

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
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
            >;
        (*pf_progressive).scan_8x8 = Some(
            zigzag_scan_8x8_frame
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
            >;
        (*pf_interlaced).scan_4x4 = Some(
            zigzag_scan_4x4_field
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
            >;
        (*pf_progressive).scan_4x4 = Some(
            zigzag_scan_4x4_frame
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                ) -> (),
            >;
        (*pf_interlaced).sub_8x8 = Some(
            zigzag_sub_8x8_field
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
            >;
        (*pf_progressive).sub_8x8 = Some(
            zigzag_sub_8x8_frame
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
            >;
        (*pf_interlaced).sub_4x4 = Some(
            zigzag_sub_4x4_field
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
            >;
        (*pf_progressive).sub_4x4 = Some(
            zigzag_sub_4x4_frame
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                ) -> ::core::ffi::c_int,
            >;
        (*pf_interlaced).sub_4x4ac = Some(
            zigzag_sub_4x4ac_field
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf_progressive).sub_4x4ac = Some(
            zigzag_sub_4x4ac_frame
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *const crate::src::common::common::pixel,
                    *mut crate::src::common::common::pixel,
                    *mut crate::src::common::common::dctcoef,
                ) -> ::core::ffi::c_int,
            >;
        (*pf_progressive).interleave_8x8_cavlc = Some(
            zigzag_interleave_8x8_cavlc
                as unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::stdlib::uint8_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::src::common::common::dctcoef,
                    *mut crate::stdlib::uint8_t,
                ) -> (),
            >;
        (*pf_interlaced).interleave_8x8_cavlc = (*pf_progressive).interleave_8x8_cavlc;
    }
}
