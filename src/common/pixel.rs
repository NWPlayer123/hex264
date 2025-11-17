use core::ffi::{c_double, c_float, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::common_h::{pixel, x264_t, FDEC_STRIDE, FENC_STRIDE, PIXEL_MAX};
use crate::pixel_h::{
    x264_pixel_cmp_t, x264_pixel_cmp_x3_t, x264_pixel_cmp_x4_t, x264_pixel_function_t, PIXEL_16x16,
    PIXEL_16x8, PIXEL_4x16, PIXEL_4x4, PIXEL_4x8, PIXEL_8x16, PIXEL_8x4, PIXEL_8x8,
};
use crate::predict_h::{
    x264_10_predict_16x16_dc_c, x264_10_predict_16x16_h_c, x264_10_predict_16x16_v_c,
    x264_10_predict_4x4_dc_c, x264_10_predict_4x4_h_c, x264_10_predict_4x4_v_c,
    x264_10_predict_8x16c_dc_c, x264_10_predict_8x16c_h_c, x264_10_predict_8x16c_v_c,
    x264_10_predict_8x8_dc_c, x264_10_predict_8x8_h_c, x264_10_predict_8x8_v_c,
    x264_10_predict_8x8c_dc_c, x264_10_predict_8x8c_h_c, x264_10_predict_8x8c_v_c,
};
use crate::stdint_h::intptr_t;
use crate::stdint_intn_h::{int16_t, int64_t};
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint64_t};
use crate::stdlib_h::abs;
use crate::string_h::memset;
#[c2rust::src_loc = "235:5"]
type sum2_t = uint64_t;
#[c2rust::src_loc = "234:5"]
type sum_t = uint32_t;
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn x264_pixel_sad_16x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 16 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 16 as c_int {
            i_sum += abs(*pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int);
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn x264_pixel_sad_16x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 8 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 16 as c_int {
            i_sum += abs(*pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int);
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn x264_pixel_sad_8x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 16 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 8 as c_int {
            i_sum += abs(*pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int);
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn x264_pixel_sad_8x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 8 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 8 as c_int {
            i_sum += abs(*pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int);
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "77:1"]
unsafe extern "C" fn x264_pixel_sad_8x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 4 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 8 as c_int {
            i_sum += abs(*pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int);
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "78:1"]
unsafe extern "C" fn x264_pixel_sad_4x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 16 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 4 as c_int {
            i_sum += abs(*pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int);
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn x264_pixel_sad_4x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 8 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 4 as c_int {
            i_sum += abs(*pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int);
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "80:1"]
unsafe extern "C" fn x264_pixel_sad_4x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 4 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 4 as c_int {
            i_sum += abs(*pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int);
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "103:1"]
unsafe extern "C" fn x264_pixel_ssd_16x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 16 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 16 as c_int {
            let mut d: c_int =
                *pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn x264_pixel_ssd_16x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 8 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 16 as c_int {
            let mut d: c_int =
                *pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "105:1"]
unsafe extern "C" fn x264_pixel_ssd_8x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 16 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 8 as c_int {
            let mut d: c_int =
                *pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn x264_pixel_ssd_8x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 8 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 8 as c_int {
            let mut d: c_int =
                *pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn x264_pixel_ssd_8x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 4 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 8 as c_int {
            let mut d: c_int =
                *pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "108:1"]
unsafe extern "C" fn x264_pixel_ssd_4x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 16 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 4 as c_int {
            let mut d: c_int =
                *pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "109:1"]
unsafe extern "C" fn x264_pixel_ssd_4x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 8 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 4 as c_int {
            let mut d: c_int =
                *pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn x264_pixel_ssd_4x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> c_int {
    let mut i_sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 4 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 4 as c_int {
            let mut d: c_int =
                *pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[no_mangle]
#[c2rust::src_loc = "112:1"]
unsafe extern "C" fn x264_10_pixel_ssd_wxh(
    mut pf: *mut x264_pixel_function_t,
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
    mut i_width: c_int,
    mut i_height: c_int,
) -> uint64_t {
    let mut i_ssd: uint64_t = 0 as uint64_t;
    let mut y: c_int = 0;
    let mut align: c_int =
        ((pix1 as intptr_t | pix2 as intptr_t | i_pix1 | i_pix2) & 15 as intptr_t == 0) as c_int;
    y = 0 as c_int;
    while y < i_height - 15 as c_int {
        let mut x: c_int = 0 as c_int;
        if align != 0 {
            while x < i_width - 15 as c_int {
                i_ssd = i_ssd.wrapping_add((*pf).ssd[PIXEL_16x16 as c_int as usize]
                    .expect("non-null function pointer")(
                    pix1.offset((y as intptr_t * i_pix1) as isize)
                        .offset(x as isize),
                    i_pix1,
                    pix2.offset((y as intptr_t * i_pix2) as isize)
                        .offset(x as isize),
                    i_pix2,
                ) as uint64_t);
                x += 16 as c_int;
            }
        }
        while x < i_width - 7 as c_int {
            i_ssd = i_ssd.wrapping_add((*pf).ssd[PIXEL_8x16 as c_int as usize]
                .expect("non-null function pointer")(
                pix1.offset((y as intptr_t * i_pix1) as isize)
                    .offset(x as isize),
                i_pix1,
                pix2.offset((y as intptr_t * i_pix2) as isize)
                    .offset(x as isize),
                i_pix2,
            ) as uint64_t);
            x += 8 as c_int;
        }
        y += 16 as c_int;
    }
    if y < i_height - 7 as c_int {
        let mut x_0: c_int = 0 as c_int;
        while x_0 < i_width - 7 as c_int {
            i_ssd = i_ssd.wrapping_add((*pf).ssd[PIXEL_8x8 as c_int as usize]
                .expect("non-null function pointer")(
                pix1.offset((y as intptr_t * i_pix1) as isize)
                    .offset(x_0 as isize),
                i_pix1,
                pix2.offset((y as intptr_t * i_pix2) as isize)
                    .offset(x_0 as isize),
                i_pix2,
            ) as uint64_t);
            x_0 += 8 as c_int;
        }
    }
    if i_width & 7 as c_int != 0 {
        y = 0 as c_int;
        while y < i_height & !(7 as c_int) {
            let mut x_1: c_int = i_width & !(7 as c_int);
            while x_1 < i_width {
                let mut d: c_int = *pix1.offset((y as intptr_t * i_pix1 + x_1 as intptr_t) as isize)
                    as c_int
                    - *pix2.offset((y as intptr_t * i_pix2 + x_1 as intptr_t) as isize) as c_int;
                i_ssd = i_ssd.wrapping_add((d * d) as uint64_t);
                x_1 += 1;
            }
            y += 1;
        }
    }
    if i_height & 7 as c_int != 0 {
        y = i_height & !(7 as c_int);
        while y < i_height {
            let mut x_2: c_int = 0 as c_int;
            while x_2 < i_width {
                let mut d_0: c_int = *pix1
                    .offset((y as intptr_t * i_pix1 + x_2 as intptr_t) as isize)
                    as c_int
                    - *pix2.offset((y as intptr_t * i_pix2 + x_2 as intptr_t) as isize) as c_int;
                i_ssd = i_ssd.wrapping_add((d_0 * d_0) as uint64_t);
                x_2 += 1;
            }
            y += 1;
        }
    }
    return i_ssd;
}
#[c2rust::src_loc = "153:1"]
unsafe extern "C" fn pixel_ssd_nv12_core(
    mut pixuv1: *mut pixel,
    mut stride1: intptr_t,
    mut pixuv2: *mut pixel,
    mut stride2: intptr_t,
    mut width: c_int,
    mut height: c_int,
    mut ssd_u: *mut uint64_t,
    mut ssd_v: *mut uint64_t,
) {
    *ssd_u = 0 as uint64_t;
    *ssd_v = 0 as uint64_t;
    let mut y: c_int = 0 as c_int;
    while y < height {
        let mut x: c_int = 0 as c_int;
        while x < width {
            let mut du: c_int = *pixuv1.offset((2 as c_int * x) as isize) as c_int
                - *pixuv2.offset((2 as c_int * x) as isize) as c_int;
            let mut dv: c_int = *pixuv1.offset((2 as c_int * x + 1 as c_int) as isize) as c_int
                - *pixuv2.offset((2 as c_int * x + 1 as c_int) as isize) as c_int;
            *ssd_u = (*ssd_u).wrapping_add((du * du) as uint64_t);
            *ssd_v = (*ssd_v).wrapping_add((dv * dv) as uint64_t);
            x += 1;
        }
        y += 1;
        pixuv1 = pixuv1.offset(stride1 as isize);
        pixuv2 = pixuv2.offset(stride2 as isize);
    }
}
#[no_mangle]
#[c2rust::src_loc = "167:1"]
unsafe extern "C" fn x264_10_pixel_ssd_nv12(
    mut pf: *mut x264_pixel_function_t,
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
    mut i_width: c_int,
    mut i_height: c_int,
    mut ssd_u: *mut uint64_t,
    mut ssd_v: *mut uint64_t,
) {
    (*pf).ssd_nv12_core.expect("non-null function pointer")(
        pix1,
        i_pix1,
        pix2,
        i_pix2,
        i_width & !(7 as c_int),
        i_height,
        ssd_u,
        ssd_v,
    );
    if i_width & 7 as c_int != 0 {
        let mut tmp: [uint64_t; 2] = [0; 2];
        pixel_ssd_nv12_core(
            pix1.offset((i_width & !(7 as c_int)) as isize),
            i_pix1,
            pix2.offset((i_width & !(7 as c_int)) as isize),
            i_pix2,
            i_width & 7 as c_int,
            i_height,
            &mut *tmp.as_mut_ptr().offset(0 as c_int as isize),
            &mut *tmp.as_mut_ptr().offset(1 as c_int as isize),
        );
        *ssd_u = (*ssd_u).wrapping_add(tmp[0 as c_int as usize]);
        *ssd_v = (*ssd_v).wrapping_add(tmp[1 as c_int as usize]);
    }
}
#[c2rust::src_loc = "199:1"]
unsafe extern "C" fn pixel_var_16x16(mut pix: *mut pixel, mut i_stride: intptr_t) -> uint64_t {
    let mut sum: uint32_t = 0 as uint32_t;
    let mut sqr: uint32_t = 0 as uint32_t;
    let mut y: c_int = 0 as c_int;
    while y < 16 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 16 as c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr.wrapping_add(
                (*pix.offset(x as isize) as c_int * *pix.offset(x as isize) as c_int) as uint32_t,
            );
            x += 1;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
    }
    return (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as c_int);
}
#[c2rust::src_loc = "200:1"]
unsafe extern "C" fn pixel_var_8x16(mut pix: *mut pixel, mut i_stride: intptr_t) -> uint64_t {
    let mut sum: uint32_t = 0 as uint32_t;
    let mut sqr: uint32_t = 0 as uint32_t;
    let mut y: c_int = 0 as c_int;
    while y < 16 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 8 as c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr.wrapping_add(
                (*pix.offset(x as isize) as c_int * *pix.offset(x as isize) as c_int) as uint32_t,
            );
            x += 1;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
    }
    return (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as c_int);
}
#[c2rust::src_loc = "201:1"]
unsafe extern "C" fn pixel_var_8x8(mut pix: *mut pixel, mut i_stride: intptr_t) -> uint64_t {
    let mut sum: uint32_t = 0 as uint32_t;
    let mut sqr: uint32_t = 0 as uint32_t;
    let mut y: c_int = 0 as c_int;
    while y < 8 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 8 as c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr.wrapping_add(
                (*pix.offset(x as isize) as c_int * *pix.offset(x as isize) as c_int) as uint32_t,
            );
            x += 1;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
    }
    return (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as c_int);
}
#[c2rust::src_loc = "230:1"]
unsafe extern "C" fn pixel_var2_8x16(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut ssd: *mut c_int,
) -> c_int {
    let mut sum_u: c_int = 0 as c_int;
    let mut sum_v: c_int = 0 as c_int;
    let mut sqr_u: c_int = 0 as c_int;
    let mut sqr_v: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 16 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 8 as c_int {
            let mut diff_u: c_int =
                *fenc.offset(x as isize) as c_int - *fdec.offset(x as isize) as c_int;
            let mut diff_v: c_int = *fenc.offset((x + FENC_STRIDE / 2 as c_int) as isize) as c_int
                - *fdec.offset((x + FDEC_STRIDE / 2 as c_int) as isize) as c_int;
            sum_u += diff_u;
            sum_v += diff_v;
            sqr_u += diff_u * diff_u;
            sqr_v += diff_v * diff_v;
            x += 1;
        }
        fenc = fenc.offset(FENC_STRIDE as isize);
        fdec = fdec.offset(FDEC_STRIDE as isize);
        y += 1;
    }
    *ssd.offset(0 as c_int as isize) = sqr_u;
    *ssd.offset(1 as c_int as isize) = sqr_v;
    return (sqr_u as int64_t - (sum_u as int64_t * sum_u as int64_t >> 7 as c_int)
        + sqr_v as int64_t
        - (sum_v as int64_t * sum_v as int64_t >> 7 as c_int)) as c_int;
}
#[c2rust::src_loc = "231:1"]
unsafe extern "C" fn pixel_var2_8x8(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut ssd: *mut c_int,
) -> c_int {
    let mut sum_u: c_int = 0 as c_int;
    let mut sum_v: c_int = 0 as c_int;
    let mut sqr_u: c_int = 0 as c_int;
    let mut sqr_v: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < 8 as c_int {
        let mut x: c_int = 0 as c_int;
        while x < 8 as c_int {
            let mut diff_u: c_int =
                *fenc.offset(x as isize) as c_int - *fdec.offset(x as isize) as c_int;
            let mut diff_v: c_int = *fenc.offset((x + FENC_STRIDE / 2 as c_int) as isize) as c_int
                - *fdec.offset((x + FDEC_STRIDE / 2 as c_int) as isize) as c_int;
            sum_u += diff_u;
            sum_v += diff_v;
            sqr_u += diff_u * diff_u;
            sqr_v += diff_v * diff_v;
            x += 1;
        }
        fenc = fenc.offset(FENC_STRIDE as isize);
        fdec = fdec.offset(FDEC_STRIDE as isize);
        y += 1;
    }
    *ssd.offset(0 as c_int as isize) = sqr_u;
    *ssd.offset(1 as c_int as isize) = sqr_v;
    return (sqr_u as int64_t - (sum_u as int64_t * sum_u as int64_t >> 6 as c_int)
        + sqr_v as int64_t
        - (sum_v as int64_t * sum_v as int64_t >> 6 as c_int)) as c_int;
}
#[c2rust::src_loc = "240:9"]
const BITS_PER_SUM: usize = (8 as usize).wrapping_mul(::core::mem::size_of::<sum_t>() as usize);
#[inline(always)]
#[c2rust::src_loc = "255:1"]
unsafe extern "C" fn abs2(mut a: sum2_t) -> sum2_t {
    let mut s: sum2_t = (a >> BITS_PER_SUM.wrapping_sub(1 as usize)
        & ((1 as c_int as sum2_t) << BITS_PER_SUM).wrapping_add(1 as sum2_t))
    .wrapping_mul(-(1 as c_int) as sum_t as sum2_t);
    return a.wrapping_add(s) ^ s;
}
#[inline(never)]
#[c2rust::src_loc = "265:1"]
unsafe extern "C" fn x264_pixel_satd_4x4(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> c_int {
    let mut tmp: [[sum2_t; 2]; 4] = [[0; 2]; 4];
    let mut a0: sum2_t = 0;
    let mut a1: sum2_t = 0;
    let mut a2: sum2_t = 0;
    let mut a3: sum2_t = 0;
    let mut b0: sum2_t = 0;
    let mut b1: sum2_t = 0;
    let mut sum: sum2_t = 0 as sum2_t;
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        a0 = (*pix1.offset(0 as c_int as isize) as c_int
            - *pix2.offset(0 as c_int as isize) as c_int) as sum2_t;
        a1 = (*pix1.offset(1 as c_int as isize) as c_int
            - *pix2.offset(1 as c_int as isize) as c_int) as sum2_t;
        b0 = a0
            .wrapping_add(a1)
            .wrapping_add(a0.wrapping_sub(a1) << BITS_PER_SUM);
        a2 = (*pix1.offset(2 as c_int as isize) as c_int
            - *pix2.offset(2 as c_int as isize) as c_int) as sum2_t;
        a3 = (*pix1.offset(3 as c_int as isize) as c_int
            - *pix2.offset(3 as c_int as isize) as c_int) as sum2_t;
        b1 = a2
            .wrapping_add(a3)
            .wrapping_add(a2.wrapping_sub(a3) << BITS_PER_SUM);
        tmp[i as usize][0 as c_int as usize] = b0.wrapping_add(b1);
        tmp[i as usize][1 as c_int as usize] = b0.wrapping_sub(b1);
        i += 1;
        pix1 = pix1.offset(i_pix1 as isize);
        pix2 = pix2.offset(i_pix2 as isize);
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 2 as c_int {
        let mut t0: sum2_t = tmp[0 as c_int as usize][i_0 as usize]
            .wrapping_add(tmp[1 as c_int as usize][i_0 as usize]);
        let mut t1: sum2_t = tmp[0 as c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[1 as c_int as usize][i_0 as usize]);
        let mut t2: sum2_t = tmp[2 as c_int as usize][i_0 as usize]
            .wrapping_add(tmp[3 as c_int as usize][i_0 as usize]);
        let mut t3: sum2_t = tmp[2 as c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[3 as c_int as usize][i_0 as usize]);
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
    return (sum >> 1 as c_int) as c_int;
}
#[inline(never)]
#[c2rust::src_loc = "290:1"]
unsafe extern "C" fn x264_pixel_satd_8x4(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> c_int {
    let mut tmp: [[sum2_t; 4]; 4] = [[0; 4]; 4];
    let mut a0: sum2_t = 0;
    let mut a1: sum2_t = 0;
    let mut a2: sum2_t = 0;
    let mut a3: sum2_t = 0;
    let mut sum: sum2_t = 0 as sum2_t;
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        a0 = ((*pix1.offset(0 as c_int as isize) as c_int
            - *pix2.offset(0 as c_int as isize) as c_int) as sum2_t)
            .wrapping_add(
                ((*pix1.offset(4 as c_int as isize) as c_int
                    - *pix2.offset(4 as c_int as isize) as c_int) as sum2_t)
                    << BITS_PER_SUM,
            );
        a1 = ((*pix1.offset(1 as c_int as isize) as c_int
            - *pix2.offset(1 as c_int as isize) as c_int) as sum2_t)
            .wrapping_add(
                ((*pix1.offset(5 as c_int as isize) as c_int
                    - *pix2.offset(5 as c_int as isize) as c_int) as sum2_t)
                    << BITS_PER_SUM,
            );
        a2 = ((*pix1.offset(2 as c_int as isize) as c_int
            - *pix2.offset(2 as c_int as isize) as c_int) as sum2_t)
            .wrapping_add(
                ((*pix1.offset(6 as c_int as isize) as c_int
                    - *pix2.offset(6 as c_int as isize) as c_int) as sum2_t)
                    << BITS_PER_SUM,
            );
        a3 = ((*pix1.offset(3 as c_int as isize) as c_int
            - *pix2.offset(3 as c_int as isize) as c_int) as sum2_t)
            .wrapping_add(
                ((*pix1.offset(7 as c_int as isize) as c_int
                    - *pix2.offset(7 as c_int as isize) as c_int) as sum2_t)
                    << BITS_PER_SUM,
            );
        let mut t0: sum2_t = a0.wrapping_add(a1);
        let mut t1: sum2_t = a0.wrapping_sub(a1);
        let mut t2: sum2_t = a2.wrapping_add(a3);
        let mut t3: sum2_t = a2.wrapping_sub(a3);
        tmp[i as usize][0 as c_int as usize] = t0.wrapping_add(t2);
        tmp[i as usize][2 as c_int as usize] = t0.wrapping_sub(t2);
        tmp[i as usize][1 as c_int as usize] = t1.wrapping_add(t3);
        tmp[i as usize][3 as c_int as usize] = t1.wrapping_sub(t3);
        i += 1;
        pix1 = pix1.offset(i_pix1 as isize);
        pix2 = pix2.offset(i_pix2 as isize);
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 4 as c_int {
        let mut t0_0: sum2_t = tmp[0 as c_int as usize][i_0 as usize]
            .wrapping_add(tmp[1 as c_int as usize][i_0 as usize]);
        let mut t1_0: sum2_t = tmp[0 as c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[1 as c_int as usize][i_0 as usize]);
        let mut t2_0: sum2_t = tmp[2 as c_int as usize][i_0 as usize]
            .wrapping_add(tmp[3 as c_int as usize][i_0 as usize]);
        let mut t3_0: sum2_t = tmp[2 as c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[3 as c_int as usize][i_0 as usize]);
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
    return ((sum as sum_t as sum2_t).wrapping_add(sum >> BITS_PER_SUM) >> 1 as c_int) as c_int;
}
#[c2rust::src_loc = "327:1"]
unsafe extern "C" fn x264_pixel_satd_16x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> c_int {
    let mut sum: c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 16 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 16 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 16 as c_int == 16 as c_int && 16 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    return sum;
}
#[c2rust::src_loc = "328:1"]
unsafe extern "C" fn x264_pixel_satd_16x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> c_int {
    let mut sum: c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 16 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 8 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 16 as c_int == 16 as c_int && 8 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    return sum;
}
#[c2rust::src_loc = "329:1"]
unsafe extern "C" fn x264_pixel_satd_8x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> c_int {
    let mut sum: c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 8 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 16 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 8 as c_int == 16 as c_int && 16 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    return sum;
}
#[c2rust::src_loc = "330:1"]
unsafe extern "C" fn x264_pixel_satd_8x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> c_int {
    let mut sum: c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 8 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 8 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 8 as c_int == 16 as c_int && 8 as c_int == 16 as c_int {
        sum += x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_8x4(
            pix1.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    return sum;
}
#[c2rust::src_loc = "331:1"]
unsafe extern "C" fn x264_pixel_satd_4x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> c_int {
    let mut sum: c_int = x264_pixel_satd_4x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_4x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 4 as c_int == 16 as c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset(8 as c_int as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 16 as c_int == 16 as c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 4 as c_int == 16 as c_int && 16 as c_int == 16 as c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    return sum;
}
#[c2rust::src_loc = "332:1"]
unsafe extern "C" fn x264_pixel_satd_4x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> c_int {
    let mut sum: c_int = x264_pixel_satd_4x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_4x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 4 as c_int == 16 as c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset(8 as c_int as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 8 as c_int == 16 as c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    if 4 as c_int == 16 as c_int && 8 as c_int == 16 as c_int {
        sum += x264_pixel_satd_4x4(
            pix1.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        ) + x264_pixel_satd_4x4(
            pix1.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((12 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    }
    return sum;
}
#[inline(never)]
#[c2rust::src_loc = "334:1"]
unsafe extern "C" fn sa8d_8x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> c_int {
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
    let mut i: c_int = 0 as c_int;
    while i < 8 as c_int {
        a0 = (*pix1.offset(0 as c_int as isize) as c_int
            - *pix2.offset(0 as c_int as isize) as c_int) as sum2_t;
        a1 = (*pix1.offset(1 as c_int as isize) as c_int
            - *pix2.offset(1 as c_int as isize) as c_int) as sum2_t;
        b0 = a0
            .wrapping_add(a1)
            .wrapping_add(a0.wrapping_sub(a1) << BITS_PER_SUM);
        a2 = (*pix1.offset(2 as c_int as isize) as c_int
            - *pix2.offset(2 as c_int as isize) as c_int) as sum2_t;
        a3 = (*pix1.offset(3 as c_int as isize) as c_int
            - *pix2.offset(3 as c_int as isize) as c_int) as sum2_t;
        b1 = a2
            .wrapping_add(a3)
            .wrapping_add(a2.wrapping_sub(a3) << BITS_PER_SUM);
        a4 = (*pix1.offset(4 as c_int as isize) as c_int
            - *pix2.offset(4 as c_int as isize) as c_int) as sum2_t;
        a5 = (*pix1.offset(5 as c_int as isize) as c_int
            - *pix2.offset(5 as c_int as isize) as c_int) as sum2_t;
        b2 = a4
            .wrapping_add(a5)
            .wrapping_add(a4.wrapping_sub(a5) << BITS_PER_SUM);
        a6 = (*pix1.offset(6 as c_int as isize) as c_int
            - *pix2.offset(6 as c_int as isize) as c_int) as sum2_t;
        a7 = (*pix1.offset(7 as c_int as isize) as c_int
            - *pix2.offset(7 as c_int as isize) as c_int) as sum2_t;
        b3 = a6
            .wrapping_add(a7)
            .wrapping_add(a6.wrapping_sub(a7) << BITS_PER_SUM);
        let mut t0: sum2_t = b0.wrapping_add(b1);
        let mut t1: sum2_t = b0.wrapping_sub(b1);
        let mut t2: sum2_t = b2.wrapping_add(b3);
        let mut t3: sum2_t = b2.wrapping_sub(b3);
        tmp[i as usize][0 as c_int as usize] = t0.wrapping_add(t2);
        tmp[i as usize][2 as c_int as usize] = t0.wrapping_sub(t2);
        tmp[i as usize][1 as c_int as usize] = t1.wrapping_add(t3);
        tmp[i as usize][3 as c_int as usize] = t1.wrapping_sub(t3);
        i += 1;
        pix1 = pix1.offset(i_pix1 as isize);
        pix2 = pix2.offset(i_pix2 as isize);
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 4 as c_int {
        let mut t0_0: sum2_t = tmp[0 as c_int as usize][i_0 as usize]
            .wrapping_add(tmp[1 as c_int as usize][i_0 as usize]);
        let mut t1_0: sum2_t = tmp[0 as c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[1 as c_int as usize][i_0 as usize]);
        let mut t2_0: sum2_t = tmp[2 as c_int as usize][i_0 as usize]
            .wrapping_add(tmp[3 as c_int as usize][i_0 as usize]);
        let mut t3_0: sum2_t = tmp[2 as c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[3 as c_int as usize][i_0 as usize]);
        a0 = t0_0.wrapping_add(t2_0);
        a2 = t0_0.wrapping_sub(t2_0);
        a1 = t1_0.wrapping_add(t3_0);
        a3 = t1_0.wrapping_sub(t3_0);
        let mut t0_1: sum2_t = tmp[4 as c_int as usize][i_0 as usize]
            .wrapping_add(tmp[5 as c_int as usize][i_0 as usize]);
        let mut t1_1: sum2_t = tmp[4 as c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[5 as c_int as usize][i_0 as usize]);
        let mut t2_1: sum2_t = tmp[6 as c_int as usize][i_0 as usize]
            .wrapping_add(tmp[7 as c_int as usize][i_0 as usize]);
        let mut t3_1: sum2_t = tmp[6 as c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[7 as c_int as usize][i_0 as usize]);
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
    return sum as c_int;
}
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn x264_pixel_sa8d_8x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> c_int {
    let mut sum: c_int = sa8d_8x8(pix1, i_pix1, pix2, i_pix2);
    return sum + 2 as c_int >> 2 as c_int;
}
#[c2rust::src_loc = "374:1"]
unsafe extern "C" fn x264_pixel_sa8d_16x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> c_int {
    let mut sum: c_int = sa8d_8x8(pix1, i_pix1, pix2, i_pix2)
        + sa8d_8x8(
            pix1.offset(8 as c_int as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize),
            i_pix2,
        )
        + sa8d_8x8(
            pix1.offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        )
        + sa8d_8x8(
            pix1.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset(8 as c_int as isize)
                .offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    return sum + 2 as c_int >> 2 as c_int;
}
#[inline(never)]
#[c2rust::src_loc = "383:1"]
unsafe extern "C" fn pixel_hadamard_ac(mut pix: *mut pixel, mut stride: intptr_t) -> uint64_t {
    let mut tmp: [sum2_t; 32] = [0; 32];
    let mut a0: sum2_t = 0;
    let mut a1: sum2_t = 0;
    let mut a2: sum2_t = 0;
    let mut a3: sum2_t = 0;
    let mut dc: sum2_t = 0;
    let mut sum4: sum2_t = 0 as sum2_t;
    let mut sum8: sum2_t = 0 as sum2_t;
    let mut i: c_int = 0 as c_int;
    while i < 8 as c_int {
        let mut t: *mut sum2_t = tmp
            .as_mut_ptr()
            .offset((i & 3 as c_int) as isize)
            .offset(((i & 4 as c_int) * 4 as c_int) as isize);
        a0 = ((*pix.offset(0 as c_int as isize) as c_int
            + *pix.offset(1 as c_int as isize) as c_int) as sum2_t)
            .wrapping_add(
                ((*pix.offset(0 as c_int as isize) as c_int
                    - *pix.offset(1 as c_int as isize) as c_int) as sum2_t)
                    << BITS_PER_SUM,
            );
        a1 = ((*pix.offset(2 as c_int as isize) as c_int
            + *pix.offset(3 as c_int as isize) as c_int) as sum2_t)
            .wrapping_add(
                ((*pix.offset(2 as c_int as isize) as c_int
                    - *pix.offset(3 as c_int as isize) as c_int) as sum2_t)
                    << BITS_PER_SUM,
            );
        *t.offset(0 as c_int as isize) = a0.wrapping_add(a1);
        *t.offset(4 as c_int as isize) = a0.wrapping_sub(a1);
        a2 = ((*pix.offset(4 as c_int as isize) as c_int
            + *pix.offset(5 as c_int as isize) as c_int) as sum2_t)
            .wrapping_add(
                ((*pix.offset(4 as c_int as isize) as c_int
                    - *pix.offset(5 as c_int as isize) as c_int) as sum2_t)
                    << BITS_PER_SUM,
            );
        a3 = ((*pix.offset(6 as c_int as isize) as c_int
            + *pix.offset(7 as c_int as isize) as c_int) as sum2_t)
            .wrapping_add(
                ((*pix.offset(6 as c_int as isize) as c_int
                    - *pix.offset(7 as c_int as isize) as c_int) as sum2_t)
                    << BITS_PER_SUM,
            );
        *t.offset(8 as c_int as isize) = a2.wrapping_add(a3);
        *t.offset(12 as c_int as isize) = a2.wrapping_sub(a3);
        i += 1;
        pix = pix.offset(stride as isize);
    }
    let mut i_0: c_int = 0 as c_int;
    while i_0 < 8 as c_int {
        let mut t0: sum2_t = tmp[(i_0 * 4 as c_int + 0 as c_int) as usize]
            .wrapping_add(tmp[(i_0 * 4 as c_int + 1 as c_int) as usize]);
        let mut t1: sum2_t = tmp[(i_0 * 4 as c_int + 0 as c_int) as usize]
            .wrapping_sub(tmp[(i_0 * 4 as c_int + 1 as c_int) as usize]);
        let mut t2: sum2_t = tmp[(i_0 * 4 as c_int + 2 as c_int) as usize]
            .wrapping_add(tmp[(i_0 * 4 as c_int + 3 as c_int) as usize]);
        let mut t3: sum2_t = tmp[(i_0 * 4 as c_int + 2 as c_int) as usize]
            .wrapping_sub(tmp[(i_0 * 4 as c_int + 3 as c_int) as usize]);
        a0 = t0.wrapping_add(t2);
        a2 = t0.wrapping_sub(t2);
        a1 = t1.wrapping_add(t3);
        a3 = t1.wrapping_sub(t3);
        tmp[(i_0 * 4 as c_int + 0 as c_int) as usize] = a0;
        tmp[(i_0 * 4 as c_int + 1 as c_int) as usize] = a1;
        tmp[(i_0 * 4 as c_int + 2 as c_int) as usize] = a2;
        tmp[(i_0 * 4 as c_int + 3 as c_int) as usize] = a3;
        sum4 = sum4.wrapping_add(
            abs2(a0)
                .wrapping_add(abs2(a1))
                .wrapping_add(abs2(a2))
                .wrapping_add(abs2(a3)),
        );
        i_0 += 1;
    }
    let mut i_1: c_int = 0 as c_int;
    while i_1 < 8 as c_int {
        let mut t0_0: sum2_t = tmp[i_1 as usize].wrapping_add(tmp[(8 as c_int + i_1) as usize]);
        let mut t1_0: sum2_t = tmp[i_1 as usize].wrapping_sub(tmp[(8 as c_int + i_1) as usize]);
        let mut t2_0: sum2_t =
            tmp[(16 as c_int + i_1) as usize].wrapping_add(tmp[(24 as c_int + i_1) as usize]);
        let mut t3_0: sum2_t =
            tmp[(16 as c_int + i_1) as usize].wrapping_sub(tmp[(24 as c_int + i_1) as usize]);
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
    dc = tmp[0 as c_int as usize]
        .wrapping_add(tmp[8 as c_int as usize])
        .wrapping_add(tmp[16 as c_int as usize])
        .wrapping_add(tmp[24 as c_int as usize]) as sum_t as sum2_t;
    sum4 = (sum4 as sum_t as sum2_t)
        .wrapping_add(sum4 >> BITS_PER_SUM)
        .wrapping_sub(dc);
    sum8 = (sum8 as sum_t as sum2_t)
        .wrapping_add(sum8 >> BITS_PER_SUM)
        .wrapping_sub(dc);
    return (sum8 << 32 as c_int).wrapping_add(sum4 as uint64_t);
}
#[c2rust::src_loc = "432:1"]
unsafe extern "C" fn x264_pixel_hadamard_ac_16x16(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 16 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8 as c_int as isize), stride));
    }
    if 16 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as intptr_t * stride) as isize),
            stride,
        ));
    }
    if 16 as c_int == 16 as c_int && 16 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as intptr_t * stride) as isize)
                .offset(8 as c_int as isize),
            stride,
        ));
    }
    return ((sum >> 34 as c_int) << 32 as c_int)
        .wrapping_add((sum as uint32_t >> 1 as c_int) as uint64_t);
}
#[c2rust::src_loc = "433:1"]
unsafe extern "C" fn x264_pixel_hadamard_ac_16x8(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 16 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8 as c_int as isize), stride));
    }
    if 8 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as intptr_t * stride) as isize),
            stride,
        ));
    }
    if 16 as c_int == 16 as c_int && 8 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as intptr_t * stride) as isize)
                .offset(8 as c_int as isize),
            stride,
        ));
    }
    return ((sum >> 34 as c_int) << 32 as c_int)
        .wrapping_add((sum as uint32_t >> 1 as c_int) as uint64_t);
}
#[c2rust::src_loc = "434:1"]
unsafe extern "C" fn x264_pixel_hadamard_ac_8x16(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 8 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8 as c_int as isize), stride));
    }
    if 16 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as intptr_t * stride) as isize),
            stride,
        ));
    }
    if 8 as c_int == 16 as c_int && 16 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as intptr_t * stride) as isize)
                .offset(8 as c_int as isize),
            stride,
        ));
    }
    return ((sum >> 34 as c_int) << 32 as c_int)
        .wrapping_add((sum as uint32_t >> 1 as c_int) as uint64_t);
}
#[c2rust::src_loc = "435:1"]
unsafe extern "C" fn x264_pixel_hadamard_ac_8x8(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 8 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(pix.offset(8 as c_int as isize), stride));
    }
    if 8 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as intptr_t * stride) as isize),
            stride,
        ));
    }
    if 8 as c_int == 16 as c_int && 8 as c_int == 16 as c_int {
        sum = sum.wrapping_add(pixel_hadamard_ac(
            pix.offset((8 as intptr_t * stride) as isize)
                .offset(8 as c_int as isize),
            stride,
        ));
    }
    return ((sum >> 34 as c_int) << 32 as c_int)
        .wrapping_add((sum as uint32_t >> 1 as c_int) as uint64_t);
}
#[c2rust::src_loc = "458:1"]
unsafe extern "C" fn x264_pixel_sad_x3_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_16x16(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_16x16(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_16x16(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "458:1"]
unsafe extern "C" fn x264_pixel_sad_x4_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_16x16(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_16x16(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_16x16(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_sad_16x16(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "459:1"]
unsafe extern "C" fn x264_pixel_sad_x4_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_16x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_16x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_16x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_sad_16x8(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "459:1"]
unsafe extern "C" fn x264_pixel_sad_x3_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_16x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_16x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_16x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "460:1"]
unsafe extern "C" fn x264_pixel_sad_x4_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_8x16(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_8x16(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_8x16(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_sad_8x16(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "460:1"]
unsafe extern "C" fn x264_pixel_sad_x3_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_8x16(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_8x16(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_8x16(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "461:1"]
unsafe extern "C" fn x264_pixel_sad_x3_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_8x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_8x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_8x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "461:1"]
unsafe extern "C" fn x264_pixel_sad_x4_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_8x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_8x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_8x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_sad_8x8(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "462:1"]
unsafe extern "C" fn x264_pixel_sad_x3_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_8x4(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_8x4(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_8x4(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "462:1"]
unsafe extern "C" fn x264_pixel_sad_x4_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_8x4(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_8x4(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_8x4(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_sad_8x4(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "463:1"]
unsafe extern "C" fn x264_pixel_sad_x3_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_4x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_4x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_4x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "463:1"]
unsafe extern "C" fn x264_pixel_sad_x4_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_4x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_4x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_4x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_sad_4x8(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "464:1"]
unsafe extern "C" fn x264_pixel_sad_x4_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_4x4(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_4x4(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_4x4(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_sad_4x4(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "464:1"]
unsafe extern "C" fn x264_pixel_sad_x3_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_sad_4x4(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_sad_4x4(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_sad_4x4(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_8x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_8x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_8x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_4x4(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_4x4(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_4x4(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_satd_4x4(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_4x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_4x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_4x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_satd_4x8(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_16x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_16x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_16x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_satd_16x8(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_8x4(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_8x4(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_8x4(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_satd_8x4(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_8x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_8x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_8x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_satd_8x8(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_8x16(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_8x16(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_8x16(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_satd_8x16(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_4x4(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_4x4(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_4x4(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_16x16(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_16x16(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_16x16(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
    *scores.offset(3 as c_int as isize) =
        x264_pixel_satd_16x16(fenc, FENC_STRIDE as intptr_t, pix3, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_4x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_4x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_4x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_8x4(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_8x4(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_8x4(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_8x16(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_8x16(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_8x16(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_16x8(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_16x8(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_16x8(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut c_int,
) {
    *scores.offset(0 as c_int as isize) =
        x264_pixel_satd_16x16(fenc, FENC_STRIDE as intptr_t, pix0, i_stride);
    *scores.offset(1 as c_int as isize) =
        x264_pixel_satd_16x16(fenc, FENC_STRIDE as intptr_t, pix1, i_stride);
    *scores.offset(2 as c_int as isize) =
        x264_pixel_satd_16x16(fenc, FENC_STRIDE as intptr_t, pix2, i_stride);
}
#[c2rust::src_loc = "530:1"]
unsafe extern "C" fn intra_sad_x3_8x8(
    mut fenc: *mut pixel,
    mut edge: *mut pixel,
    mut res: *mut c_int,
) {
    let mut pix: [pixel; 256] = [0; 256];
    x264_10_predict_8x8_v_c(pix.as_mut_ptr(), edge);
    *res.offset(0 as c_int as isize) = x264_pixel_sad_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8_h_c(pix.as_mut_ptr(), edge);
    *res.offset(1 as c_int as isize) = x264_pixel_sad_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8_dc_c(pix.as_mut_ptr(), edge);
    *res.offset(2 as c_int as isize) = x264_pixel_sad_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "531:1"]
unsafe extern "C" fn intra_sa8d_x3_8x8(
    mut fenc: *mut pixel,
    mut edge: *mut pixel,
    mut res: *mut c_int,
) {
    let mut pix: [pixel; 256] = [0; 256];
    x264_10_predict_8x8_v_c(pix.as_mut_ptr(), edge);
    *res.offset(0 as c_int as isize) = x264_pixel_sa8d_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8_h_c(pix.as_mut_ptr(), edge);
    *res.offset(1 as c_int as isize) = x264_pixel_sa8d_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8_dc_c(pix.as_mut_ptr(), edge);
    *res.offset(2 as c_int as isize) = x264_pixel_sa8d_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "553:1"]
unsafe extern "C" fn intra_sad_x3_4x4(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut c_int,
) {
    x264_10_predict_4x4_v_c(fdec);
    *res.offset(0 as c_int as isize) =
        x264_pixel_sad_4x4(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_4x4_h_c(fdec);
    *res.offset(1 as c_int as isize) =
        x264_pixel_sad_4x4(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_4x4_dc_c(fdec);
    *res.offset(2 as c_int as isize) =
        x264_pixel_sad_4x4(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
}
#[c2rust::src_loc = "554:1"]
unsafe extern "C" fn intra_satd_x3_4x4(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut c_int,
) {
    x264_10_predict_4x4_v_c(fdec);
    *res.offset(0 as c_int as isize) =
        x264_pixel_satd_4x4(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_4x4_h_c(fdec);
    *res.offset(1 as c_int as isize) =
        x264_pixel_satd_4x4(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_4x4_dc_c(fdec);
    *res.offset(2 as c_int as isize) =
        x264_pixel_satd_4x4(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
}
#[c2rust::src_loc = "555:1"]
unsafe extern "C" fn intra_sad_x3_8x8c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut c_int,
) {
    x264_10_predict_8x8c_dc_c(fdec);
    *res.offset(0 as c_int as isize) =
        x264_pixel_sad_8x8(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_8x8c_h_c(fdec);
    *res.offset(1 as c_int as isize) =
        x264_pixel_sad_8x8(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_8x8c_v_c(fdec);
    *res.offset(2 as c_int as isize) =
        x264_pixel_sad_8x8(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
}
#[c2rust::src_loc = "556:1"]
unsafe extern "C" fn intra_satd_x3_8x8c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut c_int,
) {
    x264_10_predict_8x8c_dc_c(fdec);
    *res.offset(0 as c_int as isize) =
        x264_pixel_satd_8x8(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_8x8c_h_c(fdec);
    *res.offset(1 as c_int as isize) =
        x264_pixel_satd_8x8(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_8x8c_v_c(fdec);
    *res.offset(2 as c_int as isize) =
        x264_pixel_satd_8x8(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
}
#[c2rust::src_loc = "557:1"]
unsafe extern "C" fn intra_sad_x3_8x16c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut c_int,
) {
    x264_10_predict_8x16c_dc_c(fdec);
    *res.offset(0 as c_int as isize) =
        x264_pixel_sad_8x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_8x16c_h_c(fdec);
    *res.offset(1 as c_int as isize) =
        x264_pixel_sad_8x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_8x16c_v_c(fdec);
    *res.offset(2 as c_int as isize) =
        x264_pixel_sad_8x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
}
#[c2rust::src_loc = "558:1"]
unsafe extern "C" fn intra_satd_x3_8x16c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut c_int,
) {
    x264_10_predict_8x16c_dc_c(fdec);
    *res.offset(0 as c_int as isize) =
        x264_pixel_satd_8x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_8x16c_h_c(fdec);
    *res.offset(1 as c_int as isize) =
        x264_pixel_satd_8x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_8x16c_v_c(fdec);
    *res.offset(2 as c_int as isize) =
        x264_pixel_satd_8x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
}
#[c2rust::src_loc = "559:1"]
unsafe extern "C" fn intra_sad_x3_16x16(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut c_int,
) {
    x264_10_predict_16x16_v_c(fdec);
    *res.offset(0 as c_int as isize) =
        x264_pixel_sad_16x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_16x16_h_c(fdec);
    *res.offset(1 as c_int as isize) =
        x264_pixel_sad_16x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_16x16_dc_c(fdec);
    *res.offset(2 as c_int as isize) =
        x264_pixel_sad_16x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
}
#[c2rust::src_loc = "560:1"]
unsafe extern "C" fn intra_satd_x3_16x16(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut c_int,
) {
    x264_10_predict_16x16_v_c(fdec);
    *res.offset(0 as c_int as isize) =
        x264_pixel_satd_16x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_16x16_h_c(fdec);
    *res.offset(1 as c_int as isize) =
        x264_pixel_satd_16x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
    x264_10_predict_16x16_dc_c(fdec);
    *res.offset(2 as c_int as isize) =
        x264_pixel_satd_16x16(fdec, FDEC_STRIDE as intptr_t, fenc, FENC_STRIDE as intptr_t);
}
#[c2rust::src_loc = "627:1"]
unsafe extern "C" fn ssim_4x4x2_core(
    mut pix1: *const pixel,
    mut stride1: intptr_t,
    mut pix2: *const pixel,
    mut stride2: intptr_t,
    mut sums: *mut [c_int; 4],
) {
    let mut z: c_int = 0 as c_int;
    while z < 2 as c_int {
        let mut s1: uint32_t = 0 as uint32_t;
        let mut s2: uint32_t = 0 as uint32_t;
        let mut ss: uint32_t = 0 as uint32_t;
        let mut s12: uint32_t = 0 as uint32_t;
        let mut y: c_int = 0 as c_int;
        while y < 4 as c_int {
            let mut x: c_int = 0 as c_int;
            while x < 4 as c_int {
                let mut a: c_int =
                    *pix1.offset((x as intptr_t + y as intptr_t * stride1) as isize) as c_int;
                let mut b: c_int =
                    *pix2.offset((x as intptr_t + y as intptr_t * stride2) as isize) as c_int;
                s1 = s1.wrapping_add(a as uint32_t);
                s2 = s2.wrapping_add(b as uint32_t);
                ss = ss.wrapping_add((a * a) as uint32_t);
                ss = ss.wrapping_add((b * b) as uint32_t);
                s12 = s12.wrapping_add((a * b) as uint32_t);
                x += 1;
            }
            y += 1;
        }
        (*sums.offset(z as isize))[0 as c_int as usize] = s1 as c_int;
        (*sums.offset(z as isize))[1 as c_int as usize] = s2 as c_int;
        (*sums.offset(z as isize))[2 as c_int as usize] = ss as c_int;
        (*sums.offset(z as isize))[3 as c_int as usize] = s12 as c_int;
        pix1 = pix1.offset(4 as c_int as isize);
        pix2 = pix2.offset(4 as c_int as isize);
        z += 1;
    }
}
#[c2rust::src_loc = "654:1"]
unsafe extern "C" fn ssim_end1(
    mut s1: c_int,
    mut s2: c_int,
    mut ss: c_int,
    mut s12: c_int,
) -> c_float {
    static mut ssim_c1: c_float = (0.01f64
        * 0.01f64
        * PIXEL_MAX as c_double
        * PIXEL_MAX as c_double
        * 64 as c_int as c_double) as c_float;
    static mut ssim_c2: c_float = (0.03f64
        * 0.03f64
        * PIXEL_MAX as c_double
        * PIXEL_MAX as c_double
        * 64 as c_int as c_double
        * 63 as c_int as c_double) as c_float;
    let mut fs1: c_float = s1 as c_float;
    let mut fs2: c_float = s2 as c_float;
    let mut fss: c_float = ss as c_float;
    let mut fs12: c_float = s12 as c_float;
    let mut vars: c_float = fss * 64 as c_int as c_float - fs1 * fs1 - fs2 * fs2;
    let mut covar: c_float = fs12 * 64 as c_int as c_float - fs1 * fs2;
    return (2 as c_int as c_float * fs1 * fs2 + ssim_c1)
        * (2 as c_int as c_float * covar + ssim_c2)
        / ((fs1 * fs1 + fs2 * fs2 + ssim_c1) * (vars + ssim_c2));
}
#[c2rust::src_loc = "679:1"]
unsafe extern "C" fn ssim_end4(
    mut sum0: *mut [c_int; 4],
    mut sum1: *mut [c_int; 4],
    mut width: c_int,
) -> c_float {
    let mut ssim: c_float = 0.0f32;
    let mut i: c_int = 0 as c_int;
    while i < width {
        ssim += ssim_end1(
            (*sum0.offset(i as isize))[0 as c_int as usize]
                + (*sum0.offset((i + 1 as c_int) as isize))[0 as c_int as usize]
                + (*sum1.offset(i as isize))[0 as c_int as usize]
                + (*sum1.offset((i + 1 as c_int) as isize))[0 as c_int as usize],
            (*sum0.offset(i as isize))[1 as c_int as usize]
                + (*sum0.offset((i + 1 as c_int) as isize))[1 as c_int as usize]
                + (*sum1.offset(i as isize))[1 as c_int as usize]
                + (*sum1.offset((i + 1 as c_int) as isize))[1 as c_int as usize],
            (*sum0.offset(i as isize))[2 as c_int as usize]
                + (*sum0.offset((i + 1 as c_int) as isize))[2 as c_int as usize]
                + (*sum1.offset(i as isize))[2 as c_int as usize]
                + (*sum1.offset((i + 1 as c_int) as isize))[2 as c_int as usize],
            (*sum0.offset(i as isize))[3 as c_int as usize]
                + (*sum0.offset((i + 1 as c_int) as isize))[3 as c_int as usize]
                + (*sum1.offset(i as isize))[3 as c_int as usize]
                + (*sum1.offset((i + 1 as c_int) as isize))[3 as c_int as usize],
        );
        i += 1;
    }
    return ssim;
}
#[no_mangle]
#[c2rust::src_loc = "690:1"]
unsafe extern "C" fn x264_10_pixel_ssim_wxh(
    mut pf: *mut x264_pixel_function_t,
    mut pix1: *mut pixel,
    mut stride1: intptr_t,
    mut pix2: *mut pixel,
    mut stride2: intptr_t,
    mut width: c_int,
    mut height: c_int,
    mut buf: *mut c_void,
    mut cnt: *mut c_int,
) -> c_float {
    let mut z: c_int = 0 as c_int;
    let mut ssim: c_float = 0.0f32;
    let mut sum0: *mut [c_int; 4] = buf as *mut [c_int; 4];
    let mut sum1: *mut [c_int; 4] = sum0
        .offset((width >> 2 as c_int) as isize)
        .offset(3 as c_int as isize);
    width >>= 2 as c_int;
    height >>= 2 as c_int;
    let mut y: c_int = 1 as c_int;
    while y < height {
        while z <= y {
            let mut t: *mut c_void = sum0 as *mut c_void;
            sum0 = sum1;
            sum1 = t as *mut [c_int; 4];
            let mut x: c_int = 0 as c_int;
            while x < width {
                (*pf).ssim_4x4x2_core.expect("non-null function pointer")(
                    &mut *pix1.offset(
                        (4 as intptr_t * (x as intptr_t + z as intptr_t * stride1)) as isize,
                    ),
                    stride1,
                    &mut *pix2.offset(
                        (4 as intptr_t * (x as intptr_t + z as intptr_t * stride2)) as isize,
                    ),
                    stride2,
                    &mut *sum0.offset(x as isize),
                );
                x += 2 as c_int;
            }
            z += 1;
        }
        let mut x_0: c_int = 0 as c_int;
        while x_0 < width - 1 as c_int {
            ssim += (*pf).ssim_end4.expect("non-null function pointer")(
                sum0.offset(x_0 as isize),
                sum1.offset(x_0 as isize),
                if (4 as c_int) < width - x_0 - 1 as c_int {
                    4 as c_int
                } else {
                    width - x_0 - 1 as c_int
                },
            );
            x_0 += 4 as c_int;
        }
        y += 1;
    }
    *cnt = (height - 1 as c_int) * (width - 1 as c_int);
    return ssim;
}
#[c2rust::src_loc = "716:1"]
unsafe extern "C" fn pixel_vsad(
    mut src: *mut pixel,
    mut stride: intptr_t,
    mut height: c_int,
) -> c_int {
    let mut score: c_int = 0 as c_int;
    let mut i: c_int = 1 as c_int;
    while i < height {
        let mut j: c_int = 0 as c_int;
        while j < 16 as c_int {
            score += abs(*src.offset(j as isize) as c_int
                - *src.offset((j as intptr_t + stride) as isize) as c_int);
            j += 1;
        }
        i += 1;
        src = src.offset(stride as isize);
    }
    return score;
}
#[no_mangle]
#[c2rust::src_loc = "725:1"]
unsafe extern "C" fn x264_10_field_vsad(
    mut h: *mut x264_t,
    mut mb_x: c_int,
    mut mb_y: c_int,
) -> c_int {
    let mut score_field: c_int = 0;
    let mut score_frame: c_int = 0;
    let mut stride: c_int = (*(*h).fenc).i_stride[0 as c_int as usize];
    let mut mb_stride: c_int = (*h).mb.i_mb_stride;
    let mut fenc: *mut pixel = (*(*h).fenc).plane[0 as c_int as usize]
        .offset((16 as c_int * (mb_x + mb_y * stride)) as isize);
    let mut mb_xy: c_int = mb_x + mb_y * mb_stride;
    let mut mbpair_height: c_int = if ((*h).param.i_height - mb_y * 16 as c_int) < 32 as c_int {
        (*h).param.i_height - mb_y * 16 as c_int
    } else {
        32 as c_int
    };
    score_frame =
        (*h).pixf.vsad.expect("non-null function pointer")(fenc, stride as intptr_t, mbpair_height);
    score_field = (*h).pixf.vsad.expect("non-null function pointer")(
        fenc,
        (stride * 2 as c_int) as intptr_t,
        mbpair_height >> 1 as c_int,
    );
    score_field += (*h).pixf.vsad.expect("non-null function pointer")(
        fenc.offset(stride as isize),
        (stride * 2 as c_int) as intptr_t,
        mbpair_height >> 1 as c_int,
    );
    if mb_x > 0 as c_int {
        score_field += 512 as c_int
            - *(*h).mb.field.offset((mb_xy - 1 as c_int) as isize) as c_int * 1024 as c_int;
    }
    if mb_y > 0 as c_int {
        score_field += 512 as c_int
            - *(*h).mb.field.offset((mb_xy - mb_stride) as isize) as c_int * 1024 as c_int;
    }
    return (score_field < score_frame) as c_int;
}
#[c2rust::src_loc = "747:1"]
unsafe extern "C" fn pixel_asd8(
    mut pix1: *mut pixel,
    mut stride1: intptr_t,
    mut pix2: *mut pixel,
    mut stride2: intptr_t,
    mut height: c_int,
) -> c_int {
    let mut sum: c_int = 0 as c_int;
    let mut y: c_int = 0 as c_int;
    while y < height {
        let mut x: c_int = 0 as c_int;
        while x < 8 as c_int {
            sum += *pix1.offset(x as isize) as c_int - *pix2.offset(x as isize) as c_int;
            x += 1;
        }
        y += 1;
        pix1 = pix1.offset(stride1 as isize);
        pix2 = pix2.offset(stride2 as isize);
    }
    return abs(sum);
}
#[c2rust::src_loc = "759:1"]
unsafe extern "C" fn x264_pixel_ads4(
    mut enc_dc: *mut c_int,
    mut sums: *mut uint16_t,
    mut delta: c_int,
    mut cost_mvx: *mut uint16_t,
    mut mvs: *mut int16_t,
    mut width: c_int,
    mut thresh: c_int,
) -> c_int {
    let mut nmv: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < width {
        let mut ads: c_int =
            abs(*enc_dc.offset(0 as c_int as isize) - *sums.offset(0 as c_int as isize) as c_int)
                + abs(*enc_dc.offset(1 as c_int as isize)
                    - *sums.offset(8 as c_int as isize) as c_int)
                + abs(*enc_dc.offset(2 as c_int as isize) - *sums.offset(delta as isize) as c_int)
                + abs(*enc_dc.offset(3 as c_int as isize)
                    - *sums.offset((delta + 8 as c_int) as isize) as c_int)
                + *cost_mvx.offset(i as isize) as c_int;
        if ads < thresh {
            let fresh2 = nmv;
            nmv = nmv + 1;
            *mvs.offset(fresh2 as isize) = i as int16_t;
        }
        i += 1;
        sums = sums.offset(1);
    }
    return nmv;
}
#[c2rust::src_loc = "776:1"]
unsafe extern "C" fn x264_pixel_ads2(
    mut enc_dc: *mut c_int,
    mut sums: *mut uint16_t,
    mut delta: c_int,
    mut cost_mvx: *mut uint16_t,
    mut mvs: *mut int16_t,
    mut width: c_int,
    mut thresh: c_int,
) -> c_int {
    let mut nmv: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < width {
        let mut ads: c_int =
            abs(*enc_dc.offset(0 as c_int as isize) - *sums.offset(0 as c_int as isize) as c_int)
                + abs(*enc_dc.offset(1 as c_int as isize) - *sums.offset(delta as isize) as c_int)
                + *cost_mvx.offset(i as isize) as c_int;
        if ads < thresh {
            let fresh1 = nmv;
            nmv = nmv + 1;
            *mvs.offset(fresh1 as isize) = i as int16_t;
        }
        i += 1;
        sums = sums.offset(1);
    }
    return nmv;
}
#[c2rust::src_loc = "791:1"]
unsafe extern "C" fn x264_pixel_ads1(
    mut enc_dc: *mut c_int,
    mut sums: *mut uint16_t,
    mut _delta: c_int,
    mut cost_mvx: *mut uint16_t,
    mut mvs: *mut int16_t,
    mut width: c_int,
    mut thresh: c_int,
) -> c_int {
    let mut nmv: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < width {
        let mut ads: c_int =
            abs(*enc_dc.offset(0 as c_int as isize) - *sums.offset(0 as c_int as isize) as c_int)
                + *cost_mvx.offset(i as isize) as c_int;
        if ads < thresh {
            let fresh0 = nmv;
            nmv = nmv + 1;
            *mvs.offset(fresh0 as isize) = i as int16_t;
        }
        i += 1;
        sums = sums.offset(1);
    }
    return nmv;
}
#[no_mangle]
#[c2rust::src_loc = "809:1"]
unsafe extern "C" fn x264_10_pixel_init(mut _cpu: uint32_t, mut pixf: *mut x264_pixel_function_t) {
    memset(
        pixf as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<x264_pixel_function_t>() as size_t,
    );
    (*pixf).sad[PIXEL_16x16 as c_int as usize] = Some(
        x264_pixel_sad_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_16x8 as c_int as usize] = Some(
        x264_pixel_sad_16x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_8x16 as c_int as usize] = Some(
        x264_pixel_sad_8x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_8x8 as c_int as usize] = Some(
        x264_pixel_sad_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_8x4 as c_int as usize] = Some(
        x264_pixel_sad_8x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_4x8 as c_int as usize] = Some(
        x264_pixel_sad_4x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_4x4 as c_int as usize] = Some(
        x264_pixel_sad_4x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_4x16 as c_int as usize] = Some(
        x264_pixel_sad_4x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_16x16 as c_int as usize] = Some(
        x264_pixel_sad_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_16x8 as c_int as usize] = Some(
        x264_pixel_sad_16x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_8x16 as c_int as usize] = Some(
        x264_pixel_sad_8x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_8x8 as c_int as usize] = Some(
        x264_pixel_sad_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_8x4 as c_int as usize] = Some(
        x264_pixel_sad_8x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_4x8 as c_int as usize] = Some(
        x264_pixel_sad_4x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_4x4 as c_int as usize] = Some(
        x264_pixel_sad_4x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_4x16 as c_int as usize] = Some(
        x264_pixel_sad_4x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_x3[PIXEL_16x16 as c_int as usize] = Some(
        x264_pixel_sad_x3_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_16x8 as c_int as usize] = Some(
        x264_pixel_sad_x3_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_8x16 as c_int as usize] = Some(
        x264_pixel_sad_x3_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_8x8 as c_int as usize] = Some(
        x264_pixel_sad_x3_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_8x4 as c_int as usize] = Some(
        x264_pixel_sad_x3_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_4x8 as c_int as usize] = Some(
        x264_pixel_sad_x3_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_4x4 as c_int as usize] = Some(
        x264_pixel_sad_x3_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x4[PIXEL_16x16 as c_int as usize] = Some(
        x264_pixel_sad_x4_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_16x8 as c_int as usize] = Some(
        x264_pixel_sad_x4_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_8x16 as c_int as usize] = Some(
        x264_pixel_sad_x4_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_8x8 as c_int as usize] = Some(
        x264_pixel_sad_x4_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_8x4 as c_int as usize] = Some(
        x264_pixel_sad_x4_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_4x8 as c_int as usize] = Some(
        x264_pixel_sad_x4_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_4x4 as c_int as usize] = Some(
        x264_pixel_sad_x4_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).ssd[PIXEL_16x16 as c_int as usize] = Some(
        x264_pixel_ssd_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_16x8 as c_int as usize] = Some(
        x264_pixel_ssd_16x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_8x16 as c_int as usize] = Some(
        x264_pixel_ssd_8x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_8x8 as c_int as usize] = Some(
        x264_pixel_ssd_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_8x4 as c_int as usize] = Some(
        x264_pixel_ssd_8x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_4x8 as c_int as usize] = Some(
        x264_pixel_ssd_4x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_4x4 as c_int as usize] = Some(
        x264_pixel_ssd_4x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_4x16 as c_int as usize] = Some(
        x264_pixel_ssd_4x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_16x16 as c_int as usize] = Some(
        x264_pixel_satd_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_16x8 as c_int as usize] = Some(
        x264_pixel_satd_16x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_8x16 as c_int as usize] = Some(
        x264_pixel_satd_8x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_8x8 as c_int as usize] = Some(
        x264_pixel_satd_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_8x4 as c_int as usize] = Some(
        x264_pixel_satd_8x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_4x8 as c_int as usize] = Some(
        x264_pixel_satd_4x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_4x4 as c_int as usize] = Some(
        x264_pixel_satd_4x4
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_4x16 as c_int as usize] = Some(
        x264_pixel_satd_4x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd_x3[PIXEL_16x16 as c_int as usize] = Some(
        x264_pixel_satd_x3_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_16x8 as c_int as usize] = Some(
        x264_pixel_satd_x3_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_8x16 as c_int as usize] = Some(
        x264_pixel_satd_x3_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_8x8 as c_int as usize] = Some(
        x264_pixel_satd_x3_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_8x4 as c_int as usize] = Some(
        x264_pixel_satd_x3_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_4x8 as c_int as usize] = Some(
        x264_pixel_satd_x3_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_4x4 as c_int as usize] = Some(
        x264_pixel_satd_x3_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x4[PIXEL_16x16 as c_int as usize] = Some(
        x264_pixel_satd_x4_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_16x8 as c_int as usize] = Some(
        x264_pixel_satd_x4_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_8x16 as c_int as usize] = Some(
        x264_pixel_satd_x4_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_8x8 as c_int as usize] = Some(
        x264_pixel_satd_x4_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_8x4 as c_int as usize] = Some(
        x264_pixel_satd_x4_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_4x8 as c_int as usize] = Some(
        x264_pixel_satd_x4_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_4x4 as c_int as usize] = Some(
        x264_pixel_satd_x4_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).hadamard_ac[PIXEL_16x16 as c_int as usize] = Some(
        x264_pixel_hadamard_ac_16x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    )
        as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).hadamard_ac[PIXEL_16x8 as c_int as usize] =
        Some(x264_pixel_hadamard_ac_16x8 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t)
            as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).hadamard_ac[PIXEL_8x16 as c_int as usize] =
        Some(x264_pixel_hadamard_ac_8x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t)
            as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).hadamard_ac[PIXEL_8x8 as c_int as usize] =
        Some(x264_pixel_hadamard_ac_8x8 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t)
            as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).ads[PIXEL_16x16 as c_int as usize] = Some(
        x264_pixel_ads4
            as unsafe extern "C" fn(
                *mut c_int,
                *mut uint16_t,
                c_int,
                *mut uint16_t,
                *mut int16_t,
                c_int,
                c_int,
            ) -> c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut c_int,
                *mut uint16_t,
                c_int,
                *mut uint16_t,
                *mut int16_t,
                c_int,
                c_int,
            ) -> c_int,
        >;
    (*pixf).ads[PIXEL_16x8 as c_int as usize] = Some(
        x264_pixel_ads2
            as unsafe extern "C" fn(
                *mut c_int,
                *mut uint16_t,
                c_int,
                *mut uint16_t,
                *mut int16_t,
                c_int,
                c_int,
            ) -> c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut c_int,
                *mut uint16_t,
                c_int,
                *mut uint16_t,
                *mut int16_t,
                c_int,
                c_int,
            ) -> c_int,
        >;
    (*pixf).ads[PIXEL_8x8 as c_int as usize] = Some(
        x264_pixel_ads1
            as unsafe extern "C" fn(
                *mut c_int,
                *mut uint16_t,
                c_int,
                *mut uint16_t,
                *mut int16_t,
                c_int,
                c_int,
            ) -> c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut c_int,
                *mut uint16_t,
                c_int,
                *mut uint16_t,
                *mut int16_t,
                c_int,
                c_int,
            ) -> c_int,
        >;
    (*pixf).sa8d[PIXEL_16x16 as c_int as usize] = Some(
        x264_pixel_sa8d_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sa8d[PIXEL_8x8 as c_int as usize] = Some(
        x264_pixel_sa8d_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).var[PIXEL_16x16 as c_int as usize] =
        Some(pixel_var_16x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t)
            as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).var[PIXEL_8x16 as c_int as usize] =
        Some(pixel_var_8x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t)
            as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).var[PIXEL_8x8 as c_int as usize] =
        Some(pixel_var_8x8 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t)
            as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).var2[PIXEL_8x16 as c_int as usize] =
        Some(pixel_var2_8x16 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> c_int)
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> c_int>;
    (*pixf).var2[PIXEL_8x8 as c_int as usize] =
        Some(pixel_var2_8x8 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> c_int)
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> c_int>;
    (*pixf).ssd_nv12_core = Some(
        pixel_ssd_nv12_core
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                c_int,
                c_int,
                *mut uint64_t,
                *mut uint64_t,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                c_int,
                c_int,
                *mut uint64_t,
                *mut uint64_t,
            ) -> (),
        >;
    (*pixf).ssim_4x4x2_core = Some(
        ssim_4x4x2_core
            as unsafe extern "C" fn(
                *const pixel,
                intptr_t,
                *const pixel,
                intptr_t,
                *mut [c_int; 4],
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *const pixel,
                intptr_t,
                *const pixel,
                intptr_t,
                *mut [c_int; 4],
            ) -> (),
        >;
    (*pixf).ssim_end4 =
        Some(ssim_end4 as unsafe extern "C" fn(*mut [c_int; 4], *mut [c_int; 4], c_int) -> c_float)
            as Option<unsafe extern "C" fn(*mut [c_int; 4], *mut [c_int; 4], c_int) -> c_float>;
    (*pixf).vsad = Some(pixel_vsad as unsafe extern "C" fn(*mut pixel, intptr_t, c_int) -> c_int)
        as Option<unsafe extern "C" fn(*mut pixel, intptr_t, c_int) -> c_int>;
    (*pixf).asd8 = Some(
        pixel_asd8
            as unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, c_int) -> c_int,
    )
        as Option<unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t, c_int) -> c_int>;
    (*pixf).intra_sad_x3_4x4 =
        Some(intra_sad_x3_4x4 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ())
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ()>;
    (*pixf).intra_satd_x3_4x4 =
        Some(intra_satd_x3_4x4 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ())
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ()>;
    (*pixf).intra_sad_x3_8x8 =
        Some(intra_sad_x3_8x8 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ())
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ()>;
    (*pixf).intra_sa8d_x3_8x8 =
        Some(intra_sa8d_x3_8x8 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ())
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ()>;
    (*pixf).intra_sad_x3_8x8c =
        Some(intra_sad_x3_8x8c as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ())
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ()>;
    (*pixf).intra_satd_x3_8x8c =
        Some(intra_satd_x3_8x8c as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ())
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ()>;
    (*pixf).intra_sad_x3_8x16c =
        Some(intra_sad_x3_8x16c as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ())
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ()>;
    (*pixf).intra_satd_x3_8x16c =
        Some(intra_satd_x3_8x16c as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ())
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ()>;
    (*pixf).intra_sad_x3_16x16 =
        Some(intra_sad_x3_16x16 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ())
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ()>;
    (*pixf).intra_satd_x3_16x16 =
        Some(intra_satd_x3_16x16 as unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ())
            as Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut c_int) -> ()>;
    (*pixf).ads[PIXEL_4x8 as c_int as usize] = (*pixf).ads[PIXEL_16x8 as c_int as usize];
    (*pixf).ads[PIXEL_8x4 as c_int as usize] = (*pixf).ads[PIXEL_4x8 as c_int as usize];
    (*pixf).ads[PIXEL_8x16 as c_int as usize] = (*pixf).ads[PIXEL_8x4 as c_int as usize];
    (*pixf).ads[PIXEL_4x4 as c_int as usize] = (*pixf).ads[PIXEL_8x8 as c_int as usize];
}
