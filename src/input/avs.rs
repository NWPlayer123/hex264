use core::ffi::{c_char, c_double, c_float, c_int, c_void};

use crate::__stddef_null_h::NULL;
use crate::__stddef_size_t_h::size_t;
use crate::avisynth_c_h::{
    avs_as_error, avs_as_float, avs_as_int, avs_as_string, avs_clip_get_error_func,
    avs_create_script_environment_func, avs_delete_script_environment_func,
    avs_function_exists_func, avs_get_error_func, avs_get_frame_func, avs_get_pitch_p,
    avs_get_pitch_p_func, avs_get_read_ptr_p, avs_get_read_ptr_p_func, avs_get_video_info_func,
    avs_has_video, avs_invoke_func, avs_is_420_func, avs_is_422_func, avs_is_444_func, avs_is_clip,
    avs_is_error, avs_is_field_based, avs_is_float, avs_is_int, avs_is_rgb24, avs_is_rgb32,
    avs_is_rgb48_func, avs_is_rgb64_func, avs_is_string, avs_is_tff, avs_is_y16_func, avs_is_y8,
    avs_is_y8_func, avs_is_y_func, avs_is_yuv420p16_func, avs_is_yuv422p16_func,
    avs_is_yuv444p16_func, avs_is_yuy2, avs_is_yv12, avs_is_yv12_func, avs_is_yv16,
    avs_is_yv16_func, avs_is_yv24, avs_is_yv24_func, avs_is_yv411, avs_is_yv411_func,
    avs_new_value_array, avs_new_value_bool, avs_new_value_string, avs_release_clip_func,
    avs_release_value_func, avs_release_video_frame_func, avs_take_clip_func, AVS_Clip,
    AVS_ScriptEnvironment, AVS_Value, AVS_VideoFrame, AVS_VideoInfo, C2RustUnnamed_0, AVS_PLANAR_U,
    AVS_PLANAR_V, AVS_PLANAR_Y,
};
use crate::bits_dlfcn_h::RTLD_NOW;
use crate::dlfcn_h::{dlclose, dlopen, dlsym};
use crate::input_h::{
    cli_input_opt_t, cli_input_t, cli_pic_t, video_info_t, x264_cli_csp_t, x264_cli_get_csp,
    x264_cli_pic_alloc, X264_CSP_OTHER,
};
use crate::osdep_h::x264_is_regular_file;
use crate::pixfmt_h::AV_PIX_FMT_YUV411P;
use crate::stdint_uintn_h::uint8_t;
use crate::stdio_h::{fclose, fflush, fopen, stderr};
use crate::stdlib_h::{calloc, free};
use crate::string_h::memset;
use crate::strings_h::{strcasecmp, strncasecmp};
use crate::x264_h::{
    X264_CSP_BGR, X264_CSP_BGRA, X264_CSP_HIGH_DEPTH, X264_CSP_I400, X264_CSP_I420, X264_CSP_I422,
    X264_CSP_I444, X264_CSP_NONE, X264_CSP_VFLIP, X264_CSP_YUYV, X264_LOG_DEBUG, X264_LOG_ERROR,
    X264_LOG_INFO, X264_LOG_WARNING,
};
use crate::x264cli_h::{get_filename_extension, hnd_t, x264_cli_log, x264_cli_printf};
use crate::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "78:9"]
struct avs_hnd_t {
    clip: *mut AVS_Clip,
    env: *mut AVS_ScriptEnvironment,
    library: *mut c_void,
    num_frames: c_int,
    func: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "84:5"]
