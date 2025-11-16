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
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:26"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = u8;
    #[c2rust::src_loc = "40:1"]
    pub type __uint16_t = u16;
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
#[c2rust::header_src = "/usr/include/bits/types/struct_FILE.h:26"]
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:26"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/bits/types/struct_timespec.h:26"]
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
#[c2rust::header_src = "/usr/include/bits/struct_stat.h:26"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:26"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::__int64_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:26"]
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
    #[c2rust::src_loc = "290:9"]
    pub const X264_LOG_WARNING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "291:9"]
    pub const X264_LOG_INFO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::internal::__va_list_tag;
    use super::stdint_intn_h::int64_t;
    extern "C" {
        #[c2rust::src_loc = "80:16"]
        pub type x264_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264cli.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/output/output.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/output/flv_bytestream.h:26"]
pub mod flv_bytestream_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "114:16"]
    pub struct flv_buffer {
        pub data: *mut uint8_t,
        pub d_cur: ::core::ffi::c_uint,
        pub d_max: ::core::ffi::c_uint,
        pub fp: *mut FILE,
        pub d_total: uint64_t,
    }
    #[c2rust::src_loc = "88:5"]
    pub const FLV_CODECID_H264: C2RustUnnamed_6 = 7;
    #[c2rust::src_loc = "94:5"]
    pub const FLV_FRAME_INTER: C2RustUnnamed_7 = 32;
    #[c2rust::src_loc = "93:5"]
    pub const FLV_FRAME_KEY: C2RustUnnamed_7 = 16;
    #[c2rust::src_loc = "56:5"]
    pub const FLV_TAG_TYPE_VIDEO: C2RustUnnamed_5 = 9;
    #[c2rust::src_loc = "106:5"]
    pub const AMF_DATA_TYPE_MIXEDARRAY: C2RustUnnamed_8 = 8;
    #[c2rust::src_loc = "101:5"]
    pub const AMF_DATA_TYPE_STRING: C2RustUnnamed_8 = 2;
    #[c2rust::src_loc = "57:5"]
    pub const FLV_TAG_TYPE_META: C2RustUnnamed_5 = 18;
    #[c2rust::src_loc = "53:1"]
    pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
    #[c2rust::src_loc = "55:5"]
    pub const FLV_TAG_TYPE_AUDIO: C2RustUnnamed_5 = 8;
    #[c2rust::src_loc = "86:1"]
    pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
    #[c2rust::src_loc = "91:1"]
    pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
    #[c2rust::src_loc = "97:9"]
    pub type C2RustUnnamed_8 = ::core::ffi::c_uint;
    #[c2rust::src_loc = "111:5"]
    pub const AMF_DATA_TYPE_UNSUPPORTED: C2RustUnnamed_8 = 13;
    #[c2rust::src_loc = "110:5"]
    pub const AMF_DATA_TYPE_LONG_STRING: C2RustUnnamed_8 = 12;
    #[c2rust::src_loc = "109:5"]
    pub const AMF_DATA_TYPE_DATE: C2RustUnnamed_8 = 11;
    #[c2rust::src_loc = "108:5"]
    pub const AMF_DATA_TYPE_ARRAY: C2RustUnnamed_8 = 10;
    #[c2rust::src_loc = "107:5"]
    pub const AMF_DATA_TYPE_OBJECT_END: C2RustUnnamed_8 = 9;
    #[c2rust::src_loc = "105:5"]
    pub const AMF_DATA_TYPE_REFERENCE: C2RustUnnamed_8 = 7;
    #[c2rust::src_loc = "104:5"]
    pub const AMF_DATA_TYPE_UNDEFINED: C2RustUnnamed_8 = 6;
    #[c2rust::src_loc = "103:5"]
    pub const AMF_DATA_TYPE_NULL: C2RustUnnamed_8 = 5;
    #[c2rust::src_loc = "102:5"]
    pub const AMF_DATA_TYPE_OBJECT: C2RustUnnamed_8 = 3;
    #[c2rust::src_loc = "100:5"]
    pub const AMF_DATA_TYPE_BOOL: C2RustUnnamed_8 = 1;
    #[c2rust::src_loc = "99:5"]
    pub const AMF_DATA_TYPE_NUMBER: C2RustUnnamed_8 = 0;
    #[c2rust::src_loc = "45:9"]
    pub const AMF_END_OF_OBJECT: ::core::ffi::c_int = 0x9 as ::core::ffi::c_int;
    use super::stdint_uintn_h::{uint8_t, uint64_t, uint32_t, uint16_t};
    use super::FILE_h::FILE;
    extern "C" {
        #[c2rust::src_loc = "123:1"]
        pub fn flv_create_writer(
            filename: *const ::core::ffi::c_char,
        ) -> *mut flv_buffer;
        #[c2rust::src_loc = "124:1"]
        pub fn flv_append_data(
            c: *mut flv_buffer,
            data: *mut uint8_t,
            size: ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "126:1"]
        pub fn flv_flush_data(c: *mut flv_buffer) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "127:1"]
        pub fn flv_rewrite_amf_be24(
            c: *mut flv_buffer,
            length: ::core::ffi::c_uint,
            start: ::core::ffi::c_uint,
        );
        #[c2rust::src_loc = "129:1"]
        pub fn flv_dbl2int(value: ::core::ffi::c_double) -> uint64_t;
        #[c2rust::src_loc = "130:1"]
        pub fn flv_put_byte(c: *mut flv_buffer, b: uint8_t);
        #[c2rust::src_loc = "131:1"]
        pub fn flv_put_be32(c: *mut flv_buffer, val: uint32_t);
        #[c2rust::src_loc = "133:1"]
        pub fn flv_put_be16(c: *mut flv_buffer, val: uint16_t);
        #[c2rust::src_loc = "134:1"]
        pub fn flv_put_be24(c: *mut flv_buffer, val: uint32_t);
        #[c2rust::src_loc = "135:1"]
        pub fn flv_put_tag(c: *mut flv_buffer, tag: *const ::core::ffi::c_char);
        #[c2rust::src_loc = "136:1"]
        pub fn flv_put_amf_string(c: *mut flv_buffer, str: *const ::core::ffi::c_char);
        #[c2rust::src_loc = "137:1"]
        pub fn flv_put_amf_double(c: *mut flv_buffer, d: ::core::ffi::c_double);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:26"]
