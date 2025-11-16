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
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = u32;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = i64;
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
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint8_t, __uint32_t};
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
    #[c2rust::src_loc = "282:9"]
    pub const X264_TYPE_B: ::core::ffi::c_int = 0x5 as ::core::ffi::c_int;
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/output/matroska_ebml.h:26"]
pub mod matroska_ebml_h {
    #[c2rust::src_loc = "30:9"]
    pub const DS_PIXELS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::stdint_intn_h::int64_t;
    extern "C" {
        #[c2rust::src_loc = "35:16"]
        pub type mk_writer;
        #[c2rust::src_loc = "37:1"]
        pub fn mk_create_writer(filename: *const ::core::ffi::c_char) -> *mut mk_writer;
        #[c2rust::src_loc = "39:1"]
        pub fn mk_write_header(
            w: *mut mk_writer,
            writing_app: *const ::core::ffi::c_char,
            codec_id: *const ::core::ffi::c_char,
            codec_private: *const ::core::ffi::c_void,
            codec_private_size: ::core::ffi::c_uint,
            default_frame_duration: int64_t,
            timescale: int64_t,
            width: ::core::ffi::c_uint,
            height: ::core::ffi::c_uint,
            d_width: ::core::ffi::c_uint,
            d_height: ::core::ffi::c_uint,
            display_size_units: ::core::ffi::c_int,
            stereo_mode: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "47:1"]
        pub fn mk_start_frame(w: *mut mk_writer) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "48:1"]
        pub fn mk_add_frame_data(
            w: *mut mk_writer,
            data: *const ::core::ffi::c_void,
            size: ::core::ffi::c_uint,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "49:1"]
        pub fn mk_set_frame_flags(
            w: *mut mk_writer,
            timestamp: int64_t,
            keyframe: ::core::ffi::c_int,
            skippable: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "50:1"]
        pub fn mk_close(w: *mut mk_writer, last_delta: int64_t) -> ::core::ffi::c_int;
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
pub use self::internal::__va_list_tag;
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{__uint8_t, __uint32_t, __int64_t};
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t};
pub use self::x264_h::{
    x264_nal_t, x264_zone_t, x264_param_t, C2RustUnnamed, C2RustUnnamed_0,
    C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4, x264_hrd_t,
    x264_sei_payload_t, x264_sei_t, x264_image_t, x264_image_properties_t,
    x264_picture_t, X264_TYPE_B, x264_t,
};
pub use self::x264cli_h::hnd_t;
pub use self::output_h::{cli_output_opt_t, cli_output_t};
pub use self::matroska_ebml_h::{
    DS_PIXELS, mk_writer, mk_create_writer, mk_write_header, mk_start_frame,
    mk_add_frame_data, mk_set_frame_flags, mk_close,
};
use self::stdlib_h::{malloc, calloc, free};
use self::string_h::memcpy;
pub use self::__stddef_null_h::NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "29:9"]
pub struct mkv_hnd_t {
    pub w: *mut mk_writer,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub d_width: ::core::ffi::c_int,
    pub d_height: ::core::ffi::c_int,
    pub display_size_units: ::core::ffi::c_int,
    pub stereo_mode: ::core::ffi::c_int,
    pub frame_duration: int64_t,
    pub b_writing_frame: ::core::ffi::c_char,
    pub i_timebase_num: uint32_t,
    pub i_timebase_den: uint32_t,
}
#[c2rust::src_loc = "46:1"]
unsafe extern "C" fn open_file(
    mut psz_filename: *mut ::core::ffi::c_char,
    mut p_handle: *mut hnd_t,
    mut opt: *mut cli_output_opt_t,
) -> ::core::ffi::c_int {
    *p_handle = NULL as hnd_t;
    let mut p_mkv: *mut mkv_hnd_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<mkv_hnd_t>() as size_t,
    ) as *mut mkv_hnd_t;
    if p_mkv.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*p_mkv).w = mk_create_writer(psz_filename);
    if (*p_mkv).w.is_null() {
        free(p_mkv as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    }
    *p_handle = p_mkv as hnd_t;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "65:9"]
