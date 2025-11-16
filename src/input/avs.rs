use ::c2rust_bitfields;
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:27"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = i64;
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = u64;
    #[c2rust::src_loc = "145:1"]
    pub type __dev_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "146:1"]
    pub type __uid_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "147:1"]
    pub type __gid_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "148:1"]
    pub type __ino_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "150:1"]
    pub type __mode_t = ::core::ffi::c_uint;
    #[c2rust::src_loc = "151:1"]
    pub type __nlink_t = ::core::ffi::c_ulong;
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "160:1"]
    pub type __time_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "175:1"]
    pub type __blksize_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "180:1"]
    pub type __blkcnt_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "197:1"]
    pub type __syscall_slong_t = ::core::ffi::c_long;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:27"]
pub mod struct_FILE_h {
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
    #[c2rust::src_loc = "51:8"]
    pub struct _IO_FILE {
        pub _flags: ::core::ffi::c_int,
        pub _IO_read_ptr: *mut ::core::ffi::c_char,
        pub _IO_read_end: *mut ::core::ffi::c_char,
        pub _IO_read_base: *mut ::core::ffi::c_char,
        pub _IO_write_base: *mut ::core::ffi::c_char,
        pub _IO_write_ptr: *mut ::core::ffi::c_char,
        pub _IO_write_end: *mut ::core::ffi::c_char,
        pub _IO_buf_base: *mut ::core::ffi::c_char,
        pub _IO_buf_end: *mut ::core::ffi::c_char,
        pub _IO_save_base: *mut ::core::ffi::c_char,
        pub _IO_backup_base: *mut ::core::ffi::c_char,
        pub _IO_save_end: *mut ::core::ffi::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: __off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut _IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: __uint64_t,
        pub _unused2: [::core::ffi::c_char; 8],
    }
    #[c2rust::src_loc = "45:1"]
    pub type _IO_lock_t = ();
    use super::types_h::{__off_t, __off64_t, __uint64_t};
    extern "C" {
        #[c2rust::src_loc = "40:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "39:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "38:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:27"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:27"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__time_t, __syscall_slong_t};
}
#[c2rust::header_src = "/usr/include/bits/struct_stat.h:27"]
pub mod struct_stat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "26:8"]
    pub struct stat {
        pub st_dev: __dev_t,
        pub st_ino: __ino_t,
        pub st_nlink: __nlink_t,
        pub st_mode: __mode_t,
        pub st_uid: __uid_t,
        pub st_gid: __gid_t,
        pub __pad0: ::core::ffi::c_int,
        pub st_rdev: __dev_t,
        pub st_size: __off_t,
        pub st_blksize: __blksize_t,
        pub st_blocks: __blkcnt_t,
        pub st_atim: timespec,
        pub st_mtim: timespec,
        pub st_ctim: timespec,
        pub __glibc_reserved: [__syscall_slong_t; 3],
    }
    use super::types_h::{
        __dev_t, __ino_t, __nlink_t, __mode_t, __uid_t, __gid_t, __off_t, __blksize_t,
        __blkcnt_t, __syscall_slong_t,
    };
    use super::struct_timespec_h::timespec;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::__int64_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264cli.h:27"]
