use core::ffi::{c_char, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::input_h::{cli_input, cli_pic_t, video_info_t};
use crate::stdlib_h::{calloc, free};
use crate::video_h::cli_vid_filter_t;
use crate::x264_h::x264_param_t;
use crate::x264cli_h::hnd_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "31:9"]
struct source_hnd_t {
    pic: cli_pic_t,
    hin: hnd_t,
    cur_frame: c_int,
}
#[c2rust::src_loc = "40:1"]
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut _param: *mut x264_param_t,
    mut _opt_string: *mut c_char,
) -> c_int {
    let mut h: *mut source_hnd_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<source_hnd_t>() as size_t,
    ) as *mut source_hnd_t;
    if h.is_null() {
        return -(1 as c_int);
    }
    (*h).cur_frame = -(1 as c_int);
    if cli_input.picture_alloc.expect("non-null function pointer")(
        &mut (*h).pic,
        *handle,
        (*info).csp,
        (*info).width,
        (*info).height,
    ) != 0
    {
        return -(1 as c_int);
    }
    (*h).hin = *handle;
    *handle = h as hnd_t;
    *filter = source_filter;
    return 0 as c_int;
}
#[c2rust::src_loc = "57:1"]
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut source_hnd_t = handle as *mut source_hnd_t;
    if frame <= (*h).cur_frame
        || cli_input.read_frame.expect("non-null function pointer")(&mut (*h).pic, (*h).hin, frame)
            != 0
    {
        return -(1 as c_int);
    }
    (*h).cur_frame = frame;
    *output = (*h).pic;
    return 0 as c_int;
}
#[c2rust::src_loc = "68:1"]
unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut _pic: *mut cli_pic_t,
    mut _frame: c_int,
) -> c_int {
    let mut h: *mut source_hnd_t = handle as *mut source_hnd_t;
    if cli_input.release_frame.is_some()
        && cli_input.release_frame.expect("non-null function pointer")(&mut (*h).pic, (*h).hin) != 0
    {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut source_hnd_t = handle as *mut source_hnd_t;
    cli_input.picture_clean.expect("non-null function pointer")(&mut (*h).pic, (*h).hin);
    cli_input.close_file.expect("non-null function pointer")((*h).hin);
    free(h as *mut c_void);
}
#[no_mangle]
#[c2rust::src_loc = "84:18"]
static mut source_filter: cli_vid_filter_t = cli_vid_filter_t {
    name: b"source\0" as *const u8 as *const c_char,
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
