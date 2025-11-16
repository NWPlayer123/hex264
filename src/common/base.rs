use ::c2rust_bitfields;
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
    #[c2rust::src_loc = "61:9"]
    pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:27"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stdarg___gnuc_va_list.h:27"]
pub mod __stdarg___gnuc_va_list_h {
    #[c2rust::src_loc = "12:1"]
    pub type __gnuc_va_list = __builtin_va_list;
    use super::internal::__builtin_va_list;
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
    #[c2rust::src_loc = "152:1"]
    pub type __off_t = ::core::ffi::c_long;
    #[c2rust::src_loc = "153:1"]
    pub type __off64_t = ::core::ffi::c_long;
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
#[c2rust::header_src = "/usr/include/bits/types/FILE.h:27"]
pub mod FILE_h {
    #[c2rust::src_loc = "7:1"]
    pub type FILE = _IO_FILE;
    use super::struct_FILE_h::_IO_FILE;
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    #[c2rust::src_loc = "53:1"]
    pub type va_list = __gnuc_va_list;
    #[c2rust::src_loc = "110:9"]
    pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "112:9"]
    pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::FILE_h::FILE;
    use super::__stdarg___gnuc_va_list_h::__gnuc_va_list;
    use super::__stddef_size_t_h::size_t;
    use super::internal::__va_list_tag;
    use super::types_h::__off64_t;
    extern "C" {
        #[c2rust::src_loc = "151:14"]
        pub static mut stderr: *mut FILE;
        #[c2rust::src_loc = "187:1"]
        pub fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
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
        #[c2rust::src_loc = "450:1"]
        pub fn sscanf(
            __s: *const ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "728:1"]
        pub fn fread(
            __ptr: *mut ::core::ffi::c_void,
            __size: size_t,
            __n: size_t,
            __stream: *mut FILE,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "802:1"]
        pub fn fseeko(
            __stream: *mut FILE,
            __off: __off64_t,
            __whence: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "805:1"]
        pub fn ftello(__stream: *mut FILE) -> __off64_t;
    }
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
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint32_t, __uint64_t, __uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:27"]
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
    #[c2rust::src_loc = "146:9"]
    pub const X264_CPU_SSSE3: ::core::ffi::c_uint =
        (1 as ::core::ffi::c_uint) << 6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "160:9"]
    pub const X264_CPU_SSE2_IS_SLOW: ::core::ffi::c_uint =
        (1 as ::core::ffi::c_uint) << 19 as ::core::ffi::c_int;
    #[c2rust::src_loc = "161:9"]
    pub const X264_CPU_SSE2_IS_FAST: ::core::ffi::c_uint =
        (1 as ::core::ffi::c_uint) << 20 as ::core::ffi::c_int;
    #[c2rust::src_loc = "196:9"]
    pub const X264_ANALYSE_I4x4: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
    #[c2rust::src_loc = "197:9"]
    pub const X264_ANALYSE_I8x8: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
    #[c2rust::src_loc = "198:9"]
    pub const X264_ANALYSE_PSUB16x16: ::core::ffi::c_uint = 0x10 as ::core::ffi::c_uint;
    #[c2rust::src_loc = "199:9"]
    pub const X264_ANALYSE_PSUB8x8: ::core::ffi::c_uint = 0x20 as ::core::ffi::c_uint;
    #[c2rust::src_loc = "200:9"]
    pub const X264_ANALYSE_BSUB16x16: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
    #[c2rust::src_loc = "203:9"]
    pub const X264_DIRECT_PRED_SPATIAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "205:9"]
    pub const X264_DIRECT_PRED_AUTO: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "206:9"]
    pub const X264_ME_DIA: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "207:9"]
    pub const X264_ME_HEX: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "208:9"]
    pub const X264_ME_UMH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "210:9"]
    pub const X264_ME_TESA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "211:9"]
    pub const X264_CQM_FLAT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "212:9"]
    pub const X264_CQM_JVT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "213:9"]
    pub const X264_CQM_CUSTOM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "214:9"]
    pub const X264_RC_CQP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "215:9"]
    pub const X264_RC_CRF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "216:9"]
    pub const X264_RC_ABR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "217:9"]
    pub const X264_QP_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "218:9"]
    pub const X264_AQ_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "219:9"]
    pub const X264_AQ_VARIANCE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "220:9"]
    pub const X264_AQ_AUTOVARIANCE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "222:9"]
    pub const X264_B_ADAPT_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "223:9"]
    pub const X264_B_ADAPT_FAST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "224:9"]
    pub const X264_B_ADAPT_TRELLIS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "225:9"]
    pub const X264_WEIGHTP_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "226:9"]
    pub const X264_WEIGHTP_SIMPLE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "227:9"]
    pub const X264_WEIGHTP_SMART: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "230:9"]
    pub const X264_B_PYRAMID_NORMAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "231:9"]
    pub const X264_KEYINT_MIN_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "232:9"]
    pub const X264_KEYINT_MAX_INFINITE: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int;
    #[c2rust::src_loc = "235:9"]
    pub const X264_AVCINTRA_FLAVOR_PANASONIC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
    #[c2rust::src_loc = "243:27"]
    pub static mut x264_fullrange_names: [*const ::core::ffi::c_char; 3] = [
        b"off\0" as *const u8 as *const ::core::ffi::c_char,
        b"on\0" as *const u8 as *const ::core::ffi::c_char,
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
    #[c2rust::src_loc = "250:27"]
    pub static mut x264_nal_hrd_names: [*const ::core::ffi::c_char; 4] = [
        b"none\0" as *const u8 as *const ::core::ffi::c_char,
        b"vbr\0" as *const u8 as *const ::core::ffi::c_char,
        b"cbr\0" as *const u8 as *const ::core::ffi::c_char,
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
    #[c2rust::src_loc = "266:9"]
    pub const X264_CSP_V210: ::core::ffi::c_int = 0xb as ::core::ffi::c_int;
    #[c2rust::src_loc = "267:9"]
    pub const X264_CSP_I444: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;
    #[c2rust::src_loc = "272:9"]
    pub const X264_CSP_MAX: ::core::ffi::c_int = 0x11 as ::core::ffi::c_int;
    #[c2rust::src_loc = "274:9"]
    pub const X264_CSP_HIGH_DEPTH: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "277:9"]
    pub const X264_TYPE_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0;
    #[c2rust::src_loc = "290:9"]
    pub const X264_LOG_WARNING: ::core::ffi::c_int = 1;
    #[c2rust::src_loc = "291:9"]
    pub const X264_LOG_INFO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "292:9"]
    pub const X264_LOG_DEBUG: ::core::ffi::c_int = 3;
    #[c2rust::src_loc = "295:9"]
    pub const X264_THREADS_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "296:9"]
    pub const X264_SYNC_LOOKAHEAD_AUTO: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    #[c2rust::src_loc = "299:9"]
    pub const X264_NAL_HRD_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "669:9"]
    pub const X264_PARAM_BAD_NAME: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    #[c2rust::src_loc = "670:9"]
    pub const X264_PARAM_BAD_VALUE: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
    #[c2rust::src_loc = "671:9"]
    pub const X264_PARAM_ALLOC_FAILED: ::core::ffi::c_int = -(3 as ::core::ffi::c_int);
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
    use super::internal::__va_list_tag;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "80:16"]
        pub type x264_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/cpu.h:27"]
