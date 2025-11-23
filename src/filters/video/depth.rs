use ::core::mem::size_of;
use core::ffi::{c_char, c_float, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::base_h::{x264_clip3, x264_free, x264_malloc};
use crate::common_h::{pixel, SIZEOF_PIXEL};
use crate::filters_h::{x264_get_option, x264_otoi};
use crate::input_h::{
    cli_image_t, cli_pic_t, video_info_t, x264_cli_csp_depth_factor, x264_cli_csps,
    x264_cli_pic_alloc, x264_cli_pic_clean,
};
use crate::internal::BIT_DEPTH;
use crate::src::filters::filters::x264_split_options;
use crate::src::x264::x264_cli_log;
use crate::stdint_intn_h::{int16_t, int64_t};
use crate::stdint_uintn_h::{uint16_t, uint8_t};
use crate::stdlib_h::free;
use crate::string_h::memset;
use crate::video_h::cli_vid_filter_t;
use crate::x264_h::{
    x264_param_t, X264_CSP_BGR, X264_CSP_BGRA, X264_CSP_HIGH_DEPTH, X264_CSP_I400, X264_CSP_I420,
    X264_CSP_I422, X264_CSP_I444, X264_CSP_MASK, X264_CSP_NV12, X264_CSP_NV16, X264_CSP_NV21,
    X264_CSP_RGB, X264_CSP_YV12, X264_CSP_YV16, X264_CSP_YV24, X264_LOG_ERROR,
};
use crate::x264cli_h::hnd_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "40:9"]
struct depth_hnd_t {
    prev_hnd: hnd_t,
    prev_filter: cli_vid_filter_t,
    bit_depth: c_int,
    dst_csp: c_int,
    buffer: cli_pic_t,
    error_buf: *mut int16_t,
}
#[c2rust::src_loc = "33:9"]
const NAME: [c_char; 9] = unsafe { ::core::mem::transmute::<[u8; 9], [c_char; 9]>(*b"depth_10\0") };
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn depth_filter_csp_is_supported(mut csp: c_int) -> c_int {
    let mut csp_mask: c_int = csp & X264_CSP_MASK;
    return (csp_mask == X264_CSP_I400
        || csp_mask == X264_CSP_I420
        || csp_mask == X264_CSP_I422
        || csp_mask == X264_CSP_I444
        || csp_mask == X264_CSP_YV12
        || csp_mask == X264_CSP_YV16
        || csp_mask == X264_CSP_YV24
        || csp_mask == X264_CSP_NV12
        || csp_mask == X264_CSP_NV21
        || csp_mask == X264_CSP_NV16
        || csp_mask == X264_CSP_BGR
        || csp_mask == X264_CSP_RGB
        || csp_mask == X264_CSP_BGRA) as c_int;
}
#[c2rust::src_loc = "69:1"]
unsafe extern "C" fn csp_num_interleaved(mut csp: c_int, mut plane: c_int) -> c_int {
    let mut csp_mask: c_int = csp & X264_CSP_MASK;
    return if (csp_mask == X264_CSP_NV12 || csp_mask == X264_CSP_NV21 || csp_mask == X264_CSP_NV16)
        && plane == 1 as c_int
    {
        2 as c_int
    } else if csp_mask == X264_CSP_BGR || csp_mask == X264_CSP_RGB {
        3 as c_int
    } else if csp_mask == X264_CSP_BGRA {
        4 as c_int
    } else {
        1 as c_int
    };
}
#[c2rust::src_loc = "103:1"]
unsafe extern "C" fn dither_plane_1(
    mut dst: *mut pixel,
    mut dst_stride: c_int,
    mut src: *mut uint16_t,
    mut src_stride: c_int,
    mut width: c_int,
    mut height: c_int,
    mut errors: *mut int16_t,
) {
    let lshift: c_int = 16 as c_int - BIT_DEPTH;
    let rshift: c_int = 16 as c_int - BIT_DEPTH + 2 as c_int;
    let half: c_int = (1 as c_int) << 16 as c_int - BIT_DEPTH + 1 as c_int;
    let pixel_max: c_int = ((1 as c_int) << BIT_DEPTH) - 1 as c_int;
    memset(
        errors as *mut c_void,
        0 as c_int,
        ((width + 1 as c_int) as size_t).wrapping_mul(size_of::<int16_t>() as size_t),
    );
    let mut y: c_int = 0 as c_int;
    while y < height {
        let mut err: c_int = 0 as c_int;
        let mut x: c_int = 0 as c_int;
        while x < width {
            err = err * 2 as c_int
                + *errors.offset(x as isize) as c_int
                + *errors.offset((x + 1 as c_int) as isize) as c_int;
            *dst.offset((x * 1 as c_int) as isize) = x264_clip3(
                ((*src.offset((x * 1 as c_int) as isize) as c_int) << 2 as c_int) + err + half
                    >> rshift,
                0 as c_int,
                pixel_max,
            ) as pixel;
            err = *src.offset((x * 1 as c_int) as isize) as c_int
                - ((*dst.offset((x * 1 as c_int) as isize) as c_int) << lshift);
            *errors.offset(x as isize) = err as int16_t;
            x += 1;
        }
        y += 1;
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
    }
}
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn dither_plane_2(
    mut dst: *mut pixel,
    mut dst_stride: c_int,
    mut src: *mut uint16_t,
    mut src_stride: c_int,
    mut width: c_int,
    mut height: c_int,
    mut errors: *mut int16_t,
) {
    let lshift: c_int = 16 as c_int - BIT_DEPTH;
    let rshift: c_int = 16 as c_int - BIT_DEPTH + 2 as c_int;
    let half: c_int = (1 as c_int) << 16 as c_int - BIT_DEPTH + 1 as c_int;
    let pixel_max: c_int = ((1 as c_int) << BIT_DEPTH) - 1 as c_int;
    memset(
        errors as *mut c_void,
        0 as c_int,
        ((width + 1 as c_int) as size_t).wrapping_mul(size_of::<int16_t>() as size_t),
    );
    let mut y: c_int = 0 as c_int;
    while y < height {
        let mut err: c_int = 0 as c_int;
        let mut x: c_int = 0 as c_int;
        while x < width {
            err = err * 2 as c_int
                + *errors.offset(x as isize) as c_int
                + *errors.offset((x + 1 as c_int) as isize) as c_int;
            *dst.offset((x * 2 as c_int) as isize) = x264_clip3(
                ((*src.offset((x * 2 as c_int) as isize) as c_int) << 2 as c_int) + err + half
                    >> rshift,
                0 as c_int,
                pixel_max,
            ) as pixel;
            err = *src.offset((x * 2 as c_int) as isize) as c_int
                - ((*dst.offset((x * 2 as c_int) as isize) as c_int) << lshift);
            *errors.offset(x as isize) = err as int16_t;
            x += 1;
        }
        y += 1;
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
    }
}
#[c2rust::src_loc = "105:1"]
unsafe extern "C" fn dither_plane_3(
    mut dst: *mut pixel,
    mut dst_stride: c_int,
    mut src: *mut uint16_t,
    mut src_stride: c_int,
    mut width: c_int,
    mut height: c_int,
    mut errors: *mut int16_t,
) {
    let lshift: c_int = 16 as c_int - BIT_DEPTH;
    let rshift: c_int = 16 as c_int - BIT_DEPTH + 2 as c_int;
    let half: c_int = (1 as c_int) << 16 as c_int - BIT_DEPTH + 1 as c_int;
    let pixel_max: c_int = ((1 as c_int) << BIT_DEPTH) - 1 as c_int;
    memset(
        errors as *mut c_void,
        0 as c_int,
        ((width + 1 as c_int) as size_t).wrapping_mul(size_of::<int16_t>() as size_t),
    );
    let mut y: c_int = 0 as c_int;
    while y < height {
        let mut err: c_int = 0 as c_int;
        let mut x: c_int = 0 as c_int;
        while x < width {
            err = err * 2 as c_int
                + *errors.offset(x as isize) as c_int
                + *errors.offset((x + 1 as c_int) as isize) as c_int;
            *dst.offset((x * 3 as c_int) as isize) = x264_clip3(
                ((*src.offset((x * 3 as c_int) as isize) as c_int) << 2 as c_int) + err + half
                    >> rshift,
                0 as c_int,
                pixel_max,
            ) as pixel;
            err = *src.offset((x * 3 as c_int) as isize) as c_int
                - ((*dst.offset((x * 3 as c_int) as isize) as c_int) << lshift);
            *errors.offset(x as isize) = err as int16_t;
            x += 1;
        }
        y += 1;
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
    }
}
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn dither_plane_4(
    mut dst: *mut pixel,
    mut dst_stride: c_int,
    mut src: *mut uint16_t,
    mut src_stride: c_int,
    mut width: c_int,
    mut height: c_int,
    mut errors: *mut int16_t,
) {
    let lshift: c_int = 16 as c_int - BIT_DEPTH;
    let rshift: c_int = 16 as c_int - BIT_DEPTH + 2 as c_int;
    let half: c_int = (1 as c_int) << 16 as c_int - BIT_DEPTH + 1 as c_int;
    let pixel_max: c_int = ((1 as c_int) << BIT_DEPTH) - 1 as c_int;
    memset(
        errors as *mut c_void,
        0 as c_int,
        ((width + 1 as c_int) as size_t).wrapping_mul(size_of::<int16_t>() as size_t),
    );
    let mut y: c_int = 0 as c_int;
    while y < height {
        let mut err: c_int = 0 as c_int;
        let mut x: c_int = 0 as c_int;
        while x < width {
            err = err * 2 as c_int
                + *errors.offset(x as isize) as c_int
                + *errors.offset((x + 1 as c_int) as isize) as c_int;
            *dst.offset((x * 4 as c_int) as isize) = x264_clip3(
                ((*src.offset((x * 4 as c_int) as isize) as c_int) << 2 as c_int) + err + half
                    >> rshift,
                0 as c_int,
                pixel_max,
            ) as pixel;
            err = *src.offset((x * 4 as c_int) as isize) as c_int
                - ((*dst.offset((x * 4 as c_int) as isize) as c_int) << lshift);
            *errors.offset(x as isize) = err as int16_t;
            x += 1;
        }
        y += 1;
        src = src.offset(src_stride as isize);
        dst = dst.offset(dst_stride as isize);
    }
}
#[c2rust::src_loc = "108:1"]
unsafe extern "C" fn dither_image(
    mut out: *mut cli_image_t,
    mut img: *mut cli_image_t,
    mut error_buf: *mut int16_t,
) {
    let mut csp_mask: c_int = (*img).csp & X264_CSP_MASK;
    let mut i: c_int = 0 as c_int;
    while i < (*img).planes {
        let mut num_interleaved: c_int = csp_num_interleaved((*img).csp, i);
        let mut height: c_int = ((*x264_cli_csps.as_ptr().offset(csp_mask as isize)).height
            [i as usize]
            * (*img).height as c_float) as c_int;
        let mut width: c_int = ((*x264_cli_csps.as_ptr().offset(csp_mask as isize)).width
            [i as usize]
            * (*img).width as c_float
            / num_interleaved as c_float) as c_int;
        if num_interleaved == 4 as c_int {
            dither_plane_4(
                ((*out).plane[i as usize] as *mut pixel).offset(0),
                (*out).stride[i as usize] / SIZEOF_PIXEL,
                ((*img).plane[i as usize] as *mut uint16_t).offset(0),
                (*img).stride[i as usize] / 2 as c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_4(
                ((*out).plane[i as usize] as *mut pixel).offset(1),
                (*out).stride[i as usize] / SIZEOF_PIXEL,
                ((*img).plane[i as usize] as *mut uint16_t).offset(1),
                (*img).stride[i as usize] / 2 as c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_4(
                ((*out).plane[i as usize] as *mut pixel).offset(2),
                (*out).stride[i as usize] / SIZEOF_PIXEL,
                ((*img).plane[i as usize] as *mut uint16_t).offset(2),
                (*img).stride[i as usize] / 2 as c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_4(
                ((*out).plane[i as usize] as *mut pixel).offset(3),
                (*out).stride[i as usize] / SIZEOF_PIXEL,
                ((*img).plane[i as usize] as *mut uint16_t).offset(3),
                (*img).stride[i as usize] / 2 as c_int,
                width,
                height,
                error_buf,
            );
        } else if num_interleaved == 3 as c_int {
            dither_plane_3(
                ((*out).plane[i as usize] as *mut pixel).offset(0),
                (*out).stride[i as usize] / SIZEOF_PIXEL,
                ((*img).plane[i as usize] as *mut uint16_t).offset(0),
                (*img).stride[i as usize] / 2 as c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_3(
                ((*out).plane[i as usize] as *mut pixel).offset(1),
                (*out).stride[i as usize] / SIZEOF_PIXEL,
                ((*img).plane[i as usize] as *mut uint16_t).offset(1),
                (*img).stride[i as usize] / 2 as c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_3(
                ((*out).plane[i as usize] as *mut pixel).offset(2),
                (*out).stride[i as usize] / SIZEOF_PIXEL,
                ((*img).plane[i as usize] as *mut uint16_t).offset(2),
                (*img).stride[i as usize] / 2 as c_int,
                width,
                height,
                error_buf,
            );
        } else if num_interleaved == 2 as c_int {
            dither_plane_2(
                ((*out).plane[i as usize] as *mut pixel).offset(0),
                (*out).stride[i as usize] / SIZEOF_PIXEL,
                ((*img).plane[i as usize] as *mut uint16_t).offset(0),
                (*img).stride[i as usize] / 2 as c_int,
                width,
                height,
                error_buf,
            );
            dither_plane_2(
                ((*out).plane[i as usize] as *mut pixel).offset(1),
                (*out).stride[i as usize] / SIZEOF_PIXEL,
                ((*img).plane[i as usize] as *mut uint16_t).offset(1),
                (*img).stride[i as usize] / 2 as c_int,
                width,
                height,
                error_buf,
            );
        } else {
            dither_plane_1(
                ((*out).plane[i as usize] as *mut pixel).offset(0),
                (*out).stride[i as usize] / SIZEOF_PIXEL,
                ((*img).plane[i as usize] as *mut uint16_t).offset(0),
                (*img).stride[i as usize] / 2 as c_int,
                width,
                height,
                error_buf,
            );
        }
        i += 1;
    }
}
#[c2rust::src_loc = "146:1"]
unsafe extern "C" fn scale_image(mut output: *mut cli_image_t, mut img: *mut cli_image_t) {
    let mut csp_mask: c_int = (*img).csp & X264_CSP_MASK;
    let shift: c_int = BIT_DEPTH - 8 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < (*img).planes {
        let mut src: *mut uint8_t = (*img).plane[i as usize];
        let mut dst: *mut uint16_t = (*output).plane[i as usize] as *mut uint16_t;
        let mut height: c_int = ((*x264_cli_csps.as_ptr().offset(csp_mask as isize)).height
            [i as usize]
            * (*img).height as c_float) as c_int;
        let mut width: c_int = ((*x264_cli_csps.as_ptr().offset(csp_mask as isize)).width
            [i as usize]
            * (*img).width as c_float) as c_int;
        let mut j: c_int = 0 as c_int;
        while j < height {
            let mut k: c_int = 0 as c_int;
            while k < width {
                *dst.offset(k as isize) = ((*src.offset(k as isize) as c_int) << shift) as uint16_t;
                k += 1;
            }
            src = src.offset((*img).stride[i as usize] as isize);
            dst = dst.offset(((*output).stride[i as usize] / 2 as c_int) as isize);
            j += 1;
        }
        i += 1;
    }
}
#[c2rust::src_loc = "168:1"]
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut depth_hnd_t = handle as *mut depth_hnd_t;
    if (*h)
        .prev_filter
        .get_frame
        .expect("non-null function pointer")((*h).prev_hnd, output, frame)
        != 0
    {
        return -1;
    }
    if (*h).bit_depth < 16 as c_int && (*output).img.csp & X264_CSP_HIGH_DEPTH != 0 {
        dither_image(&mut (*h).buffer.img, &mut (*output).img, (*h).error_buf);
        (*output).img = (*h).buffer.img;
    } else if (*h).bit_depth > 8 as c_int && (*output).img.csp & X264_CSP_HIGH_DEPTH == 0 {
        scale_image(&mut (*h).buffer.img, &mut (*output).img);
        (*output).img = (*h).buffer.img;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "188:1"]
unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut depth_hnd_t = handle as *mut depth_hnd_t;
    return (*h)
        .prev_filter
        .release_frame
        .expect("non-null function pointer")((*h).prev_hnd, pic, frame);
}
#[c2rust::src_loc = "194:1"]
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut depth_hnd_t = handle as *mut depth_hnd_t;
    (*h).prev_filter.free.expect("non-null function pointer")((*h).prev_hnd);
    x264_cli_pic_clean(&mut (*h).buffer);
    x264_free(h as *mut c_void);
}
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut c_char,
) -> c_int {
    let mut ret: c_int = 0 as c_int;
    let mut change_fmt: c_int = ((*info).csp ^ (*param).i_csp) & X264_CSP_HIGH_DEPTH;
    let mut csp: c_int = !(!(*info).csp ^ change_fmt);
    let mut bit_depth: c_int = 8 as c_int * x264_cli_csp_depth_factor(csp);
    if !opt_string.is_null() {
        static mut optlist: [*const c_char; 2] = [
            b"bit_depth\0" as *const u8 as *const c_char,
            0 as *const c_char,
        ];
        let mut opts: *mut *mut c_char = x264_split_options(opt_string, optlist.as_ptr());
        if !opts.is_null() {
            let mut str_bit_depth: *mut c_char =
                x264_get_option(b"bit_depth\0" as *const u8 as *const c_char, opts);
            bit_depth = x264_otoi(str_bit_depth, -1);
            ret = (bit_depth < 8 as c_int || bit_depth > 16 as c_int) as c_int;
            csp = if bit_depth > 8 as c_int {
                csp | X264_CSP_HIGH_DEPTH
            } else {
                csp & !X264_CSP_HIGH_DEPTH
            };
            change_fmt = ((*info).csp ^ csp) & X264_CSP_HIGH_DEPTH;
            free(opts as *mut c_void);
        } else {
            ret = 1 as c_int;
        }
    }
    if bit_depth != 10 as c_int {
        x264_cli_log(
            b"depth_10\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"this filter supports only bit depth %d\n\0" as *const u8 as *const c_char,
            10 as c_int,
        );
        return -1;
    }
    if ret != 0 {
        x264_cli_log(
            b"depth_10\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"unsupported bit depth conversion.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    if change_fmt != 0 || bit_depth != 8 as c_int * x264_cli_csp_depth_factor(csp) {
        if depth_filter_csp_is_supported(csp) == 0 {
            x264_cli_log(
                b"depth_10\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"unsupported colorspace.\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
        let mut h: *mut depth_hnd_t = x264_malloc((size_of::<depth_hnd_t>() as usize).wrapping_add(
            (((*info).width + 1) as usize).wrapping_mul(size_of::<int16_t>() as usize),
        ) as int64_t) as *mut depth_hnd_t;
        if h.is_null() {
            return -1;
        }
        (*h).error_buf = h.offset(1) as *mut int16_t;
        (*h).dst_csp = csp;
        (*h).bit_depth = bit_depth;
        (*h).prev_hnd = *handle;
        (*h).prev_filter = *filter;
        if x264_cli_pic_alloc(
            &mut (*h).buffer,
            (*h).dst_csp,
            (*info).width as c_int,
            (*info).height as c_int,
        ) != 0
        {
            x264_free(h as *mut c_void);
            return -1;
        }
        *handle = h as hnd_t;
        *filter = depth_10_filter;
        (*info).csp = (*h).dst_csp;
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "261:18"]
static mut depth_10_filter: cli_vid_filter_t = cli_vid_filter_t {
    name: NAME.as_ptr(),
    help: None,
    init: Some(
        init as unsafe extern "C" fn(
            *mut hnd_t,
            *mut cli_vid_filter_t,
            *mut video_info_t,
            *mut x264_param_t,
            *mut c_char,
        ) -> c_int,
    ),
    get_frame: Some(get_frame as unsafe extern "C" fn(hnd_t, *mut cli_pic_t, c_int) -> c_int),
    release_frame: Some(
        release_frame as unsafe extern "C" fn(hnd_t, *mut cli_pic_t, c_int) -> c_int,
    ),
    free: Some(free_filter as unsafe extern "C" fn(hnd_t) -> ()),
    next: 0 as *const cli_vid_filter_t as *mut cli_vid_filter_t,
};
