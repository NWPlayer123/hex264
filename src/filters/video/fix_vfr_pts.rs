use core::ffi::{c_char, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::input_h::{cli_pic_t, video_info_t, x264_cli_pic_alloc, x264_cli_pic_clean};
use crate::internal_h::x264_cli_pic_copy;
use crate::stdint_intn_h::int64_t;
use crate::stdlib_h::{calloc, free};
use crate::video_h::cli_vid_filter_t;
use crate::x264_h::x264_param_t;
use crate::x264cli_h::hnd_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "34:9"]
struct fix_vfr_pts_hnd_t {
    prev_hnd: hnd_t,
    prev_filter: cli_vid_filter_t,
    buffer: cli_pic_t,
    holder: cli_pic_t,
    buffer_allocated: c_int,
    holder_frame: c_int,
    holder_ret: c_int,
    pts: int64_t,
    last_duration: int64_t,
}
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut _param: *mut x264_param_t,
    mut _opt_string: *mut c_char,
) -> c_int {
    if !(*info).vfr {
        return 0 as c_int;
    }
    let mut h: *mut fix_vfr_pts_hnd_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<fix_vfr_pts_hnd_t>() as size_t,
    ) as *mut fix_vfr_pts_hnd_t;
    if h.is_null() {
        return -1;
    }
    (*h).holder_frame = -1;
    (*h).prev_hnd = *handle;
    (*h).prev_filter = *filter;
    *handle = h as hnd_t;
    *filter = fix_vfr_pts_filter;
    return 0 as c_int;
}
#[c2rust::src_loc = "69:1"]
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut fix_vfr_pts_hnd_t = handle as *mut fix_vfr_pts_hnd_t;
    if frame == (*h).holder_frame {
        if (*h).holder_ret != 0 {
            return (*h).holder_ret;
        }
    } else {
        if (*h).holder_frame > 0 as c_int
            && (*h).holder_frame < frame
            && (*h)
                .prev_filter
                .release_frame
                .expect("non-null function pointer")(
                (*h).prev_hnd,
                &mut (*h).holder,
                (*h).holder_frame,
            ) != 0
        {
            return -1;
        }
        (*h).holder_frame = -1;
        if (*h)
            .prev_filter
            .get_frame
            .expect("non-null function pointer")((*h).prev_hnd, &mut (*h).holder, frame)
            != 0
        {
            return -1;
        }
    }
    if (*h).holder.duration == 0 {
        if (*h).buffer_allocated == 0 {
            if x264_cli_pic_alloc(
                &mut (*h).buffer,
                (*h).holder.img.csp,
                (*h).holder.img.width,
                (*h).holder.img.height,
            ) != 0
            {
                return -1;
            }
            (*h).buffer_allocated = 1 as c_int;
        }
        (*h).holder_frame = frame + 1 as c_int;
        if x264_cli_pic_copy(&mut (*h).buffer, &mut (*h).holder) != 0
            || (*h)
                .prev_filter
                .release_frame
                .expect("non-null function pointer")(
                (*h).prev_hnd, &mut (*h).holder, frame
            ) != 0
        {
            return -1;
        }
        (*h).holder_ret = (*h)
            .prev_filter
            .get_frame
            .expect("non-null function pointer")(
            (*h).prev_hnd,
            &mut (*h).holder,
            (*h).holder_frame,
        );
        if (*h).holder_ret == 0 {
            (*h).last_duration = if (*h).holder.pts - (*h).buffer.pts > 1 as int64_t {
                (*h).holder.pts - (*h).buffer.pts
            } else {
                1 as int64_t
            };
        }
        (*h).buffer.duration = (*h).last_duration;
        *output = (*h).buffer;
    } else {
        *output = (*h).holder;
    }
    (*output).pts = (*h).pts;
    (*h).pts += (*output).duration;
    return 0 as c_int;
}
#[c2rust::src_loc = "118:1"]
unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut fix_vfr_pts_hnd_t = handle as *mut fix_vfr_pts_hnd_t;
    if frame == (*h).holder_frame - 1 as c_int {
        return 0 as c_int;
    }
    return (*h)
        .prev_filter
        .release_frame
        .expect("non-null function pointer")((*h).prev_hnd, pic, frame);
}
#[c2rust::src_loc = "127:1"]
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut fix_vfr_pts_hnd_t = handle as *mut fix_vfr_pts_hnd_t;
    (*h).prev_filter.free.expect("non-null function pointer")((*h).prev_hnd);
    if (*h).buffer_allocated != 0 {
        x264_cli_pic_clean(&mut (*h).buffer);
    }
    free(h as *mut c_void);
}
#[no_mangle]
#[c2rust::src_loc = "136:18"]
static mut fix_vfr_pts_filter: cli_vid_filter_t = cli_vid_filter_t {
    name: b"fix_vfr_pts\0" as *const u8 as *const c_char,
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