pub mod stdio_h {
    #[c2rust::src_loc = "110:9"]
    pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::FILE_h::FILE;
    use super::__stddef_size_t_h::size_t;
    use super::types_h::__off64_t;
    extern "C" {
        #[c2rust::src_loc = "187:1"]
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "735:1"]
        pub fn fwrite(
            __ptr: *const ::core::ffi::c_void,
            __size: size_t,
            __n: size_t,
            __s: *mut FILE,
        ) -> ::core::ffi::c_ulong;
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
}
#[c2rust::header_src = "/usr/include/sys/stat.h:26"]
pub mod stat_h {
    use super::struct_stat_h::stat;
    extern "C" {
        #[c2rust::src_loc = "230:1"]
        pub fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:26"]
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
    #[inline(always)]
    #[c2rust::src_loc = "475:1"]
    pub unsafe extern "C" fn endian_fix32(mut x: uint32_t) -> uint32_t {
        return (x << 24 as ::core::ffi::c_int)
            .wrapping_add(x << 8 as ::core::ffi::c_int & 0xff0000 as uint32_t)
            .wrapping_add(x >> 8 as ::core::ffi::c_int & 0xff00 as uint32_t)
            .wrapping_add(x >> 24 as ::core::ffi::c_int);
    }
    #[inline(always)]
    #[c2rust::src_loc = "487:1"]
    pub unsafe extern "C" fn endian_fix64(mut x: uint64_t) -> uint64_t {
        return (endian_fix32((x >> 32 as ::core::ffi::c_int) as uint32_t) as uint64_t)
            .wrapping_add(
                (endian_fix32(x as uint32_t) as uint64_t) << 32 as ::core::ffi::c_int,
            );
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
    use super::stdint_uintn_h::{uint32_t, uint64_t};
}
#[c2rust::header_src = "/usr/include/string.h:26"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "43:1"]
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> *mut ::core::ffi::c_void;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:26"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/bits/stat.h:26"]
pub mod bits_stat_h {
    #[c2rust::src_loc = "29:9"]
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
}
pub use self::internal::__va_list_tag;
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{
    __uint8_t, __uint16_t, __uint32_t, __int64_t, __uint64_t, __dev_t, __uid_t, __gid_t,
    __ino_t, __mode_t, __nlink_t, __off_t, __off64_t, __time_t, __blksize_t, __blkcnt_t,
    __syscall_slong_t,
};
pub use self::struct_FILE_h::{
    _IO_FILE, _IO_lock_t, _IO_wide_data, _IO_codecvt, _IO_marker,
};
pub use self::FILE_h::FILE;
pub use self::struct_timespec_h::timespec;
pub use self::struct_stat_h::stat;
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::x264_h::{
    x264_nal_t, x264_zone_t, x264_param_t, C2RustUnnamed, C2RustUnnamed_0,
    C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4, x264_hrd_t,
    x264_sei_payload_t, x264_sei_t, x264_image_t, x264_image_properties_t,
    x264_picture_t, X264_LOG_WARNING, X264_LOG_INFO, x264_t,
};
pub use self::x264cli_h::{hnd_t, x264_cli_log};
pub use self::output_h::{cli_output_opt_t, cli_output_t};
pub use self::flv_bytestream_h::{
    flv_buffer, FLV_CODECID_H264, FLV_FRAME_INTER, FLV_FRAME_KEY, FLV_TAG_TYPE_VIDEO,
    AMF_DATA_TYPE_MIXEDARRAY, AMF_DATA_TYPE_STRING, FLV_TAG_TYPE_META, C2RustUnnamed_5,
    FLV_TAG_TYPE_AUDIO, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8,
    AMF_DATA_TYPE_UNSUPPORTED, AMF_DATA_TYPE_LONG_STRING, AMF_DATA_TYPE_DATE,
    AMF_DATA_TYPE_ARRAY, AMF_DATA_TYPE_OBJECT_END, AMF_DATA_TYPE_REFERENCE,
    AMF_DATA_TYPE_UNDEFINED, AMF_DATA_TYPE_NULL, AMF_DATA_TYPE_OBJECT,
    AMF_DATA_TYPE_BOOL, AMF_DATA_TYPE_NUMBER, AMF_END_OF_OBJECT, flv_create_writer,
    flv_append_data, flv_flush_data, flv_rewrite_amf_be24, flv_dbl2int, flv_put_byte,
    flv_put_be32, flv_put_be16, flv_put_be24, flv_put_tag, flv_put_amf_string,
    flv_put_amf_double,
};
pub use self::stdio_h::{SEEK_SET, fclose, fwrite, fseeko, ftello, fileno};
use self::stat_h::fstat;
use self::stdlib_h::{malloc, calloc, free};
pub use self::osdep_h::{x264_is_regular_file, endian_fix32, endian_fix64};
use self::string_h::memcpy;
pub use self::__stddef_null_h::NULL;
pub use self::bits_stat_h::__S_IFMT;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "35:9"]
pub struct flv_hnd_t {
    pub c: *mut flv_buffer,
    pub sei: *mut uint8_t,
    pub sei_len: ::core::ffi::c_int,
    pub i_fps_num: int64_t,
    pub i_fps_den: int64_t,
    pub i_framenum: int64_t,
    pub i_framerate_pos: uint64_t,
    pub i_duration_pos: uint64_t,
    pub i_filesize_pos: uint64_t,
    pub i_bitrate_pos: uint64_t,
    pub b_write_length: uint8_t,
    pub i_prev_dts: int64_t,
    pub i_prev_cts: int64_t,
    pub i_delay_time: int64_t,
    pub i_init_delta: int64_t,
    pub i_delay_frames: ::core::ffi::c_int,
    pub d_timebase: ::core::ffi::c_double,
    pub b_vfr_input: ::core::ffi::c_int,
    pub b_dts_compress: ::core::ffi::c_int,
    pub start: ::core::ffi::c_uint,
}
#[c2rust::src_loc = "65:1"]
unsafe extern "C" fn write_header(mut c: *mut flv_buffer) -> ::core::ffi::c_int {
    flv_put_tag(c, b"FLV\0" as *const u8 as *const ::core::ffi::c_char);
    flv_put_byte(c, 1 as uint8_t);
    flv_put_byte(c, 1 as uint8_t);
    flv_put_be32(c, 9 as uint32_t);
    flv_put_be32(c, 0 as uint32_t);
    return flv_flush_data(c);
}
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut ::core::ffi::c_char,
    mut p_handle: *mut hnd_t,
    mut opt: *mut cli_output_opt_t,
) -> ::core::ffi::c_int {
    let mut p_flv: *mut flv_hnd_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<flv_hnd_t>() as size_t,
    ) as *mut flv_hnd_t;
    if !p_flv.is_null() {
        let mut c: *mut flv_buffer = flv_create_writer(psz_filename);
        if !c.is_null() {
            if write_header(c) == 0 {
                (*p_flv).c = c;
                (*p_flv).b_dts_compress = (*opt).use_dts_compress;
                *p_handle = p_flv as hnd_t;
                return 0 as ::core::ffi::c_int;
            }
            fclose((*c).fp);
            free((*c).data as *mut ::core::ffi::c_void);
            free(c as *mut ::core::ffi::c_void);
        }
        free(p_flv as *mut ::core::ffi::c_void);
    }
    *p_handle = NULL as hnd_t;
    return -(1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "103:1"]
