use core::ffi::{c_char, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::frame_h::{x264_10_frame_push, x264_10_frame_shift, x264_frame_t};
use crate::input_h::{
    cli_image_t, cli_pic_t, video_info_t, x264_cli_pic_alloc, x264_cli_pic_clean,
};
use crate::internal_h::x264_cli_pic_copy;
use crate::stdint_h::intptr_t;
use crate::stdint_uintn_h::uint8_t;
use crate::stdlib_h::{calloc, free, malloc};
use crate::video_h::cli_vid_filter_t;
use crate::x264_h::{x264_param_t, X264_LOG_ERROR};
use crate::x264cli_h::{hnd_t, x264_cli_log};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "39:9"]
struct cache_hnd_t {
    prev_hnd: hnd_t,
    prev_filter: cli_vid_filter_t,
    max_size: c_int,
    first_frame: c_int,
    cache: *mut *mut cli_pic_t,
    cur_size: c_int,
    eof: c_int,
}
#[c2rust::src_loc = "34:9"]
const NAME: [c_char; 9] = unsafe { ::core::mem::transmute::<[u8; 9], [c_char; 9]>(*b"cache_10\0") };
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut _param: *mut x264_param_t,
    mut opt_string: *mut c_char,
) -> c_int {
    let mut size: intptr_t = opt_string as intptr_t;
    if size <= 0 as intptr_t {
        return 0 as c_int;
    }
    let mut h: *mut cache_hnd_t =
        calloc(1 as size_t, ::core::mem::size_of::<cache_hnd_t>() as size_t) as *mut cache_hnd_t;
    if h.is_null() {
        return -1;
    }
    (*h).max_size = size as c_int;
    (*h).cache = malloc(
        (((*h).max_size + 1 as c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut cli_pic_t>() as size_t),
    ) as *mut *mut cli_pic_t;
    if (*h).cache.is_null() {
        return -1;
    }
    let mut i: c_int = 0 as c_int;
    while i < (*h).max_size {
        let ref mut fresh0 = *(*h).cache.offset(i as isize);
        *fresh0 = malloc(::core::mem::size_of::<cli_pic_t>() as size_t) as *mut cli_pic_t;
        if (*(*h).cache.offset(i as isize)).is_null()
            || x264_cli_pic_alloc(
                *(*h).cache.offset(i as isize),
                (*info).csp,
                (*info).width as c_int,
                (*info).height as c_int,
            ) != 0
        {
            return -1;
        }
        i += 1;
    }
    let ref mut fresh1 = *(*h).cache.offset((*h).max_size as isize);
    *fresh1 = 0 as *mut cli_pic_t;
    (*h).prev_filter = *filter;
    (*h).prev_hnd = *handle;
    *handle = h as hnd_t;
    *filter = cache_10_filter;
    return 0 as c_int;
}
#[c2rust::src_loc = "84:1"]
unsafe extern "C" fn fill_cache(mut h: *mut cache_hnd_t, mut frame: c_int) {
    let mut shift: c_int = frame - ((*h).first_frame + (*h).cur_size - 1 as c_int);
    if shift <= 0 as c_int || (*h).eof != 0 {
        return;
    }
    let mut cur_frame: c_int =
        if (*h).first_frame + (*h).cur_size > frame - (*h).max_size + 1 as c_int {
            (*h).first_frame + (*h).cur_size
        } else {
            frame - (*h).max_size + 1 as c_int
        };
    (*h).first_frame = if (*h).first_frame + shift < cur_frame {
        (*h).first_frame + shift
    } else {
        cur_frame
    };
    (*h).cur_size = if (*h).cur_size - shift > 0 as c_int {
        (*h).cur_size - shift
    } else {
        0 as c_int
    };
    while (*h).cur_size < (*h).max_size {
        let mut temp: cli_pic_t = cli_pic_t {
            img: cli_image_t {
                csp: 0,
                width: 0,
                height: 0,
                planes: 0,
                plane: [0 as *mut uint8_t; 4],
                stride: [0; 4],
            },
            pts: 0,
            duration: 0,
            opaque: 0 as *mut c_void,
        };
        let mut cache: *mut cli_pic_t = *(*h).cache.offset(0);
        if (*h)
            .prev_filter
            .get_frame
            .expect("non-null function pointer")((*h).prev_hnd, &mut temp, cur_frame)
            != 0
            || x264_cli_pic_copy(cache, &mut temp) != 0
            || (*h)
                .prev_filter
                .release_frame
                .expect("non-null function pointer")(
                (*h).prev_hnd, &mut temp, cur_frame
            ) != 0
        {
            (*h).eof = cur_frame;
            return;
        }
        x264_10_frame_push(
            (*h).cache as *mut c_void as *mut *mut x264_frame_t,
            x264_10_frame_shift((*h).cache as *mut c_void as *mut *mut x264_frame_t),
        );
        cur_frame += 1;
        (*h).cur_size += 1;
    }
}
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: c_int,
) -> c_int {
    let mut h: *mut cache_hnd_t = handle as *mut cache_hnd_t;
    if frame < (*h).first_frame {
        x264_cli_log(
            b"cache_10\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"frame %d is before first cached frame %d \n\0" as *const u8 as *const c_char,
            frame,
            (*h).first_frame,
        );
        return -1;
    }
    fill_cache(h, frame);
    if frame > (*h).first_frame + (*h).cur_size - 1 as c_int {
        return -1;
    }
    let mut idx: c_int = frame
        - (if (*h).eof != 0 {
            (*h).eof - (*h).max_size
        } else {
            (*h).first_frame
        });
    *output = **(*h).cache.offset(idx as isize);
    return 0 as c_int;
}
#[c2rust::src_loc = "132:1"]
unsafe extern "C" fn release_frame(
    mut _handle: hnd_t,
    mut _pic: *mut cli_pic_t,
    mut _frame: c_int,
) -> c_int {
    return 0 as c_int;
}
#[c2rust::src_loc = "138:1"]
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut cache_hnd_t = handle as *mut cache_hnd_t;
    (*h).prev_filter.free.expect("non-null function pointer")((*h).prev_hnd);
    let mut i: c_int = 0 as c_int;
    while i < (*h).max_size {
        x264_cli_pic_clean(*(*h).cache.offset(i as isize));
        free(*(*h).cache.offset(i as isize) as *mut c_void);
        i += 1;
    }
    free((*h).cache as *mut c_void);
    free(h as *mut c_void);
}
#[no_mangle]
#[c2rust::src_loc = "151:18"]
static mut cache_10_filter: cli_vid_filter_t = cli_vid_filter_t {
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