pub mod x264cli_h {
    #[c2rust::src_loc = "37:1"]
    pub type hnd_t = *mut ::core::ffi::c_void;
    #[inline]
    #[c2rust::src_loc = "67:1"]
    pub unsafe extern "C" fn get_filename_extension(
        mut filename: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char {
        let mut ext: *mut ::core::ffi::c_char = filename
            .offset(strlen(filename) as isize);
        while *ext as ::core::ffi::c_int != '.' as i32 && ext > filename {
            ext = ext.offset(-1);
        }
        ext = ext
            .offset(
                (*ext as ::core::ffi::c_int == '.' as i32) as ::core::ffi::c_int as isize,
            );
        return ext;
    }
    use super::string_h::strlen;
    extern "C" {
        #[c2rust::src_loc = "76:1"]
        pub fn x264_cli_log(
            name: *const ::core::ffi::c_char,
            i_level: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
        #[c2rust::src_loc = "77:1"]
        pub fn x264_cli_printf(
            i_level: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/input/input.h:27"]
pub mod input_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:9"]
    pub struct cli_input_opt_t {
        pub index_file: *mut ::core::ffi::c_char,
        pub format: *mut ::core::ffi::c_char,
        pub resolution: *mut ::core::ffi::c_char,
        pub colorspace: *mut ::core::ffi::c_char,
        pub bit_depth: ::core::ffi::c_int,
        pub timebase: *mut ::core::ffi::c_char,
        pub seek: ::core::ffi::c_int,
        pub progress: ::core::ffi::c_int,
        pub output_csp: ::core::ffi::c_int,
        pub output_range: ::core::ffi::c_int,
        pub input_range: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "54:9"]
    pub struct video_info_t {
        pub csp: ::core::ffi::c_int,
        pub fps_num: uint32_t,
        pub fps_den: uint32_t,
        pub fullrange: ::core::ffi::c_int,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub interlaced: ::core::ffi::c_int,
        pub num_frames: ::core::ffi::c_int,
        pub sar_width: uint32_t,
        pub sar_height: uint32_t,
        pub tff: ::core::ffi::c_int,
        pub thread_safe: ::core::ffi::c_int,
        pub timebase_num: uint32_t,
        pub timebase_den: uint32_t,
        pub vfr: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "74:9"]
    pub struct cli_image_t {
        pub csp: ::core::ffi::c_int,
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub planes: ::core::ffi::c_int,
        pub plane: [*mut uint8_t; 4],
        pub stride: [::core::ffi::c_int; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "84:9"]
    pub struct cli_pic_t {
        pub img: cli_image_t,
        pub pts: int64_t,
        pub duration: int64_t,
        pub opaque: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "92:9"]
    pub struct cli_input_t {
        pub open_file: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_char,
                *mut hnd_t,
                *mut video_info_t,
                *mut cli_input_opt_t,
            ) -> ::core::ffi::c_int,
        >,
        pub picture_alloc: Option<
            unsafe extern "C" fn(
                *mut cli_pic_t,
                hnd_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub read_frame: Option<
            unsafe extern "C" fn(
                *mut cli_pic_t,
                hnd_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub release_frame: Option<
            unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ::core::ffi::c_int,
        >,
        pub picture_clean: Option<unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ()>,
        pub close_file: Option<unsafe extern "C" fn(hnd_t) -> ::core::ffi::c_int>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "117:9"]
    pub struct x264_cli_csp_t {
        pub name: *const ::core::ffi::c_char,
        pub planes: ::core::ffi::c_int,
        pub width: [::core::ffi::c_float; 4],
        pub height: [::core::ffi::c_float; 4],
        pub mod_width: ::core::ffi::c_int,
        pub mod_height: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "115:9"]
    pub const X264_CSP_OTHER: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    use super::stdint_intn_h::int64_t;
    use super::x264cli_h::hnd_t;
    extern "C" {
        #[c2rust::src_loc = "131:1"]
        pub fn x264_cli_pic_alloc(
            pic: *mut cli_pic_t,
            csp: ::core::ffi::c_int,
            width: ::core::ffi::c_int,
            height: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "137:1"]
        pub fn x264_cli_get_csp(csp: ::core::ffi::c_int) -> *const x264_cli_csp_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/extras/avisynth_c.h:27"]
pub mod avisynth_c_h {
    #[c2rust::src_loc = "812:1"]
    pub type avs_is_y_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "618:16"]
    pub struct AVS_VideoInfo {
        pub width: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub fps_numerator: ::core::ffi::c_uint,
        pub fps_denominator: ::core::ffi::c_uint,
        pub num_frames: ::core::ffi::c_int,
        pub pixel_type: ::core::ffi::c_int,
        pub audio_samples_per_second: ::core::ffi::c_int,
        pub sample_type: ::core::ffi::c_int,
        pub num_audio_samples: int64_t,
        pub nchannels: ::core::ffi::c_int,
        pub image_type: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "810:1"]
    pub type avs_is_420_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "808:1"]
    pub type avs_is_422_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "806:1"]
    pub type avs_is_444_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "800:1"]
    pub type avs_is_y16_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "799:1"]
    pub type avs_is_yuv420p16_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "798:1"]
    pub type avs_is_yuv422p16_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "797:1"]
    pub type avs_is_yuv444p16_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "795:1"]
    pub type avs_is_rgb64_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "793:1"]
    pub type avs_is_rgb48_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "880:1"]
    pub type avs_get_read_ptr_p_func = Option<
        unsafe extern "C" fn(*const AVS_VideoFrame, ::core::ffi::c_int) -> *const BYTE,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "857:16"]
    pub struct AVS_VideoFrame {
        pub refcount: ::core::ffi::c_long,
        pub vfb: *mut AVS_VideoFrameBuffer,
        pub offset: ::core::ffi::c_int,
        pub pitch: ::core::ffi::c_int,
        pub row_size: ::core::ffi::c_int,
        pub height: ::core::ffi::c_int,
        pub offsetU: ::core::ffi::c_int,
        pub offsetV: ::core::ffi::c_int,
        pub pitchUV: ::core::ffi::c_int,
        pub row_sizeUV: ::core::ffi::c_int,
        pub heightUV: ::core::ffi::c_int,
        pub offsetA: ::core::ffi::c_int,
        pub pitchA: ::core::ffi::c_int,
        pub row_sizeA: ::core::ffi::c_int,
        pub properties: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "841:16"]
    pub struct AVS_VideoFrameBuffer {
        pub data: *mut BYTE,
        pub data_size: ::core::ffi::c_int,
        pub sequence_number: ::core::ffi::c_long,
        pub refcount: ::core::ffi::c_long,
        pub device: *mut ::core::ffi::c_void,
    }
    #[c2rust::src_loc = "257:1"]
    pub type BYTE = uint8_t;
    #[c2rust::src_loc = "874:1"]
    pub type avs_get_pitch_p_func = Option<
        unsafe extern "C" fn(
            *const AVS_VideoFrame,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "665:1"]
    pub type avs_is_y8_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "663:1"]
    pub type avs_is_yv411_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "661:1"]
    pub type avs_is_yv12_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "659:1"]
    pub type avs_is_yv16_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "657:1"]
    pub type avs_is_yv24_func = Option<
        unsafe extern "C" fn(*const AVS_VideoInfo) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "1022:1"]
    pub type avs_take_clip_func = Option<
        unsafe extern "C" fn(AVS_Value, *mut AVS_ScriptEnvironment) -> *mut AVS_Clip,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "994:8"]
    pub struct AVS_Value {
        pub type_0: ::core::ffi::c_short,
        pub array_size: ::core::ffi::c_short,
        pub d: C2RustUnnamed_0,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "998:3"]
    pub union C2RustUnnamed_0 {
        pub clip: *mut ::core::ffi::c_void,
        pub boolean: ::core::ffi::c_char,
        pub integer: ::core::ffi::c_int,
        pub floating_pt: ::core::ffi::c_float,
        pub string: *const ::core::ffi::c_char,
        pub array: *const AVS_Value,
        pub function: *mut ::core::ffi::c_void,
        pub longlong: int64_t,
        pub double_pt: ::core::ffi::c_double,
    }
    #[c2rust::src_loc = "937:1"]
    pub type avs_release_video_frame_func = Option<
        unsafe extern "C" fn(*mut AVS_VideoFrame) -> (),
    >;
    #[c2rust::src_loc = "1021:1"]
    pub type avs_release_value_func = Option<unsafe extern "C" fn(AVS_Value) -> ()>;
    #[c2rust::src_loc = "1079:1"]
    pub type avs_release_clip_func = Option<unsafe extern "C" fn(*mut AVS_Clip) -> ()>;
    #[c2rust::src_loc = "1199:1"]
    pub type avs_invoke_func = Option<
        unsafe extern "C" fn(
            *mut AVS_ScriptEnvironment,
            *const ::core::ffi::c_char,
            AVS_Value,
            *mut *const ::core::ffi::c_char,
        ) -> AVS_Value,
    >;
    #[c2rust::src_loc = "1197:1"]
    pub type avs_function_exists_func = Option<
        unsafe extern "C" fn(
            *mut AVS_ScriptEnvironment,
            *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >;
    #[c2rust::src_loc = "1084:1"]
    pub type avs_get_video_info_func = Option<
        unsafe extern "C" fn(*mut AVS_Clip) -> *const AVS_VideoInfo,
    >;
    #[c2rust::src_loc = "1088:1"]
    pub type avs_get_frame_func = Option<
        unsafe extern "C" fn(*mut AVS_Clip, ::core::ffi::c_int) -> *mut AVS_VideoFrame,
    >;
    #[c2rust::src_loc = "1183:1"]
    pub type avs_get_error_func = Option<
        unsafe extern "C" fn(*mut AVS_ScriptEnvironment) -> *const ::core::ffi::c_char,
    >;
    #[c2rust::src_loc = "1265:1"]
    pub type avs_delete_script_environment_func = Option<
        unsafe extern "C" fn(*mut AVS_ScriptEnvironment) -> (),
    >;
    #[c2rust::src_loc = "1257:1"]
    pub type avs_create_script_environment_func = Option<
        unsafe extern "C" fn(::core::ffi::c_int) -> *mut AVS_ScriptEnvironment,
    >;
    #[c2rust::src_loc = "1082:1"]
    pub type avs_clip_get_error_func = Option<
        unsafe extern "C" fn(*mut AVS_Clip) -> *const ::core::ffi::c_char,
    >;
    #[c2rust::src_loc = "282:7"]
    pub const AVS_PLANAR_V: C2RustUnnamed_1 = 4;
    #[c2rust::src_loc = "281:7"]
    pub const AVS_PLANAR_U: C2RustUnnamed_1 = 2;
    #[c2rust::src_loc = "280:7"]
    pub const AVS_PLANAR_Y: C2RustUnnamed_1 = 1;
    #[c2rust::src_loc = "287:7"]
    pub const AVS_PLANAR_A: C2RustUnnamed_1 = 16;
    #[c2rust::src_loc = "288:7"]
    pub const AVS_PLANAR_R: C2RustUnnamed_1 = 32;
    #[c2rust::src_loc = "290:7"]
    pub const AVS_PLANAR_B: C2RustUnnamed_1 = 128;
    #[c2rust::src_loc = "330:5"]
    pub const AVS_CS_PLANAR_FILTER: C2RustUnnamed_2 = -25;
    #[c2rust::src_loc = "361:3"]
    pub const AVS_CS_YV411: C2RustUnnamed_3 = -1610611959;
    #[c2rust::src_loc = "329:5"]
    pub const AVS_CS_PLANAR_MASK: C2RustUnnamed_2 = -133757177;
    #[c2rust::src_loc = "351:3"]
    pub const AVS_CS_YUY2: C2RustUnnamed_3 = 1610612740;
    #[c2rust::src_loc = "363:3"]
    pub const AVS_CS_Y8: C2RustUnnamed_3 = -536870912;
    #[c2rust::src_loc = "358:3"]
    pub const AVS_CS_YV12: C2RustUnnamed_3 = -1610612728;
    #[c2rust::src_loc = "357:3"]
    pub const AVS_CS_YV16: C2RustUnnamed_3 = -1610611960;
    #[c2rust::src_loc = "356:3"]
    pub const AVS_CS_YV24: C2RustUnnamed_3 = -1610611957;
    #[c2rust::src_loc = "322:5"]
    pub const AVS_CS_SAMPLE_BITS_8: C2RustUnnamed_2 = 0;
    #[c2rust::src_loc = "321:5"]
    pub const AVS_CS_SAMPLE_BITS_MASK: C2RustUnnamed_2 = 458752;
    #[c2rust::src_loc = "349:3"]
    pub const AVS_CS_BGR24: C2RustUnnamed_3 = 1342177281;
    #[c2rust::src_loc = "350:3"]
    pub const AVS_CS_BGR32: C2RustUnnamed_3 = 1342177282;
    #[c2rust::src_loc = "444:3"]
    pub const AVS_IT_TFF: C2RustUnnamed_4 = 2;
    #[c2rust::src_loc = "445:3"]
    pub const AVS_IT_FIELDBASED: C2RustUnnamed_4 = 4;
    #[c2rust::src_loc = "280:1"]
    pub type C2RustUnnamed_1 = ::core::ffi::c_uint;
    #[c2rust::src_loc = "294:7"]
    pub const AVS_PLANAR_B_ALIGNED: C2RustUnnamed_1 = 136;
    #[c2rust::src_loc = "293:7"]
    pub const AVS_PLANAR_G_ALIGNED: C2RustUnnamed_1 = 72;
    #[c2rust::src_loc = "292:7"]
    pub const AVS_PLANAR_R_ALIGNED: C2RustUnnamed_1 = 40;
    #[c2rust::src_loc = "291:7"]
    pub const AVS_PLANAR_A_ALIGNED: C2RustUnnamed_1 = 24;
    #[c2rust::src_loc = "289:7"]
    pub const AVS_PLANAR_G: C2RustUnnamed_1 = 64;
    #[c2rust::src_loc = "286:7"]
    pub const AVS_PLANAR_V_ALIGNED: C2RustUnnamed_1 = 12;
    #[c2rust::src_loc = "285:7"]
    pub const AVS_PLANAR_U_ALIGNED: C2RustUnnamed_1 = 10;
    #[c2rust::src_loc = "284:7"]
    pub const AVS_PLANAR_Y_ALIGNED: C2RustUnnamed_1 = 9;
    #[c2rust::src_loc = "283:7"]
    pub const AVS_PLANAR_ALIGNED: C2RustUnnamed_1 = 8;
    #[c2rust::src_loc = "297:1"]
    pub type C2RustUnnamed_2 = ::core::ffi::c_int;
    #[c2rust::src_loc = "343:5"]
    pub const AVS_CS_GENERIC_YUVA444: C2RustUnnamed_2 = -2013265141;
    #[c2rust::src_loc = "342:5"]
    pub const AVS_CS_GENERIC_YUVA422: C2RustUnnamed_2 = -2013265144;
    #[c2rust::src_loc = "341:5"]
    pub const AVS_CS_GENERIC_YUVA420: C2RustUnnamed_2 = -2013265912;
    #[c2rust::src_loc = "340:5"]
    pub const AVS_CS_GENERIC_RGBAP: C2RustUnnamed_2 = -1879048190;
    #[c2rust::src_loc = "339:5"]
    pub const AVS_CS_GENERIC_RGBP: C2RustUnnamed_2 = -1879048191;
    #[c2rust::src_loc = "338:5"]
    pub const AVS_CS_GENERIC_Y: C2RustUnnamed_2 = -536870912;
    #[c2rust::src_loc = "337:5"]
    pub const AVS_CS_GENERIC_YUV444: C2RustUnnamed_2 = -1610611957;
    #[c2rust::src_loc = "336:5"]
    pub const AVS_CS_GENERIC_YUV422: C2RustUnnamed_2 = -1610611960;
    #[c2rust::src_loc = "335:5"]
    pub const AVS_CS_GENERIC_YUV420: C2RustUnnamed_2 = -1610612728;
    #[c2rust::src_loc = "333:5"]
    pub const AVS_CS_RGBA_TYPE: C2RustUnnamed_2 = 2;
    #[c2rust::src_loc = "332:5"]
    pub const AVS_CS_RGB_TYPE: C2RustUnnamed_2 = 1;
    #[c2rust::src_loc = "327:5"]
    pub const AVS_CS_SAMPLE_BITS_32: C2RustUnnamed_2 = 131072;
    #[c2rust::src_loc = "326:5"]
    pub const AVS_CS_SAMPLE_BITS_16: C2RustUnnamed_2 = 65536;
    #[c2rust::src_loc = "325:5"]
    pub const AVS_CS_SAMPLE_BITS_14: C2RustUnnamed_2 = 458752;
    #[c2rust::src_loc = "324:5"]
    pub const AVS_CS_SAMPLE_BITS_12: C2RustUnnamed_2 = 393216;
    #[c2rust::src_loc = "323:5"]
    pub const AVS_CS_SAMPLE_BITS_10: C2RustUnnamed_2 = 327680;
    #[c2rust::src_loc = "319:5"]
    pub const AVS_CS_SUB_HEIGHT_4: C2RustUnnamed_2 = 256;
    #[c2rust::src_loc = "318:5"]
    pub const AVS_CS_SUB_HEIGHT_2: C2RustUnnamed_2 = 0;
    #[c2rust::src_loc = "317:5"]
    pub const AVS_CS_SUB_HEIGHT_1: C2RustUnnamed_2 = 768;
    #[c2rust::src_loc = "316:5"]
    pub const AVS_CS_SUB_HEIGHT_MASK: C2RustUnnamed_2 = 1792;
    #[c2rust::src_loc = "314:5"]
    pub const AVS_CS_UPLANEFIRST: C2RustUnnamed_2 = 16;
    #[c2rust::src_loc = "313:5"]
    pub const AVS_CS_VPLANEFIRST: C2RustUnnamed_2 = 8;
    #[c2rust::src_loc = "311:5"]
    pub const AVS_CS_SUB_WIDTH_4: C2RustUnnamed_2 = 1;
    #[c2rust::src_loc = "310:5"]
    pub const AVS_CS_SUB_WIDTH_2: C2RustUnnamed_2 = 0;
    #[c2rust::src_loc = "309:5"]
    pub const AVS_CS_SUB_WIDTH_1: C2RustUnnamed_2 = 3;
    #[c2rust::src_loc = "308:5"]
    pub const AVS_CS_SUB_WIDTH_MASK: C2RustUnnamed_2 = 7;
    #[c2rust::src_loc = "306:5"]
    pub const AVS_CS_SHIFT_SAMPLE_BITS: C2RustUnnamed_2 = 16;
    #[c2rust::src_loc = "305:5"]
    pub const AVS_CS_SHIFT_SUB_HEIGHT: C2RustUnnamed_2 = 8;
    #[c2rust::src_loc = "304:5"]
    pub const AVS_CS_SHIFT_SUB_WIDTH: C2RustUnnamed_2 = 0;
    #[c2rust::src_loc = "302:5"]
    pub const AVS_CS_PLANAR: C2RustUnnamed_2 = -2147483648;
    #[c2rust::src_loc = "301:5"]
    pub const AVS_CS_INTERLEAVED: C2RustUnnamed_2 = 1073741824;
    #[c2rust::src_loc = "300:5"]
    pub const AVS_CS_YUV: C2RustUnnamed_2 = 536870912;
    #[c2rust::src_loc = "299:5"]
    pub const AVS_CS_BGR: C2RustUnnamed_2 = 268435456;
    #[c2rust::src_loc = "298:5"]
    pub const AVS_CS_YUVA: C2RustUnnamed_2 = 134217728;
    #[c2rust::src_loc = "347:1"]
    pub type C2RustUnnamed_3 = ::core::ffi::c_int;
    #[c2rust::src_loc = "438:3"]
    pub const AVS_CS_YUVA420PS: C2RustUnnamed_3 = -2013134840;
    #[c2rust::src_loc = "437:3"]
    pub const AVS_CS_YUVA422PS: C2RustUnnamed_3 = -2013134072;
    #[c2rust::src_loc = "436:3"]
    pub const AVS_CS_YUVA444PS: C2RustUnnamed_3 = -2013134069;
    #[c2rust::src_loc = "434:3"]
    pub const AVS_CS_YUVA420P16: C2RustUnnamed_3 = -2013200376;
    #[c2rust::src_loc = "433:3"]
    pub const AVS_CS_YUVA422P16: C2RustUnnamed_3 = -2013199608;
    #[c2rust::src_loc = "432:3"]
    pub const AVS_CS_YUVA444P16: C2RustUnnamed_3 = -2013199605;
    #[c2rust::src_loc = "430:3"]
    pub const AVS_CS_YUVA420P14: C2RustUnnamed_3 = -2012807160;
    #[c2rust::src_loc = "429:3"]
    pub const AVS_CS_YUVA422P14: C2RustUnnamed_3 = -2012806392;
    #[c2rust::src_loc = "428:3"]
    pub const AVS_CS_YUVA444P14: C2RustUnnamed_3 = -2012806389;
    #[c2rust::src_loc = "426:3"]
    pub const AVS_CS_YUVA420P12: C2RustUnnamed_3 = -2012872696;
    #[c2rust::src_loc = "425:3"]
    pub const AVS_CS_YUVA422P12: C2RustUnnamed_3 = -2012871928;
    #[c2rust::src_loc = "424:3"]
    pub const AVS_CS_YUVA444P12: C2RustUnnamed_3 = -2012871925;
    #[c2rust::src_loc = "422:3"]
    pub const AVS_CS_YUVA420P10: C2RustUnnamed_3 = -2012938232;
    #[c2rust::src_loc = "421:3"]
    pub const AVS_CS_YUVA422P10: C2RustUnnamed_3 = -2012937464;
    #[c2rust::src_loc = "420:3"]
    pub const AVS_CS_YUVA444P10: C2RustUnnamed_3 = -2012937461;
    #[c2rust::src_loc = "418:3"]
    pub const AVS_CS_YUVA420: C2RustUnnamed_3 = -2013265912;
    #[c2rust::src_loc = "417:3"]
    pub const AVS_CS_YUVA422: C2RustUnnamed_3 = -2013265144;
    #[c2rust::src_loc = "416:3"]
    pub const AVS_CS_YUVA444: C2RustUnnamed_3 = -2013265141;
    #[c2rust::src_loc = "413:3"]
    pub const AVS_CS_RGBAPS: C2RustUnnamed_3 = -1878917118;
    #[c2rust::src_loc = "412:3"]
    pub const AVS_CS_RGBAP16: C2RustUnnamed_3 = -1878982654;
    #[c2rust::src_loc = "411:3"]
    pub const AVS_CS_RGBAP14: C2RustUnnamed_3 = -1878589438;
    #[c2rust::src_loc = "410:3"]
    pub const AVS_CS_RGBAP12: C2RustUnnamed_3 = -1878654974;
    #[c2rust::src_loc = "409:3"]
    pub const AVS_CS_RGBAP10: C2RustUnnamed_3 = -1878720510;
    #[c2rust::src_loc = "408:3"]
    pub const AVS_CS_RGBAP: C2RustUnnamed_3 = -1879048190;
    #[c2rust::src_loc = "405:3"]
    pub const AVS_CS_RGBPS: C2RustUnnamed_3 = -1878917119;
    #[c2rust::src_loc = "404:3"]
    pub const AVS_CS_RGBP16: C2RustUnnamed_3 = -1878982655;
    #[c2rust::src_loc = "403:3"]
    pub const AVS_CS_RGBP14: C2RustUnnamed_3 = -1878589439;
    #[c2rust::src_loc = "402:3"]
    pub const AVS_CS_RGBP12: C2RustUnnamed_3 = -1878654975;
    #[c2rust::src_loc = "401:3"]
    pub const AVS_CS_RGBP10: C2RustUnnamed_3 = -1878720511;
    #[c2rust::src_loc = "400:3"]
    pub const AVS_CS_RGBP: C2RustUnnamed_3 = -1879048191;
    #[c2rust::src_loc = "396:3"]
    pub const AVS_CS_BGR64: C2RustUnnamed_3 = 1342242818;
    #[c2rust::src_loc = "395:3"]
    pub const AVS_CS_BGR48: C2RustUnnamed_3 = 1342242817;
    #[c2rust::src_loc = "392:3"]
    pub const AVS_CS_Y32: C2RustUnnamed_3 = -536739840;
    #[c2rust::src_loc = "391:3"]
    pub const AVS_CS_YUV420PS: C2RustUnnamed_3 = -1610481656;
    #[c2rust::src_loc = "390:3"]
    pub const AVS_CS_YUV422PS: C2RustUnnamed_3 = -1610480888;
    #[c2rust::src_loc = "389:3"]
    pub const AVS_CS_YUV444PS: C2RustUnnamed_3 = -1610480885;
    #[c2rust::src_loc = "386:3"]
    pub const AVS_CS_Y16: C2RustUnnamed_3 = -536805376;
    #[c2rust::src_loc = "385:3"]
    pub const AVS_CS_YUV420P16: C2RustUnnamed_3 = -1610547192;
    #[c2rust::src_loc = "384:3"]
    pub const AVS_CS_YUV422P16: C2RustUnnamed_3 = -1610546424;
    #[c2rust::src_loc = "383:3"]
    pub const AVS_CS_YUV444P16: C2RustUnnamed_3 = -1610546421;
    #[c2rust::src_loc = "381:3"]
    pub const AVS_CS_Y14: C2RustUnnamed_3 = -536412160;
    #[c2rust::src_loc = "380:3"]
    pub const AVS_CS_YUV420P14: C2RustUnnamed_3 = -1610153976;
    #[c2rust::src_loc = "379:3"]
    pub const AVS_CS_YUV422P14: C2RustUnnamed_3 = -1610153208;
    #[c2rust::src_loc = "378:3"]
    pub const AVS_CS_YUV444P14: C2RustUnnamed_3 = -1610153205;
    #[c2rust::src_loc = "376:3"]
    pub const AVS_CS_Y12: C2RustUnnamed_3 = -536477696;
    #[c2rust::src_loc = "375:3"]
    pub const AVS_CS_YUV420P12: C2RustUnnamed_3 = -1610219512;
    #[c2rust::src_loc = "374:3"]
    pub const AVS_CS_YUV422P12: C2RustUnnamed_3 = -1610218744;
    #[c2rust::src_loc = "373:3"]
    pub const AVS_CS_YUV444P12: C2RustUnnamed_3 = -1610218741;
    #[c2rust::src_loc = "371:3"]
    pub const AVS_CS_Y10: C2RustUnnamed_3 = -536543232;
    #[c2rust::src_loc = "370:3"]
    pub const AVS_CS_YUV420P10: C2RustUnnamed_3 = -1610285048;
    #[c2rust::src_loc = "369:3"]
    pub const AVS_CS_YUV422P10: C2RustUnnamed_3 = -1610284280;
    #[c2rust::src_loc = "368:3"]
    pub const AVS_CS_YUV444P10: C2RustUnnamed_3 = -1610284277;
    #[c2rust::src_loc = "362:3"]
    pub const AVS_CS_YUV9: C2RustUnnamed_3 = -1610612471;
    #[c2rust::src_loc = "360:3"]
    pub const AVS_CS_IYUV: C2RustUnnamed_3 = -1610612720;
    #[c2rust::src_loc = "359:3"]
    pub const AVS_CS_I420: C2RustUnnamed_3 = -1610612720;
    #[c2rust::src_loc = "354:3"]
    pub const AVS_CS_RAW32: C2RustUnnamed_3 = 1073741856;
    #[c2rust::src_loc = "348:3"]
    pub const AVS_CS_UNKNOWN: C2RustUnnamed_3 = 0;
    #[c2rust::src_loc = "442:1"]
    pub type C2RustUnnamed_4 = ::core::ffi::c_uint;
    #[c2rust::src_loc = "443:3"]
    pub const AVS_IT_BFF: C2RustUnnamed_4 = 1;
    #[inline]
    #[c2rust::src_loc = "636:1"]
    pub unsafe extern "C" fn avs_has_video(
        mut p: *const AVS_VideoInfo,
    ) -> ::core::ffi::c_int {
        return ((*p).width != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "645:1"]
    pub unsafe extern "C" fn avs_is_rgb24(
        mut p: *const AVS_VideoInfo,
    ) -> ::core::ffi::c_int {
        return ((*p).pixel_type & AVS_CS_BGR24 as ::core::ffi::c_int
            == AVS_CS_BGR24 as ::core::ffi::c_int
            && (*p).pixel_type & AVS_CS_SAMPLE_BITS_MASK as ::core::ffi::c_int
                == AVS_CS_SAMPLE_BITS_8 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "648:1"]
    pub unsafe extern "C" fn avs_is_rgb32(
        mut p: *const AVS_VideoInfo,
    ) -> ::core::ffi::c_int {
        return ((*p).pixel_type & AVS_CS_BGR32 as ::core::ffi::c_int
            == AVS_CS_BGR32 as ::core::ffi::c_int
            && (*p).pixel_type & AVS_CS_SAMPLE_BITS_MASK as ::core::ffi::c_int
                == AVS_CS_SAMPLE_BITS_8 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "654:1"]
    pub unsafe extern "C" fn avs_is_yuy2(
        mut p: *const AVS_VideoInfo,
    ) -> ::core::ffi::c_int {
        return ((*p).pixel_type & AVS_CS_YUY2 as ::core::ffi::c_int
            == AVS_CS_YUY2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "668:1"]
    pub unsafe extern "C" fn avs_is_yv24(
        mut p: *const AVS_VideoInfo,
    ) -> ::core::ffi::c_int {
        return ((*p).pixel_type & AVS_CS_PLANAR_MASK as ::core::ffi::c_int
            == AVS_CS_YV24 as ::core::ffi::c_int
                & AVS_CS_PLANAR_FILTER as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "671:1"]
    pub unsafe extern "C" fn avs_is_yv16(
        mut p: *const AVS_VideoInfo,
    ) -> ::core::ffi::c_int {
        return ((*p).pixel_type & AVS_CS_PLANAR_MASK as ::core::ffi::c_int
            == AVS_CS_YV16 as ::core::ffi::c_int
                & AVS_CS_PLANAR_FILTER as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "674:1"]
    pub unsafe extern "C" fn avs_is_yv12(
        mut p: *const AVS_VideoInfo,
    ) -> ::core::ffi::c_int {
        return ((*p).pixel_type & AVS_CS_PLANAR_MASK as ::core::ffi::c_int
            == AVS_CS_YV12 as ::core::ffi::c_int
                & AVS_CS_PLANAR_FILTER as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "677:1"]
    pub unsafe extern "C" fn avs_is_yv411(
        mut p: *const AVS_VideoInfo,
    ) -> ::core::ffi::c_int {
        return ((*p).pixel_type & AVS_CS_PLANAR_MASK as ::core::ffi::c_int
            == AVS_CS_YV411 as ::core::ffi::c_int
                & AVS_CS_PLANAR_FILTER as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "680:1"]
    pub unsafe extern "C" fn avs_is_y8(
        mut p: *const AVS_VideoInfo,
    ) -> ::core::ffi::c_int {
        return ((*p).pixel_type & AVS_CS_PLANAR_MASK as ::core::ffi::c_int
            == AVS_CS_Y8 as ::core::ffi::c_int
                & AVS_CS_PLANAR_FILTER as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "709:1"]
    pub unsafe extern "C" fn avs_is_field_based(
        mut p: *const AVS_VideoInfo,
    ) -> ::core::ffi::c_int {
        return ((*p).image_type & AVS_IT_FIELDBASED as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "724:1"]
    pub unsafe extern "C" fn avs_is_tff(
        mut p: *const AVS_VideoInfo,
    ) -> ::core::ffi::c_int {
        return ((*p).image_type & AVS_IT_TFF as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "883:1"]
    pub unsafe extern "C" fn avs_get_pitch_p(
        mut p: *const AVS_VideoFrame,
        mut plane: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        match plane {
            2 | 4 => return (*p).pitchUV,
            16 => return (*p).pitchA,
            _ => {}
        }
        return (*p).pitch;
    }
    #[inline]
    #[c2rust::src_loc = "916:1"]
    pub unsafe extern "C" fn avs_get_read_ptr_p(
        mut p: *const AVS_VideoFrame,
        mut plane: ::core::ffi::c_int,
    ) -> *const BYTE {
        match plane {
            2 | 128 => return (*(*p).vfb).data.offset((*p).offsetU as isize),
            4 | 32 => return (*(*p).vfb).data.offset((*p).offsetV as isize),
            16 => return (*(*p).vfb).data.offset((*p).offsetA as isize),
            _ => {}
        }
        return (*(*p).vfb).data.offset((*p).offset as isize);
    }
    #[inline]
    #[c2rust::src_loc = "1028:1"]
    pub unsafe extern "C" fn avs_is_clip(mut v: AVS_Value) -> ::core::ffi::c_int {
        return (v.type_0 as ::core::ffi::c_int == 'c' as i32) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "1030:1"]
    pub unsafe extern "C" fn avs_is_int(mut v: AVS_Value) -> ::core::ffi::c_int {
        return (v.type_0 as ::core::ffi::c_int == 'i' as i32) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "1031:1"]
    pub unsafe extern "C" fn avs_is_float(mut v: AVS_Value) -> ::core::ffi::c_int {
        return (v.type_0 as ::core::ffi::c_int == 'f' as i32
            || v.type_0 as ::core::ffi::c_int == 'i' as i32) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "1032:1"]
    pub unsafe extern "C" fn avs_is_string(mut v: AVS_Value) -> ::core::ffi::c_int {
        return (v.type_0 as ::core::ffi::c_int == 's' as i32) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "1034:1"]
    pub unsafe extern "C" fn avs_is_error(mut v: AVS_Value) -> ::core::ffi::c_int {
        return (v.type_0 as ::core::ffi::c_int == 'e' as i32) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "1038:1"]
    pub unsafe extern "C" fn avs_as_int(mut v: AVS_Value) -> ::core::ffi::c_int {
        return v.d.integer;
    }
    #[inline]
    #[c2rust::src_loc = "1040:1"]
    pub unsafe extern "C" fn avs_as_string(
        mut v: AVS_Value,
    ) -> *const ::core::ffi::c_char {
        return if avs_is_error(v) != 0 || avs_is_string(v) != 0 {
            v.d.string
        } else {
            0 as *const ::core::ffi::c_char
        };
    }
    #[inline]
    #[c2rust::src_loc = "1042:1"]
    pub unsafe extern "C" fn avs_as_float(mut v: AVS_Value) -> ::core::ffi::c_double {
        return (if avs_is_int(v) != 0 {
            v.d.integer as ::core::ffi::c_float
        } else {
            v.d.floating_pt
        }) as ::core::ffi::c_double;
    }
    #[inline]
    #[c2rust::src_loc = "1044:1"]
    pub unsafe extern "C" fn avs_as_error(
        mut v: AVS_Value,
    ) -> *const ::core::ffi::c_char {
        return if avs_is_error(v) != 0 {
            v.d.string
        } else {
            0 as *const ::core::ffi::c_char
        };
    }
    #[inline]
    #[c2rust::src_loc = "1055:1"]
    pub unsafe extern "C" fn avs_new_value_bool(
        mut v0: ::core::ffi::c_int,
    ) -> AVS_Value {
        let mut v: AVS_Value = AVS_Value {
            type_0: 0,
            array_size: 0,
            d: C2RustUnnamed_0 {
                clip: 0 as *mut ::core::ffi::c_void,
            },
        };
        v.type_0 = 'b' as i32 as ::core::ffi::c_short;
        v.d.boolean = (if v0 == 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as ::core::ffi::c_char;
        return v;
    }
    #[inline]
    #[c2rust::src_loc = "1059:1"]
    pub unsafe extern "C" fn avs_new_value_string(
        mut v0: *const ::core::ffi::c_char,
    ) -> AVS_Value {
        let mut v: AVS_Value = AVS_Value {
            type_0: 0,
            array_size: 0,
            d: C2RustUnnamed_0 {
                clip: 0 as *mut ::core::ffi::c_void,
            },
        };
        v.type_0 = 's' as i32 as ::core::ffi::c_short;
        v.d.string = v0;
        return v;
    }
    #[inline]
    #[c2rust::src_loc = "1070:1"]
    pub unsafe extern "C" fn avs_new_value_array(
        mut v0: *mut AVS_Value,
        mut size: ::core::ffi::c_int,
    ) -> AVS_Value {
        let mut v: AVS_Value = AVS_Value {
            type_0: 0,
            array_size: 0,
            d: C2RustUnnamed_0 {
                clip: 0 as *mut ::core::ffi::c_void,
            },
        };
        v.type_0 = 'a' as i32 as ::core::ffi::c_short;
        v.d.array = v0;
        v.array_size = size as ::core::ffi::c_short;
        return v;
    }
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "610:16"]
        pub type AVS_ScriptEnvironment;
        #[c2rust::src_loc = "609:16"]
        pub type AVS_Clip;
    }
}
#[c2rust::header_src = "/usr/include/libavutil/pixfmt.h:27"]
pub mod pixfmt_h {
    #[c2rust::src_loc = "80:5"]
    pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
    #[c2rust::src_loc = "71:1"]
    pub type AVPixelFormat = ::core::ffi::c_int;
    #[c2rust::src_loc = "502:5"]
    pub const AV_PIX_FMT_NB: AVPixelFormat = 267;
    #[c2rust::src_loc = "500:5"]
    pub const AV_PIX_FMT_OHCODEC: AVPixelFormat = 266;
    #[c2rust::src_loc = "498:5"]
    pub const AV_PIX_FMT_GBRP12MSBLE: AVPixelFormat = 265;
    #[c2rust::src_loc = "497:5"]
    pub const AV_PIX_FMT_GBRP12MSBBE: AVPixelFormat = 264;
    #[c2rust::src_loc = "496:5"]
    pub const AV_PIX_FMT_GBRP10MSBLE: AVPixelFormat = 263;
    #[c2rust::src_loc = "495:5"]
    pub const AV_PIX_FMT_GBRP10MSBBE: AVPixelFormat = 262;
    #[c2rust::src_loc = "494:5"]
    pub const AV_PIX_FMT_YUV444P12MSBLE: AVPixelFormat = 261;
    #[c2rust::src_loc = "493:5"]
    pub const AV_PIX_FMT_YUV444P12MSBBE: AVPixelFormat = 260;
    #[c2rust::src_loc = "492:5"]
    pub const AV_PIX_FMT_YUV444P10MSBLE: AVPixelFormat = 259;
    #[c2rust::src_loc = "491:5"]
    pub const AV_PIX_FMT_YUV444P10MSBBE: AVPixelFormat = 258;
    #[c2rust::src_loc = "489:5"]
    pub const AV_PIX_FMT_GBRAP32LE: AVPixelFormat = 257;
    #[c2rust::src_loc = "488:5"]
    pub const AV_PIX_FMT_GBRAP32BE: AVPixelFormat = 256;
    #[c2rust::src_loc = "486:5"]
    pub const AV_PIX_FMT_YAF16LE: AVPixelFormat = 255;
    #[c2rust::src_loc = "485:5"]
    pub const AV_PIX_FMT_YAF16BE: AVPixelFormat = 254;
    #[c2rust::src_loc = "483:5"]
    pub const AV_PIX_FMT_YAF32LE: AVPixelFormat = 253;
    #[c2rust::src_loc = "482:5"]
    pub const AV_PIX_FMT_YAF32BE: AVPixelFormat = 252;
    #[c2rust::src_loc = "480:5"]
    pub const AV_PIX_FMT_GRAY32LE: AVPixelFormat = 251;
    #[c2rust::src_loc = "479:5"]
    pub const AV_PIX_FMT_GRAY32BE: AVPixelFormat = 250;
    #[c2rust::src_loc = "477:5"]
    pub const AV_PIX_FMT_AMF_SURFACE: AVPixelFormat = 249;
    #[c2rust::src_loc = "472:5"]
    pub const AV_PIX_FMT_GRAYF16LE: AVPixelFormat = 248;
    #[c2rust::src_loc = "471:5"]
    pub const AV_PIX_FMT_GRAYF16BE: AVPixelFormat = 247;
    #[c2rust::src_loc = "469:5"]
    pub const AV_PIX_FMT_GBRAPF16LE: AVPixelFormat = 246;
    #[c2rust::src_loc = "468:5"]
    pub const AV_PIX_FMT_GBRAPF16BE: AVPixelFormat = 245;
    #[c2rust::src_loc = "467:5"]
    pub const AV_PIX_FMT_GBRPF16LE: AVPixelFormat = 244;
    #[c2rust::src_loc = "466:5"]
    pub const AV_PIX_FMT_GBRPF16BE: AVPixelFormat = 243;
    #[c2rust::src_loc = "464:5"]
    pub const AV_PIX_FMT_XV48LE: AVPixelFormat = 242;
    #[c2rust::src_loc = "463:5"]
    pub const AV_PIX_FMT_XV48BE: AVPixelFormat = 241;
    #[c2rust::src_loc = "461:5"]
    pub const AV_PIX_FMT_Y216LE: AVPixelFormat = 240;
    #[c2rust::src_loc = "460:5"]
    pub const AV_PIX_FMT_Y216BE: AVPixelFormat = 239;
    #[c2rust::src_loc = "458:5"]
    pub const AV_PIX_FMT_RGB96LE: AVPixelFormat = 238;
    #[c2rust::src_loc = "457:5"]
    pub const AV_PIX_FMT_RGB96BE: AVPixelFormat = 237;
    #[c2rust::src_loc = "455:5"]
    pub const AV_PIX_FMT_RGBA128LE: AVPixelFormat = 236;
    #[c2rust::src_loc = "454:5"]
    pub const AV_PIX_FMT_RGBA128BE: AVPixelFormat = 235;
    #[c2rust::src_loc = "452:5"]
    pub const AV_PIX_FMT_RGBF16LE: AVPixelFormat = 234;
    #[c2rust::src_loc = "451:5"]
    pub const AV_PIX_FMT_RGBF16BE: AVPixelFormat = 233;
    #[c2rust::src_loc = "449:5"]
    pub const AV_PIX_FMT_V30XLE: AVPixelFormat = 232;
    #[c2rust::src_loc = "448:5"]
    pub const AV_PIX_FMT_V30XBE: AVPixelFormat = 231;
    #[c2rust::src_loc = "446:5"]
    pub const AV_PIX_FMT_VYU444: AVPixelFormat = 230;
    #[c2rust::src_loc = "444:5"]
    pub const AV_PIX_FMT_UYVA: AVPixelFormat = 229;
    #[c2rust::src_loc = "442:5"]
    pub const AV_PIX_FMT_AYUV: AVPixelFormat = 228;
    #[c2rust::src_loc = "440:5"]
    pub const AV_PIX_FMT_D3D12: AVPixelFormat = 227;
    #[c2rust::src_loc = "433:5"]
    pub const AV_PIX_FMT_GBRAP14LE: AVPixelFormat = 226;
    #[c2rust::src_loc = "432:5"]
    pub const AV_PIX_FMT_GBRAP14BE: AVPixelFormat = 225;
    #[c2rust::src_loc = "430:5"]
    pub const AV_PIX_FMT_P412LE: AVPixelFormat = 224;
    #[c2rust::src_loc = "429:5"]
    pub const AV_PIX_FMT_P412BE: AVPixelFormat = 223;
    #[c2rust::src_loc = "427:5"]
    pub const AV_PIX_FMT_P212LE: AVPixelFormat = 222;
    #[c2rust::src_loc = "426:5"]
    pub const AV_PIX_FMT_P212BE: AVPixelFormat = 221;
    #[c2rust::src_loc = "424:5"]
    pub const AV_PIX_FMT_RGBAF32LE: AVPixelFormat = 220;
    #[c2rust::src_loc = "423:5"]
    pub const AV_PIX_FMT_RGBAF32BE: AVPixelFormat = 219;
    #[c2rust::src_loc = "421:5"]
    pub const AV_PIX_FMT_RGBF32LE: AVPixelFormat = 218;
    #[c2rust::src_loc = "420:5"]
    pub const AV_PIX_FMT_RGBF32BE: AVPixelFormat = 217;
    #[c2rust::src_loc = "418:5"]
    pub const AV_PIX_FMT_XV36LE: AVPixelFormat = 216;
    #[c2rust::src_loc = "417:5"]
    pub const AV_PIX_FMT_XV36BE: AVPixelFormat = 215;
    #[c2rust::src_loc = "415:5"]
    pub const AV_PIX_FMT_XV30LE: AVPixelFormat = 214;
    #[c2rust::src_loc = "414:5"]
    pub const AV_PIX_FMT_XV30BE: AVPixelFormat = 213;
    #[c2rust::src_loc = "412:5"]
    pub const AV_PIX_FMT_Y212LE: AVPixelFormat = 212;
    #[c2rust::src_loc = "411:5"]
    pub const AV_PIX_FMT_Y212BE: AVPixelFormat = 211;
    #[c2rust::src_loc = "409:5"]
    pub const AV_PIX_FMT_P012BE: AVPixelFormat = 210;
    #[c2rust::src_loc = "408:5"]
    pub const AV_PIX_FMT_P012LE: AVPixelFormat = 209;
    #[c2rust::src_loc = "406:5"]
    pub const AV_PIX_FMT_VUYX: AVPixelFormat = 208;
    #[c2rust::src_loc = "404:5"]
    pub const AV_PIX_FMT_RGBAF16LE: AVPixelFormat = 207;
    #[c2rust::src_loc = "403:5"]
    pub const AV_PIX_FMT_RGBAF16BE: AVPixelFormat = 206;
    #[c2rust::src_loc = "401:5"]
    pub const AV_PIX_FMT_VUYA: AVPixelFormat = 205;
    #[c2rust::src_loc = "399:5"]
    pub const AV_PIX_FMT_P416LE: AVPixelFormat = 204;
    #[c2rust::src_loc = "398:5"]
    pub const AV_PIX_FMT_P416BE: AVPixelFormat = 203;
    #[c2rust::src_loc = "396:5"]
    pub const AV_PIX_FMT_P216LE: AVPixelFormat = 202;
    #[c2rust::src_loc = "395:5"]
    pub const AV_PIX_FMT_P216BE: AVPixelFormat = 201;
    #[c2rust::src_loc = "393:5"]
    pub const AV_PIX_FMT_P410LE: AVPixelFormat = 200;
    #[c2rust::src_loc = "392:5"]
    pub const AV_PIX_FMT_P410BE: AVPixelFormat = 199;
    #[c2rust::src_loc = "390:5"]
    pub const AV_PIX_FMT_P210LE: AVPixelFormat = 198;
    #[c2rust::src_loc = "389:5"]
    pub const AV_PIX_FMT_P210BE: AVPixelFormat = 197;
    #[c2rust::src_loc = "387:5"]
    pub const AV_PIX_FMT_X2BGR10BE: AVPixelFormat = 196;
    #[c2rust::src_loc = "386:5"]
    pub const AV_PIX_FMT_X2BGR10LE: AVPixelFormat = 195;
    #[c2rust::src_loc = "385:5"]
    pub const AV_PIX_FMT_X2RGB10BE: AVPixelFormat = 194;
    #[c2rust::src_loc = "384:5"]
    pub const AV_PIX_FMT_X2RGB10LE: AVPixelFormat = 193;
    #[c2rust::src_loc = "382:5"]
    pub const AV_PIX_FMT_Y210LE: AVPixelFormat = 192;
    #[c2rust::src_loc = "381:5"]
    pub const AV_PIX_FMT_Y210BE: AVPixelFormat = 191;
    #[c2rust::src_loc = "379:5"]
    pub const AV_PIX_FMT_VULKAN: AVPixelFormat = 190;
    #[c2rust::src_loc = "372:5"]
    pub const AV_PIX_FMT_NV42: AVPixelFormat = 189;
    #[c2rust::src_loc = "371:5"]
    pub const AV_PIX_FMT_NV24: AVPixelFormat = 188;
    #[c2rust::src_loc = "369:5"]
    pub const AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 187;
    #[c2rust::src_loc = "368:5"]
    pub const AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 186;
    #[c2rust::src_loc = "367:5"]
    pub const AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 185;
    #[c2rust::src_loc = "366:5"]
    pub const AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 184;
    #[c2rust::src_loc = "364:5"]
    pub const AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 183;
    #[c2rust::src_loc = "363:5"]
    pub const AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 182;
    #[c2rust::src_loc = "361:5"]
    pub const AV_PIX_FMT_GRAY14LE: AVPixelFormat = 181;
    #[c2rust::src_loc = "360:5"]
    pub const AV_PIX_FMT_GRAY14BE: AVPixelFormat = 180;
    #[c2rust::src_loc = "358:5"]
    pub const AV_PIX_FMT_OPENCL: AVPixelFormat = 179;
    #[c2rust::src_loc = "351:5"]
    pub const AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 178;
    #[c2rust::src_loc = "344:5"]
    pub const AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 177;
    #[c2rust::src_loc = "343:5"]
    pub const AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 176;
    #[c2rust::src_loc = "342:5"]
    pub const AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 175;
    #[c2rust::src_loc = "341:5"]
    pub const AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 174;
    #[c2rust::src_loc = "339:5"]
    pub const AV_PIX_FMT_GRAY9LE: AVPixelFormat = 173;
    #[c2rust::src_loc = "338:5"]
    pub const AV_PIX_FMT_GRAY9BE: AVPixelFormat = 172;
    #[c2rust::src_loc = "336:5"]
    pub const AV_PIX_FMT_D3D11: AVPixelFormat = 171;
    #[c2rust::src_loc = "324:5"]
    pub const AV_PIX_FMT_P016BE: AVPixelFormat = 170;
    #[c2rust::src_loc = "323:5"]
    pub const AV_PIX_FMT_P016LE: AVPixelFormat = 169;
    #[c2rust::src_loc = "321:5"]
    pub const AV_PIX_FMT_GRAY10LE: AVPixelFormat = 168;
    #[c2rust::src_loc = "320:5"]
    pub const AV_PIX_FMT_GRAY10BE: AVPixelFormat = 167;
    #[c2rust::src_loc = "319:5"]
    pub const AV_PIX_FMT_GRAY12LE: AVPixelFormat = 166;
    #[c2rust::src_loc = "318:5"]
    pub const AV_PIX_FMT_GRAY12BE: AVPixelFormat = 165;
    #[c2rust::src_loc = "316:5"]
    pub const AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 164;
    #[c2rust::src_loc = "314:5"]
    pub const AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 163;
    #[c2rust::src_loc = "313:5"]
    pub const AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 162;
    #[c2rust::src_loc = "311:5"]
    pub const AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 161;
    #[c2rust::src_loc = "310:5"]
    pub const AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 160;
    #[c2rust::src_loc = "308:5"]
    pub const AV_PIX_FMT_P010BE: AVPixelFormat = 159;
    #[c2rust::src_loc = "307:5"]
    pub const AV_PIX_FMT_P010LE: AVPixelFormat = 158;
    #[c2rust::src_loc = "305:5"]
    pub const AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 157;
    #[c2rust::src_loc = "303:5"]
    pub const AV_PIX_FMT_AYUV64BE: AVPixelFormat = 156;
    #[c2rust::src_loc = "302:5"]
    pub const AV_PIX_FMT_AYUV64LE: AVPixelFormat = 155;
    #[c2rust::src_loc = "301:5"]
    pub const AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 154;
    #[c2rust::src_loc = "300:5"]
    pub const AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 153;
    #[c2rust::src_loc = "299:5"]
    pub const AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 152;
    #[c2rust::src_loc = "298:5"]
    pub const AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 151;
    #[c2rust::src_loc = "296:5"]
    pub const AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 150;
    #[c2rust::src_loc = "295:5"]
    pub const AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 149;
    #[c2rust::src_loc = "294:5"]
    pub const AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 148;
    #[c2rust::src_loc = "293:5"]
    pub const AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 147;
    #[c2rust::src_loc = "292:5"]
    pub const AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 146;
    #[c2rust::src_loc = "291:5"]
    pub const AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 145;
    #[c2rust::src_loc = "290:5"]
    pub const AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 144;
    #[c2rust::src_loc = "289:5"]
    pub const AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 143;
    #[c2rust::src_loc = "288:5"]
    pub const AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 142;
    #[c2rust::src_loc = "287:5"]
    pub const AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 141;
    #[c2rust::src_loc = "286:5"]
    pub const AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 140;
    #[c2rust::src_loc = "285:5"]
    pub const AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 139;
    #[c2rust::src_loc = "283:5"]
    pub const AV_PIX_FMT_YUVJ411P: AVPixelFormat = 138;
    #[c2rust::src_loc = "282:5"]
    pub const AV_PIX_FMT_GBRP14LE: AVPixelFormat = 137;
    #[c2rust::src_loc = "281:5"]
    pub const AV_PIX_FMT_GBRP14BE: AVPixelFormat = 136;
    #[c2rust::src_loc = "280:5"]
    pub const AV_PIX_FMT_GBRP12LE: AVPixelFormat = 135;
    #[c2rust::src_loc = "279:5"]
    pub const AV_PIX_FMT_GBRP12BE: AVPixelFormat = 134;
    #[c2rust::src_loc = "278:5"]
    pub const AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 133;
    #[c2rust::src_loc = "277:5"]
    pub const AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 132;
    #[c2rust::src_loc = "276:5"]
    pub const AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 131;
    #[c2rust::src_loc = "275:5"]
    pub const AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 130;
    #[c2rust::src_loc = "274:5"]
    pub const AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 129;
    #[c2rust::src_loc = "273:5"]
    pub const AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 128;
    #[c2rust::src_loc = "272:5"]
    pub const AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 127;
    #[c2rust::src_loc = "271:5"]
    pub const AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 126;
    #[c2rust::src_loc = "270:5"]
    pub const AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 125;
    #[c2rust::src_loc = "269:5"]
    pub const AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 124;
    #[c2rust::src_loc = "268:5"]
    pub const AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 123;
    #[c2rust::src_loc = "267:5"]
    pub const AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 122;
    #[c2rust::src_loc = "265:5"]
    pub const AV_PIX_FMT_BGR0: AVPixelFormat = 121;
    #[c2rust::src_loc = "264:5"]
    pub const AV_PIX_FMT_0BGR: AVPixelFormat = 120;
    #[c2rust::src_loc = "263:5"]
    pub const AV_PIX_FMT_RGB0: AVPixelFormat = 119;
    #[c2rust::src_loc = "262:5"]
    pub const AV_PIX_FMT_0RGB: AVPixelFormat = 118;
    #[c2rust::src_loc = "260:5"]
    pub const AV_PIX_FMT_CUDA: AVPixelFormat = 117;
    #[c2rust::src_loc = "254:5"]
    pub const AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 116;
    #[c2rust::src_loc = "252:5"]
    pub const AV_PIX_FMT_MMAL: AVPixelFormat = 115;
    #[c2rust::src_loc = "247:5"]
    pub const AV_PIX_FMT_QSV: AVPixelFormat = 114;
    #[c2rust::src_loc = "214:5"]
    pub const AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 113;
    #[c2rust::src_loc = "213:5"]
    pub const AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 112;
    #[c2rust::src_loc = "212:5"]
    pub const AV_PIX_FMT_GBRAP: AVPixelFormat = 111;
    #[c2rust::src_loc = "210:5"]
    pub const AV_PIX_FMT_YA16LE: AVPixelFormat = 110;
    #[c2rust::src_loc = "209:5"]
    pub const AV_PIX_FMT_YA16BE: AVPixelFormat = 109;
    #[c2rust::src_loc = "207:5"]
    pub const AV_PIX_FMT_YVYU422: AVPixelFormat = 108;
    #[c2rust::src_loc = "205:5"]
    pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 107;
    #[c2rust::src_loc = "204:5"]
    pub const AV_PIX_FMT_BGRA64BE: AVPixelFormat = 106;
    #[c2rust::src_loc = "203:5"]
    pub const AV_PIX_FMT_RGBA64LE: AVPixelFormat = 105;
    #[c2rust::src_loc = "202:5"]
    pub const AV_PIX_FMT_RGBA64BE: AVPixelFormat = 104;
    #[c2rust::src_loc = "200:5"]
    pub const AV_PIX_FMT_NV20BE: AVPixelFormat = 103;
    #[c2rust::src_loc = "199:5"]
    pub const AV_PIX_FMT_NV20LE: AVPixelFormat = 102;
    #[c2rust::src_loc = "198:5"]
    pub const AV_PIX_FMT_NV16: AVPixelFormat = 101;
    #[c2rust::src_loc = "197:5"]
    pub const AV_PIX_FMT_XYZ12BE: AVPixelFormat = 100;
    #[c2rust::src_loc = "196:5"]
    pub const AV_PIX_FMT_XYZ12LE: AVPixelFormat = 99;
    #[c2rust::src_loc = "194:5"]
    pub const AV_PIX_FMT_VDPAU: AVPixelFormat = 98;
    #[c2rust::src_loc = "192:5"]
    pub const AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 97;
    #[c2rust::src_loc = "191:5"]
    pub const AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 96;
    #[c2rust::src_loc = "190:5"]
    pub const AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 95;
    #[c2rust::src_loc = "189:5"]
    pub const AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 94;
    #[c2rust::src_loc = "188:5"]
    pub const AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 93;
    #[c2rust::src_loc = "187:5"]
    pub const AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 92;
    #[c2rust::src_loc = "186:5"]
    pub const AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 91;
    #[c2rust::src_loc = "185:5"]
    pub const AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 90;
    #[c2rust::src_loc = "184:5"]
    pub const AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 89;
    #[c2rust::src_loc = "183:5"]
    pub const AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 88;
    #[c2rust::src_loc = "182:5"]
    pub const AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 87;
    #[c2rust::src_loc = "181:5"]
    pub const AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 86;
    #[c2rust::src_loc = "180:5"]
    pub const AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 85;
    #[c2rust::src_loc = "179:5"]
    pub const AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 84;
    #[c2rust::src_loc = "178:5"]
    pub const AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 83;
    #[c2rust::src_loc = "177:5"]
    pub const AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 82;
    #[c2rust::src_loc = "176:5"]
    pub const AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 81;
    #[c2rust::src_loc = "175:5"]
    pub const AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 80;
    #[c2rust::src_loc = "174:5"]
    pub const AV_PIX_FMT_YUVA444P: AVPixelFormat = 79;
    #[c2rust::src_loc = "173:5"]
    pub const AV_PIX_FMT_YUVA422P: AVPixelFormat = 78;
    #[c2rust::src_loc = "172:5"]
    pub const AV_PIX_FMT_GBRP16LE: AVPixelFormat = 77;
    #[c2rust::src_loc = "171:5"]
    pub const AV_PIX_FMT_GBRP16BE: AVPixelFormat = 76;
    #[c2rust::src_loc = "170:5"]
    pub const AV_PIX_FMT_GBRP10LE: AVPixelFormat = 75;
    #[c2rust::src_loc = "169:5"]
    pub const AV_PIX_FMT_GBRP10BE: AVPixelFormat = 74;
    #[c2rust::src_loc = "168:5"]
    pub const AV_PIX_FMT_GBRP9LE: AVPixelFormat = 73;
    #[c2rust::src_loc = "167:5"]
    pub const AV_PIX_FMT_GBRP9BE: AVPixelFormat = 72;
    #[c2rust::src_loc = "166:5"]
    pub const AV_PIX_FMT_GBR24P: AVPixelFormat = 71;
    #[c2rust::src_loc = "165:5"]
    pub const AV_PIX_FMT_GBRP: AVPixelFormat = 71;
    #[c2rust::src_loc = "164:5"]
    pub const AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 70;
    #[c2rust::src_loc = "163:5"]
    pub const AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 69;
    #[c2rust::src_loc = "162:5"]
    pub const AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 68;
    #[c2rust::src_loc = "161:5"]
    pub const AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 67;
    #[c2rust::src_loc = "160:5"]
    pub const AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 66;
    #[c2rust::src_loc = "159:5"]
    pub const AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 65;
    #[c2rust::src_loc = "158:5"]
    pub const AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 64;
    #[c2rust::src_loc = "157:5"]
    pub const AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 63;
    #[c2rust::src_loc = "156:5"]
    pub const AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 62;
    #[c2rust::src_loc = "155:5"]
    pub const AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 61;
    #[c2rust::src_loc = "154:5"]
    pub const AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 60;
    #[c2rust::src_loc = "153:5"]
    pub const AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 59;
    #[c2rust::src_loc = "146:5"]
    pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 58;
    #[c2rust::src_loc = "145:5"]
    pub const AV_PIX_FMT_BGR48BE: AVPixelFormat = 57;
    #[c2rust::src_loc = "143:5"]
    pub const AV_PIX_FMT_GRAY8A: AVPixelFormat = 56;
    #[c2rust::src_loc = "142:5"]
    pub const AV_PIX_FMT_Y400A: AVPixelFormat = 56;
    #[c2rust::src_loc = "140:5"]
    pub const AV_PIX_FMT_YA8: AVPixelFormat = 56;
    #[c2rust::src_loc = "139:5"]
    pub const AV_PIX_FMT_BGR444BE: AVPixelFormat = 55;
    #[c2rust::src_loc = "138:5"]
    pub const AV_PIX_FMT_BGR444LE: AVPixelFormat = 54;
    #[c2rust::src_loc = "137:5"]
    pub const AV_PIX_FMT_RGB444BE: AVPixelFormat = 53;
    #[c2rust::src_loc = "136:5"]
    pub const AV_PIX_FMT_RGB444LE: AVPixelFormat = 52;
    #[c2rust::src_loc = "134:5"]
    pub const AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 51;
    #[c2rust::src_loc = "133:5"]
    pub const AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 50;
    #[c2rust::src_loc = "132:5"]
    pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 49;
    #[c2rust::src_loc = "131:5"]
    pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 48;
    #[c2rust::src_loc = "130:5"]
    pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 47;
    #[c2rust::src_loc = "129:5"]
    pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 46;
    #[c2rust::src_loc = "128:5"]
    pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 45;
    #[c2rust::src_loc = "126:5"]
    pub const AV_PIX_FMT_VAAPI: AVPixelFormat = 44;
    #[c2rust::src_loc = "120:5"]
    pub const AV_PIX_FMT_BGR555LE: AVPixelFormat = 43;
    #[c2rust::src_loc = "119:5"]
    pub const AV_PIX_FMT_BGR555BE: AVPixelFormat = 42;
    #[c2rust::src_loc = "118:5"]
    pub const AV_PIX_FMT_BGR565LE: AVPixelFormat = 41;
    #[c2rust::src_loc = "117:5"]
    pub const AV_PIX_FMT_BGR565BE: AVPixelFormat = 40;
    #[c2rust::src_loc = "115:5"]
    pub const AV_PIX_FMT_RGB555LE: AVPixelFormat = 39;
    #[c2rust::src_loc = "114:5"]
    pub const AV_PIX_FMT_RGB555BE: AVPixelFormat = 38;
    #[c2rust::src_loc = "113:5"]
    pub const AV_PIX_FMT_RGB565LE: AVPixelFormat = 37;
    #[c2rust::src_loc = "112:5"]
    pub const AV_PIX_FMT_RGB565BE: AVPixelFormat = 36;
    #[c2rust::src_loc = "110:5"]
    pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
    #[c2rust::src_loc = "109:5"]
    pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
    #[c2rust::src_loc = "108:5"]
    pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
    #[c2rust::src_loc = "107:5"]
    pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
    #[c2rust::src_loc = "106:5"]
    pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
    #[c2rust::src_loc = "105:5"]
    pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
    #[c2rust::src_loc = "104:5"]
    pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
    #[c2rust::src_loc = "102:5"]
    pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28;
    #[c2rust::src_loc = "101:5"]
    pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27;
    #[c2rust::src_loc = "100:5"]
    pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26;
    #[c2rust::src_loc = "99:5"]
    pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25;
    #[c2rust::src_loc = "97:5"]
    pub const AV_PIX_FMT_NV21: AVPixelFormat = 24;
    #[c2rust::src_loc = "96:5"]
    pub const AV_PIX_FMT_NV12: AVPixelFormat = 23;
    #[c2rust::src_loc = "95:5"]
    pub const AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22;
    #[c2rust::src_loc = "94:5"]
    pub const AV_PIX_FMT_RGB4: AVPixelFormat = 21;
    #[c2rust::src_loc = "93:5"]
    pub const AV_PIX_FMT_RGB8: AVPixelFormat = 20;
    #[c2rust::src_loc = "92:5"]
    pub const AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19;
    #[c2rust::src_loc = "91:5"]
    pub const AV_PIX_FMT_BGR4: AVPixelFormat = 18;
    #[c2rust::src_loc = "90:5"]
    pub const AV_PIX_FMT_BGR8: AVPixelFormat = 17;
    #[c2rust::src_loc = "89:5"]
    pub const AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16;
    #[c2rust::src_loc = "88:5"]
    pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
    #[c2rust::src_loc = "87:5"]
    pub const AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14;
    #[c2rust::src_loc = "86:5"]
    pub const AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13;
    #[c2rust::src_loc = "85:5"]
    pub const AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12;
    #[c2rust::src_loc = "84:5"]
    pub const AV_PIX_FMT_PAL8: AVPixelFormat = 11;
    #[c2rust::src_loc = "83:5"]
    pub const AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10;
    #[c2rust::src_loc = "82:5"]
    pub const AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9;
    #[c2rust::src_loc = "81:5"]
    pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
    #[c2rust::src_loc = "79:5"]
    pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
    #[c2rust::src_loc = "78:5"]
    pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
    #[c2rust::src_loc = "77:5"]
    pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
    #[c2rust::src_loc = "76:5"]
    pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3;
    #[c2rust::src_loc = "75:5"]
    pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2;
    #[c2rust::src_loc = "74:5"]
    pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
    #[c2rust::src_loc = "73:5"]
    pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
    #[c2rust::src_loc = "72:5"]
    pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "151:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "187:1"]
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "239:1"]
        pub fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "279:1"]
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        #[c2rust::src_loc = "873:1"]
        pub fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:27"]
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "230:1"]
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "675:1"]
        pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/usr/include/strings.h:27"]
pub mod strings_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "116:1"]
        pub fn strcasecmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "120:1"]
        pub fn strncasecmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:27"]
pub mod osdep_h {
    #[inline]
    #[c2rust::src_loc = "270:1"]
    pub unsafe extern "C" fn x264_is_regular_file(
        mut filehandle: *mut FILE,
    ) -> ::core::ffi::c_int {
        let mut file_stat: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
            __glibc_reserved: [0; 3],
        };
        if fstat(fileno(filehandle), &mut file_stat) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return (file_stat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
            as ::core::ffi::c_int;
    }
    use super::FILE_h::FILE;
    use super::struct_stat_h::stat;
    use super::types_h::{
        __dev_t, __ino_t, __nlink_t, __mode_t, __uid_t, __gid_t, __off_t, __blksize_t,
        __blkcnt_t, __syscall_slong_t, __time_t,
    };
    use super::struct_timespec_h::timespec;
    use super::stat_h::fstat;
    use super::stdio_h::fileno;
    use super::bits_stat_h::__S_IFMT;
}
#[c2rust::header_src = "/usr/include/string.h:27"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "61:1"]
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:27"]
pub mod x264_h {
    #[c2rust::src_loc = "255:9"]
    pub const X264_CSP_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "256:9"]
    pub const X264_CSP_I400: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "257:9"]
    pub const X264_CSP_I420: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "261:9"]
    pub const X264_CSP_I422: ::core::ffi::c_int = 0x6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "264:9"]
    pub const X264_CSP_YUYV: ::core::ffi::c_int = 0x9 as ::core::ffi::c_int;
    #[c2rust::src_loc = "267:9"]
    pub const X264_CSP_I444: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;
    #[c2rust::src_loc = "269:9"]
    pub const X264_CSP_BGR: ::core::ffi::c_int = 0xe as ::core::ffi::c_int;
    #[c2rust::src_loc = "270:9"]
    pub const X264_CSP_BGRA: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;
    #[c2rust::src_loc = "273:9"]
    pub const X264_CSP_VFLIP: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "274:9"]
    pub const X264_CSP_HIGH_DEPTH: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "290:9"]
    pub const X264_LOG_WARNING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "291:9"]
    pub const X264_LOG_INFO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "292:9"]
    pub const X264_LOG_DEBUG: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/dlfcn.h:27"]
