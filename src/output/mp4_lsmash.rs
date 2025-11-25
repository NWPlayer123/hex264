use core::ffi::{c_char, c_double, c_int, c_uint, c_void};

use log::error;

use crate::__stddef_size_t_h::size_t;
use crate::lsmash_h::{
    lsmash_add_codec_specific_data, lsmash_add_sample_entry, lsmash_adhoc_remux_t,
    lsmash_append_h264_parameter_set, lsmash_append_sample, lsmash_brand_type,
    lsmash_cleanup_summary, lsmash_close_file, lsmash_codec_specific_t,
    lsmash_create_codec_specific_data, lsmash_create_explicit_timeline_map,
    lsmash_create_fragment_movie, lsmash_create_root, lsmash_create_sample, lsmash_create_summary,
    lsmash_create_track, lsmash_destroy_codec_specific_data, lsmash_destroy_root, lsmash_edit_t,
    lsmash_file_mode, lsmash_file_parameters_t, lsmash_finish_movie, lsmash_flush_pooled_samples,
    lsmash_get_media_timescale, lsmash_get_movie_timescale, lsmash_h264_specific_parameters_t,
    lsmash_initialize_media_parameters, lsmash_initialize_movie_parameters,
    lsmash_initialize_track_parameters, lsmash_media_parameters_t, lsmash_media_type,
    lsmash_modify_explicit_timeline_map, lsmash_movie_parameters_t, lsmash_open_file,
    lsmash_random_access_flag, lsmash_root_t, lsmash_sample_t, lsmash_set_file,
    lsmash_set_media_parameters, lsmash_set_movie_parameters, lsmash_set_track_parameters,
    lsmash_summary_t, lsmash_track_mode, lsmash_track_parameters_t, lsmash_video_summary_t,
    H264_PARAMETER_SET_TYPE_PPS, H264_PARAMETER_SET_TYPE_SPS, ISOM_BRAND_TYPE_AVC1,
    ISOM_BRAND_TYPE_ISO6, ISOM_BRAND_TYPE_ISOM, ISOM_BRAND_TYPE_MP41, ISOM_BRAND_TYPE_MP42,
    ISOM_CODEC_TYPE_AVC1_VIDEO, ISOM_EDIT_DURATION_UNKNOWN32, ISOM_EDIT_MODE_NORMAL,
    ISOM_MATRIX_INDEX_UNSPECIFIED, ISOM_MEDIA_HANDLER_TYPE_VIDEO_TRACK,
    ISOM_SAMPLE_RANDOM_ACCESS_FLAG_NONE, ISOM_SAMPLE_RANDOM_ACCESS_FLAG_SYNC, ISOM_TRACK_ENABLED,
    ISOM_TRACK_IN_MOVIE, ISOM_TRACK_IN_PREVIEW, LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_H264,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_H264_BITRATE,
    LSMASH_CODEC_SPECIFIC_FORMAT_STRUCTURED, LSMASH_FILE_MODE_FRAGMENTED,
    LSMASH_SUMMARY_TYPE_VIDEO,
};
use crate::osdep_h::{x264_is_regular_file, x264_is_regular_file_path};
use crate::output_h::{cli_output_opt_t, cli_output_t};
use crate::src::x264::x264_cli_log;
use crate::stdint_intn_h::{int32_t, int64_t};
use crate::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use crate::stdio_h::{fclose, fopen};
use crate::stdlib_h::{calloc, free, malloc};
use crate::string_h::{memcpy, strcmp};
use crate::x264_h::{x264_nal_t, x264_param_t, x264_picture_t, BPyramid, X264_LOG_ERROR};
use crate::x264cli_h::hnd_t;
use crate::FILE_h::FILE;
use crate::__stddef_null_h::NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "66:9"]
struct mp4_hnd_t {
    p_root: *mut lsmash_root_t,
    summary: *mut lsmash_video_summary_t,
    file_param: lsmash_file_parameters_t,
    i_track: uint32_t,
    i_sample_entry: uint32_t,

