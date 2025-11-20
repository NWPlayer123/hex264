use core::ffi::{c_char, c_double, c_int, c_uint, c_void};

use crate::__stddef_null_h::NULL;
use crate::__stddef_size_t_h::size_t;
use crate::matroska_ebml_h::{
    mk_add_frame_data, mk_close, mk_create_writer, mk_set_frame_flags, mk_start_frame,
    mk_write_header, mk_writer, DS_PIXELS,
};
use crate::output_h::{cli_output_opt_t, cli_output_t};
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::{uint32_t, uint8_t};
use crate::stdlib_h::{calloc, free, malloc};
use crate::string_h::memcpy;
use crate::x264_h::{x264_nal_t, x264_param_t, x264_picture_t, X264_TYPE_B};
use crate::x264cli_h::hnd_t;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "29:9"]
struct mkv_hnd_t {
    w: *mut mk_writer,
    width: c_int,
    height: c_int,
    d_width: c_int,
    d_height: c_int,
    display_size_units: c_int,
    stereo_mode: c_int,
    frame_duration: int64_t,
    b_writing_frame: c_char,
    i_timebase_num: uint32_t,
    i_timebase_den: uint32_t,
}
#[c2rust::src_loc = "46:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut c_char,
    mut p_handle: *mut hnd_t,
    mut _opt: *mut cli_output_opt_t,
) -> c_int {
    *p_handle = NULL as hnd_t;
    let mut p_mkv: *mut mkv_hnd_t =
        calloc(1 as size_t, ::core::mem::size_of::<mkv_hnd_t>() as size_t) as *mut mkv_hnd_t;
    if p_mkv.is_null() {
        return -(1 as c_int);
    }
    (*p_mkv).w = mk_create_writer(psz_filename);
    if (*p_mkv).w.is_null() {
        free(p_mkv as *mut c_void);
        return -(1 as c_int);
    }
    *p_handle = p_mkv as hnd_t;
    return 0 as c_int;
}
#[c2rust::src_loc = "65:9"]
const STEREO_COUNT: c_int = 7 as c_int;
#[c2rust::src_loc = "66:22"]
static mut stereo_modes: [uint8_t; 7] = [
    5 as c_int as uint8_t,
    9 as c_int as uint8_t,
    7 as c_int as uint8_t,
    1 as c_int as uint8_t,
    3 as c_int as uint8_t,
    13 as c_int as uint8_t,
    0 as c_int as uint8_t,
];
#[c2rust::src_loc = "67:22"]
static mut stereo_w_div: [uint8_t; 7] = [
    1 as c_int as uint8_t,
    2 as c_int as uint8_t,
    1 as c_int as uint8_t,
    2 as c_int as uint8_t,
    1 as c_int as uint8_t,
    1 as c_int as uint8_t,
    1 as c_int as uint8_t,
];
#[c2rust::src_loc = "68:22"]
static mut stereo_h_div: [uint8_t; 7] = [
    1 as c_int as uint8_t,
    1 as c_int as uint8_t,
    2 as c_int as uint8_t,
    1 as c_int as uint8_t,
    2 as c_int as uint8_t,
    1 as c_int as uint8_t,
    1 as c_int as uint8_t,
];
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn set_param(mut handle: hnd_t, mut p_param: *mut x264_param_t) -> c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut dw: int64_t = 0;
    let mut dh: int64_t = 0;
    if (*p_param).i_fps_num > 0 as uint32_t && (*p_param).b_vfr_input == 0 {
        (*p_mkv).frame_duration = (*p_param).i_fps_den as int64_t * 1000000000 as c_int as int64_t
            / (*p_param).i_fps_num as int64_t;
    } else {
        (*p_mkv).frame_duration = 0 as int64_t;
    }
    (*p_mkv).width = (*p_param).i_width;
    dw = (*p_mkv).width as int64_t;
    (*p_mkv).height = (*p_param).i_height;
    dh = (*p_mkv).height as int64_t;
    (*p_mkv).display_size_units = DS_PIXELS;
    (*p_mkv).stereo_mode = -(1 as c_int);
    if let Some(frame_packing) = (*p_param).frame_packing {
        (*p_mkv).stereo_mode = stereo_modes[frame_packing as usize] as c_int;
        dw /= stereo_w_div[frame_packing as usize] as int64_t;
        dh /= stereo_h_div[frame_packing as usize] as int64_t;
    }
    if (*p_param).vui.i_sar_width != 0
        && (*p_param).vui.i_sar_height != 0
        && (*p_param).vui.i_sar_width != (*p_param).vui.i_sar_height
    {
        if (*p_param).vui.i_sar_width > (*p_param).vui.i_sar_height {
            dw =
                dw * (*p_param).vui.i_sar_width as int64_t / (*p_param).vui.i_sar_height as int64_t;
        } else {
            dh =
                dh * (*p_param).vui.i_sar_height as int64_t / (*p_param).vui.i_sar_width as int64_t;
        }
    }
    (*p_mkv).d_width = dw as c_int;
    (*p_mkv).d_height = dh as c_int;
    (*p_mkv).i_timebase_num = (*p_param).i_timebase_num;
    (*p_mkv).i_timebase_den = (*p_param).i_timebase_den;
    return 0 as c_int;
}
#[c2rust::src_loc = "116:1"]
unsafe extern "C" fn write_headers(mut handle: hnd_t, mut p_nal: *mut x264_nal_t) -> c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut sps_size: c_int = (*p_nal.offset(0 as c_int as isize)).i_payload - 4 as c_int;
    let mut pps_size: c_int = (*p_nal.offset(1 as c_int as isize)).i_payload - 4 as c_int;
    let mut sei_size: c_int = (*p_nal.offset(2 as c_int as isize)).i_payload;
    let mut sps: *mut uint8_t = (*p_nal.offset(0 as c_int as isize))
        .p_payload
        .offset(4 as c_int as isize);
    let mut pps: *mut uint8_t = (*p_nal.offset(1 as c_int as isize))
        .p_payload
        .offset(4 as c_int as isize);
    let mut sei: *mut uint8_t = (*p_nal.offset(2 as c_int as isize)).p_payload;
    let mut ret: c_int = 0;
    let mut avcC: *mut uint8_t = 0 as *mut uint8_t;
    let mut avcC_len: c_int = 0;
    if (*p_mkv).width == 0
        || (*p_mkv).height == 0
        || (*p_mkv).d_width == 0
        || (*p_mkv).d_height == 0
    {
        return -(1 as c_int);
    }
    avcC_len = 5 as c_int + 1 as c_int + 2 as c_int + sps_size + 1 as c_int + 2 as c_int + pps_size;
    avcC = malloc(avcC_len as size_t) as *mut uint8_t;
    if avcC.is_null() {
        return -(1 as c_int);
    }
    *avcC.offset(0 as c_int as isize) = 1 as uint8_t;
    *avcC.offset(1 as c_int as isize) = *sps.offset(1 as c_int as isize);
    *avcC.offset(2 as c_int as isize) = *sps.offset(2 as c_int as isize);
    *avcC.offset(3 as c_int as isize) = *sps.offset(3 as c_int as isize);
    *avcC.offset(4 as c_int as isize) = 0xff as uint8_t;
    *avcC.offset(5 as c_int as isize) = 0xe1 as uint8_t;
    *avcC.offset(6 as c_int as isize) = (sps_size >> 8 as c_int) as uint8_t;
    *avcC.offset(7 as c_int as isize) = sps_size as uint8_t;
    memcpy(
        avcC.offset(8 as c_int as isize) as *mut c_void,
        sps as *const c_void,
        sps_size as size_t,
    );
    *avcC.offset((8 as c_int + sps_size) as isize) = 1 as uint8_t;
    *avcC.offset((9 as c_int + sps_size) as isize) = (pps_size >> 8 as c_int) as uint8_t;
    *avcC.offset((10 as c_int + sps_size) as isize) = pps_size as uint8_t;
    memcpy(
        avcC.offset(11 as c_int as isize).offset(sps_size as isize) as *mut c_void,
        pps as *const c_void,
        pps_size as size_t,
    );
    ret = mk_write_header(
        (*p_mkv).w,
        b"x264 r3223M 0480cb0\0" as *const u8 as *const c_char,
        b"V_MPEG4/ISO/AVC\0" as *const u8 as *const c_char,
        avcC as *const c_void,
        avcC_len as c_uint,
        (*p_mkv).frame_duration,
        50000 as int64_t,
        (*p_mkv).width as c_uint,
        (*p_mkv).height as c_uint,
        (*p_mkv).d_width as c_uint,
        (*p_mkv).d_height as c_uint,
        (*p_mkv).display_size_units,
        (*p_mkv).stereo_mode,
    );
    free(avcC as *mut c_void);
    if ret < 0 as c_int {
        return ret;
    }
    if (*p_mkv).b_writing_frame == 0 {
        if mk_start_frame((*p_mkv).w) < 0 as c_int {
            return -(1 as c_int);
        }
        (*p_mkv).b_writing_frame = 1 as c_char;
    }
    if mk_add_frame_data((*p_mkv).w, sei as *const c_void, sei_size as c_uint) < 0 as c_int {
        return -(1 as c_int);
    }
    return sei_size + sps_size + pps_size;
}
#[c2rust::src_loc = "182:1"]
unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: c_int,
    mut p_picture: *mut x264_picture_t,
) -> c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    if (*p_mkv).b_writing_frame == 0 {
        if mk_start_frame((*p_mkv).w) < 0 as c_int {
            return -(1 as c_int);
        }
        (*p_mkv).b_writing_frame = 1 as c_char;
    }
    if mk_add_frame_data((*p_mkv).w, p_nalu as *const c_void, i_size as c_uint) < 0 as c_int {
        return -(1 as c_int);
    }
    let mut i_stamp: int64_t =
        ((*p_picture).i_pts as c_double * 1e9f64 * (*p_mkv).i_timebase_num as c_double
            / (*p_mkv).i_timebase_den as c_double
            + 0.5f64) as int64_t;
    (*p_mkv).b_writing_frame = 0 as c_char;
    if mk_set_frame_flags(
        (*p_mkv).w,
        i_stamp,
        (*p_picture).b_keyframe,
        ((*p_picture).i_type == X264_TYPE_B) as c_int,
    ) < 0 as c_int
    {
        return -(1 as c_int);
    }
    return i_size;
}
#[c2rust::src_loc = "206:1"]
unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut largest_pts: int64_t,
    mut second_largest_pts: int64_t,
) -> c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut ret: c_int = 0;
    let mut i_last_delta: int64_t = 0;
    i_last_delta = if (*p_mkv).i_timebase_den != 0 {
        (((largest_pts - second_largest_pts) * (*p_mkv).i_timebase_num as int64_t
            / (*p_mkv).i_timebase_den as int64_t) as c_double
            + 0.5f64) as int64_t
    } else {
        0 as int64_t
    };
    ret = mk_close((*p_mkv).w, i_last_delta);
    free(p_mkv as *mut c_void);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "221:20"]
static mut mkv_output: cli_output_t = cli_output_t {
    open_file: Some(
        open_file as unsafe extern "C" fn(*mut c_char, *mut hnd_t, *mut cli_output_opt_t) -> c_int,
    ),
    set_param: Some(set_param as unsafe extern "C" fn(hnd_t, *mut x264_param_t) -> c_int),
    write_headers: Some(write_headers as unsafe extern "C" fn(hnd_t, *mut x264_nal_t) -> c_int),
    write_frame: Some(
        write_frame
            as unsafe extern "C" fn(hnd_t, *mut uint8_t, c_int, *mut x264_picture_t) -> c_int,
    ),
    close_file: Some(close_file as unsafe extern "C" fn(hnd_t, int64_t, int64_t) -> c_int),
};
