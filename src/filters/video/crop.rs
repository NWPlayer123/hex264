use core::ffi::{c_char, c_float, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::filters_h::{x264_get_option, x264_otoi};
use crate::input_h::{
    cli_pic_t, video_info_t, x264_cli_csp_depth_factor, x264_cli_csp_is_invalid, x264_cli_csp_t,
    x264_cli_get_csp,
};
use crate::src::filters::filters::x264_split_options;
use crate::stdint_h::intptr_t;
use crate::stdio_h::printf;
use crate::stdlib_h::{calloc, free};
use crate::video_h::cli_vid_filter_t;
use crate::x264_h::{x264_param_t, X264_LOG_ERROR, X264_LOG_INFO};
use crate::x264cli_h::{hnd_t, x264_cli_log};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "34:9"]
struct crop_hnd_t {
    prev_hnd: hnd_t,
    prev_filter: cli_vid_filter_t,
    dims: [u32; 4],
    csp: *const x264_cli_csp_t,
}
#[c2rust::src_loc = "29:9"]
const NAME: [c_char; 5] = unsafe { ::core::mem::transmute::<[u8; 5], [c_char; 5]>(*b"crop\0") };
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn help(mut longhelp: c_int) {
    printf(b"      crop:left,top,right,bottom\n\0" as *const u8 as *const c_char);
    if longhelp == 0 {
        return;
    }
    printf(
        b"            removes pixels from the edges of the frame\n\0" as *const u8 as *const c_char,
    );
}
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn handle_opts(
    mut h: *mut crop_hnd_t,
    mut info: *mut video_info_t,
    mut opts: *mut *mut c_char,
    mut optlist: *const *const c_char,
) -> c_int {
    let mut i: c_int = 0 as c_int;
    while i < 4 as c_int {
        let mut opt: *mut c_char = x264_get_option(*optlist.offset(i as isize), opts);
        if opt.is_null() {
            x264_cli_log(
                b"crop\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"%s crop value not specified\n\0" as *const u8 as *const c_char,
                *optlist.offset(i as isize),
            );
            return -1;
        }
        // TODO: fix otoi
        (*h).dims[i as usize] = x264_otoi(opt, -1) as u32;
        if (*h).dims[i as usize] < 0 {
            x264_cli_log(
                b"crop\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"%s crop value `%s' is less than 0\n\0" as *const u8 as *const c_char,
                *optlist.offset(i as isize),
                opt,
            );
            return -1;
        }
        let mut dim_mod: c_int = if i & 1 as c_int != 0 {
            (*(*h).csp).mod_height << (*info).interlaced
        } else {
            (*(*h).csp).mod_width
        };
        if (*h).dims[i as usize] as c_int % dim_mod != 0 {
            x264_cli_log(
                b"crop\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"%s crop value `%s' is not a multiple of %d\n\0" as *const u8 as *const c_char,
                *optlist.offset(i as isize),
                opt,
                dim_mod,
            );
            return -1;
        }
        i += 1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "65:1"]
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut _param: *mut x264_param_t,
    mut opt_string: *mut c_char,
) -> c_int {
    if x264_cli_csp_is_invalid((*info).csp) != 0 {
        x264_cli_log(
            b"crop\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"invalid csp %d\n\0" as *const u8 as *const c_char,
            (*info).csp,
        );
        return -1;
    }
    let mut h: *mut crop_hnd_t =
        calloc(1 as size_t, ::core::mem::size_of::<crop_hnd_t>() as size_t) as *mut crop_hnd_t;
    if h.is_null() {
        return -1;
    }
    (*h).csp = x264_cli_get_csp((*info).csp);
    static mut optlist: [*const c_char; 5] = [
        b"left\0" as *const u8 as *const c_char,
        b"top\0" as *const u8 as *const c_char,
        b"right\0" as *const u8 as *const c_char,
        b"bottom\0" as *const u8 as *const c_char,
        0 as *const c_char,
    ];
    let mut opts: *mut *mut c_char = x264_split_options(opt_string, optlist.as_ptr());
    if opts.is_null() {
        return -1;
    }
    let mut err: c_int = handle_opts(h, info, opts, optlist.as_ptr());
    free(opts as *mut c_void);
    if err != 0 {
        return -1;
    }
    (*h).dims[2] = (*info).width - (*h).dims[0] - (*h).dims[2];
    (*h).dims[3] = (*info).height - (*h).dims[1] - (*h).dims[3];
    if (*h).dims[2] <= 0 || (*h).dims[3] <= 0 {
        x264_cli_log(
            b"crop\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"invalid output resolution %dx%d\n\0" as *const u8 as *const c_char,
            (*h).dims[2],
            (*h).dims[3],
        );
        return -1;
    }
    if (*info).width != (*h).dims[2] || (*info).height != (*h).dims[3] {
        x264_cli_log(
            NAME.as_ptr(),
            X264_LOG_INFO,
            b"cropping to %dx%d\n\0" as *const u8 as *const c_char,
            (*h).dims[2],
            (*h).dims[3],
        );
    } else {
        free(h as *mut c_void);
        return 0 as c_int;
    }
    (*info).width = (*h).dims[2];
    (*info).height = (*h).dims[3];
    (*h).prev_filter = *filter;
    (*h).prev_hnd = *handle;
    *handle = h as hnd_t;
    *filter = crop_filter;
    return 0 as c_int;
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut crop_hnd_t = handle as *mut crop_hnd_t;
    if (*h)
        .prev_filter
        .get_frame
        .expect("non-null function pointer")((*h).prev_hnd, output, frame)
        != 0
    {
        return -1;
    }
    (*output).img.width = (*h).dims[2] as c_int;
    (*output).img.height = (*h).dims[3] as c_int;
    let mut i: c_int = 0 as c_int;
    while i < (*output).img.planes {
        let mut offset: intptr_t = (((*output).img.stride[i as usize] as u32 * (*h).dims[1])
            as c_float
            * (*(*h).csp).height[i as usize]) as intptr_t;
        offset = (offset as c_float
            + (*h).dims[0] as c_float
                * (*(*h).csp).width[i as usize]
                * x264_cli_csp_depth_factor((*output).img.csp) as c_float)
            as intptr_t;
        (*output).img.plane[i as usize] = (*output).img.plane[i as usize].offset(offset as isize);
        i += 1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "124:1"]
unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut crop_hnd_t = handle as *mut crop_hnd_t;
    return (*h)
        .prev_filter
        .release_frame
        .expect("non-null function pointer")((*h).prev_hnd, pic, frame);
}
#[c2rust::src_loc = "132:1"]
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut crop_hnd_t = handle as *mut crop_hnd_t;
    (*h).prev_filter.free.expect("non-null function pointer")((*h).prev_hnd);
    free(h as *mut c_void);
}
#[no_mangle]
#[c2rust::src_loc = "139:18"]
static mut crop_filter: cli_vid_filter_t = cli_vid_filter_t {
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