pub mod cpu_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "49:9"]
    pub struct x264_cpu_name_t {
        pub name: *const ::core::ffi::c_char,
        pub flags: uint32_t,
    }
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "29:10"]
        pub fn x264_cpu_detect() -> uint32_t;
        #[c2rust::src_loc = "54:39"]
        pub static x264_cpu_names: [x264_cpu_name_t; 0];
    }
}
#[c2rust::header_src = "/usr/include/ctype.h:27"]
pub mod ctype_h {
    #[c2rust::src_loc = "51:3"]
    pub const _ISdigit: C2RustUnnamed_5 = 2048;
    #[c2rust::src_loc = "46:1"]
    pub type C2RustUnnamed_5 = ::core::ffi::c_uint;
    #[c2rust::src_loc = "59:3"]
    pub const _ISalnum: C2RustUnnamed_5 = 8;
    #[c2rust::src_loc = "58:3"]
    pub const _ISpunct: C2RustUnnamed_5 = 4;
    #[c2rust::src_loc = "57:3"]
    pub const _IScntrl: C2RustUnnamed_5 = 2;
    #[c2rust::src_loc = "56:3"]
    pub const _ISblank: C2RustUnnamed_5 = 1;
    #[c2rust::src_loc = "55:3"]
    pub const _ISgraph: C2RustUnnamed_5 = 32768;
    #[c2rust::src_loc = "54:3"]
    pub const _ISprint: C2RustUnnamed_5 = 16384;
    #[c2rust::src_loc = "53:3"]
    pub const _ISspace: C2RustUnnamed_5 = 8192;
    #[c2rust::src_loc = "52:3"]
    pub const _ISxdigit: C2RustUnnamed_5 = 4096;
    #[c2rust::src_loc = "50:3"]
    pub const _ISalpha: C2RustUnnamed_5 = 1024;
    #[c2rust::src_loc = "49:3"]
    pub const _ISlower: C2RustUnnamed_5 = 512;
    #[c2rust::src_loc = "48:3"]
    pub const _ISupper: C2RustUnnamed_5 = 256;
    extern "C" {
        #[c2rust::src_loc = "79:1"]
        pub fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:27"]
pub mod base_h {
    #[c2rust::src_loc = "96:5"]
    pub const PROFILE_MAIN: profile_e = 77;
    #[c2rust::src_loc = "100:5"]
    pub const PROFILE_HIGH444_PREDICTIVE: profile_e = 244;
    #[c2rust::src_loc = "99:5"]
    pub const PROFILE_HIGH422: profile_e = 122;
    #[c2rust::src_loc = "98:5"]
    pub const PROFILE_HIGH10: profile_e = 110;
    #[c2rust::src_loc = "97:5"]
    pub const PROFILE_HIGH: profile_e = 100;
    #[c2rust::src_loc = "95:5"]
    pub const PROFILE_BASELINE: profile_e = 66;
    #[c2rust::src_loc = "93:1"]
    pub type profile_e = ::core::ffi::c_uint;
    #[inline(always)]
    #[c2rust::src_loc = "206:1"]
    pub unsafe extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return if v < i_min {
            i_min
        } else if v > i_max {
            i_max
        } else {
            v
        };
    }
}
#[c2rust::header_src = "/usr/include/malloc.h:32"]
pub mod malloc_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "39:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "51:1"]
        pub fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t)
            -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "64:1"]
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        #[c2rust::src_loc = "67:1"]
        pub fn memalign(__alignment: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    }
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
        #[c2rust::src_loc = "156:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "159:1"]
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "187:1"]
        pub fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "246:1"]
        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "293:1"]
        pub fn strcspn(
            __s: *const ::core::ffi::c_char,
            __reject: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "297:1"]
        pub fn strspn(
            __s: *const ::core::ffi::c_char,
            __accept: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "350:1"]
        pub fn strstr(
            __haystack: *const ::core::ffi::c_char,
            __needle: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "366:1"]
        pub fn strtok_r(
            __s: *mut ::core::ffi::c_char,
            __delim: *const ::core::ffi::c_char,
            __save_ptr: *mut *mut ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/limits.h:27"]
pub mod limits_h {
    #[c2rust::src_loc = "50:9"]
    pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
    use super::internal::__INT_MAX__;
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
#[c2rust::header_src = "/usr/include/sys/mman.h:27"]
pub mod mman_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "94:1"]
        pub fn madvise(
            __addr: *mut ::core::ffi::c_void,
            __len: size_t,
            __advice: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:27"]
pub mod osdep_h {
    #[c2rust::src_loc = "317:9"]
    pub const NATIVE_ALIGN: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
    #[c2rust::src_loc = "452:9"]
    pub const WORD_SIZE: uint64_t = ::core::mem::size_of::<*mut ::core::ffi::c_void>() as uint64_t;
    use super::stdint_uintn_h::uint64_t;
}
#[c2rust::header_src = "/usr/include/stdint.h:27"]
pub mod stdint_h {
    #[c2rust::src_loc = "112:10"]
    pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
    #[c2rust::src_loc = "118:10"]
    pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
    #[c2rust::src_loc = "216:11"]
    pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
}
#[c2rust::header_src = "/usr/include/stdlib.h:27"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "118:1"]
        pub fn strtod(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_double;
        #[c2rust::src_loc = "215:1"]
        pub fn strtol(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:27"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264_config.h:27"]
pub mod x264_config_h {
    #[c2rust::src_loc = "4:9"]
    pub const X264_CHROMA_FORMAT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/include/bits/mman-linux.h:27"]
pub mod mman_linux_h {
    #[c2rust::src_loc = "99:10"]
    pub const MADV_HUGEPAGE: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
}
pub use self::__stdarg___gnuc_va_list_h::__gnuc_va_list;
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
pub use self::base_h::{
    profile_e, x264_clip3, PROFILE_BASELINE, PROFILE_HIGH, PROFILE_HIGH10, PROFILE_HIGH422,
    PROFILE_HIGH444_PREDICTIVE, PROFILE_MAIN,
};
pub use self::cpu_h::{x264_cpu_detect, x264_cpu_name_t, x264_cpu_names};
pub use self::ctype_h::{
    C2RustUnnamed_5, _ISalnum, _ISalpha, _ISblank, _IScntrl, _ISdigit, _ISgraph, _ISlower,
    _ISprint, _ISpunct, _ISspace, _ISupper, _ISxdigit, __ctype_b_loc,
};
pub use self::internal::{__builtin_va_list, __va_list_tag, __INT_MAX__};
pub use self::limits_h::INT_MAX;
use self::malloc_h::{free, malloc, memalign, realloc};
use self::mman_h::madvise;
pub use self::mman_linux_h::MADV_HUGEPAGE;
pub use self::osdep_h::{NATIVE_ALIGN, WORD_SIZE};
pub use self::stdint_h::{INT32_MAX, SIZE_MAX, UINT32_MAX};
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint32_t, uint64_t, uint8_t};
pub use self::stdio_h::{
    fclose, fopen, fprintf, fread, fseeko, ftello, sprintf, sscanf, stderr, va_list, vfprintf,
    SEEK_END, SEEK_SET,
};
use self::stdlib_h::{strtod, strtol};
use self::string_h::{
    memset, strchr, strcmp, strcspn, strdup, strlen, strncmp, strspn, strstr, strtok_r,
};
use self::strings_h::{strcasecmp, strncasecmp};
pub use self::struct_FILE_h::{_IO_codecvt, _IO_lock_t, _IO_marker, _IO_wide_data, _IO_FILE};
pub use self::types_h::{__int64_t, __off64_t, __off_t, __uint32_t, __uint64_t, __uint8_t};
pub use self::x264_config_h::X264_CHROMA_FORMAT;
pub use self::x264_h::{
    pic_struct_e, x264_avcintra_flavor_names, x264_b_pyramid_names, x264_colmatrix_names,
    x264_colorprim_names, x264_direct_pred_names, x264_fullrange_names, x264_hrd_t,
    x264_image_properties_t, x264_image_t, x264_motion_est_names, x264_nal_hrd_names, x264_nal_t,
    x264_overscan_names, x264_param_t, x264_picture_t, x264_preset_names, x264_sei_payload_t,
    x264_sei_t, x264_t, x264_transfer_names, x264_vidformat_names, x264_zone_t, C2RustUnnamed,
    C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4,
    X264_ANALYSE_BSUB16x16, X264_ANALYSE_I4x4, X264_ANALYSE_I8x8, X264_ANALYSE_PSUB16x16,
    X264_ANALYSE_PSUB8x8, PIC_STRUCT_AUTO, PIC_STRUCT_BOTTOM_TOP, PIC_STRUCT_BOTTOM_TOP_BOTTOM,
    PIC_STRUCT_DOUBLE, PIC_STRUCT_PROGRESSIVE, PIC_STRUCT_TOP_BOTTOM, PIC_STRUCT_TOP_BOTTOM_TOP,
    PIC_STRUCT_TRIPLE, X264_AQ_AUTOVARIANCE, X264_AQ_NONE, X264_AQ_VARIANCE,
    X264_AVCINTRA_FLAVOR_PANASONIC, X264_B_ADAPT_FAST, X264_B_ADAPT_NONE, X264_B_ADAPT_TRELLIS,
    X264_B_PYRAMID_NORMAL, X264_CPU_SSE2_IS_FAST, X264_CPU_SSE2_IS_SLOW, X264_CPU_SSSE3,
    X264_CQM_CUSTOM, X264_CQM_FLAT, X264_CQM_JVT, X264_CSP_HIGH_DEPTH, X264_CSP_I400,
    X264_CSP_I420, X264_CSP_I422, X264_CSP_I444, X264_CSP_MASK, X264_CSP_MAX, X264_CSP_NONE,
    X264_CSP_V210, X264_DIRECT_PRED_AUTO, X264_DIRECT_PRED_SPATIAL, X264_KEYINT_MAX_INFINITE,
    X264_KEYINT_MIN_AUTO, X264_LOG_DEBUG, X264_LOG_ERROR, X264_LOG_INFO, X264_LOG_WARNING,
    X264_ME_DIA, X264_ME_HEX, X264_ME_TESA, X264_ME_UMH, X264_NAL_HRD_NONE,
    X264_PARAM_ALLOC_FAILED, X264_PARAM_BAD_NAME, X264_PARAM_BAD_VALUE, X264_QP_AUTO, X264_RC_ABR,
    X264_RC_CQP, X264_RC_CRF, X264_SYNC_LOOKAHEAD_AUTO, X264_THREADS_AUTO, X264_TYPE_AUTO,
    X264_WEIGHTP_NONE, X264_WEIGHTP_SIMPLE, X264_WEIGHTP_SMART,
};
pub use self::FILE_h::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "206:9"]
pub struct strdup_buffer {
    pub size: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub ptr: [*mut ::core::ffi::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "279:13"]
pub struct x264_csp_tab_t {
    pub planes: ::core::ffi::c_int,
    pub width_fix8: [::core::ffi::c_int; 3],
    pub height_fix8: [::core::ffi::c_int; 3],
}
#[no_mangle]
#[c2rust::src_loc = "62:1"]
pub unsafe extern "C" fn x264_reduce_fraction(mut n: *mut uint32_t, mut d: *mut uint32_t) {
    let mut a: uint32_t = *n;
    let mut b: uint32_t = *d;
    let mut c: uint32_t = 0;
    if a == 0 || b == 0 {
        return;
    }
    c = a.wrapping_rem(b);
    while c != 0 {
        a = b;
        b = c;
        c = a.wrapping_rem(b);
    }
    *n = (*n).wrapping_div(b);
    *d = (*d).wrapping_div(b);
}
#[no_mangle]
#[c2rust::src_loc = "63:1"]
pub unsafe extern "C" fn x264_reduce_fraction64(mut n: *mut uint64_t, mut d: *mut uint64_t) {
    let mut a: uint64_t = *n;
    let mut b: uint64_t = *d;
    let mut c: uint64_t = 0;
    if a == 0 || b == 0 {
        return;
    }
    c = a.wrapping_rem(b);
    while c != 0 {
        a = b;
        b = c;
        c = a.wrapping_rem(b);
    }
    *n = (*n).wrapping_div(b);
    *d = (*d).wrapping_div(b);
}
#[no_mangle]
#[c2rust::src_loc = "68:1"]
pub unsafe extern "C" fn x264_log_default(
    mut p_unused: *mut ::core::ffi::c_void,
    mut i_level: ::core::ffi::c_int,
    mut psz_fmt: *const ::core::ffi::c_char,
    mut arg: ::core::ffi::VaList,
) {
    let mut psz_prefix: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    match i_level {
        X264_LOG_ERROR => {
            psz_prefix =
                b"error\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        X264_LOG_WARNING => {
            psz_prefix =
                b"warning\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        X264_LOG_INFO => {
            psz_prefix =
                b"info\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        X264_LOG_DEBUG => {
            psz_prefix =
                b"debug\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        _ => {
            psz_prefix =
                b"unknown\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
    }
    fprintf(
        stderr,
        b"x264 [%s]: \0" as *const u8 as *const ::core::ffi::c_char,
        psz_prefix,
    );
    vfprintf(stderr, psz_fmt, arg.as_va_list() as ::core::ffi::VaList);
}
#[no_mangle]
#[c2rust::src_loc = "93:1"]
pub unsafe extern "C" fn x264_log_internal(
    mut i_level: ::core::ffi::c_int,
    mut psz_fmt: *const ::core::ffi::c_char,
    mut args: ...
) {
    let mut arg: ::core::ffi::VaListImpl;
    arg = args.clone();
    x264_log_default(NULL, i_level, psz_fmt, arg.as_va_list());
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn x264_malloc(mut i_size: int64_t) -> *mut ::core::ffi::c_void {
    if i_size < 0 as int64_t
        || i_size as uint64_t > (SIZE_MAX as uint64_t).wrapping_sub(HUGE_PAGE_SIZE as uint64_t)
    {
        x264_log_internal(
            X264_LOG_ERROR,
            b"invalid size of malloc: %ld\n\0" as *const u8 as *const ::core::ffi::c_char,
            i_size,
        );
        return NULL;
    }
    let mut align_buf: *mut uint8_t = 0 as *mut uint8_t;
    if i_size >= (HUGE_PAGE_SIZE * 7 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as int64_t {
        align_buf = memalign(HUGE_PAGE_SIZE as size_t, i_size as size_t) as *mut uint8_t;
        if !align_buf.is_null() {
            let mut madv_size: size_t = (i_size + HUGE_PAGE_SIZE as int64_t
                - (HUGE_PAGE_SIZE * 7 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as int64_t
                & !(HUGE_PAGE_SIZE - 1 as ::core::ffi::c_int) as int64_t)
                as size_t;
            madvise(
                align_buf as *mut ::core::ffi::c_void,
                madv_size,
                MADV_HUGEPAGE,
            );
        }
    } else {
        align_buf = memalign(NATIVE_ALIGN as size_t, i_size as size_t) as *mut uint8_t;
    }
    if align_buf.is_null() {
        x264_log_internal(
            X264_LOG_ERROR,
            b"malloc of size %ld failed\n\0" as *const u8 as *const ::core::ffi::c_char,
            i_size,
        );
    }
    return align_buf as *mut ::core::ffi::c_void;
}
#[c2rust::src_loc = "106:9"]
pub const HUGE_PAGE_SIZE: ::core::ffi::c_int =
    2 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "149:1"]
pub unsafe extern "C" fn x264_free(mut p: *mut ::core::ffi::c_void) {
    if !p.is_null() {
        free(p);
    }
}
#[no_mangle]
#[c2rust::src_loc = "164:1"]
pub unsafe extern "C" fn x264_slurp_file(
    mut filename: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut b_error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_size: int64_t = 0;
    let mut buf: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut fh: *mut FILE =
        fopen(filename, b"rb\0" as *const u8 as *const ::core::ffi::c_char) as *mut FILE;
    if fh.is_null() {
        return 0 as *mut ::core::ffi::c_char;
    }
    b_error |=
        (fseeko(fh, 0 as __off64_t, SEEK_END) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    i_size = ftello(fh) as int64_t;
    b_error |= (i_size <= 0 as int64_t) as ::core::ffi::c_int;
    if WORD_SIZE == 4 as uint64_t {
        b_error |= (i_size > INT32_MAX as int64_t) as ::core::ffi::c_int;
    }
    b_error |=
        (fseeko(fh, 0 as __off64_t, SEEK_SET) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if !(b_error != 0) {
        buf = x264_malloc(i_size + 2 as int64_t) as *mut ::core::ffi::c_char;
        if !buf.is_null() {
            b_error |= (fread(
                buf as *mut ::core::ffi::c_void,
                1 as size_t,
                i_size as size_t,
                fh,
            ) as uint64_t
                != i_size as uint64_t) as ::core::ffi::c_int;
            fclose(fh);
            if b_error != 0 {
                x264_free(buf as *mut ::core::ffi::c_void);
                return 0 as *mut ::core::ffi::c_char;
            }
            if *buf.offset((i_size - 1 as int64_t) as isize) as ::core::ffi::c_int != '\n' as i32 {
                let fresh11 = i_size;
                i_size = i_size + 1;
                *buf.offset(fresh11 as isize) = '\n' as i32 as ::core::ffi::c_char;
            }
            *buf.offset(i_size as isize) = '\0' as i32 as ::core::ffi::c_char;
            return buf;
        }
    }
    fclose(fh);
    return 0 as *mut ::core::ffi::c_char;
}
#[c2rust::src_loc = "213:9"]
pub const BUFFER_DEFAULT_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
#[c2rust::src_loc = "215:1"]
pub unsafe extern "C" fn x264_param_strdup(
    mut param: *mut x264_param_t,
    mut src: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut res: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut current_block: u64;
    let mut buf: *mut strdup_buffer = (*param).opaque as *mut strdup_buffer;
    if buf.is_null() {
        buf = malloc(
            (8 as ::core::ffi::c_ulong as ::core::ffi::c_int as size_t).wrapping_add(
                (BUFFER_DEFAULT_SIZE as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
            ),
        ) as *mut strdup_buffer;
        if buf.is_null() {
            current_block = 14623623056652471472;
        } else {
            (*buf).size = BUFFER_DEFAULT_SIZE;
            (*buf).count = 0 as ::core::ffi::c_int;
            (*param).opaque = buf as *mut ::core::ffi::c_void;
            current_block = 11650488183268122163;
        }
    } else if (*buf).count == (*buf).size {
        if (*buf).size
            > (INT_MAX - 8 as ::core::ffi::c_ulong as ::core::ffi::c_int)
                / 2 as ::core::ffi::c_int
                / ::core::mem::size_of::<*mut ::core::ffi::c_void>() as ::core::ffi::c_int
        {
            current_block = 14623623056652471472;
        } else {
            let mut new_size: ::core::ffi::c_int = (*buf).size * 2 as ::core::ffi::c_int;
            buf = realloc(
                buf as *mut ::core::ffi::c_void,
                (8 as ::core::ffi::c_ulong as ::core::ffi::c_int as size_t).wrapping_add(
                    (new_size as size_t)
                        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t),
                ),
            ) as *mut strdup_buffer;
            if buf.is_null() {
                current_block = 14623623056652471472;
            } else {
                (*buf).size = new_size;
                (*param).opaque = buf as *mut ::core::ffi::c_void;
                current_block = 11650488183268122163;
            }
        }
    } else {
        current_block = 11650488183268122163;
    }
    match current_block {
        11650488183268122163 => {
            res = strdup(src);
            if !res.is_null() {
                let fresh0 = (*buf).count;
                (*buf).count = (*buf).count + 1;
                let ref mut fresh1 = *(*buf).ptr.as_mut_ptr().offset(fresh0 as isize);
                *fresh1 = res as *mut ::core::ffi::c_void;
                return res;
            }
        }
        _ => {}
    }
    x264_log_internal(
        X264_LOG_ERROR,
        b"x264_param_strdup failed\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return 0 as *mut ::core::ffi::c_char;
}
#[no_mangle]
#[c2rust::src_loc = "251:1"]
pub unsafe extern "C" fn x264_param_cleanup(mut param: *mut x264_param_t) {
    let mut buf: *mut strdup_buffer = (*param).opaque as *mut strdup_buffer;
    if !buf.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*buf).count {
            free(*(*buf).ptr.as_mut_ptr().offset(i as isize));
            i += 1;
        }
        free(buf as *mut ::core::ffi::c_void);
        (*param).opaque = NULL;
    }
}
#[no_mangle]
#[c2rust::src_loc = "266:1"]
pub unsafe extern "C" fn x264_picture_init(mut pic: *mut x264_picture_t) {
    memset(
        pic as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<x264_picture_t>() as size_t,
    );
    (*pic).i_type = X264_TYPE_AUTO;
    (*pic).i_qpplus1 = X264_QP_AUTO;
    (*pic).i_pic_struct = PIC_STRUCT_AUTO as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "277:1"]
pub unsafe extern "C" fn x264_picture_alloc(
    mut pic: *mut x264_picture_t,
    mut i_csp: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    static mut csp_tab: [x264_csp_tab_t; 17] = [
        x264_csp_tab_t {
            planes: 0,
            width_fix8: [0; 3],
            height_fix8: [0; 3],
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as ::core::ffi::c_int,
                width_fix8: [256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int, 0, 0],
                height_fix8: [256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int, 0, 0],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as ::core::ffi::c_int,
                width_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                ],
                height_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as ::core::ffi::c_int,
                width_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                ],
                height_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 2 as ::core::ffi::c_int,
                width_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    0,
                ],
                height_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                    0,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 2 as ::core::ffi::c_int,
                width_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    0,
                ],
                height_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                    0,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as ::core::ffi::c_int,
                width_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                ],
                height_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as ::core::ffi::c_int,
                width_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int / 2 as ::core::ffi::c_int,
                ],
                height_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 2 as ::core::ffi::c_int,
                width_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    0,
                ],
                height_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    0,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as ::core::ffi::c_int,
                width_fix8: [256 as ::core::ffi::c_int * 2 as ::core::ffi::c_int, 0, 0],
                height_fix8: [256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int, 0, 0],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as ::core::ffi::c_int,
                width_fix8: [256 as ::core::ffi::c_int * 2 as ::core::ffi::c_int, 0, 0],
                height_fix8: [256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int, 0, 0],
            };
            init
        },
        x264_csp_tab_t {
            planes: 0,
            width_fix8: [0; 3],
            height_fix8: [0; 3],
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as ::core::ffi::c_int,
                width_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                ],
                height_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 3 as ::core::ffi::c_int,
                width_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                ],
                height_fix8: [
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                ],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as ::core::ffi::c_int,
                width_fix8: [256 as ::core::ffi::c_int * 3 as ::core::ffi::c_int, 0, 0],
                height_fix8: [256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int, 0, 0],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as ::core::ffi::c_int,
                width_fix8: [256 as ::core::ffi::c_int * 4 as ::core::ffi::c_int, 0, 0],
                height_fix8: [256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int, 0, 0],
            };
            init
        },
        {
            let mut init = x264_csp_tab_t {
                planes: 1 as ::core::ffi::c_int,
                width_fix8: [256 as ::core::ffi::c_int * 3 as ::core::ffi::c_int, 0, 0],
                height_fix8: [256 as ::core::ffi::c_int * 1 as ::core::ffi::c_int, 0, 0],
            };
            init
        },
    ];
    let mut csp: ::core::ffi::c_int = i_csp & X264_CSP_MASK;
    if csp <= X264_CSP_NONE || csp >= X264_CSP_MAX || csp == X264_CSP_V210 {
        return -(1 as ::core::ffi::c_int);
    }
    x264_picture_init(pic);
    (*pic).img.i_csp = i_csp;
    (*pic).img.i_plane = csp_tab[csp as usize].planes;
    let mut depth_factor: ::core::ffi::c_int = if i_csp & X264_CSP_HIGH_DEPTH != 0 {
        2 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    let mut plane_offset: [int64_t; 3] = [0 as ::core::ffi::c_int as int64_t, 0, 0];
    let mut frame_size: int64_t = 0 as int64_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*pic).img.i_plane {
        let mut stride: ::core::ffi::c_int =
            ((i_width as int64_t * csp_tab[csp as usize].width_fix8[i as usize] as int64_t
                >> 8 as ::core::ffi::c_int)
                * depth_factor as int64_t) as ::core::ffi::c_int;
        let mut plane_size: int64_t = (i_height as int64_t
            * csp_tab[csp as usize].height_fix8[i as usize] as int64_t
            >> 8 as ::core::ffi::c_int)
            * stride as int64_t;
        (*pic).img.i_stride[i as usize] = stride;
        plane_offset[i as usize] = frame_size;
        frame_size += plane_size;
        i += 1;
    }
    (*pic).img.plane[0 as ::core::ffi::c_int as usize] = x264_malloc(frame_size) as *mut uint8_t;
    if (*pic).img.plane[0 as ::core::ffi::c_int as usize].is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut i_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i_0 < (*pic).img.i_plane {
        (*pic).img.plane[i_0 as usize] = (*pic).img.plane[0 as ::core::ffi::c_int as usize]
            .offset(plane_offset[i_0 as usize] as isize);
        i_0 += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "333:1"]
