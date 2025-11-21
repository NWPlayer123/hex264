use core::ffi::{c_char, c_int, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::output_h::{cli_output_opt_t, cli_output_t};
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::uint8_t;
use crate::stdio_h::{fclose, fopen, fwrite, stdout};
use crate::string_h::strcmp;
use crate::x264_h::{x264_nal_t, x264_param_t, x264_picture_t};
use crate::x264cli_h::hnd_t;
use crate::FILE_h::FILE;
#[c2rust::src_loc = "29:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut c_char,
    mut p_handle: *mut hnd_t,
    mut _opt: *mut cli_output_opt_t,
) -> c_int {
    if strcmp(psz_filename, b"-\0" as *const u8 as *const c_char) == 0 {
        *p_handle = stdout as hnd_t;
    } else {
        *p_handle = fopen(psz_filename, b"w+b\0" as *const u8 as *const c_char) as hnd_t;
        if (*p_handle).is_null() {
            return -1;
        }
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "39:1"]
unsafe extern "C" fn set_param(mut _handle: hnd_t, mut _p_param: *mut x264_param_t) -> c_int {
    return 0 as c_int;
}
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn write_headers(mut handle: hnd_t, mut p_nal: *mut x264_nal_t) -> c_int {
    let mut size: c_int =
        (*p_nal.offset(0)).i_payload + (*p_nal.offset(1)).i_payload + (*p_nal.offset(2)).i_payload;
    if fwrite(
        (*p_nal.offset(0)).p_payload as *const c_void,
        size as size_t,
        1 as size_t,
        handle as *mut FILE,
    ) != 0
    {
        return size;
    }
    return -1;
}
#[c2rust::src_loc = "53:1"]
unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: c_int,
    mut _p_picture: *mut x264_picture_t,
) -> c_int {
    if fwrite(
        p_nalu as *const c_void,
        i_size as size_t,
        1 as size_t,
        handle as *mut FILE,
    ) != 0
    {
        return i_size;
    }
    return -1;
}
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut _largest_pts: int64_t,
    mut _second_largest_pts: int64_t,
) -> c_int {
    if handle.is_null() || handle == stdout as hnd_t {
        return 0 as c_int;
    }
    return fclose(handle as *mut FILE);
}
#[no_mangle]
#[c2rust::src_loc = "68:20"]
static mut raw_output: cli_output_t = cli_output_t {
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
