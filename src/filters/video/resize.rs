use core::ffi::{c_char, c_double, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::base_h::{x264_reduce_fraction, x264_reduce_fraction64};
use crate::filters_h::{x264_get_option, x264_otoi, x264_otos, x264_split_options};
use crate::input_h::{
    cli_image_t, cli_pic_t, video_info_t, x264_cli_csp_depth_factor, x264_cli_csp_t, x264_cli_csps,
    x264_cli_get_csp, x264_cli_pic_alloc_aligned, x264_cli_pic_clean, X264_CSP_CLI_MAX,
    X264_CSP_OTHER,
};
use crate::limits_h::INT_MAX;
use crate::mathcalls_h::round;
use crate::opt_h::av_opt_set_int;
use crate::pixdesc_h::{
    av_get_pix_fmt_name, av_pix_fmt_desc_get, AVPixFmtDescriptor, AV_PIX_FMT_FLAG_PAL,
    AV_PIX_FMT_FLAG_RGB,
};
use crate::pixfmt_h::{
    AVPixelFormat, AV_PIX_FMT_BGR24, AV_PIX_FMT_BGR48LE, AV_PIX_FMT_BGRA, AV_PIX_FMT_BGRA64LE,
    AV_PIX_FMT_GRAY16LE, AV_PIX_FMT_GRAY8, AV_PIX_FMT_NONE, AV_PIX_FMT_NV12, AV_PIX_FMT_NV21,
    AV_PIX_FMT_RGB24, AV_PIX_FMT_RGB48LE, AV_PIX_FMT_UYVY422, AV_PIX_FMT_YUV420P,
    AV_PIX_FMT_YUV420P16LE, AV_PIX_FMT_YUV422P, AV_PIX_FMT_YUV422P16LE, AV_PIX_FMT_YUV444P,
    AV_PIX_FMT_YUV444P16LE, AV_PIX_FMT_YUYV422,
};
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::{uint32_t, uint64_t, uint8_t};
use crate::stdio_h::{printf, sscanf};
use crate::stdlib_h::{calloc, free};
use crate::string_h::{memcmp, strchr, strcmp, strlen, strstr};
use crate::strings_h::strcasecmp;
use crate::swscale_h::{
    sws_alloc_context, sws_freeContext, sws_getCoefficients, sws_init_context,
    sws_isSupportedInput, sws_isSupportedOutput, sws_scale, sws_setColorspaceDetails, SwsContext,
    SwsFilter, SWS_ACCURATE_RND, SWS_AREA, SWS_BICUBIC, SWS_BICUBLIN, SWS_BILINEAR, SWS_CS_DEFAULT,
    SWS_FAST_BILINEAR, SWS_FULL_CHR_H_INP, SWS_FULL_CHR_H_INT, SWS_GAUSS, SWS_LANCZOS, SWS_POINT,
    SWS_SINC, SWS_SPLINE, SWS_X,
};
use crate::video_h::cli_vid_filter_t;
use crate::x264_h::{
    x264_param_t, X264_CSP_BGR, X264_CSP_BGRA, X264_CSP_HIGH_DEPTH, X264_CSP_I400, X264_CSP_I420,
    X264_CSP_I422, X264_CSP_I444, X264_CSP_MASK, X264_CSP_NONE, X264_CSP_NV12, X264_CSP_NV16,
    X264_CSP_NV21, X264_CSP_RGB, X264_CSP_UYVY, X264_CSP_VFLIP, X264_CSP_YUYV, X264_CSP_YV12,
    X264_CSP_YV16, X264_CSP_YV24, X264_LOG_ERROR, X264_LOG_INFO, X264_LOG_WARNING,
};
use crate::x264cli_h::{hnd_t, x264_cli_log};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "61:9"]
struct resizer_hnd_t {
    prev_hnd: hnd_t,
    prev_filter: cli_vid_filter_t,
    buffer: cli_pic_t,
    buffer_allocated: c_int,
    dst_csp: c_int,
    input_range: c_int,
    ctx: *mut SwsContext,
    ctx_flags: uint32_t,
    pre_swap_chroma: c_int,
    post_swap_chroma: c_int,
    fast_mono: c_int,
    variable_input: c_int,
    working: c_int,
    dst: frame_prop_t,
    scale: frame_prop_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "53:9"]