pub unsafe extern "C" fn x264_picture_clean(mut pic: *mut x264_picture_t) {
    x264_free((*pic).img.plane[0 as ::core::ffi::c_int as usize] as *mut ::core::ffi::c_void);
    memset(
        pic as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<x264_picture_t>() as size_t,
    );
}
#[no_mangle]
#[c2rust::src_loc = "344:1"]
pub unsafe extern "C" fn x264_param_default(mut param: *mut x264_param_t) {
    memset(
        param as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<x264_param_t>() as size_t,
    );
    (*param).cpu = x264_cpu_detect();
    (*param).i_threads = X264_THREADS_AUTO;
    (*param).i_lookahead_threads = X264_THREADS_AUTO;
    (*param).b_deterministic = 1 as ::core::ffi::c_int;
    (*param).i_sync_lookahead = X264_SYNC_LOOKAHEAD_AUTO;
    (*param).i_csp = if X264_CHROMA_FORMAT != 0 {
        X264_CHROMA_FORMAT
    } else {
        X264_CSP_I420
    };
    (*param).i_width = 0 as ::core::ffi::c_int;
    (*param).i_height = 0 as ::core::ffi::c_int;
    (*param).vui.i_sar_width = 0 as ::core::ffi::c_int;
    (*param).vui.i_sar_height = 0 as ::core::ffi::c_int;
    (*param).vui.i_overscan = 0 as ::core::ffi::c_int;
    (*param).vui.i_vidformat = 5 as ::core::ffi::c_int;
    (*param).vui.b_fullrange = -(1 as ::core::ffi::c_int);
    (*param).vui.i_colorprim = 2 as ::core::ffi::c_int;
    (*param).vui.i_transfer = 2 as ::core::ffi::c_int;
    (*param).vui.i_colmatrix = -(1 as ::core::ffi::c_int);
    (*param).vui.i_chroma_loc = 0 as ::core::ffi::c_int;
    (*param).i_fps_num = 25 as uint32_t;
    (*param).i_fps_den = 1 as uint32_t;
    (*param).i_level_idc = -(1 as ::core::ffi::c_int);
    (*param).i_slice_max_size = 0 as ::core::ffi::c_int;
    (*param).i_slice_max_mbs = 0 as ::core::ffi::c_int;
    (*param).i_slice_count = 0 as ::core::ffi::c_int;
    (*param).i_bitdepth = 8 as ::core::ffi::c_int;
    (*param).i_frame_reference = 3 as ::core::ffi::c_int;
    (*param).i_keyint_max = 250 as ::core::ffi::c_int;
    (*param).i_keyint_min = X264_KEYINT_MIN_AUTO;
    (*param).i_bframe = 3 as ::core::ffi::c_int;
    (*param).i_scenecut_threshold = 40 as ::core::ffi::c_int;
    (*param).i_bframe_adaptive = X264_B_ADAPT_FAST;
    (*param).i_bframe_bias = 0 as ::core::ffi::c_int;
    (*param).i_bframe_pyramid = X264_B_PYRAMID_NORMAL;
    (*param).b_interlaced = 0 as ::core::ffi::c_int;
    (*param).b_constrained_intra = 0 as ::core::ffi::c_int;
    (*param).b_deblocking_filter = 1 as ::core::ffi::c_int;
    (*param).i_deblocking_filter_alphac0 = 0 as ::core::ffi::c_int;
    (*param).i_deblocking_filter_beta = 0 as ::core::ffi::c_int;
    (*param).b_cabac = 1 as ::core::ffi::c_int;
    (*param).i_cabac_init_idc = 0 as ::core::ffi::c_int;
    (*param).rc.i_rc_method = X264_RC_CRF;
    (*param).rc.i_bitrate = 0 as ::core::ffi::c_int;
    (*param).rc.f_rate_tolerance = 1.0f32;
    (*param).rc.i_vbv_max_bitrate = 0 as ::core::ffi::c_int;
    (*param).rc.i_vbv_buffer_size = 0 as ::core::ffi::c_int;
    (*param).rc.f_vbv_buffer_init = 0.9f32;
    (*param).rc.i_qp_constant = -(1 as ::core::ffi::c_int);
    (*param).rc.f_rf_constant = 23 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*param).rc.i_qp_min = 0 as ::core::ffi::c_int;
    (*param).rc.i_qp_max = INT_MAX;
    (*param).rc.i_qp_step = 4 as ::core::ffi::c_int;
    (*param).rc.f_ip_factor = 1.4f32;
    (*param).rc.f_pb_factor = 1.3f32;
    (*param).rc.i_aq_mode = X264_AQ_VARIANCE;
    (*param).rc.f_aq_strength = 1.0f32;
    (*param).rc.i_lookahead = 40 as ::core::ffi::c_int;
    (*param).rc.b_stat_write = 0 as ::core::ffi::c_int;
    (*param).rc.psz_stat_out =
        b"x264_2pass.log\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    (*param).rc.b_stat_read = 0 as ::core::ffi::c_int;
    (*param).rc.psz_stat_in =
        b"x264_2pass.log\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    (*param).rc.f_qcompress = 0.6f32;
    (*param).rc.f_qblur = 0.5f32;
    (*param).rc.f_complexity_blur = 20 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*param).rc.i_zones = 0 as ::core::ffi::c_int;
    (*param).rc.b_mb_tree = 1 as ::core::ffi::c_int;
    (*param).pf_log = Some(
        x264_log_default
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                ::core::ffi::VaList,
            ) -> (),
        >;
    (*param).p_log_private = NULL;
    (*param).i_log_level = X264_LOG_INFO;
    (*param).analyse.intra = X264_ANALYSE_I4x4 | X264_ANALYSE_I8x8;
    (*param).analyse.inter =
        X264_ANALYSE_I4x4 | X264_ANALYSE_I8x8 | X264_ANALYSE_PSUB16x16 | X264_ANALYSE_BSUB16x16;
    (*param).analyse.i_direct_mv_pred = X264_DIRECT_PRED_SPATIAL;
    (*param).analyse.i_me_method = X264_ME_HEX;
    (*param).analyse.f_psy_rd = 1.0f32;
    (*param).analyse.b_psy = 1 as ::core::ffi::c_int;
    (*param).analyse.f_psy_trellis = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
    (*param).analyse.i_me_range = 16 as ::core::ffi::c_int;
    (*param).analyse.i_subpel_refine = 7 as ::core::ffi::c_int;
    (*param).analyse.b_mixed_references = 1 as ::core::ffi::c_int;
    (*param).analyse.b_chroma_me = 1 as ::core::ffi::c_int;
    (*param).analyse.i_mv_range_thread = -(1 as ::core::ffi::c_int);
    (*param).analyse.i_mv_range = -(1 as ::core::ffi::c_int);
    (*param).analyse.i_chroma_qp_offset = 0 as ::core::ffi::c_int;
    (*param).analyse.b_fast_pskip = 1 as ::core::ffi::c_int;
    (*param).analyse.b_weighted_bipred = 1 as ::core::ffi::c_int;
    (*param).analyse.i_weighted_pred = X264_WEIGHTP_SMART;
    (*param).analyse.b_dct_decimate = 1 as ::core::ffi::c_int;
    (*param).analyse.b_transform_8x8 = 1 as ::core::ffi::c_int;
    (*param).analyse.i_trellis = 1 as ::core::ffi::c_int;
    (*param).analyse.i_luma_deadzone[0 as ::core::ffi::c_int as usize] = 21 as ::core::ffi::c_int;
    (*param).analyse.i_luma_deadzone[1 as ::core::ffi::c_int as usize] = 11 as ::core::ffi::c_int;
    (*param).analyse.b_psnr = 0 as ::core::ffi::c_int;
    (*param).analyse.b_ssim = 0 as ::core::ffi::c_int;
    (*param).i_cqm_preset = X264_CQM_FLAT;
    memset(
        (*param).cqm_4iy.as_mut_ptr() as *mut ::core::ffi::c_void,
        16 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    memset(
        (*param).cqm_4py.as_mut_ptr() as *mut ::core::ffi::c_void,
        16 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    memset(
        (*param).cqm_4ic.as_mut_ptr() as *mut ::core::ffi::c_void,
        16 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    memset(
        (*param).cqm_4pc.as_mut_ptr() as *mut ::core::ffi::c_void,
        16 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    memset(
        (*param).cqm_8iy.as_mut_ptr() as *mut ::core::ffi::c_void,
        16 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    memset(
        (*param).cqm_8py.as_mut_ptr() as *mut ::core::ffi::c_void,
        16 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    memset(
        (*param).cqm_8ic.as_mut_ptr() as *mut ::core::ffi::c_void,
        16 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    memset(
        (*param).cqm_8pc.as_mut_ptr() as *mut ::core::ffi::c_void,
        16 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    (*param).b_repeat_headers = 1 as ::core::ffi::c_int;
    (*param).b_annexb = 1 as ::core::ffi::c_int;
    (*param).b_aud = 0 as ::core::ffi::c_int;
    (*param).b_vfr_input = 1 as ::core::ffi::c_int;
    (*param).i_nal_hrd = X264_NAL_HRD_NONE;
    (*param).b_tff = 1 as ::core::ffi::c_int;
    (*param).b_pic_struct = 0 as ::core::ffi::c_int;
    (*param).b_fake_interlaced = 0 as ::core::ffi::c_int;
    (*param).i_frame_packing = -(1 as ::core::ffi::c_int);
    (*param).i_alternative_transfer = 2 as ::core::ffi::c_int;
    (*param).b_opencl = 0 as ::core::ffi::c_int;
    (*param).i_opencl_device = 0 as ::core::ffi::c_int;
    (*param).opencl_device_id = NULL;
    (*param).psz_clbin_file = 0 as *mut ::core::ffi::c_char;
    (*param).i_avcintra_class = 0 as ::core::ffi::c_int;
    (*param).i_avcintra_flavor = X264_AVCINTRA_FLAVOR_PANASONIC;
}
#[c2rust::src_loc = "489:1"]
unsafe extern "C" fn param_apply_preset(
    mut param: *mut x264_param_t,
    mut preset: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut end: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut i: ::core::ffi::c_int =
        strtol(preset, &mut end, 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if *end as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && i >= 0 as ::core::ffi::c_int
        && i < (::core::mem::size_of::<[*const ::core::ffi::c_char; 11]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int
    {
        preset = x264_preset_names[i as usize];
    }
    if strcasecmp(
        preset,
        b"ultrafast\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*param).i_frame_reference = 1 as ::core::ffi::c_int;
        (*param).i_scenecut_threshold = 0 as ::core::ffi::c_int;
        (*param).b_deblocking_filter = 0 as ::core::ffi::c_int;
        (*param).b_cabac = 0 as ::core::ffi::c_int;
        (*param).i_bframe = 0 as ::core::ffi::c_int;
        (*param).analyse.intra = 0 as ::core::ffi::c_uint;
        (*param).analyse.inter = 0 as ::core::ffi::c_uint;
        (*param).analyse.b_transform_8x8 = 0 as ::core::ffi::c_int;
        (*param).analyse.i_me_method = X264_ME_DIA;
        (*param).analyse.i_subpel_refine = 0 as ::core::ffi::c_int;
        (*param).rc.i_aq_mode = 0 as ::core::ffi::c_int;
        (*param).analyse.b_mixed_references = 0 as ::core::ffi::c_int;
        (*param).analyse.i_trellis = 0 as ::core::ffi::c_int;
        (*param).i_bframe_adaptive = X264_B_ADAPT_NONE;
        (*param).rc.b_mb_tree = 0 as ::core::ffi::c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_NONE;
        (*param).analyse.b_weighted_bipred = 0 as ::core::ffi::c_int;
        (*param).rc.i_lookahead = 0 as ::core::ffi::c_int;
    } else if strcasecmp(
        preset,
        b"superfast\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*param).analyse.inter = X264_ANALYSE_I8x8 | X264_ANALYSE_I4x4;
        (*param).analyse.i_me_method = X264_ME_DIA;
        (*param).analyse.i_subpel_refine = 1 as ::core::ffi::c_int;
        (*param).i_frame_reference = 1 as ::core::ffi::c_int;
        (*param).analyse.b_mixed_references = 0 as ::core::ffi::c_int;
        (*param).analyse.i_trellis = 0 as ::core::ffi::c_int;
        (*param).rc.b_mb_tree = 0 as ::core::ffi::c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_SIMPLE;
        (*param).rc.i_lookahead = 0 as ::core::ffi::c_int;
    } else if strcasecmp(
        preset,
        b"veryfast\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*param).analyse.i_subpel_refine = 2 as ::core::ffi::c_int;
        (*param).i_frame_reference = 1 as ::core::ffi::c_int;
        (*param).analyse.b_mixed_references = 0 as ::core::ffi::c_int;
        (*param).analyse.i_trellis = 0 as ::core::ffi::c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_SIMPLE;
        (*param).rc.i_lookahead = 10 as ::core::ffi::c_int;
    } else if strcasecmp(
        preset,
        b"faster\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*param).analyse.b_mixed_references = 0 as ::core::ffi::c_int;
        (*param).i_frame_reference = 2 as ::core::ffi::c_int;
        (*param).analyse.i_subpel_refine = 4 as ::core::ffi::c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_SIMPLE;
        (*param).rc.i_lookahead = 20 as ::core::ffi::c_int;
    } else if strcasecmp(preset, b"fast\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*param).i_frame_reference = 2 as ::core::ffi::c_int;
        (*param).analyse.i_subpel_refine = 6 as ::core::ffi::c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_SIMPLE;
        (*param).rc.i_lookahead = 30 as ::core::ffi::c_int;
    } else if !(strcasecmp(
        preset,
        b"medium\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0)
    {
        if strcasecmp(preset, b"slow\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            (*param).analyse.i_subpel_refine = 8 as ::core::ffi::c_int;
            (*param).i_frame_reference = 5 as ::core::ffi::c_int;
            (*param).analyse.i_direct_mv_pred = X264_DIRECT_PRED_AUTO;
            (*param).analyse.i_trellis = 2 as ::core::ffi::c_int;
            (*param).rc.i_lookahead = 50 as ::core::ffi::c_int;
        } else if strcasecmp(
            preset,
            b"slower\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            (*param).analyse.i_me_method = X264_ME_UMH;
            (*param).analyse.i_subpel_refine = 9 as ::core::ffi::c_int;
            (*param).i_frame_reference = 8 as ::core::ffi::c_int;
            (*param).i_bframe_adaptive = X264_B_ADAPT_TRELLIS;
            (*param).analyse.i_direct_mv_pred = X264_DIRECT_PRED_AUTO;
            (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
            (*param).analyse.i_trellis = 2 as ::core::ffi::c_int;
            (*param).rc.i_lookahead = 60 as ::core::ffi::c_int;
        } else if strcasecmp(
            preset,
            b"veryslow\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            (*param).analyse.i_me_method = X264_ME_UMH;
            (*param).analyse.i_subpel_refine = 10 as ::core::ffi::c_int;
            (*param).analyse.i_me_range = 24 as ::core::ffi::c_int;
            (*param).i_frame_reference = 16 as ::core::ffi::c_int;
            (*param).i_bframe_adaptive = X264_B_ADAPT_TRELLIS;
            (*param).analyse.i_direct_mv_pred = X264_DIRECT_PRED_AUTO;
            (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
            (*param).analyse.i_trellis = 2 as ::core::ffi::c_int;
            (*param).i_bframe = 8 as ::core::ffi::c_int;
            (*param).rc.i_lookahead = 60 as ::core::ffi::c_int;
        } else if strcasecmp(
            preset,
            b"placebo\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            (*param).analyse.i_me_method = X264_ME_TESA;
            (*param).analyse.i_subpel_refine = 11 as ::core::ffi::c_int;
            (*param).analyse.i_me_range = 24 as ::core::ffi::c_int;
            (*param).i_frame_reference = 16 as ::core::ffi::c_int;
            (*param).i_bframe_adaptive = X264_B_ADAPT_TRELLIS;
            (*param).analyse.i_direct_mv_pred = X264_DIRECT_PRED_AUTO;
            (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
            (*param).analyse.b_fast_pskip = 0 as ::core::ffi::c_int;
            (*param).analyse.i_trellis = 2 as ::core::ffi::c_int;
            (*param).i_bframe = 16 as ::core::ffi::c_int;
            (*param).rc.i_lookahead = 60 as ::core::ffi::c_int;
        } else {
            x264_log_internal(
                X264_LOG_ERROR,
                b"invalid preset '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                preset,
            );
            return -(1 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "611:1"]
unsafe extern "C" fn param_apply_tune(
    mut param: *mut x264_param_t,
    mut tune: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut psy_tuning_used: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = 0;
    loop {
        tune = tune
            .offset(strspn(tune, b",./-+\0" as *const u8 as *const ::core::ffi::c_char) as isize);
        len = strcspn(tune, b",./-+\0" as *const u8 as *const ::core::ffi::c_char)
            as ::core::ffi::c_int;
        if !(len != 0) {
            break;
        }
        if len == 4 as ::core::ffi::c_int
            && strncasecmp(
                tune,
                b"film\0" as *const u8 as *const ::core::ffi::c_char,
                4 as size_t,
            ) == 0
        {
            let fresh4 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh4 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).i_deblocking_filter_alphac0 = -(1 as ::core::ffi::c_int);
                (*param).i_deblocking_filter_beta = -(1 as ::core::ffi::c_int);
                (*param).analyse.f_psy_trellis = 0.15f32;
                current_block = 11174649648027449784;
            }
        } else if len == 9 as ::core::ffi::c_int
            && strncasecmp(
                tune,
                b"animation\0" as *const u8 as *const ::core::ffi::c_char,
                9 as size_t,
            ) == 0
        {
            let fresh5 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh5 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).i_frame_reference = if (*param).i_frame_reference > 1 as ::core::ffi::c_int
                {
                    (*param).i_frame_reference * 2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
                (*param).i_deblocking_filter_alphac0 = 1 as ::core::ffi::c_int;
                (*param).i_deblocking_filter_beta = 1 as ::core::ffi::c_int;
                (*param).analyse.f_psy_rd = 0.4f32;
                (*param).rc.f_aq_strength = 0.6f32;
                (*param).i_bframe += 2 as ::core::ffi::c_int;
                current_block = 11174649648027449784;
            }
        } else if len == 5 as ::core::ffi::c_int
            && strncasecmp(
                tune,
                b"grain\0" as *const u8 as *const ::core::ffi::c_char,
                5 as size_t,
            ) == 0
        {
            let fresh6 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh6 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).i_deblocking_filter_alphac0 = -(2 as ::core::ffi::c_int);
                (*param).i_deblocking_filter_beta = -(2 as ::core::ffi::c_int);
                (*param).analyse.f_psy_trellis = 0.25f32;
                (*param).analyse.b_dct_decimate = 0 as ::core::ffi::c_int;
                (*param).rc.f_pb_factor = 1.1f32;
                (*param).rc.f_ip_factor = 1.1f32;
                (*param).rc.f_aq_strength = 0.5f32;
                (*param).analyse.i_luma_deadzone[0 as ::core::ffi::c_int as usize] =
                    6 as ::core::ffi::c_int;
                (*param).analyse.i_luma_deadzone[1 as ::core::ffi::c_int as usize] =
                    6 as ::core::ffi::c_int;
                (*param).rc.f_qcompress = 0.8f32;
                current_block = 11174649648027449784;
            }
        } else if len == 10 as ::core::ffi::c_int
            && strncasecmp(
                tune,
                b"stillimage\0" as *const u8 as *const ::core::ffi::c_char,
                10 as size_t,
            ) == 0
        {
            let fresh7 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh7 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).i_deblocking_filter_alphac0 = -(3 as ::core::ffi::c_int);
                (*param).i_deblocking_filter_beta = -(3 as ::core::ffi::c_int);
                (*param).analyse.f_psy_rd = 2.0f32;
                (*param).analyse.f_psy_trellis = 0.7f32;
                (*param).rc.f_aq_strength = 1.2f32;
                current_block = 11174649648027449784;
            }
        } else if len == 4 as ::core::ffi::c_int
            && strncasecmp(
                tune,
                b"psnr\0" as *const u8 as *const ::core::ffi::c_char,
                4 as size_t,
            ) == 0
        {
            let fresh8 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh8 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).rc.i_aq_mode = X264_AQ_NONE;
                (*param).analyse.b_psy = 0 as ::core::ffi::c_int;
                current_block = 11174649648027449784;
            }
        } else if len == 4 as ::core::ffi::c_int
            && strncasecmp(
                tune,
                b"ssim\0" as *const u8 as *const ::core::ffi::c_char,
                4 as size_t,
            ) == 0
        {
            let fresh9 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh9 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).rc.i_aq_mode = X264_AQ_AUTOVARIANCE;
                (*param).analyse.b_psy = 0 as ::core::ffi::c_int;
                current_block = 11174649648027449784;
            }
        } else if len == 10 as ::core::ffi::c_int
            && strncasecmp(
                tune,
                b"fastdecode\0" as *const u8 as *const ::core::ffi::c_char,
                10 as size_t,
            ) == 0
        {
            (*param).b_deblocking_filter = 0 as ::core::ffi::c_int;
            (*param).b_cabac = 0 as ::core::ffi::c_int;
            (*param).analyse.b_weighted_bipred = 0 as ::core::ffi::c_int;
            (*param).analyse.i_weighted_pred = X264_WEIGHTP_NONE;
            current_block = 11174649648027449784;
        } else if len == 11 as ::core::ffi::c_int
            && strncasecmp(
                tune,
                b"zerolatency\0" as *const u8 as *const ::core::ffi::c_char,
                11 as size_t,
            ) == 0
        {
            (*param).rc.i_lookahead = 0 as ::core::ffi::c_int;
            (*param).i_sync_lookahead = 0 as ::core::ffi::c_int;
            (*param).i_bframe = 0 as ::core::ffi::c_int;
            (*param).b_sliced_threads = 1 as ::core::ffi::c_int;
            (*param).b_vfr_input = 0 as ::core::ffi::c_int;
            (*param).rc.b_mb_tree = 0 as ::core::ffi::c_int;
            current_block = 11174649648027449784;
        } else if len == 6 as ::core::ffi::c_int
            && strncasecmp(
                tune,
                b"touhou\0" as *const u8 as *const ::core::ffi::c_char,
                6 as size_t,
            ) == 0
        {
            let fresh10 = psy_tuning_used;
            psy_tuning_used = psy_tuning_used + 1;
            if fresh10 != 0 {
                current_block = 11494378617088087400;
            } else {
                (*param).i_frame_reference = if (*param).i_frame_reference > 1 as ::core::ffi::c_int
                {
                    (*param).i_frame_reference * 2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
                (*param).i_deblocking_filter_alphac0 = -(1 as ::core::ffi::c_int);
                (*param).i_deblocking_filter_beta = -(1 as ::core::ffi::c_int);
                (*param).analyse.f_psy_trellis = 0.2f32;
                (*param).rc.f_aq_strength = 1.3f32;
                if (*param).analyse.inter & X264_ANALYSE_PSUB16x16 != 0 {
                    (*param).analyse.inter |= X264_ANALYSE_PSUB8x8;
                }
                current_block = 11174649648027449784;
            }
        } else {
            x264_log_internal(
                X264_LOG_ERROR,
                b"invalid tune '%.*s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                len,
                tune,
            );
            return -(1 as ::core::ffi::c_int);
        }
        match current_block {
            11494378617088087400 => {
                x264_log_internal(
                    X264_LOG_WARNING,
                    b"only 1 psy tuning can be used: ignoring tune %.*s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    len,
                    tune,
                );
            }
            _ => {}
        }
        tune = tune.offset(len as isize);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "706:1"]
pub unsafe extern "C" fn x264_param_default_preset(
    mut param: *mut x264_param_t,
    mut preset: *const ::core::ffi::c_char,
    mut tune: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    x264_param_default(param);
    if !preset.is_null() && param_apply_preset(param, preset) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    if !tune.is_null() && param_apply_tune(param, tune) < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "717:1"]
pub unsafe extern "C" fn x264_param_apply_fastfirstpass(mut param: *mut x264_param_t) {
    if (*param).rc.b_stat_write != 0 && (*param).rc.b_stat_read == 0 {
        (*param).i_frame_reference = 1 as ::core::ffi::c_int;
        (*param).analyse.b_transform_8x8 = 0 as ::core::ffi::c_int;
        (*param).analyse.inter = 0 as ::core::ffi::c_uint;
        (*param).analyse.i_me_method = X264_ME_DIA;
        (*param).analyse.i_subpel_refine =
            if (2 as ::core::ffi::c_int) < (*param).analyse.i_subpel_refine {
                2 as ::core::ffi::c_int
            } else {
                (*param).analyse.i_subpel_refine
            };
        (*param).analyse.i_trellis = 0 as ::core::ffi::c_int;
        (*param).analyse.b_fast_pskip = 1 as ::core::ffi::c_int;
    }
}
#[c2rust::src_loc = "732:1"]
unsafe extern "C" fn profile_string_to_int(
    mut str: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if strcasecmp(
        str,
        b"baseline\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        return PROFILE_BASELINE as ::core::ffi::c_int;
    }
    if strcasecmp(str, b"main\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return PROFILE_MAIN as ::core::ffi::c_int;
    }
    if strcasecmp(str, b"high\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return PROFILE_HIGH as ::core::ffi::c_int;
    }
    if strcasecmp(str, b"high10\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return PROFILE_HIGH10 as ::core::ffi::c_int;
    }
    if strcasecmp(str, b"high422\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return PROFILE_HIGH422 as ::core::ffi::c_int;
    }
    if strcasecmp(str, b"high444\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        return PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "749:1"]
pub unsafe extern "C" fn x264_param_apply_profile(
    mut param: *mut x264_param_t,
    mut profile: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if profile.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    let qp_bd_offset: ::core::ffi::c_int =
        6 as ::core::ffi::c_int * ((*param).i_bitdepth - 8 as ::core::ffi::c_int);
    let mut p: ::core::ffi::c_int = profile_string_to_int(profile);
    if p < 0 as ::core::ffi::c_int {
        x264_log_internal(
            X264_LOG_ERROR,
            b"invalid profile: %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            profile,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if p < PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int
        && ((*param).rc.i_rc_method == X264_RC_CQP
            && (*param).rc.i_qp_constant <= 0 as ::core::ffi::c_int
            || (*param).rc.i_rc_method == X264_RC_CRF
                && ((*param).rc.f_rf_constant + qp_bd_offset as ::core::ffi::c_float)
                    as ::core::ffi::c_int
                    <= 0 as ::core::ffi::c_int)
    {
        x264_log_internal(
            X264_LOG_ERROR,
            b"%s profile doesn't support lossless\n\0" as *const u8 as *const ::core::ffi::c_char,
            profile,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if p < PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int
        && (*param).i_csp & X264_CSP_MASK >= X264_CSP_I444
    {
        x264_log_internal(
            X264_LOG_ERROR,
            b"%s profile doesn't support 4:4:4\n\0" as *const u8 as *const ::core::ffi::c_char,
            profile,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if p < PROFILE_HIGH422 as ::core::ffi::c_int && (*param).i_csp & X264_CSP_MASK >= X264_CSP_I422
    {
        x264_log_internal(
            X264_LOG_ERROR,
            b"%s profile doesn't support 4:2:2\n\0" as *const u8 as *const ::core::ffi::c_char,
            profile,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if p < PROFILE_HIGH10 as ::core::ffi::c_int && (*param).i_bitdepth > 8 as ::core::ffi::c_int {
        x264_log_internal(
            X264_LOG_ERROR,
            b"%s profile doesn't support a bit depth of %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            profile,
            (*param).i_bitdepth,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if p < PROFILE_HIGH as ::core::ffi::c_int && (*param).i_csp & X264_CSP_MASK == X264_CSP_I400 {
        x264_log_internal(
            X264_LOG_ERROR,
            b"%s profile doesn't support 4:0:0\n\0" as *const u8 as *const ::core::ffi::c_char,
            profile,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if p == PROFILE_BASELINE as ::core::ffi::c_int {
        (*param).analyse.b_transform_8x8 = 0 as ::core::ffi::c_int;
        (*param).b_cabac = 0 as ::core::ffi::c_int;
        (*param).i_cqm_preset = X264_CQM_FLAT;
        (*param).psz_cqm_file = 0 as *mut ::core::ffi::c_char;
        (*param).i_bframe = 0 as ::core::ffi::c_int;
        (*param).analyse.i_weighted_pred = X264_WEIGHTP_NONE;
        if (*param).b_interlaced != 0 {
            x264_log_internal(
                X264_LOG_ERROR,
                b"baseline profile doesn't support interlacing\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
        if (*param).b_fake_interlaced != 0 {
            x264_log_internal(
                X264_LOG_ERROR,
                b"baseline profile doesn't support fake interlacing\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -(1 as ::core::ffi::c_int);
        }
    } else if p == PROFILE_MAIN as ::core::ffi::c_int {
        (*param).analyse.b_transform_8x8 = 0 as ::core::ffi::c_int;
        (*param).i_cqm_preset = X264_CQM_FLAT;
        (*param).psz_cqm_file = 0 as *mut ::core::ffi::c_char;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "816:1"]
unsafe extern "C" fn parse_enum(
    mut arg: *const ::core::ffi::c_char,
    mut names: *const *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*names.offset(i as isize)).is_null() {
        if **names.offset(i as isize) as ::core::ffi::c_int != 0
            && strcasecmp(arg, *names.offset(i as isize)) == 0
        {
            *dst = i;
            return 0 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return -(1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "827:1"]
unsafe extern "C" fn parse_cqm(
    mut str: *const ::core::ffi::c_char,
    mut cqm: *mut uint8_t,
    mut length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut coef: ::core::ffi::c_int = 0;
        if sscanf(
            str,
            b"%d\0" as *const u8 as *const ::core::ffi::c_char,
            &mut coef as *mut ::core::ffi::c_int,
        ) == 0
            || coef < 1 as ::core::ffi::c_int
            || coef > 255 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        let fresh2 = i;
        i = i + 1;
        *cqm.offset(fresh2 as isize) = coef as uint8_t;
        if !(i < length
            && {
                str = strchr(str, ',' as i32);
                !str.is_null()
            }
            && {
                let fresh3 = str;
                str = str.offset(1);
                !fresh3.is_null()
            })
        {
            break;
        }
    }
    return if i == length {
        0 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
#[c2rust::src_loc = "839:1"]
unsafe extern "C" fn atobool_internal(
    mut str: *const ::core::ffi::c_char,
    mut b_error: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if strcmp(str, b"1\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcasecmp(str, b"true\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcasecmp(str, b"yes\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        return 1 as ::core::ffi::c_int;
    }
    if strcmp(str, b"0\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcasecmp(str, b"false\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcasecmp(str, b"no\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    *b_error = 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "853:1"]
unsafe extern "C" fn atoi_internal(
    mut str: *const ::core::ffi::c_char,
    mut b_error: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut end: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut v: ::core::ffi::c_int =
        strtol(str, &mut end, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if end == str as *mut ::core::ffi::c_char || *end as ::core::ffi::c_int != '\0' as i32 {
        *b_error = 1 as ::core::ffi::c_int;
    }
    return v;
}
#[c2rust::src_loc = "862:1"]
unsafe extern "C" fn atof_internal(
    mut str: *const ::core::ffi::c_char,
    mut b_error: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_double {
    let mut end: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut v: ::core::ffi::c_double = strtod(str, &mut end);
    if end == str as *mut ::core::ffi::c_char || *end as ::core::ffi::c_int != '\0' as i32 {
        *b_error = 1 as ::core::ffi::c_int;
    }
    return v;
}
#[no_mangle]
#[c2rust::src_loc = "886:1"]
pub unsafe extern "C" fn x264_param_parse(
    mut p: *mut x264_param_t,
    mut name: *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut name_buf: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut b_error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut errortype: ::core::ffi::c_int = X264_PARAM_BAD_VALUE;
    let mut name_was_bool: ::core::ffi::c_int = 0;
    let mut value_was_null: ::core::ffi::c_int = value.is_null() as ::core::ffi::c_int;
    if name.is_null() {
        return X264_PARAM_BAD_NAME;
    }
    if value.is_null() {
        value = b"true\0" as *const u8 as *const ::core::ffi::c_char;
    }
    if *value.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '=' as i32 {
        value = value.offset(1);
    }
    if !strchr(name, '_' as i32).is_null() {
        let mut c: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
        name_buf = strdup(name);
        if name_buf.is_null() {
            return X264_PARAM_ALLOC_FAILED;
        }
        loop {
            c = strchr(name_buf, '_' as i32);
            if c.is_null() {
                break;
            }
            *c = '-' as i32 as ::core::ffi::c_char;
        }
        name = name_buf;
    }
    if strncmp(
        name,
        b"no\0" as *const u8 as *const ::core::ffi::c_char,
        2 as size_t,
    ) == 0
    {
        name = name.offset(2 as ::core::ffi::c_int as isize);
        if *name.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32 {
            name = name.offset(1);
        }
        name_was_bool = 1 as ::core::ffi::c_int;
        value = if atobool_internal(value, &mut b_error) != 0 {
            b"false\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"true\0" as *const u8 as *const ::core::ffi::c_char
        };
    }
    name_was_bool = 0 as ::core::ffi::c_int;
    if strcmp(name, b"asm\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).cpu = if *(*__ctype_b_loc()).offset(*value.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_uchar
            as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & _ISdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            != 0
        {
            atoi_internal(value, &mut b_error) as uint32_t
        } else if strcasecmp(value, b"auto\0" as *const u8 as *const ::core::ffi::c_char) == 0 || {
            name_was_bool = 1 as ::core::ffi::c_int;
            atobool_internal(value, &mut b_error) != 0
        } {
            x264_cpu_detect()
        } else {
            0 as uint32_t
        };
        if b_error != 0 {
            let mut buf: *mut ::core::ffi::c_char = strdup(value);
            if !buf.is_null() {
                let mut tok: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
                let mut saveptr: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
                let mut init: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
                b_error = 0 as ::core::ffi::c_int;
                (*p).cpu = 0 as uint32_t;
                init = buf;
                loop {
                    tok = strtok_r(
                        init,
                        b",\0" as *const u8 as *const ::core::ffi::c_char,
                        &mut saveptr,
                    );
                    if tok.is_null() {
                        break;
                    }
                    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while (*x264_cpu_names.as_ptr().offset(i as isize)).flags != 0
                        && strcasecmp(tok, (*x264_cpu_names.as_ptr().offset(i as isize)).name) != 0
                    {
                        i += 1;
                    }
                    (*p).cpu |= (*x264_cpu_names.as_ptr().offset(i as isize)).flags;
                    if (*x264_cpu_names.as_ptr().offset(i as isize)).flags == 0 {
                        b_error = 1 as ::core::ffi::c_int;
                    }
                    init = 0 as *mut ::core::ffi::c_char;
                }
                free(buf as *mut ::core::ffi::c_void);
                if (*p).cpu & X264_CPU_SSSE3 as uint32_t != 0
                    && (*p).cpu & X264_CPU_SSE2_IS_SLOW as uint32_t == 0
                {
                    (*p).cpu =
                        ((*p).cpu as ::core::ffi::c_uint | X264_CPU_SSE2_IS_FAST) as uint32_t;
                }
            } else {
                errortype = X264_PARAM_ALLOC_FAILED;
            }
        }
    } else if strcmp(
        name,
        b"threads\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if strcasecmp(value, b"auto\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            (*p).i_threads = X264_THREADS_AUTO;
        } else {
            (*p).i_threads = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(
        name,
        b"lookahead-threads\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if strcasecmp(value, b"auto\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            (*p).i_lookahead_threads = X264_THREADS_AUTO;
        } else {
            (*p).i_lookahead_threads = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(
        name,
        b"sliced-threads\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_sliced_threads = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"sync-lookahead\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if strcasecmp(value, b"auto\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            (*p).i_sync_lookahead = X264_SYNC_LOOKAHEAD_AUTO;
        } else {
            (*p).i_sync_lookahead = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(
        name,
        b"deterministic\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            name,
            b"n-deterministic\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_deterministic = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"cpu-independent\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_cpu_independent = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"level\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcmp(
            name,
            b"level-idc\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        if strcmp(value, b"1b\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
            (*p).i_level_idc = 9 as ::core::ffi::c_int;
        } else if atof_internal(value, &mut b_error)
            < 7 as ::core::ffi::c_int as ::core::ffi::c_double
        {
            (*p).i_level_idc = (10 as ::core::ffi::c_int as ::core::ffi::c_double
                * atof_internal(value, &mut b_error)
                + 0.5f64) as ::core::ffi::c_int;
        } else {
            (*p).i_level_idc = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(
        name,
        b"bluray-compat\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_bluray_compat = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"avcintra-class\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).i_avcintra_class = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"avcintra-flavor\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        b_error |= parse_enum(
            value,
            x264_avcintra_flavor_names.as_ptr(),
            &mut (*p).i_avcintra_flavor,
        );
    } else if strcmp(name, b"sar\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        b_error |= (2 as ::core::ffi::c_int
            != sscanf(
                value,
                b"%d:%d\0" as *const u8 as *const ::core::ffi::c_char,
                &mut (*p).vui.i_sar_width as *mut ::core::ffi::c_int,
                &mut (*p).vui.i_sar_height as *mut ::core::ffi::c_int,
            )
            && 2 as ::core::ffi::c_int
                != sscanf(
                    value,
                    b"%d/%d\0" as *const u8 as *const ::core::ffi::c_char,
                    &mut (*p).vui.i_sar_width as *mut ::core::ffi::c_int,
                    &mut (*p).vui.i_sar_height as *mut ::core::ffi::c_int,
                )) as ::core::ffi::c_int;
    } else if strcmp(
        name,
        b"overscan\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        b_error |= parse_enum(
            value,
            x264_overscan_names.as_ptr(),
            &mut (*p).vui.i_overscan,
        );
    } else if strcmp(
        name,
        b"videoformat\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        b_error |= parse_enum(
            value,
            x264_vidformat_names.as_ptr(),
            &mut (*p).vui.i_vidformat,
        );
    } else if strcmp(
        name,
        b"fullrange\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        b_error |= parse_enum(
            value,
            x264_fullrange_names.as_ptr(),
            &mut (*p).vui.b_fullrange,
        );
    } else if strcmp(
        name,
        b"colorprim\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        b_error |= parse_enum(
            value,
            x264_colorprim_names.as_ptr(),
            &mut (*p).vui.i_colorprim,
        );
    } else if strcmp(
        name,
        b"transfer\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        b_error |= parse_enum(
            value,
            x264_transfer_names.as_ptr(),
            &mut (*p).vui.i_transfer,
        );
    } else if strcmp(
        name,
        b"colormatrix\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        b_error |= parse_enum(
            value,
            x264_colmatrix_names.as_ptr(),
            &mut (*p).vui.i_colmatrix,
        );
    } else if strcmp(
        name,
        b"chromaloc\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).vui.i_chroma_loc = atoi_internal(value, &mut b_error);
        b_error |= ((*p).vui.i_chroma_loc < 0 as ::core::ffi::c_int
            || (*p).vui.i_chroma_loc > 5 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    } else if strcmp(
        name,
        b"mastering-display\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        if strcasecmp(value, b"undef\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
            b_error |= (sscanf(
                value,
                b"G(%d,%d)B(%d,%d)R(%d,%d)WP(%d,%d)L(%ld,%ld)\0" as *const u8
                    as *const ::core::ffi::c_char,
                &mut (*p).mastering_display.i_green_x as *mut ::core::ffi::c_int,
                &mut (*p).mastering_display.i_green_y as *mut ::core::ffi::c_int,
                &mut (*p).mastering_display.i_blue_x as *mut ::core::ffi::c_int,
                &mut (*p).mastering_display.i_blue_y as *mut ::core::ffi::c_int,
                &mut (*p).mastering_display.i_red_x as *mut ::core::ffi::c_int,
                &mut (*p).mastering_display.i_red_y as *mut ::core::ffi::c_int,
                &mut (*p).mastering_display.i_white_x as *mut ::core::ffi::c_int,
                &mut (*p).mastering_display.i_white_y as *mut ::core::ffi::c_int,
                &mut (*p).mastering_display.i_display_max as *mut int64_t,
                &mut (*p).mastering_display.i_display_min as *mut int64_t,
            ) != 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
            (*p).mastering_display.b_mastering_display = (b_error == 0) as ::core::ffi::c_int;
        } else {
            (*p).mastering_display.b_mastering_display = 0 as ::core::ffi::c_int;
        }
    } else if strcmp(name, b"cll\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        if strcasecmp(value, b"undef\0" as *const u8 as *const ::core::ffi::c_char) != 0 {
            b_error |= (sscanf(
                value,
                b"%d,%d\0" as *const u8 as *const ::core::ffi::c_char,
                &mut (*p).content_light_level.i_max_cll as *mut ::core::ffi::c_int,
                &mut (*p).content_light_level.i_max_fall as *mut ::core::ffi::c_int,
            ) != 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
            (*p).content_light_level.b_cll = (b_error == 0) as ::core::ffi::c_int;
        } else {
            (*p).content_light_level.b_cll = 0 as ::core::ffi::c_int;
        }
    } else if strcmp(
        name,
        b"alternative-transfer\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        b_error |= parse_enum(
            value,
            x264_transfer_names.as_ptr(),
            &mut (*p).i_alternative_transfer,
        );
    } else if strcmp(name, b"fps\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut i_fps_num: int64_t = 0;
        let mut i_fps_den: int64_t = 0;
        if sscanf(
            value,
            b"%ld/%ld\0" as *const u8 as *const ::core::ffi::c_char,
            &mut i_fps_num as *mut int64_t,
            &mut i_fps_den as *mut int64_t,
        ) == 2 as ::core::ffi::c_int
        {
            (*p).i_fps_num = i_fps_num as uint32_t;
            (*p).i_fps_den = i_fps_den as uint32_t;
            b_error |= (i_fps_num < 1 as int64_t
                || i_fps_num > UINT32_MAX as int64_t
                || i_fps_den < 1 as int64_t
                || i_fps_den > UINT32_MAX as int64_t) as ::core::ffi::c_int;
        } else {
            let mut fps: ::core::ffi::c_double = atof_internal(value, &mut b_error);
            if fps < 0.0005f64 || fps > INT_MAX as ::core::ffi::c_double {
                b_error = 1 as ::core::ffi::c_int;
            } else if fps <= INT_MAX as ::core::ffi::c_double / 1000.0f64 {
                (*p).i_fps_num = (fps * 1000.0f64 + 0.5f64) as ::core::ffi::c_int as uint32_t;
                (*p).i_fps_den = 1000 as uint32_t;
            } else {
                (*p).i_fps_num = atoi_internal(value, &mut b_error) as uint32_t;
                (*p).i_fps_den = 1 as uint32_t;
            }
        }
    } else if strcmp(name, b"ref\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcmp(
            name,
            b"frameref\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*p).i_frame_reference = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"dpb-size\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).i_dpb_size = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"keyint\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        if !strstr(
            value,
            b"infinite\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null()
        {
            (*p).i_keyint_max = X264_KEYINT_MAX_INFINITE;
        } else {
            (*p).i_keyint_max = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(
        name,
        b"min-keyint\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            name,
            b"keyint-min\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*p).i_keyint_min = atoi_internal(value, &mut b_error);
        if (*p).i_keyint_max < (*p).i_keyint_min {
            (*p).i_keyint_max = (*p).i_keyint_min;
        }
    } else if strcmp(
        name,
        b"scenecut\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).i_scenecut_threshold = atobool_internal(value, &mut b_error);
        if b_error != 0 || (*p).i_scenecut_threshold != 0 {
            b_error = 0 as ::core::ffi::c_int;
            (*p).i_scenecut_threshold = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(
        name,
        b"intra-refresh\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_intra_refresh = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"bframes\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).i_bframe = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"b-adapt\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).i_bframe_adaptive = atobool_internal(value, &mut b_error);
        if b_error != 0 {
            b_error = 0 as ::core::ffi::c_int;
            (*p).i_bframe_adaptive = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(name, b"b-bias\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_bframe_bias = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"b-pyramid\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        b_error |= parse_enum(
            value,
            x264_b_pyramid_names.as_ptr(),
            &mut (*p).i_bframe_pyramid,
        );
        if b_error != 0 {
            b_error = 0 as ::core::ffi::c_int;
            (*p).i_bframe_pyramid = atoi_internal(value, &mut b_error);
        }
    } else if strcmp(
        name,
        b"open-gop\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_open_gop = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"nf\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_deblocking_filter =
            (atobool_internal(value, &mut b_error) == 0) as ::core::ffi::c_int;
    } else if strcmp(name, b"filter\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcmp(
            name,
            b"deblock\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        if 2 as ::core::ffi::c_int
            == sscanf(
                value,
                b"%d:%d\0" as *const u8 as *const ::core::ffi::c_char,
                &mut (*p).i_deblocking_filter_alphac0 as *mut ::core::ffi::c_int,
                &mut (*p).i_deblocking_filter_beta as *mut ::core::ffi::c_int,
            )
            || 2 as ::core::ffi::c_int
                == sscanf(
                    value,
                    b"%d,%d\0" as *const u8 as *const ::core::ffi::c_char,
                    &mut (*p).i_deblocking_filter_alphac0 as *mut ::core::ffi::c_int,
                    &mut (*p).i_deblocking_filter_beta as *mut ::core::ffi::c_int,
                )
        {
            (*p).b_deblocking_filter = 1 as ::core::ffi::c_int;
        } else if sscanf(
            value,
            b"%d\0" as *const u8 as *const ::core::ffi::c_char,
            &mut (*p).i_deblocking_filter_alphac0 as *mut ::core::ffi::c_int,
        ) != 0
        {
            (*p).b_deblocking_filter = 1 as ::core::ffi::c_int;
            (*p).i_deblocking_filter_beta = (*p).i_deblocking_filter_alphac0;
        } else {
            name_was_bool = 1 as ::core::ffi::c_int;
            (*p).b_deblocking_filter = atobool_internal(value, &mut b_error);
        }
    } else if strcmp(
        name,
        b"slice-max-size\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).i_slice_max_size = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"slice-max-mbs\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).i_slice_max_mbs = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"slice-min-mbs\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).i_slice_min_mbs = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"slices\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_slice_count = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"slices-max\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).i_slice_count_max = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"cabac\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_cabac = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"cabac-idc\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).i_cabac_init_idc = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"interlaced\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_interlaced = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"tff\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_tff = atobool_internal(value, &mut b_error);
        (*p).b_interlaced = (*p).b_tff;
    } else if strcmp(name, b"bff\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_interlaced = atobool_internal(value, &mut b_error);
        (*p).b_tff = ((*p).b_interlaced == 0) as ::core::ffi::c_int;
    } else if strcmp(
        name,
        b"constrained-intra\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_constrained_intra = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"cqm\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        if !strstr(value, b"flat\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            (*p).i_cqm_preset = X264_CQM_FLAT;
        } else if !strstr(value, b"jvt\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            (*p).i_cqm_preset = X264_CQM_JVT;
        } else {
            (*p).psz_cqm_file = x264_param_strdup(p, value);
            if (*p).psz_cqm_file.is_null() {
                b_error = 1 as ::core::ffi::c_int;
                errortype = X264_PARAM_ALLOC_FAILED;
            }
        }
    } else if strcmp(
        name,
        b"cqmfile\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).psz_cqm_file = x264_param_strdup(p, value);
        if (*p).psz_cqm_file.is_null() {
            b_error = 1 as ::core::ffi::c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
    } else if strcmp(name, b"cqm4\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4iy.as_mut_ptr(), 16 as ::core::ffi::c_int);
        b_error |= parse_cqm(value, (*p).cqm_4py.as_mut_ptr(), 16 as ::core::ffi::c_int);
        b_error |= parse_cqm(value, (*p).cqm_4ic.as_mut_ptr(), 16 as ::core::ffi::c_int);
        b_error |= parse_cqm(value, (*p).cqm_4pc.as_mut_ptr(), 16 as ::core::ffi::c_int);
    } else if strcmp(name, b"cqm8\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_8iy.as_mut_ptr(), 64 as ::core::ffi::c_int);
        b_error |= parse_cqm(value, (*p).cqm_8py.as_mut_ptr(), 64 as ::core::ffi::c_int);
        b_error |= parse_cqm(value, (*p).cqm_8ic.as_mut_ptr(), 64 as ::core::ffi::c_int);
        b_error |= parse_cqm(value, (*p).cqm_8pc.as_mut_ptr(), 64 as ::core::ffi::c_int);
    } else if strcmp(name, b"cqm4i\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4iy.as_mut_ptr(), 16 as ::core::ffi::c_int);
        b_error |= parse_cqm(value, (*p).cqm_4ic.as_mut_ptr(), 16 as ::core::ffi::c_int);
    } else if strcmp(name, b"cqm4p\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4py.as_mut_ptr(), 16 as ::core::ffi::c_int);
        b_error |= parse_cqm(value, (*p).cqm_4pc.as_mut_ptr(), 16 as ::core::ffi::c_int);
    } else if strcmp(name, b"cqm4iy\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4iy.as_mut_ptr(), 16 as ::core::ffi::c_int);
    } else if strcmp(name, b"cqm4ic\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4ic.as_mut_ptr(), 16 as ::core::ffi::c_int);
    } else if strcmp(name, b"cqm4py\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4py.as_mut_ptr(), 16 as ::core::ffi::c_int);
    } else if strcmp(name, b"cqm4pc\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_4pc.as_mut_ptr(), 16 as ::core::ffi::c_int);
    } else if strcmp(name, b"cqm8i\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_8iy.as_mut_ptr(), 64 as ::core::ffi::c_int);
        b_error |= parse_cqm(value, (*p).cqm_8ic.as_mut_ptr(), 64 as ::core::ffi::c_int);
    } else if strcmp(name, b"cqm8p\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_cqm_preset = X264_CQM_CUSTOM;
        b_error |= parse_cqm(value, (*p).cqm_8py.as_mut_ptr(), 64 as ::core::ffi::c_int);
        b_error |= parse_cqm(value, (*p).cqm_8pc.as_mut_ptr(), 64 as ::core::ffi::c_int);
    } else if strcmp(name, b"log\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_log_level = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"dump-yuv\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).psz_dump_yuv = x264_param_strdup(p, value);
        if (*p).psz_dump_yuv.is_null() {
            b_error = 1 as ::core::ffi::c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
    } else if strcmp(
        name,
        b"analyse\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            name,
            b"partitions\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*p).analyse.inter = 0 as ::core::ffi::c_uint;
        if !strstr(value, b"none\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            (*p).analyse.inter = 0 as ::core::ffi::c_uint;
        }
        if !strstr(value, b"all\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            (*p).analyse.inter = !(0 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        }
        if !strstr(value, b"i4x4\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            (*p).analyse.inter |= X264_ANALYSE_I4x4;
        }
        if !strstr(value, b"i8x8\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            (*p).analyse.inter |= X264_ANALYSE_I8x8;
        }
        if !strstr(value, b"p8x8\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            (*p).analyse.inter |= X264_ANALYSE_PSUB16x16;
        }
        if !strstr(value, b"p4x4\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            (*p).analyse.inter |= X264_ANALYSE_PSUB8x8;
        }
        if !strstr(value, b"b8x8\0" as *const u8 as *const ::core::ffi::c_char).is_null() {
            (*p).analyse.inter |= X264_ANALYSE_BSUB16x16;
        }
    } else if strcmp(name, b"8x8dct\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).analyse.b_transform_8x8 = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"weightb\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            name,
            b"weight-b\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).analyse.b_weighted_bipred = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"weightp\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).analyse.i_weighted_pred = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"direct\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcmp(
            name,
            b"direct-pred\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        b_error |= parse_enum(
            value,
            x264_direct_pred_names.as_ptr(),
            &mut (*p).analyse.i_direct_mv_pred,
        );
    } else if strcmp(
        name,
        b"chroma-qp-offset\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).analyse.i_chroma_qp_offset = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"me\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        b_error |= parse_enum(
            value,
            x264_motion_est_names.as_ptr(),
            &mut (*p).analyse.i_me_method,
        );
    } else if strcmp(
        name,
        b"merange\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            name,
            b"me-range\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*p).analyse.i_me_range = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"mvrange\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            name,
            b"mv-range\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*p).analyse.i_mv_range = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"mvrange-thread\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            name,
            b"mv-range-thread\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*p).analyse.i_mv_range_thread = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"subme\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcmp(name, b"subq\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        (*p).analyse.i_subpel_refine = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"psy-rd\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        if !(2 as ::core::ffi::c_int
            == sscanf(
                value,
                b"%f:%f\0" as *const u8 as *const ::core::ffi::c_char,
                &mut (*p).analyse.f_psy_rd as *mut ::core::ffi::c_float,
                &mut (*p).analyse.f_psy_trellis as *mut ::core::ffi::c_float,
            )
            || 2 as ::core::ffi::c_int
                == sscanf(
                    value,
                    b"%f,%f\0" as *const u8 as *const ::core::ffi::c_char,
                    &mut (*p).analyse.f_psy_rd as *mut ::core::ffi::c_float,
                    &mut (*p).analyse.f_psy_trellis as *mut ::core::ffi::c_float,
                )
            || 2 as ::core::ffi::c_int
                == sscanf(
                    value,
                    b"%f|%f\0" as *const u8 as *const ::core::ffi::c_char,
                    &mut (*p).analyse.f_psy_rd as *mut ::core::ffi::c_float,
                    &mut (*p).analyse.f_psy_trellis as *mut ::core::ffi::c_float,
                ))
        {
            if sscanf(
                value,
                b"%f\0" as *const u8 as *const ::core::ffi::c_char,
                &mut (*p).analyse.f_psy_rd as *mut ::core::ffi::c_float,
            ) != 0
            {
                (*p).analyse.f_psy_trellis = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
            } else {
                (*p).analyse.f_psy_rd = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
                (*p).analyse.f_psy_trellis = 0 as ::core::ffi::c_int as ::core::ffi::c_float;
            }
        }
    } else if strcmp(name, b"psy\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).analyse.b_psy = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"chroma-me\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).analyse.b_chroma_me = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"mixed-refs\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).analyse.b_mixed_references = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"trellis\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).analyse.i_trellis = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"fast-pskip\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).analyse.b_fast_pskip = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"dct-decimate\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).analyse.b_dct_decimate = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"deadzone-inter\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).analyse.i_luma_deadzone[0 as ::core::ffi::c_int as usize] =
            atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"deadzone-intra\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).analyse.i_luma_deadzone[1 as ::core::ffi::c_int as usize] =
            atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"nr\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).analyse.i_noise_reduction = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"bitrate\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).rc.i_bitrate = atoi_internal(value, &mut b_error);
        (*p).rc.i_rc_method = X264_RC_ABR;
    } else if strcmp(name, b"qp\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcmp(
            name,
            b"qp_constant\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*p).rc.i_qp_constant = atoi_internal(value, &mut b_error);
        (*p).rc.i_rc_method = X264_RC_CQP;
    } else if strcmp(name, b"crf\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).rc.f_rf_constant = atof_internal(value, &mut b_error) as ::core::ffi::c_float;
        (*p).rc.i_rc_method = X264_RC_CRF;
    } else if strcmp(
        name,
        b"crf-max\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).rc.f_rf_constant_max = atof_internal(value, &mut b_error) as ::core::ffi::c_float;
    } else if strcmp(
        name,
        b"rc-lookahead\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).rc.i_lookahead = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"qpmin\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcmp(name, b"qp-min\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        (*p).rc.i_qp_min = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"qpmax\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcmp(name, b"qp-max\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        (*p).rc.i_qp_max = atoi_internal(value, &mut b_error);
    } else if strcmp(name, b"qpstep\0" as *const u8 as *const ::core::ffi::c_char) == 0
        || strcmp(
            name,
            b"qp-step\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*p).rc.i_qp_step = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"ratetol\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).rc.f_rate_tolerance = (if strncmp(
            b"inf\0" as *const u8 as *const ::core::ffi::c_char,
            value,
            3 as size_t,
        ) == 0
        {
            1e9f64
        } else {
            atof_internal(value, &mut b_error)
        }) as ::core::ffi::c_float;
    } else if strcmp(
        name,
        b"vbv-maxrate\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).rc.i_vbv_max_bitrate = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"vbv-bufsize\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).rc.i_vbv_buffer_size = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"vbv-init\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).rc.f_vbv_buffer_init = atof_internal(value, &mut b_error) as ::core::ffi::c_float;
    } else if strcmp(
        name,
        b"ipratio\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            name,
            b"ip-factor\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*p).rc.f_ip_factor = atof_internal(value, &mut b_error) as ::core::ffi::c_float;
    } else if strcmp(
        name,
        b"pbratio\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            name,
            b"pb-factor\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*p).rc.f_pb_factor = atof_internal(value, &mut b_error) as ::core::ffi::c_float;
    } else if strcmp(
        name,
        b"aq-mode\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).rc.i_aq_mode = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"aq-strength\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).rc.f_aq_strength = atof_internal(value, &mut b_error) as ::core::ffi::c_float;
    } else if strcmp(name, b"pass\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        let mut pass: ::core::ffi::c_int = x264_clip3(
            atoi_internal(value, &mut b_error),
            0 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
        );
        (*p).rc.b_stat_write = pass & 1 as ::core::ffi::c_int;
        (*p).rc.b_stat_read = pass & 2 as ::core::ffi::c_int;
    } else if strcmp(name, b"stats\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).rc.psz_stat_in = x264_param_strdup(p, value);
        if (*p).rc.psz_stat_in.is_null() {
            b_error = 1 as ::core::ffi::c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
        (*p).rc.psz_stat_out = x264_param_strdup(p, value);
        if (*p).rc.psz_stat_out.is_null() {
            b_error = 1 as ::core::ffi::c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
    } else if strcmp(name, b"qcomp\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).rc.f_qcompress = atof_internal(value, &mut b_error) as ::core::ffi::c_float;
    } else if strcmp(name, b"mbtree\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).rc.b_mb_tree = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"qblur\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).rc.f_qblur = atof_internal(value, &mut b_error) as ::core::ffi::c_float;
    } else if strcmp(
        name,
        b"cplxblur\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
        || strcmp(
            name,
            b"cplx-blur\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
    {
        (*p).rc.f_complexity_blur = atof_internal(value, &mut b_error) as ::core::ffi::c_float;
    } else if strcmp(name, b"zones\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).rc.psz_zones = x264_param_strdup(p, value);
        if (*p).rc.psz_zones.is_null() {
            b_error = 1 as ::core::ffi::c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
    } else if strcmp(
        name,
        b"crop-rect\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        b_error |= (sscanf(
            value,
            b"%d,%d,%d,%d\0" as *const u8 as *const ::core::ffi::c_char,
            &mut (*p).crop_rect.i_left as *mut ::core::ffi::c_int,
            &mut (*p).crop_rect.i_top as *mut ::core::ffi::c_int,
            &mut (*p).crop_rect.i_right as *mut ::core::ffi::c_int,
            &mut (*p).crop_rect.i_bottom as *mut ::core::ffi::c_int,
        ) != 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
    } else if strcmp(name, b"psnr\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).analyse.b_psnr = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"ssim\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).analyse.b_ssim = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"aud\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_aud = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"sps-id\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        (*p).i_sps_id = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"global-header\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_repeat_headers = (atobool_internal(value, &mut b_error) == 0) as ::core::ffi::c_int;
    } else if strcmp(
        name,
        b"repeat-headers\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_repeat_headers = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"annexb\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_annexb = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"force-cfr\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_vfr_input = (atobool_internal(value, &mut b_error) == 0) as ::core::ffi::c_int;
    } else if strcmp(
        name,
        b"nal-hrd\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        b_error |= parse_enum(value, x264_nal_hrd_names.as_ptr(), &mut (*p).i_nal_hrd);
    } else if strcmp(name, b"filler\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).rc.b_filler = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"pic-struct\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_pic_struct = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"fake-interlaced\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_fake_interlaced = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"frame-packing\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).i_frame_packing = atoi_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"stitchable\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_stitchable = atobool_internal(value, &mut b_error);
    } else if strcmp(name, b"opencl\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        name_was_bool = 1 as ::core::ffi::c_int;
        (*p).b_opencl = atobool_internal(value, &mut b_error);
    } else if strcmp(
        name,
        b"opencl-clbin\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).psz_clbin_file = x264_param_strdup(p, value);
        if (*p).psz_clbin_file.is_null() {
            b_error = 1 as ::core::ffi::c_int;
            errortype = X264_PARAM_ALLOC_FAILED;
        }
    } else if strcmp(
        name,
        b"opencl-device\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        (*p).i_opencl_device = atoi_internal(value, &mut b_error);
    } else {
        b_error = 1 as ::core::ffi::c_int;
        errortype = X264_PARAM_BAD_NAME;
    }
    if !name_buf.is_null() {
        free(name_buf as *mut ::core::ffi::c_void);
    }
    b_error |= (value_was_null != 0 && name_was_bool == 0) as ::core::ffi::c_int;
    return if b_error != 0 {
        errortype
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
#[c2rust::src_loc = "1428:1"]
pub unsafe extern "C" fn x264_param2string(
    mut p: *mut x264_param_t,
    mut b_res: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut len: ::core::ffi::c_int = 2000 as ::core::ffi::c_int;
    let mut buf: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut s: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    if !(*p).rc.psz_zones.is_null() {
        len = (len as size_t).wrapping_add(strlen((*p).rc.psz_zones)) as ::core::ffi::c_int
            as ::core::ffi::c_int;
    }
    s = x264_malloc(len as int64_t) as *mut ::core::ffi::c_char;
    buf = s;
    if buf.is_null() {
        return 0 as *mut ::core::ffi::c_char;
    }
    if b_res != 0 {
        s = s.offset(sprintf(
            s,
            b"%dx%d \0" as *const u8 as *const ::core::ffi::c_char,
            (*p).i_width,
            (*p).i_height,
        ) as isize);
        s = s.offset(sprintf(
            s,
            b"fps=%u/%u \0" as *const u8 as *const ::core::ffi::c_char,
            (*p).i_fps_num,
            (*p).i_fps_den,
        ) as isize);
        s = s.offset(sprintf(
            s,
            b"timebase=%u/%u \0" as *const u8 as *const ::core::ffi::c_char,
            (*p).i_timebase_num,
            (*p).i_timebase_den,
        ) as isize);
        s = s.offset(sprintf(
            s,
            b"bitdepth=%d \0" as *const u8 as *const ::core::ffi::c_char,
            (*p).i_bitdepth,
        ) as isize);
    }
    if (*p).b_opencl != 0 {
        s = s.offset(sprintf(
            s,
            b"opencl=%d \0" as *const u8 as *const ::core::ffi::c_char,
            (*p).b_opencl,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b"cabac=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).b_cabac,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" ref=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).i_frame_reference,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" deblock=%d:%d:%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).b_deblocking_filter,
        (*p).i_deblocking_filter_alphac0,
        (*p).i_deblocking_filter_beta,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" analyse=%#x:%#x\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.intra,
        (*p).analyse.inter,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" me=%s\0" as *const u8 as *const ::core::ffi::c_char,
        x264_motion_est_names[(*p).analyse.i_me_method as usize],
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" subme=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.i_subpel_refine,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" psy=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.b_psy,
    ) as isize);
    if (*p).analyse.b_psy != 0 {
        s = s.offset(sprintf(
            s,
            b" psy_rd=%.2f:%.2f\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).analyse.f_psy_rd as ::core::ffi::c_double,
            (*p).analyse.f_psy_trellis as ::core::ffi::c_double,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" mixed_ref=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.b_mixed_references,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" me_range=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.i_me_range,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" chroma_me=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.b_chroma_me,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" trellis=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.i_trellis,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" 8x8dct=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.b_transform_8x8,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" cqm=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).i_cqm_preset,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" deadzone=%d,%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.i_luma_deadzone[0 as ::core::ffi::c_int as usize],
        (*p).analyse.i_luma_deadzone[1 as ::core::ffi::c_int as usize],
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" fast_pskip=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.b_fast_pskip,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" chroma_qp_offset=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.i_chroma_qp_offset,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" threads=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).i_threads,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" lookahead_threads=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).i_lookahead_threads,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" sliced_threads=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).b_sliced_threads,
    ) as isize);
    if (*p).i_slice_count != 0 {
        s = s.offset(sprintf(
            s,
            b" slices=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).i_slice_count,
        ) as isize);
    }
    if (*p).i_slice_count_max != 0 {
        s = s.offset(sprintf(
            s,
            b" slices_max=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).i_slice_count_max,
        ) as isize);
    }
    if (*p).i_slice_max_size != 0 {
        s = s.offset(sprintf(
            s,
            b" slice_max_size=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).i_slice_max_size,
        ) as isize);
    }
    if (*p).i_slice_max_mbs != 0 {
        s = s.offset(sprintf(
            s,
            b" slice_max_mbs=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).i_slice_max_mbs,
        ) as isize);
    }
    if (*p).i_slice_min_mbs != 0 {
        s = s.offset(sprintf(
            s,
            b" slice_min_mbs=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).i_slice_min_mbs,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" nr=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.i_noise_reduction,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" decimate=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).analyse.b_dct_decimate,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" interlaced=%s\0" as *const u8 as *const ::core::ffi::c_char,
        if (*p).b_interlaced != 0 {
            if (*p).b_tff != 0 {
                b"tff\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"bff\0" as *const u8 as *const ::core::ffi::c_char
            }
        } else if (*p).b_fake_interlaced != 0 {
            b"fake\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"0\0" as *const u8 as *const ::core::ffi::c_char
        },
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" bluray_compat=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).b_bluray_compat,
    ) as isize);
    if (*p).b_stitchable != 0 {
        s = s.offset(sprintf(
            s,
            b" stitchable=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).b_stitchable,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" constrained_intra=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).b_constrained_intra,
    ) as isize);
    s = s.offset(sprintf(
        s,
        b" bframes=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).i_bframe,
    ) as isize);
    if (*p).i_bframe != 0 {
        s = s.offset(sprintf(
            s,
            b" b_pyramid=%d b_adapt=%d b_bias=%d direct=%d weightb=%d open_gop=%d\0" as *const u8
                as *const ::core::ffi::c_char,
            (*p).i_bframe_pyramid,
            (*p).i_bframe_adaptive,
            (*p).i_bframe_bias,
            (*p).analyse.i_direct_mv_pred,
            (*p).analyse.b_weighted_bipred,
            (*p).b_open_gop,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" weightp=%d\0" as *const u8 as *const ::core::ffi::c_char,
        if (*p).analyse.i_weighted_pred > 0 as ::core::ffi::c_int {
            (*p).analyse.i_weighted_pred
        } else {
            0 as ::core::ffi::c_int
        },
    ) as isize);
    if (*p).i_keyint_max == X264_KEYINT_MAX_INFINITE {
        s = s.offset(sprintf(
            s,
            b" keyint=infinite\0" as *const u8 as *const ::core::ffi::c_char,
        ) as isize);
    } else {
        s = s.offset(sprintf(
            s,
            b" keyint=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).i_keyint_max,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" keyint_min=%d scenecut=%d intra_refresh=%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*p).i_keyint_min,
        (*p).i_scenecut_threshold,
        (*p).b_intra_refresh,
    ) as isize);
    if (*p).rc.b_mb_tree != 0 || (*p).rc.i_vbv_buffer_size != 0 {
        s = s.offset(sprintf(
            s,
            b" rc_lookahead=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).rc.i_lookahead,
        ) as isize);
    }
    s = s.offset(sprintf(
        s,
        b" rc=%s mbtree=%d\0" as *const u8 as *const ::core::ffi::c_char,
        if (*p).rc.i_rc_method == X264_RC_ABR {
            if (*p).rc.b_stat_read != 0 {
                b"2pass\0" as *const u8 as *const ::core::ffi::c_char
            } else if (*p).rc.i_vbv_max_bitrate == (*p).rc.i_bitrate {
                b"cbr\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"abr\0" as *const u8 as *const ::core::ffi::c_char
            }
        } else if (*p).rc.i_rc_method == X264_RC_CRF {
            b"crf\0" as *const u8 as *const ::core::ffi::c_char
        } else {
            b"cqp\0" as *const u8 as *const ::core::ffi::c_char
        },
        (*p).rc.b_mb_tree,
    ) as isize);
    if (*p).rc.i_rc_method == X264_RC_ABR || (*p).rc.i_rc_method == X264_RC_CRF {
        if (*p).rc.i_rc_method == X264_RC_CRF {
            s = s.offset(sprintf(
                s,
                b" crf=%.1f\0" as *const u8 as *const ::core::ffi::c_char,
                (*p).rc.f_rf_constant as ::core::ffi::c_double,
            ) as isize);
        } else {
            s = s.offset(sprintf(
                s,
                b" bitrate=%d ratetol=%.1f\0" as *const u8 as *const ::core::ffi::c_char,
                (*p).rc.i_bitrate,
                (*p).rc.f_rate_tolerance as ::core::ffi::c_double,
            ) as isize);
        }
        s = s.offset(sprintf(
            s,
            b" qcomp=%.2f qpmin=%d qpmax=%d qpstep=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).rc.f_qcompress as ::core::ffi::c_double,
            (*p).rc.i_qp_min,
            (*p).rc.i_qp_max,
            (*p).rc.i_qp_step,
        ) as isize);
        if (*p).rc.b_stat_read != 0 {
            s = s.offset(sprintf(
                s,
                b" cplxblur=%.1f qblur=%.1f\0" as *const u8 as *const ::core::ffi::c_char,
                (*p).rc.f_complexity_blur as ::core::ffi::c_double,
                (*p).rc.f_qblur as ::core::ffi::c_double,
            ) as isize);
        }
        if (*p).rc.i_vbv_buffer_size != 0 {
            s = s.offset(sprintf(
                s,
                b" vbv_maxrate=%d vbv_bufsize=%d\0" as *const u8 as *const ::core::ffi::c_char,
                (*p).rc.i_vbv_max_bitrate,
                (*p).rc.i_vbv_buffer_size,
            ) as isize);
            if (*p).rc.i_rc_method == X264_RC_CRF {
                s = s.offset(sprintf(
                    s,
                    b" crf_max=%.1f\0" as *const u8 as *const ::core::ffi::c_char,
                    (*p).rc.f_rf_constant_max as ::core::ffi::c_double,
                ) as isize);
            }
        }
    } else if (*p).rc.i_rc_method == X264_RC_CQP {
        s = s.offset(sprintf(
            s,
            b" qp=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).rc.i_qp_constant,
        ) as isize);
    }
    if (*p).rc.i_vbv_buffer_size != 0 {
        s = s.offset(sprintf(
            s,
            b" nal_hrd=%s filler=%d\0" as *const u8 as *const ::core::ffi::c_char,
            x264_nal_hrd_names[(*p).i_nal_hrd as usize],
            (*p).rc.b_filler,
        ) as isize);
    }
    if (*p).crop_rect.i_left
        | (*p).crop_rect.i_top
        | (*p).crop_rect.i_right
        | (*p).crop_rect.i_bottom
        != 0
    {
        s = s.offset(sprintf(
            s,
            b" crop_rect=%d,%d,%d,%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).crop_rect.i_left,
            (*p).crop_rect.i_top,
            (*p).crop_rect.i_right,
            (*p).crop_rect.i_bottom,
        ) as isize);
    }
    if (*p).mastering_display.b_mastering_display != 0 {
        s = s.offset(sprintf(
            s,
            b" mastering-display=G(%d,%d)B(%d,%d)R(%d,%d)WP(%d,%d)L(%ld,%ld)\0" as *const u8
                as *const ::core::ffi::c_char,
            (*p).mastering_display.i_green_x,
            (*p).mastering_display.i_green_y,
            (*p).mastering_display.i_blue_x,
            (*p).mastering_display.i_blue_y,
            (*p).mastering_display.i_red_x,
            (*p).mastering_display.i_red_y,
            (*p).mastering_display.i_white_x,
            (*p).mastering_display.i_white_y,
            (*p).mastering_display.i_display_max,
            (*p).mastering_display.i_display_min,
        ) as isize);
    }
    if (*p).content_light_level.b_cll != 0 {
        s = s.offset(sprintf(
            s,
            b" cll=%d,%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).content_light_level.i_max_cll,
            (*p).content_light_level.i_max_fall,
        ) as isize);
    }
    if (*p).i_frame_packing >= 0 as ::core::ffi::c_int {
        s = s.offset(sprintf(
            s,
            b" frame-packing=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).i_frame_packing,
        ) as isize);
    }
    if !((*p).rc.i_rc_method == X264_RC_CQP && (*p).rc.i_qp_constant == 0 as ::core::ffi::c_int) {
        s = s.offset(sprintf(
            s,
            b" ip_ratio=%.2f\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).rc.f_ip_factor as ::core::ffi::c_double,
        ) as isize);
        if (*p).i_bframe != 0 && (*p).rc.b_mb_tree == 0 {
            s = s.offset(sprintf(
                s,
                b" pb_ratio=%.2f\0" as *const u8 as *const ::core::ffi::c_char,
                (*p).rc.f_pb_factor as ::core::ffi::c_double,
            ) as isize);
        }
        s = s.offset(sprintf(
            s,
            b" aq=%d\0" as *const u8 as *const ::core::ffi::c_char,
            (*p).rc.i_aq_mode,
        ) as isize);
        if (*p).rc.i_aq_mode != 0 {
            s = s.offset(sprintf(
                s,
                b":%.2f\0" as *const u8 as *const ::core::ffi::c_char,
                (*p).rc.f_aq_strength as ::core::ffi::c_double,
            ) as isize);
        }
        if !(*p).rc.psz_zones.is_null() {
            s = s.offset(sprintf(
                s,
                b" zones=%s\0" as *const u8 as *const ::core::ffi::c_char,
                (*p).rc.psz_zones,
            ) as isize);
        } else if (*p).rc.i_zones != 0 {
            s = s.offset(
                sprintf(s, b" zones\0" as *const u8 as *const ::core::ffi::c_char) as isize,
            );
        }
    }
    return buf;
}
