use core::ffi::{c_char, c_int};
use core::ptr::addr_of_mut;

use crate::input_h::video_info_t;
use crate::strings_h::strcasecmp;
use crate::video_h::cli_vid_filter_t;
use crate::x264_h::{x264_param_t, X264_LOG_ERROR};
use crate::x264cli_h::{hnd_t, x264_cli_log};
#[c2rust::src_loc = "28:26"]
static mut first_filter: *mut cli_vid_filter_t =
    0 as *const cli_vid_filter_t as *mut cli_vid_filter_t;
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn register_vid_filter(mut new_filter: *mut cli_vid_filter_t) {
    let mut filter_i: *mut cli_vid_filter_t = first_filter;
    while !(*filter_i).next.is_null() {
        filter_i = (*filter_i).next;
    }
    (*filter_i).next = new_filter;
    (*new_filter).next = 0 as *mut cli_vid_filter_t;
}
#[no_mangle]
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn x264_register_vid_filters() {
    extern "C" {
        static mut source_filter: cli_vid_filter_t;
    }
    first_filter = addr_of_mut!(source_filter);
    extern "C" {
        static mut cache_8_filter: cli_vid_filter_t;
    }
    register_vid_filter(addr_of_mut!(cache_8_filter));
    extern "C" {
        static mut depth_8_filter: cli_vid_filter_t;
    }
    register_vid_filter(addr_of_mut!(depth_8_filter));
    extern "C" {
        static mut cache_10_filter: cli_vid_filter_t;
    }
    register_vid_filter(addr_of_mut!(cache_10_filter));
    extern "C" {
        static mut depth_10_filter: cli_vid_filter_t;
    }
    register_vid_filter(addr_of_mut!(depth_10_filter));
    extern "C" {
        static mut crop_filter: cli_vid_filter_t;
    }
    register_vid_filter(addr_of_mut!(crop_filter));
    extern "C" {
        static mut fix_vfr_pts_filter: cli_vid_filter_t;
    }
    register_vid_filter(addr_of_mut!(fix_vfr_pts_filter));
    extern "C" {
        static mut resize_filter: cli_vid_filter_t;
    }
    register_vid_filter(addr_of_mut!(resize_filter));
    extern "C" {
        static mut select_every_filter: cli_vid_filter_t;
    }
    register_vid_filter(addr_of_mut!(select_every_filter));
}
#[no_mangle]
#[c2rust::src_loc = "65:1"]
unsafe extern "C" fn x264_init_vid_filter(
    mut name: *const c_char,
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut c_char,
) -> c_int {
    let mut filter_i: *mut cli_vid_filter_t = first_filter;
    while !filter_i.is_null() && strcasecmp(name, (*filter_i).name) != 0 {
        filter_i = (*filter_i).next;
    }
    if filter_i.is_null() {
        x264_cli_log(
            b"x264\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"invalid filter `%s'\n\0" as *const u8 as *const c_char,
            name,
        );
        return -(1 as c_int);
    }
    if (*filter_i).init.expect("non-null function pointer")(handle, filter, info, param, opt_string)
        != 0
    {
        return -(1 as c_int);
    }
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "78:1"]
unsafe extern "C" fn x264_vid_filter_help(mut longhelp: c_int) {
    let mut filter_i: *mut cli_vid_filter_t = first_filter;
    while !filter_i.is_null() {
        if (*filter_i).help.is_some() {
            (*filter_i).help.expect("non-null function pointer")(longhelp);
        }
        filter_i = (*filter_i).next;
    }
}