struct frame_prop_t {
    width: c_int,
    height: c_int,
    pix_fmt: c_int,
    range: c_int,
}
#[c2rust::src_loc = "28:9"]
const NAME: [c_char; 7] = unsafe { ::core::mem::transmute::<[u8; 7], [c_char; 7]>(*b"resize\0") };
#[c2rust::src_loc = "33:1"]
unsafe extern "C" fn full_check(
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
) -> c_int {
    let mut required: c_int = 0 as c_int;
    required |= ((*info).csp != (*param).i_csp) as c_int;
    required |= ((*info).width != (*param).i_width) as c_int;
    required |= ((*info).height != (*param).i_height) as c_int;
    required |= ((*info).fullrange != (*param).vui.b_fullrange) as c_int;
    return required;
}
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn help(mut longhelp: c_int) {
    printf(
        b"      resize:[width,height][,sar][,fittobox][,csp][,method]\n\0" as *const u8
            as *const c_char,
    );
    if longhelp == 0 {
        return;
    }
    printf(
        b"            resizes frames based on the given criteria:\n            - resolution only: resizes and adapts sar to avoid stretching\n            - sar only: sets the sar and resizes to avoid stretching\n            - resolution and sar: resizes to given resolution and sets the sar\n            - fittobox: resizes the video based on the desired constraints\n               - width, height, both\n            - fittobox and sar: same as above except with specified sar\n            - csp: convert to the given csp. syntax: [name][:depth]\n               - valid csp names [keep current]: \0"
            as *const u8 as *const c_char,
    );
    let mut i: c_int = X264_CSP_NONE + 1 as c_int;
    while i < X264_CSP_CLI_MAX {
        if !(*x264_cli_csps.as_ptr().offset(i as isize)).name.is_null() {
            printf(
                b"%s\0" as *const u8 as *const c_char,
                (*x264_cli_csps.as_ptr().offset(i as isize)).name,
            );
            if (i + 1 as c_int) < X264_CSP_CLI_MAX {
                printf(b", \0" as *const u8 as *const c_char);
            }
        }
        i += 1;
    }
    printf(
        b"\n               - depth: 8 or 16 bits per pixel [keep current]\n            note: not all depths are supported by all csps.\n            - method: use resizer method [\"bicubic\"]\n               - fastbilinear, bilinear, bicubic, experimental, point,\n               - area, bicublin, gauss, sinc, lanczos, spline\n\0"
            as *const u8 as *const c_char,
    );
}
#[c2rust::src_loc = "114:1"]
unsafe extern "C" fn convert_method_to_flag(mut name: *const c_char) -> uint32_t {
    let mut flag: uint32_t = 0 as uint32_t;
    if strcasecmp(name, b"fastbilinear\0" as *const u8 as *const c_char) == 0 {
        flag = SWS_FAST_BILINEAR as c_int as uint32_t;
    } else if strcasecmp(name, b"bilinear\0" as *const u8 as *const c_char) == 0 {
        flag = SWS_BILINEAR as c_int as uint32_t;
    } else if strcasecmp(name, b"bicubic\0" as *const u8 as *const c_char) == 0 {
        flag = SWS_BICUBIC as c_int as uint32_t;
    } else if strcasecmp(name, b"experimental\0" as *const u8 as *const c_char) == 0 {
        flag = SWS_X as c_int as uint32_t;
    } else if strcasecmp(name, b"point\0" as *const u8 as *const c_char) == 0 {
        flag = SWS_POINT as c_int as uint32_t;
    } else if strcasecmp(name, b"area\0" as *const u8 as *const c_char) == 0 {
        flag = SWS_AREA as c_int as uint32_t;
    } else if strcasecmp(name, b"bicublin\0" as *const u8 as *const c_char) == 0 {
        flag = SWS_BICUBLIN as c_int as uint32_t;
    } else if strcasecmp(name, b"gauss\0" as *const u8 as *const c_char) == 0 {
        flag = SWS_GAUSS as c_int as uint32_t;
    } else if strcasecmp(name, b"sinc\0" as *const u8 as *const c_char) == 0 {
        flag = SWS_SINC as c_int as uint32_t;
    } else if strcasecmp(name, b"lanczos\0" as *const u8 as *const c_char) == 0 {
        flag = SWS_LANCZOS as c_int as uint32_t;
    } else if strcasecmp(name, b"spline\0" as *const u8 as *const c_char) == 0 {
        flag = SWS_SPLINE as c_int as uint32_t;
    } else {
        flag = SWS_BICUBIC as c_int as uint32_t;
    }
    return flag;
}
#[c2rust::src_loc = "144:1"]
unsafe extern "C" fn convert_csp_to_pix_fmt(mut csp: c_int) -> c_int {
    if csp & X264_CSP_OTHER != 0 {
        return csp & X264_CSP_MASK;
    }
    match csp & X264_CSP_MASK {
        X264_CSP_I400 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_GRAY16LE as c_int
            } else {
                AV_PIX_FMT_GRAY8 as c_int
            };
        }
        X264_CSP_YV12 | X264_CSP_I420 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_YUV420P16LE as c_int
            } else {
                AV_PIX_FMT_YUV420P as c_int
            };
        }
        X264_CSP_YV16 | X264_CSP_I422 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_YUV422P16LE as c_int
            } else {
                AV_PIX_FMT_YUV422P as c_int
            };
        }
        X264_CSP_YV24 | X264_CSP_I444 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_YUV444P16LE as c_int
            } else {
                AV_PIX_FMT_YUV444P as c_int
            };
        }
        X264_CSP_RGB => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_RGB48LE as c_int
            } else {
                AV_PIX_FMT_RGB24 as c_int
            };
        }
        X264_CSP_BGR => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_BGR48LE as c_int
            } else {
                AV_PIX_FMT_BGR24 as c_int
            };
        }
        X264_CSP_BGRA => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_BGRA64LE as c_int
            } else {
                AV_PIX_FMT_BGRA as c_int
            };
        }
        X264_CSP_NV12 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_NONE as c_int
            } else {
                AV_PIX_FMT_NV12 as c_int
            };
        }
        X264_CSP_NV21 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_NONE as c_int
            } else {
                AV_PIX_FMT_NV21 as c_int
            };
        }
        X264_CSP_YUYV => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_NONE as c_int
            } else {
                AV_PIX_FMT_YUYV422 as c_int
            };
        }
        X264_CSP_UYVY => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_NONE as c_int
            } else {
                AV_PIX_FMT_UYVY422 as c_int
            };
        }
        X264_CSP_NV16 | _ => return AV_PIX_FMT_NONE as c_int,
    };
}
#[c2rust::src_loc = "171:1"]
unsafe extern "C" fn pix_number_of_planes(mut pix_desc: *const AVPixFmtDescriptor) -> c_int {
    let mut num_planes: c_int = 0 as c_int;
    let mut i: c_int = 0 as c_int;
    while i < (*pix_desc).nb_components as c_int {
        let mut plane_plus1: c_int = (*pix_desc).comp[i as usize].plane + 1 as c_int;
        num_planes = if plane_plus1 > num_planes {
            plane_plus1
        } else {
            num_planes
        };
        i += 1;
    }
    return num_planes;
}
#[c2rust::src_loc = "182:1"]
unsafe extern "C" fn pick_closest_supported_csp(mut csp: c_int) -> c_int {
    let mut pix_fmt: c_int = convert_csp_to_pix_fmt(csp);
    let mut ret: c_int = X264_CSP_NONE;
    let mut pix_desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(pix_fmt as AVPixelFormat);
    if pix_desc.is_null() || (*pix_desc).name.is_null() {
        return ret;
    }
    let mut pix_fmt_name: *const c_char = (*pix_desc).name;
    let mut is_rgb: c_int =
        ((*pix_desc).flags & (AV_PIX_FMT_FLAG_RGB | AV_PIX_FMT_FLAG_PAL) as uint64_t) as c_int;
    let mut is_bgr: c_int =
        !strstr(pix_fmt_name, b"bgr\0" as *const u8 as *const c_char).is_null() as c_int;
    if is_bgr != 0 || is_rgb != 0 {
        if (*pix_desc).nb_components as c_int == 4 as c_int {
            ret = X264_CSP_BGRA;
        } else if is_bgr != 0 {
            ret = X264_CSP_BGR;
        } else {
            ret = X264_CSP_RGB;
        }
    } else if (*pix_desc).nb_components as c_int == 1 as c_int
        || (*pix_desc).nb_components as c_int == 2 as c_int
    {
        ret = X264_CSP_I400;
    } else if (*pix_desc).log2_chroma_w as c_int != 0 && (*pix_desc).log2_chroma_h as c_int != 0 {
        ret = if pix_number_of_planes(pix_desc) == 2 as c_int {
            X264_CSP_NV12
        } else {
            X264_CSP_I420
        };
    } else if (*pix_desc).log2_chroma_w != 0 {
        ret = X264_CSP_I422;
    } else {
        ret = X264_CSP_I444;
    }
    let mut i: c_int = 0 as c_int;
    while i < (*pix_desc).nb_components as c_int {
        if (*pix_desc).comp[i as usize].depth > 8 as c_int {
            ret |= X264_CSP_HIGH_DEPTH;
        }
        i += 1;
    }
    return ret;
}
#[c2rust::src_loc = "222:1"]
unsafe extern "C" fn handle_opts(
    mut optlist: *const *const c_char,
    mut opts: *mut *mut c_char,
    mut info: *mut video_info_t,
    mut h: *mut resizer_hnd_t,
) -> c_int {
    let mut out_sar_w: uint32_t = 0;
    let mut out_sar_h: uint32_t = 0;
    let mut str_width: *mut c_char = x264_get_option(*optlist.offset(0 as c_int as isize), opts);
    let mut str_height: *mut c_char = x264_get_option(*optlist.offset(1 as c_int as isize), opts);
    let mut str_sar: *mut c_char = x264_get_option(*optlist.offset(2 as c_int as isize), opts);
    let mut fittobox: *mut c_char = x264_get_option(*optlist.offset(3 as c_int as isize), opts);
    let mut str_csp: *mut c_char = x264_get_option(*optlist.offset(4 as c_int as isize), opts);
    let mut width: c_int = x264_otoi(str_width, -(1 as c_int));
    let mut height: c_int = x264_otoi(str_height, -(1 as c_int));
    let mut csp_only: c_int = 0 as c_int;
    let mut in_sar_w: uint32_t = (*info).sar_width;
    let mut in_sar_h: uint32_t = (*info).sar_height;
    if !str_csp.is_null() {
        let mut str_depth: *mut c_char = strchr(str_csp, ':' as i32);
        let mut depth: c_int = x264_cli_csp_depth_factor((*info).csp) * 8 as c_int;
        if !str_depth.is_null() {
            let fresh0 = str_depth;
            str_depth = str_depth.offset(1);
            *fresh0 = '\0' as i32 as c_char;
            depth = x264_otoi(str_depth, -(1 as c_int));
            if depth != 8 as c_int && depth != 16 as c_int {
                x264_cli_log(
                    b"resize\0" as *const u8 as *const c_char,
                    X264_LOG_ERROR,
                    b"unsupported bit depth %d\n\0" as *const u8 as *const c_char,
                    depth,
                );
                return -(1 as c_int);
            }
        }
        let mut csp: c_int = 0;
        if strlen(str_csp) == 0 as size_t {
            csp = (*info).csp & X264_CSP_MASK;
        } else {
            csp = X264_CSP_CLI_MAX - 1 as c_int;
            while csp > X264_CSP_NONE {
                if !(*x264_cli_csps.as_ptr().offset(csp as isize))
                    .name
                    .is_null()
                    && strcasecmp((*x264_cli_csps.as_ptr().offset(csp as isize)).name, str_csp) == 0
                {
                    break;
                }
                csp -= 1;
            }
        }
        if csp == 0 as c_int {
            x264_cli_log(
                b"resize\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"unsupported colorspace `%s'\n\0" as *const u8 as *const c_char,
                str_csp,
            );
            return -(1 as c_int);
        }
        (*h).dst_csp = csp;
        if depth == 16 as c_int {
            (*h).dst_csp |= X264_CSP_HIGH_DEPTH;
        }
    }
    if in_sar_w == 0 || in_sar_h == 0 {
        in_sar_h = 1 as uint32_t;
        in_sar_w = in_sar_h;
    }
    if !str_sar.is_null() {
        if 2 as c_int
            != sscanf(
                str_sar,
                b"%u:%u\0" as *const u8 as *const c_char,
                &mut out_sar_w as *mut uint32_t,
                &mut out_sar_h as *mut uint32_t,
            )
            && 2 as c_int
                != sscanf(
                    str_sar,
                    b"%u/%u\0" as *const u8 as *const c_char,
                    &mut out_sar_w as *mut uint32_t,
                    &mut out_sar_h as *mut uint32_t,
                )
        {
            x264_cli_log(
                b"resize\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"invalid sar `%s'\n\0" as *const u8 as *const c_char,
                str_sar,
            );
            return -(1 as c_int);
        }
    } else {
        out_sar_h = 1 as uint32_t;
        out_sar_w = out_sar_h;
    }
    if !fittobox.is_null() {
        if strcasecmp(fittobox, b"both\0" as *const u8 as *const c_char) == 0 {
            if width <= 0 as c_int || height <= 0 as c_int {
                x264_cli_log(
                    b"resize\0" as *const u8 as *const c_char,
                    X264_LOG_ERROR,
                    b"invalid box resolution %sx%s\n\0" as *const u8 as *const c_char,
                    x264_otos(
                        str_width,
                        b"<unset>\0" as *const u8 as *const c_char as *mut c_char,
                    ),
                    x264_otos(
                        str_height,
                        b"<unset>\0" as *const u8 as *const c_char as *mut c_char,
                    ),
                );
                return -(1 as c_int);
            }
        } else if strcasecmp(fittobox, b"width\0" as *const u8 as *const c_char) == 0 {
            if width <= 0 as c_int {
                x264_cli_log(
                    b"resize\0" as *const u8 as *const c_char,
                    X264_LOG_ERROR,
                    b"invalid box width `%s'\n\0" as *const u8 as *const c_char,
                    x264_otos(
                        str_width,
                        b"<unset>\0" as *const u8 as *const c_char as *mut c_char,
                    ),
                );
                return -(1 as c_int);
            }
            height = INT_MAX;
        } else if strcasecmp(fittobox, b"height\0" as *const u8 as *const c_char) == 0 {
            if height <= 0 as c_int {
                x264_cli_log(
                    b"resize\0" as *const u8 as *const c_char,
                    X264_LOG_ERROR,
                    b"invalid box height `%s'\n\0" as *const u8 as *const c_char,
                    x264_otos(
                        str_height,
                        b"<unset>\0" as *const u8 as *const c_char as *mut c_char,
                    ),
                );
                return -(1 as c_int);
            }
            width = INT_MAX;
        } else {
            x264_cli_log(
                b"resize\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"invalid fittobox mode `%s'\n\0" as *const u8 as *const c_char,
                fittobox,
            );
            return -(1 as c_int);
        }
        let mut csp_0: *const x264_cli_csp_t = x264_cli_get_csp((*h).dst_csp);
        let mut width_units: c_double =
            (*info).height as c_double * in_sar_h as c_double * out_sar_w as c_double;
        let mut height_units: c_double =
            (*info).width as c_double * in_sar_w as c_double * out_sar_h as c_double;
        width = width / (*csp_0).mod_width * (*csp_0).mod_width;
        height = height / (*csp_0).mod_height * (*csp_0).mod_height;
        if width as c_double * width_units > height as c_double * height_units {
            let mut new_width: c_int = round(
                height as c_double * height_units / (width_units * (*csp_0).mod_width as c_double),
            ) as c_int;
            new_width *= (*csp_0).mod_width;
            width = if new_width < width { new_width } else { width };
        } else {
            let mut new_height: c_int = round(
                width as c_double * width_units / (height_units * (*csp_0).mod_height as c_double),
            ) as c_int;
            new_height *= (*csp_0).mod_height;
            height = if new_height < height {
                new_height
            } else {
                height
            };
        }
    } else if !str_width.is_null() || !str_height.is_null() {
        if width <= 0 as c_int || height <= 0 as c_int {
            x264_cli_log(
                b"resize\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"invalid resolution %sx%s\n\0" as *const u8 as *const c_char,
                x264_otos(
                    str_width,
                    b"<unset>\0" as *const u8 as *const c_char as *mut c_char,
                ),
                x264_otos(
                    str_height,
                    b"<unset>\0" as *const u8 as *const c_char as *mut c_char,
                ),
            );
            return -(1 as c_int);
        }
        if str_sar.is_null() {
            let mut num: uint64_t = ((*info).width as uint64_t).wrapping_mul(height as uint64_t);
            let mut den: uint64_t = ((*info).height as uint64_t).wrapping_mul(width as uint64_t);
            x264_reduce_fraction64(&mut num, &mut den);
            out_sar_w = num.wrapping_mul(in_sar_w as uint64_t) as uint32_t;
            out_sar_h = den.wrapping_mul(in_sar_h as uint64_t) as uint32_t;
            x264_reduce_fraction(&mut out_sar_w, &mut out_sar_h);
        }
    } else if !str_sar.is_null() {
        let mut csp_1: *const x264_cli_csp_t = x264_cli_get_csp((*h).dst_csp);
        let mut width_units_0: c_double = in_sar_h as c_double * out_sar_w as c_double;
        let mut height_units_0: c_double = in_sar_w as c_double * out_sar_h as c_double;
        width = (*info).width;
        height = (*info).height;
        if width_units_0 > height_units_0 {
            width = round(
                (*info).width as c_double * height_units_0
                    / (width_units_0 * (*csp_1).mod_width as c_double),
            ) as c_int;
            width *= (*csp_1).mod_width;
        } else {
            height = round(
                (*info).height as c_double * width_units_0
                    / (height_units_0 * (*csp_1).mod_height as c_double),
            ) as c_int;
            height *= (*csp_1).mod_height;
        }
    } else {
        (*h).dst.width = (*info).width;
        (*h).dst.height = (*info).height;
        csp_only = 1 as c_int;
    }
    if csp_only == 0 {
        (*info).sar_width = out_sar_w;
        (*info).sar_height = out_sar_h;
        (*h).dst.width = width;
        (*h).dst.height = height;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn init_sws_context(mut h: *mut resizer_hnd_t) -> c_int {
    if !(*h).ctx.is_null() {
        sws_freeContext((*h).ctx as *mut SwsContext);
    }
    (*h).ctx = sws_alloc_context() as *mut SwsContext;
    if (*h).ctx.is_null() {
        return -(1 as c_int);
    }
    av_opt_set_int(
        (*h).ctx as *mut c_void,
        b"sws_flags\0" as *const u8 as *const c_char,
        (*h).ctx_flags as int64_t,
        0 as c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut c_void,
        b"dstw\0" as *const u8 as *const c_char,
        (*h).dst.width as int64_t,
        0 as c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut c_void,
        b"dsth\0" as *const u8 as *const c_char,
        (*h).dst.height as int64_t,
        0 as c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut c_void,
        b"dst_format\0" as *const u8 as *const c_char,
        (*h).dst.pix_fmt as int64_t,
        0 as c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut c_void,
        b"dst_range\0" as *const u8 as *const c_char,
        (*h).dst.range as int64_t,
        0 as c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut c_void,
        b"srcw\0" as *const u8 as *const c_char,
        (*h).scale.width as int64_t,
        0 as c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut c_void,
        b"srch\0" as *const u8 as *const c_char,
        (*h).scale.height as int64_t,
        0 as c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut c_void,
        b"src_format\0" as *const u8 as *const c_char,
        (*h).scale.pix_fmt as int64_t,
        0 as c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut c_void,
        b"src_range\0" as *const u8 as *const c_char,
        (*h).scale.range as int64_t,
        0 as c_int,
    );
    sws_setColorspaceDetails(
        (*h).ctx as *mut SwsContext,
        sws_getCoefficients(SWS_CS_DEFAULT) as *const c_int,
        (*h).scale.range,
        sws_getCoefficients(SWS_CS_DEFAULT) as *const c_int,
        (*h).dst.range,
        0 as c_int,
        (1 as c_int) << 16 as c_int,
        (1 as c_int) << 16 as c_int,
    );
    return (sws_init_context(
        (*h).ctx as *mut SwsContext,
        0 as *mut SwsFilter,
        0 as *mut SwsFilter,
    ) < 0 as c_int) as c_int;
}
#[c2rust::src_loc = "396:1"]
unsafe extern "C" fn check_resizer(mut h: *mut resizer_hnd_t, mut in_0: *mut cli_pic_t) -> c_int {
    let mut input_prop: frame_prop_t = {
        let mut init = frame_prop_t {
            width: (*in_0).img.width,
            height: (*in_0).img.height,
            pix_fmt: convert_csp_to_pix_fmt((*in_0).img.csp),
            range: (*h).input_range,
        };
        init
    };
    if memcmp(
        &mut input_prop as *mut frame_prop_t as *const c_void,
        &mut (*h).scale as *mut frame_prop_t as *const c_void,
        ::core::mem::size_of::<frame_prop_t>() as size_t,
    ) == 0
    {
        return 0 as c_int;
    }
    if !(*h).ctx.is_null() || (*h).working != 0 {
        x264_cli_log(
            NAME.as_ptr(),
            X264_LOG_WARNING,
            b"stream properties changed at pts %ld\n\0" as *const u8 as *const c_char,
            (*in_0).pts,
        );
        (*h).fast_mono = 0 as c_int;
    }
    (*h).scale = input_prop;
    if (*h).buffer_allocated == 0 && (*h).fast_mono == 0 {
        if x264_cli_pic_alloc_aligned(
            &mut (*h).buffer,
            (*h).dst_csp,
            (*h).dst.width,
            (*h).dst.height,
        ) != 0
        {
            return -(1 as c_int);
        }
        (*h).buffer_allocated = 1 as c_int;
    }
    if init_sws_context(h) != 0 {
        x264_cli_log(
            b"resize\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"swscale init failed\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "418:1"]
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut c_char,
) -> c_int {
    if !opt_string.is_null()
        && strcmp(opt_string, b"normcsp\0" as *const u8 as *const c_char) == 0
        && (*info).csp & X264_CSP_OTHER == 0
    {
        return 0 as c_int;
    }
    if opt_string.is_null() && full_check(info, param) == 0 {
        return 0 as c_int;
    }
    static mut optlist: [*const c_char; 7] = [
        b"width\0" as *const u8 as *const c_char,
        b"height\0" as *const u8 as *const c_char,
        b"sar\0" as *const u8 as *const c_char,
        b"fittobox\0" as *const u8 as *const c_char,
        b"csp\0" as *const u8 as *const c_char,
        b"method\0" as *const u8 as *const c_char,
        0 as *const c_char,
    ];
    let mut opts: *mut *mut c_char = x264_split_options(opt_string, optlist.as_ptr());
    if opts.is_null() && !opt_string.is_null() {
        return -(1 as c_int);
    }
    let mut h: *mut resizer_hnd_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<resizer_hnd_t>() as size_t,
    ) as *mut resizer_hnd_t;
    if h.is_null() {
        return -(1 as c_int);
    }
    (*h).ctx_flags = convert_method_to_flag(x264_otos(
        x264_get_option(optlist[5 as c_int as usize], opts),
        b"\0" as *const u8 as *const c_char as *mut c_char,
    ));
    if !opts.is_null() {
        (*h).dst_csp = (*info).csp;
        (*h).dst.width = (*info).width;
        (*h).dst.height = (*info).height;
        (*h).dst.range = (*info).fullrange;
        if strcmp(opt_string, b"normcsp\0" as *const u8 as *const c_char) == 0 {
            free(opts as *mut c_void);
            (*h).variable_input = 1 as c_int;
            (*h).dst_csp = pick_closest_supported_csp((*info).csp);
            if (*h).dst_csp == 0 as c_int {
                x264_cli_log(
                    b"resize\0" as *const u8 as *const c_char,
                    X264_LOG_ERROR,
                    b"filter get invalid input pixel format %d (colorspace %d)\n\0" as *const u8
                        as *const c_char,
                    convert_csp_to_pix_fmt((*info).csp),
                    (*info).csp,
                );
                return -(1 as c_int);
            }
        } else {
            let mut err: c_int = handle_opts(optlist.as_ptr(), opts, info, h);
            free(opts as *mut c_void);
            if err != 0 {
                return -(1 as c_int);
            }
        }
    } else {
        (*h).dst_csp = (*param).i_csp;
        (*h).dst.width = (*param).i_width;
        (*h).dst.height = (*param).i_height;
        (*h).dst.range = (*param).vui.b_fullrange;
    }
    if (*h).ctx_flags != SWS_FAST_BILINEAR as c_int as uint32_t {
        (*h).ctx_flags |= (SWS_FULL_CHR_H_INT as c_int
            | SWS_FULL_CHR_H_INP as c_int
            | SWS_ACCURATE_RND as c_int) as uint32_t;
    }
    (*h).dst.pix_fmt = convert_csp_to_pix_fmt((*h).dst_csp);
    (*h).scale = (*h).dst;
    (*h).input_range = (*info).fullrange;
    let mut src_csp: c_int = (*info).csp & (X264_CSP_MASK | X264_CSP_OTHER);
    let mut dst_csp: c_int = (*h).dst_csp & (X264_CSP_MASK | X264_CSP_OTHER);
    (*h).pre_swap_chroma =
        (src_csp == X264_CSP_YV12 || src_csp == X264_CSP_YV16 || src_csp == X264_CSP_YV24) as c_int;
    (*h).post_swap_chroma =
        (dst_csp == X264_CSP_YV12 || dst_csp == X264_CSP_YV16 || dst_csp == X264_CSP_YV24) as c_int;
    let mut src_pix_fmt: c_int = convert_csp_to_pix_fmt((*info).csp);
    let mut src_pix_fmt_inv: c_int = convert_csp_to_pix_fmt((*info).csp ^ X264_CSP_HIGH_DEPTH);
    let mut dst_pix_fmt_inv: c_int = convert_csp_to_pix_fmt((*h).dst_csp ^ X264_CSP_HIGH_DEPTH);
    if (*h).dst.width <= 0 as c_int
        || (*h).dst.height <= 0 as c_int
        || (*h).dst.width > 16384 as c_int
        || (*h).dst.height > 16384 as c_int
    {
        x264_cli_log(
            b"resize\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"invalid width x height (%dx%d)\n\0" as *const u8 as *const c_char,
            (*h).dst.width,
            (*h).dst.height,
        );
        return -(1 as c_int);
    }
    if src_pix_fmt == AV_PIX_FMT_NONE as c_int && src_pix_fmt_inv != AV_PIX_FMT_NONE as c_int {
        x264_cli_log(
            b"resize\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"input colorspace %s with bit depth %d is not supported\n\0" as *const u8
                as *const c_char,
            av_get_pix_fmt_name(src_pix_fmt_inv as AVPixelFormat),
            if (*info).csp & 0x2000 as c_int != 0 {
                16 as c_int
            } else {
                8 as c_int
            },
        );
        return -(1 as c_int);
    }
    if sws_isSupportedInput(src_pix_fmt as AVPixelFormat) == 0 {
        x264_cli_log(
            b"resize\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"input colorspace %s is not supported\n\0" as *const u8 as *const c_char,
            av_get_pix_fmt_name(src_pix_fmt as AVPixelFormat),
        );
        return -(1 as c_int);
    }
    if (*h).dst.pix_fmt == AV_PIX_FMT_NONE as c_int && dst_pix_fmt_inv != AV_PIX_FMT_NONE as c_int {
        x264_cli_log(
            b"resize\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"input colorspace %s with bit depth %d is not supported\n\0" as *const u8
                as *const c_char,
            av_get_pix_fmt_name(dst_pix_fmt_inv as AVPixelFormat),
            if (*h).dst_csp & 0x2000 as c_int != 0 {
                16 as c_int
            } else {
                8 as c_int
            },
        );
        return -(1 as c_int);
    }
    if sws_isSupportedOutput((*h).dst.pix_fmt as AVPixelFormat) == 0 {
        x264_cli_log(
            b"resize\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"output colorspace %s is not supported\n\0" as *const u8 as *const c_char,
            av_get_pix_fmt_name((*h).dst.pix_fmt as AVPixelFormat),
        );
        return -(1 as c_int);
    }
    if (*h).dst.height != (*info).height && (*info).interlaced != 0 {
        x264_cli_log(
            b"resize\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"swscale is not compatible with interlaced vertical resizing\n\0" as *const u8
                as *const c_char,
        );
        return -(1 as c_int);
    }
    let mut csp: *const x264_cli_csp_t = x264_cli_get_csp((*h).dst_csp);
    if (*h).dst.width % (*csp).mod_width != 0 || (*h).dst.height % (*csp).mod_height != 0 {
        x264_cli_log(
            b"resize\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"resolution %dx%d is not compliant with colorspace %s\n\0" as *const u8
                as *const c_char,
            (*h).dst.width,
            (*h).dst.height,
            (*csp).name,
        );
        return -(1 as c_int);
    }
    if (*h).dst.width != (*info).width || (*h).dst.height != (*info).height {
        x264_cli_log(
            NAME.as_ptr(),
            X264_LOG_INFO,
            b"resizing to %dx%d\n\0" as *const u8 as *const c_char,
            (*h).dst.width,
            (*h).dst.height,
        );
    }
    if (*h).dst.pix_fmt != src_pix_fmt {
        x264_cli_log(
            NAME.as_ptr(),
            X264_LOG_WARNING,
            b"converting from %s to %s\n\0" as *const u8 as *const c_char,
            av_get_pix_fmt_name(src_pix_fmt as AVPixelFormat),
            av_get_pix_fmt_name((*h).dst.pix_fmt as AVPixelFormat),
        );
    } else if (*h).dst.range != (*h).input_range {
        x264_cli_log(
            NAME.as_ptr(),
            X264_LOG_WARNING,
            b"converting range from %s to %s\n\0" as *const u8 as *const c_char,
            if (*h).input_range != 0 {
                b"PC\0" as *const u8 as *const c_char
            } else {
                b"TV\0" as *const u8 as *const c_char
            },
            if (*h).dst.range != 0 {
                b"PC\0" as *const u8 as *const c_char
            } else {
                b"TV\0" as *const u8 as *const c_char
            },
        );
    }
    (*h).dst_csp |= (*info).csp & X264_CSP_VFLIP;
    if dst_csp == X264_CSP_I400
        && (src_csp >= X264_CSP_I420 && src_csp <= X264_CSP_NV16
            || src_csp == X264_CSP_I444
            || src_csp == X264_CSP_YV24)
        && (*h).dst.width == (*info).width
        && (*h).dst.height == (*info).height
        && (*h).dst.range == (*h).input_range
    {
        (*h).fast_mono = 1 as c_int;
    }
    if (*h).variable_input == 0 {
        let mut input_pic: cli_pic_t = {
            let mut init = cli_pic_t {
                img: {
                    let mut init = cli_image_t {
                        csp: (*info).csp,
                        width: (*info).width,
                        height: (*info).height,
                        planes: 0 as c_int,
                        plane: [0 as *mut uint8_t; 4],
                        stride: [0; 4],
                    };
                    init
                },
                pts: 0 as int64_t,
                duration: 0,
                opaque: 0 as *mut c_void,
            };
            init
        };
        if check_resizer(h, &mut input_pic) != 0 {
            return -(1 as c_int);
        }
    }
    (*info).csp = (*h).dst_csp;
    (*info).width = (*h).dst.width;
    (*info).height = (*h).dst.height;
    (*info).fullrange = (*h).dst.range;
    (*h).prev_filter = *filter;
    (*h).prev_hnd = *handle;
    *handle = h as hnd_t;
    *filter = resize_filter;
    return 0 as c_int;
}
#[c2rust::src_loc = "543:1"]
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut resizer_hnd_t = handle as *mut resizer_hnd_t;
    if (*h)
        .prev_filter
        .get_frame
        .expect("non-null function pointer")((*h).prev_hnd, output, frame)
        != 0
    {
        return -(1 as c_int);
    }
    if (*h).variable_input != 0 && check_resizer(h, output) != 0 {
        return -(1 as c_int);
    }
    (*h).working = 1 as c_int;
    if (*h).pre_swap_chroma != 0 {
        let mut t: *mut uint8_t = (*output).img.plane[1 as c_int as usize];
        (*output).img.plane[1 as c_int as usize] = (*output).img.plane[2 as c_int as usize];
        (*output).img.plane[2 as c_int as usize] = t;
    }
    if !(*h).ctx.is_null() && (*h).fast_mono == 0 {
        sws_scale(
            (*h).ctx as *mut SwsContext,
            (*output).img.plane.as_mut_ptr() as *const *const uint8_t,
            (*output).img.stride.as_mut_ptr() as *const c_int,
            0 as c_int,
            (*output).img.height,
            (*h).buffer.img.plane.as_mut_ptr() as *const *mut uint8_t,
            (*h).buffer.img.stride.as_mut_ptr() as *const c_int,
        );
        (*output).img = (*h).buffer.img;
    } else {
        (*output).img.csp = (*h).dst_csp;
    }
    if (*h).post_swap_chroma != 0 {
        let mut t_0: *mut uint8_t = (*output).img.plane[1 as c_int as usize];
        (*output).img.plane[1 as c_int as usize] = (*output).img.plane[2 as c_int as usize];
        (*output).img.plane[2 as c_int as usize] = t_0;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "567:1"]
unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut resizer_hnd_t = handle as *mut resizer_hnd_t;
    return (*h)
        .prev_filter
        .release_frame
        .expect("non-null function pointer")((*h).prev_hnd, pic, frame);
}
#[c2rust::src_loc = "573:1"]
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut resizer_hnd_t = handle as *mut resizer_hnd_t;
    (*h).prev_filter.free.expect("non-null function pointer")((*h).prev_hnd);
    if !(*h).ctx.is_null() {
        sws_freeContext((*h).ctx as *mut SwsContext);
    }
    if (*h).buffer_allocated != 0 {
        x264_cli_pic_clean(&mut (*h).buffer);
    }
    free(h as *mut c_void);
}
#[no_mangle]
#[c2rust::src_loc = "612:18"]
static mut resize_filter: cli_vid_filter_t = cli_vid_filter_t {
    name: NAME.as_ptr(),
    help: Some(help as unsafe extern "C" fn(c_int) -> ()),
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