pub mod dlfcn_h {
    extern "C" {
        #[c2rust::src_loc = "56:1"]
        pub fn dlopen(
            __file: *const ::core::ffi::c_char,
            __mode: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "60:1"]
        pub fn dlclose(__handle: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "64:1"]
        pub fn dlsym(
            __handle: *mut ::core::ffi::c_void,
            __name: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/usr/include/bits/dlfcn.h:27"]
pub mod bits_dlfcn_h {
    #[c2rust::src_loc = "25:9"]
    pub const RTLD_NOW: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:27"]
pub mod bits_stat_h {
    #[c2rust::src_loc = "29:9"]
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:27"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{
    __uint8_t, __uint32_t, __int64_t, __uint64_t, __dev_t, __uid_t, __gid_t, __ino_t,
    __mode_t, __nlink_t, __off_t, __off64_t, __time_t, __blksize_t, __blkcnt_t,
    __syscall_slong_t,
};
pub use self::struct_FILE_h::{
    _IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt, _IO_marker,
};
pub use self::FILE_h::FILE;
pub use self::struct_timespec_h::timespec;
pub use self::struct_stat_h::stat;
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::x264cli_h::{hnd_t, get_filename_extension, x264_cli_log, x264_cli_printf};
pub use self::input_h::{
    cli_input_opt_t, video_info_t, cli_image_t, cli_pic_t, cli_input_t, x264_cli_csp_t,
    X264_CSP_OTHER, x264_cli_pic_alloc, x264_cli_get_csp,
};
pub use self::avisynth_c_h::{
    avs_is_y_func, AVS_VideoInfo, avs_is_420_func, avs_is_422_func, avs_is_444_func,
    avs_is_y16_func, avs_is_yuv420p16_func, avs_is_yuv422p16_func, avs_is_yuv444p16_func,
    avs_is_rgb64_func, avs_is_rgb48_func, avs_get_read_ptr_p_func, AVS_VideoFrame,
    AVS_VideoFrameBuffer, BYTE, avs_get_pitch_p_func, avs_is_y8_func, avs_is_yv411_func,
    avs_is_yv12_func, avs_is_yv16_func, avs_is_yv24_func, avs_take_clip_func, AVS_Value,
    C2RustUnnamed_0, avs_release_video_frame_func, avs_release_value_func,
    avs_release_clip_func, avs_invoke_func, avs_function_exists_func,
    avs_get_video_info_func, avs_get_frame_func, avs_get_error_func,
    avs_delete_script_environment_func, avs_create_script_environment_func,
    avs_clip_get_error_func, AVS_PLANAR_V, AVS_PLANAR_U, AVS_PLANAR_Y, AVS_PLANAR_A,
    AVS_PLANAR_R, AVS_PLANAR_B, AVS_CS_PLANAR_FILTER, AVS_CS_YV411, AVS_CS_PLANAR_MASK,
    AVS_CS_YUY2, AVS_CS_Y8, AVS_CS_YV12, AVS_CS_YV16, AVS_CS_YV24, AVS_CS_SAMPLE_BITS_8,
    AVS_CS_SAMPLE_BITS_MASK, AVS_CS_BGR24, AVS_CS_BGR32, AVS_IT_TFF, AVS_IT_FIELDBASED,
    C2RustUnnamed_1, AVS_PLANAR_B_ALIGNED, AVS_PLANAR_G_ALIGNED, AVS_PLANAR_R_ALIGNED,
    AVS_PLANAR_A_ALIGNED, AVS_PLANAR_G, AVS_PLANAR_V_ALIGNED, AVS_PLANAR_U_ALIGNED,
    AVS_PLANAR_Y_ALIGNED, AVS_PLANAR_ALIGNED, C2RustUnnamed_2, AVS_CS_GENERIC_YUVA444,
    AVS_CS_GENERIC_YUVA422, AVS_CS_GENERIC_YUVA420, AVS_CS_GENERIC_RGBAP,
    AVS_CS_GENERIC_RGBP, AVS_CS_GENERIC_Y, AVS_CS_GENERIC_YUV444, AVS_CS_GENERIC_YUV422,
    AVS_CS_GENERIC_YUV420, AVS_CS_RGBA_TYPE, AVS_CS_RGB_TYPE, AVS_CS_SAMPLE_BITS_32,
    AVS_CS_SAMPLE_BITS_16, AVS_CS_SAMPLE_BITS_14, AVS_CS_SAMPLE_BITS_12,
    AVS_CS_SAMPLE_BITS_10, AVS_CS_SUB_HEIGHT_4, AVS_CS_SUB_HEIGHT_2, AVS_CS_SUB_HEIGHT_1,
    AVS_CS_SUB_HEIGHT_MASK, AVS_CS_UPLANEFIRST, AVS_CS_VPLANEFIRST, AVS_CS_SUB_WIDTH_4,
    AVS_CS_SUB_WIDTH_2, AVS_CS_SUB_WIDTH_1, AVS_CS_SUB_WIDTH_MASK,
    AVS_CS_SHIFT_SAMPLE_BITS, AVS_CS_SHIFT_SUB_HEIGHT, AVS_CS_SHIFT_SUB_WIDTH,
    AVS_CS_PLANAR, AVS_CS_INTERLEAVED, AVS_CS_YUV, AVS_CS_BGR, AVS_CS_YUVA,
    C2RustUnnamed_3, AVS_CS_YUVA420PS, AVS_CS_YUVA422PS, AVS_CS_YUVA444PS,
    AVS_CS_YUVA420P16, AVS_CS_YUVA422P16, AVS_CS_YUVA444P16, AVS_CS_YUVA420P14,
    AVS_CS_YUVA422P14, AVS_CS_YUVA444P14, AVS_CS_YUVA420P12, AVS_CS_YUVA422P12,
    AVS_CS_YUVA444P12, AVS_CS_YUVA420P10, AVS_CS_YUVA422P10, AVS_CS_YUVA444P10,
    AVS_CS_YUVA420, AVS_CS_YUVA422, AVS_CS_YUVA444, AVS_CS_RGBAPS, AVS_CS_RGBAP16,
    AVS_CS_RGBAP14, AVS_CS_RGBAP12, AVS_CS_RGBAP10, AVS_CS_RGBAP, AVS_CS_RGBPS,
    AVS_CS_RGBP16, AVS_CS_RGBP14, AVS_CS_RGBP12, AVS_CS_RGBP10, AVS_CS_RGBP,
    AVS_CS_BGR64, AVS_CS_BGR48, AVS_CS_Y32, AVS_CS_YUV420PS, AVS_CS_YUV422PS,
    AVS_CS_YUV444PS, AVS_CS_Y16, AVS_CS_YUV420P16, AVS_CS_YUV422P16, AVS_CS_YUV444P16,
    AVS_CS_Y14, AVS_CS_YUV420P14, AVS_CS_YUV422P14, AVS_CS_YUV444P14, AVS_CS_Y12,
    AVS_CS_YUV420P12, AVS_CS_YUV422P12, AVS_CS_YUV444P12, AVS_CS_Y10, AVS_CS_YUV420P10,
    AVS_CS_YUV422P10, AVS_CS_YUV444P10, AVS_CS_YUV9, AVS_CS_IYUV, AVS_CS_I420,
    AVS_CS_RAW32, AVS_CS_UNKNOWN, C2RustUnnamed_4, AVS_IT_BFF, avs_has_video,
    avs_is_rgb24, avs_is_rgb32, avs_is_yuy2, avs_is_yv24, avs_is_yv16, avs_is_yv12,
    avs_is_yv411, avs_is_y8, avs_is_field_based, avs_is_tff, avs_get_pitch_p,
    avs_get_read_ptr_p, avs_is_clip, avs_is_int, avs_is_float, avs_is_string,
    avs_is_error, avs_as_int, avs_as_string, avs_as_float, avs_as_error,
    avs_new_value_bool, avs_new_value_string, avs_new_value_array, AVS_ScriptEnvironment,
    AVS_Clip,
};
pub use self::pixfmt_h::{
    AV_PIX_FMT_YUV411P, AVPixelFormat, AV_PIX_FMT_NB, AV_PIX_FMT_OHCODEC,
    AV_PIX_FMT_GBRP12MSBLE, AV_PIX_FMT_GBRP12MSBBE, AV_PIX_FMT_GBRP10MSBLE,
    AV_PIX_FMT_GBRP10MSBBE, AV_PIX_FMT_YUV444P12MSBLE, AV_PIX_FMT_YUV444P12MSBBE,
    AV_PIX_FMT_YUV444P10MSBLE, AV_PIX_FMT_YUV444P10MSBBE, AV_PIX_FMT_GBRAP32LE,
    AV_PIX_FMT_GBRAP32BE, AV_PIX_FMT_YAF16LE, AV_PIX_FMT_YAF16BE, AV_PIX_FMT_YAF32LE,
    AV_PIX_FMT_YAF32BE, AV_PIX_FMT_GRAY32LE, AV_PIX_FMT_GRAY32BE, AV_PIX_FMT_AMF_SURFACE,
    AV_PIX_FMT_GRAYF16LE, AV_PIX_FMT_GRAYF16BE, AV_PIX_FMT_GBRAPF16LE,
    AV_PIX_FMT_GBRAPF16BE, AV_PIX_FMT_GBRPF16LE, AV_PIX_FMT_GBRPF16BE, AV_PIX_FMT_XV48LE,
    AV_PIX_FMT_XV48BE, AV_PIX_FMT_Y216LE, AV_PIX_FMT_Y216BE, AV_PIX_FMT_RGB96LE,
    AV_PIX_FMT_RGB96BE, AV_PIX_FMT_RGBA128LE, AV_PIX_FMT_RGBA128BE, AV_PIX_FMT_RGBF16LE,
    AV_PIX_FMT_RGBF16BE, AV_PIX_FMT_V30XLE, AV_PIX_FMT_V30XBE, AV_PIX_FMT_VYU444,
    AV_PIX_FMT_UYVA, AV_PIX_FMT_AYUV, AV_PIX_FMT_D3D12, AV_PIX_FMT_GBRAP14LE,
    AV_PIX_FMT_GBRAP14BE, AV_PIX_FMT_P412LE, AV_PIX_FMT_P412BE, AV_PIX_FMT_P212LE,
    AV_PIX_FMT_P212BE, AV_PIX_FMT_RGBAF32LE, AV_PIX_FMT_RGBAF32BE, AV_PIX_FMT_RGBF32LE,
    AV_PIX_FMT_RGBF32BE, AV_PIX_FMT_XV36LE, AV_PIX_FMT_XV36BE, AV_PIX_FMT_XV30LE,
    AV_PIX_FMT_XV30BE, AV_PIX_FMT_Y212LE, AV_PIX_FMT_Y212BE, AV_PIX_FMT_P012BE,
    AV_PIX_FMT_P012LE, AV_PIX_FMT_VUYX, AV_PIX_FMT_RGBAF16LE, AV_PIX_FMT_RGBAF16BE,
    AV_PIX_FMT_VUYA, AV_PIX_FMT_P416LE, AV_PIX_FMT_P416BE, AV_PIX_FMT_P216LE,
    AV_PIX_FMT_P216BE, AV_PIX_FMT_P410LE, AV_PIX_FMT_P410BE, AV_PIX_FMT_P210LE,
    AV_PIX_FMT_P210BE, AV_PIX_FMT_X2BGR10BE, AV_PIX_FMT_X2BGR10LE, AV_PIX_FMT_X2RGB10BE,
    AV_PIX_FMT_X2RGB10LE, AV_PIX_FMT_Y210LE, AV_PIX_FMT_Y210BE, AV_PIX_FMT_VULKAN,
    AV_PIX_FMT_NV42, AV_PIX_FMT_NV24, AV_PIX_FMT_YUVA444P12LE, AV_PIX_FMT_YUVA444P12BE,
    AV_PIX_FMT_YUVA422P12LE, AV_PIX_FMT_YUVA422P12BE, AV_PIX_FMT_GRAYF32LE,
    AV_PIX_FMT_GRAYF32BE, AV_PIX_FMT_GRAY14LE, AV_PIX_FMT_GRAY14BE, AV_PIX_FMT_OPENCL,
    AV_PIX_FMT_DRM_PRIME, AV_PIX_FMT_GBRAPF32LE, AV_PIX_FMT_GBRAPF32BE,
    AV_PIX_FMT_GBRPF32LE, AV_PIX_FMT_GBRPF32BE, AV_PIX_FMT_GRAY9LE, AV_PIX_FMT_GRAY9BE,
    AV_PIX_FMT_D3D11, AV_PIX_FMT_P016BE, AV_PIX_FMT_P016LE, AV_PIX_FMT_GRAY10LE,
    AV_PIX_FMT_GRAY10BE, AV_PIX_FMT_GRAY12LE, AV_PIX_FMT_GRAY12BE, AV_PIX_FMT_MEDIACODEC,
    AV_PIX_FMT_GBRAP10LE, AV_PIX_FMT_GBRAP10BE, AV_PIX_FMT_GBRAP12LE,
    AV_PIX_FMT_GBRAP12BE, AV_PIX_FMT_P010BE, AV_PIX_FMT_P010LE, AV_PIX_FMT_VIDEOTOOLBOX,
    AV_PIX_FMT_AYUV64BE, AV_PIX_FMT_AYUV64LE, AV_PIX_FMT_YUV440P12BE,
    AV_PIX_FMT_YUV440P12LE, AV_PIX_FMT_YUV440P10BE, AV_PIX_FMT_YUV440P10LE,
    AV_PIX_FMT_BAYER_GRBG16BE, AV_PIX_FMT_BAYER_GRBG16LE, AV_PIX_FMT_BAYER_GBRG16BE,
    AV_PIX_FMT_BAYER_GBRG16LE, AV_PIX_FMT_BAYER_RGGB16BE, AV_PIX_FMT_BAYER_RGGB16LE,
    AV_PIX_FMT_BAYER_BGGR16BE, AV_PIX_FMT_BAYER_BGGR16LE, AV_PIX_FMT_BAYER_GRBG8,
    AV_PIX_FMT_BAYER_GBRG8, AV_PIX_FMT_BAYER_RGGB8, AV_PIX_FMT_BAYER_BGGR8,
    AV_PIX_FMT_YUVJ411P, AV_PIX_FMT_GBRP14LE, AV_PIX_FMT_GBRP14BE, AV_PIX_FMT_GBRP12LE,
    AV_PIX_FMT_GBRP12BE, AV_PIX_FMT_YUV444P14LE, AV_PIX_FMT_YUV444P14BE,
    AV_PIX_FMT_YUV444P12LE, AV_PIX_FMT_YUV444P12BE, AV_PIX_FMT_YUV422P14LE,
    AV_PIX_FMT_YUV422P14BE, AV_PIX_FMT_YUV422P12LE, AV_PIX_FMT_YUV422P12BE,
    AV_PIX_FMT_YUV420P14LE, AV_PIX_FMT_YUV420P14BE, AV_PIX_FMT_YUV420P12LE,
    AV_PIX_FMT_YUV420P12BE, AV_PIX_FMT_BGR0, AV_PIX_FMT_0BGR, AV_PIX_FMT_RGB0,
    AV_PIX_FMT_0RGB, AV_PIX_FMT_CUDA, AV_PIX_FMT_D3D11VA_VLD, AV_PIX_FMT_MMAL,
    AV_PIX_FMT_QSV, AV_PIX_FMT_GBRAP16LE, AV_PIX_FMT_GBRAP16BE, AV_PIX_FMT_GBRAP,
    AV_PIX_FMT_YA16LE, AV_PIX_FMT_YA16BE, AV_PIX_FMT_YVYU422, AV_PIX_FMT_BGRA64LE,
    AV_PIX_FMT_BGRA64BE, AV_PIX_FMT_RGBA64LE, AV_PIX_FMT_RGBA64BE, AV_PIX_FMT_NV20BE,
    AV_PIX_FMT_NV20LE, AV_PIX_FMT_NV16, AV_PIX_FMT_XYZ12BE, AV_PIX_FMT_XYZ12LE,
    AV_PIX_FMT_VDPAU, AV_PIX_FMT_YUVA444P16LE, AV_PIX_FMT_YUVA444P16BE,
    AV_PIX_FMT_YUVA422P16LE, AV_PIX_FMT_YUVA422P16BE, AV_PIX_FMT_YUVA420P16LE,
    AV_PIX_FMT_YUVA420P16BE, AV_PIX_FMT_YUVA444P10LE, AV_PIX_FMT_YUVA444P10BE,
    AV_PIX_FMT_YUVA422P10LE, AV_PIX_FMT_YUVA422P10BE, AV_PIX_FMT_YUVA420P10LE,
    AV_PIX_FMT_YUVA420P10BE, AV_PIX_FMT_YUVA444P9LE, AV_PIX_FMT_YUVA444P9BE,
    AV_PIX_FMT_YUVA422P9LE, AV_PIX_FMT_YUVA422P9BE, AV_PIX_FMT_YUVA420P9LE,
    AV_PIX_FMT_YUVA420P9BE, AV_PIX_FMT_YUVA444P, AV_PIX_FMT_YUVA422P,
    AV_PIX_FMT_GBRP16LE, AV_PIX_FMT_GBRP16BE, AV_PIX_FMT_GBRP10LE, AV_PIX_FMT_GBRP10BE,
    AV_PIX_FMT_GBRP9LE, AV_PIX_FMT_GBRP9BE, AV_PIX_FMT_GBR24P, AV_PIX_FMT_GBRP,
    AV_PIX_FMT_YUV422P9LE, AV_PIX_FMT_YUV422P9BE, AV_PIX_FMT_YUV444P10LE,
    AV_PIX_FMT_YUV444P10BE, AV_PIX_FMT_YUV444P9LE, AV_PIX_FMT_YUV444P9BE,
    AV_PIX_FMT_YUV422P10LE, AV_PIX_FMT_YUV422P10BE, AV_PIX_FMT_YUV420P10LE,
    AV_PIX_FMT_YUV420P10BE, AV_PIX_FMT_YUV420P9LE, AV_PIX_FMT_YUV420P9BE,
    AV_PIX_FMT_BGR48LE, AV_PIX_FMT_BGR48BE, AV_PIX_FMT_GRAY8A, AV_PIX_FMT_Y400A,
    AV_PIX_FMT_YA8, AV_PIX_FMT_BGR444BE, AV_PIX_FMT_BGR444LE, AV_PIX_FMT_RGB444BE,
    AV_PIX_FMT_RGB444LE, AV_PIX_FMT_DXVA2_VLD, AV_PIX_FMT_YUV444P16BE,
    AV_PIX_FMT_YUV444P16LE, AV_PIX_FMT_YUV422P16BE, AV_PIX_FMT_YUV422P16LE,
    AV_PIX_FMT_YUV420P16BE, AV_PIX_FMT_YUV420P16LE, AV_PIX_FMT_VAAPI,
    AV_PIX_FMT_BGR555LE, AV_PIX_FMT_BGR555BE, AV_PIX_FMT_BGR565LE, AV_PIX_FMT_BGR565BE,
    AV_PIX_FMT_RGB555LE, AV_PIX_FMT_RGB555BE, AV_PIX_FMT_RGB565LE, AV_PIX_FMT_RGB565BE,
    AV_PIX_FMT_RGB48LE, AV_PIX_FMT_RGB48BE, AV_PIX_FMT_YUVA420P, AV_PIX_FMT_YUVJ440P,
    AV_PIX_FMT_YUV440P, AV_PIX_FMT_GRAY16LE, AV_PIX_FMT_GRAY16BE, AV_PIX_FMT_BGRA,
    AV_PIX_FMT_ABGR, AV_PIX_FMT_RGBA, AV_PIX_FMT_ARGB, AV_PIX_FMT_NV21, AV_PIX_FMT_NV12,
    AV_PIX_FMT_RGB4_BYTE, AV_PIX_FMT_RGB4, AV_PIX_FMT_RGB8, AV_PIX_FMT_BGR4_BYTE,
    AV_PIX_FMT_BGR4, AV_PIX_FMT_BGR8, AV_PIX_FMT_UYYVYY411, AV_PIX_FMT_UYVY422,
    AV_PIX_FMT_YUVJ444P, AV_PIX_FMT_YUVJ422P, AV_PIX_FMT_YUVJ420P, AV_PIX_FMT_PAL8,
    AV_PIX_FMT_MONOBLACK, AV_PIX_FMT_MONOWHITE, AV_PIX_FMT_GRAY8, AV_PIX_FMT_YUV410P,
    AV_PIX_FMT_YUV444P, AV_PIX_FMT_YUV422P, AV_PIX_FMT_BGR24, AV_PIX_FMT_RGB24,
    AV_PIX_FMT_YUYV422, AV_PIX_FMT_YUV420P, AV_PIX_FMT_NONE,
};
use self::stdio_h::{stderr, fclose, fflush, fopen, fileno};
use self::stat_h::fstat;
use self::stdlib_h::{calloc, free};
use self::strings_h::{strcasecmp, strncasecmp};
pub use self::osdep_h::x264_is_regular_file;
use self::string_h::{memset, strlen};
pub use self::x264_h::{
    X264_CSP_NONE, X264_CSP_I400, X264_CSP_I420, X264_CSP_I422, X264_CSP_YUYV,
    X264_CSP_I444, X264_CSP_BGR, X264_CSP_BGRA, X264_CSP_VFLIP, X264_CSP_HIGH_DEPTH,
    X264_LOG_ERROR, X264_LOG_WARNING, X264_LOG_INFO, X264_LOG_DEBUG,
};
use self::dlfcn_h::{dlopen, dlclose, dlsym};
pub use self::bits_dlfcn_h::RTLD_NOW;
pub use self::bits_stat_h::__S_IFMT;
pub use self::__stddef_null_h::NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "78:9"]
pub struct avs_hnd_t {
    pub clip: *mut AVS_Clip,
    pub env: *mut AVS_ScriptEnvironment,
    pub library: *mut ::core::ffi::c_void,
    pub num_frames: ::core::ffi::c_int,
    pub func: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "84:5"]
pub struct C2RustUnnamed {
    pub avs_clip_get_error: avs_clip_get_error_func,
    pub avs_create_script_environment: avs_create_script_environment_func,
    pub avs_delete_script_environment: avs_delete_script_environment_func,
    pub avs_get_error: avs_get_error_func,
    pub avs_get_frame: avs_get_frame_func,
    pub avs_get_video_info: avs_get_video_info_func,
    pub avs_function_exists: avs_function_exists_func,
    pub avs_invoke: avs_invoke_func,
    pub avs_release_clip: avs_release_clip_func,
    pub avs_release_value: avs_release_value_func,
    pub avs_release_video_frame: avs_release_video_frame_func,
    pub avs_take_clip: avs_take_clip_func,
    pub avs_is_yv24: avs_is_yv24_func,
    pub avs_is_yv16: avs_is_yv16_func,
    pub avs_is_yv12: avs_is_yv12_func,
    pub avs_is_yv411: avs_is_yv411_func,
    pub avs_is_y8: avs_is_y8_func,
    pub avs_get_pitch_p: avs_get_pitch_p_func,
    pub avs_get_read_ptr_p: avs_get_read_ptr_p_func,
    pub avs_is_rgb48: avs_is_rgb48_func,
    pub avs_is_rgb64: avs_is_rgb64_func,
    pub avs_is_yuv444p16: avs_is_yuv444p16_func,
    pub avs_is_yuv422p16: avs_is_yuv422p16_func,
    pub avs_is_yuv420p16: avs_is_yuv420p16_func,
    pub avs_is_y16: avs_is_y16_func,
    pub avs_is_444: avs_is_444_func,
    pub avs_is_422: avs_is_422_func,
    pub avs_is_420: avs_is_420_func,
    pub avs_is_y: avs_is_y_func,
}
#[c2rust::src_loc = "54:9"]
pub const AVS_INTERFACE_25: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[c2rust::src_loc = "61:9"]
pub const AVS_MAX_SEQUENCE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
#[c2rust::src_loc = "120:1"]
unsafe extern "C" fn custom_avs_load_library(
    mut h: *mut avs_hnd_t,
) -> ::core::ffi::c_int {
    (*h).library = dlopen(
        b"libavisynth.so\0" as *const u8 as *const ::core::ffi::c_char,
        RTLD_NOW,
    );
    if (*h).library.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*h).func.avs_clip_get_error = ::core::mem::transmute::<
        *mut ::core::ffi::c_void,
        avs_clip_get_error_func,
    >(
        dlsym(
            (*h).library,
            b"avs_clip_get_error\0" as *const u8 as *const ::core::ffi::c_char,
        ),
    );
    if !(0 as ::core::ffi::c_int == 0 && (*h).func.avs_clip_get_error.is_none()) {
        (*h).func.avs_create_script_environment = ::core::mem::transmute::<
            *mut ::core::ffi::c_void,
            avs_create_script_environment_func,
        >(
            dlsym(
                (*h).library,
                b"avs_create_script_environment\0" as *const u8
                    as *const ::core::ffi::c_char,
            ),
        );
        if !(0 as ::core::ffi::c_int == 0
            && (*h).func.avs_create_script_environment.is_none())
        {
            (*h).func.avs_delete_script_environment = ::core::mem::transmute::<
                *mut ::core::ffi::c_void,
                avs_delete_script_environment_func,
            >(
                dlsym(
                    (*h).library,
                    b"avs_delete_script_environment\0" as *const u8
                        as *const ::core::ffi::c_char,
                ),
            );
            if !(1 as ::core::ffi::c_int == 0
                && (*h).func.avs_delete_script_environment.is_none())
            {
                (*h).func.avs_get_error = ::core::mem::transmute::<
                    *mut ::core::ffi::c_void,
                    avs_get_error_func,
                >(
                    dlsym(
                        (*h).library,
                        b"avs_get_error\0" as *const u8 as *const ::core::ffi::c_char,
                    ),
                );
                if !(1 as ::core::ffi::c_int == 0 && (*h).func.avs_get_error.is_none()) {
                    (*h).func.avs_get_frame = ::core::mem::transmute::<
                        *mut ::core::ffi::c_void,
                        avs_get_frame_func,
                    >(
                        dlsym(
                            (*h).library,
                            b"avs_get_frame\0" as *const u8 as *const ::core::ffi::c_char,
                        ),
                    );
                    if !(0 as ::core::ffi::c_int == 0
                        && (*h).func.avs_get_frame.is_none())
                    {
                        (*h).func.avs_get_video_info = ::core::mem::transmute::<
                            *mut ::core::ffi::c_void,
                            avs_get_video_info_func,
                        >(
                            dlsym(
                                (*h).library,
                                b"avs_get_video_info\0" as *const u8
                                    as *const ::core::ffi::c_char,
                            ),
                        );
                        if !(0 as ::core::ffi::c_int == 0
                            && (*h).func.avs_get_video_info.is_none())
                        {
                            (*h).func.avs_function_exists = ::core::mem::transmute::<
                                *mut ::core::ffi::c_void,
                                avs_function_exists_func,
                            >(
                                dlsym(
                                    (*h).library,
                                    b"avs_function_exists\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                ),
                            );
                            if !(0 as ::core::ffi::c_int == 0
                                && (*h).func.avs_function_exists.is_none())
                            {
                                (*h).func.avs_invoke = ::core::mem::transmute::<
                                    *mut ::core::ffi::c_void,
                                    avs_invoke_func,
                                >(
                                    dlsym(
                                        (*h).library,
                                        b"avs_invoke\0" as *const u8 as *const ::core::ffi::c_char,
                                    ),
                                );
                                if !(0 as ::core::ffi::c_int == 0
                                    && (*h).func.avs_invoke.is_none())
                                {
                                    (*h).func.avs_release_clip = ::core::mem::transmute::<
                                        *mut ::core::ffi::c_void,
                                        avs_release_clip_func,
                                    >(
                                        dlsym(
                                            (*h).library,
                                            b"avs_release_clip\0" as *const u8
                                                as *const ::core::ffi::c_char,
                                        ),
                                    );
                                    if !(0 as ::core::ffi::c_int == 0
                                        && (*h).func.avs_release_clip.is_none())
                                    {
                                        (*h).func.avs_release_value = ::core::mem::transmute::<
                                            *mut ::core::ffi::c_void,
                                            avs_release_value_func,
                                        >(
                                            dlsym(
                                                (*h).library,
                                                b"avs_release_value\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                            ),
                                        );
                                        if !(0 as ::core::ffi::c_int == 0
                                            && (*h).func.avs_release_value.is_none())
                                        {
                                            (*h).func.avs_release_video_frame = ::core::mem::transmute::<
                                                *mut ::core::ffi::c_void,
                                                avs_release_video_frame_func,
                                            >(
                                                dlsym(
                                                    (*h).library,
                                                    b"avs_release_video_frame\0" as *const u8
                                                        as *const ::core::ffi::c_char,
                                                ),
                                            );
                                            if !(0 as ::core::ffi::c_int == 0
                                                && (*h).func.avs_release_video_frame.is_none())
                                            {
                                                (*h).func.avs_take_clip = ::core::mem::transmute::<
                                                    *mut ::core::ffi::c_void,
                                                    avs_take_clip_func,
                                                >(
                                                    dlsym(
                                                        (*h).library,
                                                        b"avs_take_clip\0" as *const u8
                                                            as *const ::core::ffi::c_char,
                                                    ),
                                                );
                                                if !(0 as ::core::ffi::c_int == 0
                                                    && (*h).func.avs_take_clip.is_none())
                                                {
                                                    (*h).func.avs_is_yv24 = ::core::mem::transmute::<
                                                        *mut ::core::ffi::c_void,
                                                        avs_is_yv24_func,
                                                    >(
                                                        dlsym(
                                                            (*h).library,
                                                            b"avs_is_yv24\0" as *const u8 as *const ::core::ffi::c_char,
                                                        ),
                                                    );
                                                    if !(1 as ::core::ffi::c_int == 0
                                                        && (*h).func.avs_is_yv24.is_none())
                                                    {
                                                        (*h).func.avs_is_yv16 = ::core::mem::transmute::<
                                                            *mut ::core::ffi::c_void,
                                                            avs_is_yv16_func,
                                                        >(
                                                            dlsym(
                                                                (*h).library,
                                                                b"avs_is_yv16\0" as *const u8 as *const ::core::ffi::c_char,
                                                            ),
                                                        );
                                                        if !(1 as ::core::ffi::c_int == 0
                                                            && (*h).func.avs_is_yv16.is_none())
                                                        {
                                                            (*h).func.avs_is_yv12 = ::core::mem::transmute::<
                                                                *mut ::core::ffi::c_void,
                                                                avs_is_yv12_func,
                                                            >(
                                                                dlsym(
                                                                    (*h).library,
                                                                    b"avs_is_yv12\0" as *const u8 as *const ::core::ffi::c_char,
                                                                ),
                                                            );
                                                            if !(1 as ::core::ffi::c_int == 0
                                                                && (*h).func.avs_is_yv12.is_none())
                                                            {
                                                                (*h).func.avs_is_yv411 = ::core::mem::transmute::<
                                                                    *mut ::core::ffi::c_void,
                                                                    avs_is_yv411_func,
                                                                >(
                                                                    dlsym(
                                                                        (*h).library,
                                                                        b"avs_is_yv411\0" as *const u8 as *const ::core::ffi::c_char,
                                                                    ),
                                                                );
                                                                if !(1 as ::core::ffi::c_int == 0
                                                                    && (*h).func.avs_is_yv411.is_none())
                                                                {
                                                                    (*h).func.avs_is_y8 = ::core::mem::transmute::<
                                                                        *mut ::core::ffi::c_void,
                                                                        avs_is_y8_func,
                                                                    >(
                                                                        dlsym(
                                                                            (*h).library,
                                                                            b"avs_is_y8\0" as *const u8 as *const ::core::ffi::c_char,
                                                                        ),
                                                                    );
                                                                    if !(1 as ::core::ffi::c_int == 0
                                                                        && (*h).func.avs_is_y8.is_none())
                                                                    {
                                                                        (*h).func.avs_get_pitch_p = ::core::mem::transmute::<
                                                                            *mut ::core::ffi::c_void,
                                                                            avs_get_pitch_p_func,
                                                                        >(
                                                                            dlsym(
                                                                                (*h).library,
                                                                                b"avs_get_pitch_p\0" as *const u8
                                                                                    as *const ::core::ffi::c_char,
                                                                            ),
                                                                        );
                                                                        if !(1 as ::core::ffi::c_int == 0
                                                                            && (*h).func.avs_get_pitch_p.is_none())
                                                                        {
                                                                            (*h).func.avs_get_read_ptr_p = ::core::mem::transmute::<
                                                                                *mut ::core::ffi::c_void,
                                                                                avs_get_read_ptr_p_func,
                                                                            >(
                                                                                dlsym(
                                                                                    (*h).library,
                                                                                    b"avs_get_read_ptr_p\0" as *const u8
                                                                                        as *const ::core::ffi::c_char,
                                                                                ),
                                                                            );
                                                                            if !(1 as ::core::ffi::c_int == 0
                                                                                && (*h).func.avs_get_read_ptr_p.is_none())
                                                                            {
                                                                                (*h).func.avs_is_rgb48 = ::core::mem::transmute::<
                                                                                    *mut ::core::ffi::c_void,
                                                                                    avs_is_rgb48_func,
                                                                                >(
                                                                                    dlsym(
                                                                                        (*h).library,
                                                                                        b"avs_is_rgb48\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                    ),
                                                                                );
                                                                                if !(1 as ::core::ffi::c_int == 0
                                                                                    && (*h).func.avs_is_rgb48.is_none())
                                                                                {
                                                                                    if (*h).func.avs_is_rgb48.is_none() {
                                                                                        (*h).func.avs_is_rgb48 = ::core::mem::transmute::<
                                                                                            *mut ::core::ffi::c_void,
                                                                                            avs_is_rgb48_func,
                                                                                        >(
                                                                                            dlsym(
                                                                                                (*h).library,
                                                                                                b"_avs_is_rgb48@4\0" as *const u8
                                                                                                    as *const ::core::ffi::c_char,
                                                                                            ),
                                                                                        );
                                                                                    }
                                                                                    if !(1 as ::core::ffi::c_int == 0
                                                                                        && (*h).func.avs_is_rgb48.is_none())
                                                                                    {
                                                                                        (*h).func.avs_is_rgb64 = ::core::mem::transmute::<
                                                                                            *mut ::core::ffi::c_void,
                                                                                            avs_is_rgb64_func,
                                                                                        >(
                                                                                            dlsym(
                                                                                                (*h).library,
                                                                                                b"avs_is_rgb64\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                            ),
                                                                                        );
                                                                                        if !(1 as ::core::ffi::c_int == 0
                                                                                            && (*h).func.avs_is_rgb64.is_none())
                                                                                        {
                                                                                            if (*h).func.avs_is_rgb64.is_none() {
                                                                                                (*h).func.avs_is_rgb64 = ::core::mem::transmute::<
                                                                                                    *mut ::core::ffi::c_void,
                                                                                                    avs_is_rgb64_func,
                                                                                                >(
                                                                                                    dlsym(
                                                                                                        (*h).library,
                                                                                                        b"_avs_is_rgb64@4\0" as *const u8
                                                                                                            as *const ::core::ffi::c_char,
                                                                                                    ),
                                                                                                );
                                                                                            }
                                                                                            if !(1 as ::core::ffi::c_int == 0
                                                                                                && (*h).func.avs_is_rgb64.is_none())
                                                                                            {
                                                                                                (*h).func.avs_is_yuv444p16 = ::core::mem::transmute::<
                                                                                                    *mut ::core::ffi::c_void,
                                                                                                    avs_is_yuv444p16_func,
                                                                                                >(
                                                                                                    dlsym(
                                                                                                        (*h).library,
                                                                                                        b"avs_is_yuv444p16\0" as *const u8
                                                                                                            as *const ::core::ffi::c_char,
                                                                                                    ),
                                                                                                );
                                                                                                if !(1 as ::core::ffi::c_int == 0
                                                                                                    && (*h).func.avs_is_yuv444p16.is_none())
                                                                                                {
                                                                                                    (*h).func.avs_is_yuv422p16 = ::core::mem::transmute::<
                                                                                                        *mut ::core::ffi::c_void,
                                                                                                        avs_is_yuv422p16_func,
                                                                                                    >(
                                                                                                        dlsym(
                                                                                                            (*h).library,
                                                                                                            b"avs_is_yuv422p16\0" as *const u8
                                                                                                                as *const ::core::ffi::c_char,
                                                                                                        ),
                                                                                                    );
                                                                                                    if !(1 as ::core::ffi::c_int == 0
                                                                                                        && (*h).func.avs_is_yuv422p16.is_none())
                                                                                                    {
                                                                                                        (*h).func.avs_is_yuv420p16 = ::core::mem::transmute::<
                                                                                                            *mut ::core::ffi::c_void,
                                                                                                            avs_is_yuv420p16_func,
                                                                                                        >(
                                                                                                            dlsym(
                                                                                                                (*h).library,
                                                                                                                b"avs_is_yuv420p16\0" as *const u8
                                                                                                                    as *const ::core::ffi::c_char,
                                                                                                            ),
                                                                                                        );
                                                                                                        if !(1 as ::core::ffi::c_int == 0
                                                                                                            && (*h).func.avs_is_yuv420p16.is_none())
                                                                                                        {
                                                                                                            (*h).func.avs_is_y16 = ::core::mem::transmute::<
                                                                                                                *mut ::core::ffi::c_void,
                                                                                                                avs_is_y16_func,
                                                                                                            >(
                                                                                                                dlsym(
                                                                                                                    (*h).library,
                                                                                                                    b"avs_is_y16\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                ),
                                                                                                            );
                                                                                                            if !(1 as ::core::ffi::c_int == 0
                                                                                                                && (*h).func.avs_is_y16.is_none())
                                                                                                            {
                                                                                                                (*h).func.avs_is_444 = ::core::mem::transmute::<
                                                                                                                    *mut ::core::ffi::c_void,
                                                                                                                    avs_is_444_func,
                                                                                                                >(
                                                                                                                    dlsym(
                                                                                                                        (*h).library,
                                                                                                                        b"avs_is_444\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                    ),
                                                                                                                );
                                                                                                                if !(1 as ::core::ffi::c_int == 0
                                                                                                                    && (*h).func.avs_is_444.is_none())
                                                                                                                {
                                                                                                                    (*h).func.avs_is_422 = ::core::mem::transmute::<
                                                                                                                        *mut ::core::ffi::c_void,
                                                                                                                        avs_is_422_func,
                                                                                                                    >(
                                                                                                                        dlsym(
                                                                                                                            (*h).library,
                                                                                                                            b"avs_is_422\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                        ),
                                                                                                                    );
                                                                                                                    if !(1 as ::core::ffi::c_int == 0
                                                                                                                        && (*h).func.avs_is_422.is_none())
                                                                                                                    {
                                                                                                                        (*h).func.avs_is_420 = ::core::mem::transmute::<
                                                                                                                            *mut ::core::ffi::c_void,
                                                                                                                            avs_is_420_func,
                                                                                                                        >(
                                                                                                                            dlsym(
                                                                                                                                (*h).library,
                                                                                                                                b"avs_is_420\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                            ),
                                                                                                                        );
                                                                                                                        if !(1 as ::core::ffi::c_int == 0
                                                                                                                            && (*h).func.avs_is_420.is_none())
                                                                                                                        {
                                                                                                                            (*h).func.avs_is_y = ::core::mem::transmute::<
                                                                                                                                *mut ::core::ffi::c_void,
                                                                                                                                avs_is_y_func,
                                                                                                                            >(
                                                                                                                                dlsym(
                                                                                                                                    (*h).library,
                                                                                                                                    b"avs_is_y\0" as *const u8 as *const ::core::ffi::c_char,
                                                                                                                                ),
                                                                                                                            );
                                                                                                                            if !(1 as ::core::ffi::c_int == 0
                                                                                                                                && (*h).func.avs_is_y.is_none())
                                                                                                                            {
                                                                                                                                return 0 as ::core::ffi::c_int;
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
    return -(1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "185:1"]
unsafe extern "C" fn avs_build_filter_sequence(
    mut filename_ext: *mut ::core::ffi::c_char,
    mut filter: *mut *const ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut all_purpose: [*const ::core::ffi::c_char; 2] = [
        b"FFVideoSource\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
    (*h).func.avs_release_clip.expect("non-null function pointer")((*h).clip);
    (*h).clip = (*h)
        .func
        .avs_take_clip
        .expect("non-null function pointer")(res, (*h).env);
    (*h).func.avs_release_value.expect("non-null function pointer")(release);
    *vi = (*h).func.avs_get_video_info.expect("non-null function pointer")((*h).clip);
    return res;
}
#[c2rust::src_loc = "212:1"]
unsafe extern "C" fn get_avs_version(mut h: *mut avs_hnd_t) -> ::core::ffi::c_float {
    if (*h)
        .func
        .avs_function_exists
        .expect(
            "non-null function pointer",
        )((*h).env, b"VersionNumber\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        x264_cli_log(
            b"avs\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"VersionNumber does not exist\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_float;
    }
    let mut ver: AVS_Value = (*h)
        .func
        .avs_invoke
        .expect(
            "non-null function pointer",
        )(
        (*h).env,
        b"VersionNumber\0" as *const u8 as *const ::core::ffi::c_char,
        avs_new_value_array(0 as *mut AVS_Value, 0 as ::core::ffi::c_int),
        0 as *mut *const ::core::ffi::c_char,
    );
    if avs_is_error(ver) != 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"unable to determine avisynth version: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            avs_as_error(ver),
        );
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_float;
    }
    if avs_is_float(ver) == 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"VersionNumber did not return a float value\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as ::core::ffi::c_float;
    }
    let mut ret: ::core::ffi::c_float = avs_as_float(ver) as ::core::ffi::c_float;
    (*h).func.avs_release_value.expect("non-null function pointer")(ver);
    return ret;
}
#[c2rust::src_loc = "269:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut ::core::ffi::c_char,
    mut p_handle: *mut hnd_t,
    mut info: *mut video_info_t,
    mut opt: *mut cli_input_opt_t,
) -> ::core::ffi::c_int {
    let mut fh: *mut FILE = fopen(
        psz_filename,
        b"r\0" as *const u8 as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if fh.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut b_regular: ::core::ffi::c_int = x264_is_regular_file(fh);
    fclose(fh);
    if b_regular == 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"AVS input is incompatible with non-regular file `%s'\n\0" as *const u8
                as *const ::core::ffi::c_char,
            psz_filename,
        );
        return -(1 as ::core::ffi::c_int);
    }
    let mut h: *mut avs_hnd_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<avs_hnd_t>() as size_t,
    ) as *mut avs_hnd_t;
    if h.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    if custom_avs_load_library(h) != 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to load avisynth\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*h).env = (*h)
        .func
        .avs_create_script_environment
        .expect("non-null function pointer")(AVS_INTERFACE_25);
    if (*h).func.avs_get_error.is_some() {
        let mut error: *const ::core::ffi::c_char = (*h)
            .func
            .avs_get_error
            .expect("non-null function pointer")((*h).env);
        if !error.is_null() {
            x264_cli_log(
                b"avs\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                error,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    let mut avs_version: ::core::ffi::c_float = get_avs_version(h);
    if avs_version <= 0 as ::core::ffi::c_int as ::core::ffi::c_float {
        return -(1 as ::core::ffi::c_int);
    }
    x264_cli_log(
        b"avs\0" as *const u8 as *const ::core::ffi::c_char,
        X264_LOG_DEBUG,
        b"using avisynth version %.2f\n\0" as *const u8 as *const ::core::ffi::c_char,
        avs_version as ::core::ffi::c_double,
    );
    let mut arg: AVS_Value = avs_new_value_string(psz_filename);
    let mut res: AVS_Value = AVS_Value {
        type_0: 0,
        array_size: 0,
        d: C2RustUnnamed_0 {
            clip: 0 as *mut ::core::ffi::c_void,
        },
    };
    let mut filename_ext: *mut ::core::ffi::c_char = get_filename_extension(
        psz_filename,
    );
    if strcasecmp(filename_ext, b"avs\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        res = (*h)
            .func
            .avs_invoke
            .expect(
                "non-null function pointer",
            )(
            (*h).env,
            b"Import\0" as *const u8 as *const ::core::ffi::c_char,
            arg,
            0 as *mut *const ::core::ffi::c_char,
        );
        if avs_is_error(res) != 0 {
            x264_cli_log(
                b"avs\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"%s\n\0" as *const u8 as *const ::core::ffi::c_char,
                avs_as_error(res),
            );
            return -(1 as ::core::ffi::c_int);
        }
        let mut mt_test: AVS_Value = (*h)
            .func
            .avs_invoke
            .expect(
                "non-null function pointer",
            )(
            (*h).env,
            b"GetMTMode\0" as *const u8 as *const ::core::ffi::c_char,
            avs_new_value_bool(0 as ::core::ffi::c_int),
            0 as *mut *const ::core::ffi::c_char,
        );
        let mut mt_mode: ::core::ffi::c_int = if avs_is_int(mt_test) != 0 {
            avs_as_int(mt_test)
        } else {
            0 as ::core::ffi::c_int
        };
        (*h).func.avs_release_value.expect("non-null function pointer")(mt_test);
        if mt_mode > 0 as ::core::ffi::c_int && mt_mode < 5 as ::core::ffi::c_int {
            let mut temp: AVS_Value = (*h)
                .func
                .avs_invoke
                .expect(
                    "non-null function pointer",
                )(
                (*h).env,
                b"Distributor\0" as *const u8 as *const ::core::ffi::c_char,
                res,
                0 as *mut *const ::core::ffi::c_char,
            );
            (*h).func.avs_release_value.expect("non-null function pointer")(res);
            res = temp;
        }
    } else {
        let mut filter: [*const ::core::ffi::c_char; 6] = [
            0 as *const ::core::ffi::c_char,
            0 as *const ::core::ffi::c_char,
            0 as *const ::core::ffi::c_char,
            0 as *const ::core::ffi::c_char,
            0 as *const ::core::ffi::c_char,
            0 as *const ::core::ffi::c_char,
        ];
        avs_build_filter_sequence(filename_ext, filter.as_mut_ptr());
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while !filter[i as usize].is_null() {
            x264_cli_log(
                b"avs\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_INFO,
                b"trying %s... \0" as *const u8 as *const ::core::ffi::c_char,
                filter[i as usize],
            );
            if (*h)
                .func
                .avs_function_exists
                .expect("non-null function pointer")((*h).env, filter[i as usize]) == 0
            {
                x264_cli_printf(
                    X264_LOG_INFO,
                    b"not found\n\0" as *const u8 as *const ::core::ffi::c_char,
                );
            } else {
                if strncasecmp(
                    filter[i as usize],
                    b"FFmpegSource\0" as *const u8 as *const ::core::ffi::c_char,
                    12 as size_t,
                ) == 0
                {
                    x264_cli_printf(
                        X264_LOG_INFO,
                        b"indexing... \0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    fflush(stderr);
                }
                res = (*h)
                    .func
                    .avs_invoke
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).env,
                    filter[i as usize],
                    arg,
                    0 as *mut *const ::core::ffi::c_char,
                );
                if avs_is_error(res) == 0 {
                    x264_cli_printf(
                        X264_LOG_INFO,
                        b"succeeded\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    break;
                } else {
                    x264_cli_printf(
                        X264_LOG_INFO,
                        b"failed\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                }
            }
            i += 1;
        }
        if filter[i as usize].is_null() {
            x264_cli_log(
                b"avs\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"unable to find source filter to open `%s'\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                psz_filename,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    if avs_is_clip(res) == 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"`%s' didn't return a video clip\n\0" as *const u8
                as *const ::core::ffi::c_char,
            psz_filename,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*h).clip = (*h)
        .func
        .avs_take_clip
        .expect("non-null function pointer")(res, (*h).env);
    let mut vi: *const AVS_VideoInfo = (*h)
        .func
        .avs_get_video_info
        .expect("non-null function pointer")((*h).clip);
    if avs_has_video(vi) == 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"`%s' has no video data\n\0" as *const u8 as *const ::core::ffi::c_char,
            psz_filename,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if avs_is_field_based(vi) != 0 {
        x264_cli_log(
            b"avs\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_WARNING,
            b"detected fieldbased (separated) input, weaving to frames\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        let mut tmp: AVS_Value = (*h)
            .func
            .avs_invoke
            .expect(
                "non-null function pointer",
            )(
            (*h).env,
            b"Weave\0" as *const u8 as *const ::core::ffi::c_char,
            res,
            0 as *mut *const ::core::ffi::c_char,
        );
        if avs_is_error(tmp) != 0 {
            x264_cli_log(
                b"avs\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"couldn't weave fields into frames: %s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                avs_as_error(tmp),
            );
            return -(1 as ::core::ffi::c_int);
        }
        res = update_clip(h, &mut vi, tmp, res);
        (*info).interlaced = 1 as ::core::ffi::c_int;
        (*info).tff = avs_is_tff(vi);
    }
    (*h).func.avs_release_value.expect("non-null function pointer")(res);
    (*info).width = (*vi).width;
    (*info).height = (*vi).height;
    (*info).fps_num = (*vi).fps_numerator as uint32_t;
    (*info).fps_den = (*vi).fps_denominator as uint32_t;
    (*info).num_frames = (*vi).num_frames;
    (*h).num_frames = (*info).num_frames;
    (*info).thread_safe = 1 as ::core::ffi::c_int;
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
        && (*h).func.avs_is_yuv444p16.expect("non-null function pointer")(vi) != 0
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
        && (*h).func.avs_is_yuv422p16.expect("non-null function pointer")(vi) != 0
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
        && (*h).func.avs_is_yuv420p16.expect("non-null function pointer")(vi) != 0
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
        (*info).csp = AV_PIX_FMT_YUV411P as ::core::ffi::c_int | X264_CSP_OTHER;
    } else {
        let mut pixel_type: AVS_Value = (*h)
            .func
            .avs_invoke
            .expect(
                "non-null function pointer",
            )(
            (*h).env,
            b"PixelType\0" as *const u8 as *const ::core::ffi::c_char,
            res,
            0 as *mut *const ::core::ffi::c_char,
        );
        let mut pixel_type_name: *const ::core::ffi::c_char = if avs_is_string(
            pixel_type,
        ) != 0
        {
            avs_as_string(pixel_type)
        } else {
            b"unknown\0" as *const u8 as *const ::core::ffi::c_char
        };
        x264_cli_log(
            b"avs\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"not supported pixel type: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            pixel_type_name,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*info).vfr = 0 as ::core::ffi::c_int;
    *p_handle = h as hnd_t;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "504:1"]
unsafe extern "C" fn picture_alloc(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut csp: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if x264_cli_pic_alloc(pic, X264_CSP_NONE, width, height) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    (*pic).img.csp = csp;
    let mut cli_csp: *const x264_cli_csp_t = x264_cli_get_csp(csp);
    if !cli_csp.is_null() {
        (*pic).img.planes = (*cli_csp).planes;
    } else if csp == AV_PIX_FMT_YUV411P as ::core::ffi::c_int | X264_CSP_OTHER {
        (*pic).img.planes = 3 as ::core::ffi::c_int;
    } else {
        (*pic).img.planes = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "521:1"]
unsafe extern "C" fn read_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
    mut i_frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    static mut plane: [::core::ffi::c_int; 3] = [
        AVS_PLANAR_Y as ::core::ffi::c_int,
        AVS_PLANAR_U as ::core::ffi::c_int,
        AVS_PLANAR_V as ::core::ffi::c_int,
    ];
    let mut h: *mut avs_hnd_t = handle as *mut avs_hnd_t;
    if i_frame >= (*h).num_frames {
        return -(1 as ::core::ffi::c_int);
    }
    (*pic).opaque = (*h)
        .func
        .avs_get_frame
        .expect("non-null function pointer")((*h).clip, i_frame)
        as *mut ::core::ffi::c_void;
    let mut frm: *mut AVS_VideoFrame = (*pic).opaque as *mut AVS_VideoFrame;
    let mut err: *const ::core::ffi::c_char = (*h)
        .func
        .avs_clip_get_error
        .expect("non-null function pointer")((*h).clip);
    if !err.is_null() {
        x264_cli_log(
            b"avs\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"%s occurred while reading frame %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            err,
            i_frame,
        );
        return -(1 as ::core::ffi::c_int);
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*pic).img.planes {
        (*pic).img.plane[i as usize] = (if (*h).func.avs_get_read_ptr_p.is_some() {
            (*h)
                .func
                .avs_get_read_ptr_p
                .expect("non-null function pointer")(frm, plane[i as usize])
        } else {
            avs_get_read_ptr_p(frm, plane[i as usize])
        }) as *mut uint8_t;
        (*pic).img.stride[i as usize] = if (*h).func.avs_get_pitch_p.is_some() {
            (*h)
                .func
                .avs_get_pitch_p
                .expect("non-null function pointer")(frm, plane[i as usize])
        } else {
            avs_get_pitch_p(frm, plane[i as usize])
        };
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "539:1"]
unsafe extern "C" fn release_frame(
    mut pic: *mut cli_pic_t,
    mut handle: hnd_t,
) -> ::core::ffi::c_int {
    let mut h: *mut avs_hnd_t = handle as *mut avs_hnd_t;
    (*h)
        .func
        .avs_release_video_frame
        .expect("non-null function pointer")((*pic).opaque as *mut AVS_VideoFrame);
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "546:1"]
unsafe extern "C" fn picture_clean(mut pic: *mut cli_pic_t, mut handle: hnd_t) {
    memset(
        pic as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cli_pic_t>() as size_t,
    );
}
#[c2rust::src_loc = "551:1"]
unsafe extern "C" fn close_file(mut handle: hnd_t) -> ::core::ffi::c_int {
    let mut h: *mut avs_hnd_t = handle as *mut avs_hnd_t;
    if (*h).func.avs_release_clip.is_some() && !(*h).clip.is_null() {
        (*h).func.avs_release_clip.expect("non-null function pointer")((*h).clip);
    }
    if (*h).func.avs_delete_script_environment.is_some() && !(*h).env.is_null() {
        (*h)
            .func
            .avs_delete_script_environment
            .expect("non-null function pointer")((*h).env);
    }
    if !(*h).library.is_null() {
        dlclose((*h).library);
    }
    free(h as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "564:19"]
pub static mut avs_input: cli_input_t = unsafe {
    {
        let mut init = cli_input_t {
            open_file: Some(
                open_file
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut hnd_t,
                        *mut video_info_t,
                        *mut cli_input_opt_t,
                    ) -> ::core::ffi::c_int,
            ),
            picture_alloc: Some(
                picture_alloc
                    as unsafe extern "C" fn(
                        *mut cli_pic_t,
                        hnd_t,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            read_frame: Some(
                read_frame
                    as unsafe extern "C" fn(
                        *mut cli_pic_t,
                        hnd_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            release_frame: Some(
                release_frame
                    as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ::core::ffi::c_int,
            ),
            picture_clean: Some(
                picture_clean as unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> (),
            ),
            close_file: Some(
                close_file as unsafe extern "C" fn(hnd_t) -> ::core::ffi::c_int,
            ),
        };
        init
    }
};
