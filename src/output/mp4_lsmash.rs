use ::c2rust_bitfields;
#[c2rust::header_src = "internal:0"]
pub mod internal {
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
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:30"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:30"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = i16;
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:30"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:30"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:30"]
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
#[c2rust::header_src = "/usr/include/bits/struct_stat.h:30"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:30"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:30"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "25:1"]
    pub type uint16_t = __uint16_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t, __uint64_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:30"]
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
            unsafe extern "C" fn(
                *mut x264_t,
                *mut x264_nal_t,
                *mut ::core::ffi::c_void,
            ) -> (),
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
        pub quant_offsets_free: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
        >,
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
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::internal::__va_list_tag;
    use super::stdint_intn_h::int64_t;
    extern "C" {
        #[c2rust::src_loc = "80:16"]
        pub type x264_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264cli.h:30"]
pub mod x264cli_h {
    #[c2rust::src_loc = "37:1"]
    pub type hnd_t = *mut ::core::ffi::c_void;
    extern "C" {
        #[c2rust::src_loc = "76:1"]
        pub fn x264_cli_log(
            name: *const ::core::ffi::c_char,
            i_level: ::core::ffi::c_int,
            fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/output/output.h:30"]
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
        pub set_param: Option<
            unsafe extern "C" fn(hnd_t, *mut x264_param_t) -> ::core::ffi::c_int,
        >,
        pub write_headers: Option<
            unsafe extern "C" fn(hnd_t, *mut x264_nal_t) -> ::core::ffi::c_int,
        >,
        pub write_frame: Option<
            unsafe extern "C" fn(
                hnd_t,
                *mut uint8_t,
                ::core::ffi::c_int,
                *mut x264_picture_t,
            ) -> ::core::ffi::c_int,
        >,
        pub close_file: Option<
            unsafe extern "C" fn(hnd_t, int64_t, int64_t) -> ::core::ffi::c_int,
        >,
    }
    use super::x264cli_h::hnd_t;
    use super::x264_h::{x264_param_t, x264_nal_t, x264_picture_t};
    use super::stdint_uintn_h::uint8_t;
    use super::stdint_intn_h::int64_t;
}
#[c2rust::header_src = "/usr/include/lsmash.h:30"]
pub mod lsmash_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "204:9"]
    pub struct lsmash_file_parameters_t {
        pub mode: lsmash_file_mode,
        pub opaque: *mut ::core::ffi::c_void,
        pub read: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub write: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut uint8_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub seek: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                int64_t,
                ::core::ffi::c_int,
            ) -> int64_t,
        >,
        pub major_brand: lsmash_brand_type,
        pub brands: *mut lsmash_brand_type,
        pub brand_count: uint32_t,
        pub minor_version: uint32_t,
        pub max_chunk_duration: ::core::ffi::c_double,
        pub max_async_tolerance: ::core::ffi::c_double,
        pub max_chunk_size: uint64_t,
        pub max_read_size: uint64_t,
    }
    #[c2rust::src_loc = "114:9"]
    pub type lsmash_brand_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "201:5"]
    pub const ISOM_BRAND_TYPE_SSSS: lsmash_brand_type = 1936946035;
    #[c2rust::src_loc = "200:5"]
    pub const ISOM_BRAND_TYPE_SISX: lsmash_brand_type = 1936290680;
    #[c2rust::src_loc = "199:5"]
    pub const ISOM_BRAND_TYPE_SIMS: lsmash_brand_type = 1936289139;
    #[c2rust::src_loc = "198:5"]
    pub const ISOM_BRAND_TYPE_SDV: lsmash_brand_type = 1935963680;
    #[c2rust::src_loc = "197:5"]
    pub const ISOM_BRAND_TYPE_RISX: lsmash_brand_type = 1919513464;
    #[c2rust::src_loc = "196:5"]
    pub const ISOM_BRAND_TYPE_QT: lsmash_brand_type = 1903435808;
    #[c2rust::src_loc = "195:5"]
    pub const ISOM_BRAND_TYPE_PNVI: lsmash_brand_type = 1886287465;
    #[c2rust::src_loc = "194:5"]
    pub const ISOM_BRAND_TYPE_PIFF: lsmash_brand_type = 1885955686;
    #[c2rust::src_loc = "193:5"]
    pub const ISOM_BRAND_TYPE_PANA: lsmash_brand_type = 1885433441;
    #[c2rust::src_loc = "192:5"]
    pub const ISOM_BRAND_TYPE_OPX2: lsmash_brand_type = 1869641778;
    #[c2rust::src_loc = "191:5"]
    pub const ISOM_BRAND_TYPE_OPF2: lsmash_brand_type = 1869637170;
    #[c2rust::src_loc = "190:5"]
    pub const ISOM_BRAND_TYPE_ODCF: lsmash_brand_type = 1868850022;
    #[c2rust::src_loc = "189:5"]
    pub const ISOM_BRAND_TYPE_NIKO: lsmash_brand_type = 1852402543;
    #[c2rust::src_loc = "188:5"]
    pub const ISOM_BRAND_TYPE_MSIX: lsmash_brand_type = 1836280184;
    #[c2rust::src_loc = "187:5"]
    pub const ISOM_BRAND_TYPE_MSDH: lsmash_brand_type = 1836278888;
    #[c2rust::src_loc = "186:5"]
    pub const ISOM_BRAND_TYPE_MP71: lsmash_brand_type = 1836070705;
    #[c2rust::src_loc = "185:5"]
    pub const ISOM_BRAND_TYPE_MP42: lsmash_brand_type = 1836069938;
    #[c2rust::src_loc = "184:5"]
    pub const ISOM_BRAND_TYPE_MP41: lsmash_brand_type = 1836069937;
    #[c2rust::src_loc = "183:5"]
    pub const ISOM_BRAND_TYPE_MP21: lsmash_brand_type = 1836069425;
    #[c2rust::src_loc = "182:5"]
    pub const ISOM_BRAND_TYPE_MJP2: lsmash_brand_type = 1835692082;
    #[c2rust::src_loc = "181:5"]
    pub const ISOM_BRAND_TYPE_MJ2S: lsmash_brand_type = 1835676275;
    #[c2rust::src_loc = "180:5"]
    pub const ISOM_BRAND_TYPE_LMSG: lsmash_brand_type = 1819112295;
    #[c2rust::src_loc = "179:5"]
    pub const ISOM_BRAND_TYPE_JPSI: lsmash_brand_type = 1785754473;
    #[c2rust::src_loc = "178:5"]
    pub const ISOM_BRAND_TYPE_ISOM: lsmash_brand_type = 1769172845;
    #[c2rust::src_loc = "177:5"]
    pub const ISOM_BRAND_TYPE_ISO7: lsmash_brand_type = 1769172791;
    #[c2rust::src_loc = "176:5"]
    pub const ISOM_BRAND_TYPE_ISO6: lsmash_brand_type = 1769172790;
    #[c2rust::src_loc = "175:5"]
    pub const ISOM_BRAND_TYPE_ISO5: lsmash_brand_type = 1769172789;
    #[c2rust::src_loc = "174:5"]
    pub const ISOM_BRAND_TYPE_ISO4: lsmash_brand_type = 1769172788;
    #[c2rust::src_loc = "173:5"]
    pub const ISOM_BRAND_TYPE_ISO3: lsmash_brand_type = 1769172787;
    #[c2rust::src_loc = "172:5"]
    pub const ISOM_BRAND_TYPE_ISO2: lsmash_brand_type = 1769172786;
    #[c2rust::src_loc = "171:5"]
    pub const ISOM_BRAND_TYPE_ISC2: lsmash_brand_type = 1769169714;
    #[c2rust::src_loc = "170:5"]
    pub const ISOM_BRAND_TYPE_IFRM: lsmash_brand_type = 1768321645;
    #[c2rust::src_loc = "169:5"]
    pub const ISOM_BRAND_TYPE_DVT1: lsmash_brand_type = 1685484593;
    #[c2rust::src_loc = "168:5"]
    pub const ISOM_BRAND_TYPE_DVR1: lsmash_brand_type = 1685484081;
    #[c2rust::src_loc = "167:5"]
    pub const ISOM_BRAND_TYPE_DV3B: lsmash_brand_type = 1685468002;
    #[c2rust::src_loc = "166:5"]
    pub const ISOM_BRAND_TYPE_DV3A: lsmash_brand_type = 1685468001;
    #[c2rust::src_loc = "165:5"]
    pub const ISOM_BRAND_TYPE_DV2B: lsmash_brand_type = 1685467746;
    #[c2rust::src_loc = "164:5"]
    pub const ISOM_BRAND_TYPE_DV2A: lsmash_brand_type = 1685467745;
    #[c2rust::src_loc = "163:5"]
    pub const ISOM_BRAND_TYPE_DV1B: lsmash_brand_type = 1685467490;
    #[c2rust::src_loc = "162:5"]
    pub const ISOM_BRAND_TYPE_DV1A: lsmash_brand_type = 1685467489;
    #[c2rust::src_loc = "161:5"]
    pub const ISOM_BRAND_TYPE_DSMS: lsmash_brand_type = 1685286259;
    #[c2rust::src_loc = "160:5"]
    pub const ISOM_BRAND_TYPE_DMB1: lsmash_brand_type = 1684890161;
    #[c2rust::src_loc = "159:5"]
    pub const ISOM_BRAND_TYPE_DBY1: lsmash_brand_type = 1684175153;
    #[c2rust::src_loc = "158:5"]
    pub const ISOM_BRAND_TYPE_DASH: lsmash_brand_type = 1684108136;
    #[c2rust::src_loc = "157:5"]
    pub const ISOM_BRAND_TYPE_DA3B: lsmash_brand_type = 1684091746;
    #[c2rust::src_loc = "156:5"]
    pub const ISOM_BRAND_TYPE_DA3A: lsmash_brand_type = 1684091745;
    #[c2rust::src_loc = "155:5"]
    pub const ISOM_BRAND_TYPE_DA2B: lsmash_brand_type = 1684091490;
    #[c2rust::src_loc = "154:5"]
    pub const ISOM_BRAND_TYPE_DA2A: lsmash_brand_type = 1684091489;
    #[c2rust::src_loc = "153:5"]
    pub const ISOM_BRAND_TYPE_DA1B: lsmash_brand_type = 1684091234;
    #[c2rust::src_loc = "152:5"]
    pub const ISOM_BRAND_TYPE_DA1A: lsmash_brand_type = 1684091233;
    #[c2rust::src_loc = "151:5"]
    pub const ISOM_BRAND_TYPE_DA0B: lsmash_brand_type = 1684090978;
    #[c2rust::src_loc = "150:5"]
    pub const ISOM_BRAND_TYPE_DA0A: lsmash_brand_type = 1684090977;
    #[c2rust::src_loc = "149:5"]
    pub const ISOM_BRAND_TYPE_CCFF: lsmash_brand_type = 1667458662;
    #[c2rust::src_loc = "148:5"]
    pub const ISOM_BRAND_TYPE_CAQV: lsmash_brand_type = 1667330422;
    #[c2rust::src_loc = "147:5"]
    pub const ISOM_BRAND_TYPE_BBXM: lsmash_brand_type = 1650620525;
    #[c2rust::src_loc = "146:5"]
    pub const ISOM_BRAND_TYPE_AVC1: lsmash_brand_type = 1635148593;
    #[c2rust::src_loc = "145:5"]
    pub const ISOM_BRAND_TYPE_ROSS: lsmash_brand_type = 1380930387;
    #[c2rust::src_loc = "144:5"]
    pub const ISOM_BRAND_TYPE_MPPI: lsmash_brand_type = 1297109065;
    #[c2rust::src_loc = "143:5"]
    pub const ISOM_BRAND_TYPE_MFSM: lsmash_brand_type = 1296454477;
    #[c2rust::src_loc = "142:5"]
    pub const ISOM_BRAND_TYPE_M4V: lsmash_brand_type = 1295275552;
    #[c2rust::src_loc = "141:5"]
    pub const ISOM_BRAND_TYPE_M4P: lsmash_brand_type = 1295274016;
    #[c2rust::src_loc = "140:5"]
    pub const ISOM_BRAND_TYPE_M4B: lsmash_brand_type = 1295270432;
    #[c2rust::src_loc = "139:5"]
    pub const ISOM_BRAND_TYPE_M4A: lsmash_brand_type = 1295270176;
    #[c2rust::src_loc = "138:5"]
    pub const ISOM_BRAND_TYPE_LCAG: lsmash_brand_type = 1279476039;
    #[c2rust::src_loc = "137:5"]
    pub const ISOM_BRAND_TYPE_CDES: lsmash_brand_type = 1128555891;
    #[c2rust::src_loc = "136:5"]
    pub const ISOM_BRAND_TYPE_CAEP: lsmash_brand_type = 1128351056;
    #[c2rust::src_loc = "135:5"]
    pub const ISOM_BRAND_TYPE_ARRI: lsmash_brand_type = 1095914057;
    #[c2rust::src_loc = "134:5"]
    pub const ISOM_BRAND_TYPE_3GT9: lsmash_brand_type = 862417977;
    #[c2rust::src_loc = "133:5"]
    pub const ISOM_BRAND_TYPE_3GS9: lsmash_brand_type = 862417721;
    #[c2rust::src_loc = "132:5"]
    pub const ISOM_BRAND_TYPE_3GS6: lsmash_brand_type = 862417718;
    #[c2rust::src_loc = "131:5"]
    pub const ISOM_BRAND_TYPE_3GR9: lsmash_brand_type = 862417465;
    #[c2rust::src_loc = "130:5"]
    pub const ISOM_BRAND_TYPE_3GR6: lsmash_brand_type = 862417462;
    #[c2rust::src_loc = "129:5"]
    pub const ISOM_BRAND_TYPE_3GP9: lsmash_brand_type = 862416953;
    #[c2rust::src_loc = "128:5"]
    pub const ISOM_BRAND_TYPE_3GP8: lsmash_brand_type = 862416952;
    #[c2rust::src_loc = "127:5"]
    pub const ISOM_BRAND_TYPE_3GP7: lsmash_brand_type = 862416951;
    #[c2rust::src_loc = "126:5"]
    pub const ISOM_BRAND_TYPE_3GP6: lsmash_brand_type = 862416950;
    #[c2rust::src_loc = "125:5"]
    pub const ISOM_BRAND_TYPE_3GP5: lsmash_brand_type = 862416949;
    #[c2rust::src_loc = "124:5"]
    pub const ISOM_BRAND_TYPE_3GP4: lsmash_brand_type = 862416948;
    #[c2rust::src_loc = "123:5"]
    pub const ISOM_BRAND_TYPE_3GM9: lsmash_brand_type = 862416185;
    #[c2rust::src_loc = "122:5"]
    pub const ISOM_BRAND_TYPE_3GH9: lsmash_brand_type = 862414905;
    #[c2rust::src_loc = "121:5"]
    pub const ISOM_BRAND_TYPE_3GG9: lsmash_brand_type = 862414649;
    #[c2rust::src_loc = "120:5"]
    pub const ISOM_BRAND_TYPE_3GG6: lsmash_brand_type = 862414646;
    #[c2rust::src_loc = "119:5"]
    pub const ISOM_BRAND_TYPE_3GF9: lsmash_brand_type = 862414393;
    #[c2rust::src_loc = "118:5"]
    pub const ISOM_BRAND_TYPE_3GE9: lsmash_brand_type = 862414137;
    #[c2rust::src_loc = "117:5"]
    pub const ISOM_BRAND_TYPE_3GE6: lsmash_brand_type = 862414134;
    #[c2rust::src_loc = "116:5"]
    pub const ISOM_BRAND_TYPE_3G2A: lsmash_brand_type = 862401121;
    #[c2rust::src_loc = "100:9"]
    pub type lsmash_file_mode = ::core::ffi::c_uint;
    #[c2rust::src_loc = "111:5"]
    pub const LSMASH_FILE_MODE_WRITE_FRAGMENTED: lsmash_file_mode = 5;
    #[c2rust::src_loc = "110:5"]
    pub const LSMASH_FILE_MODE_SEGMENT: lsmash_file_mode = 256;
    #[c2rust::src_loc = "109:5"]
    pub const LSMASH_FILE_MODE_INDEX: lsmash_file_mode = 128;
    #[c2rust::src_loc = "108:5"]
    pub const LSMASH_FILE_MODE_MEDIA: lsmash_file_mode = 64;
    #[c2rust::src_loc = "107:5"]
    pub const LSMASH_FILE_MODE_INITIALIZATION: lsmash_file_mode = 32;
    #[c2rust::src_loc = "106:5"]
    pub const LSMASH_FILE_MODE_BOX: lsmash_file_mode = 16;
    #[c2rust::src_loc = "105:5"]
    pub const LSMASH_FILE_MODE_DUMP: lsmash_file_mode = 8;
    #[c2rust::src_loc = "104:5"]
    pub const LSMASH_FILE_MODE_FRAGMENTED: lsmash_file_mode = 4;
    #[c2rust::src_loc = "103:5"]
    pub const LSMASH_FILE_MODE_READ: lsmash_file_mode = 2;
    #[c2rust::src_loc = "102:5"]
    pub const LSMASH_FILE_MODE_WRITE: lsmash_file_mode = 1;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1373:9"]
    pub struct lsmash_video_summary_t {
        pub summary_type: lsmash_summary_type,
        pub sample_type: lsmash_codec_type_t,
        pub opaque: *mut lsmash_codec_specific_list_t,
        pub max_au_length: uint32_t,
        pub data_ref_index: uint32_t,
        pub timescale: uint32_t,
        pub timebase: uint32_t,
        pub vfr: uint8_t,
        pub sample_per_field: uint8_t,
        pub width: uint32_t,
        pub height: uint32_t,
        pub compressorname: [::core::ffi::c_char; 33],
        pub depth: lsmash_video_depth,
        pub clap: lsmash_clap_t,
        pub par_h: uint32_t,
        pub par_v: uint32_t,
        pub color: C2RustUnnamed_5,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1392:5"]
    pub struct C2RustUnnamed_5 {
        pub primaries_index: uint16_t,
        pub transfer_index: uint16_t,
        pub matrix_index: uint16_t,
        pub full_range: uint8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1210:9"]
    pub struct lsmash_clap_t {
        pub width: lsmash_rational_u32_t,
        pub height: lsmash_rational_u32_t,
        pub horizontal_offset: lsmash_rational_s32_t,
        pub vertical_offset: lsmash_rational_s32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "374:9"]
    pub struct lsmash_rational_s32_t {
        pub n: int32_t,
        pub d: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "368:9"]
    pub struct lsmash_rational_u32_t {
        pub n: uint32_t,
        pub d: uint32_t,
    }
    #[c2rust::src_loc = "1227:9"]
    pub type lsmash_video_depth = ::core::ffi::c_uint;
    #[c2rust::src_loc = "1253:5"]
    pub const QT_VIDEO_DEPTH_32ARGB: lsmash_video_depth = 32;
    #[c2rust::src_loc = "1252:5"]
    pub const QT_VIDEO_DEPTH_24RGB: lsmash_video_depth = 24;
    #[c2rust::src_loc = "1251:5"]
    pub const QT_VIDEO_DEPTH_555RGB: lsmash_video_depth = 16;
    #[c2rust::src_loc = "1248:5"]
    pub const QT_VIDEO_DEPTH_GRAYSCALE_8: lsmash_video_depth = 40;
    #[c2rust::src_loc = "1247:5"]
    pub const QT_VIDEO_DEPTH_GRAYSCALE_4: lsmash_video_depth = 36;
    #[c2rust::src_loc = "1246:5"]
    pub const QT_VIDEO_DEPTH_GRAYSCALE_2: lsmash_video_depth = 34;
    #[c2rust::src_loc = "1245:5"]
    pub const QT_VIDEO_DEPTH_GRAYSCALE_1: lsmash_video_depth = 33;
    #[c2rust::src_loc = "1244:5"]
    pub const QT_VIDEO_DEPTH_COLOR_32: lsmash_video_depth = 32;
    #[c2rust::src_loc = "1243:5"]
    pub const QT_VIDEO_DEPTH_COLOR_24: lsmash_video_depth = 24;
    #[c2rust::src_loc = "1242:5"]
    pub const QT_VIDEO_DEPTH_COLOR_16: lsmash_video_depth = 16;
    #[c2rust::src_loc = "1241:5"]
    pub const QT_VIDEO_DEPTH_COLOR_8: lsmash_video_depth = 8;
    #[c2rust::src_loc = "1240:5"]
    pub const QT_VIDEO_DEPTH_COLOR_4: lsmash_video_depth = 4;
    #[c2rust::src_loc = "1239:5"]
    pub const QT_VIDEO_DEPTH_COLOR_2: lsmash_video_depth = 2;
    #[c2rust::src_loc = "1238:5"]
    pub const QT_VIDEO_DEPTH_COLOR_1: lsmash_video_depth = 1;
    #[c2rust::src_loc = "1234:5"]
    pub const AVC_DEPTH_WITH_ALPHA: lsmash_video_depth = 32;
    #[c2rust::src_loc = "1233:5"]
    pub const AVC_DEPTH_GRAYSCALE_WITH_NO_ALPHA: lsmash_video_depth = 40;
    #[c2rust::src_loc = "1232:5"]
    pub const AVC_DEPTH_COLOR_WITH_NO_ALPHA: lsmash_video_depth = 24;
    #[c2rust::src_loc = "1229:5"]
    pub const ISOM_DEPTH_TEMPLATE: lsmash_video_depth = 24;
    #[c2rust::src_loc = "967:1"]
    pub type lsmash_codec_specific_list_t = lsmash_codec_specific_list_tag;
    #[c2rust::src_loc = "478:1"]
    pub type lsmash_box_type_t = lsmash_codec_type_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "478:9"]
    pub struct lsmash_codec_type_t {
        pub fourcc: lsmash_compact_box_type_t,
        pub user: lsmash_extended_box_type_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "465:9"]
    pub struct lsmash_extended_box_type_t {
        pub fourcc: uint32_t,
        pub id: [uint8_t; 12],
    }
    #[c2rust::src_loc = "462:1"]
    pub type lsmash_compact_box_type_t = uint32_t;
    #[c2rust::src_loc = "903:9"]
    pub type lsmash_summary_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "907:5"]
    pub const LSMASH_SUMMARY_TYPE_AUDIO: lsmash_summary_type = 2;
    #[c2rust::src_loc = "906:5"]
    pub const LSMASH_SUMMARY_TYPE_VIDEO: lsmash_summary_type = 1;
    #[c2rust::src_loc = "905:5"]
    pub const LSMASH_SUMMARY_TYPE_UNKNOWN: lsmash_summary_type = 0;
    #[c2rust::src_loc = "77:1"]
    pub type lsmash_root_t = lsmash_root_tag;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "987:9"]
    pub struct lsmash_summary_t {
        pub summary_type: lsmash_summary_type,
        pub sample_type: lsmash_codec_type_t,
        pub opaque: *mut lsmash_codec_specific_list_t,
        pub max_au_length: uint32_t,
        pub data_ref_index: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "261:9"]
    pub struct lsmash_adhoc_remux_t {
        pub buffer_size: uint64_t,
        pub func: lsmash_adhoc_remux_callback,
        pub param: *mut ::core::ffi::c_void,
    }
    #[c2rust::src_loc = "260:1"]
    pub type lsmash_adhoc_remux_callback = Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            uint64_t,
            uint64_t,
        ) -> ::core::ffi::c_int,
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2044:9"]
    pub struct lsmash_edit_t {
        pub duration: uint64_t,
        pub start_time: int64_t,
        pub rate: int32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1542:9"]
    pub struct lsmash_sample_t {
        pub length: uint32_t,
        pub data: *mut uint8_t,
        pub dts: uint64_t,
        pub cts: uint64_t,
        pub pos: uint64_t,
        pub index: uint32_t,
        pub prop: lsmash_sample_property_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1528:9"]
    pub struct lsmash_sample_property_t {
        pub ra_flags: lsmash_random_access_flag,
        pub post_roll: lsmash_post_roll_t,
        pub pre_roll: lsmash_pre_roll_t,
        pub allow_earlier: uint8_t,
        pub leading: uint8_t,
        pub independent: uint8_t,
        pub disposable: uint8_t,
        pub redundant: uint8_t,
        pub reserved: [uint8_t; 3],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1513:9"]
    pub struct lsmash_pre_roll_t {
        pub distance: uint32_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1499:9"]
    pub struct lsmash_post_roll_t {
        pub identifier: uint32_t,
        pub complete: uint32_t,
    }
    #[c2rust::src_loc = "1444:9"]
    pub type lsmash_random_access_flag = ::core::ffi::c_uint;
    #[c2rust::src_loc = "1488:5"]
    pub const QT_SAMPLE_RANDOM_ACCESS_FLAG_OPEN_RAP: lsmash_random_access_flag = 20;
    #[c2rust::src_loc = "1485:5"]
    pub const QT_SAMPLE_RANDOM_ACCESS_FLAG_CLOSED_RAP: lsmash_random_access_flag = 12;
    #[c2rust::src_loc = "1483:5"]
    pub const QT_SAMPLE_RANDOM_ACCESS_FLAG_OPEN: lsmash_random_access_flag = 16;
    #[c2rust::src_loc = "1482:5"]
    pub const QT_SAMPLE_RANDOM_ACCESS_FLAG_CLOSED: lsmash_random_access_flag = 8;
    #[c2rust::src_loc = "1481:5"]
    pub const QT_SAMPLE_RANDOM_ACCESS_FLAG_RAP: lsmash_random_access_flag = 4;
    #[c2rust::src_loc = "1476:5"]
    pub const QT_SAMPLE_RANDOM_ACCESS_FLAG_PARTIAL_SYNC: lsmash_random_access_flag = 2;
    #[c2rust::src_loc = "1475:5"]
    pub const QT_SAMPLE_RANDOM_ACCESS_FLAG_SYNC: lsmash_random_access_flag = 1;
    #[c2rust::src_loc = "1474:5"]
    pub const QT_SAMPLE_RANDOM_ACCESS_FLAG_NONE: lsmash_random_access_flag = 0;
    #[c2rust::src_loc = "1469:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_PRE_ROLL_END: lsmash_random_access_flag = 160;
    #[c2rust::src_loc = "1466:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_POST_ROLL_START: lsmash_random_access_flag = 96;
    #[c2rust::src_loc = "1463:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_OPEN_RAP: lsmash_random_access_flag = 20;
    #[c2rust::src_loc = "1460:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_CLOSED_RAP: lsmash_random_access_flag = 12;
    #[c2rust::src_loc = "1457:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_GDR_END: lsmash_random_access_flag = 128;
    #[c2rust::src_loc = "1455:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_GDR_START: lsmash_random_access_flag = 64;
    #[c2rust::src_loc = "1454:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_GDR: lsmash_random_access_flag = 32;
    #[c2rust::src_loc = "1452:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_OPEN: lsmash_random_access_flag = 16;
    #[c2rust::src_loc = "1450:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_CLOSED: lsmash_random_access_flag = 8;
    #[c2rust::src_loc = "1449:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_RAP: lsmash_random_access_flag = 4;
    #[c2rust::src_loc = "1448:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_SYNC: lsmash_random_access_flag = 1;
    #[c2rust::src_loc = "1447:5"]
    pub const ISOM_SAMPLE_RANDOM_ACCESS_FLAG_NONE: lsmash_random_access_flag = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "958:9"]
    pub struct lsmash_codec_specific_t {
        pub type_0: lsmash_codec_specific_data_type,
        pub format: lsmash_codec_specific_format,
        pub data: lsmash_codec_specific_data_t,
        pub size: uint32_t,
        pub destruct: lsmash_codec_specific_destructor_t,
    }
    #[c2rust::src_loc = "957:1"]
    pub type lsmash_codec_specific_destructor_t = Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> (),
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "950:9"]
    pub union lsmash_codec_specific_data_t {
        pub always_null: *mut ::core::ffi::c_void,
        pub structured: *mut ::core::ffi::c_void,
        pub unstructured: *mut uint8_t,
    }
    #[c2rust::src_loc = "943:9"]
    pub type lsmash_codec_specific_format = ::core::ffi::c_int;
    #[c2rust::src_loc = "947:5"]
    pub const LSMASH_CODEC_SPECIFIC_FORMAT_UNSTRUCTURED: lsmash_codec_specific_format = 1;
    #[c2rust::src_loc = "946:5"]
    pub const LSMASH_CODEC_SPECIFIC_FORMAT_STRUCTURED: lsmash_codec_specific_format = 0;
    #[c2rust::src_loc = "945:5"]
    pub const LSMASH_CODEC_SPECIFIC_FORMAT_UNSPECIFIED: lsmash_codec_specific_format = -1;
    #[c2rust::src_loc = "910:9"]
    pub type lsmash_codec_specific_data_type = ::core::ffi::c_int;
    #[c2rust::src_loc = "940:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_CODEC_GLOBAL_HEADER: lsmash_codec_specific_data_type = 20;
    #[c2rust::src_loc = "938:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_AUDIO_CHANNEL_LAYOUT: lsmash_codec_specific_data_type = 19;
    #[c2rust::src_loc = "937:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_VIDEO_GAMMA_LEVEL: lsmash_codec_specific_data_type = 18;
    #[c2rust::src_loc = "936:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_VIDEO_SIGNIFICANT_BITS: lsmash_codec_specific_data_type = 17;
    #[c2rust::src_loc = "935:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_VIDEO_PIXEL_FORMAT: lsmash_codec_specific_data_type = 16;
    #[c2rust::src_loc = "934:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_VIDEO_FIELD_INFO: lsmash_codec_specific_data_type = 15;
    #[c2rust::src_loc = "932:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_AUDIO_DECOMPRESSION_PARAMETERS: lsmash_codec_specific_data_type = 14;
    #[c2rust::src_loc = "931:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_AUDIO_FORMAT_SPECIFIC_FLAGS: lsmash_codec_specific_data_type = 13;
    #[c2rust::src_loc = "930:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_AUDIO_COMMON: lsmash_codec_specific_data_type = 12;
    #[c2rust::src_loc = "929:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_VIDEO_COMMON: lsmash_codec_specific_data_type = 11;
    #[c2rust::src_loc = "927:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_H264_BITRATE: lsmash_codec_specific_data_type = 10;
    #[c2rust::src_loc = "926:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_SAMPLE_SCALE: lsmash_codec_specific_data_type = 9;
    #[c2rust::src_loc = "924:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_AUDIO_ALAC: lsmash_codec_specific_data_type = 8;
    #[c2rust::src_loc = "923:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_AUDIO_DTS: lsmash_codec_specific_data_type = 7;
    #[c2rust::src_loc = "922:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_AUDIO_EC_3: lsmash_codec_specific_data_type = 6;
    #[c2rust::src_loc = "921:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_AUDIO_AC_3: lsmash_codec_specific_data_type = 5;
    #[c2rust::src_loc = "920:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_VC_1: lsmash_codec_specific_data_type = 4;
    #[c2rust::src_loc = "919:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_HEVC: lsmash_codec_specific_data_type = 3;
    #[c2rust::src_loc = "918:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_H264: lsmash_codec_specific_data_type = 2;
    #[c2rust::src_loc = "916:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_MP4SYS_DECODER_CONFIG: lsmash_codec_specific_data_type = 1;
    #[c2rust::src_loc = "914:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_UNKNOWN: lsmash_codec_specific_data_type = 0;
    #[c2rust::src_loc = "912:5"]
    pub const LSMASH_CODEC_SPECIFIC_DATA_TYPE_UNSPECIFIED: lsmash_codec_specific_data_type = -1;
    #[c2rust::src_loc = "3069:9"]
    pub type lsmash_h264_parameter_set_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "3075:5"]
    pub const H264_PARAMETER_SET_TYPE_NUM: lsmash_h264_parameter_set_type = 3;
    #[c2rust::src_loc = "3073:5"]
    pub const H264_PARAMETER_SET_TYPE_SPSEXT: lsmash_h264_parameter_set_type = 2;
    #[c2rust::src_loc = "3072:5"]
    pub const H264_PARAMETER_SET_TYPE_PPS: lsmash_h264_parameter_set_type = 1;
    #[c2rust::src_loc = "3071:5"]
    pub const H264_PARAMETER_SET_TYPE_SPS: lsmash_h264_parameter_set_type = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "3080:9"]
    pub struct lsmash_h264_specific_parameters_t {
        pub AVCProfileIndication: uint8_t,
        pub profile_compatibility: uint8_t,
        pub AVCLevelIndication: uint8_t,
        pub lengthSizeMinusOne: uint8_t,
        pub chroma_format: uint8_t,
        pub bit_depth_luma_minus8: uint8_t,
        pub bit_depth_chroma_minus8: uint8_t,
        pub parameter_sets: *mut lsmash_h264_parameter_sets_t,
    }
    #[c2rust::src_loc = "3078:1"]
    pub type lsmash_h264_parameter_sets_t = lsmash_h264_parameter_sets_tag;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1756:9"]
    pub struct lsmash_media_parameters_t {
        pub handler_type: lsmash_media_type,
        pub timescale: uint32_t,
        pub duration: uint64_t,
        pub roll_grouping: uint8_t,
        pub rap_grouping: uint8_t,
        pub MAC_language: uint16_t,
        pub ISO_language: uint16_t,
        pub media_handler_name: *mut ::core::ffi::c_char,
        pub data_handler_name: *mut ::core::ffi::c_char,
        pub media_handler_name_shadow: [::core::ffi::c_char; 256],
        pub data_handler_name_shadow: [::core::ffi::c_char; 256],
        pub compact_sample_size_table: uint8_t,
        pub no_sample_dependency_table: uint8_t,
        pub reserved: [uint8_t; 2],
    }
    #[c2rust::src_loc = "1619:9"]
    pub type lsmash_media_type = ::core::ffi::c_uint;
    #[c2rust::src_loc = "1645:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_VIDEO_TRACK: lsmash_media_type = 1986618469;
    #[c2rust::src_loc = "1644:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_PROPRIETARY_DESCRIPTIVE_METADATA: lsmash_media_type = 1970432288;
    #[c2rust::src_loc = "1643:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_TEXT_TRACK: lsmash_media_type = 1952807028;
    #[c2rust::src_loc = "1642:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_AUDIO_TRACK: lsmash_media_type = 1936684398;
    #[c2rust::src_loc = "1641:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_KEY_MANAGEMENT_MESSAGES: lsmash_media_type = 1936420205;
    #[c2rust::src_loc = "1640:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_SCENE_DESCRIPTION_STREAM: lsmash_media_type = 1935962989;
    #[c2rust::src_loc = "1639:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_OBJECT_DESCRIPTOR_STREAM: lsmash_media_type = 1868854125;
    #[c2rust::src_loc = "1638:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_OBJECT_CONTENT_INFO_STREAM: lsmash_media_type = 1868788589;
    #[c2rust::src_loc = "1637:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_MPEG21_DIGITAL_ITEM: lsmash_media_type = 1836069425;
    #[c2rust::src_loc = "1636:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_MPEGJ_STREAM: lsmash_media_type = 1835692909;
    #[c2rust::src_loc = "1635:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_TIMED_METADATA_TRACK: lsmash_media_type = 1835365473;
    #[c2rust::src_loc = "1634:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_MPEG7_STREAM: lsmash_media_type = 1832350573;
    #[c2rust::src_loc = "1633:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_IPMP_STREAM: lsmash_media_type = 1768977261;
    #[c2rust::src_loc = "1632:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_IPDC_ELECTRONIC_SERVICE_GUIDE: lsmash_media_type = 1768973411;
    #[c2rust::src_loc = "1631:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_HINT_TRACK: lsmash_media_type = 1751740020;
    #[c2rust::src_loc = "1630:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_GENERAL_MPEG4_SYSTEM_STREAM: lsmash_media_type = 1734701933;
    #[c2rust::src_loc = "1629:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_FONT_DATA_STREAM: lsmash_media_type = 1717859181;
    #[c2rust::src_loc = "1628:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_BROADBAND_CONTENT_GUIDE: lsmash_media_type = 1685354081;
    #[c2rust::src_loc = "1627:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_TV_ANYTIME: lsmash_media_type = 1685354081;
    #[c2rust::src_loc = "1626:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_DVB_MANDATORY_BASIC_DESCRIPTION: lsmash_media_type = 1684890212;
    #[c2rust::src_loc = "1625:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_CLOCK_REFERENCE_STREAM: lsmash_media_type = 1668445037;
    #[c2rust::src_loc = "1624:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_CPCM_AUXILIARY_METADATA: lsmash_media_type = 1668309348;
    #[c2rust::src_loc = "1623:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_AUXILIARY_VIDEO_TRACK: lsmash_media_type = 1635088502;
    #[c2rust::src_loc = "1622:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_ID3_VERSION2_METADATA: lsmash_media_type = 1229206322;
    #[c2rust::src_loc = "1621:5"]
    pub const ISOM_MEDIA_HANDLER_TYPE_3GPP_SCENE_DESCRIPTION: lsmash_media_type = 862417764;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "1993:9"]
    pub struct lsmash_track_parameters_t {
        pub mode: lsmash_track_mode,
        pub track_ID: uint32_t,
        pub duration: uint64_t,
        pub alternate_group: int16_t,
        pub video_layer: int16_t,
        pub audio_volume: int16_t,
        pub matrix: [int32_t; 9],
        pub display_width: uint32_t,
        pub display_height: uint32_t,
        pub aperture_modes: uint8_t,
    }
    #[c2rust::src_loc = "1981:9"]
    pub type lsmash_track_mode = ::core::ffi::c_uint;
    #[c2rust::src_loc = "1990:5"]
    pub const QT_TRACK_IN_POSTER: lsmash_track_mode = 8;
    #[c2rust::src_loc = "1988:5"]
    pub const ISOM_TRACK_IN_PREVIEW: lsmash_track_mode = 4;
    #[c2rust::src_loc = "1987:5"]
    pub const ISOM_TRACK_IN_MOVIE: lsmash_track_mode = 2;
    #[c2rust::src_loc = "1985:5"]
    pub const ISOM_TRACK_ENABLED: lsmash_track_mode = 1;
    #[c2rust::src_loc = "1350:5"]
    pub const ISOM_MATRIX_INDEX_UNSPECIFIED: C2RustUnnamed_6 = 2;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "2217:9"]
    pub struct lsmash_movie_parameters_t {
        pub timescale: uint32_t,
        pub duration: uint64_t,
        pub number_of_tracks: uint32_t,
        pub playback_rate: int32_t,
        pub playback_volume: int32_t,
        pub preview_time: int32_t,
        pub preview_duration: int32_t,
        pub poster_time: int32_t,
    }
    #[c2rust::src_loc = "98:1"]
    pub type lsmash_file_t = lsmash_file_tag;
    #[c2rust::src_loc = "1340:1"]
    pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
    #[c2rust::src_loc = "1370:5"]
    pub const QT_MATRIX_INDEX_SMPTE_240M_1995: C2RustUnnamed_6 = 7;
    #[c2rust::src_loc = "1369:5"]
    pub const QT_MATRIX_INDEX_ITU_R_601_4: C2RustUnnamed_6 = 6;
    #[c2rust::src_loc = "1368:5"]
    pub const QT_MATRIX_INDEX_UNSPECIFIED: C2RustUnnamed_6 = 2;
    #[c2rust::src_loc = "1367:5"]
    pub const QT_MATRIX_INDEX_ITU_R_709_2: C2RustUnnamed_6 = 1;
    #[c2rust::src_loc = "1364:5"]
    pub const ISOM_MATRIX_INDEX_YCGCO: C2RustUnnamed_6 = 8;
    #[c2rust::src_loc = "1362:5"]
    pub const ISOM_MATRIX_INDEX_SMPTE_240M_1999: C2RustUnnamed_6 = 7;
    #[c2rust::src_loc = "1357:5"]
    pub const ISOM_MATRIX_INDEX_SMPTE_170M_2004: C2RustUnnamed_6 = 6;
    #[c2rust::src_loc = "1353:5"]
    pub const ISOM_MATRIX_INDEX_ITU_R470BG: C2RustUnnamed_6 = 5;
    #[c2rust::src_loc = "1351:5"]
    pub const ISOM_MATRIX_INDEX_USFCCT_47_CFR: C2RustUnnamed_6 = 4;
    #[c2rust::src_loc = "1345:5"]
    pub const ISOM_MATRIX_INDEX_ITU_R_709_5: C2RustUnnamed_6 = 1;
    #[c2rust::src_loc = "1343:5"]
    pub const ISOM_MATRIX_INDEX_NO_MATRIX: C2RustUnnamed_6 = 0;
    #[c2rust::src_loc = "2037:9"]
    pub const ISOM_EDIT_MODE_NORMAL: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
        << 16 as ::core::ffi::c_int;
    #[c2rust::src_loc = "2040:9"]
    pub const ISOM_EDIT_DURATION_UNKNOWN32: ::core::ffi::c_uint = 0xffffffff
        as ::core::ffi::c_uint;
    use super::stdint_uintn_h::{uint8_t, uint32_t, uint64_t, uint16_t};
    use super::stdint_intn_h::{int64_t, int32_t, int16_t};
    extern "C" {
        #[c2rust::src_loc = "967:16"]
        pub type lsmash_codec_specific_list_tag;
        #[c2rust::src_loc = "77:16"]
        pub type lsmash_root_tag;
        #[c2rust::src_loc = "3078:16"]
        pub type lsmash_h264_parameter_sets_tag;
        #[c2rust::src_loc = "98:16"]
        pub type lsmash_file_tag;
        #[c2rust::src_loc = "84:1"]
        pub fn lsmash_create_root() -> *mut lsmash_root_t;
        #[c2rust::src_loc = "90:1"]
        pub fn lsmash_destroy_root(root: *mut lsmash_root_t);
        #[c2rust::src_loc = "291:1"]
        pub fn lsmash_open_file(
            filename: *const ::core::ffi::c_char,
            open_mode: ::core::ffi::c_int,
            param: *mut lsmash_file_parameters_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "302:1"]
        pub fn lsmash_close_file(
            param: *mut lsmash_file_parameters_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "313:1"]
        pub fn lsmash_set_file(
            root: *mut lsmash_root_t,
            param: *mut lsmash_file_parameters_t,
        ) -> *mut lsmash_file_t;
        #[c2rust::src_loc = "751:25"]
        pub static ISOM_CODEC_TYPE_AVC1_VIDEO: lsmash_codec_type_t;
        #[c2rust::src_loc = "997:1"]
        pub fn lsmash_create_summary(
            summary_type: lsmash_summary_type,
        ) -> *mut lsmash_summary_t;
        #[c2rust::src_loc = "1003:1"]
        pub fn lsmash_cleanup_summary(summary: *mut lsmash_summary_t);
        #[c2rust::src_loc = "1012:1"]
        pub fn lsmash_add_sample_entry(
            root: *mut lsmash_root_t,
            track_ID: uint32_t,
            summary: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1046:1"]
        pub fn lsmash_create_codec_specific_data(
            type_0: lsmash_codec_specific_data_type,
            format: lsmash_codec_specific_format,
        ) -> *mut lsmash_codec_specific_t;
        #[c2rust::src_loc = "1053:1"]
        pub fn lsmash_destroy_codec_specific_data(
            specific: *mut lsmash_codec_specific_t,
        );
        #[c2rust::src_loc = "1062:1"]
        pub fn lsmash_add_codec_specific_data(
            summary: *mut lsmash_summary_t,
            specific: *mut lsmash_codec_specific_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1579:1"]
        pub fn lsmash_create_sample(size: uint32_t) -> *mut lsmash_sample_t;
        #[c2rust::src_loc = "1608:1"]
        pub fn lsmash_append_sample(
            root: *mut lsmash_root_t,
            track_ID: uint32_t,
            sample: *mut lsmash_sample_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1792:1"]
        pub fn lsmash_initialize_media_parameters(param: *mut lsmash_media_parameters_t);
        #[c2rust::src_loc = "1801:1"]
        pub fn lsmash_set_media_parameters(
            root: *mut lsmash_root_t,
            track_ID: uint32_t,
            param: *mut lsmash_media_parameters_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1824:1"]
        pub fn lsmash_flush_pooled_samples(
            root: *mut lsmash_root_t,
            track_ID: uint32_t,
            last_sample_delta: uint32_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1868:1"]
        pub fn lsmash_get_media_timescale(
            root: *mut lsmash_root_t,
            track_ID: uint32_t,
        ) -> uint32_t;
        #[c2rust::src_loc = "2068:1"]
        pub fn lsmash_create_track(
            root: *mut lsmash_root_t,
            media_type: lsmash_media_type,
        ) -> uint32_t;
        #[c2rust::src_loc = "2082:1"]
        pub fn lsmash_initialize_track_parameters(param: *mut lsmash_track_parameters_t);
        #[c2rust::src_loc = "2091:1"]
        pub fn lsmash_set_track_parameters(
            root: *mut lsmash_root_t,
            track_ID: uint32_t,
            param: *mut lsmash_track_parameters_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2159:1"]
        pub fn lsmash_create_explicit_timeline_map(
            root: *mut lsmash_root_t,
            track_ID: uint32_t,
            edit: lsmash_edit_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2206:1"]
        pub fn lsmash_modify_explicit_timeline_map(
            root: *mut lsmash_root_t,
            track_ID: uint32_t,
            edit_number: uint32_t,
            edit: lsmash_edit_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2233:1"]
        pub fn lsmash_initialize_movie_parameters(param: *mut lsmash_movie_parameters_t);
        #[c2rust::src_loc = "2242:1"]
        pub fn lsmash_set_movie_parameters(
            root: *mut lsmash_root_t,
            param: *mut lsmash_movie_parameters_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2256:1"]
        pub fn lsmash_finish_movie(
            root: *mut lsmash_root_t,
            remux: *mut lsmash_adhoc_remux_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "2287:1"]
        pub fn lsmash_get_movie_timescale(root: *mut lsmash_root_t) -> uint32_t;
        #[c2rust::src_loc = "2365:1"]
        pub fn lsmash_create_fragment_movie(
            root: *mut lsmash_root_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "3129:1"]
        pub fn lsmash_append_h264_parameter_set(
            param: *mut lsmash_h264_specific_parameters_t,
            ps_type: lsmash_h264_parameter_set_type,
            ps_data: *mut ::core::ffi::c_void,
            ps_length: uint32_t,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:30"]
pub mod stdio_h {
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "187:1"]
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "279:1"]
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut FILE;
        #[c2rust::src_loc = "873:1"]
        pub fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/sys/stat.h:30"]
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "227:1"]
        pub fn stat(
            __file: *const ::core::ffi::c_char,
            __buf: *mut stat,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "230:1"]
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:30"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "672:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "675:1"]
        pub fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "687:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:30"]
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
            st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
            st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
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
    use super::struct_stat_h::stat;
    use super::types_h::{
        __dev_t, __ino_t, __nlink_t, __mode_t, __uid_t, __gid_t, __off_t, __blksize_t,
        __blkcnt_t, __syscall_slong_t, __time_t,
    };
    use super::struct_timespec_h::timespec;
    use super::stat_h::{stat, fstat};
    use super::bits_stat_h::__S_IFMT;
    use super::FILE_h::FILE;
    use super::stdio_h::fileno;
}
#[c2rust::header_src = "/usr/include/string.h:30"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "156:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:30"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:30"]
pub mod bits_stat_h {
    #[c2rust::src_loc = "29:9"]
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
pub use self::internal::__va_list_tag;
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{
    __uint8_t, __int16_t, __uint16_t, __int32_t, __uint32_t, __int64_t, __uint64_t,
    __dev_t, __uid_t, __gid_t, __ino_t, __mode_t, __nlink_t, __off_t, __off64_t,
    __time_t, __blksize_t, __blkcnt_t, __syscall_slong_t,
};
pub use self::struct_FILE_h::{
    _IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt, _IO_marker,
};
pub use self::FILE_h::FILE;
pub use self::struct_timespec_h::timespec;
pub use self::struct_stat_h::stat;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::x264_h::{
    x264_nal_t, x264_zone_t, x264_param_t, C2RustUnnamed, C2RustUnnamed_0,
    C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4, x264_hrd_t,
    x264_sei_payload_t, x264_sei_t, x264_image_t, x264_image_properties_t,
    x264_picture_t, X264_LOG_ERROR, x264_t,
};
pub use self::x264cli_h::{hnd_t, x264_cli_log};
pub use self::output_h::{cli_output_opt_t, cli_output_t};
pub use self::lsmash_h::{
    lsmash_file_parameters_t, lsmash_brand_type, ISOM_BRAND_TYPE_SSSS,
    ISOM_BRAND_TYPE_SISX, ISOM_BRAND_TYPE_SIMS, ISOM_BRAND_TYPE_SDV,
    ISOM_BRAND_TYPE_RISX, ISOM_BRAND_TYPE_QT, ISOM_BRAND_TYPE_PNVI, ISOM_BRAND_TYPE_PIFF,
    ISOM_BRAND_TYPE_PANA, ISOM_BRAND_TYPE_OPX2, ISOM_BRAND_TYPE_OPF2,
    ISOM_BRAND_TYPE_ODCF, ISOM_BRAND_TYPE_NIKO, ISOM_BRAND_TYPE_MSIX,
    ISOM_BRAND_TYPE_MSDH, ISOM_BRAND_TYPE_MP71, ISOM_BRAND_TYPE_MP42,
    ISOM_BRAND_TYPE_MP41, ISOM_BRAND_TYPE_MP21, ISOM_BRAND_TYPE_MJP2,
    ISOM_BRAND_TYPE_MJ2S, ISOM_BRAND_TYPE_LMSG, ISOM_BRAND_TYPE_JPSI,
    ISOM_BRAND_TYPE_ISOM, ISOM_BRAND_TYPE_ISO7, ISOM_BRAND_TYPE_ISO6,
    ISOM_BRAND_TYPE_ISO5, ISOM_BRAND_TYPE_ISO4, ISOM_BRAND_TYPE_ISO3,
    ISOM_BRAND_TYPE_ISO2, ISOM_BRAND_TYPE_ISC2, ISOM_BRAND_TYPE_IFRM,
    ISOM_BRAND_TYPE_DVT1, ISOM_BRAND_TYPE_DVR1, ISOM_BRAND_TYPE_DV3B,
    ISOM_BRAND_TYPE_DV3A, ISOM_BRAND_TYPE_DV2B, ISOM_BRAND_TYPE_DV2A,
    ISOM_BRAND_TYPE_DV1B, ISOM_BRAND_TYPE_DV1A, ISOM_BRAND_TYPE_DSMS,
    ISOM_BRAND_TYPE_DMB1, ISOM_BRAND_TYPE_DBY1, ISOM_BRAND_TYPE_DASH,
    ISOM_BRAND_TYPE_DA3B, ISOM_BRAND_TYPE_DA3A, ISOM_BRAND_TYPE_DA2B,
    ISOM_BRAND_TYPE_DA2A, ISOM_BRAND_TYPE_DA1B, ISOM_BRAND_TYPE_DA1A,
    ISOM_BRAND_TYPE_DA0B, ISOM_BRAND_TYPE_DA0A, ISOM_BRAND_TYPE_CCFF,
    ISOM_BRAND_TYPE_CAQV, ISOM_BRAND_TYPE_BBXM, ISOM_BRAND_TYPE_AVC1,
    ISOM_BRAND_TYPE_ROSS, ISOM_BRAND_TYPE_MPPI, ISOM_BRAND_TYPE_MFSM,
    ISOM_BRAND_TYPE_M4V, ISOM_BRAND_TYPE_M4P, ISOM_BRAND_TYPE_M4B, ISOM_BRAND_TYPE_M4A,
    ISOM_BRAND_TYPE_LCAG, ISOM_BRAND_TYPE_CDES, ISOM_BRAND_TYPE_CAEP,
    ISOM_BRAND_TYPE_ARRI, ISOM_BRAND_TYPE_3GT9, ISOM_BRAND_TYPE_3GS9,
    ISOM_BRAND_TYPE_3GS6, ISOM_BRAND_TYPE_3GR9, ISOM_BRAND_TYPE_3GR6,
    ISOM_BRAND_TYPE_3GP9, ISOM_BRAND_TYPE_3GP8, ISOM_BRAND_TYPE_3GP7,
    ISOM_BRAND_TYPE_3GP6, ISOM_BRAND_TYPE_3GP5, ISOM_BRAND_TYPE_3GP4,
    ISOM_BRAND_TYPE_3GM9, ISOM_BRAND_TYPE_3GH9, ISOM_BRAND_TYPE_3GG9,
    ISOM_BRAND_TYPE_3GG6, ISOM_BRAND_TYPE_3GF9, ISOM_BRAND_TYPE_3GE9,
    ISOM_BRAND_TYPE_3GE6, ISOM_BRAND_TYPE_3G2A, lsmash_file_mode,
    LSMASH_FILE_MODE_WRITE_FRAGMENTED, LSMASH_FILE_MODE_SEGMENT, LSMASH_FILE_MODE_INDEX,
    LSMASH_FILE_MODE_MEDIA, LSMASH_FILE_MODE_INITIALIZATION, LSMASH_FILE_MODE_BOX,
    LSMASH_FILE_MODE_DUMP, LSMASH_FILE_MODE_FRAGMENTED, LSMASH_FILE_MODE_READ,
    LSMASH_FILE_MODE_WRITE, lsmash_video_summary_t, C2RustUnnamed_5, lsmash_clap_t,
    lsmash_rational_s32_t, lsmash_rational_u32_t, lsmash_video_depth,
    QT_VIDEO_DEPTH_32ARGB, QT_VIDEO_DEPTH_24RGB, QT_VIDEO_DEPTH_555RGB,
    QT_VIDEO_DEPTH_GRAYSCALE_8, QT_VIDEO_DEPTH_GRAYSCALE_4, QT_VIDEO_DEPTH_GRAYSCALE_2,
    QT_VIDEO_DEPTH_GRAYSCALE_1, QT_VIDEO_DEPTH_COLOR_32, QT_VIDEO_DEPTH_COLOR_24,
    QT_VIDEO_DEPTH_COLOR_16, QT_VIDEO_DEPTH_COLOR_8, QT_VIDEO_DEPTH_COLOR_4,
    QT_VIDEO_DEPTH_COLOR_2, QT_VIDEO_DEPTH_COLOR_1, AVC_DEPTH_WITH_ALPHA,
    AVC_DEPTH_GRAYSCALE_WITH_NO_ALPHA, AVC_DEPTH_COLOR_WITH_NO_ALPHA,
    ISOM_DEPTH_TEMPLATE, lsmash_codec_specific_list_t, lsmash_box_type_t,
    lsmash_codec_type_t, lsmash_extended_box_type_t, lsmash_compact_box_type_t,
    lsmash_summary_type, LSMASH_SUMMARY_TYPE_AUDIO, LSMASH_SUMMARY_TYPE_VIDEO,
    LSMASH_SUMMARY_TYPE_UNKNOWN, lsmash_root_t, lsmash_summary_t, lsmash_adhoc_remux_t,
    lsmash_adhoc_remux_callback, lsmash_edit_t, lsmash_sample_t,
    lsmash_sample_property_t, lsmash_pre_roll_t, lsmash_post_roll_t,
    lsmash_random_access_flag, QT_SAMPLE_RANDOM_ACCESS_FLAG_OPEN_RAP,
    QT_SAMPLE_RANDOM_ACCESS_FLAG_CLOSED_RAP, QT_SAMPLE_RANDOM_ACCESS_FLAG_OPEN,
    QT_SAMPLE_RANDOM_ACCESS_FLAG_CLOSED, QT_SAMPLE_RANDOM_ACCESS_FLAG_RAP,
    QT_SAMPLE_RANDOM_ACCESS_FLAG_PARTIAL_SYNC, QT_SAMPLE_RANDOM_ACCESS_FLAG_SYNC,
    QT_SAMPLE_RANDOM_ACCESS_FLAG_NONE, ISOM_SAMPLE_RANDOM_ACCESS_FLAG_PRE_ROLL_END,
    ISOM_SAMPLE_RANDOM_ACCESS_FLAG_POST_ROLL_START,
    ISOM_SAMPLE_RANDOM_ACCESS_FLAG_OPEN_RAP, ISOM_SAMPLE_RANDOM_ACCESS_FLAG_CLOSED_RAP,
    ISOM_SAMPLE_RANDOM_ACCESS_FLAG_GDR_END, ISOM_SAMPLE_RANDOM_ACCESS_FLAG_GDR_START,
    ISOM_SAMPLE_RANDOM_ACCESS_FLAG_GDR, ISOM_SAMPLE_RANDOM_ACCESS_FLAG_OPEN,
    ISOM_SAMPLE_RANDOM_ACCESS_FLAG_CLOSED, ISOM_SAMPLE_RANDOM_ACCESS_FLAG_RAP,
    ISOM_SAMPLE_RANDOM_ACCESS_FLAG_SYNC, ISOM_SAMPLE_RANDOM_ACCESS_FLAG_NONE,
    lsmash_codec_specific_t, lsmash_codec_specific_destructor_t,
    lsmash_codec_specific_data_t, lsmash_codec_specific_format,
    LSMASH_CODEC_SPECIFIC_FORMAT_UNSTRUCTURED, LSMASH_CODEC_SPECIFIC_FORMAT_STRUCTURED,
    LSMASH_CODEC_SPECIFIC_FORMAT_UNSPECIFIED, lsmash_codec_specific_data_type,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_CODEC_GLOBAL_HEADER,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_AUDIO_CHANNEL_LAYOUT,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_VIDEO_GAMMA_LEVEL,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_VIDEO_SIGNIFICANT_BITS,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_VIDEO_PIXEL_FORMAT,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_VIDEO_FIELD_INFO,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_AUDIO_DECOMPRESSION_PARAMETERS,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_AUDIO_FORMAT_SPECIFIC_FLAGS,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_AUDIO_COMMON,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_QT_VIDEO_COMMON,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_H264_BITRATE,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_SAMPLE_SCALE,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_AUDIO_ALAC,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_AUDIO_DTS,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_AUDIO_EC_3,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_AUDIO_AC_3,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_VC_1,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_HEVC,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_H264,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_MP4SYS_DECODER_CONFIG,
    LSMASH_CODEC_SPECIFIC_DATA_TYPE_UNKNOWN, LSMASH_CODEC_SPECIFIC_DATA_TYPE_UNSPECIFIED,
    lsmash_h264_parameter_set_type, H264_PARAMETER_SET_TYPE_NUM,
    H264_PARAMETER_SET_TYPE_SPSEXT, H264_PARAMETER_SET_TYPE_PPS,
    H264_PARAMETER_SET_TYPE_SPS, lsmash_h264_specific_parameters_t,
    lsmash_h264_parameter_sets_t, lsmash_media_parameters_t, lsmash_media_type,
    ISOM_MEDIA_HANDLER_TYPE_VIDEO_TRACK,
    ISOM_MEDIA_HANDLER_TYPE_PROPRIETARY_DESCRIPTIVE_METADATA,
    ISOM_MEDIA_HANDLER_TYPE_TEXT_TRACK, ISOM_MEDIA_HANDLER_TYPE_AUDIO_TRACK,
    ISOM_MEDIA_HANDLER_TYPE_KEY_MANAGEMENT_MESSAGES,
    ISOM_MEDIA_HANDLER_TYPE_SCENE_DESCRIPTION_STREAM,
    ISOM_MEDIA_HANDLER_TYPE_OBJECT_DESCRIPTOR_STREAM,
    ISOM_MEDIA_HANDLER_TYPE_OBJECT_CONTENT_INFO_STREAM,
    ISOM_MEDIA_HANDLER_TYPE_MPEG21_DIGITAL_ITEM, ISOM_MEDIA_HANDLER_TYPE_MPEGJ_STREAM,
    ISOM_MEDIA_HANDLER_TYPE_TIMED_METADATA_TRACK, ISOM_MEDIA_HANDLER_TYPE_MPEG7_STREAM,
    ISOM_MEDIA_HANDLER_TYPE_IPMP_STREAM,
    ISOM_MEDIA_HANDLER_TYPE_IPDC_ELECTRONIC_SERVICE_GUIDE,
    ISOM_MEDIA_HANDLER_TYPE_HINT_TRACK,
    ISOM_MEDIA_HANDLER_TYPE_GENERAL_MPEG4_SYSTEM_STREAM,
    ISOM_MEDIA_HANDLER_TYPE_FONT_DATA_STREAM,
    ISOM_MEDIA_HANDLER_TYPE_BROADBAND_CONTENT_GUIDE, ISOM_MEDIA_HANDLER_TYPE_TV_ANYTIME,
    ISOM_MEDIA_HANDLER_TYPE_DVB_MANDATORY_BASIC_DESCRIPTION,
    ISOM_MEDIA_HANDLER_TYPE_CLOCK_REFERENCE_STREAM,
    ISOM_MEDIA_HANDLER_TYPE_CPCM_AUXILIARY_METADATA,
    ISOM_MEDIA_HANDLER_TYPE_AUXILIARY_VIDEO_TRACK,
    ISOM_MEDIA_HANDLER_TYPE_ID3_VERSION2_METADATA,
    ISOM_MEDIA_HANDLER_TYPE_3GPP_SCENE_DESCRIPTION, lsmash_track_parameters_t,
    lsmash_track_mode, QT_TRACK_IN_POSTER, ISOM_TRACK_IN_PREVIEW, ISOM_TRACK_IN_MOVIE,
    ISOM_TRACK_ENABLED, ISOM_MATRIX_INDEX_UNSPECIFIED, lsmash_movie_parameters_t,
    lsmash_file_t, C2RustUnnamed_6, QT_MATRIX_INDEX_SMPTE_240M_1995,
    QT_MATRIX_INDEX_ITU_R_601_4, QT_MATRIX_INDEX_UNSPECIFIED,
    QT_MATRIX_INDEX_ITU_R_709_2, ISOM_MATRIX_INDEX_YCGCO,
    ISOM_MATRIX_INDEX_SMPTE_240M_1999, ISOM_MATRIX_INDEX_SMPTE_170M_2004,
    ISOM_MATRIX_INDEX_ITU_R470BG, ISOM_MATRIX_INDEX_USFCCT_47_CFR,
    ISOM_MATRIX_INDEX_ITU_R_709_5, ISOM_MATRIX_INDEX_NO_MATRIX, ISOM_EDIT_MODE_NORMAL,
    ISOM_EDIT_DURATION_UNKNOWN32, lsmash_codec_specific_list_tag, lsmash_root_tag,
    lsmash_h264_parameter_sets_tag, lsmash_file_tag, lsmash_create_root,
    lsmash_destroy_root, lsmash_open_file, lsmash_close_file, lsmash_set_file,
    ISOM_CODEC_TYPE_AVC1_VIDEO, lsmash_create_summary, lsmash_cleanup_summary,
    lsmash_add_sample_entry, lsmash_create_codec_specific_data,
    lsmash_destroy_codec_specific_data, lsmash_add_codec_specific_data,
    lsmash_create_sample, lsmash_append_sample, lsmash_initialize_media_parameters,
    lsmash_set_media_parameters, lsmash_flush_pooled_samples, lsmash_get_media_timescale,
    lsmash_create_track, lsmash_initialize_track_parameters, lsmash_set_track_parameters,
    lsmash_create_explicit_timeline_map, lsmash_modify_explicit_timeline_map,
    lsmash_initialize_movie_parameters, lsmash_set_movie_parameters, lsmash_finish_movie,
    lsmash_get_movie_timescale, lsmash_create_fragment_movie,
    lsmash_append_h264_parameter_set,
};
use self::stdio_h::{fclose, fopen, fileno};
use self::stat_h::{stat, fstat};
use self::stdlib_h::{malloc, calloc, free};
pub use self::osdep_h::{x264_is_regular_file_path, x264_is_regular_file};
use self::string_h::{memcpy, strcmp};
pub use self::__stddef_null_h::NULL;
pub use self::bits_stat_h::__S_IFMT;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "66:9"]
pub struct mp4_hnd_t {
    pub p_root: *mut lsmash_root_t,
    pub summary: *mut lsmash_video_summary_t,
    pub b_stdout: ::core::ffi::c_int,
    pub i_movie_timescale: uint32_t,
    pub i_video_timescale: uint32_t,
    pub i_track: uint32_t,
    pub i_sample_entry: uint32_t,
    pub i_time_inc: uint64_t,
    pub i_start_offset: int64_t,
    pub i_first_cts: uint64_t,
    pub i_prev_dts: uint64_t,
    pub i_sei_size: uint32_t,
    pub p_sei_buffer: *mut uint8_t,
    pub i_numframe: ::core::ffi::c_int,
    pub i_init_delta: int64_t,
    pub i_delay_frames: ::core::ffi::c_int,
    pub b_dts_compress: ::core::ffi::c_int,
    pub i_dts_compress_multiplier: ::core::ffi::c_int,
    pub b_use_recovery: ::core::ffi::c_int,
    pub b_fragments: ::core::ffi::c_int,
    pub file_param: lsmash_file_parameters_t,
}
#[c2rust::src_loc = "33:9"]
pub const H264_NALU_LENGTH_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[c2rust::src_loc = "93:1"]
unsafe extern "C" fn remove_mp4_hnd(mut handle: hnd_t) {
    let mut p_mp4: *mut mp4_hnd_t = handle as *mut mp4_hnd_t;
    if p_mp4.is_null() {
        return;
    }
    lsmash_cleanup_summary((*p_mp4).summary as *mut lsmash_summary_t);
    lsmash_close_file(&mut (*p_mp4).file_param);
    lsmash_destroy_root((*p_mp4).p_root);
    free((*p_mp4).p_sei_buffer as *mut ::core::ffi::c_void);
    free(p_mp4 as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut largest_pts: int64_t,
    mut second_largest_pts: int64_t,
) -> ::core::ffi::c_int {
    let mut p_mp4: *mut mp4_hnd_t = handle as *mut mp4_hnd_t;
    if p_mp4.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if !(*p_mp4).p_root.is_null() {
        let mut actual_duration: ::core::ffi::c_double = 0 as ::core::ffi::c_int
            as ::core::ffi::c_double;
        if (*p_mp4).i_track != 0 {
            let mut last_delta: uint32_t = (largest_pts - second_largest_pts)
                as uint32_t;
            if lsmash_flush_pooled_samples(
                (*p_mp4).p_root,
                (*p_mp4).i_track,
                ((if last_delta != 0 { last_delta } else { 1 as uint32_t }) as uint64_t)
                    .wrapping_mul((*p_mp4).i_time_inc) as uint32_t,
            ) != 0
            {
                x264_cli_log(
                    b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"failed to flush the rest of samples.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if (*p_mp4).i_movie_timescale != 0 as uint32_t
                && (*p_mp4).i_video_timescale != 0 as uint32_t
            {
                actual_duration = ((largest_pts + last_delta as int64_t) as uint64_t)
                    .wrapping_mul((*p_mp4).i_time_inc) as ::core::ffi::c_double
                    / (*p_mp4).i_video_timescale as ::core::ffi::c_double
                    * (*p_mp4).i_movie_timescale as ::core::ffi::c_double;
            } else {
                x264_cli_log(
                    b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"timescale is broken.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
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
                if lsmash_create_explicit_timeline_map(
                    (*p_mp4).p_root,
                    (*p_mp4).i_track,
                    edit,
                ) != 0
                {
                    x264_cli_log(
                        b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
                        X264_LOG_ERROR,
                        b"failed to set timeline map for video.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            } else if (*p_mp4).b_stdout == 0 {
                if lsmash_modify_explicit_timeline_map(
                    (*p_mp4).p_root,
                    (*p_mp4).i_track,
                    1 as uint32_t,
                    edit,
                ) != 0
                {
                    x264_cli_log(
                        b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
                        X264_LOG_ERROR,
                        b"failed to update timeline map for video.\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                }
            }
        }
        if lsmash_finish_movie((*p_mp4).p_root, 0 as *mut lsmash_adhoc_remux_t) != 0 {
            x264_cli_log(
                b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"failed to finish movie.\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
        }
    }
    remove_mp4_hnd(p_mp4 as hnd_t);
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "163:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut ::core::ffi::c_char,
    mut p_handle: *mut hnd_t,
    mut opt: *mut cli_output_opt_t,
) -> ::core::ffi::c_int {
    *p_handle = NULL as hnd_t;
    let mut b_regular: ::core::ffi::c_int = strcmp(
        psz_filename,
        b"-\0" as *const u8 as *const ::core::ffi::c_char,
    );
    b_regular = (b_regular != 0 && x264_is_regular_file_path(psz_filename) != 0)
        as ::core::ffi::c_int;
    if b_regular != 0 {
        let mut fh: *mut FILE = fopen(
            psz_filename,
            b"wb\0" as *const u8 as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if fh.is_null() {
            x264_cli_log(
                b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"cannot open output file `%s'.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                psz_filename,
            );
            return -(1 as ::core::ffi::c_int);
        }
        b_regular = x264_is_regular_file(fh);
        fclose(fh);
    }
    let mut p_mp4: *mut mp4_hnd_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<mp4_hnd_t>() as size_t,
    ) as *mut mp4_hnd_t;
    if p_mp4.is_null() {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to allocate memory for muxer information.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*p_mp4).b_dts_compress = (*opt).use_dts_compress;
    (*p_mp4).b_use_recovery = 0 as ::core::ffi::c_int;
    (*p_mp4).b_fragments = (b_regular == 0) as ::core::ffi::c_int;
    (*p_mp4).b_stdout = (strcmp(
        psz_filename,
        b"-\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0) as ::core::ffi::c_int;
    (*p_mp4).p_root = lsmash_create_root();
    if (*p_mp4).p_root.is_null() {
        remove_mp4_hnd(p_mp4 as hnd_t);
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to create root.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if lsmash_open_file(psz_filename, 0 as ::core::ffi::c_int, &mut (*p_mp4).file_param)
        < 0 as ::core::ffi::c_int
    {
        remove_mp4_hnd(p_mp4 as hnd_t);
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to open an output file.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*p_mp4).b_fragments != 0 {
        (*p_mp4).file_param.mode = ::core::mem::transmute::<
            ::core::ffi::c_uint,
            lsmash_file_mode,
        >(
            (*p_mp4).file_param.mode as ::core::ffi::c_uint
                | LSMASH_FILE_MODE_FRAGMENTED as ::core::ffi::c_int
                    as ::core::ffi::c_uint,
        );
    }
    (*p_mp4).summary = lsmash_create_summary(LSMASH_SUMMARY_TYPE_VIDEO)
        as *mut lsmash_video_summary_t;
    if (*p_mp4).summary.is_null() {
        remove_mp4_hnd(p_mp4 as hnd_t);
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to allocate memory for summary information of video.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*(*p_mp4).summary).sample_type = ISOM_CODEC_TYPE_AVC1_VIDEO;
    *p_handle = p_mp4 as hnd_t;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "202:1"]
unsafe extern "C" fn set_param(
    mut handle: hnd_t,
    mut p_param: *mut x264_param_t,
) -> ::core::ffi::c_int {
    let mut p_mp4: *mut mp4_hnd_t = handle as *mut mp4_hnd_t;
    let mut i_media_timescale: uint64_t = 0;
    (*p_mp4).i_delay_frames = if (*p_param).i_bframe != 0 {
        if (*p_param).i_bframe_pyramid != 0 {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }
    } else {
        0 as ::core::ffi::c_int
    };
    (*p_mp4).i_dts_compress_multiplier = (*p_mp4).b_dts_compress
        * (*p_mp4).i_delay_frames + 1 as ::core::ffi::c_int;
    i_media_timescale = ((*p_param).i_timebase_den as uint64_t)
        .wrapping_mul((*p_mp4).i_dts_compress_multiplier as uint64_t);
    (*p_mp4).i_time_inc = ((*p_param).i_timebase_num as uint64_t)
        .wrapping_mul((*p_mp4).i_dts_compress_multiplier as uint64_t);
    if i_media_timescale > 4294967295 as uint64_t {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"MP4 media timescale %lu exceeds maximum\n\0" as *const u8
                as *const ::core::ffi::c_char,
            i_media_timescale,
        );
        return -(1 as ::core::ffi::c_int);
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
        if (*p_param).b_open_gop != 0 {
            let fresh4 = brand_count;
            brand_count = brand_count.wrapping_add(1);
            brands[fresh4 as usize] = ISOM_BRAND_TYPE_ISO6;
        }
    }
    let mut file_param: *mut lsmash_file_parameters_t = &mut (*p_mp4).file_param;
    (*file_param).major_brand = brands[0 as ::core::ffi::c_int as usize];
    (*file_param).brands = brands.as_mut_ptr();
    (*file_param).brand_count = brand_count;
    (*file_param).minor_version = 0 as uint32_t;
    if lsmash_set_file((*p_mp4).p_root, file_param).is_null() {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to add an output file into a ROOT.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
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
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to set movie parameters.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*p_mp4).i_movie_timescale = lsmash_get_movie_timescale((*p_mp4).p_root);
    if (*p_mp4).i_movie_timescale == 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"movie timescale is broken.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*p_mp4).i_track = lsmash_create_track(
        (*p_mp4).p_root,
        ISOM_MEDIA_HANDLER_TYPE_VIDEO_TRACK,
    );
    if (*p_mp4).i_track == 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to create a video track.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*(*p_mp4).summary).width = (*p_param).i_width as uint32_t;
    (*(*p_mp4).summary).height = (*p_param).i_height as uint32_t;
    let mut i_display_width: uint32_t = ((*p_param).i_width << 16 as ::core::ffi::c_int)
        as uint32_t;
    let mut i_display_height: uint32_t = ((*p_param).i_height
        << 16 as ::core::ffi::c_int) as uint32_t;
    if (*p_param).vui.i_sar_width != 0 && (*p_param).vui.i_sar_height != 0 {
        let mut sar: ::core::ffi::c_double = (*p_param).vui.i_sar_width
            as ::core::ffi::c_double
            / (*p_param).vui.i_sar_height as ::core::ffi::c_double;
        if sar > 1.0f64 {
            i_display_width = (i_display_width as ::core::ffi::c_double * sar)
                as uint32_t;
        } else {
            i_display_height = (i_display_height as ::core::ffi::c_double / sar)
                as uint32_t;
        }
        (*(*p_mp4).summary).par_h = (*p_param).vui.i_sar_width as uint32_t;
        (*(*p_mp4).summary).par_v = (*p_param).vui.i_sar_height as uint32_t;
    }
    (*(*p_mp4).summary).color.primaries_index = (*p_param).vui.i_colorprim as uint16_t;
    (*(*p_mp4).summary).color.transfer_index = (*p_param).vui.i_transfer as uint16_t;
    (*(*p_mp4).summary).color.matrix_index = (if (*p_param).vui.i_colmatrix
        >= 0 as ::core::ffi::c_int
    {
        (*p_param).vui.i_colmatrix
    } else {
        ISOM_MATRIX_INDEX_UNSPECIFIED as ::core::ffi::c_int
    }) as uint16_t;
    (*(*p_mp4).summary).color.full_range = (if (*p_param).vui.b_fullrange
        >= 0 as ::core::ffi::c_int
    {
        (*p_param).vui.b_fullrange
    } else {
        0 as ::core::ffi::c_int
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
    let mut track_mode: lsmash_track_mode = (ISOM_TRACK_ENABLED as ::core::ffi::c_int
        | ISOM_TRACK_IN_MOVIE as ::core::ffi::c_int
        | ISOM_TRACK_IN_PREVIEW as ::core::ffi::c_int) as lsmash_track_mode;
    track_param.mode = track_mode;
    track_param.display_width = i_display_width;
    track_param.display_height = i_display_height;
    if lsmash_set_track_parameters((*p_mp4).p_root, (*p_mp4).i_track, &mut track_param)
        != 0
    {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to set track parameters for video.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    let mut media_param: lsmash_media_parameters_t = lsmash_media_parameters_t {
        handler_type: 0 as lsmash_media_type,
        timescale: 0,
        duration: 0,
        roll_grouping: 0,
        rap_grouping: 0,
        MAC_language: 0,
        ISO_language: 0,
        media_handler_name: 0 as *mut ::core::ffi::c_char,
        data_handler_name: 0 as *mut ::core::ffi::c_char,
        media_handler_name_shadow: [0; 256],
        data_handler_name_shadow: [0; 256],
        compact_sample_size_table: 0,
        no_sample_dependency_table: 0,
        reserved: [0; 2],
    };
    lsmash_initialize_media_parameters(&mut media_param);
    media_param.timescale = i_media_timescale as uint32_t;
    media_param.media_handler_name = b"L-SMASH Video Media Handler\0" as *const u8
        as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    if (*p_mp4).b_use_recovery != 0 {
        media_param.roll_grouping = (*p_param).b_intra_refresh as uint8_t;
        media_param.rap_grouping = (*p_param).b_open_gop as uint8_t;
    }
    if lsmash_set_media_parameters((*p_mp4).p_root, (*p_mp4).i_track, &mut media_param)
        != 0
    {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to set media parameters for video.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*p_mp4).i_video_timescale = lsmash_get_media_timescale(
        (*p_mp4).p_root,
        (*p_mp4).i_track,
    );
    if (*p_mp4).i_video_timescale == 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"media timescale for video is broken.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "294:1"]
unsafe extern "C" fn write_headers(
    mut handle: hnd_t,
    mut p_nal: *mut x264_nal_t,
) -> ::core::ffi::c_int {
    let mut p_mp4: *mut mp4_hnd_t = handle as *mut mp4_hnd_t;
    let mut sps_size: uint32_t = ((*p_nal.offset(0 as ::core::ffi::c_int as isize))
        .i_payload - H264_NALU_LENGTH_SIZE) as uint32_t;
    let mut pps_size: uint32_t = ((*p_nal.offset(1 as ::core::ffi::c_int as isize))
        .i_payload - H264_NALU_LENGTH_SIZE) as uint32_t;
    let mut sei_size: uint32_t = (*p_nal.offset(2 as ::core::ffi::c_int as isize))
        .i_payload as uint32_t;
    let mut sps: *mut uint8_t = (*p_nal.offset(0 as ::core::ffi::c_int as isize))
        .p_payload
        .offset(H264_NALU_LENGTH_SIZE as isize);
    let mut pps: *mut uint8_t = (*p_nal.offset(1 as ::core::ffi::c_int as isize))
        .p_payload
        .offset(H264_NALU_LENGTH_SIZE as isize);
    let mut sei: *mut uint8_t = (*p_nal.offset(2 as ::core::ffi::c_int as isize))
        .p_payload;
    let mut cs: *mut lsmash_codec_specific_t = lsmash_create_codec_specific_data(
        LSMASH_CODEC_SPECIFIC_DATA_TYPE_ISOM_VIDEO_H264,
        LSMASH_CODEC_SPECIFIC_FORMAT_STRUCTURED,
    );
    let mut param: *mut lsmash_h264_specific_parameters_t = (*cs).data.structured
        as *mut lsmash_h264_specific_parameters_t;
    (*param).lengthSizeMinusOne = (H264_NALU_LENGTH_SIZE - 1 as ::core::ffi::c_int)
        as uint8_t;
    if lsmash_append_h264_parameter_set(
        param,
        H264_PARAMETER_SET_TYPE_SPS,
        sps as *mut ::core::ffi::c_void,
        sps_size,
    ) != 0
    {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to append SPS.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if lsmash_append_h264_parameter_set(
        param,
        H264_PARAMETER_SET_TYPE_PPS,
        pps as *mut ::core::ffi::c_void,
        pps_size,
    ) != 0
    {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to append PPS.\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if lsmash_add_codec_specific_data((*p_mp4).summary as *mut lsmash_summary_t, cs) != 0
    {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to add H.264 specific info.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
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
        (*p_mp4).summary as *mut ::core::ffi::c_void,
    ) as uint32_t;
    if (*p_mp4).i_sample_entry == 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to add sample entry for video.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*p_mp4).p_sei_buffer = malloc(sei_size as size_t) as *mut uint8_t;
    if (*p_mp4).p_sei_buffer.is_null() {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to allocate sei transition buffer.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    memcpy(
        (*p_mp4).p_sei_buffer as *mut ::core::ffi::c_void,
        sei as *const ::core::ffi::c_void,
        sei_size as size_t,
    );
    (*p_mp4).i_sei_size = sei_size;
    return sei_size.wrapping_add(sps_size).wrapping_add(pps_size) as ::core::ffi::c_int;
}
#[c2rust::src_loc = "357:1"]
unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: ::core::ffi::c_int,
    mut p_picture: *mut x264_picture_t,
) -> ::core::ffi::c_int {
    let mut p_mp4: *mut mp4_hnd_t = handle as *mut mp4_hnd_t;
    let mut dts: uint64_t = 0;
    let mut cts: uint64_t = 0;
    if (*p_mp4).i_numframe == 0 {
        (*p_mp4).i_start_offset = (*p_picture).i_dts
            * -(1 as ::core::ffi::c_int) as int64_t;
        (*p_mp4).i_first_cts = if (*p_mp4).b_dts_compress != 0 {
            0 as uint64_t
        } else {
            ((*p_mp4).i_start_offset as uint64_t).wrapping_mul((*p_mp4).i_time_inc)
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
            if lsmash_create_explicit_timeline_map(
                (*p_mp4).p_root,
                (*p_mp4).i_track,
                edit,
            ) != 0
            {
                x264_cli_log(
                    b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"failed to set timeline map for video.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
        }
    }
    let mut p_sample: *mut lsmash_sample_t = lsmash_create_sample(
        (i_size as uint32_t).wrapping_add((*p_mp4).i_sei_size),
    );
    if p_sample.is_null() {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to create a video sample data.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if !(*p_mp4).p_sei_buffer.is_null() {
        memcpy(
            (*p_sample).data as *mut ::core::ffi::c_void,
            (*p_mp4).p_sei_buffer as *const ::core::ffi::c_void,
            (*p_mp4).i_sei_size as size_t,
        );
        free((*p_mp4).p_sei_buffer as *mut ::core::ffi::c_void);
        (*p_mp4).p_sei_buffer = 0 as *mut uint8_t;
    }
    memcpy(
        (*p_sample).data.offset((*p_mp4).i_sei_size as isize)
            as *mut ::core::ffi::c_void,
        p_nalu as *const ::core::ffi::c_void,
        i_size as size_t,
    );
    (*p_mp4).i_sei_size = 0 as uint32_t;
    if (*p_mp4).b_dts_compress != 0 {
        if (*p_mp4).i_numframe == 1 as ::core::ffi::c_int {
            (*p_mp4).i_init_delta = (((*p_picture).i_dts + (*p_mp4).i_start_offset)
                as uint64_t)
                .wrapping_mul((*p_mp4).i_time_inc) as int64_t;
        }
        dts = if (*p_mp4).i_numframe > (*p_mp4).i_delay_frames {
            ((*p_picture).i_dts as uint64_t).wrapping_mul((*p_mp4).i_time_inc)
        } else {
            ((*p_mp4).i_numframe as int64_t
                * ((*p_mp4).i_init_delta
                    / (*p_mp4).i_dts_compress_multiplier as int64_t)) as uint64_t
        };
        cts = ((*p_picture).i_pts as uint64_t).wrapping_mul((*p_mp4).i_time_inc);
    } else {
        dts = (((*p_picture).i_dts + (*p_mp4).i_start_offset) as uint64_t)
            .wrapping_mul((*p_mp4).i_time_inc);
        cts = (((*p_picture).i_pts + (*p_mp4).i_start_offset) as uint64_t)
            .wrapping_mul((*p_mp4).i_time_inc);
    }
    (*p_sample).dts = dts;
    (*p_sample).cts = cts;
    (*p_sample).index = (*p_mp4).i_sample_entry;
    (*p_sample).prop.ra_flags = (if (*p_picture).b_keyframe != 0 {
        ISOM_SAMPLE_RANDOM_ACCESS_FLAG_SYNC as ::core::ffi::c_int
    } else {
        ISOM_SAMPLE_RANDOM_ACCESS_FLAG_NONE as ::core::ffi::c_int
    }) as lsmash_random_access_flag;
    if (*p_mp4).b_fragments != 0 && (*p_mp4).i_numframe != 0
        && (*p_sample).prop.ra_flags as ::core::ffi::c_uint
            != ISOM_SAMPLE_RANDOM_ACCESS_FLAG_NONE as ::core::ffi::c_int
                as ::core::ffi::c_uint
    {
        if lsmash_flush_pooled_samples(
            (*p_mp4).p_root,
            (*p_mp4).i_track,
            (*p_sample).dts.wrapping_sub((*p_mp4).i_prev_dts) as uint32_t,
        ) != 0
        {
            x264_cli_log(
                b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"failed to flush the rest of samples.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if lsmash_create_fragment_movie((*p_mp4).p_root) != 0 {
            x264_cli_log(
                b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"failed to create a movie fragment.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    if lsmash_append_sample((*p_mp4).p_root, (*p_mp4).i_track, p_sample) != 0 {
        x264_cli_log(
            b"mp4\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"failed to append a video frame.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*p_mp4).i_prev_dts = dts;
    (*p_mp4).i_numframe += 1;
    return i_size;
}
#[no_mangle]
#[c2rust::src_loc = "429:20"]
pub static mut mp4_output: cli_output_t = unsafe {
    {
        let mut init = cli_output_t {
            open_file: Some(
                open_file
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        *mut hnd_t,
                        *mut cli_output_opt_t,
                    ) -> ::core::ffi::c_int,
            ),
            set_param: Some(
                set_param
                    as unsafe extern "C" fn(
                        hnd_t,
                        *mut x264_param_t,
                    ) -> ::core::ffi::c_int,
            ),
            write_headers: Some(
                write_headers
                    as unsafe extern "C" fn(hnd_t, *mut x264_nal_t) -> ::core::ffi::c_int,
            ),
            write_frame: Some(
                write_frame
                    as unsafe extern "C" fn(
                        hnd_t,
                        *mut uint8_t,
                        ::core::ffi::c_int,
                        *mut x264_picture_t,
                    ) -> ::core::ffi::c_int,
            ),
            close_file: Some(
                close_file
                    as unsafe extern "C" fn(
                        hnd_t,
                        int64_t,
                        int64_t,
                    ) -> ::core::ffi::c_int,
            ),
        };
        init
    }
};