unsafe extern "C" fn set_param(
    mut handle: hnd_t,
    mut p_param: *mut x264_param_t,
) -> ::core::ffi::c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    flv_put_byte(c, FLV_TAG_TYPE_META as ::core::ffi::c_int as uint8_t);
    let mut start: ::core::ffi::c_int = (*c).d_cur as ::core::ffi::c_int;
    flv_put_be24(c, 0 as uint32_t);
    flv_put_be24(c, 0 as uint32_t);
    flv_put_be32(c, 0 as uint32_t);
    flv_put_byte(c, AMF_DATA_TYPE_STRING as ::core::ffi::c_int as uint8_t);
    flv_put_amf_string(c, b"onMetaData\0" as *const u8 as *const ::core::ffi::c_char);
    flv_put_byte(c, AMF_DATA_TYPE_MIXEDARRAY as ::core::ffi::c_int as uint8_t);
    flv_put_be32(c, 7 as uint32_t);
    flv_put_amf_string(c, b"width\0" as *const u8 as *const ::core::ffi::c_char);
    flv_put_amf_double(c, (*p_param).i_width as ::core::ffi::c_double);
    flv_put_amf_string(c, b"height\0" as *const u8 as *const ::core::ffi::c_char);
    flv_put_amf_double(c, (*p_param).i_height as ::core::ffi::c_double);
    flv_put_amf_string(c, b"framerate\0" as *const u8 as *const ::core::ffi::c_char);
    if (*p_param).b_vfr_input == 0 {
        flv_put_amf_double(
            c,
            (*p_param).i_fps_num as ::core::ffi::c_double
                / (*p_param).i_fps_den as ::core::ffi::c_double,
        );
    } else {
        (*p_flv).i_framerate_pos = ((*c).d_cur as uint64_t)
            .wrapping_add((*c).d_total)
            .wrapping_add(1 as uint64_t);
        flv_put_amf_double(c, 0 as ::core::ffi::c_int as ::core::ffi::c_double);
    }
    flv_put_amf_string(c, b"videocodecid\0" as *const u8 as *const ::core::ffi::c_char);
    flv_put_amf_double(
        c,
        FLV_CODECID_H264 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    flv_put_amf_string(c, b"duration\0" as *const u8 as *const ::core::ffi::c_char);
    (*p_flv).i_duration_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as uint64_t);
    flv_put_amf_double(c, 0 as ::core::ffi::c_int as ::core::ffi::c_double);
    flv_put_amf_string(c, b"filesize\0" as *const u8 as *const ::core::ffi::c_char);
    (*p_flv).i_filesize_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as uint64_t);
    flv_put_amf_double(c, 0 as ::core::ffi::c_int as ::core::ffi::c_double);
    flv_put_amf_string(c, b"videodatarate\0" as *const u8 as *const ::core::ffi::c_char);
    (*p_flv).i_bitrate_pos = ((*c).d_cur as uint64_t)
        .wrapping_add((*c).d_total)
        .wrapping_add(1 as uint64_t);
    flv_put_amf_double(c, 0 as ::core::ffi::c_int as ::core::ffi::c_double);
    flv_put_amf_string(c, b"\0" as *const u8 as *const ::core::ffi::c_char);
    flv_put_byte(c, AMF_END_OF_OBJECT as uint8_t);
    let mut length: ::core::ffi::c_uint = (*c)
        .d_cur
        .wrapping_sub(start as ::core::ffi::c_uint);
    flv_rewrite_amf_be24(
        c,
        length.wrapping_sub(10 as ::core::ffi::c_uint),
        start as ::core::ffi::c_uint,
    );
    flv_put_be32(c, (length as uint32_t).wrapping_add(1 as uint32_t));
    (*p_flv).i_fps_num = (*p_param).i_fps_num as int64_t;
    (*p_flv).i_fps_den = (*p_param).i_fps_den as int64_t;
    (*p_flv).d_timebase = (*p_param).i_timebase_num as ::core::ffi::c_double
        / (*p_param).i_timebase_den as ::core::ffi::c_double;
    (*p_flv).b_vfr_input = (*p_param).b_vfr_input;
    (*p_flv).i_delay_frames = if (*p_param).i_bframe != 0 {
        if (*p_param).i_bframe_pyramid != 0 {
            2 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }
    } else {
        0 as ::core::ffi::c_int
    };
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "169:1"]
unsafe extern "C" fn write_headers(
    mut handle: hnd_t,
    mut p_nal: *mut x264_nal_t,
) -> ::core::ffi::c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    let mut sps_size: ::core::ffi::c_int = (*p_nal
        .offset(0 as ::core::ffi::c_int as isize))
        .i_payload;
    let mut pps_size: ::core::ffi::c_int = (*p_nal
        .offset(1 as ::core::ffi::c_int as isize))
        .i_payload;
    let mut sei_size: ::core::ffi::c_int = (*p_nal
        .offset(2 as ::core::ffi::c_int as isize))
        .i_payload;
    (*p_flv).sei = malloc(sei_size as size_t) as *mut uint8_t;
    if (*p_flv).sei.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*p_flv).sei_len = sei_size;
    memcpy(
        (*p_flv).sei as *mut ::core::ffi::c_void,
        (*p_nal.offset(2 as ::core::ffi::c_int as isize)).p_payload
            as *const ::core::ffi::c_void,
        sei_size as size_t,
    );
    let mut sps: *mut uint8_t = (*p_nal.offset(0 as ::core::ffi::c_int as isize))
        .p_payload
        .offset(4 as ::core::ffi::c_int as isize);
    flv_put_byte(c, FLV_TAG_TYPE_VIDEO as ::core::ffi::c_int as uint8_t);
    flv_put_be24(c, 0 as uint32_t);
    flv_put_be24(c, 0 as uint32_t);
    flv_put_byte(c, 0 as uint8_t);
    flv_put_be24(c, 0 as uint32_t);
    (*p_flv).start = (*c).d_cur;
    flv_put_byte(
        c,
        (FLV_FRAME_KEY as ::core::ffi::c_int | FLV_CODECID_H264 as ::core::ffi::c_int)
            as uint8_t,
    );
    flv_put_byte(c, 0 as uint8_t);
    flv_put_be24(c, 0 as uint32_t);
    flv_put_byte(c, 1 as uint8_t);
    flv_put_byte(c, *sps.offset(1 as ::core::ffi::c_int as isize));
    flv_put_byte(c, *sps.offset(2 as ::core::ffi::c_int as isize));
    flv_put_byte(c, *sps.offset(3 as ::core::ffi::c_int as isize));
    flv_put_byte(c, 0xff as uint8_t);
    flv_put_byte(c, 0xe1 as uint8_t);
    flv_put_be16(c, (sps_size - 4 as ::core::ffi::c_int) as uint16_t);
    flv_append_data(c, sps, (sps_size - 4 as ::core::ffi::c_int) as ::core::ffi::c_uint);
    flv_put_byte(c, 1 as uint8_t);
    flv_put_be16(c, (pps_size - 4 as ::core::ffi::c_int) as uint16_t);
    flv_append_data(
        c,
        (*p_nal.offset(1 as ::core::ffi::c_int as isize))
            .p_payload
            .offset(4 as ::core::ffi::c_int as isize),
        (pps_size - 4 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    let mut length: ::core::ffi::c_uint = (*c).d_cur.wrapping_sub((*p_flv).start);
    flv_rewrite_amf_be24(
        c,
        length,
        (*p_flv).start.wrapping_sub(10 as ::core::ffi::c_uint),
    );
    flv_put_be32(c, (length as uint32_t).wrapping_add(11 as uint32_t));
    if flv_flush_data(c) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return sei_size + sps_size + pps_size;
}
#[c2rust::src_loc = "227:1"]
unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: ::core::ffi::c_int,
    mut p_picture: *mut x264_picture_t,
) -> ::core::ffi::c_int {
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    if (*p_flv).i_framenum == 0 {
        (*p_flv).i_delay_time = (*p_picture).i_dts
            * -(1 as ::core::ffi::c_int) as int64_t;
        if (*p_flv).b_dts_compress == 0 && (*p_flv).i_delay_time != 0 {
            x264_cli_log(
                b"flv\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_INFO,
                b"initial delay %ld ms\n\0" as *const u8 as *const ::core::ffi::c_char,
                (((*p_picture).i_pts + (*p_flv).i_delay_time) as ::core::ffi::c_double
                    * (*p_flv).d_timebase
                    * 1000 as ::core::ffi::c_int as ::core::ffi::c_double + 0.5f64)
                    as int64_t,
            );
        }
    }
    let mut dts: int64_t = 0;
    let mut cts: int64_t = 0;
    let mut offset: int64_t = 0;
    if (*p_flv).b_dts_compress != 0 {
        if (*p_flv).i_framenum == 1 as int64_t {
            (*p_flv).i_init_delta = (((*p_picture).i_dts + (*p_flv).i_delay_time)
                as ::core::ffi::c_double * (*p_flv).d_timebase
                * 1000 as ::core::ffi::c_int as ::core::ffi::c_double + 0.5f64)
                as int64_t;
        }
        dts = if (*p_flv).i_framenum > (*p_flv).i_delay_frames as int64_t {
            ((*p_picture).i_dts as ::core::ffi::c_double * (*p_flv).d_timebase
                * 1000 as ::core::ffi::c_int as ::core::ffi::c_double + 0.5f64)
                as int64_t
        } else {
            (*p_flv).i_framenum * (*p_flv).i_init_delta
                / ((*p_flv).i_delay_frames + 1 as ::core::ffi::c_int) as int64_t
        };
        cts = ((*p_picture).i_pts as ::core::ffi::c_double * (*p_flv).d_timebase
            * 1000 as ::core::ffi::c_int as ::core::ffi::c_double + 0.5f64) as int64_t;
    } else {
        dts = (((*p_picture).i_dts + (*p_flv).i_delay_time) as ::core::ffi::c_double
            * (*p_flv).d_timebase * 1000 as ::core::ffi::c_int as ::core::ffi::c_double
            + 0.5f64) as int64_t;
        cts = (((*p_picture).i_pts + (*p_flv).i_delay_time) as ::core::ffi::c_double
            * (*p_flv).d_timebase * 1000 as ::core::ffi::c_int as ::core::ffi::c_double
            + 0.5f64) as int64_t;
    }
    offset = cts - dts;
    if (*p_flv).i_framenum != 0 {
        if (*p_flv).i_prev_dts == dts {
            x264_cli_log(
                b"flv\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_WARNING,
                b"duplicate DTS %ld generated by rounding\n               decoding framerate cannot exceed 1000fps\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                dts,
            );
        }
        if (*p_flv).i_prev_cts == cts {
            x264_cli_log(
                b"flv\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_WARNING,
                b"duplicate CTS %ld generated by rounding\n               composition framerate cannot exceed 1000fps\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                cts,
            );
        }
    }
    (*p_flv).i_prev_dts = dts;
    (*p_flv).i_prev_cts = cts;
    flv_put_byte(c, FLV_TAG_TYPE_VIDEO as ::core::ffi::c_int as uint8_t);
    flv_put_be24(c, 0 as uint32_t);
    flv_put_be24(c, dts as uint32_t);
    flv_put_byte(c, (dts >> 24 as ::core::ffi::c_int) as uint8_t);
    flv_put_be24(c, 0 as uint32_t);
    (*p_flv).start = (*c).d_cur;
    flv_put_byte(
        c,
        ((if (*p_picture).b_keyframe != 0 {
            FLV_FRAME_KEY as ::core::ffi::c_int
        } else {
            FLV_FRAME_INTER as ::core::ffi::c_int
        }) | FLV_CODECID_H264 as ::core::ffi::c_int) as uint8_t,
    );
    flv_put_byte(c, 1 as uint8_t);
    flv_put_be24(c, offset as uint32_t);
    if !(*p_flv).sei.is_null() {
        flv_append_data(c, (*p_flv).sei, (*p_flv).sei_len as ::core::ffi::c_uint);
        free((*p_flv).sei as *mut ::core::ffi::c_void);
        (*p_flv).sei = 0 as *mut uint8_t;
    }
    flv_append_data(c, p_nalu, i_size as ::core::ffi::c_uint);
    let mut length: ::core::ffi::c_uint = (*c).d_cur.wrapping_sub((*p_flv).start);
    flv_rewrite_amf_be24(
        c,
        length,
        (*p_flv).start.wrapping_sub(10 as ::core::ffi::c_uint),
    );
    flv_put_be32(c, (11 as uint32_t).wrapping_add(length as uint32_t));
    if flv_flush_data(c) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    (*p_flv).i_framenum += 1;
    return i_size;
}
#[c2rust::src_loc = "304:1"]
unsafe extern "C" fn rewrite_amf_double(
    mut fp: *mut FILE,
    mut position: uint64_t,
    mut value: ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut x: uint64_t = endian_fix64(flv_dbl2int(value));
    return if fseeko(fp, position as __off64_t, SEEK_SET) == 0
        && fwrite(
            &mut x as *mut uint64_t as *const ::core::ffi::c_void,
            8 as size_t,
            1 as size_t,
            fp,
        ) == 1 as ::core::ffi::c_ulong
    {
        0 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
#[c2rust::src_loc = "317:1"]
unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut largest_pts: int64_t,
    mut second_largest_pts: int64_t,
) -> ::core::ffi::c_int {
    let mut total_duration: ::core::ffi::c_double = 0.;
    let mut current_block: u64;
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut p_flv: *mut flv_hnd_t = handle as *mut flv_hnd_t;
    let mut c: *mut flv_buffer = (*p_flv).c;
    if !(flv_flush_data(c) < 0 as ::core::ffi::c_int) {
        total_duration = 0.;
        if (*p_flv).i_framenum == 1 as int64_t {
            total_duration = if (*p_flv).i_fps_num != 0 {
                (*p_flv).i_fps_den as ::core::ffi::c_double
                    / (*p_flv).i_fps_num as ::core::ffi::c_double
            } else {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            };
        } else {
            total_duration = (2 as int64_t * largest_pts - second_largest_pts)
                as ::core::ffi::c_double * (*p_flv).d_timebase;
        }
        if x264_is_regular_file((*c).fp) != 0
            && total_duration > 0 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            let mut framerate: ::core::ffi::c_double = 0.;
            let mut filesize: int64_t = ftello((*c).fp) as int64_t;
            if (*p_flv).i_framerate_pos != 0 {
                framerate = (*p_flv).i_framenum as ::core::ffi::c_double
                    / total_duration;
                if rewrite_amf_double((*c).fp, (*p_flv).i_framerate_pos, framerate)
                    < 0 as ::core::ffi::c_int
                {
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
                    if rewrite_amf_double(
                        (*c).fp,
                        (*p_flv).i_duration_pos,
                        total_duration,
                    ) < 0 as ::core::ffi::c_int
                    {
                        current_block = 8311716385155032230;
                    } else if rewrite_amf_double(
                        (*c).fp,
                        (*p_flv).i_filesize_pos,
                        filesize as ::core::ffi::c_double,
                    ) < 0 as ::core::ffi::c_int
                    {
                        current_block = 8311716385155032230;
                    } else if rewrite_amf_double(
                        (*c).fp,
                        (*p_flv).i_bitrate_pos,
                        filesize as ::core::ffi::c_double * 8.0f64
                            / (total_duration
                                * 1000 as ::core::ffi::c_int as ::core::ffi::c_double),
                    ) < 0 as ::core::ffi::c_int
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
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    fclose((*c).fp);
    free((*c).data as *mut ::core::ffi::c_void);
    free(c as *mut ::core::ffi::c_void);
    free(p_flv as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "358:20"]
pub static mut flv_output: cli_output_t = unsafe {
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