    i_movie_timescale: uint32_t,
    i_video_timescale: uint32_t,
    /// The duration between each individual frame.
    frame_duration: u32,

    i_start_offset: int64_t,
    i_first_cts: uint64_t,
    i_prev_dts: uint64_t,
    i_init_delta: int64_t,

    b_dts_compress: c_int,
    i_dts_compress_multiplier: c_int,
    i_delay_frames: c_int,

    p_sei_buffer: *mut uint8_t,
    i_sei_size: uint32_t,

    b_stdout: c_int,
    b_fragments: c_int,
    b_use_recovery: c_int,

    i_numframe: c_int,
}
#[c2rust::src_loc = "33:9"]
const H264_NALU_LENGTH_SIZE: c_int = 4 as c_int;
#[c2rust::src_loc = "93:1"]
unsafe extern "C" fn remove_mp4_hnd(mut handle: hnd_t) {
    let mut p_mp4: *mut mp4_hnd_t = handle as *mut mp4_hnd_t;
    if p_mp4.is_null() {
        return;
    }
    lsmash_cleanup_summary((*p_mp4).summary as *mut lsmash_summary_t);
    lsmash_close_file(&mut (*p_mp4).file_param);
    lsmash_destroy_root((*p_mp4).p_root);
    free((*p_mp4).p_sei_buffer as *mut c_void);
    free(p_mp4 as *mut c_void);
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut largest_pts: int64_t,
    mut second_largest_pts: int64_t,
) -> c_int {
    let mut p_mp4: *mut mp4_hnd_t = handle as *mut mp4_hnd_t;
    if p_mp4.is_null() {
        return 0 as c_int;
    }
    if !(*p_mp4).p_root.is_null() {
        let mut actual_duration: c_double = 0 as c_int as c_double;
        if (*p_mp4).i_track != 0 {
            let mut last_delta: uint32_t = (largest_pts - second_largest_pts) as uint32_t;
            if lsmash_flush_pooled_samples(
                (*p_mp4).p_root,
                (*p_mp4).i_track,
                last_delta.max(1) * (*p_mp4).frame_duration,
            ) != 0
            {
                error!("failed to flush the rest of samples.");
            }
            if (*p_mp4).i_movie_timescale != 0 && (*p_mp4).i_video_timescale != 0 {
                let end_time = (largest_pts + last_delta as i64) as u64;
                let duration_ticks = end_time * u64::from((*p_mp4).frame_duration);
                actual_duration = duration_ticks as f64 / (*p_mp4).i_video_timescale as f64
                    * (*p_mp4).i_movie_timescale as f64;
            } else {
                error!("timescale is broken.");
            }
            let mut edit: lsmash_edit_t = lsmash_edit_t {
                duration: 0,
                start_time: 0,
                rate: 0,
            };
            edit.duration = actual_duration as uint64_t;
            edit.start_time = (*p_mp4).i_first_cts as int64_t;
            edit.rate = ISOM_EDIT_MODE_NORMAL as int32_t;
            if (*p_mp4).b_fragments == 0 {
                if lsmash_create_explicit_timeline_map((*p_mp4).p_root, (*p_mp4).i_track, edit) != 0
                {
                    error!("failed to set timeline map for video.");
                }
            } else if (*p_mp4).b_stdout == 0 {
                if lsmash_modify_explicit_timeline_map(
                    (*p_mp4).p_root,
                    (*p_mp4).i_track,
                    1 as uint32_t,
                    edit,
                ) != 0
                {
                    error!("failed to update timeline map for video.");
                }
            }
        }
        if lsmash_finish_movie((*p_mp4).p_root, 0 as *mut lsmash_adhoc_remux_t) != 0 {
            error!("failed to finish movie.");
        }
    }
    remove_mp4_hnd(p_mp4 as hnd_t);
    return 0 as c_int;
}
#[c2rust::src_loc = "163:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut c_char,
    mut p_handle: *mut hnd_t,
    mut opt: *mut cli_output_opt_t,
) -> c_int {
    *p_handle = NULL as hnd_t;
    let mut b_regular: c_int = strcmp(psz_filename, b"-\0" as *const u8 as *const c_char);
    b_regular = (b_regular != 0 && x264_is_regular_file_path(psz_filename) != 0) as c_int;
    if b_regular != 0 {
        let mut fh: *mut FILE =
            fopen(psz_filename, b"wb\0" as *const u8 as *const c_char) as *mut FILE;
        if fh.is_null() {
            error!("cannot open output file {psz_filename:?}.");
            return -1;
        }
        b_regular = x264_is_regular_file(fh);
        fclose(fh);
    }
    let mut p_mp4: *mut mp4_hnd_t =
        calloc(1 as size_t, ::core::mem::size_of::<mp4_hnd_t>() as size_t) as *mut mp4_hnd_t;
    if p_mp4.is_null() {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"failed to allocate memory for muxer information.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    (*p_mp4).b_dts_compress = (*opt).use_dts_compress;
    (*p_mp4).b_use_recovery = 0 as c_int;
    (*p_mp4).b_fragments = (b_regular == 0) as c_int;
    (*p_mp4).b_stdout = (strcmp(psz_filename, b"-\0" as *const u8 as *const c_char) == 0) as c_int;
    (*p_mp4).p_root = lsmash_create_root();
    if (*p_mp4).p_root.is_null() {
        remove_mp4_hnd(p_mp4 as hnd_t);
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"failed to create root.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    if lsmash_open_file(psz_filename, 0 as c_int, &mut (*p_mp4).file_param) < 0 as c_int {
        remove_mp4_hnd(p_mp4 as hnd_t);
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"failed to open an output file.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    if (*p_mp4).b_fragments != 0 {
        (*p_mp4).file_param.mode = ::core::mem::transmute::<c_uint, lsmash_file_mode>(
            (*p_mp4).file_param.mode as c_uint | LSMASH_FILE_MODE_FRAGMENTED as c_int as c_uint,
        );
    }
    (*p_mp4).summary =
        lsmash_create_summary(LSMASH_SUMMARY_TYPE_VIDEO) as *mut lsmash_video_summary_t;
    if (*p_mp4).summary.is_null() {
        remove_mp4_hnd(p_mp4 as hnd_t);
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"failed to allocate memory for summary information of video.\n\0" as *const u8
                as *const c_char,
        );
        return -1;
    }
    (*(*p_mp4).summary).sample_type = ISOM_CODEC_TYPE_AVC1_VIDEO;
    *p_handle = p_mp4 as hnd_t;
    return 0 as c_int;
}
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn set_param(mut handle: hnd_t, mut p_param: *mut x264_param_t) -> c_int {
    let mut p_mp4: *mut mp4_hnd_t = handle as *mut mp4_hnd_t;
    let mut i_media_timescale: uint64_t = 0;
    (*p_mp4).i_delay_frames = if (*p_param).i_bframe != 0 {
        if (*p_param).bframe_pyramid != BPyramid::None {
            2 as c_int
        } else {
            1 as c_int
        }
    } else {
        0 as c_int
    };
    (*p_mp4).i_dts_compress_multiplier =
        (*p_mp4).b_dts_compress * (*p_mp4).i_delay_frames + 1 as c_int;
    i_media_timescale = ((*p_param).i_timebase_den as uint64_t)
        .wrapping_mul((*p_mp4).i_dts_compress_multiplier as uint64_t);
    (*p_mp4).frame_duration =
        (*p_param).i_timebase_num * ((*p_mp4).i_dts_compress_multiplier as u32);
    if i_media_timescale > 4294967295 as uint64_t {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"MP4 media timescale %lu exceeds maximum\n\0" as *const u8 as *const c_char,
            i_media_timescale,
        );
        return -1;
    }
    let mut brands: [lsmash_brand_type; 6] = [
        0 as lsmash_brand_type,
        0 as lsmash_brand_type,
        0 as lsmash_brand_type,
        0 as lsmash_brand_type,
        0 as lsmash_brand_type,
        0 as lsmash_brand_type,
    ];
    let mut brand_count: uint32_t = 0 as uint32_t;
    let fresh0 = brand_count;
    brand_count = brand_count.wrapping_add(1);
    brands[fresh0 as usize] = ISOM_BRAND_TYPE_MP42;
    let fresh1 = brand_count;
    brand_count = brand_count.wrapping_add(1);
    brands[fresh1 as usize] = ISOM_BRAND_TYPE_MP41;
    let fresh2 = brand_count;
    brand_count = brand_count.wrapping_add(1);
    brands[fresh2 as usize] = ISOM_BRAND_TYPE_ISOM;
    if (*p_mp4).b_use_recovery != 0 {
        let fresh3 = brand_count;
        brand_count = brand_count.wrapping_add(1);
        brands[fresh3 as usize] = ISOM_BRAND_TYPE_AVC1;
        if (*p_param).open_gop {
            let fresh4 = brand_count;
            brand_count = brand_count.wrapping_add(1);
            brands[fresh4 as usize] = ISOM_BRAND_TYPE_ISO6;
        }
    }
    let mut file_param: *mut lsmash_file_parameters_t = &mut (*p_mp4).file_param;
    (*file_param).major_brand = brands[0];
    (*file_param).brands = brands.as_mut_ptr();
    (*file_param).brand_count = brand_count;
    (*file_param).minor_version = 0 as uint32_t;
    if lsmash_set_file((*p_mp4).p_root, file_param).is_null() {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"failed to add an output file into a ROOT.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    let mut movie_param: lsmash_movie_parameters_t = lsmash_movie_parameters_t {
        timescale: 0,
        duration: 0,
        number_of_tracks: 0,
        playback_rate: 0,
        playback_volume: 0,
        preview_time: 0,
        preview_duration: 0,
        poster_time: 0,
    };
    lsmash_initialize_movie_parameters(&mut movie_param);
    if lsmash_set_movie_parameters((*p_mp4).p_root, &mut movie_param) != 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"failed to set movie parameters.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    (*p_mp4).i_movie_timescale = lsmash_get_movie_timescale((*p_mp4).p_root);
    if (*p_mp4).i_movie_timescale == 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"movie timescale is broken.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    (*p_mp4).i_track = lsmash_create_track((*p_mp4).p_root, ISOM_MEDIA_HANDLER_TYPE_VIDEO_TRACK);
    if (*p_mp4).i_track == 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"failed to create a video track.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    (*(*p_mp4).summary).width = (*p_param).width as uint32_t;
    (*(*p_mp4).summary).height = (*p_param).height as uint32_t;
    let mut i_display_width: uint32_t = ((*p_param).width << 16 as c_int) as uint32_t;
    let mut i_display_height: uint32_t = ((*p_param).height << 16 as c_int) as uint32_t;
    if (*p_param).vui.i_sar_width != 0 && (*p_param).vui.i_sar_height != 0 {
        let mut sar: c_double =
            (*p_param).vui.i_sar_width as c_double / (*p_param).vui.i_sar_height as c_double;
        if sar > 1.0f64 {
            i_display_width = (i_display_width as c_double * sar) as uint32_t;
        } else {
            i_display_height = (i_display_height as c_double / sar) as uint32_t;
        }
        (*(*p_mp4).summary).par_h = (*p_param).vui.i_sar_width as uint32_t;
        (*(*p_mp4).summary).par_v = (*p_param).vui.i_sar_height as uint32_t;
    }
    (*(*p_mp4).summary).color.primaries_index = (*p_param).vui.i_colorprim as uint16_t;
    (*(*p_mp4).summary).color.transfer_index = (*p_param).vui.i_transfer as uint16_t;
    (*(*p_mp4).summary).color.matrix_index = (if (*p_param).vui.i_colmatrix >= 0 as c_int {
        (*p_param).vui.i_colmatrix
    } else {
        ISOM_MATRIX_INDEX_UNSPECIFIED as c_int
    }) as uint16_t;
    (*(*p_mp4).summary).color.full_range = (if (*p_param).vui.b_fullrange >= 0 as c_int {
        (*p_param).vui.b_fullrange
    } else {
        0 as c_int
    }) as uint8_t;
    let mut track_param: lsmash_track_parameters_t = lsmash_track_parameters_t {
        mode: 0 as lsmash_track_mode,
        track_ID: 0,
        duration: 0,
        alternate_group: 0,
        video_layer: 0,
        audio_volume: 0,
        matrix: [0; 9],
        display_width: 0,
        display_height: 0,
        aperture_modes: 0,
    };
    lsmash_initialize_track_parameters(&mut track_param);
    let mut track_mode: lsmash_track_mode = (ISOM_TRACK_ENABLED as c_int
        | ISOM_TRACK_IN_MOVIE as c_int
        | ISOM_TRACK_IN_PREVIEW as c_int)
        as lsmash_track_mode;
    track_param.mode = track_mode;
    track_param.display_width = i_display_width;
    track_param.display_height = i_display_height;
    if lsmash_set_track_parameters((*p_mp4).p_root, (*p_mp4).i_track, &mut track_param) != 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"failed to set track parameters for video.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    let mut media_param: lsmash_media_parameters_t = lsmash_media_parameters_t {
        handler_type: 0 as lsmash_media_type,
        timescale: 0,
        duration: 0,
        roll_grouping: 0,
        rap_grouping: 0,
        MAC_language: 0,
        ISO_language: 0,
        media_handler_name: 0 as *mut c_char,
        data_handler_name: 0 as *mut c_char,
        media_handler_name_shadow: [0; 256],
        data_handler_name_shadow: [0; 256],
        compact_sample_size_table: 0,
        no_sample_dependency_table: 0,
        reserved: [0; 2],
    };
    lsmash_initialize_media_parameters(&mut media_param);
    media_param.timescale = i_media_timescale as uint32_t;
    media_param.media_handler_name =
        b"L-SMASH Video Media Handler\0" as *const u8 as *const c_char as *mut c_char;
    if (*p_mp4).b_use_recovery != 0 {
        media_param.roll_grouping = (*p_param).intra_refresh as uint8_t;
        media_param.rap_grouping = (*p_param).open_gop as uint8_t;
    }
    if lsmash_set_media_parameters((*p_mp4).p_root, (*p_mp4).i_track, &mut media_param) != 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"failed to set media parameters for video.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    (*p_mp4).i_video_timescale = lsmash_get_media_timescale((*p_mp4).p_root, (*p_mp4).i_track);
    if (*p_mp4).i_video_timescale == 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"media timescale for video is broken.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "294:1"]
