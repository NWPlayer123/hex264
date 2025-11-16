#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(c_variadic)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(extern_types)]
#![feature(register_tool)]
//#![feature(stdsimd)]
#![register_tool(c2rust)]

#[macro_use]
extern crate c2rust_bitfields;
pub mod src {
    pub mod autocomplete;
    pub mod common {
        pub mod base;
        pub mod bitstream;
        pub mod cabac;
        pub mod common;
        pub mod cpu;
        pub mod dct;
        pub mod deblock;
        pub mod frame;
        pub mod macroblock;
        pub mod mc;
        pub mod mvpred;
        pub mod osdep;
        pub mod pixel;
        pub mod predict;
        pub mod quant;
        pub mod rectangle;
        pub mod set;
        pub mod tables;
        pub mod threadpool;
        pub mod vlc;
    } // mod common
    pub mod encoder {
        pub mod analyse;
        pub mod api;
        pub mod cabac;
        pub mod cavlc;
        pub mod encoder;
        pub mod lookahead;
        pub mod macroblock;
        pub mod me;
        pub mod ratecontrol;
        pub mod set;
    } // mod encoder
    pub mod filters {
        pub mod filters;
        pub mod video {
            pub mod cache;
            pub mod crop;
            pub mod depth;
            pub mod fix_vfr_pts;
            pub mod internal;
            pub mod resize;
            pub mod select_every;
            pub mod source;
            pub mod video;
        } // mod video
    } // mod filters
    pub mod input {
        pub mod avs;
        pub mod input;
        pub mod lavf;
        pub mod raw;
        pub mod thread;
        pub mod timecode;
        pub mod y4m;
    } // mod input
    pub mod output {
        pub mod flv;
        pub mod flv_bytestream;
        pub mod matroska;
        pub mod matroska_ebml;
        pub mod mp4_lsmash;
        pub mod raw;
    } // mod output
    pub mod x264;
} // mod src

#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "0:0"]
    pub type __builtin_va_list = [__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "0:0"]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:37"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stdarg___gnuc_va_list.h:37"]
pub mod __stdarg___gnuc_va_list_h {
    #[c2rust::src_loc = "12:1"]
    pub type __gnuc_va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
}
#[c2rust::header_src = "/usr/include/bits/types.h:37"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = u16;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = i32;
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:37"]
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
    use super::types_h::{__off64_t, __off_t, __uint64_t};
    extern "C" {
        #[c2rust::src_loc = "40:8"]
        pub type _IO_wide_data;
        #[c2rust::src_loc = "39:8"]
        pub type _IO_codecvt;
        #[c2rust::src_loc = "38:8"]
        pub type _IO_marker;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:37"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdio.h:37"]
pub mod stdio_h {
    #[c2rust::src_loc = "53:1"]
    pub type va_list = __gnuc_va_list;
    #[c2rust::src_loc = "105:9"]
    pub const EOF: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    #[c2rust::src_loc = "110:9"]
    pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::FILE_h::FILE;
    use super::__stdarg___gnuc_va_list_h::__gnuc_va_list;
    use super::types_h::__off64_t;
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
        #[c2rust::src_loc = "360:1"]
        pub fn fprintf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "366:1"]
        pub fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "368:1"]
        pub fn sprintf(
            __s: *mut ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "375:1"]
        pub fn vfprintf(
            __s: *mut FILE,
            __format: *const ::core::ffi::c_char,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "445:1"]
        pub fn fscanf(
            __stream: *mut FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "450:1"]
        pub fn sscanf(
            __s: *const ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "802:1"]
        pub fn fseeko(
            __stream: *mut FILE,
            __off: __off64_t,
            __whence: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "805:1"]
        pub fn ftello(__stream: *mut FILE) -> __off64_t;
        #[c2rust::src_loc = "873:1"]
        pub fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    }
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "150:14"]
        pub static mut stdout: *mut FILE;
        #[c2rust::src_loc = "388:1"]
        pub fn snprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "612:1"]
        pub fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:37"]
