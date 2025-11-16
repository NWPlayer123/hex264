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
    use super::types_h::{__uint32_t, __uint8_t};
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
    #[c2rust::src_loc = "788:16"]
    pub struct x264_sei_payload_t {
        pub payload_size: ::core::ffi::c_int,
        pub payload_type: ::core::ffi::c_int,
        pub payload: *mut uint8_t,
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
    #[c2rust::src_loc = "803:16"]
    pub struct x264_image_t {
        pub i_csp: ::core::ffi::c_int,
        pub i_plane: ::core::ffi::c_int,
        pub i_stride: [::core::ffi::c_int; 4],
        pub plane: [*mut uint8_t; 4],
    }
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::internal::__va_list_tag;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "80:16"]
        pub type x264_t;
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:27"]
pub mod base_h {
    extern "C" {
        #[c2rust::src_loc = "275:10"]
        pub fn x264_log_internal(
            i_level: ::core::ffi::c_int,
            psz_fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264_config.h:27"]
pub mod x264_config_h {
    #[c2rust::src_loc = "4:9"]
    pub const X264_CHROMA_FORMAT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:27"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::base_h::x264_log_internal;
pub use self::internal::__va_list_tag;
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::stdlib_h::{calloc, free};
pub use self::types_h::{__int64_t, __uint32_t, __uint8_t};
pub use self::x264_config_h::X264_CHROMA_FORMAT;
pub use self::x264_h::{
    x264_hrd_t, x264_image_properties_t, x264_image_t, x264_nal_t, x264_param_t, x264_picture_t,
    x264_sei_payload_t, x264_sei_t, x264_t, x264_zone_t, C2RustUnnamed, C2RustUnnamed_0,
    C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4, X264_LOG_ERROR,
};
extern "C" {
    #[c2rust::src_loc = "34:1"]
    pub fn x264_8_encoder_open(_: *mut x264_param_t, _: *mut ::core::ffi::c_void) -> *mut x264_t;
    #[c2rust::src_loc = "35:1"]
    pub fn x264_8_nal_encode(h: *mut x264_t, dst: *mut uint8_t, nal: *mut x264_nal_t);
    #[c2rust::src_loc = "36:1"]
    pub fn x264_8_encoder_reconfig(_: *mut x264_t, _: *mut x264_param_t) -> ::core::ffi::c_int;
    #[c2rust::src_loc = "37:1"]
    pub fn x264_8_encoder_parameters(_: *mut x264_t, _: *mut x264_param_t);
    #[c2rust::src_loc = "38:1"]
    pub fn x264_8_encoder_headers(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[c2rust::src_loc = "39:1"]
    pub fn x264_8_encoder_encode(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut ::core::ffi::c_int,
        pic_in: *mut x264_picture_t,
        pic_out: *mut x264_picture_t,
    ) -> ::core::ffi::c_int;
    #[c2rust::src_loc = "40:1"]
    pub fn x264_8_encoder_close(_: *mut x264_t);
    #[c2rust::src_loc = "41:1"]
    pub fn x264_8_encoder_delayed_frames(_: *mut x264_t) -> ::core::ffi::c_int;
    #[c2rust::src_loc = "42:1"]
    pub fn x264_8_encoder_maximum_delayed_frames(_: *mut x264_t) -> ::core::ffi::c_int;
    #[c2rust::src_loc = "43:1"]
    pub fn x264_8_encoder_intra_refresh(_: *mut x264_t);
    #[c2rust::src_loc = "44:1"]
    pub fn x264_8_encoder_invalidate_reference(_: *mut x264_t, pts: int64_t) -> ::core::ffi::c_int;
    #[c2rust::src_loc = "46:1"]
    pub fn x264_10_encoder_open(_: *mut x264_param_t, _: *mut ::core::ffi::c_void) -> *mut x264_t;
    #[c2rust::src_loc = "47:1"]
    pub fn x264_10_nal_encode(h: *mut x264_t, dst: *mut uint8_t, nal: *mut x264_nal_t);
    #[c2rust::src_loc = "48:1"]
    pub fn x264_10_encoder_reconfig(_: *mut x264_t, _: *mut x264_param_t) -> ::core::ffi::c_int;
    #[c2rust::src_loc = "49:1"]
    pub fn x264_10_encoder_parameters(_: *mut x264_t, _: *mut x264_param_t);
    #[c2rust::src_loc = "50:1"]
    pub fn x264_10_encoder_headers(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[c2rust::src_loc = "51:1"]
    pub fn x264_10_encoder_encode(
        _: *mut x264_t,
        pp_nal: *mut *mut x264_nal_t,
        pi_nal: *mut ::core::ffi::c_int,
        pic_in: *mut x264_picture_t,
        pic_out: *mut x264_picture_t,
    ) -> ::core::ffi::c_int;
    #[c2rust::src_loc = "52:1"]
    pub fn x264_10_encoder_close(_: *mut x264_t);
    #[c2rust::src_loc = "53:1"]
    pub fn x264_10_encoder_delayed_frames(_: *mut x264_t) -> ::core::ffi::c_int;
    #[c2rust::src_loc = "54:1"]
    pub fn x264_10_encoder_maximum_delayed_frames(_: *mut x264_t) -> ::core::ffi::c_int;
    #[c2rust::src_loc = "55:1"]
    pub fn x264_10_encoder_intra_refresh(_: *mut x264_t);
    #[c2rust::src_loc = "56:1"]
    pub fn x264_10_encoder_invalidate_reference(_: *mut x264_t, pts: int64_t)
        -> ::core::ffi::c_int;
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "58:16"]
pub struct x264_api_t {
    pub x264: *mut x264_t,
    pub nal_encode: Option<unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> ()>,
    pub encoder_reconfig:
        Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> ::core::ffi::c_int>,
    pub encoder_parameters: Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> ()>,
    pub encoder_headers: Option<
        unsafe extern "C" fn(
            *mut x264_t,
            *mut *mut x264_nal_t,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub encoder_encode: Option<
        unsafe extern "C" fn(
            *mut x264_t,
            *mut *mut x264_nal_t,
            *mut ::core::ffi::c_int,
            *mut x264_picture_t,
            *mut x264_picture_t,
        ) -> ::core::ffi::c_int,
    >,
    pub encoder_close: Option<unsafe extern "C" fn(*mut x264_t) -> ()>,
    pub encoder_delayed_frames: Option<unsafe extern "C" fn(*mut x264_t) -> ::core::ffi::c_int>,
    pub encoder_maximum_delayed_frames:
        Option<unsafe extern "C" fn(*mut x264_t) -> ::core::ffi::c_int>,
    pub encoder_intra_refresh: Option<unsafe extern "C" fn(*mut x264_t) -> ()>,
    pub encoder_invalidate_reference:
        Option<unsafe extern "C" fn(*mut x264_t, int64_t) -> ::core::ffi::c_int>,
}
#[no_mangle]
#[c2rust::src_loc = "32:11"]
pub static mut x264_chroma_format: ::core::ffi::c_int = X264_CHROMA_FORMAT;
#[no_mangle]
#[c2rust::src_loc = "76:1"]
pub unsafe extern "C" fn x264_encoder_open_165(mut param: *mut x264_param_t) -> *mut x264_t {
    let mut api: *mut x264_api_t =
        calloc(1 as size_t, ::core::mem::size_of::<x264_api_t>() as size_t) as *mut x264_api_t;
    if api.is_null() {
        return 0 as *mut x264_t;
    }
    if (*param).i_bitdepth == 8 as ::core::ffi::c_int {
        (*api).nal_encode = Some(
            x264_8_nal_encode
                as unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> (),
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> ()>;
        (*api).encoder_reconfig = Some(
            x264_8_encoder_reconfig
                as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> ::core::ffi::c_int>;
        (*api).encoder_parameters = Some(
            x264_8_encoder_parameters as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> (),
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> ()>;
        (*api).encoder_headers = Some(
            x264_8_encoder_headers
                as unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*api).encoder_encode = Some(
            x264_8_encoder_encode
                as unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut ::core::ffi::c_int,
                    *mut x264_picture_t,
                    *mut x264_picture_t,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut ::core::ffi::c_int,
                    *mut x264_picture_t,
                    *mut x264_picture_t,
                ) -> ::core::ffi::c_int,
            >;
        (*api).encoder_close = Some(x264_8_encoder_close as unsafe extern "C" fn(*mut x264_t) -> ())
            as Option<unsafe extern "C" fn(*mut x264_t) -> ()>;
        (*api).encoder_delayed_frames = Some(
            x264_8_encoder_delayed_frames
                as unsafe extern "C" fn(*mut x264_t) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t) -> ::core::ffi::c_int>;
        (*api).encoder_maximum_delayed_frames = Some(
            x264_8_encoder_maximum_delayed_frames
                as unsafe extern "C" fn(*mut x264_t) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t) -> ::core::ffi::c_int>;
        (*api).encoder_intra_refresh =
            Some(x264_8_encoder_intra_refresh as unsafe extern "C" fn(*mut x264_t) -> ())
                as Option<unsafe extern "C" fn(*mut x264_t) -> ()>;
        (*api).encoder_invalidate_reference = Some(
            x264_8_encoder_invalidate_reference
                as unsafe extern "C" fn(*mut x264_t, int64_t) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t, int64_t) -> ::core::ffi::c_int>;
        (*api).x264 = x264_8_encoder_open(param, api as *mut ::core::ffi::c_void);
    } else if (*param).i_bitdepth == 10 as ::core::ffi::c_int {
        (*api).nal_encode = Some(
            x264_10_nal_encode
                as unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> (),
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut uint8_t, *mut x264_nal_t) -> ()>;
        (*api).encoder_reconfig = Some(
            x264_10_encoder_reconfig
                as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> ::core::ffi::c_int>;
        (*api).encoder_parameters = Some(
            x264_10_encoder_parameters
                as unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> (),
        )
            as Option<unsafe extern "C" fn(*mut x264_t, *mut x264_param_t) -> ()>;
        (*api).encoder_headers = Some(
            x264_10_encoder_headers
                as unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
        (*api).encoder_encode = Some(
            x264_10_encoder_encode
                as unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut ::core::ffi::c_int,
                    *mut x264_picture_t,
                    *mut x264_picture_t,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut x264_t,
                    *mut *mut x264_nal_t,
                    *mut ::core::ffi::c_int,
                    *mut x264_picture_t,
                    *mut x264_picture_t,
                ) -> ::core::ffi::c_int,
            >;
        (*api).encoder_close =
            Some(x264_10_encoder_close as unsafe extern "C" fn(*mut x264_t) -> ())
                as Option<unsafe extern "C" fn(*mut x264_t) -> ()>;
        (*api).encoder_delayed_frames = Some(
            x264_10_encoder_delayed_frames
                as unsafe extern "C" fn(*mut x264_t) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t) -> ::core::ffi::c_int>;
        (*api).encoder_maximum_delayed_frames = Some(
            x264_10_encoder_maximum_delayed_frames
                as unsafe extern "C" fn(*mut x264_t) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t) -> ::core::ffi::c_int>;
        (*api).encoder_intra_refresh =
            Some(x264_10_encoder_intra_refresh as unsafe extern "C" fn(*mut x264_t) -> ())
                as Option<unsafe extern "C" fn(*mut x264_t) -> ()>;
        (*api).encoder_invalidate_reference = Some(
            x264_10_encoder_invalidate_reference
                as unsafe extern "C" fn(*mut x264_t, int64_t) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut x264_t, int64_t) -> ::core::ffi::c_int>;
        (*api).x264 = x264_10_encoder_open(param, api as *mut ::core::ffi::c_void);
    } else {
        x264_log_internal(
            X264_LOG_ERROR,
            b"not compiled with %d bit depth support\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*param).i_bitdepth,
        );
    }
    if (*api).x264.is_null() {
        free(api as *mut ::core::ffi::c_void);
        return 0 as *mut x264_t;
    }
    return api as *mut x264_t;
}
#[no_mangle]
#[c2rust::src_loc = "130:1"]
pub unsafe extern "C" fn x264_encoder_close(mut h: *mut x264_t) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    (*api).encoder_close.expect("non-null function pointer")((*api).x264);
    free(api as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "138:1"]
pub unsafe extern "C" fn x264_nal_encode(
    mut h: *mut x264_t,
    mut dst: *mut uint8_t,
    mut nal: *mut x264_nal_t,
) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    (*api).nal_encode.expect("non-null function pointer")((*api).x264, dst, nal);
}
#[no_mangle]
#[c2rust::src_loc = "145:1"]
pub unsafe extern "C" fn x264_encoder_reconfig(
    mut h: *mut x264_t,
    mut param: *mut x264_param_t,
) -> ::core::ffi::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api).encoder_reconfig.expect("non-null function pointer")((*api).x264, param);
}
#[no_mangle]
#[c2rust::src_loc = "152:1"]
pub unsafe extern "C" fn x264_encoder_parameters(mut h: *mut x264_t, mut param: *mut x264_param_t) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    (*api)
        .encoder_parameters
        .expect("non-null function pointer")((*api).x264, param);
}
#[no_mangle]
#[c2rust::src_loc = "159:1"]
pub unsafe extern "C" fn x264_encoder_headers(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api).encoder_headers.expect("non-null function pointer")((*api).x264, pp_nal, pi_nal);
}
#[no_mangle]
#[c2rust::src_loc = "166:1"]
pub unsafe extern "C" fn x264_encoder_encode(
    mut h: *mut x264_t,
    mut pp_nal: *mut *mut x264_nal_t,
    mut pi_nal: *mut ::core::ffi::c_int,
    mut pic_in: *mut x264_picture_t,
    mut pic_out: *mut x264_picture_t,
) -> ::core::ffi::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api).encoder_encode.expect("non-null function pointer")(
        (*api).x264,
        pp_nal,
        pi_nal,
        pic_in,
        pic_out,
    );
}
#[no_mangle]
#[c2rust::src_loc = "173:1"]
pub unsafe extern "C" fn x264_encoder_delayed_frames(mut h: *mut x264_t) -> ::core::ffi::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api)
        .encoder_delayed_frames
        .expect("non-null function pointer")((*api).x264);
}
#[no_mangle]
#[c2rust::src_loc = "180:1"]
pub unsafe extern "C" fn x264_encoder_maximum_delayed_frames(
    mut h: *mut x264_t,
) -> ::core::ffi::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api)
        .encoder_maximum_delayed_frames
        .expect("non-null function pointer")((*api).x264);
}
#[no_mangle]
#[c2rust::src_loc = "187:1"]
pub unsafe extern "C" fn x264_encoder_intra_refresh(mut h: *mut x264_t) {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    (*api)
        .encoder_intra_refresh
        .expect("non-null function pointer")((*api).x264);
}
#[no_mangle]
#[c2rust::src_loc = "194:1"]
pub unsafe extern "C" fn x264_encoder_invalidate_reference(
    mut h: *mut x264_t,
    mut pts: int64_t,
) -> ::core::ffi::c_int {
    let mut api: *mut x264_api_t = h as *mut x264_api_t;
    return (*api)
        .encoder_invalidate_reference
        .expect("non-null function pointer")((*api).x264, pts);
}