unsafe extern "C" fn write_headers(mut handle: hnd_t, mut p_nal: *mut x264_nal_t) -> c_int {
    let mut p_mp4: *mut mp4_hnd_t = handle as *mut mp4_hnd_t;
    let mut sps_size: uint32_t = ((*p_nal.offset(0)).i_payload - H264_NALU_LENGTH_SIZE) as uint32_t;
    let mut pps_size: uint32_t = ((*p_nal.offset(1)).i_payload - H264_NALU_LENGTH_SIZE) as uint32_t;
    let mut sei_size: uint32_t = (*p_nal.offset(2)).i_payload as uint32_t;
    let mut sps: *mut uint8_t = (*p_nal.offset(0))
        .p_payload
        .offset(H264_NALU_LENGTH_SIZE as isize);
    let mut pps: *mut uint8_t = (*p_nal.offset(1))
        .p_payload
        .offset(H264_NALU_LENGTH_SIZE as isize);
    let mut sei: *mut uint8_t = (*p_nal.offset(2)).p_payload;
    let mut cs: *mut lsmash_codec_specific_t = lsmash_create_codec_specific_data(
        LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_H264,
        LSMASH_CODEC_SPECIFIC_FORMAT_STRUCTURED,
    );
    let mut param: *mut lsmash_h264_specific_parameters_t = (*cs).data.structured as _;
    (*param).lengthSizeMinusOne = (H264_NALU_LENGTH_SIZE - 1) as u8;
    if lsmash_append_h264_parameter_set(param, H264_PARAMETER_SET_TYPE_SPS, sps as _, sps_size) != 0
    {
        error!("failed to append SPS.");
        return -1;
    }
    if lsmash_append_h264_parameter_set(param, H264_PARAMETER_SET_TYPE_PPS, pps as _, pps_size) != 0
    {
        error!("failed to append PPS.");
        return -1;
    }
    if lsmash_add_codec_specific_data((*p_mp4).summary as *mut lsmash_summary_t, cs) != 0 {
        error!("failed to add H.264 specific info.");
        return -1;
    }
    lsmash_destroy_codec_specific_data(cs);
    cs = lsmash_create_codec_specific_data(
        LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_H264_BITRATE,
        LSMASH_CODEC_SPECIFIC_FORMAT_STRUCTURED,
    );
    if !cs.is_null() {
        lsmash_add_codec_specific_data((*p_mp4).summary as *mut lsmash_summary_t, cs);
    }
    lsmash_destroy_codec_specific_data(cs);
    (*p_mp4).i_sample_entry = lsmash_add_sample_entry(
        (*p_mp4).p_root,
        (*p_mp4).i_track,
        (*p_mp4).summary as *mut c_void,
    ) as uint32_t;
    if (*p_mp4).i_sample_entry == 0 {
        error!("failed to add sample entry for video.");
        return -1;
    }
    (*p_mp4).p_sei_buffer = malloc(sei_size as size_t) as *mut uint8_t;
    if (*p_mp4).p_sei_buffer.is_null() {
        error!("failed to allocate sei transition buffer.");
        return -1;
    }
    memcpy(
        (*p_mp4).p_sei_buffer as *mut c_void,
        sei as *const c_void,
        sei_size as size_t,
    );
    (*p_mp4).i_sei_size = sei_size;
    return sei_size.wrapping_add(sps_size).wrapping_add(pps_size) as c_int;
}
#[c2rust::src_loc = "357:1"]
unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: c_int,
    mut p_picture: *mut x264_picture_t,
) -> c_int {
    let mut p_mp4: *mut mp4_hnd_t = handle as *mut mp4_hnd_t;
    let mut dts: uint64_t = 0;
    let mut cts: uint64_t = 0;
    if (*p_mp4).i_numframe == 0 {
        (*p_mp4).i_start_offset = (*p_picture).i_dts * -1 as int64_t;
        (*p_mp4).i_first_cts = if (*p_mp4).b_dts_compress != 0 {
            0 as uint64_t
        } else {
            ((*p_mp4).i_start_offset as u64) * u64::from((*p_mp4).frame_duration)
        };
        if (*p_mp4).b_fragments != 0 {
            let mut edit: lsmash_edit_t = lsmash_edit_t {
                duration: 0,
                start_time: 0,
                rate: 0,
            };
            edit.duration = ISOM_EDIT_DURATION_UNKNOWN32 as uint64_t;
            edit.start_time = (*p_mp4).i_first_cts as int64_t;
            edit.rate = ISOM_EDIT_MODE_NORMAL as int32_t;
            if lsmash_create_explicit_timeline_map((*p_mp4).p_root, (*p_mp4).i_track, edit) != 0 {
                x264_cli_log(
                    b"mp4\0" as *const u8 as *const c_char,
                    X264_LOG_ERROR,
                    b"failed to set timeline map for video.\n\0" as *const u8 as *const c_char,
                );
            }
        }
    }
    let mut p_sample: *mut lsmash_sample_t =
        lsmash_create_sample((i_size as uint32_t).wrapping_add((*p_mp4).i_sei_size));
    if p_sample.is_null() {
        error!("failed to create a video sample data.");
        return -1;
    }
    if !(*p_mp4).p_sei_buffer.is_null() {
        memcpy(
            (*p_sample).data as *mut c_void,
            (*p_mp4).p_sei_buffer as *const c_void,
            (*p_mp4).i_sei_size as size_t,
        );
        free((*p_mp4).p_sei_buffer as *mut c_void);
        (*p_mp4).p_sei_buffer = 0 as *mut uint8_t;
    }
    memcpy(
        (*p_sample).data.offset((*p_mp4).i_sei_size as isize) as *mut c_void,
        p_nalu as *const c_void,
        i_size as size_t,
    );
    (*p_mp4).i_sei_size = 0 as uint32_t;
    if (*p_mp4).b_dts_compress != 0 {
        if (*p_mp4).i_numframe == 1 as c_int {
            (*p_mp4).i_init_delta = ((((*p_picture).i_dts + (*p_mp4).i_start_offset) as uint64_t)
                * u64::from((*p_mp4).frame_duration))
                as int64_t;
        }
        dts = if (*p_mp4).i_numframe > (*p_mp4).i_delay_frames {
            ((*p_picture).i_dts as u64) * u64::from((*p_mp4).frame_duration)
        } else {
            ((*p_mp4).i_numframe as int64_t
                * ((*p_mp4).i_init_delta / (*p_mp4).i_dts_compress_multiplier as int64_t))
                as uint64_t
        };
        cts = ((*p_picture).i_pts as uint64_t) * u64::from((*p_mp4).frame_duration);
    } else {
        dts = (((*p_picture).i_dts + (*p_mp4).i_start_offset) as uint64_t)
            * u64::from((*p_mp4).frame_duration);
        cts = (((*p_picture).i_pts + (*p_mp4).i_start_offset) as uint64_t)
            * u64::from((*p_mp4).frame_duration);
    }
    (*p_sample).dts = dts;
    (*p_sample).cts = cts;
    (*p_sample).index = (*p_mp4).i_sample_entry;
    (*p_sample).prop.ra_flags = (if (*p_picture).keyframe {
        ISOM_SAMPLE_RANDOM_ACCESS_FLAG_SYNC
    } else {
        ISOM_SAMPLE_RANDOM_ACCESS_FLAG_NONE
    }) as lsmash_random_access_flag;
    if (*p_mp4).b_fragments != 0
        && (*p_mp4).i_numframe != 0
        && (*p_sample).prop.ra_flags != ISOM_SAMPLE_RANDOM_ACCESS_FLAG_NONE
    {
        if lsmash_flush_pooled_samples(
            (*p_mp4).p_root,
            (*p_mp4).i_track,
            (*p_sample).dts.wrapping_sub((*p_mp4).i_prev_dts) as uint32_t,
        ) != 0
        {
            x264_cli_log(
                b"mp4\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"failed to flush the rest of samples.\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
        if lsmash_create_fragment_movie((*p_mp4).p_root) != 0 {
            x264_cli_log(
                b"mp4\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"failed to create a movie fragment.\n\0" as *const u8 as *const c_char,
            );
            return -1;
        }
    }
    if lsmash_append_sample((*p_mp4).p_root, (*p_mp4).i_track, p_sample) != 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"failed to append a video frame.\n\0" as *const u8 as *const c_char,
        );
        return -1;
    }
    (*p_mp4).i_prev_dts = dts;
    (*p_mp4).i_numframe += 1;
    return i_size;
}
#[no_mangle]
#[c2rust::src_loc = "429:20"]
static mut mp4_output: cli_output_t = cli_output_t {
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