struct C2RustUnnamed {
    avs_clip_get_error: avs_clip_get_error_func,
    avs_create_script_environment: avs_create_script_environment_func,
    avs_delete_script_environment: avs_delete_script_environment_func,
    avs_get_error: avs_get_error_func,
    avs_get_frame: avs_get_frame_func,
    avs_get_video_info: avs_get_video_info_func,
    avs_function_exists: avs_function_exists_func,
    avs_invoke: avs_invoke_func,
    avs_release_clip: avs_release_clip_func,
    avs_release_value: avs_release_value_func,
    avs_release_video_frame: avs_release_video_frame_func,
    avs_take_clip: avs_take_clip_func,
    avs_is_yv24: avs_is_yv24_func,
    avs_is_yv16: avs_is_yv16_func,
    avs_is_yv12: avs_is_yv12_func,
    avs_is_yv411: avs_is_yv411_func,
    avs_is_y8: avs_is_y8_func,
    avs_get_pitch_p: avs_get_pitch_p_func,
    avs_get_read_ptr_p: avs_get_read_ptr_p_func,
    avs_is_rgb48: avs_is_rgb48_func,
    avs_is_rgb64: avs_is_rgb64_func,
    avs_is_yuv444p16: avs_is_yuv444p16_func,
    avs_is_yuv422p16: avs_is_yuv422p16_func,
    avs_is_yuv420p16: avs_is_yuv420p16_func,
    avs_is_y16: avs_is_y16_func,
    avs_is_444: avs_is_444_func,
    avs_is_422: avs_is_422_func,
    avs_is_420: avs_is_420_func,
    avs_is_y: avs_is_y_func,
}
#[c2rust::src_loc = "54:9"]
const AVS_INTERFACE_25: c_int = 2 as c_int;
#[c2rust::src_loc = "61:9"]
const AVS_MAX_SEQUENCE: c_int = 5 as c_int;
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn custom_avs_load_library(mut h: *mut avs_hnd_t) -> c_int {
    (*h).library = dlopen(b"libavisynth.so\0" as *const u8 as *const c_char, RTLD_NOW);
    if (*h).library.is_null() {
        return -(1 as c_int);
    }
    (*h).func.avs_clip_get_error =
        ::core::mem::transmute::<*mut c_void, avs_clip_get_error_func>(dlsym(
            (*h).library,
            b"avs_clip_get_error\0" as *const u8 as *const c_char,
        ));
    if !(0 as c_int == 0 && (*h).func.avs_clip_get_error.is_none()) {
        (*h).func.avs_create_script_environment =
            ::core::mem::transmute::<*mut c_void, avs_create_script_environment_func>(dlsym(
                (*h).library,
                b"avs_create_script_environment\0" as *const u8 as *const c_char,
            ));
        if !(0 as c_int == 0 && (*h).func.avs_create_script_environment.is_none()) {
            (*h).func.avs_delete_script_environment =
                ::core::mem::transmute::<*mut c_void, avs_delete_script_environment_func>(dlsym(
                    (*h).library,
                    b"avs_delete_script_environment\0" as *const u8 as *const c_char,
                ));
            if !(1 as c_int == 0 && (*h).func.avs_delete_script_environment.is_none()) {
                (*h).func.avs_get_error =
                    ::core::mem::transmute::<*mut c_void, avs_get_error_func>(dlsym(
                        (*h).library,
                        b"avs_get_error\0" as *const u8 as *const c_char,
                    ));
                if !(1 as c_int == 0 && (*h).func.avs_get_error.is_none()) {
                    (*h).func.avs_get_frame =
                        ::core::mem::transmute::<*mut c_void, avs_get_frame_func>(dlsym(
                            (*h).library,
                            b"avs_get_frame\0" as *const u8 as *const c_char,
                        ));
                    if !(0 as c_int == 0 && (*h).func.avs_get_frame.is_none()) {
                        (*h).func.avs_get_video_info =
                            ::core::mem::transmute::<*mut c_void, avs_get_video_info_func>(dlsym(
                                (*h).library,
                                b"avs_get_video_info\0" as *const u8 as *const c_char,
                            ));
                        if !(0 as c_int == 0 && (*h).func.avs_get_video_info.is_none()) {
                            (*h).func.avs_function_exists = ::core::mem::transmute::<
                                *mut c_void,
                                avs_function_exists_func,
                            >(dlsym(
                                (*h).library,
                                b"avs_function_exists\0" as *const u8 as *const c_char,
                            ));
                            if !(0 as c_int == 0 && (*h).func.avs_function_exists.is_none()) {
                                (*h).func.avs_invoke =
                                    ::core::mem::transmute::<*mut c_void, avs_invoke_func>(dlsym(
                                        (*h).library,
                                        b"avs_invoke\0" as *const u8 as *const c_char,
                                    ));
                                if !(0 as c_int == 0 && (*h).func.avs_invoke.is_none()) {
                                    (*h).func.avs_release_clip = ::core::mem::transmute::<
                                        *mut c_void,
                                        avs_release_clip_func,
                                    >(
                                        dlsym(
                                            (*h).library,
                                            b"avs_release_clip\0" as *const u8 as *const c_char,
                                        ),
                                    );
                                    if !(0 as c_int == 0 && (*h).func.avs_release_clip.is_none()) {
                                        (*h).func.avs_release_value =
                                            ::core::mem::transmute::<
                                                *mut c_void,
                                                avs_release_value_func,
                                            >(dlsym(
                                                (*h).library,
                                                b"avs_release_value\0" as *const u8
                                                    as *const c_char,
                                            ));
                                        if !(0 as c_int == 0
                                            && (*h).func.avs_release_value.is_none())
                                        {
                                            (*h).func.avs_release_video_frame =
                                                ::core::mem::transmute::<
                                                    *mut c_void,
                                                    avs_release_video_frame_func,
                                                >(
                                                    dlsym(
                                                        (*h).library,
                                                        b"avs_release_video_frame\0" as *const u8
                                                            as *const c_char,
                                                    ),
                                                );
                                            if !(0 as c_int == 0
                                                && (*h).func.avs_release_video_frame.is_none())
                                            {
                                                (*h).func.avs_take_clip = ::core::mem::transmute::<
                                                    *mut c_void,
                                                    avs_take_clip_func,
                                                >(
                                                    dlsym(
                                                        (*h).library,
                                                        b"avs_take_clip\0" as *const u8
                                                            as *const c_char,
                                                    ),
                                                );
                                                if !(0 as c_int == 0
                                                    && (*h).func.avs_take_clip.is_none())
                                                {
                                                    (*h).func.avs_is_yv24 = ::core::mem::transmute::<
                                                        *mut c_void,
                                                        avs_is_yv24_func,
                                                    >(
                                                        dlsym(
                                                            (*h).library,
                                                            b"avs_is_yv24\0" as *const u8
                                                                as *const c_char,
                                                        ),
                                                    );
                                                    if !(1 as c_int == 0
                                                        && (*h).func.avs_is_yv24.is_none())
                                                    {
                                                        (*h).func.avs_is_yv16 =
                                                            ::core::mem::transmute::<
                                                                *mut c_void,
                                                                avs_is_yv16_func,
                                                            >(
                                                                dlsym(
                                                                    (*h).library,
                                                                    b"avs_is_yv16\0" as *const u8
                                                                        as *const c_char,
                                                                ),
                                                            );
                                                        if !(1 as c_int == 0
                                                            && (*h).func.avs_is_yv16.is_none())
                                                        {
                                                            (*h).func.avs_is_yv12 =
                                                                ::core::mem::transmute::<
                                                                    *mut c_void,
                                                                    avs_is_yv12_func,
                                                                >(
                                                                    dlsym(
                                                                        (*h).library,
                                                                        b"avs_is_yv12\0"
                                                                            as *const u8
                                                                            as *const c_char,
                                                                    ),
                                                                );
                                                            if !(1 as c_int == 0
                                                                && (*h).func.avs_is_yv12.is_none())
                                                            {
                                                                (*h).func.avs_is_yv411 =
                                                                    ::core::mem::transmute::<
                                                                        *mut c_void,
                                                                        avs_is_yv411_func,
                                                                    >(
                                                                        dlsym(
                                                                            (*h).library,
                                                                            b"avs_is_yv411\0"
                                                                                as *const u8
                                                                                as *const c_char,
                                                                        ),
                                                                    );
                                                                if !(1 as c_int == 0
                                                                    && (*h)
                                                                        .func
                                                                        .avs_is_yv411
                                                                        .is_none())
                                                                {
                                                                    (*h).func.avs_is_y8 = ::core::mem::transmute::<
                                                                        *mut c_void,
                                                                        avs_is_y8_func,
                                                                    >(
                                                                        dlsym(
                                                                            (*h).library,
                                                                            b"avs_is_y8\0" as *const u8 as *const c_char,
                                                                        ),
                                                                    );
                                                                    if !(1 as c_int == 0
                                                                        && (*h)
                                                                            .func
                                                                            .avs_is_y8
                                                                            .is_none())
                                                                    {
                                                                        (*h).func.avs_get_pitch_p = ::core::mem::transmute::<
                                                                            *mut c_void,
                                                                            avs_get_pitch_p_func,
                                                                        >(
                                                                            dlsym(
                                                                                (*h).library,
                                                                                b"avs_get_pitch_p\0" as *const u8
                                                                                    as *const c_char,
                                                                            ),
                                                                        );
                                                                        if !(1 as c_int == 0
                                                                            && (*h)
                                                                                .func
                                                                                .avs_get_pitch_p
                                                                                .is_none())
                                                                        {
                                                                            (*h).func.avs_get_read_ptr_p = ::core::mem::transmute::<
                                                                                *mut c_void,
                                                                                avs_get_read_ptr_p_func,
                                                                            >(
                                                                                dlsym(
                                                                                    (*h).library,
                                                                                    b"avs_get_read_ptr_p\0" as *const u8
                                                                                        as *const c_char,
                                                                                ),
                                                                            );
                                                                            if !(1 as c_int == 0
                                                                                && (*h).func.avs_get_read_ptr_p.is_none())
                                                                            {
                                                                                (*h).func.avs_is_rgb48 = ::core::mem::transmute::<
                                                                                    *mut c_void,
                                                                                    avs_is_rgb48_func,
                                                                                >(
                                                                                    dlsym(
                                                                                        (*h).library,
                                                                                        b"avs_is_rgb48\0" as *const u8 as *const c_char,
                                                                                    ),
                                                                                );
                                                                                if !(1 as c_int == 0
                                                                                    && (*h).func.avs_is_rgb48.is_none())
                                                                                {
                                                                                    if (*h).func.avs_is_rgb48.is_none() {
                                                                                        (*h).func.avs_is_rgb48 = ::core::mem::transmute::<
                                                                                            *mut c_void,
                                                                                            avs_is_rgb48_func,
                                                                                        >(
                                                                                            dlsym(
                                                                                                (*h).library,
                                                                                                b"_avs_is_rgb48@4\0" as *const u8
                                                                                                    as *const c_char,
                                                                                            ),
                                                                                        );
                                                                                    }
                                                                                    if !(1 as c_int == 0
                                                                                        && (*h).func.avs_is_rgb48.is_none())
                                                                                    {
                                                                                        (*h).func.avs_is_rgb64 = ::core::mem::transmute::<
                                                                                            *mut c_void,
                                                                                            avs_is_rgb64_func,
                                                                                        >(
                                                                                            dlsym(
                                                                                                (*h).library,
                                                                                                b"avs_is_rgb64\0" as *const u8 as *const c_char,
                                                                                            ),
                                                                                        );
                                                                                        if !(1 as c_int == 0
                                                                                            && (*h).func.avs_is_rgb64.is_none())
                                                                                        {
                                                                                            if (*h).func.avs_is_rgb64.is_none() {
                                                                                                (*h).func.avs_is_rgb64 = ::core::mem::transmute::<
                                                                                                    *mut c_void,
                                                                                                    avs_is_rgb64_func,
                                                                                                >(
                                                                                                    dlsym(
                                                                                                        (*h).library,
                                                                                                        b"_avs_is_rgb64@4\0" as *const u8
                                                                                                            as *const c_char,
                                                                                                    ),
                                                                                                );
                                                                                            }
                                                                                            if !(1 as c_int == 0
                                                                                                && (*h).func.avs_is_rgb64.is_none())
                                                                                            {
                                                                                                (*h).func.avs_is_yuv444p16 = ::core::mem::transmute::<
                                                                                                    *mut c_void,
                                                                                                    avs_is_yuv444p16_func,
                                                                                                >(
                                                                                                    dlsym(
                                                                                                        (*h).library,
                                                                                                        b"avs_is_yuv444p16\0" as *const u8
                                                                                                            as *const c_char,
                                                                                                    ),
                                                                                                );
                                                                                                if !(1 as c_int == 0
                                                                                                    && (*h).func.avs_is_yuv444p16.is_none())
                                                                                                {
                                                                                                    (*h).func.avs_is_yuv422p16 = ::core::mem::transmute::<
                                                                                                        *mut c_void,
                                                                                                        avs_is_yuv422p16_func,
                                                                                                    >(
                                                                                                        dlsym(
                                                                                                            (*h).library,
                                                                                                            b"avs_is_yuv422p16\0" as *const u8
                                                                                                                as *const c_char,
                                                                                                        ),
                                                                                                    );
                                                                                                    if !(1 as c_int == 0
                                                                                                        && (*h).func.avs_is_yuv422p16.is_none())
                                                                                                    {
                                                                                                        (*h).func.avs_is_yuv420p16 = ::core::mem::transmute::<
                                                                                                            *mut c_void,
                                                                                                            avs_is_yuv420p16_func,
                                                                                                        >(
                                                                                                            dlsym(
                                                                                                                (*h).library,
                                                                                                                b"avs_is_yuv420p16\0" as *const u8
                                                                                                                    as *const c_char,
                                                                                                            ),
                                                                                                        );
                                                                                                        if !(1 as c_int == 0
                                                                                                            && (*h).func.avs_is_yuv420p16.is_none())
                                                                                                        {
                                                                                                            (*h).func.avs_is_y16 = ::core::mem::transmute::<
                                                                                                                *mut c_void,
                                                                                                                avs_is_y16_func,
                                                                                                            >(
                                                                                                                dlsym(
                                                                                                                    (*h).library,
                                                                                                                    b"avs_is_y16\0" as *const u8 as *const c_char,
                                                                                                                ),
                                                                                                            );
                                                                                                            if !(1 as c_int == 0
                                                                                                                && (*h).func.avs_is_y16.is_none())
                                                                                                            {
                                                                                                                (*h).func.avs_is_444 = ::core::mem::transmute::<
                                                                                                                    *mut c_void,
                                                                                                                    avs_is_444_func,
                                                                                                                >(
                                                                                                                    dlsym(
                                                                                                                        (*h).library,
                                                                                                                        b"avs_is_444\0" as *const u8 as *const c_char,
                                                                                                                    ),
                                                                                                                );
                                                                                                                if !(1 as c_int == 0
                                                                                                                    && (*h).func.avs_is_444.is_none())
                                                                                                                {
                                                                                                                    (*h).func.avs_is_422 = ::core::mem::transmute::<
                                                                                                                        *mut c_void,
                                                                                                                        avs_is_422_func,
                                                                                                                    >(
                                                                                                                        dlsym(
                                                                                                                            (*h).library,
                                                                                                                            b"avs_is_422\0" as *const u8 as *const c_char,
                                                                                                                        ),
                                                                                                                    );
                                                                                                                    if !(1 as c_int == 0
                                                                                                                        && (*h).func.avs_is_422.is_none())
                                                                                                                    {
                                                                                                                        (*h).func.avs_is_420 = ::core::mem::transmute::<
                                                                                                                            *mut c_void,
                                                                                                                            avs_is_420_func,
                                                                                                                        >(
                                                                                                                            dlsym(
                                                                                                                                (*h).library,
                                                                                                                                b"avs_is_420\0" as *const u8 as *const c_char,
                                                                                                                            ),
                                                                                                                        );
                                                                                                                        if !(1 as c_int == 0
                                                                                                                            && (*h).func.avs_is_420.is_none())
                                                                                                                        {
                                                                                                                            (*h).func.avs_is_y = ::core::mem::transmute::<
                                                                                                                                *mut c_void,
                                                                                                                                avs_is_y_func,
                                                                                                                            >(
                                                                                                                                dlsym(
                                                                                                                                    (*h).library,
                                                                                                                                    b"avs_is_y\0" as *const u8 as *const c_char,
                                                                                                                                ),
                                                                                                                            );
                                                                                                                            if !(1 as c_int == 0
                                                                                                                                && (*h).func.avs_is_y.is_none())
                                                                                                                            {
                                                                                                                                return 0 as c_int;
                                                                                                                            }
                                                                                                                        }
                                                                                                                    }
                                                                                                                }
                                                                                                            }
                                                                                                        }
                                                                                                    }
                                                                                                }
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    dlclose((*h).library);
    (*h).library = NULL;
    return -(1 as c_int);
}
#[c2rust::src_loc = "185:1"]
unsafe extern "C" fn avs_build_filter_sequence(
    mut _filename_ext: *mut c_char,
    mut filter: *mut *const c_char,
) {
    let mut i: c_int = 0 as c_int;
    let mut all_purpose: [*const c_char; 2] = [
        b"FFVideoSource\0" as *const u8 as *const c_char,
        0 as *const c_char,
    ];
    let mut j: c_int = 0 as c_int;
    while !all_purpose[j as usize].is_null() && i < AVS_MAX_SEQUENCE {
        let fresh0 = i;
        i = i + 1;
        let ref mut fresh1 = *filter.offset(fresh0 as isize);
        *fresh1 = all_purpose[j as usize];
        j += 1;
    }
}
#[c2rust::src_loc = "203:1"]
unsafe extern "C" fn update_clip(
    mut h: *mut avs_hnd_t,
    mut vi: *mut *const AVS_VideoInfo,
    mut res: AVS_Value,
    mut release: AVS_Value,
) -> AVS_Value {
    (*h).func
        .avs_release_clip
        .expect("non-null function pointer")((*h).clip);
    (*h).clip = (*h).func.avs_take_clip.expect("non-null function pointer")(res, (*h).env);
    (*h).func
        .avs_release_value
        .expect("non-null function pointer")(release);
    *vi = (*h)
        .func
        .avs_get_video_info
        .expect("non-null function pointer")((*h).clip);
    return res;
}
#[c2rust::src_loc = "212:1"]
unsafe extern "C" fn get_avs_version(mut h: *mut avs_hnd_t) -> c_float {
    if (*h)
        .func
        .avs_function_exists
        .expect("non-null function pointer")(
        (*h).env,
        b"VersionNumber\0" as *const u8 as *const c_char,
    ) == 0
    {
        x264_cli_log(
            b"avs\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"VersionNumber does not exist\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int) as c_float;
    }
    let mut ver: AVS_Value = (*h).func.avs_invoke.expect("non-null function pointer")(
        (*h).env,
        b"VersionNumber\0" as *const u8 as *const c_char,
        avs_new_value_array(0 as *mut AVS_Value, 0 as c_int),
        0 as *mut *const c_char,
    );
    if avs_is_error(ver) != 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"unable to determine avisynth version: %s\n\0" as *const u8 as *const c_char,
            avs_as_error(ver),
        );
        return -(1 as c_int) as c_float;
    }
    if avs_is_float(ver) == 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"VersionNumber did not return a float value\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int) as c_float;
    }
    let mut ret: c_float = avs_as_float(ver) as c_float;
    (*h).func
        .avs_release_value
        .expect("non-null function pointer")(ver);
    return ret;
}
#[c2rust::src_loc = "269:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut _opt: *mut cli_input_opt_t,
) -> c_int {
    let mut fh: *mut FILE = fopen(psz_filename, b"r\0" as *const u8 as *const c_char) as *mut FILE;
    if fh.is_null() {
        return -(1 as c_int);
    }
    let mut b_regular: c_int = x264_is_regular_file(fh);
    fclose(fh);
    if b_regular == 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"AVS input is incompatible with non-regular file `%s'\n\0" as *const u8
                as *const c_char,
            psz_filename,
        );
        return -(1 as c_int);
    }
    let mut h: *mut avs_hnd_t =
        calloc(1 as size_t, ::core::mem::size_of::<avs_hnd_t>() as size_t) as *mut avs_hnd_t;
    if h.is_null() {
        return -(1 as c_int);
    }
    if custom_avs_load_library(h) != 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"failed to load avisynth\n\0" as *const u8 as *const c_char,
        );
        return -(1 as c_int);
    }
    (*h).env = (*h)
        .func
        .avs_create_script_environment
        .expect("non-null function pointer")(AVS_INTERFACE_25);
    if (*h).func.avs_get_error.is_some() {
        let mut error: *const c_char =
            (*h).func.avs_get_error.expect("non-null function pointer")((*h).env);
        if !error.is_null() {
            x264_cli_log(
                b"avs\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"%s\n\0" as *const u8 as *const c_char,
                error,
            );
            return -(1 as c_int);
        }
    }
    let mut avs_version: c_float = get_avs_version(h);
    if avs_version <= 0 as c_int as c_float {
        return -(1 as c_int);
    }
    x264_cli_log(
        b"avs\0" as *const u8 as *const c_char,
        X264_LOG_DEBUG,
        b"using avisynth version %.2f\n\0" as *const u8 as *const c_char,
        avs_version as c_double,
    );
    let mut arg: AVS_Value = avs_new_value_string(psz_filename);
    let mut res: AVS_Value = AVS_Value {
        type_0: 0,
        array_size: 0,
        d: C2RustUnnamed_0 {
            clip: 0 as *mut c_void,
        },
    };
    let mut filename_ext: *mut c_char = get_filename_extension(psz_filename);
    if strcasecmp(filename_ext, b"avs\0" as *const u8 as *const c_char) == 0 {
        res = (*h).func.avs_invoke.expect("non-null function pointer")(
            (*h).env,
            b"Import\0" as *const u8 as *const c_char,
            arg,
            0 as *mut *const c_char,
        );
        if avs_is_error(res) != 0 {
            x264_cli_log(
                b"avs\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"%s\n\0" as *const u8 as *const c_char,
                avs_as_error(res),
            );
            return -(1 as c_int);
        }
        let mut mt_test: AVS_Value = (*h).func.avs_invoke.expect("non-null function pointer")(
            (*h).env,
            b"GetMTMode\0" as *const u8 as *const c_char,
            avs_new_value_bool(0 as c_int),
            0 as *mut *const c_char,
        );
        let mut mt_mode: c_int = if avs_is_int(mt_test) != 0 {
            avs_as_int(mt_test)
        } else {
            0 as c_int
        };
        (*h).func
            .avs_release_value
            .expect("non-null function pointer")(mt_test);
        if mt_mode > 0 as c_int && mt_mode < 5 as c_int {
            let mut temp: AVS_Value = (*h).func.avs_invoke.expect("non-null function pointer")(
                (*h).env,
                b"Distributor\0" as *const u8 as *const c_char,
                res,
                0 as *mut *const c_char,
            );
            (*h).func
                .avs_release_value
                .expect("non-null function pointer")(res);
            res = temp;
        }
    } else {
        let mut filter: [*const c_char; 6] = [
            0 as *const c_char,
            0 as *const c_char,
            0 as *const c_char,
            0 as *const c_char,
            0 as *const c_char,
            0 as *const c_char,
        ];
        avs_build_filter_sequence(filename_ext, filter.as_mut_ptr());
        let mut i: c_int = 0;
        i = 0 as c_int;
        while !filter[i as usize].is_null() {
            x264_cli_log(
                b"avs\0" as *const u8 as *const c_char,
                X264_LOG_INFO,
                b"trying %s... \0" as *const u8 as *const c_char,
                filter[i as usize],
            );
            if (*h)
                .func
                .avs_function_exists
                .expect("non-null function pointer")((*h).env, filter[i as usize])
                == 0
            {
                x264_cli_printf(
                    X264_LOG_INFO,
                    b"not found\n\0" as *const u8 as *const c_char,
                );
            } else {
                if strncasecmp(
                    filter[i as usize],
                    b"FFmpegSource\0" as *const u8 as *const c_char,
                    12 as size_t,
                ) == 0
                {
                    x264_cli_printf(
                        X264_LOG_INFO,
                        b"indexing... \0" as *const u8 as *const c_char,
                    );
                    fflush(stderr);
                }
                res = (*h).func.avs_invoke.expect("non-null function pointer")(
                    (*h).env,
                    filter[i as usize],
                    arg,
                    0 as *mut *const c_char,
                );
                if avs_is_error(res) == 0 {
                    x264_cli_printf(
                        X264_LOG_INFO,
                        b"succeeded\n\0" as *const u8 as *const c_char,
                    );
                    break;
                } else {
                    x264_cli_printf(X264_LOG_INFO, b"failed\n\0" as *const u8 as *const c_char);
                }
            }
            i += 1;
        }
        if filter[i as usize].is_null() {
            x264_cli_log(
                b"avs\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"unable to find source filter to open `%s'\n\0" as *const u8 as *const c_char,
                psz_filename,
            );
            return -(1 as c_int);
        }
    }
    if avs_is_clip(res) == 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"`%s' didn't return a video clip\n\0" as *const u8 as *const c_char,
            psz_filename,
        );
        return -(1 as c_int);
    }
    (*h).clip = (*h).func.avs_take_clip.expect("non-null function pointer")(res, (*h).env);
    let mut vi: *const AVS_VideoInfo = (*h)
        .func
        .avs_get_video_info
        .expect("non-null function pointer")((*h).clip);
    if avs_has_video(vi) == 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"`%s' has no video data\n\0" as *const u8 as *const c_char,
            psz_filename,
        );
        return -(1 as c_int);
    }
    if avs_is_field_based(vi) != 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const c_char,
            X264_LOG_WARNING,
            b"detected fieldbased (separated) input, weaving to frames\n\0" as *const u8
                as *const c_char,
        );
        let mut tmp: AVS_Value = (*h).func.avs_invoke.expect("non-null function pointer")(
            (*h).env,
            b"Weave\0" as *const u8 as *const c_char,
            res,
            0 as *mut *const c_char,
        );
        if avs_is_error(tmp) != 0 {
            x264_cli_log(
                b"avs\0" as *const u8 as *const c_char,
                X264_LOG_ERROR,
                b"couldn't weave fields into frames: %s\n\0" as *const u8 as *const c_char,
                avs_as_error(tmp),
            );
            return -(1 as c_int);
        }
        res = update_clip(h, &mut vi, tmp, res);
        (*info).interlaced = 1 as c_int;
        (*info).tff = avs_is_tff(vi);
    }
    (*h).func
        .avs_release_value
        .expect("non-null function pointer")(res);
    (*info).width = (*vi).width as u32;
    (*info).height = (*vi).height as u32;
    (*info).fps_num = (*vi).fps_numerator;
    (*info).fps_den = (*vi).fps_denominator;
    (*info).num_frames = (*vi).num_frames;
    (*h).num_frames = (*info).num_frames;
    (*info).thread_safe = 1 as c_int;
    if (*h).func.avs_is_rgb64.is_some()
        && (*h).func.avs_is_rgb64.expect("non-null function pointer")(vi) != 0
    {
        (*info).csp = X264_CSP_BGRA | X264_CSP_VFLIP | X264_CSP_HIGH_DEPTH;
    } else if avs_is_rgb32(vi) != 0 {
        (*info).csp = X264_CSP_BGRA | X264_CSP_VFLIP;
    } else if (*h).func.avs_is_rgb48.is_some()
        && (*h).func.avs_is_rgb48.expect("non-null function pointer")(vi) != 0
    {
        (*info).csp = X264_CSP_BGR | X264_CSP_VFLIP | X264_CSP_HIGH_DEPTH;
    } else if avs_is_rgb24(vi) != 0 {
        (*info).csp = X264_CSP_BGR | X264_CSP_VFLIP;
    } else if (*h).func.avs_is_yuv444p16.is_some()
        && (*h)
            .func
            .avs_is_yuv444p16
            .expect("non-null function pointer")(vi)
            != 0
    {
        (*info).csp = X264_CSP_I444 | X264_CSP_HIGH_DEPTH;
    } else if if (*h).func.avs_is_yv24.is_some() {
        (*h).func.avs_is_yv24.expect("non-null function pointer")(vi)
    } else {
        avs_is_yv24(vi)
    } != 0
    {
        (*info).csp = X264_CSP_I444;
    } else if (*h).func.avs_is_yuv422p16.is_some()
        && (*h)
            .func
            .avs_is_yuv422p16
            .expect("non-null function pointer")(vi)
            != 0
    {
        (*info).csp = X264_CSP_I422 | X264_CSP_HIGH_DEPTH;
    } else if if (*h).func.avs_is_yv16.is_some() {
        (*h).func.avs_is_yv16.expect("non-null function pointer")(vi)
    } else {
        avs_is_yv16(vi)
    } != 0
    {
        (*info).csp = X264_CSP_I422;
    } else if (*h).func.avs_is_yuv420p16.is_some()
        && (*h)
            .func
            .avs_is_yuv420p16
            .expect("non-null function pointer")(vi)
            != 0
    {
        (*info).csp = X264_CSP_I420 | X264_CSP_HIGH_DEPTH;
    } else if if (*h).func.avs_is_yv12.is_some() {
        (*h).func.avs_is_yv12.expect("non-null function pointer")(vi)
    } else {
        avs_is_yv12(vi)
    } != 0
    {
        (*info).csp = X264_CSP_I420;
    } else if (*h).func.avs_is_y16.is_some()
        && (*h).func.avs_is_y16.expect("non-null function pointer")(vi) != 0
    {
        (*info).csp = X264_CSP_I400 | X264_CSP_HIGH_DEPTH;
    } else if if (*h).func.avs_is_y8.is_some() {
        (*h).func.avs_is_y8.expect("non-null function pointer")(vi)
    } else {
        avs_is_y8(vi)
    } != 0
    {
        (*info).csp = X264_CSP_I400;
    } else if avs_is_yuy2(vi) != 0 {
        (*info).csp = X264_CSP_YUYV;
    } else if if (*h).func.avs_is_yv411.is_some() {
        (*h).func.avs_is_yv411.expect("non-null function pointer")(vi)
    } else {
        avs_is_yv411(vi)
    } != 0
    {
        (*info).csp = AV_PIX_FMT_YUV411P as c_int | X264_CSP_OTHER;
    } else {
        let mut pixel_type: AVS_Value = (*h).func.avs_invoke.expect("non-null function pointer")(
            (*h).env,
            b"PixelType\0" as *const u8 as *const c_char,
            res,
            0 as *mut *const c_char,
        );
        let mut pixel_type_name: *const c_char = if avs_is_string(pixel_type) != 0 {
            avs_as_string(pixel_type)
        } else {
            b"unknown\0" as *const u8 as *const c_char
        };
        x264_cli_log(
            b"avs\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"not supported pixel type: %s\n\0" as *const u8 as *const c_char,
            pixel_type_name,
        );
        return -(1 as c_int);
    }
    (*info).vfr = 0 as c_int;
    *p_handle = h as hnd_t;
    return 0 as c_int;
}
#[c2rust::src_loc = "504:1"]
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut _handle: hnd_t,
    mut csp: c_int,
    mut width: c_int,
    mut height: c_int,
) -> c_int {
    if x264_cli_pic_alloc(pic, X264_CSP_NONE, width, height) != 0 {
        return -(1 as c_int);
    }
    (*pic).img.csp = csp;
    let mut cli_csp: *const x264_cli_csp_t = x264_cli_get_csp(csp);
    if !cli_csp.is_null() {
        (*pic).img.planes = (*cli_csp).planes;
    } else if csp == AV_PIX_FMT_YUV411P as c_int | X264_CSP_OTHER {
        (*pic).img.planes = 3 as c_int;
    } else {
        (*pic).img.planes = 1 as c_int;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "521:1"]
unsafe extern "C" fn read_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut i_frame: c_int,
) -> c_int {
    static mut plane: [c_int; 3] = [
        AVS_PLANAR_Y as c_int,
        AVS_PLANAR_U as c_int,
        AVS_PLANAR_V as c_int,
    ];
    let mut h: *mut avs_hnd_t = handle as *mut avs_hnd_t;
    if i_frame >= (*h).num_frames {
        return -(1 as c_int);
    }
    (*pic).opaque = (*h).func.avs_get_frame.expect("non-null function pointer")((*h).clip, i_frame)
        as *mut c_void;
    let mut frm: *mut AVS_VideoFrame = (*pic).opaque as *mut AVS_VideoFrame;
    let mut err: *const c_char = (*h)
        .func
        .avs_clip_get_error
        .expect("non-null function pointer")((*h).clip);
    if !err.is_null() {
        x264_cli_log(
            b"avs\0" as *const u8 as *const c_char,
            X264_LOG_ERROR,
            b"%s occurred while reading frame %d\n\0" as *const u8 as *const c_char,
            err,
            i_frame,
        );
        return -(1 as c_int);
    }
    let mut i: c_int = 0 as c_int;
    while i < (*pic).img.planes {
        (*pic).img.plane[i as usize] = (if (*h).func.avs_get_read_ptr_p.is_some() {
            (*h).func
                .avs_get_read_ptr_p
                .expect("non-null function pointer")(frm, plane[i as usize])
        } else {
            avs_get_read_ptr_p(frm, plane[i as usize])
        }) as *mut uint8_t;
        (*pic).img.stride[i as usize] = if (*h).func.avs_get_pitch_p.is_some() {
            (*h).func
                .avs_get_pitch_p
                .expect("non-null function pointer")(frm, plane[i as usize])
        } else {
            avs_get_pitch_p(frm, plane[i as usize])
        };
        i += 1;
    }
    return 0 as c_int;
}
#[c2rust::src_loc = "539:1"]
unsafe extern "C" fn release_frame(mut pic: *mut cli_pic_t, mut handle: hnd_t) -> c_int {
    let mut h: *mut avs_hnd_t = handle as *mut avs_hnd_t;
    (*h).func
        .avs_release_video_frame
        .expect("non-null function pointer")((*pic).opaque as *mut AVS_VideoFrame);
    return 0 as c_int;
}
#[c2rust::src_loc = "546:1"]
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut _handle: hnd_t) {
    memset(
        pic as *mut c_void,
        0 as c_int,
        ::core::mem::size_of::<cli_pic_t>() as size_t,
    );
}
#[c2rust::src_loc = "551:1"]
unsafe extern "C" fn close_file(mut handle: hnd_t) -> c_int {
    let mut h: *mut avs_hnd_t = handle as *mut avs_hnd_t;
    if (*h).func.avs_release_clip.is_some() && !(*h).clip.is_null() {
        (*h).func
            .avs_release_clip
            .expect("non-null function pointer")((*h).clip);
    }
    if (*h).func.avs_delete_script_environment.is_some() && !(*h).env.is_null() {
        (*h).func
            .avs_delete_script_environment
            .expect("non-null function pointer")((*h).env);
    }
    if !(*h).library.is_null() {
        dlclose((*h).library);
    }
    free(h as *mut c_void);
    return 0 as c_int;
}
#[no_mangle]
#[c2rust::src_loc = "564:19"]
static mut avs_input: cli_input_t = cli_input_t {
    open_file: Some(
        open_file
            as unsafe extern "C" fn(
                *mut c_char,
                *mut hnd_t,
                *mut video_info_t,
                *mut cli_input_opt_t,
            ) -> c_int,
    ),
    picture_alloc: Some(
        picture_alloc as unsafe extern "C" fn(*mut cli_pic_t, hnd_t, c_int, c_int, c_int) -> c_int,
    ),
    read_frame: Some(read_frame as unsafe extern "C" fn(*mut cli_pic_t, hnd_t, c_int) -> c_int),
    release_frame: Some(release_frame as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> c_int),
    picture_clean: Some(picture_clean as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()),
    close_file: Some(close_file as unsafe extern "C" fn(hnd_t) -> c_int),
};
