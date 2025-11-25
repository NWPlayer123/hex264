use core::ffi::{c_char, c_double, c_int, c_uint, c_ulong, c_void};

use crate::__stddef_size_t_h::size_t;
use crate::flv_bytestream_h::{
    flv_append_data, flv_buffer, flv_create_writer, flv_dbl2int, flv_flush_data,
    flv_put_amf_double, flv_put_amf_string, flv_put_be16, flv_put_be24, flv_put_be32, flv_put_byte,
    flv_put_tag, flv_rewrite_amf_be24, AMF_DATA_TYPE_MIXEDARRAY, AMF_DATA_TYPE_STRING,
    AMF_END_OF_OBJECT, FLV_CODECID_H264, FLV_FRAME_INTER, FLV_FRAME_KEY, FLV_TAG_TYPE_META,
    FLV_TAG_TYPE_VIDEO,
};
use crate::osdep_h::{endian_fix64, x264_is_regular_file};
use crate::output_h::{cli_output_opt_t, cli_output_t};
use crate::src::x264::x264_cli_log;
use crate::stdint_intn_h::int64_t;
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use crate::stdio_h::{fclose, fseeko, ftello, fwrite, SEEK_SET};
use crate::stdlib_h::{calloc, free, malloc};
use crate::string_h::memcpy;
use crate::types_h::__off64_t;
use crate::x264_h::{
    x264_nal_t, x264_param_t, x264_picture_t, BPyramid, X264_LOG_INFO, X264_LOG_WARNING,
};
use crate::x264cli_h::hnd_t;
use crate::FILE_h::FILE;
use crate::__stddef_null_h::NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "35:9"]
struct flv_hnd_t {
    c: *mut flv_buffer,
    sei: *mut uint8_t,
    sei_len: c_int,
    i_fps_num: int64_t,
    i_fps_den: int64_t,
    i_framenum: int64_t,
    i_framerate_pos: uint64_t,
    i_duration_pos: uint64_t,
    i_filesize_pos: uint64_t,
    i_bitrate_pos: uint64_t,
    b_write_length: uint8_t,
    i_prev_dts: int64_t,
    i_prev_cts: int64_t,
    i_delay_time: int64_t,
    i_init_delta: int64_t,
    i_delay_frames: c_int,
    d_timebase: c_double,
    vfr_input: bool,
    b_dts_compress: c_int,
    start: c_uint,
}
#[c2rust::src_loc = "65:1"]
unsafe extern "C" fn write_header(mut c: *mut flv_buffer) -> c_int {
    flv_put_tag(c, b"FLV\0" as *const u8 as *const c_char);
    flv_put_byte(c, 1 as uint8_t);
    flv_put_byte(c, 1 as uint8_t);
    flv_put_be32(c, 9 as uint32_t);
    flv_put_be32(c, 0 as uint32_t);
    return flv_flush_data(c);
}
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut c_char,
    mut p_handle: *mut hnd_t,
    mut opt: *mut cli_output_opt_t,
) -> c_int {
    let mut p_flv: *mut flv_hnd_t =
        calloc(1 as size_t, ::core::mem::size_of::<flv_hnd_t>() as size_t) as *mut flv_hnd_t;
    if !p_flv.is_null() {
        let mut c: *mut flv_buffer = flv_create_writer(psz_filename);
        if !c.is_null() {
            if write_header(c) == 0 {
                (*p_flv).c = c;
                (*p_flv).b_dts_compress = (*opt).use_dts_compress;
                *p_handle = p_flv as hnd_t;
                return 0 as c_int;
            }
            fclose((*c).fp);
            free((*c).data as *mut c_void);
            free(c as *mut c_void);
        }
        free(p_flv as *mut c_void);
    }
    *p_handle = NULL as hnd_t;
    return -1;
}
#[c2rust::src_loc = "103:1"]
unsafe extern "C" fn set_param(mut handle: hnd_t, mut p_param: *mut x264_param_t) -> c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    flv_put_byte(c, FLV_TAG_TYPE_META as c_int as uint8_t);
    let mut start: c_int = (*c).d_cur as c_int;
    flv_put_be24(c, 0 as uint32_t);
    flv_put_be24(c, 0 as uint32_t);
    flv_put_be32(c, 0 as uint32_t);
    flv_put_byte(c, AMF_DATA_TYPE_STRING as c_int as uint8_t);
    flv_put_amf_string(c, b"onMetaData\0" as *const u8 as *const c_char);
    flv_put_byte(c, AMF_DATA_TYPE_MIXEDARRAY as c_int as uint8_t);
    flv_put_be32(c, 7 as uint32_t);
    flv_put_amf_string(c, b"width\0" as *const u8 as *const c_char);
    flv_put_amf_double(c, (*p_param).width as c_double);
    flv_put_amf_string(c, b"height\0" as *const u8 as *const c_char);
    flv_put_amf_double(c, (*p_param).height as c_double);
    flv_put_amf_string(c, b"framerate\0" as *const u8 as *const c_char);
    if !(*p_param).vfr_input {
        flv_put_amf_double(
            c,
            (*p_param).i_fps_num as c_double / (*p_param).i_fps_den as c_double,
        );
    } else {
        (*p_flv).i_framerate_pos = ((*c).d_cur as uint64_t)
            .wrapping_add((*c).d_total)
            .wrapping_add(1 as uint64_t);
        flv_put_amf_double(c, 0 as c_int as c_double);
    }
    flv_put_amf_string(c, b"videocodecid\0" as *const u8 as *const c_char);
    flv_put_amf_double(c, FLV_CODECID_H264 as c_int as c_double);
    flv_put_amf_string(c, b"duration\0" as *const u8 as *const c_char);
    (*p_flv).i_duration_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as uint64_t);
    flv_put_amf_double(c, 0 as c_int as c_double);
    flv_put_amf_string(c, b"filesize\0" as *const u8 as *const c_char);
    (*p_flv).i_filesize_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as uint64_t);
    flv_put_amf_double(c, 0 as c_int as c_double);
    flv_put_amf_string(c, b"videodatarate\0" as *const u8 as *const c_char);
    (*p_flv).i_bitrate_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as uint64_t);
    flv_put_amf_double(c, 0 as c_int as c_double);
    flv_put_amf_string(c, b"\0" as *const u8 as *const c_char);
    flv_put_byte(c, AMF_END_OF_OBJECT as uint8_t);
    let mut length: c_uint = (*c).d_cur.wrapping_sub(start as c_uint);
    flv_rewrite_amf_be24(c, length.wrapping_sub(10 as c_uint), start as c_uint);
    flv_put_be32(c, (length as uint32_t).wrapping_add(1 as uint32_t));
    (*p_flv).i_fps_num = (*p_param).i_fps_num as int64_t;
    (*p_flv).i_fps_den = (*p_param).i_fps_den as int64_t;
    (*p_flv).d_timebase =
        (*p_param).i_timebase_num as c_double / (*p_param).i_timebase_den as c_double;
    (*p_flv).vfr_input = (*p_param).vfr_input;
    (*p_flv).i_delay_frames = if (*p_param).i_bframe != 0 {
        if (*p_param).bframe_pyramid != BPyramid::None {
            2 as c_int
        } else {
            1 as c_int
        }
    } else {
        0 as c_int
    };
    return 0 as c_int;
}
#[c2rust::src_loc = "169:1"]
unsafe extern "C" fn write_headers(mut handle: hnd_t, mut p_nal: *mut x264_nal_t) -> c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    let mut sps_size: c_int = (*p_nal.offset(0)).i_payload;
    let mut pps_size: c_int = (*p_nal.offset(1)).i_payload;
    let mut sei_size: c_int = (*p_nal.offset(2)).i_payload;
    (*p_flv).sei = malloc(sei_size as size_t) as *mut uint8_t;
    if (*p_flv).sei.is_null() {
        return -1;
    }
    (*p_flv).sei_len = sei_size;
    memcpy(
        (*p_flv).sei as *mut c_void,
        (*p_nal.offset(2)).p_payload as *const c_void,
        sei_size as size_t,
    );
    let mut sps: *mut uint8_t = (*p_nal.offset(0)).p_payload.offset(4);
    flv_put_byte(c, FLV_TAG_TYPE_VIDEO as c_int as uint8_t);
    flv_put_be24(c, 0 as uint32_t);
    flv_put_be24(c, 0 as uint32_t);
    flv_put_byte(c, 0 as uint8_t);
    flv_put_be24(c, 0 as uint32_t);
    (*p_flv).start = (*c).d_cur;
    flv_put_byte(
        c,
        (FLV_FRAME_KEY as c_int | FLV_CODECID_H264 as c_int) as uint8_t,
    );
    flv_put_byte(c, 0 as uint8_t);
    flv_put_be24(c, 0 as uint32_t);
    flv_put_byte(c, 1 as uint8_t);
    flv_put_byte(c, *sps.offset(1));
    flv_put_byte(c, *sps.offset(2));
    flv_put_byte(c, *sps.offset(3));
    flv_put_byte(c, 0xff as uint8_t);
    flv_put_byte(c, 0xe1 as uint8_t);
    flv_put_be16(c, (sps_size - 4 as c_int) as uint16_t);
    flv_append_data(c, sps, (sps_size - 4 as c_int) as c_uint);
    flv_put_byte(c, 1 as uint8_t);
    flv_put_be16(c, (pps_size - 4 as c_int) as uint16_t);
    flv_append_data(
        c,
        (*p_nal.offset(1)).p_payload.offset(4),
        (pps_size - 4 as c_int) as c_uint,
    );
    let mut length: c_uint = (*c).d_cur.wrapping_sub((*p_flv).start);
    flv_rewrite_amf_be24(c, length, (*p_flv).start.wrapping_sub(10 as c_uint));
    flv_put_be32(c, (length as uint32_t).wrapping_add(11 as uint32_t));
    if flv_flush_data(c) < 0 as c_int {
        return -1;
    }
    return sei_size + sps_size + pps_size;
}
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: c_int,
    mut p_picture: *mut x264_picture_t,
) -> c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    if (*p_flv).i_framenum == 0 {
        (*p_flv).i_delay_time = (*p_picture).i_dts * -1 as int64_t;
        if (*p_flv).b_dts_compress == 0 && (*p_flv).i_delay_time != 0 {
            x264_cli_log(
                b"flv\0" as *const u8 as *const c_char,
                X264_LOG_INFO,
                b"initial delay %ld ms\n\0" as *const u8 as *const c_char,
                (((*p_picture).i_pts + (*p_flv).i_delay_time) as c_double
                    * (*p_flv).d_timebase
                    * 1000 as c_int as c_double
                    + 0.5f64) as int64_t,
            );
        }
    }
    let mut dts: int64_t = 0;
    let mut cts: int64_t = 0;
    let mut offset: int64_t = 0;
    if (*p_flv).b_dts_compress != 0 {
        if (*p_flv).i_framenum == 1 as int64_t {
            (*p_flv).i_init_delta = (((*p_picture).i_dts + (*p_flv).i_delay_time) as c_double
                * (*p_flv).d_timebase
                * 1000 as c_int as c_double
                + 0.5f64) as int64_t;
        }
        dts = if (*p_flv).i_framenum > (*p_flv).i_delay_frames as int64_t {
            ((*p_picture).i_dts as c_double * (*p_flv).d_timebase * 1000 as c_int as c_double
                + 0.5f64) as int64_t
        } else {
            (*p_flv).i_framenum * (*p_flv).i_init_delta
                / ((*p_flv).i_delay_frames + 1 as c_int) as int64_t
        };
        cts = ((*p_picture).i_pts as c_double * (*p_flv).d_timebase * 1000 as c_int as c_double
            + 0.5f64) as int64_t;
    } else {
        dts = (((*p_picture).i_dts + (*p_flv).i_delay_time) as c_double
            * (*p_flv).d_timebase
            * 1000 as c_int as c_double
            + 0.5f64) as int64_t;
        cts = (((*p_picture).i_pts + (*p_flv).i_delay_time) as c_double
            * (*p_flv).d_timebase
            * 1000 as c_int as c_double
            + 0.5f64) as int64_t;
    }
    offset = cts - dts;
    if (*p_flv).i_framenum != 0 {
        if (*p_flv).i_prev_dts == dts {
            x264_cli_log(
                b"flv\0" as *const u8 as *const c_char,
                X264_LOG_WARNING,
                b"duplicate DTS %ld generated by rounding\n               decoding framerate cannot exceed 1000fps\n\0"
                    as *const u8 as *const c_char,
                dts,
            );
        }
        if (*p_flv).i_prev_cts == cts {
            x264_cli_log(
                b"flv\0" as *const u8 as *const c_char,
                X264_LOG_WARNING,
                b"duplicate CTS %ld generated by rounding\n               composition framerate cannot exceed 1000fps\n\0"
                    as *const u8 as *const c_char,
                cts,
            );
        }
    }
    (*p_flv).i_prev_dts = dts;
    (*p_flv).i_prev_cts = cts;
    flv_put_byte(c, FLV_TAG_TYPE_VIDEO as c_int as uint8_t);
    flv_put_be24(c, 0 as uint32_t);
    flv_put_be24(c, dts as uint32_t);
    flv_put_byte(c, (dts >> 24 as c_int) as uint8_t);
    flv_put_be24(c, 0 as uint32_t);
    (*p_flv).start = (*c).d_cur;
    flv_put_byte(
        c,
        ((if (*p_picture).keyframe {
            FLV_FRAME_KEY as c_int
        } else {
            FLV_FRAME_INTER as c_int
        }) | FLV_CODECID_H264 as c_int) as uint8_t,
    );
    flv_put_byte(c, 1 as uint8_t);
    flv_put_be24(c, offset as uint32_t);
    if !(*p_flv).sei.is_null() {
        flv_append_data(c, (*p_flv).sei, (*p_flv).sei_len as c_uint);
        free((*p_flv).sei as *mut c_void);
        (*p_flv).sei = 0 as *mut uint8_t;
    }
    flv_append_data(c, p_nalu, i_size as c_uint);
    let mut length: c_uint = (*c).d_cur.wrapping_sub((*p_flv).start);
    flv_rewrite_amf_be24(c, length, (*p_flv).start.wrapping_sub(10 as c_uint));
    flv_put_be32(c, (11 as uint32_t).wrapping_add(length as uint32_t));
    if flv_flush_data(c) < 0 as c_int {
        return -1;
    }
    (*p_flv).i_framenum += 1;
    return i_size;
}
#[c2rust::src_loc = "304:1"]
unsafe extern "C" fn rewrite_amf_double(
    mut fp: *mut FILE,
    mut position: uint64_t,
    mut value: c_double,
) -> c_int {
    let mut x: uint64_t = endian_fix64(flv_dbl2int(value));
    return if fseeko(fp, position as __off64_t, SEEK_SET) == 0
        && fwrite(
            &mut x as *mut uint64_t as *const c_void,
            8 as size_t,
            1 as size_t,
            fp,
        ) == 1 as c_ulong
    {
        0 as c_int
    } else {
        -1
    };
}
#[c2rust::src_loc = "317:1"]
unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut largest_pts: int64_t,
    mut second_largest_pts: int64_t,
) -> c_int {
    let mut total_duration: c_double = 0.;
    let mut current_block: u64;
    let mut ret: c_int = -1;
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    if !(flv_flush_data(c) < 0 as c_int) {
        total_duration = 0.;
        if (*p_flv).i_framenum == 1 as int64_t {
            total_duration = if (*p_flv).i_fps_num != 0 {
                (*p_flv).i_fps_den as c_double / (*p_flv).i_fps_num as c_double
            } else {
                0 as c_int as c_double
            };
        } else {
            total_duration =
                (2 as int64_t * largest_pts - second_largest_pts) as c_double * (*p_flv).d_timebase;
        }
        if x264_is_regular_file((*c).fp) != 0 && total_duration > 0 as c_int as c_double {
            let mut framerate: c_double = 0.;
            let mut filesize: int64_t = ftello((*c).fp) as int64_t;
            if (*p_flv).i_framerate_pos != 0 {
                framerate = (*p_flv).i_framenum as c_double / total_duration;
                if rewrite_amf_double((*c).fp, (*p_flv).i_framerate_pos, framerate) < 0 as c_int {
                    current_block = 8311716385155032230;
                } else {
                    current_block = 13586036798005543211;
                }
            } else {
                current_block = 13586036798005543211;
            }
            match current_block {
                8311716385155032230 => {}
                _ => {
                    if rewrite_amf_double((*c).fp, (*p_flv).i_duration_pos, total_duration)
                        < 0 as c_int
                    {
                        current_block = 8311716385155032230;
                    } else if rewrite_amf_double(
                        (*c).fp,
                        (*p_flv).i_filesize_pos,
                        filesize as c_double,
                    ) < 0 as c_int
                    {
                        current_block = 8311716385155032230;
                    } else if rewrite_amf_double(
                        (*c).fp,
                        (*p_flv).i_bitrate_pos,
                        filesize as c_double * 8.0f64
                            / (total_duration * 1000 as c_int as c_double),
                    ) < 0 as c_int
                    {
                        current_block = 8311716385155032230;
                    } else {
                        current_block = 224731115979188411;
                    }
                }
            }
        } else {
            current_block = 224731115979188411;
        }
        match current_block {
            8311716385155032230 => {}
            _ => {
                ret = 0 as c_int;
            }
        }
    }
    fclose((*c).fp);
    free((*c).data as *mut c_void);
    free(c as *mut c_void);
    free(p_flv as *mut c_void);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "358:20"]
static mut flv_output: cli_output_t = cli_output_t {
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