pub const STEREO_COUNT: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
#[c2rust::src_loc = "66:22"]
static mut stereo_modes: [uint8_t; 7] = [
    5 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
#[c2rust::src_loc = "67:22"]
static mut stereo_w_div: [uint8_t; 7] = [
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
];
#[c2rust::src_loc = "68:22"]
static mut stereo_h_div: [uint8_t; 7] = [
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
];
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn set_param(
    mut handle: hnd_t,
    mut p_param: *mut x264_param_t,
) -> ::core::ffi::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut dw: int64_t = 0;
    let mut dh: int64_t = 0;
    if (*p_param).i_fps_num > 0 as uint32_t && (*p_param).b_vfr_input == 0 {
        (*p_mkv).frame_duration = (*p_param).i_fps_den as int64_t
            * 1000000000 as ::core::ffi::c_int as int64_t
            / (*p_param).i_fps_num as int64_t;
    } else {
        (*p_mkv).frame_duration = 0 as int64_t;
    }
    (*p_mkv).width = (*p_param).i_width;
    dw = (*p_mkv).width as int64_t;
    (*p_mkv).height = (*p_param).i_height;
    dh = (*p_mkv).height as int64_t;
    (*p_mkv).display_size_units = DS_PIXELS;
    (*p_mkv).stereo_mode = -(1 as ::core::ffi::c_int);
    if (*p_param).i_frame_packing >= 0 as ::core::ffi::c_int
        && (*p_param).i_frame_packing < STEREO_COUNT
    {
        (*p_mkv).stereo_mode = stereo_modes[(*p_param).i_frame_packing as usize]
            as ::core::ffi::c_int;
        dw /= stereo_w_div[(*p_param).i_frame_packing as usize] as int64_t;
        dh /= stereo_h_div[(*p_param).i_frame_packing as usize] as int64_t;
    }
    if (*p_param).vui.i_sar_width != 0 && (*p_param).vui.i_sar_height != 0
        && (*p_param).vui.i_sar_width != (*p_param).vui.i_sar_height
    {
        if (*p_param).vui.i_sar_width > (*p_param).vui.i_sar_height {
            dw = dw * (*p_param).vui.i_sar_width as int64_t
                / (*p_param).vui.i_sar_height as int64_t;
        } else {
            dh = dh * (*p_param).vui.i_sar_height as int64_t
                / (*p_param).vui.i_sar_width as int64_t;
        }
    }
    (*p_mkv).d_width = dw as ::core::ffi::c_int;
    (*p_mkv).d_height = dh as ::core::ffi::c_int;
    (*p_mkv).i_timebase_num = (*p_param).i_timebase_num;
    (*p_mkv).i_timebase_den = (*p_param).i_timebase_den;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "116:1"]