pub mod struct_timespec_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "11:8"]
    pub struct timespec {
        pub tv_sec: __time_t,
        pub tv_nsec: __syscall_slong_t,
    }
    use super::types_h::{__syscall_slong_t, __time_t};
}
#[c2rust::header_src = "/usr/include/bits/struct_stat.h:37"]
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
    use super::struct_timespec_h::timespec;
    use super::types_h::{
        __blkcnt_t, __blksize_t, __dev_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t,
        __syscall_slong_t, __uid_t,
    };
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:37"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:37"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t, __uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:37"]
pub mod x264_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:16"]
    pub struct x264_nal_t {
        pub i_ref_idc: ::core::ffi::c_int,
        pub i_type: ::core::ffi::c_int,
        pub b_long_startcode: ::core::ffi::c_int,
        pub i_first_mb: ::core::ffi::c_int,
        pub i_last_mb: ::core::ffi::c_int,
        pub i_payload: ::core::ffi::c_int,
        pub p_payload: *mut uint8_t,
        pub i_padding: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "306:16"]
    pub struct x264_zone_t {
        pub i_start: ::core::ffi::c_int,
        pub i_end: ::core::ffi::c_int,
        pub b_force_qp: ::core::ffi::c_int,
        pub i_qp: ::core::ffi::c_int,
        pub f_bitrate_factor: ::core::ffi::c_float,
        pub param: *mut x264_param_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "315:16"]
    pub struct x264_param_t {
        pub cpu: uint32_t,
        pub i_threads: ::core::ffi::c_int,
        pub i_lookahead_threads: ::core::ffi::c_int,
        pub b_sliced_threads: ::core::ffi::c_int,
        pub b_deterministic: ::core::ffi::c_int,
        pub b_cpu_independent: ::core::ffi::c_int,
        pub i_sync_lookahead: ::core::ffi::c_int,
        pub i_width: ::core::ffi::c_int,
        pub i_height: ::core::ffi::c_int,
        pub i_csp: ::core::ffi::c_int,
        pub i_bitdepth: ::core::ffi::c_int,
        pub i_level_idc: ::core::ffi::c_int,
        pub i_frame_total: ::core::ffi::c_int,
        pub i_nal_hrd: ::core::ffi::c_int,
        pub vui: C2RustUnnamed_4,
        pub i_frame_reference: ::core::ffi::c_int,
        pub i_dpb_size: ::core::ffi::c_int,
        pub i_keyint_max: ::core::ffi::c_int,
        pub i_keyint_min: ::core::ffi::c_int,
        pub i_scenecut_threshold: ::core::ffi::c_int,
        pub b_intra_refresh: ::core::ffi::c_int,
        pub i_bframe: ::core::ffi::c_int,
        pub i_bframe_adaptive: ::core::ffi::c_int,
        pub i_bframe_bias: ::core::ffi::c_int,
        pub i_bframe_pyramid: ::core::ffi::c_int,
        pub b_open_gop: ::core::ffi::c_int,
        pub b_bluray_compat: ::core::ffi::c_int,
        pub i_avcintra_class: ::core::ffi::c_int,
        pub i_avcintra_flavor: ::core::ffi::c_int,
        pub b_deblocking_filter: ::core::ffi::c_int,
        pub i_deblocking_filter_alphac0: ::core::ffi::c_int,
        pub i_deblocking_filter_beta: ::core::ffi::c_int,
        pub b_cabac: ::core::ffi::c_int,
        pub i_cabac_init_idc: ::core::ffi::c_int,
        pub b_interlaced: ::core::ffi::c_int,
        pub b_constrained_intra: ::core::ffi::c_int,
        pub i_cqm_preset: ::core::ffi::c_int,
        pub psz_cqm_file: *mut ::core::ffi::c_char,
        pub cqm_4iy: [uint8_t; 16],
        pub cqm_4py: [uint8_t; 16],
        pub cqm_4ic: [uint8_t; 16],
        pub cqm_4pc: [uint8_t; 16],
        pub cqm_8iy: [uint8_t; 64],
        pub cqm_8py: [uint8_t; 64],
        pub cqm_8ic: [uint8_t; 64],
        pub cqm_8pc: [uint8_t; 64],
        pub pf_log: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> (),
        >,
        pub p_log_private: *mut ::core::ffi::c_void,
        pub i_log_level: ::core::ffi::c_int,
        pub b_full_recon: ::core::ffi::c_int,
        pub psz_dump_yuv: *mut ::core::ffi::c_char,
        pub analyse: C2RustUnnamed_3,
        pub rc: C2RustUnnamed_2,
        pub crop_rect: C2RustUnnamed_1,
        pub i_frame_packing: ::core::ffi::c_int,
        pub mastering_display: C2RustUnnamed_0,
        pub content_light_level: C2RustUnnamed,
        pub i_alternative_transfer: ::core::ffi::c_int,
        pub b_aud: ::core::ffi::c_int,
        pub b_repeat_headers: ::core::ffi::c_int,
        pub b_annexb: ::core::ffi::c_int,
        pub i_sps_id: ::core::ffi::c_int,
        pub b_vfr_input: ::core::ffi::c_int,
        pub b_pulldown: ::core::ffi::c_int,
        pub i_fps_num: uint32_t,
        pub i_fps_den: uint32_t,
        pub i_timebase_num: uint32_t,
        pub i_timebase_den: uint32_t,
        pub b_tff: ::core::ffi::c_int,
        pub b_pic_struct: ::core::ffi::c_int,
        pub b_fake_interlaced: ::core::ffi::c_int,
        pub b_stitchable: ::core::ffi::c_int,
        pub b_opencl: ::core::ffi::c_int,
        pub i_opencl_device: ::core::ffi::c_int,
        pub opencl_device_id: *mut ::core::ffi::c_void,
        pub psz_clbin_file: *mut ::core::ffi::c_char,
        pub i_slice_max_size: ::core::ffi::c_int,
        pub i_slice_max_mbs: ::core::ffi::c_int,
        pub i_slice_min_mbs: ::core::ffi::c_int,
        pub i_slice_count: ::core::ffi::c_int,
        pub i_slice_count_max: ::core::ffi::c_int,
        pub param_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub nalu_process: Option<
            unsafe extern "C" fn(*mut x264_t, *mut x264_nal_t, *mut ::core::ffi::c_void) -> (),
        >,
        pub opaque: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "517:5"]
    pub struct C2RustUnnamed {
        pub b_cll: ::core::ffi::c_int,
        pub i_max_cll: ::core::ffi::c_int,
        pub i_max_fall: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "501:5"]
    pub struct C2RustUnnamed_0 {
        pub b_mastering_display: ::core::ffi::c_int,
        pub i_green_x: ::core::ffi::c_int,
        pub i_green_y: ::core::ffi::c_int,
        pub i_blue_x: ::core::ffi::c_int,
        pub i_blue_y: ::core::ffi::c_int,
        pub i_red_x: ::core::ffi::c_int,
        pub i_red_y: ::core::ffi::c_int,
        pub i_white_x: ::core::ffi::c_int,
        pub i_white_y: ::core::ffi::c_int,
        pub i_display_max: int64_t,
        pub i_display_min: int64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "488:5"]
    pub struct C2RustUnnamed_1 {
        pub i_left: ::core::ffi::c_int,
        pub i_top: ::core::ffi::c_int,
        pub i_right: ::core::ffi::c_int,
        pub i_bottom: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "443:5"]
    pub struct C2RustUnnamed_2 {
        pub i_rc_method: ::core::ffi::c_int,
        pub i_qp_constant: ::core::ffi::c_int,
        pub i_qp_min: ::core::ffi::c_int,
        pub i_qp_max: ::core::ffi::c_int,
        pub i_qp_step: ::core::ffi::c_int,
        pub i_bitrate: ::core::ffi::c_int,
        pub f_rf_constant: ::core::ffi::c_float,
        pub f_rf_constant_max: ::core::ffi::c_float,
        pub f_rate_tolerance: ::core::ffi::c_float,
        pub i_vbv_max_bitrate: ::core::ffi::c_int,
        pub i_vbv_buffer_size: ::core::ffi::c_int,
        pub f_vbv_buffer_init: ::core::ffi::c_float,
        pub f_ip_factor: ::core::ffi::c_float,
        pub f_pb_factor: ::core::ffi::c_float,
        pub b_filler: ::core::ffi::c_int,
        pub i_aq_mode: ::core::ffi::c_int,
        pub f_aq_strength: ::core::ffi::c_float,
        pub b_mb_tree: ::core::ffi::c_int,
        pub i_lookahead: ::core::ffi::c_int,
        pub b_stat_write: ::core::ffi::c_int,
        pub psz_stat_out: *mut ::core::ffi::c_char,
        pub b_stat_read: ::core::ffi::c_int,
        pub psz_stat_in: *mut ::core::ffi::c_char,
        pub f_qcompress: ::core::ffi::c_float,
        pub f_qblur: ::core::ffi::c_float,
        pub f_complexity_blur: ::core::ffi::c_float,
        pub zones: *mut x264_zone_t,
        pub i_zones: ::core::ffi::c_int,
        pub psz_zones: *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "406:5"]
    pub struct C2RustUnnamed_3 {
        pub intra: ::core::ffi::c_uint,
        pub inter: ::core::ffi::c_uint,
        pub b_transform_8x8: ::core::ffi::c_int,
        pub i_weighted_pred: ::core::ffi::c_int,
        pub b_weighted_bipred: ::core::ffi::c_int,
        pub i_direct_mv_pred: ::core::ffi::c_int,
        pub i_chroma_qp_offset: ::core::ffi::c_int,
        pub i_me_method: ::core::ffi::c_int,
        pub i_me_range: ::core::ffi::c_int,
        pub i_mv_range: ::core::ffi::c_int,
        pub i_mv_range_thread: ::core::ffi::c_int,
        pub i_subpel_refine: ::core::ffi::c_int,
        pub b_chroma_me: ::core::ffi::c_int,
        pub b_mixed_references: ::core::ffi::c_int,
        pub i_trellis: ::core::ffi::c_int,
        pub b_fast_pskip: ::core::ffi::c_int,
        pub b_dct_decimate: ::core::ffi::c_int,
        pub i_noise_reduction: ::core::ffi::c_int,
        pub f_psy_rd: ::core::ffi::c_float,
        pub f_psy_trellis: ::core::ffi::c_float,
        pub b_psy: ::core::ffi::c_int,
        pub b_mb_info: ::core::ffi::c_int,
        pub b_mb_info_update: ::core::ffi::c_int,
        pub i_luma_deadzone: [::core::ffi::c_int; 2],
        pub b_psnr: ::core::ffi::c_int,
        pub b_ssim: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "342:5"]
    pub struct C2RustUnnamed_4 {
        pub i_sar_height: ::core::ffi::c_int,
        pub i_sar_width: ::core::ffi::c_int,
        pub i_overscan: ::core::ffi::c_int,
        pub i_vidformat: ::core::ffi::c_int,
        pub b_fullrange: ::core::ffi::c_int,
        pub i_colorprim: ::core::ffi::c_int,
        pub i_transfer: ::core::ffi::c_int,
        pub i_colmatrix: ::core::ffi::c_int,
        pub i_chroma_loc: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "633:16"]
    pub struct x264_level_t {
        pub level_idc: uint8_t,
        pub mbps: int32_t,
        pub frame_size: int32_t,
        pub dpb: int32_t,
        pub bitrate: int32_t,
        pub cpb: int32_t,
        pub mv_range: uint16_t,
        pub mvs_per_2mb: uint8_t,
        pub slice_rate: uint8_t,
        pub mincr: uint8_t,
        pub bipred8x8: uint8_t,
        pub direct8x8: uint8_t,
        pub frame_only: uint8_t,
    }
    #[c2rust::src_loc = "757:1"]
    pub type pic_struct_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "767:5"]
    pub const PIC_STRUCT_TRIPLE: pic_struct_e = 9;
    #[c2rust::src_loc = "766:5"]
    pub const PIC_STRUCT_DOUBLE: pic_struct_e = 8;
    #[c2rust::src_loc = "765:5"]
    pub const PIC_STRUCT_BOTTOM_TOP_BOTTOM: pic_struct_e = 7;
    #[c2rust::src_loc = "764:5"]
    pub const PIC_STRUCT_TOP_BOTTOM_TOP: pic_struct_e = 6;
    #[c2rust::src_loc = "763:5"]
    pub const PIC_STRUCT_BOTTOM_TOP: pic_struct_e = 5;
    #[c2rust::src_loc = "762:5"]
    pub const PIC_STRUCT_TOP_BOTTOM: pic_struct_e = 4;
    #[c2rust::src_loc = "760:5"]
    pub const PIC_STRUCT_PROGRESSIVE: pic_struct_e = 1;
    #[c2rust::src_loc = "759:5"]
    pub const PIC_STRUCT_AUTO: pic_struct_e = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "770:16"]
    pub struct x264_hrd_t {
        pub cpb_initial_arrival_time: ::core::ffi::c_double,
        pub cpb_final_arrival_time: ::core::ffi::c_double,
        pub cpb_removal_time: ::core::ffi::c_double,
        pub dpb_output_time: ::core::ffi::c_double,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "788:16"]
    pub struct x264_sei_payload_t {
        pub payload_size: ::core::ffi::c_int,
        pub payload_type: ::core::ffi::c_int,
        pub payload: *mut uint8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "795:16"]
    pub struct x264_sei_t {
        pub num_payloads: ::core::ffi::c_int,
        pub payloads: *mut x264_sei_payload_t,
        pub sei_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "803:16"]
    pub struct x264_image_t {
        pub i_csp: ::core::ffi::c_int,
        pub i_plane: ::core::ffi::c_int,
        pub i_stride: [::core::ffi::c_int; 4],
        pub plane: [*mut uint8_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "811:16"]
    pub struct x264_image_properties_t {
        pub quant_offsets: *mut ::core::ffi::c_float,
        pub quant_offsets_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub mb_info: *mut uint8_t,
        pub mb_info_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub f_ssim: ::core::ffi::c_double,
        pub f_psnr_avg: ::core::ffi::c_double,
        pub f_psnr: [::core::ffi::c_double; 3],
        pub f_crf_avg: ::core::ffi::c_double,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "867:16"]
    pub struct x264_picture_t {
        pub i_type: ::core::ffi::c_int,
        pub i_qpplus1: ::core::ffi::c_int,
        pub i_pic_struct: ::core::ffi::c_int,
        pub b_keyframe: ::core::ffi::c_int,
        pub i_pts: int64_t,
        pub i_dts: int64_t,
        pub param: *mut x264_param_t,
        pub img: x264_image_t,
        pub prop: x264_image_properties_t,
        pub hrd_timing: x264_hrd_t,
        pub extra_sei: x264_sei_t,
        pub opaque: *mut ::core::ffi::c_void,
    }
    #[c2rust::src_loc = "48:9"]
    pub const X264_BUILD: ::core::ffi::c_int = 165 as ::core::ffi::c_int;
    #[c2rust::src_loc = "217:9"]
    pub const X264_QP_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "238:27"]
    pub static mut x264_direct_pred_names: [*const ::core::ffi::c_char; 5] = [
        b"none\0" as *const u8 as *const ::core::ffi::c_char,
        b"spatial\0" as *const u8 as *const ::core::ffi::c_char,
        b"temporal\0" as *const u8 as *const ::core::ffi::c_char,
        b"auto\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "239:27"]
    pub static mut x264_motion_est_names: [*const ::core::ffi::c_char; 6] = [
        b"dia\0" as *const u8 as *const ::core::ffi::c_char,
        b"hex\0" as *const u8 as *const ::core::ffi::c_char,
        b"umh\0" as *const u8 as *const ::core::ffi::c_char,
        b"esa\0" as *const u8 as *const ::core::ffi::c_char,
        b"tesa\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "240:27"]
    pub static mut x264_b_pyramid_names: [*const ::core::ffi::c_char; 4] = [
        b"none\0" as *const u8 as *const ::core::ffi::c_char,
        b"strict\0" as *const u8 as *const ::core::ffi::c_char,
        b"normal\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "241:27"]
    pub static mut x264_overscan_names: [*const ::core::ffi::c_char; 4] = [
        b"undef\0" as *const u8 as *const ::core::ffi::c_char,
        b"show\0" as *const u8 as *const ::core::ffi::c_char,
        b"crop\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "242:27"]
    pub static mut x264_vidformat_names: [*const ::core::ffi::c_char; 7] = [
        b"component\0" as *const u8 as *const ::core::ffi::c_char,
        b"pal\0" as *const u8 as *const ::core::ffi::c_char,
        b"ntsc\0" as *const u8 as *const ::core::ffi::c_char,
        b"secam\0" as *const u8 as *const ::core::ffi::c_char,
        b"mac\0" as *const u8 as *const ::core::ffi::c_char,
        b"undef\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "244:27"]
    pub static mut x264_colorprim_names: [*const ::core::ffi::c_char; 14] = [
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt709\0" as *const u8 as *const ::core::ffi::c_char,
        b"undef\0" as *const u8 as *const ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt470m\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt470bg\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte170m\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte240m\0" as *const u8 as *const ::core::ffi::c_char,
        b"film\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt2020\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte428\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte431\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte432\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "246:27"]
    pub static mut x264_transfer_names: [*const ::core::ffi::c_char; 20] = [
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt709\0" as *const u8 as *const ::core::ffi::c_char,
        b"undef\0" as *const u8 as *const ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt470m\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt470bg\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte170m\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte240m\0" as *const u8 as *const ::core::ffi::c_char,
        b"linear\0" as *const u8 as *const ::core::ffi::c_char,
        b"log100\0" as *const u8 as *const ::core::ffi::c_char,
        b"log316\0" as *const u8 as *const ::core::ffi::c_char,
        b"iec61966-2-4\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt1361e\0" as *const u8 as *const ::core::ffi::c_char,
        b"iec61966-2-1\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt2020-10\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt2020-12\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte2084\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte428\0" as *const u8 as *const ::core::ffi::c_char,
        b"arib-std-b67\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "248:27"]
    pub static mut x264_colmatrix_names: [*const ::core::ffi::c_char; 16] = [
        b"GBR\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt709\0" as *const u8 as *const ::core::ffi::c_char,
        b"undef\0" as *const u8 as *const ::core::ffi::c_char,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        b"fcc\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt470bg\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte170m\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte240m\0" as *const u8 as *const ::core::ffi::c_char,
        b"YCgCo\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt2020nc\0" as *const u8 as *const ::core::ffi::c_char,
        b"bt2020c\0" as *const u8 as *const ::core::ffi::c_char,
        b"smpte2085\0" as *const u8 as *const ::core::ffi::c_char,
        b"chroma-derived-nc\0" as *const u8 as *const ::core::ffi::c_char,
        b"chroma-derived-c\0" as *const u8 as *const ::core::ffi::c_char,
        b"ICtCp\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "251:27"]
    pub static mut x264_avcintra_flavor_names: [*const ::core::ffi::c_char; 3] = [
        b"panasonic\0" as *const u8 as *const ::core::ffi::c_char,
        b"sony\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "254:9"]
    pub const X264_CSP_MASK: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
    #[c2rust::src_loc = "255:9"]
    pub const X264_CSP_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "256:9"]
    pub const X264_CSP_I400: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "257:9"]
    pub const X264_CSP_I420: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "261:9"]
    pub const X264_CSP_I422: ::core::ffi::c_int = 0x6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "267:9"]
    pub const X264_CSP_I444: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;
    #[c2rust::src_loc = "269:9"]
    pub const X264_CSP_BGR: ::core::ffi::c_int = 0xe as ::core::ffi::c_int;
    #[c2rust::src_loc = "271:9"]
    pub const X264_CSP_RGB: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    #[c2rust::src_loc = "272:9"]
    pub const X264_CSP_MAX: ::core::ffi::c_int = 0x11 as ::core::ffi::c_int;
    #[c2rust::src_loc = "274:9"]
    pub const X264_CSP_HIGH_DEPTH: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "277:9"]
    pub const X264_TYPE_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "278:9"]
    pub const X264_TYPE_IDR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "279:9"]
    pub const X264_TYPE_I: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "280:9"]
    pub const X264_TYPE_P: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "281:9"]
    pub const X264_TYPE_BREF: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "282:9"]
    pub const X264_TYPE_B: ::core::ffi::c_int = 0x5 as ::core::ffi::c_int;
    #[c2rust::src_loc = "283:9"]
    pub const X264_TYPE_KEYFRAME: ::core::ffi::c_int = 0x6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "288:9"]
    pub const X264_LOG_NONE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0;
    #[c2rust::src_loc = "290:9"]
    pub const X264_LOG_WARNING: ::core::ffi::c_int = 1;
    #[c2rust::src_loc = "291:9"]
    pub const X264_LOG_INFO: ::core::ffi::c_int = 2;
    #[c2rust::src_loc = "292:9"]
    pub const X264_LOG_DEBUG: ::core::ffi::c_int = 3;
    #[c2rust::src_loc = "295:9"]
    pub const X264_THREADS_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "300:9"]
    pub const X264_NAL_HRD_VBR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "301:9"]
    pub const X264_NAL_HRD_CBR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::stdint_intn_h::{int32_t, int64_t};
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "80:16"]
        pub type x264_t;
        #[c2rust::src_loc = "651:36"]
        pub static x264_levels: [x264_level_t; 0];
        #[c2rust::src_loc = "659:10"]
        pub fn x264_param_default(_: *mut x264_param_t);
        #[c2rust::src_loc = "672:10"]
        pub fn x264_param_parse(
            _: *mut x264_param_t,
            name: *const ::core::ffi::c_char,
            value: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "679:10"]
        pub fn x264_param_cleanup(param: *mut x264_param_t);
        #[c2rust::src_loc = "723:10"]
        pub fn x264_param_default_preset(
            _: *mut x264_param_t,
            preset: *const ::core::ffi::c_char,
            tune: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "729:10"]
        pub fn x264_param_apply_fastfirstpass(_: *mut x264_param_t);
        #[c2rust::src_loc = "744:10"]
        pub fn x264_param_apply_profile(
            _: *mut x264_param_t,
            profile: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "755:27"]
        pub static x264_chroma_format: ::core::ffi::c_int;
        #[c2rust::src_loc = "914:10"]
        pub fn x264_picture_init(pic: *mut x264_picture_t);
        #[c2rust::src_loc = "939:10"]
        pub fn x264_encoder_open_165(_: *mut x264_param_t) -> *mut x264_t;
        #[c2rust::src_loc = "962:10"]
        pub fn x264_encoder_parameters(_: *mut x264_t, _: *mut x264_param_t);
        #[c2rust::src_loc = "969:10"]
        pub fn x264_encoder_headers(
            _: *mut x264_t,
            pp_nal: *mut *mut x264_nal_t,
            pi_nal: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "976:10"]
        pub fn x264_encoder_encode(
            _: *mut x264_t,
            pp_nal: *mut *mut x264_nal_t,
            pi_nal: *mut ::core::ffi::c_int,
            pic_in: *mut x264_picture_t,
            pic_out: *mut x264_picture_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "979:10"]
        pub fn x264_encoder_close(_: *mut x264_t);
        #[c2rust::src_loc = "983:10"]
        pub fn x264_encoder_delayed_frames(_: *mut x264_t) -> ::core::ffi::c_int;
    }

    #[c2rust::src_loc = "250:27"]
    pub static mut x264_nal_hrd_names: [*const ::core::ffi::c_char; 4] = [
        b"none\0" as *const u8 as *const ::core::ffi::c_char,
        b"vbr\0" as *const u8 as *const ::core::ffi::c_char,
        b"cbr\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "704:27"]
    pub static mut x264_preset_names: [*const ::core::ffi::c_char; 11] = [
        b"ultrafast\0" as *const u8 as *const ::core::ffi::c_char,
        b"superfast\0" as *const u8 as *const ::core::ffi::c_char,
        b"veryfast\0" as *const u8 as *const ::core::ffi::c_char,
        b"faster\0" as *const u8 as *const ::core::ffi::c_char,
        b"fast\0" as *const u8 as *const ::core::ffi::c_char,
        b"medium\0" as *const u8 as *const ::core::ffi::c_char,
        b"slow\0" as *const u8 as *const ::core::ffi::c_char,
        b"slower\0" as *const u8 as *const ::core::ffi::c_char,
        b"veryslow\0" as *const u8 as *const ::core::ffi::c_char,
        b"placebo\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    #[c2rust::src_loc = "716:27"]
    pub static mut x264_tune_names: [*const ::core::ffi::c_char; 9] = [
        b"film\0" as *const u8 as *const ::core::ffi::c_char,
        b"animation\0" as *const u8 as *const ::core::ffi::c_char,
        b"grain\0" as *const u8 as *const ::core::ffi::c_char,
        b"stillimage\0" as *const u8 as *const ::core::ffi::c_char,
        b"psnr\0" as *const u8 as *const ::core::ffi::c_char,
        b"ssim\0" as *const u8 as *const ::core::ffi::c_char,
        b"fastdecode\0" as *const u8 as *const ::core::ffi::c_char,
        b"zerolatency\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264cli.h:37"]
pub mod x264cli_h {
    #[c2rust::src_loc = "37:1"]
    pub type hnd_t = *mut ::core::ffi::c_void;
    #[c2rust::src_loc = "99:9"]
    pub type C2RustUnnamed_5 = ::core::ffi::c_int;
    #[c2rust::src_loc = "103:5"]
    pub const RANGE_PC: C2RustUnnamed_5 = 1;
    #[c2rust::src_loc = "102:5"]
    pub const RANGE_TV: C2RustUnnamed_5 = 0;
    #[c2rust::src_loc = "101:5"]
    pub const RANGE_AUTO: C2RustUnnamed_5 = -1;
    #[c2rust::src_loc = "33:9"]
    pub const UPDATE_INTERVAL: ::core::ffi::c_int = 250000 as ::core::ffi::c_int;
    #[inline]
    #[c2rust::src_loc = "67:1"]
    pub unsafe extern "C" fn get_filename_extension(
        mut filename: *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char {
        let mut ext: *mut ::core::ffi::c_char = filename.offset(strlen(filename) as isize);
        while *ext as ::core::ffi::c_int != '.' as i32 && ext > filename {
            ext = ext.offset(-1);
        }
        ext = ext.offset((*ext as ::core::ffi::c_int == '.' as i32) as ::core::ffi::c_int as isize);
        return ext;
    }
    use super::string_h::strlen;
    extern "C" {
        #[c2rust::src_loc = "78:1"]
        pub fn x264_cli_autocomplete(
            prev: *const ::core::ffi::c_char,
            cur: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
    extern "C" {
        #[c2rust::src_loc = "39:27"]
        pub static x264_avcintra_class_names: [*const ::core::ffi::c_char; 0];
        #[c2rust::src_loc = "40:27"]
        pub static x264_cqm_names: [*const ::core::ffi::c_char; 0];
        #[c2rust::src_loc = "41:27"]
        pub static x264_log_level_names: [*const ::core::ffi::c_char; 0];
        #[c2rust::src_loc = "42:27"]
        pub static x264_partition_names: [*const ::core::ffi::c_char; 0];
        #[c2rust::src_loc = "43:27"]
        pub static x264_pulldown_names: [*const ::core::ffi::c_char; 0];
        #[c2rust::src_loc = "44:27"]
        pub static x264_range_names: [*const ::core::ffi::c_char; 0];
        #[c2rust::src_loc = "45:27"]
        pub static x264_output_csp_names: [*const ::core::ffi::c_char; 0];
        #[c2rust::src_loc = "46:27"]
        pub static x264_valid_profile_names: [*const ::core::ffi::c_char; 0];
        #[c2rust::src_loc = "47:27"]
        pub static x264_demuxer_names: [*const ::core::ffi::c_char; 0];
        #[c2rust::src_loc = "48:27"]
        pub static x264_muxer_names: [*const ::core::ffi::c_char; 0];
    }
}
#[c2rust::header_src = "/usr/include/signal.h:37"]
pub mod signal_h {
    #[c2rust::src_loc = "72:1"]
    pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
    extern "C" {
        #[c2rust::src_loc = "88:1"]
        pub fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_ext.h:37"]
pub mod getopt_ext_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:8"]
    pub struct option {
        pub name: *const ::core::ffi::c_char,
        pub has_arg: ::core::ffi::c_int,
        pub flag: *mut ::core::ffi::c_int,
        pub val: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "62:9"]
    pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "63:9"]
    pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    extern "C" {
        #[c2rust::src_loc = "66:1"]
        pub fn getopt_long(
            ___argc: ::core::ffi::c_int,
            ___argv: *const *mut ::core::ffi::c_char,
            __shortopts: *const ::core::ffi::c_char,
            __longopts: *const option,
            __longind: *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/input/input.h:37"]
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
            unsafe extern "C" fn(*mut cli_pic_t, hnd_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub release_frame:
            Option<unsafe extern "C" fn(*mut cli_pic_t, hnd_t) -> ::core::ffi::c_int>,
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
    #[c2rust::src_loc = "114:9"]
    pub const X264_CSP_CLI_MAX: ::core::ffi::c_int = X264_CSP_MAX;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    use super::x264_h::X264_CSP_MAX;
    use super::x264cli_h::hnd_t;
    extern "C" {
        #[c2rust::src_loc = "102:26"]
        pub static raw_input: cli_input_t;
        #[c2rust::src_loc = "103:26"]
        pub static y4m_input: cli_input_t;
        #[c2rust::src_loc = "104:26"]
        pub static avs_input: cli_input_t;
        #[c2rust::src_loc = "105:26"]
        pub static thread_8_input: cli_input_t;
        #[c2rust::src_loc = "106:26"]
        pub static thread_10_input: cli_input_t;
        #[c2rust::src_loc = "107:26"]
        pub static lavf_input: cli_input_t;
        #[c2rust::src_loc = "109:26"]
        pub static timecode_input: cli_input_t;
        #[c2rust::src_loc = "127:29"]
        pub static x264_cli_csps: [x264_cli_csp_t; 0];
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/output/output.h:37"]
pub mod output_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:9"]
    pub struct cli_output_opt_t {
        pub use_dts_compress: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:9"]
    pub struct cli_output_t {
        pub open_file: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_char,
                *mut hnd_t,
                *mut cli_output_opt_t,
            ) -> ::core::ffi::c_int,
        >,
        pub set_param: Option<unsafe extern "C" fn(hnd_t, *mut x264_param_t) -> ::core::ffi::c_int>,
        pub write_headers:
            Option<unsafe extern "C" fn(hnd_t, *mut x264_nal_t) -> ::core::ffi::c_int>,
        pub write_frame: Option<
            unsafe extern "C" fn(
                hnd_t,
                *mut uint8_t,
                ::core::ffi::c_int,
                *mut x264_picture_t,
            ) -> ::core::ffi::c_int,
        >,
        pub close_file: Option<unsafe extern "C" fn(hnd_t, int64_t, int64_t) -> ::core::ffi::c_int>,
    }
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::uint8_t;
    use super::x264_h::{x264_nal_t, x264_param_t, x264_picture_t};
    use super::x264cli_h::hnd_t;
    extern "C" {
        #[c2rust::src_loc = "46:27"]
        pub static raw_output: cli_output_t;
        #[c2rust::src_loc = "47:27"]
        pub static mkv_output: cli_output_t;
        #[c2rust::src_loc = "48:27"]
        pub static mp4_output: cli_output_t;
        #[c2rust::src_loc = "49:27"]
        pub static flv_output: cli_output_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/filters/video/video.h:0"]
pub mod video_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "34:8"]
    pub struct cli_vid_filter_t {
        pub name: *const ::core::ffi::c_char,
        pub help: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
        pub init: Option<
            unsafe extern "C" fn(
                *mut hnd_t,
                *mut cli_vid_filter_t,
                *mut video_info_t,
                *mut x264_param_t,
                *mut ::core::ffi::c_char,
            ) -> ::core::ffi::c_int,
        >,
        pub get_frame: Option<
            unsafe extern "C" fn(hnd_t, *mut cli_pic_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub release_frame: Option<
            unsafe extern "C" fn(hnd_t, *mut cli_pic_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub free: Option<unsafe extern "C" fn(hnd_t) -> ()>,
        pub next: *mut cli_vid_filter_t,
    }
    use super::input_h::{cli_pic_t, video_info_t};
    use super::x264_h::x264_param_t;
    use super::x264cli_h::hnd_t;
    extern "C" {
        #[c2rust::src_loc = "58:1"]
        pub fn x264_register_vid_filters();
        #[c2rust::src_loc = "59:1"]
        pub fn x264_vid_filter_help(longhelp: ::core::ffi::c_int);
        #[c2rust::src_loc = "60:1"]
        pub fn x264_init_vid_filter(
            name: *const ::core::ffi::c_char,
            handle: *mut hnd_t,
            filter_0: *mut cli_vid_filter_t,
            info: *mut video_info_t,
            param: *mut x264_param_t,
            opt_string: *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libavutil/pixfmt.h:45"]
pub mod pixfmt_h {
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
    #[c2rust::src_loc = "80:5"]
    pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
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
#[c2rust::header_src = "/usr/include/sys/stat.h:37"]
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "227:1"]
        pub fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "230:1"]
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:37"]
pub mod stdlib_h {
    #[inline]
    #[c2rust::src_loc = "480:1"]
    pub unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
        return strtol(
            __nptr,
            NULL as *mut *mut ::core::ffi::c_char,
            10 as ::core::ffi::c_int,
        ) as ::core::ffi::c_int;
    }
    use super::__stddef_null_h::NULL;
    extern "C" {
        #[c2rust::src_loc = "215:1"]
        pub fn strtol(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
        #[c2rust::src_loc = "219:1"]
        pub fn strtoul(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "756:1"]
        pub fn exit(__status: ::core::ffi::c_int) -> !;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:37"]
pub mod osdep_h {
    #[inline]
    #[c2rust::src_loc = "261:1"]
    pub unsafe extern "C" fn x264_is_regular_file_path(
        mut filename: *const ::core::ffi::c_char,
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
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        if stat(filename, &mut file_stat) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return (file_stat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
            as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "270:1"]
    pub unsafe extern "C" fn x264_is_regular_file(mut filehandle: *mut FILE) -> ::core::ffi::c_int {
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
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        if fstat(fileno(filehandle), &mut file_stat) != 0 {
            return 1 as ::core::ffi::c_int;
        }
        return (file_stat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
            as ::core::ffi::c_int;
    }
    use super::bits_stat_h::__S_IFMT;
    use super::stat_h::{fstat, stat};
    use super::stdint_intn_h::int64_t;
    use super::stdio_h::fileno;
    use super::struct_stat_h::stat;
    use super::struct_timespec_h::timespec;
    use super::types_h::__mode_t;
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "206:10"]
        pub fn x264_mdate() -> int64_t;
    }
}
#[c2rust::header_src = "/usr/include/string.h:37"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "61:1"]
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "141:1"]
        pub fn strcpy(
            __dest: *mut ::core::ffi::c_char,
            __src: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "156:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "293:1"]
        pub fn strcspn(
            __s: *const ::core::ffi::c_char,
            __reject: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
    extern "C" {
        #[c2rust::src_loc = "159:1"]
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "246:1"]
        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:37"]
pub mod base_h {
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "271:10"]
        pub fn x264_reduce_fraction(n: *mut uint32_t, d: *mut uint32_t);
    }
}
#[c2rust::header_src = "/usr/include/libavutil/pixdesc.h:58"]
pub mod pixdesc_h {
    use super::pixfmt_h::AVPixelFormat;
    extern "C" {
        #[c2rust::src_loc = "313:1"]
        pub fn av_get_pix_fmt_name(pix_fmt: AVPixelFormat) -> *const ::core::ffi::c_char;
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "69:16"]
    pub struct AVPixFmtDescriptor {
        pub name: *const ::core::ffi::c_char,
        pub nb_components: uint8_t,
        pub log2_chroma_w: uint8_t,
        pub log2_chroma_h: uint8_t,
        pub flags: uint64_t,
        pub comp: [AVComponentDescriptor; 4],
        pub alias: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:16"]
    pub struct AVComponentDescriptor {
        pub plane: ::core::ffi::c_int,
        pub step: ::core::ffi::c_int,
        pub offset: ::core::ffi::c_int,
        pub shift: ::core::ffi::c_int,
        pub depth: ::core::ffi::c_int,
    }
    use super::stdint_uintn_h::{uint64_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "195:1"]
        pub fn av_pix_fmt_desc_next(prev: *const AVPixFmtDescriptor) -> *const AVPixFmtDescriptor;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:37"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "222:1"]
        pub fn fmod(
            __x: ::core::ffi::c_double,
            __y: ::core::ffi::c_double,
        ) -> ::core::ffi::c_double;
    }
}
#[c2rust::header_src = "/usr/include/strings.h:37"]
pub mod strings_h {
    extern "C" {
        #[c2rust::src_loc = "116:1"]
        pub fn strcasecmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "120:1"]
        pub fn strncasecmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/cpu.h:37"]
pub mod cpu_h {
    extern "C" {
        #[c2rust::src_loc = "30:10"]
        pub fn x264_cpu_num_processors() -> ::core::ffi::c_int;
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:9"]
    pub struct x264_cpu_name_t {
        pub name: *const ::core::ffi::c_char,
        pub flags: uint32_t,
    }
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "54:39"]
        pub static x264_cpu_names: [x264_cpu_name_t; 0];
    }
}
#[c2rust::header_src = "/usr/include/bits/getopt_core.h:37"]
pub mod getopt_core_h {
    extern "C" {
        #[c2rust::src_loc = "36:14"]
        pub static mut optarg: *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "50:12"]
        pub static mut optind: ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libswscale/swscale.h:58"]
pub mod swscale_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn swscale_license() -> *const ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/usr/include/lsmash.h:58"]
pub mod lsmash_h {
    #[c2rust::src_loc = "49:9"]
    pub const LSMASH_VERSION_MAJOR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "50:9"]
    pub const LSMASH_VERSION_MINOR: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
    #[c2rust::src_loc = "51:9"]
    pub const LSMASH_VERSION_MICRO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:58"]
pub mod bits_stat_h {
    #[c2rust::src_loc = "29:9"]
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:58"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
    #[c2rust::src_loc = "26:9"]
    pub const NULL_0: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/config.h:58"]
pub mod config_h {
    #[c2rust::src_loc = "17:9"]
    pub const HAVE_LAVF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "25:9"]
    pub const HAVE_GPL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "36:9"]
    pub const HAVE_FFMS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264_config.h:58"]
pub mod x264_config_h {
    #[c2rust::src_loc = "4:9"]
    pub const X264_CHROMA_FORMAT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "7:9"]
    pub const X264_VERSION: [::core::ffi::c_char; 16] = unsafe {
        ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b" r3223M 0480cb0\0")
    };
}
#[c2rust::header_src = "/usr/include/libswscale/version.h:58"]
pub mod version_h {
    #[c2rust::src_loc = "31:9"]
    pub const LIBSWSCALE_VERSION_MINOR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "32:9"]
    pub const LIBSWSCALE_VERSION_MICRO: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/bits/signum-generic.h:58"]
pub mod signum_generic_h {
    #[c2rust::src_loc = "48:9"]
    pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/libswscale/version_major.h:58"]
pub mod version_major_h {
    #[c2rust::src_loc = "27:9"]
    pub const LIBSWSCALE_VERSION_MAJOR: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/libavformat/version_major.h:58"]
pub mod libavformat_version_major_h {
    #[c2rust::src_loc = "32:9"]
    pub const LIBAVFORMAT_VERSION_MAJOR: ::core::ffi::c_int = 62 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/libavformat/version.h:58"]
pub mod libavformat_version_h {
    #[c2rust::src_loc = "34:9"]
    pub const LIBAVFORMAT_VERSION_MINOR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "35:9"]
    pub const LIBAVFORMAT_VERSION_MICRO: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/libavformat/avformat.h:31"]
pub mod avformat_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "544:16"]
    pub struct AVInputFormat {
        pub name: *const ::core::ffi::c_char,
        pub long_name: *const ::core::ffi::c_char,
        pub flags: ::core::ffi::c_int,
        pub extensions: *const ::core::ffi::c_char,
        pub codec_tag: *const *const AVCodecTag,
        pub priv_class: *const AVClass,
        pub mime_type: *const ::core::ffi::c_char,
    }
    use super::log_h::AVClass;
    extern "C" {
        #[c2rust::src_loc = "446:8"]
        pub type AVCodecTag;
        #[c2rust::src_loc = "1959:1"]
        pub fn av_demuxer_iterate(opaque: *mut *mut ::core::ffi::c_void) -> *const AVInputFormat;
    }
}
#[c2rust::header_src = "/usr/include/libavutil/log.h:26"]
pub mod log_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:16"]
    pub struct AVClass {
        pub class_name: *const ::core::ffi::c_char,
        pub item_name:
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const ::core::ffi::c_char>,
        pub option: *const AVOption,
        pub version: ::core::ffi::c_int,
        pub log_level_offset_offset: ::core::ffi::c_int,
        pub parent_log_context_offset: ::core::ffi::c_int,
        pub category: AVClassCategory,
        pub get_category: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> AVClassCategory>,
        pub query_ranges: Option<
            unsafe extern "C" fn(
                *mut *mut AVOptionRanges,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub child_next: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub child_class_iterate:
            Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass>,
        pub state_flags_offset: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "28:9"]
    pub type AVClassCategory = ::core::ffi::c_uint;
    #[c2rust::src_loc = "47:5"]
    pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
    #[c2rust::src_loc = "46:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
    #[c2rust::src_loc = "45:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
    #[c2rust::src_loc = "44:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
    #[c2rust::src_loc = "43:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
    #[c2rust::src_loc = "42:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
    #[c2rust::src_loc = "41:5"]
    pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
    #[c2rust::src_loc = "40:5"]
    pub const AV_CLASS_CATEGORY_HWDEVICE: AVClassCategory = 11;
    #[c2rust::src_loc = "39:5"]
    pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
    #[c2rust::src_loc = "38:5"]
    pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
    #[c2rust::src_loc = "37:5"]
    pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
    #[c2rust::src_loc = "36:5"]
    pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
    #[c2rust::src_loc = "35:5"]
    pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
    #[c2rust::src_loc = "34:5"]
    pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
    #[c2rust::src_loc = "33:5"]
    pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
    #[c2rust::src_loc = "32:5"]
    pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
    #[c2rust::src_loc = "31:5"]
    pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
    #[c2rust::src_loc = "30:5"]
    pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
    #[c2rust::src_loc = "29:5"]
    pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
    extern "C" {
        #[c2rust::src_loc = "69:8"]
        pub type AVOptionRanges;
        #[c2rust::src_loc = "96:18"]
        pub type AVOption;
    }
}
#[c2rust::header_src = "/usr/include/bits/stdio.h:26"]
pub mod bits_stdio_h {
    #[inline]
    #[c2rust::src_loc = "81:1"]
    pub unsafe extern "C" fn putchar(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
        return putc(__c, stdout);
    }
    use super::stdio_h::{putc, stdout};
}
