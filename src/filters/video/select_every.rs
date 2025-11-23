use core::ffi::{c_char, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::base_h::x264_reduce_fraction;
use crate::filters_h::x264_otoi;
use crate::input_h::{cli_pic_t, video_info_t};
use crate::src::filters::video::video::x264_init_vid_filter;
use crate::src::x264::x264_cli_log;
use crate::stdint_h::intptr_t;
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::{uint32_t, uint64_t};
use crate::stdio_h::{printf, sprintf};
use crate::stdlib_h::{free, malloc};
use crate::string_h::{memcpy, strtok_r};
use crate::video_h::cli_vid_filter_t;
use crate::x264_h::{x264_param_t, X264_LOG_ERROR};
use crate::x264cli_h::hnd_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "33:9"]
struct selvry_hnd_t {
    prev_hnd: hnd_t,
    prev_filter: cli_vid_filter_t,
    pattern: *mut c_int,
    pattern_len: c_int,
    step_size: c_int,
    vfr: c_int,
    pts: int64_t,
}
#[c2rust::src_loc = "28:9"]
const NAME: [c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [c_char; 13]>(*b"select_every\0") };
#[c2rust::src_loc = "47:1"]
unsafe extern "C" fn help(mut longhelp: c_int) {
    printf(b"      select_every:step,offset1[,...]\n\0" as *const u8 as *const c_char);
    if longhelp == 0 {
        return;
    }
    printf(
        b"            apply a selection pattern to input frames\n            step: the number of frames in the pattern\n            offsets: the offset into the step to select a frame\n            see: http://avisynth.nl/index.php/Select#SelectEvery\n\0"
            as *const u8 as *const c_char,
    );
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut c_char,
) -> c_int {
    let mut h: *mut selvry_hnd_t =
        malloc(::core::mem::size_of::<selvry_hnd_t>() as size_t) as *mut selvry_hnd_t;
    if h.is_null() {
        return -1;
    }
    (*h).pattern_len = 0 as c_int;
    (*h).step_size = 0 as c_int;
    let mut offsets: [c_int; 100] = [0; 100];
    let mut tok: *mut c_char = 0 as *mut c_char;
    let mut p: *mut c_char = opt_string;
    let mut saveptr: *mut c_char = 0 as *mut c_char;
    loop {
        tok = strtok_r(p, b",\0" as *const u8 as *const c_char, &mut saveptr);
        if tok.is_null() {
            break;
        }
        let mut val: c_int = x264_otoi(tok, -1);
        if !p.is_null() {
            if val <= 0 as c_int {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const c_char,
                    X264_LOG_ERROR,
                    b"invalid step `%s'\n\0" as *const u8 as *const c_char,
                    tok,
                );
                return -1;
            }
            (*h).step_size = val;
        } else {
            if val < 0 as c_int || val >= (*h).step_size {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const c_char,
                    X264_LOG_ERROR,
                    b"invalid offset `%s'\n\0" as *const u8 as *const c_char,
                    tok,
                );
                return -1;
            }
            if (*h).pattern_len >= 100 as c_int {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const c_char,
                    X264_LOG_ERROR,
                    b"max pattern size %d reached\n\0" as *const u8 as *const c_char,
                    100 as c_int,
                );
                return -1;
            }
            let fresh0 = (*h).pattern_len;
            (*h).pattern_len = (*h).pattern_len + 1;
            offsets[fresh0 as usize] = val;
        }
        p = 0 as *mut c_char;
    }
    if (*h).step_size == 0 {
        x264_cli_log(
            b"select_every\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"no step size provided\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    if (*h).pattern_len == 0 {
        x264_cli_log(
            b"select_every\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"no offsets supplied\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    (*h).pattern = malloc(
        ((*h).pattern_len as size_t).wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
    ) as *mut c_int;
    if (*h).pattern.is_null() {
        return -1;
    }
    memcpy(
        (*h).pattern as *mut c_void,
        offsets.as_mut_ptr() as *const c_void,
        ((*h).pattern_len as size_t).wrapping_mul(::core::mem::size_of::<c_int>() as size_t),
    );
    let mut max_rewind: intptr_t = 0 as intptr_t;
    let mut min: c_int = (*h).step_size;
    let mut i: c_int = (*h).pattern_len - 1 as c_int;
    while i >= 0 as c_int {
        min = if min < offsets[i as usize] {
            min
        } else {
            offsets[i as usize]
        };
        if i != 0 {
            max_rewind = if max_rewind
                > (offsets[(i - 1 as c_int) as usize] - min + 1 as c_int) as intptr_t
            {
                max_rewind
            } else {
                (offsets[(i - 1 as c_int) as usize] - min + 1 as c_int) as intptr_t
            };
        }
        if max_rewind == (*h).step_size as intptr_t {
            break;
        }
        i -= 1;
    }
    let mut name: [c_char; 20] = [0; 20];
    sprintf(
        name.as_mut_ptr(),
        b"cache_%d\0" as *const u8 as *const c_char,
        (*param).i_bitdepth,
    );
    if x264_init_vid_filter(
        name.as_mut_ptr(),
        handle,
        filter,
        info,
        param,
        max_rewind as *mut c_void as *mut c_char,
    ) != 0
    {
        return -1;
    }
    if (*h).step_size != (*h).pattern_len {
        (*info).num_frames = ((*info).num_frames as uint64_t)
            .wrapping_mul((*h).pattern_len as uint64_t)
            .wrapping_div((*h).step_size as uint64_t) as c_int;
        (*info).fps_den = (*info).fps_den.wrapping_mul((*h).step_size as uint32_t);
        (*info).fps_num = (*info).fps_num.wrapping_mul((*h).pattern_len as uint32_t);
        x264_reduce_fraction(&mut (*info).fps_num, &mut (*info).fps_den);
        if (*info).vfr != 0 {
            (*info).timebase_den = (*info)
                .timebase_den
                .wrapping_mul((*h).pattern_len as uint32_t);
            (*info).timebase_num = (*info)
                .timebase_num
                .wrapping_mul((*h).step_size as uint32_t);
            x264_reduce_fraction(&mut (*info).timebase_num, &mut (*info).timebase_den);
        }
    }
    (*h).pts = 0 as int64_t;
    (*h).vfr = (*info).vfr;
    (*h).prev_filter = *filter;
    (*h).prev_hnd = *handle;
    *filter = select_every_filter;
    *handle = h as hnd_t;
    return 0 as c_int;
}
#[c2rust::src_loc = "129:1"]
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    let mut pat_frame: c_int = *(*h).pattern.offset((frame % (*h).pattern_len) as isize)
        + frame / (*h).pattern_len * (*h).step_size;
    if (*h)
        .prev_filter
        .get_frame
        .expect("non-null function pointer")((*h).prev_hnd, output, pat_frame)
        != 0
    {
        return -1;
    }
    if (*h).vfr != 0 {
        (*output).pts = (*h).pts;
        (*h).pts += (*output).duration;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "143:1"]
unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    let mut pat_frame: c_int = *(*h).pattern.offset((frame % (*h).pattern_len) as isize)
        + frame / (*h).pattern_len * (*h).step_size;
    return (*h)
        .prev_filter
        .release_frame
        .expect("non-null function pointer")((*h).prev_hnd, pic, pat_frame);
}
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    (*h).prev_filter.free.expect("non-null function pointer")((*h).prev_hnd);
    free((*h).pattern as *mut c_void);
    free(h as *mut c_void);
}
#[no_mangle]
#[c2rust::src_loc = "158:18"]
static mut select_every_filter: cli_vid_filter_t = cli_vid_filter_t {
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