unsafe extern "C" fn write_headers(
    mut handle: hnd_t,
    mut p_nal: *mut x264_nal_t,
) -> ::core::ffi::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut sps_size: ::core::ffi::c_int = (*p_nal
        .offset(0 as ::core::ffi::c_int as isize))
        .i_payload - 4 as ::core::ffi::c_int;
    let mut pps_size: ::core::ffi::c_int = (*p_nal
        .offset(1 as ::core::ffi::c_int as isize))
        .i_payload - 4 as ::core::ffi::c_int;
    let mut sei_size: ::core::ffi::c_int = (*p_nal
        .offset(2 as ::core::ffi::c_int as isize))
        .i_payload;
    let mut sps: *mut uint8_t = (*p_nal.offset(0 as ::core::ffi::c_int as isize))
        .p_payload
        .offset(4 as ::core::ffi::c_int as isize);
    let mut pps: *mut uint8_t = (*p_nal.offset(1 as ::core::ffi::c_int as isize))
        .p_payload
        .offset(4 as ::core::ffi::c_int as isize);
    let mut sei: *mut uint8_t = (*p_nal.offset(2 as ::core::ffi::c_int as isize))
        .p_payload;
    let mut ret: ::core::ffi::c_int = 0;
    let mut avcC: *mut uint8_t = 0 as *mut uint8_t;
    let mut avcC_len: ::core::ffi::c_int = 0;
    if (*p_mkv).width == 0 || (*p_mkv).height == 0 || (*p_mkv).d_width == 0
        || (*p_mkv).d_height == 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    avcC_len = 5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int + sps_size + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int + pps_size;
    avcC = malloc(avcC_len as size_t) as *mut uint8_t;
    if avcC.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    *avcC.offset(0 as ::core::ffi::c_int as isize) = 1 as uint8_t;
    *avcC.offset(1 as ::core::ffi::c_int as isize) = *sps
        .offset(1 as ::core::ffi::c_int as isize);
    *avcC.offset(2 as ::core::ffi::c_int as isize) = *sps
        .offset(2 as ::core::ffi::c_int as isize);
    *avcC.offset(3 as ::core::ffi::c_int as isize) = *sps
        .offset(3 as ::core::ffi::c_int as isize);
    *avcC.offset(4 as ::core::ffi::c_int as isize) = 0xff as uint8_t;
    *avcC.offset(5 as ::core::ffi::c_int as isize) = 0xe1 as uint8_t;
    *avcC.offset(6 as ::core::ffi::c_int as isize) = (sps_size
        >> 8 as ::core::ffi::c_int) as uint8_t;
    *avcC.offset(7 as ::core::ffi::c_int as isize) = sps_size as uint8_t;
    memcpy(
        avcC.offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
        sps as *const ::core::ffi::c_void,
        sps_size as size_t,
    );
    *avcC.offset((8 as ::core::ffi::c_int + sps_size) as isize) = 1 as uint8_t;
    *avcC.offset((9 as ::core::ffi::c_int + sps_size) as isize) = (pps_size
        >> 8 as ::core::ffi::c_int) as uint8_t;
    *avcC.offset((10 as ::core::ffi::c_int + sps_size) as isize) = pps_size as uint8_t;
    memcpy(
        avcC.offset(11 as ::core::ffi::c_int as isize).offset(sps_size as isize)
            as *mut ::core::ffi::c_void,
        pps as *const ::core::ffi::c_void,
        pps_size as size_t,
    );
    ret = mk_write_header(
        (*p_mkv).w,
        b"x264 r3223M 0480cb0\0" as *const u8 as *const ::core::ffi::c_char,
        b"V_MPEG4/ISO/AVC\0" as *const u8 as *const ::core::ffi::c_char,
        avcC as *const ::core::ffi::c_void,
        avcC_len as ::core::ffi::c_uint,
        (*p_mkv).frame_duration,
        50000 as int64_t,
        (*p_mkv).width as ::core::ffi::c_uint,
        (*p_mkv).height as ::core::ffi::c_uint,
        (*p_mkv).d_width as ::core::ffi::c_uint,
        (*p_mkv).d_height as ::core::ffi::c_uint,
        (*p_mkv).display_size_units,
        (*p_mkv).stereo_mode,
    );
    free(avcC as *mut ::core::ffi::c_void);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*p_mkv).b_writing_frame == 0 {
        if mk_start_frame((*p_mkv).w) < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        (*p_mkv).b_writing_frame = 1 as ::core::ffi::c_char;
    }
    if mk_add_frame_data(
        (*p_mkv).w,
        sei as *const ::core::ffi::c_void,
        sei_size as ::core::ffi::c_uint,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    return sei_size + sps_size + pps_size;
}
#[c2rust::src_loc = "182:1"]
unsafe extern "C" fn write_frame(
    mut handle: hnd_t,
    mut p_nalu: *mut uint8_t,
    mut i_size: ::core::ffi::c_int,
    mut p_picture: *mut x264_picture_t,
) -> ::core::ffi::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    if (*p_mkv).b_writing_frame == 0 {
        if mk_start_frame((*p_mkv).w) < 0 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
        (*p_mkv).b_writing_frame = 1 as ::core::ffi::c_char;
    }
    if mk_add_frame_data(
        (*p_mkv).w,
        p_nalu as *const ::core::ffi::c_void,
        i_size as ::core::ffi::c_uint,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    let mut i_stamp: int64_t = ((*p_picture).i_pts as ::core::ffi::c_double * 1e9f64
        * (*p_mkv).i_timebase_num as ::core::ffi::c_double
        / (*p_mkv).i_timebase_den as ::core::ffi::c_double + 0.5f64) as int64_t;
    (*p_mkv).b_writing_frame = 0 as ::core::ffi::c_char;
    if mk_set_frame_flags(
        (*p_mkv).w,
        i_stamp,
        (*p_picture).b_keyframe,
        ((*p_picture).i_type == X264_TYPE_B) as ::core::ffi::c_int,
    ) < 0 as ::core::ffi::c_int
    {
        return -(1 as ::core::ffi::c_int);
    }
    return i_size;
}
#[c2rust::src_loc = "206:1"]
unsafe extern "C" fn close_file(
    mut handle: hnd_t,
    mut largest_pts: int64_t,
    mut second_largest_pts: int64_t,
) -> ::core::ffi::c_int {
    let mut p_mkv: *mut mkv_hnd_t = handle as *mut mkv_hnd_t;
    let mut ret: ::core::ffi::c_int = 0;
    let mut i_last_delta: int64_t = 0;
    i_last_delta = if (*p_mkv).i_timebase_den != 0 {
        (((largest_pts - second_largest_pts) * (*p_mkv).i_timebase_num as int64_t
            / (*p_mkv).i_timebase_den as int64_t) as ::core::ffi::c_double + 0.5f64)
            as int64_t
    } else {
        0 as int64_t
    };
    ret = mk_close((*p_mkv).w, i_last_delta);
    free(p_mkv as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
#[c2rust::src_loc = "221:20"]
pub static mut mkv_output: cli_output_t = unsafe {
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
