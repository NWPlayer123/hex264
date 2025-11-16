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
    #[c2rust::src_loc = "61:9"]
    pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
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
    #[c2rust::src_loc = "45:1"]
    pub type __uint64_t = u64;
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
    #[c2rust::src_loc = "27:1"]
    pub type uint64_t = __uint64_t;
    use super::types_h::{__uint8_t, __uint32_t, __uint64_t};
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
    #[c2rust::src_loc = "254:9"]
    pub const X264_CSP_MASK: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
    #[c2rust::src_loc = "255:9"]
    pub const X264_CSP_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "256:9"]
    pub const X264_CSP_I400: ::core::ffi::c_int = 1;
    #[c2rust::src_loc = "257:9"]
    pub const X264_CSP_I420: ::core::ffi::c_int = 2;
    #[c2rust::src_loc = "258:9"]
    pub const X264_CSP_YV12: ::core::ffi::c_int = 3;
    #[c2rust::src_loc = "259:9"]
    pub const X264_CSP_NV12: ::core::ffi::c_int = 4;
    #[c2rust::src_loc = "260:9"]
    pub const X264_CSP_NV21: ::core::ffi::c_int = 5;
    #[c2rust::src_loc = "261:9"]
    pub const X264_CSP_I422: ::core::ffi::c_int = 6;
    #[c2rust::src_loc = "262:9"]
    pub const X264_CSP_YV16: ::core::ffi::c_int = 7;
    #[c2rust::src_loc = "263:9"]
    pub const X264_CSP_NV16: ::core::ffi::c_int = 8;
    #[c2rust::src_loc = "264:9"]
    pub const X264_CSP_YUYV: ::core::ffi::c_int = 9;
    #[c2rust::src_loc = "265:9"]
    pub const X264_CSP_UYVY: ::core::ffi::c_int = 10;
    #[c2rust::src_loc = "267:9"]
    pub const X264_CSP_I444: ::core::ffi::c_int = 12;
    #[c2rust::src_loc = "268:9"]
    pub const X264_CSP_YV24: ::core::ffi::c_int = 13;
    #[c2rust::src_loc = "269:9"]
    pub const X264_CSP_BGR: ::core::ffi::c_int = 14;
    #[c2rust::src_loc = "270:9"]
    pub const X264_CSP_BGRA: ::core::ffi::c_int = 15;
    #[c2rust::src_loc = "271:9"]
    pub const X264_CSP_RGB: ::core::ffi::c_int = 16;
    #[c2rust::src_loc = "272:9"]
    pub const X264_CSP_MAX: ::core::ffi::c_int = 0x11 as ::core::ffi::c_int;
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/input/input.h:26"]
pub mod input_h {
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
    #[c2rust::src_loc = "115:9"]
    pub const X264_CSP_OTHER: ::core::ffi::c_int = 0x4000 as ::core::ffi::c_int;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    use super::stdint_intn_h::int64_t;
    use super::x264_h::X264_CSP_MAX;
    extern "C" {
        #[c2rust::src_loc = "127:29"]
        pub static x264_cli_csps: [x264_cli_csp_t; 0];
        #[c2rust::src_loc = "130:1"]
        pub fn x264_cli_csp_depth_factor(csp: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "132:1"]
        pub fn x264_cli_pic_alloc_aligned(
            pic: *mut cli_pic_t,
            csp: ::core::ffi::c_int,
            width: ::core::ffi::c_int,
            height: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "134:1"]
        pub fn x264_cli_pic_clean(pic: *mut cli_pic_t);
        #[c2rust::src_loc = "137:1"]
        pub fn x264_cli_get_csp(csp: ::core::ffi::c_int) -> *const x264_cli_csp_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/filters/video/video.h:26"]
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
            unsafe extern "C" fn(
                hnd_t,
                *mut cli_pic_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub release_frame: Option<
            unsafe extern "C" fn(
                hnd_t,
                *mut cli_pic_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub free: Option<unsafe extern "C" fn(hnd_t) -> ()>,
        pub next: *mut cli_vid_filter_t,
    }
    use super::x264cli_h::hnd_t;
    use super::input_h::{video_info_t, cli_pic_t};
    use super::x264_h::x264_param_t;
}
#[c2rust::header_src = "/usr/include/libswscale/swscale.h:26"]
pub mod swscale_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "182:16"]
    pub struct SwsContext {
        pub av_class: *const AVClass,
        pub opaque: *mut ::core::ffi::c_void,
        pub flags: ::core::ffi::c_uint,
        pub scaler_params: [::core::ffi::c_double; 2],
        pub threads: ::core::ffi::c_int,
        pub dither: SwsDither,
        pub alpha_blend: SwsAlphaBlend,
        pub gamma_flag: ::core::ffi::c_int,
        pub src_w: ::core::ffi::c_int,
        pub src_h: ::core::ffi::c_int,
        pub dst_w: ::core::ffi::c_int,
        pub dst_h: ::core::ffi::c_int,
        pub src_format: ::core::ffi::c_int,
        pub dst_format: ::core::ffi::c_int,
        pub src_range: ::core::ffi::c_int,
        pub dst_range: ::core::ffi::c_int,
        pub src_v_chr_pos: ::core::ffi::c_int,
        pub src_h_chr_pos: ::core::ffi::c_int,
        pub dst_v_chr_pos: ::core::ffi::c_int,
        pub dst_h_chr_pos: ::core::ffi::c_int,
        pub intent: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "87:9"]
    pub type SwsAlphaBlend = ::core::ffi::c_uint;
    #[c2rust::src_loc = "91:5"]
    pub const SWS_ALPHA_BLEND_NB: SwsAlphaBlend = 3;
    #[c2rust::src_loc = "90:5"]
    pub const SWS_ALPHA_BLEND_CHECKERBOARD: SwsAlphaBlend = 2;
    #[c2rust::src_loc = "89:5"]
    pub const SWS_ALPHA_BLEND_UNIFORM: SwsAlphaBlend = 1;
    #[c2rust::src_loc = "88:5"]
    pub const SWS_ALPHA_BLEND_NONE: SwsAlphaBlend = 0;
    #[c2rust::src_loc = "77:9"]
    pub type SwsDither = ::core::ffi::c_uint;
    #[c2rust::src_loc = "84:5"]
    pub const SWS_DITHER_NB: SwsDither = 6;
    #[c2rust::src_loc = "83:5"]
    pub const SWS_DITHER_X_DITHER: SwsDither = 5;
    #[c2rust::src_loc = "82:5"]
    pub const SWS_DITHER_A_DITHER: SwsDither = 4;
    #[c2rust::src_loc = "81:5"]
    pub const SWS_DITHER_ED: SwsDither = 3;
    #[c2rust::src_loc = "80:5"]
    pub const SWS_DITHER_BAYER: SwsDither = 2;
    #[c2rust::src_loc = "79:5"]
    pub const SWS_DITHER_AUTO: SwsDither = 1;
    #[c2rust::src_loc = "78:5"]
    pub const SWS_DITHER_NONE: SwsDither = 0;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "395:16"]
    pub struct SwsFilter {
        pub lumH: *mut SwsVector,
        pub lumV: *mut SwsVector,
        pub chrH: *mut SwsVector,
        pub chrV: *mut SwsVector,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "389:16"]
    pub struct SwsVector {
        pub coeff: *mut ::core::ffi::c_double,
        pub length: ::core::ffi::c_int,
    }
    #[c2rust::src_loc = "155:5"]
    pub const SWS_ACCURATE_RND: SwsFlags = 262144;
    #[c2rust::src_loc = "145:5"]
    pub const SWS_FULL_CHR_H_INP: SwsFlags = 16384;
    #[c2rust::src_loc = "132:5"]
    pub const SWS_FULL_CHR_H_INT: SwsFlags = 8192;
    #[c2rust::src_loc = "98:5"]
    pub const SWS_FAST_BILINEAR: SwsFlags = 1;
    #[c2rust::src_loc = "100:5"]
    pub const SWS_BICUBIC: SwsFlags = 4;
    #[c2rust::src_loc = "108:5"]
    pub const SWS_SPLINE: SwsFlags = 1024;
    #[c2rust::src_loc = "107:5"]
    pub const SWS_LANCZOS: SwsFlags = 512;
    #[c2rust::src_loc = "106:5"]
    pub const SWS_SINC: SwsFlags = 256;
    #[c2rust::src_loc = "105:5"]
    pub const SWS_GAUSS: SwsFlags = 128;
    #[c2rust::src_loc = "104:5"]
    pub const SWS_BICUBLIN: SwsFlags = 64;
    #[c2rust::src_loc = "103:5"]
    pub const SWS_AREA: SwsFlags = 32;
    #[c2rust::src_loc = "102:5"]
    pub const SWS_POINT: SwsFlags = 16;
    #[c2rust::src_loc = "101:5"]
    pub const SWS_X: SwsFlags = 8;
    #[c2rust::src_loc = "99:5"]
    pub const SWS_BILINEAR: SwsFlags = 2;
    #[c2rust::src_loc = "94:9"]
    pub type SwsFlags = ::core::ffi::c_uint;
    #[c2rust::src_loc = "162:5"]
    pub const SWS_ERROR_DIFFUSION: SwsFlags = 8388608;
    #[c2rust::src_loc = "161:5"]
    pub const SWS_DIRECT_BGR: SwsFlags = 32768;
    #[c2rust::src_loc = "156:5"]
    pub const SWS_BITEXACT: SwsFlags = 524288;
    #[c2rust::src_loc = "119:5"]
    pub const SWS_PRINT_INFO: SwsFlags = 4096;
    #[c2rust::src_loc = "114:5"]
    pub const SWS_STRICT: SwsFlags = 2048;
    #[c2rust::src_loc = "375:9"]
    pub const SWS_CS_DEFAULT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
    use super::log_h::AVClass;
    use super::stdint_uintn_h::uint8_t;
    use super::pixfmt_h::AVPixelFormat;
    extern "C" {
        #[c2rust::src_loc = "248:1"]
        pub fn sws_alloc_context() -> *mut SwsContext;
        #[c2rust::src_loc = "385:1"]
        pub fn sws_getCoefficients(
            colorspace: ::core::ffi::c_int,
        ) -> *const ::core::ffi::c_int;
        #[c2rust::src_loc = "406:1"]
        pub fn sws_isSupportedInput(pix_fmt: AVPixelFormat) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "412:1"]
        pub fn sws_isSupportedOutput(pix_fmt: AVPixelFormat) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "432:1"]
        pub fn sws_init_context(
            sws_context: *mut SwsContext,
            srcFilter: *mut SwsFilter,
            dstFilter: *mut SwsFilter,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "439:1"]
        pub fn sws_freeContext(swsContext: *mut SwsContext);
        #[c2rust::src_loc = "494:1"]
        pub fn sws_scale(
            c: *mut SwsContext,
            srcSlice: *const *const uint8_t,
            srcStride: *const ::core::ffi::c_int,
            srcSliceY: ::core::ffi::c_int,
            srcSliceH: ::core::ffi::c_int,
            dst: *const *mut uint8_t,
            dstStride: *const ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "596:1"]
        pub fn sws_setColorspaceDetails(
            c: *mut SwsContext,
            inv_table: *const ::core::ffi::c_int,
            srcRange: ::core::ffi::c_int,
            table: *const ::core::ffi::c_int,
            dstRange: ::core::ffi::c_int,
            brightness: ::core::ffi::c_int,
            contrast: ::core::ffi::c_int,
            saturation: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libavutil/log.h:26"]
pub mod log_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "76:16"]
    pub struct AVClass {
        pub class_name: *const ::core::ffi::c_char,
        pub item_name: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const ::core::ffi::c_char,
        >,
        pub option: *const AVOption,
        pub version: ::core::ffi::c_int,
        pub log_level_offset_offset: ::core::ffi::c_int,
        pub parent_log_context_offset: ::core::ffi::c_int,
        pub category: AVClassCategory,
        pub get_category: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> AVClassCategory,
        >,
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
        pub child_class_iterate: Option<
            unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass,
        >,
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
    use super::opt_h::{AVOption, AVOptionRanges};
}
#[c2rust::header_src = "/usr/include/libavutil/opt.h:26"]
pub mod opt_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "508:16"]
    pub struct AVOptionRanges {
        pub range: *mut *mut AVOptionRange,
        pub nb_ranges: ::core::ffi::c_int,
        pub nb_components: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "485:16"]
    pub struct AVOptionRange {
        pub str_0: *const ::core::ffi::c_char,
        pub value_min: ::core::ffi::c_double,
        pub value_max: ::core::ffi::c_double,
        pub component_min: ::core::ffi::c_double,
        pub component_max: ::core::ffi::c_double,
        pub is_range: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "429:16"]
    pub struct AVOption {
        pub name: *const ::core::ffi::c_char,
        pub help: *const ::core::ffi::c_char,
        pub offset: ::core::ffi::c_int,
        pub type_0: AVOptionType,
        pub default_val: C2RustUnnamed_5,
        pub min: ::core::ffi::c_double,
        pub max: ::core::ffi::c_double,
        pub flags: ::core::ffi::c_int,
        pub unit: *const ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "451:5"]
    pub union C2RustUnnamed_5 {
        pub i64_0: int64_t,
        pub dbl: ::core::ffi::c_double,
        pub str_0: *const ::core::ffi::c_char,
        pub q: AVRational,
        pub arr: *const AVOptionArrayDef,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "395:16"]
    pub struct AVOptionArrayDef {
        pub def: *const ::core::ffi::c_char,
        pub size_min: ::core::ffi::c_uint,
        pub size_max: ::core::ffi::c_uint,
        pub sep: ::core::ffi::c_char,
    }
    #[c2rust::src_loc = "251:1"]
    pub type AVOptionType = ::core::ffi::c_uint;
    #[c2rust::src_loc = "346:5"]
    pub const AV_OPT_TYPE_FLAG_ARRAY: AVOptionType = 65536;
    #[c2rust::src_loc = "335:5"]
    pub const AV_OPT_TYPE_UINT: AVOptionType = 20;
    #[c2rust::src_loc = "331:5"]
    pub const AV_OPT_TYPE_CHLAYOUT: AVOptionType = 19;
    #[c2rust::src_loc = "327:5"]
    pub const AV_OPT_TYPE_BOOL: AVOptionType = 18;
    #[c2rust::src_loc = "323:5"]
    pub const AV_OPT_TYPE_COLOR: AVOptionType = 17;
    #[c2rust::src_loc = "319:5"]
    pub const AV_OPT_TYPE_DURATION: AVOptionType = 16;
    #[c2rust::src_loc = "315:5"]
    pub const AV_OPT_TYPE_VIDEO_RATE: AVOptionType = 15;
    #[c2rust::src_loc = "311:5"]
    pub const AV_OPT_TYPE_SAMPLE_FMT: AVOptionType = 14;
    #[c2rust::src_loc = "307:5"]
    pub const AV_OPT_TYPE_PIXEL_FMT: AVOptionType = 13;
    #[c2rust::src_loc = "303:5"]
    pub const AV_OPT_TYPE_IMAGE_SIZE: AVOptionType = 12;
    #[c2rust::src_loc = "299:5"]
    pub const AV_OPT_TYPE_CONST: AVOptionType = 11;
    #[c2rust::src_loc = "294:5"]
    pub const AV_OPT_TYPE_UINT64: AVOptionType = 10;
    #[c2rust::src_loc = "290:5"]
    pub const AV_OPT_TYPE_DICT: AVOptionType = 9;
    #[c2rust::src_loc = "286:5"]
    pub const AV_OPT_TYPE_BINARY: AVOptionType = 8;
    #[c2rust::src_loc = "280:5"]
    pub const AV_OPT_TYPE_RATIONAL: AVOptionType = 7;
    #[c2rust::src_loc = "276:5"]
    pub const AV_OPT_TYPE_STRING: AVOptionType = 6;
    #[c2rust::src_loc = "271:5"]
    pub const AV_OPT_TYPE_FLOAT: AVOptionType = 5;
    #[c2rust::src_loc = "267:5"]
    pub const AV_OPT_TYPE_DOUBLE: AVOptionType = 4;
    #[c2rust::src_loc = "263:5"]
    pub const AV_OPT_TYPE_INT64: AVOptionType = 3;
    #[c2rust::src_loc = "259:5"]
    pub const AV_OPT_TYPE_INT: AVOptionType = 2;
    #[c2rust::src_loc = "255:5"]
    pub const AV_OPT_TYPE_FLAGS: AVOptionType = 1;
    use super::stdint_intn_h::int64_t;
    use super::rational_h::AVRational;
    extern "C" {
        #[c2rust::src_loc = "870:1"]
        pub fn av_opt_set_int(
            obj: *mut ::core::ffi::c_void,
            name: *const ::core::ffi::c_char,
            val: int64_t,
            search_flags: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/libavutil/rational.h:26"]
pub mod rational_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "58:16"]
    pub struct AVRational {
        pub num: ::core::ffi::c_int,
        pub den: ::core::ffi::c_int,
    }
}
#[c2rust::header_src = "/usr/include/libavutil/pixfmt.h:26"]
pub mod pixfmt_h {
    #[c2rust::src_loc = "72:5"]
    pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
    #[c2rust::src_loc = "88:5"]
    pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
    #[c2rust::src_loc = "74:5"]
    pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
    #[c2rust::src_loc = "97:5"]
    pub const AV_PIX_FMT_NV21: AVPixelFormat = 24;
    #[c2rust::src_loc = "96:5"]
    pub const AV_PIX_FMT_NV12: AVPixelFormat = 23;
    #[c2rust::src_loc = "102:5"]
    pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28;
    #[c2rust::src_loc = "205:5"]
    pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 107;
    #[c2rust::src_loc = "76:5"]
    pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3;
    #[c2rust::src_loc = "146:5"]
    pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 58;
    #[c2rust::src_loc = "75:5"]
    pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2;
    #[c2rust::src_loc = "110:5"]
    pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
    #[c2rust::src_loc = "78:5"]
    pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
    #[c2rust::src_loc = "132:5"]
    pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 49;
    #[c2rust::src_loc = "77:5"]
    pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
    #[c2rust::src_loc = "130:5"]
    pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 47;
    #[c2rust::src_loc = "73:5"]
    pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
    #[c2rust::src_loc = "128:5"]
    pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 45;
    #[c2rust::src_loc = "81:5"]
    pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
    #[c2rust::src_loc = "105:5"]
    pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
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
    #[c2rust::src_loc = "131:5"]
    pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 48;
    #[c2rust::src_loc = "129:5"]
    pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 46;
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
    #[c2rust::src_loc = "109:5"]
    pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
    #[c2rust::src_loc = "108:5"]
    pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
    #[c2rust::src_loc = "107:5"]
    pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
    #[c2rust::src_loc = "106:5"]
    pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
    #[c2rust::src_loc = "104:5"]
    pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
    #[c2rust::src_loc = "101:5"]
    pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27;
    #[c2rust::src_loc = "100:5"]
    pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26;
    #[c2rust::src_loc = "99:5"]
    pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25;
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
    #[c2rust::src_loc = "80:5"]
    pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
    #[c2rust::src_loc = "79:5"]
    pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
}
#[c2rust::header_src = "/usr/include/libavutil/pixdesc.h:26"]
pub mod pixdesc_h {
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
    #[c2rust::src_loc = "120:9"]
    pub const AV_PIX_FMT_FLAG_PAL: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
        << 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "136:9"]
    pub const AV_PIX_FMT_FLAG_RGB: ::core::ffi::c_int = (1 as ::core::ffi::c_int)
        << 5 as ::core::ffi::c_int;
    use super::stdint_uintn_h::{uint8_t, uint64_t};
    use super::pixfmt_h::{AVPixelFormat, AV_PIX_FMT_YUV420P};
    extern "C" {
        #[c2rust::src_loc = "186:1"]
        pub fn av_pix_fmt_desc_get(pix_fmt: AVPixelFormat) -> *const AVPixFmtDescriptor;
        #[c2rust::src_loc = "313:1"]
        pub fn av_get_pix_fmt_name(pix_fmt: AVPixelFormat) -> *const ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:26"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
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
        #[c2rust::src_loc = "64:1"]
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "156:1"]
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "246:1"]
        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "350:1"]
        pub fn strstr(
            __haystack: *const ::core::ffi::c_char,
            __needle: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:26"]
pub mod base_h {
    use super::stdint_uintn_h::{uint32_t, uint64_t};
    extern "C" {
        #[c2rust::src_loc = "271:10"]
        pub fn x264_reduce_fraction(n: *mut uint32_t, d: *mut uint32_t);
        #[c2rust::src_loc = "272:10"]
        pub fn x264_reduce_fraction64(n: *mut uint64_t, d: *mut uint64_t);
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/limits.h:45"]
pub mod limits_h {
    #[c2rust::src_loc = "50:9"]
    pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
    use super::internal::__INT_MAX__;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:0"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/stdio.h:26"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "366:1"]
        pub fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "450:1"]
        pub fn sscanf(
            __s: *const ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:26"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "355:1"]
        pub fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    }
}
#[c2rust::header_src = "/usr/include/strings.h:26"]
pub mod strings_h {
    extern "C" {
        #[c2rust::src_loc = "116:1"]
        pub fn strcasecmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/filters/filters.h:26"]
pub mod filters_h {
    extern "C" {
        #[c2rust::src_loc = "33:1"]
        pub fn x264_split_options(
            opt_str: *const ::core::ffi::c_char,
            options: *const *const ::core::ffi::c_char,
        ) -> *mut *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "34:1"]
        pub fn x264_get_option(
            name: *const ::core::ffi::c_char,
            split_options: *mut *mut ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "37:1"]
        pub fn x264_otoi(
            str: *const ::core::ffi::c_char,
            def: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "38:1"]
        pub fn x264_otos(
            str: *mut ::core::ffi::c_char,
            def: *mut ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
    }
}
pub use self::internal::{__va_list_tag, __INT_MAX__};
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{__uint8_t, __uint32_t, __int64_t, __uint64_t};
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t, uint64_t};
pub use self::x264_h::{
    x264_nal_t, x264_zone_t, x264_param_t, C2RustUnnamed, C2RustUnnamed_0,
    C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4, X264_CSP_MASK,
    X264_CSP_NONE, X264_CSP_I400, X264_CSP_I420, X264_CSP_YV12, X264_CSP_NV12,
    X264_CSP_NV21, X264_CSP_I422, X264_CSP_YV16, X264_CSP_NV16, X264_CSP_YUYV,
    X264_CSP_UYVY, X264_CSP_I444, X264_CSP_YV24, X264_CSP_BGR, X264_CSP_BGRA,
    X264_CSP_RGB, X264_CSP_MAX, X264_CSP_VFLIP, X264_CSP_HIGH_DEPTH, X264_LOG_ERROR,
    X264_LOG_WARNING, X264_LOG_INFO, x264_t,
};
pub use self::x264cli_h::{hnd_t, x264_cli_log};
pub use self::input_h::{
    video_info_t, cli_image_t, cli_pic_t, x264_cli_csp_t, X264_CSP_CLI_MAX,
    X264_CSP_OTHER, x264_cli_csps, x264_cli_csp_depth_factor, x264_cli_pic_alloc_aligned,
    x264_cli_pic_clean, x264_cli_get_csp,
};
pub use self::video_h::cli_vid_filter_t;
pub use self::swscale_h::{
    SwsContext, SwsAlphaBlend, SWS_ALPHA_BLEND_NB, SWS_ALPHA_BLEND_CHECKERBOARD,
    SWS_ALPHA_BLEND_UNIFORM, SWS_ALPHA_BLEND_NONE, SwsDither, SWS_DITHER_NB,
    SWS_DITHER_X_DITHER, SWS_DITHER_A_DITHER, SWS_DITHER_ED, SWS_DITHER_BAYER,
    SWS_DITHER_AUTO, SWS_DITHER_NONE, SwsFilter, SwsVector, SWS_ACCURATE_RND,
    SWS_FULL_CHR_H_INP, SWS_FULL_CHR_H_INT, SWS_FAST_BILINEAR, SWS_BICUBIC, SWS_SPLINE,
    SWS_LANCZOS, SWS_SINC, SWS_GAUSS, SWS_BICUBLIN, SWS_AREA, SWS_POINT, SWS_X,
    SWS_BILINEAR, SwsFlags, SWS_ERROR_DIFFUSION, SWS_DIRECT_BGR, SWS_BITEXACT,
    SWS_PRINT_INFO, SWS_STRICT, SWS_CS_DEFAULT, sws_alloc_context, sws_getCoefficients,
    sws_isSupportedInput, sws_isSupportedOutput, sws_init_context, sws_freeContext,
    sws_scale, sws_setColorspaceDetails,
};
pub use self::log_h::{
    AVClass, AVClassCategory, AV_CLASS_CATEGORY_NB, AV_CLASS_CATEGORY_DEVICE_INPUT,
    AV_CLASS_CATEGORY_DEVICE_OUTPUT, AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT,
    AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT, AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT,
    AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT, AV_CLASS_CATEGORY_HWDEVICE,
    AV_CLASS_CATEGORY_SWRESAMPLER, AV_CLASS_CATEGORY_SWSCALER,
    AV_CLASS_CATEGORY_BITSTREAM_FILTER, AV_CLASS_CATEGORY_FILTER,
    AV_CLASS_CATEGORY_DECODER, AV_CLASS_CATEGORY_ENCODER, AV_CLASS_CATEGORY_DEMUXER,
    AV_CLASS_CATEGORY_MUXER, AV_CLASS_CATEGORY_OUTPUT, AV_CLASS_CATEGORY_INPUT,
    AV_CLASS_CATEGORY_NA,
};
pub use self::opt_h::{
    AVOptionRanges, AVOptionRange, AVOption, C2RustUnnamed_5, AVOptionArrayDef,
    AVOptionType, AV_OPT_TYPE_FLAG_ARRAY, AV_OPT_TYPE_UINT, AV_OPT_TYPE_CHLAYOUT,
    AV_OPT_TYPE_BOOL, AV_OPT_TYPE_COLOR, AV_OPT_TYPE_DURATION, AV_OPT_TYPE_VIDEO_RATE,
    AV_OPT_TYPE_SAMPLE_FMT, AV_OPT_TYPE_PIXEL_FMT, AV_OPT_TYPE_IMAGE_SIZE,
    AV_OPT_TYPE_CONST, AV_OPT_TYPE_UINT64, AV_OPT_TYPE_DICT, AV_OPT_TYPE_BINARY,
    AV_OPT_TYPE_RATIONAL, AV_OPT_TYPE_STRING, AV_OPT_TYPE_FLOAT, AV_OPT_TYPE_DOUBLE,
    AV_OPT_TYPE_INT64, AV_OPT_TYPE_INT, AV_OPT_TYPE_FLAGS, av_opt_set_int,
};
pub use self::rational_h::AVRational;
pub use self::pixfmt_h::{
    AV_PIX_FMT_NONE, AV_PIX_FMT_UYVY422, AV_PIX_FMT_YUYV422, AV_PIX_FMT_NV21,
    AV_PIX_FMT_NV12, AV_PIX_FMT_BGRA, AV_PIX_FMT_BGRA64LE, AV_PIX_FMT_BGR24,
    AV_PIX_FMT_BGR48LE, AV_PIX_FMT_RGB24, AV_PIX_FMT_RGB48LE, AV_PIX_FMT_YUV444P,
    AV_PIX_FMT_YUV444P16LE, AV_PIX_FMT_YUV422P, AV_PIX_FMT_YUV422P16LE,
    AV_PIX_FMT_YUV420P, AV_PIX_FMT_YUV420P16LE, AV_PIX_FMT_GRAY8, AV_PIX_FMT_GRAY16LE,
    AVPixelFormat, AV_PIX_FMT_NB, AV_PIX_FMT_OHCODEC, AV_PIX_FMT_GBRP12MSBLE,
    AV_PIX_FMT_GBRP12MSBBE, AV_PIX_FMT_GBRP10MSBLE, AV_PIX_FMT_GBRP10MSBBE,
    AV_PIX_FMT_YUV444P12MSBLE, AV_PIX_FMT_YUV444P12MSBBE, AV_PIX_FMT_YUV444P10MSBLE,
    AV_PIX_FMT_YUV444P10MSBBE, AV_PIX_FMT_GBRAP32LE, AV_PIX_FMT_GBRAP32BE,
    AV_PIX_FMT_YAF16LE, AV_PIX_FMT_YAF16BE, AV_PIX_FMT_YAF32LE, AV_PIX_FMT_YAF32BE,
    AV_PIX_FMT_GRAY32LE, AV_PIX_FMT_GRAY32BE, AV_PIX_FMT_AMF_SURFACE,
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
    AV_PIX_FMT_YA16LE, AV_PIX_FMT_YA16BE, AV_PIX_FMT_YVYU422, AV_PIX_FMT_BGRA64BE,
    AV_PIX_FMT_RGBA64LE, AV_PIX_FMT_RGBA64BE, AV_PIX_FMT_NV20BE, AV_PIX_FMT_NV20LE,
    AV_PIX_FMT_NV16, AV_PIX_FMT_XYZ12BE, AV_PIX_FMT_XYZ12LE, AV_PIX_FMT_VDPAU,
    AV_PIX_FMT_YUVA444P16LE, AV_PIX_FMT_YUVA444P16BE, AV_PIX_FMT_YUVA422P16LE,
    AV_PIX_FMT_YUVA422P16BE, AV_PIX_FMT_YUVA420P16LE, AV_PIX_FMT_YUVA420P16BE,
    AV_PIX_FMT_YUVA444P10LE, AV_PIX_FMT_YUVA444P10BE, AV_PIX_FMT_YUVA422P10LE,
    AV_PIX_FMT_YUVA422P10BE, AV_PIX_FMT_YUVA420P10LE, AV_PIX_FMT_YUVA420P10BE,
    AV_PIX_FMT_YUVA444P9LE, AV_PIX_FMT_YUVA444P9BE, AV_PIX_FMT_YUVA422P9LE,
    AV_PIX_FMT_YUVA422P9BE, AV_PIX_FMT_YUVA420P9LE, AV_PIX_FMT_YUVA420P9BE,
    AV_PIX_FMT_YUVA444P, AV_PIX_FMT_YUVA422P, AV_PIX_FMT_GBRP16LE, AV_PIX_FMT_GBRP16BE,
    AV_PIX_FMT_GBRP10LE, AV_PIX_FMT_GBRP10BE, AV_PIX_FMT_GBRP9LE, AV_PIX_FMT_GBRP9BE,
    AV_PIX_FMT_GBR24P, AV_PIX_FMT_GBRP, AV_PIX_FMT_YUV422P9LE, AV_PIX_FMT_YUV422P9BE,
    AV_PIX_FMT_YUV444P10LE, AV_PIX_FMT_YUV444P10BE, AV_PIX_FMT_YUV444P9LE,
    AV_PIX_FMT_YUV444P9BE, AV_PIX_FMT_YUV422P10LE, AV_PIX_FMT_YUV422P10BE,
    AV_PIX_FMT_YUV420P10LE, AV_PIX_FMT_YUV420P10BE, AV_PIX_FMT_YUV420P9LE,
    AV_PIX_FMT_YUV420P9BE, AV_PIX_FMT_BGR48BE, AV_PIX_FMT_GRAY8A, AV_PIX_FMT_Y400A,
    AV_PIX_FMT_YA8, AV_PIX_FMT_BGR444BE, AV_PIX_FMT_BGR444LE, AV_PIX_FMT_RGB444BE,
    AV_PIX_FMT_RGB444LE, AV_PIX_FMT_DXVA2_VLD, AV_PIX_FMT_YUV444P16BE,
    AV_PIX_FMT_YUV422P16BE, AV_PIX_FMT_YUV420P16BE, AV_PIX_FMT_VAAPI,
    AV_PIX_FMT_BGR555LE, AV_PIX_FMT_BGR555BE, AV_PIX_FMT_BGR565LE, AV_PIX_FMT_BGR565BE,
    AV_PIX_FMT_RGB555LE, AV_PIX_FMT_RGB555BE, AV_PIX_FMT_RGB565LE, AV_PIX_FMT_RGB565BE,
    AV_PIX_FMT_RGB48BE, AV_PIX_FMT_YUVA420P, AV_PIX_FMT_YUVJ440P, AV_PIX_FMT_YUV440P,
    AV_PIX_FMT_GRAY16BE, AV_PIX_FMT_ABGR, AV_PIX_FMT_RGBA, AV_PIX_FMT_ARGB,
    AV_PIX_FMT_RGB4_BYTE, AV_PIX_FMT_RGB4, AV_PIX_FMT_RGB8, AV_PIX_FMT_BGR4_BYTE,
    AV_PIX_FMT_BGR4, AV_PIX_FMT_BGR8, AV_PIX_FMT_UYYVYY411, AV_PIX_FMT_YUVJ444P,
    AV_PIX_FMT_YUVJ422P, AV_PIX_FMT_YUVJ420P, AV_PIX_FMT_PAL8, AV_PIX_FMT_MONOBLACK,
    AV_PIX_FMT_MONOWHITE, AV_PIX_FMT_YUV411P, AV_PIX_FMT_YUV410P,
};
pub use self::pixdesc_h::{
    AVComponentDescriptor, AVPixFmtDescriptor, AV_PIX_FMT_FLAG_PAL, AV_PIX_FMT_FLAG_RGB,
    av_pix_fmt_desc_get, av_get_pix_fmt_name,
};
use self::stdlib_h::{calloc, free};
use self::string_h::{memcmp, strcmp, strchr, strstr, strlen};
use self::base_h::{x264_reduce_fraction, x264_reduce_fraction64};
pub use self::limits_h::INT_MAX;
pub use self::__stddef_null_h::NULL;
use self::stdio_h::{printf, sscanf};
use self::mathcalls_h::round;
use self::strings_h::strcasecmp;
use self::filters_h::{x264_split_options, x264_get_option, x264_otoi, x264_otos};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "61:9"]
pub struct resizer_hnd_t {
    pub prev_hnd: hnd_t,
    pub prev_filter: cli_vid_filter_t,
    pub buffer: cli_pic_t,
    pub buffer_allocated: ::core::ffi::c_int,
    pub dst_csp: ::core::ffi::c_int,
    pub input_range: ::core::ffi::c_int,
    pub ctx: *mut SwsContext,
    pub ctx_flags: uint32_t,
    pub pre_swap_chroma: ::core::ffi::c_int,
    pub post_swap_chroma: ::core::ffi::c_int,
    pub fast_mono: ::core::ffi::c_int,
    pub variable_input: ::core::ffi::c_int,
    pub working: ::core::ffi::c_int,
    pub dst: frame_prop_t,
    pub scale: frame_prop_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "53:9"]
pub struct frame_prop_t {
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub pix_fmt: ::core::ffi::c_int,
    pub range: ::core::ffi::c_int,
}
#[c2rust::src_loc = "28:9"]
pub const NAME: [::core::ffi::c_char; 7] = unsafe {
    ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"resize\0")
};
#[c2rust::src_loc = "33:1"]
unsafe extern "C" fn full_check(
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
) -> ::core::ffi::c_int {
    let mut required: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    required |= ((*info).csp != (*param).i_csp) as ::core::ffi::c_int;
    required |= ((*info).width != (*param).i_width) as ::core::ffi::c_int;
    required |= ((*info).height != (*param).i_height) as ::core::ffi::c_int;
    required |= ((*info).fullrange != (*param).vui.b_fullrange) as ::core::ffi::c_int;
    return required;
}
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn help(mut longhelp: ::core::ffi::c_int) {
    printf(
        b"      resize:[width,height][,sar][,fittobox][,csp][,method]\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if longhelp == 0 {
        return;
    }
    printf(
        b"            resizes frames based on the given criteria:\n            - resolution only: resizes and adapts sar to avoid stretching\n            - sar only: sets the sar and resizes to avoid stretching\n            - resolution and sar: resizes to given resolution and sets the sar\n            - fittobox: resizes the video based on the desired constraints\n               - width, height, both\n            - fittobox and sar: same as above except with specified sar\n            - csp: convert to the given csp. syntax: [name][:depth]\n               - valid csp names [keep current]: \0"
            as *const u8 as *const ::core::ffi::c_char,
    );
    let mut i: ::core::ffi::c_int = X264_CSP_NONE + 1 as ::core::ffi::c_int;
    while i < X264_CSP_CLI_MAX {
        if !(*x264_cli_csps.as_ptr().offset(i as isize)).name.is_null() {
            printf(
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                (*x264_cli_csps.as_ptr().offset(i as isize)).name,
            );
            if (i + 1 as ::core::ffi::c_int) < X264_CSP_CLI_MAX {
                printf(b", \0" as *const u8 as *const ::core::ffi::c_char);
            }
        }
        i += 1;
    }
    printf(
        b"\n               - depth: 8 or 16 bits per pixel [keep current]\n            note: not all depths are supported by all csps.\n            - method: use resizer method [\"bicubic\"]\n               - fastbilinear, bilinear, bicubic, experimental, point,\n               - area, bicublin, gauss, sinc, lanczos, spline\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
}
#[c2rust::src_loc = "114:1"]
unsafe extern "C" fn convert_method_to_flag(
    mut name: *const ::core::ffi::c_char,
) -> uint32_t {
    let mut flag: uint32_t = 0 as uint32_t;
    if strcasecmp(name, b"fastbilinear\0" as *const u8 as *const ::core::ffi::c_char)
        == 0
    {
        flag = SWS_FAST_BILINEAR as ::core::ffi::c_int as uint32_t;
    } else if strcasecmp(name, b"bilinear\0" as *const u8 as *const ::core::ffi::c_char)
        == 0
    {
        flag = SWS_BILINEAR as ::core::ffi::c_int as uint32_t;
    } else if strcasecmp(name, b"bicubic\0" as *const u8 as *const ::core::ffi::c_char)
        == 0
    {
        flag = SWS_BICUBIC as ::core::ffi::c_int as uint32_t;
    } else if strcasecmp(
        name,
        b"experimental\0" as *const u8 as *const ::core::ffi::c_char,
    ) == 0
    {
        flag = SWS_X as ::core::ffi::c_int as uint32_t;
    } else if strcasecmp(name, b"point\0" as *const u8 as *const ::core::ffi::c_char)
        == 0
    {
        flag = SWS_POINT as ::core::ffi::c_int as uint32_t;
    } else if strcasecmp(name, b"area\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        flag = SWS_AREA as ::core::ffi::c_int as uint32_t;
    } else if strcasecmp(name, b"bicublin\0" as *const u8 as *const ::core::ffi::c_char)
        == 0
    {
        flag = SWS_BICUBLIN as ::core::ffi::c_int as uint32_t;
    } else if strcasecmp(name, b"gauss\0" as *const u8 as *const ::core::ffi::c_char)
        == 0
    {
        flag = SWS_GAUSS as ::core::ffi::c_int as uint32_t;
    } else if strcasecmp(name, b"sinc\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        flag = SWS_SINC as ::core::ffi::c_int as uint32_t;
    } else if strcasecmp(name, b"lanczos\0" as *const u8 as *const ::core::ffi::c_char)
        == 0
    {
        flag = SWS_LANCZOS as ::core::ffi::c_int as uint32_t;
    } else if strcasecmp(name, b"spline\0" as *const u8 as *const ::core::ffi::c_char)
        == 0
    {
        flag = SWS_SPLINE as ::core::ffi::c_int as uint32_t;
    } else {
        flag = SWS_BICUBIC as ::core::ffi::c_int as uint32_t;
    }
    return flag;
}
#[c2rust::src_loc = "144:1"]
unsafe extern "C" fn convert_csp_to_pix_fmt(
    mut csp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if csp & X264_CSP_OTHER != 0 {
        return csp & X264_CSP_MASK;
    }
    match csp & X264_CSP_MASK {
        X264_CSP_I400 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_GRAY16LE as ::core::ffi::c_int
            } else {
                AV_PIX_FMT_GRAY8 as ::core::ffi::c_int
            };
        }
        X264_CSP_YV12 | X264_CSP_I420 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_YUV420P16LE as ::core::ffi::c_int
            } else {
                AV_PIX_FMT_YUV420P as ::core::ffi::c_int
            };
        }
        X264_CSP_YV16 | X264_CSP_I422 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_YUV422P16LE as ::core::ffi::c_int
            } else {
                AV_PIX_FMT_YUV422P as ::core::ffi::c_int
            };
        }
        X264_CSP_YV24 | X264_CSP_I444 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_YUV444P16LE as ::core::ffi::c_int
            } else {
                AV_PIX_FMT_YUV444P as ::core::ffi::c_int
            };
        }
        X264_CSP_RGB => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_RGB48LE as ::core::ffi::c_int
            } else {
                AV_PIX_FMT_RGB24 as ::core::ffi::c_int
            };
        }
        X264_CSP_BGR => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_BGR48LE as ::core::ffi::c_int
            } else {
                AV_PIX_FMT_BGR24 as ::core::ffi::c_int
            };
        }
        X264_CSP_BGRA => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_BGRA64LE as ::core::ffi::c_int
            } else {
                AV_PIX_FMT_BGRA as ::core::ffi::c_int
            };
        }
        X264_CSP_NV12 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_NONE as ::core::ffi::c_int
            } else {
                AV_PIX_FMT_NV12 as ::core::ffi::c_int
            };
        }
        X264_CSP_NV21 => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_NONE as ::core::ffi::c_int
            } else {
                AV_PIX_FMT_NV21 as ::core::ffi::c_int
            };
        }
        X264_CSP_YUYV => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_NONE as ::core::ffi::c_int
            } else {
                AV_PIX_FMT_YUYV422 as ::core::ffi::c_int
            };
        }
        X264_CSP_UYVY => {
            return if csp & X264_CSP_HIGH_DEPTH != 0 {
                AV_PIX_FMT_NONE as ::core::ffi::c_int
            } else {
                AV_PIX_FMT_UYVY422 as ::core::ffi::c_int
            };
        }
        X264_CSP_NV16 | _ => return AV_PIX_FMT_NONE as ::core::ffi::c_int,
    };
}
#[c2rust::src_loc = "171:1"]
unsafe extern "C" fn pix_number_of_planes(
    mut pix_desc: *const AVPixFmtDescriptor,
) -> ::core::ffi::c_int {
    let mut num_planes: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*pix_desc).nb_components as ::core::ffi::c_int {
        let mut plane_plus1: ::core::ffi::c_int = (*pix_desc).comp[i as usize].plane
            + 1 as ::core::ffi::c_int;
        num_planes = if plane_plus1 > num_planes { plane_plus1 } else { num_planes };
        i += 1;
    }
    return num_planes;
}
#[c2rust::src_loc = "182:1"]
unsafe extern "C" fn pick_closest_supported_csp(
    mut csp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut pix_fmt: ::core::ffi::c_int = convert_csp_to_pix_fmt(csp);
    let mut ret: ::core::ffi::c_int = X264_CSP_NONE;
    let mut pix_desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get(
        pix_fmt as AVPixelFormat,
    );
    if pix_desc.is_null() || (*pix_desc).name.is_null() {
        return ret;
    }
    let mut pix_fmt_name: *const ::core::ffi::c_char = (*pix_desc).name;
    let mut is_rgb: ::core::ffi::c_int = ((*pix_desc).flags
        & (AV_PIX_FMT_FLAG_RGB | AV_PIX_FMT_FLAG_PAL) as uint64_t) as ::core::ffi::c_int;
    let mut is_bgr: ::core::ffi::c_int = !strstr(
            pix_fmt_name,
            b"bgr\0" as *const u8 as *const ::core::ffi::c_char,
        )
        .is_null() as ::core::ffi::c_int;
    if is_bgr != 0 || is_rgb != 0 {
        if (*pix_desc).nb_components as ::core::ffi::c_int == 4 as ::core::ffi::c_int {
            ret = X264_CSP_BGRA;
        } else if is_bgr != 0 {
            ret = X264_CSP_BGR;
        } else {
            ret = X264_CSP_RGB;
        }
    } else if (*pix_desc).nb_components as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        || (*pix_desc).nb_components as ::core::ffi::c_int == 2 as ::core::ffi::c_int
    {
        ret = X264_CSP_I400;
    } else if (*pix_desc).log2_chroma_w as ::core::ffi::c_int != 0
        && (*pix_desc).log2_chroma_h as ::core::ffi::c_int != 0
    {
        ret = if pix_number_of_planes(pix_desc) == 2 as ::core::ffi::c_int {
            X264_CSP_NV12
        } else {
            X264_CSP_I420
        };
    } else if (*pix_desc).log2_chroma_w != 0 {
        ret = X264_CSP_I422;
    } else {
        ret = X264_CSP_I444;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*pix_desc).nb_components as ::core::ffi::c_int {
        if (*pix_desc).comp[i as usize].depth > 8 as ::core::ffi::c_int {
            ret |= X264_CSP_HIGH_DEPTH;
        }
        i += 1;
    }
    return ret;
}
#[c2rust::src_loc = "222:1"]
unsafe extern "C" fn handle_opts(
    mut optlist: *const *const ::core::ffi::c_char,
    mut opts: *mut *mut ::core::ffi::c_char,
    mut info: *mut video_info_t,
    mut h: *mut resizer_hnd_t,
) -> ::core::ffi::c_int {
    let mut out_sar_w: uint32_t = 0;
    let mut out_sar_h: uint32_t = 0;
    let mut str_width: *mut ::core::ffi::c_char = x264_get_option(
        *optlist.offset(0 as ::core::ffi::c_int as isize),
        opts,
    );
    let mut str_height: *mut ::core::ffi::c_char = x264_get_option(
        *optlist.offset(1 as ::core::ffi::c_int as isize),
        opts,
    );
    let mut str_sar: *mut ::core::ffi::c_char = x264_get_option(
        *optlist.offset(2 as ::core::ffi::c_int as isize),
        opts,
    );
    let mut fittobox: *mut ::core::ffi::c_char = x264_get_option(
        *optlist.offset(3 as ::core::ffi::c_int as isize),
        opts,
    );
    let mut str_csp: *mut ::core::ffi::c_char = x264_get_option(
        *optlist.offset(4 as ::core::ffi::c_int as isize),
        opts,
    );
    let mut width: ::core::ffi::c_int = x264_otoi(str_width, -(1 as ::core::ffi::c_int));
    let mut height: ::core::ffi::c_int = x264_otoi(
        str_height,
        -(1 as ::core::ffi::c_int),
    );
    let mut csp_only: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut in_sar_w: uint32_t = (*info).sar_width;
    let mut in_sar_h: uint32_t = (*info).sar_height;
    if !str_csp.is_null() {
        let mut str_depth: *mut ::core::ffi::c_char = strchr(str_csp, ':' as i32);
        let mut depth: ::core::ffi::c_int = x264_cli_csp_depth_factor((*info).csp)
            * 8 as ::core::ffi::c_int;
        if !str_depth.is_null() {
            let fresh0 = str_depth;
            str_depth = str_depth.offset(1);
            *fresh0 = '\0' as i32 as ::core::ffi::c_char;
            depth = x264_otoi(str_depth, -(1 as ::core::ffi::c_int));
            if depth != 8 as ::core::ffi::c_int && depth != 16 as ::core::ffi::c_int {
                x264_cli_log(
                    b"resize\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"unsupported bit depth %d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    depth,
                );
                return -(1 as ::core::ffi::c_int);
            }
        }
        let mut csp: ::core::ffi::c_int = 0;
        if strlen(str_csp) == 0 as size_t {
            csp = (*info).csp & X264_CSP_MASK;
        } else {
            csp = X264_CSP_CLI_MAX - 1 as ::core::ffi::c_int;
            while csp > X264_CSP_NONE {
                if !(*x264_cli_csps.as_ptr().offset(csp as isize)).name.is_null()
                    && strcasecmp(
                        (*x264_cli_csps.as_ptr().offset(csp as isize)).name,
                        str_csp,
                    ) == 0
                {
                    break;
                }
                csp -= 1;
            }
        }
        if csp == 0 as ::core::ffi::c_int {
            x264_cli_log(
                b"resize\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"unsupported colorspace `%s'\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                str_csp,
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*h).dst_csp = csp;
        if depth == 16 as ::core::ffi::c_int {
            (*h).dst_csp |= X264_CSP_HIGH_DEPTH;
        }
    }
    if in_sar_w == 0 || in_sar_h == 0 {
        in_sar_h = 1 as uint32_t;
        in_sar_w = in_sar_h;
    }
    if !str_sar.is_null() {
        if 2 as ::core::ffi::c_int
            != sscanf(
                str_sar,
                b"%u:%u\0" as *const u8 as *const ::core::ffi::c_char,
                &mut out_sar_w as *mut uint32_t,
                &mut out_sar_h as *mut uint32_t,
            )
            && 2 as ::core::ffi::c_int
                != sscanf(
                    str_sar,
                    b"%u/%u\0" as *const u8 as *const ::core::ffi::c_char,
                    &mut out_sar_w as *mut uint32_t,
                    &mut out_sar_h as *mut uint32_t,
                )
        {
            x264_cli_log(
                b"resize\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"invalid sar `%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                str_sar,
            );
            return -(1 as ::core::ffi::c_int);
        }
    } else {
        out_sar_h = 1 as uint32_t;
        out_sar_w = out_sar_h;
    }
    if !fittobox.is_null() {
        if strcasecmp(fittobox, b"both\0" as *const u8 as *const ::core::ffi::c_char)
            == 0
        {
            if width <= 0 as ::core::ffi::c_int || height <= 0 as ::core::ffi::c_int {
                x264_cli_log(
                    b"resize\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"invalid box resolution %sx%s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    x264_otos(
                        str_width,
                        b"<unset>\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    ),
                    x264_otos(
                        str_height,
                        b"<unset>\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    ),
                );
                return -(1 as ::core::ffi::c_int);
            }
        } else if strcasecmp(
            fittobox,
            b"width\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            if width <= 0 as ::core::ffi::c_int {
                x264_cli_log(
                    b"resize\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"invalid box width `%s'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    x264_otos(
                        str_width,
                        b"<unset>\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    ),
                );
                return -(1 as ::core::ffi::c_int);
            }
            height = INT_MAX;
        } else if strcasecmp(
            fittobox,
            b"height\0" as *const u8 as *const ::core::ffi::c_char,
        ) == 0
        {
            if height <= 0 as ::core::ffi::c_int {
                x264_cli_log(
                    b"resize\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"invalid box height `%s'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    x264_otos(
                        str_height,
                        b"<unset>\0" as *const u8 as *const ::core::ffi::c_char
                            as *mut ::core::ffi::c_char,
                    ),
                );
                return -(1 as ::core::ffi::c_int);
            }
            width = INT_MAX;
        } else {
            x264_cli_log(
                b"resize\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"invalid fittobox mode `%s'\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                fittobox,
            );
            return -(1 as ::core::ffi::c_int);
        }
        let mut csp_0: *const x264_cli_csp_t = x264_cli_get_csp((*h).dst_csp);
        let mut width_units: ::core::ffi::c_double = (*info).height
            as ::core::ffi::c_double * in_sar_h as ::core::ffi::c_double
            * out_sar_w as ::core::ffi::c_double;
        let mut height_units: ::core::ffi::c_double = (*info).width
            as ::core::ffi::c_double * in_sar_w as ::core::ffi::c_double
            * out_sar_h as ::core::ffi::c_double;
        width = width / (*csp_0).mod_width * (*csp_0).mod_width;
        height = height / (*csp_0).mod_height * (*csp_0).mod_height;
        if width as ::core::ffi::c_double * width_units
            > height as ::core::ffi::c_double * height_units
        {
            let mut new_width: ::core::ffi::c_int = round(
                height as ::core::ffi::c_double * height_units
                    / (width_units * (*csp_0).mod_width as ::core::ffi::c_double),
            ) as ::core::ffi::c_int;
            new_width *= (*csp_0).mod_width;
            width = if new_width < width { new_width } else { width };
        } else {
            let mut new_height: ::core::ffi::c_int = round(
                width as ::core::ffi::c_double * width_units
                    / (height_units * (*csp_0).mod_height as ::core::ffi::c_double),
            ) as ::core::ffi::c_int;
            new_height *= (*csp_0).mod_height;
            height = if new_height < height { new_height } else { height };
        }
    } else if !str_width.is_null() || !str_height.is_null() {
        if width <= 0 as ::core::ffi::c_int || height <= 0 as ::core::ffi::c_int {
            x264_cli_log(
                b"resize\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"invalid resolution %sx%s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                x264_otos(
                    str_width,
                    b"<unset>\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                ),
                x264_otos(
                    str_height,
                    b"<unset>\0" as *const u8 as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                ),
            );
            return -(1 as ::core::ffi::c_int);
        }
        if str_sar.is_null() {
            let mut num: uint64_t = ((*info).width as uint64_t)
                .wrapping_mul(height as uint64_t);
            let mut den: uint64_t = ((*info).height as uint64_t)
                .wrapping_mul(width as uint64_t);
            x264_reduce_fraction64(&mut num, &mut den);
            out_sar_w = num.wrapping_mul(in_sar_w as uint64_t) as uint32_t;
            out_sar_h = den.wrapping_mul(in_sar_h as uint64_t) as uint32_t;
            x264_reduce_fraction(&mut out_sar_w, &mut out_sar_h);
        }
    } else if !str_sar.is_null() {
        let mut csp_1: *const x264_cli_csp_t = x264_cli_get_csp((*h).dst_csp);
        let mut width_units_0: ::core::ffi::c_double = in_sar_h as ::core::ffi::c_double
            * out_sar_w as ::core::ffi::c_double;
        let mut height_units_0: ::core::ffi::c_double = in_sar_w as ::core::ffi::c_double
            * out_sar_h as ::core::ffi::c_double;
        width = (*info).width;
        height = (*info).height;
        if width_units_0 > height_units_0 {
            width = round(
                (*info).width as ::core::ffi::c_double * height_units_0
                    / (width_units_0 * (*csp_1).mod_width as ::core::ffi::c_double),
            ) as ::core::ffi::c_int;
            width *= (*csp_1).mod_width;
        } else {
            height = round(
                (*info).height as ::core::ffi::c_double * width_units_0
                    / (height_units_0 * (*csp_1).mod_height as ::core::ffi::c_double),
            ) as ::core::ffi::c_int;
            height *= (*csp_1).mod_height;
        }
    } else {
        (*h).dst.width = (*info).width;
        (*h).dst.height = (*info).height;
        csp_only = 1 as ::core::ffi::c_int;
    }
    if csp_only == 0 {
        (*info).sar_width = out_sar_w;
        (*info).sar_height = out_sar_h;
        (*h).dst.width = width;
        (*h).dst.height = height;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn init_sws_context(mut h: *mut resizer_hnd_t) -> ::core::ffi::c_int {
    if !(*h).ctx.is_null() {
        sws_freeContext((*h).ctx as *mut SwsContext);
    }
    (*h).ctx = sws_alloc_context() as *mut SwsContext;
    if (*h).ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    av_opt_set_int(
        (*h).ctx as *mut ::core::ffi::c_void,
        b"sws_flags\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).ctx_flags as int64_t,
        0 as ::core::ffi::c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut ::core::ffi::c_void,
        b"dstw\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).dst.width as int64_t,
        0 as ::core::ffi::c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut ::core::ffi::c_void,
        b"dsth\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).dst.height as int64_t,
        0 as ::core::ffi::c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut ::core::ffi::c_void,
        b"dst_format\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).dst.pix_fmt as int64_t,
        0 as ::core::ffi::c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut ::core::ffi::c_void,
        b"dst_range\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).dst.range as int64_t,
        0 as ::core::ffi::c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut ::core::ffi::c_void,
        b"srcw\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).scale.width as int64_t,
        0 as ::core::ffi::c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut ::core::ffi::c_void,
        b"srch\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).scale.height as int64_t,
        0 as ::core::ffi::c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut ::core::ffi::c_void,
        b"src_format\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).scale.pix_fmt as int64_t,
        0 as ::core::ffi::c_int,
    );
    av_opt_set_int(
        (*h).ctx as *mut ::core::ffi::c_void,
        b"src_range\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).scale.range as int64_t,
        0 as ::core::ffi::c_int,
    );
    sws_setColorspaceDetails(
        (*h).ctx as *mut SwsContext,
        sws_getCoefficients(SWS_CS_DEFAULT) as *const ::core::ffi::c_int,
        (*h).scale.range,
        sws_getCoefficients(SWS_CS_DEFAULT) as *const ::core::ffi::c_int,
        (*h).dst.range,
        0 as ::core::ffi::c_int,
        (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int,
        (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int,
    );
    return (sws_init_context(
        (*h).ctx as *mut SwsContext,
        0 as *mut SwsFilter,
        0 as *mut SwsFilter,
    ) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[c2rust::src_loc = "396:1"]
unsafe extern "C" fn check_resizer(
    mut h: *mut resizer_hnd_t,
    mut in_0: *mut cli_pic_t,
) -> ::core::ffi::c_int {
    let mut input_prop: frame_prop_t = {
        let mut init = frame_prop_t {
            width: (*in_0).img.width,
            height: (*in_0).img.height,
            pix_fmt: convert_csp_to_pix_fmt((*in_0).img.csp),
            range: (*h).input_range,
        };
        init
    };
    if memcmp(
        &mut input_prop as *mut frame_prop_t as *const ::core::ffi::c_void,
        &mut (*h).scale as *mut frame_prop_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<frame_prop_t>() as size_t,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if !(*h).ctx.is_null() || (*h).working != 0 {
        x264_cli_log(
            NAME.as_ptr(),
            X264_LOG_WARNING,
            b"stream properties changed at pts %ld\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*in_0).pts,
        );
        (*h).fast_mono = 0 as ::core::ffi::c_int;
    }
    (*h).scale = input_prop;
    if (*h).buffer_allocated == 0 && (*h).fast_mono == 0 {
        if x264_cli_pic_alloc_aligned(
            &mut (*h).buffer,
            (*h).dst_csp,
            (*h).dst.width,
            (*h).dst.height,
        ) != 0
        {
            return -(1 as ::core::ffi::c_int);
        }
        (*h).buffer_allocated = 1 as ::core::ffi::c_int;
    }
    if init_sws_context(h) != 0 {
        x264_cli_log(
            b"resize\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"swscale init failed\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "418:1"]
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !opt_string.is_null()
        && strcmp(opt_string, b"normcsp\0" as *const u8 as *const ::core::ffi::c_char)
            == 0 && (*info).csp & X264_CSP_OTHER == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if opt_string.is_null() && full_check(info, param) == 0 {
        return 0 as ::core::ffi::c_int;
    }
    static mut optlist: [*const ::core::ffi::c_char; 7] = [
        b"width\0" as *const u8 as *const ::core::ffi::c_char,
        b"height\0" as *const u8 as *const ::core::ffi::c_char,
        b"sar\0" as *const u8 as *const ::core::ffi::c_char,
        b"fittobox\0" as *const u8 as *const ::core::ffi::c_char,
        b"csp\0" as *const u8 as *const ::core::ffi::c_char,
        b"method\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    let mut opts: *mut *mut ::core::ffi::c_char = x264_split_options(
        opt_string,
        optlist.as_ptr(),
    );
    if opts.is_null() && !opt_string.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut h: *mut resizer_hnd_t = calloc(
        1 as size_t,
        ::core::mem::size_of::<resizer_hnd_t>() as size_t,
    ) as *mut resizer_hnd_t;
    if h.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*h).ctx_flags = convert_method_to_flag(
        x264_otos(
            x264_get_option(optlist[5 as ::core::ffi::c_int as usize], opts),
            b"\0" as *const u8 as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ),
    );
    if !opts.is_null() {
        (*h).dst_csp = (*info).csp;
        (*h).dst.width = (*info).width;
        (*h).dst.height = (*info).height;
        (*h).dst.range = (*info).fullrange;
        if strcmp(opt_string, b"normcsp\0" as *const u8 as *const ::core::ffi::c_char)
            == 0
        {
            free(opts as *mut ::core::ffi::c_void);
            (*h).variable_input = 1 as ::core::ffi::c_int;
            (*h).dst_csp = pick_closest_supported_csp((*info).csp);
            if (*h).dst_csp == 0 as ::core::ffi::c_int {
                x264_cli_log(
                    b"resize\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"filter get invalid input pixel format %d (colorspace %d)\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    convert_csp_to_pix_fmt((*info).csp),
                    (*info).csp,
                );
                return -(1 as ::core::ffi::c_int);
            }
        } else {
            let mut err: ::core::ffi::c_int = handle_opts(
                optlist.as_ptr(),
                opts,
                info,
                h,
            );
            free(opts as *mut ::core::ffi::c_void);
            if err != 0 {
                return -(1 as ::core::ffi::c_int);
            }
        }
    } else {
        (*h).dst_csp = (*param).i_csp;
        (*h).dst.width = (*param).i_width;
        (*h).dst.height = (*param).i_height;
        (*h).dst.range = (*param).vui.b_fullrange;
    }
    if (*h).ctx_flags != SWS_FAST_BILINEAR as ::core::ffi::c_int as uint32_t {
        (*h).ctx_flags
            |= (SWS_FULL_CHR_H_INT as ::core::ffi::c_int
                | SWS_FULL_CHR_H_INP as ::core::ffi::c_int
                | SWS_ACCURATE_RND as ::core::ffi::c_int) as uint32_t;
    }
    (*h).dst.pix_fmt = convert_csp_to_pix_fmt((*h).dst_csp);
    (*h).scale = (*h).dst;
    (*h).input_range = (*info).fullrange;
    let mut src_csp: ::core::ffi::c_int = (*info).csp & (X264_CSP_MASK | X264_CSP_OTHER);
    let mut dst_csp: ::core::ffi::c_int = (*h).dst_csp
        & (X264_CSP_MASK | X264_CSP_OTHER);
    (*h).pre_swap_chroma = (src_csp == X264_CSP_YV12 || src_csp == X264_CSP_YV16
        || src_csp == X264_CSP_YV24) as ::core::ffi::c_int;
    (*h).post_swap_chroma = (dst_csp == X264_CSP_YV12 || dst_csp == X264_CSP_YV16
        || dst_csp == X264_CSP_YV24) as ::core::ffi::c_int;
    let mut src_pix_fmt: ::core::ffi::c_int = convert_csp_to_pix_fmt((*info).csp);
    let mut src_pix_fmt_inv: ::core::ffi::c_int = convert_csp_to_pix_fmt(
        (*info).csp ^ X264_CSP_HIGH_DEPTH,
    );
    let mut dst_pix_fmt_inv: ::core::ffi::c_int = convert_csp_to_pix_fmt(
        (*h).dst_csp ^ X264_CSP_HIGH_DEPTH,
    );
    if (*h).dst.width <= 0 as ::core::ffi::c_int
        || (*h).dst.height <= 0 as ::core::ffi::c_int
        || (*h).dst.width > 16384 as ::core::ffi::c_int
        || (*h).dst.height > 16384 as ::core::ffi::c_int
    {
        x264_cli_log(
            b"resize\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"invalid width x height (%dx%d)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*h).dst.width,
            (*h).dst.height,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if src_pix_fmt == AV_PIX_FMT_NONE as ::core::ffi::c_int
        && src_pix_fmt_inv != AV_PIX_FMT_NONE as ::core::ffi::c_int
    {
        x264_cli_log(
            b"resize\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"input colorspace %s with bit depth %d is not supported\n\0" as *const u8
                as *const ::core::ffi::c_char,
            av_get_pix_fmt_name(src_pix_fmt_inv as AVPixelFormat),
            if (*info).csp & 0x2000 as ::core::ffi::c_int != 0 {
                16 as ::core::ffi::c_int
            } else {
                8 as ::core::ffi::c_int
            },
        );
        return -(1 as ::core::ffi::c_int);
    }
    if sws_isSupportedInput(src_pix_fmt as AVPixelFormat) == 0 {
        x264_cli_log(
            b"resize\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"input colorspace %s is not supported\n\0" as *const u8
                as *const ::core::ffi::c_char,
            av_get_pix_fmt_name(src_pix_fmt as AVPixelFormat),
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).dst.pix_fmt == AV_PIX_FMT_NONE as ::core::ffi::c_int
        && dst_pix_fmt_inv != AV_PIX_FMT_NONE as ::core::ffi::c_int
    {
        x264_cli_log(
            b"resize\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"input colorspace %s with bit depth %d is not supported\n\0" as *const u8
                as *const ::core::ffi::c_char,
            av_get_pix_fmt_name(dst_pix_fmt_inv as AVPixelFormat),
            if (*h).dst_csp & 0x2000 as ::core::ffi::c_int != 0 {
                16 as ::core::ffi::c_int
            } else {
                8 as ::core::ffi::c_int
            },
        );
        return -(1 as ::core::ffi::c_int);
    }
    if sws_isSupportedOutput((*h).dst.pix_fmt as AVPixelFormat) == 0 {
        x264_cli_log(
            b"resize\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"output colorspace %s is not supported\n\0" as *const u8
                as *const ::core::ffi::c_char,
            av_get_pix_fmt_name((*h).dst.pix_fmt as AVPixelFormat),
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).dst.height != (*info).height && (*info).interlaced != 0 {
        x264_cli_log(
            b"resize\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"swscale is not compatible with interlaced vertical resizing\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    let mut csp: *const x264_cli_csp_t = x264_cli_get_csp((*h).dst_csp);
    if (*h).dst.width % (*csp).mod_width != 0 || (*h).dst.height % (*csp).mod_height != 0
    {
        x264_cli_log(
            b"resize\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"resolution %dx%d is not compliant with colorspace %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*h).dst.width,
            (*h).dst.height,
            (*csp).name,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).dst.width != (*info).width || (*h).dst.height != (*info).height {
        x264_cli_log(
            NAME.as_ptr(),
            X264_LOG_INFO,
            b"resizing to %dx%d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*h).dst.width,
            (*h).dst.height,
        );
    }
    if (*h).dst.pix_fmt != src_pix_fmt {
        x264_cli_log(
            NAME.as_ptr(),
            X264_LOG_WARNING,
            b"converting from %s to %s\n\0" as *const u8 as *const ::core::ffi::c_char,
            av_get_pix_fmt_name(src_pix_fmt as AVPixelFormat),
            av_get_pix_fmt_name((*h).dst.pix_fmt as AVPixelFormat),
        );
    } else if (*h).dst.range != (*h).input_range {
        x264_cli_log(
            NAME.as_ptr(),
            X264_LOG_WARNING,
            b"converting range from %s to %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            if (*h).input_range != 0 {
                b"PC\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"TV\0" as *const u8 as *const ::core::ffi::c_char
            },
            if (*h).dst.range != 0 {
                b"PC\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"TV\0" as *const u8 as *const ::core::ffi::c_char
            },
        );
    }
    (*h).dst_csp |= (*info).csp & X264_CSP_VFLIP;
    if dst_csp == X264_CSP_I400
        && (src_csp >= X264_CSP_I420 && src_csp <= X264_CSP_NV16
            || src_csp == X264_CSP_I444 || src_csp == X264_CSP_YV24)
        && (*h).dst.width == (*info).width && (*h).dst.height == (*info).height
        && (*h).dst.range == (*h).input_range
    {
        (*h).fast_mono = 1 as ::core::ffi::c_int;
    }
    if (*h).variable_input == 0 {
        let mut input_pic: cli_pic_t = {
            let mut init = cli_pic_t {
                img: {
                    let mut init = cli_image_t {
                        csp: (*info).csp,
                        width: (*info).width,
                        height: (*info).height,
                        planes: 0 as ::core::ffi::c_int,
                        plane: [0 as *mut uint8_t; 4],
                        stride: [0; 4],
                    };
                    init
                },
                pts: 0 as int64_t,
                duration: 0,
                opaque: 0 as *mut ::core::ffi::c_void,
            };
            init
        };
        if check_resizer(h, &mut input_pic) != 0 {
            return -(1 as ::core::ffi::c_int);
        }
    }
    (*info).csp = (*h).dst_csp;
    (*info).width = (*h).dst.width;
    (*info).height = (*h).dst.height;
    (*info).fullrange = (*h).dst.range;
    (*h).prev_filter = *filter;
    (*h).prev_hnd = *handle;
    *handle = h as hnd_t;
    *filter = resize_filter;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "543:1"]
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *mut resizer_hnd_t = handle as *mut resizer_hnd_t;
    if (*h)
        .prev_filter
        .get_frame
        .expect("non-null function pointer")((*h).prev_hnd, output, frame) != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).variable_input != 0 && check_resizer(h, output) != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    (*h).working = 1 as ::core::ffi::c_int;
    if (*h).pre_swap_chroma != 0 {
        let mut t: *mut uint8_t = (*output).img.plane[1 as ::core::ffi::c_int as usize];
        (*output).img.plane[1 as ::core::ffi::c_int as usize] = (*output)
            .img
            .plane[2 as ::core::ffi::c_int as usize];
        (*output).img.plane[2 as ::core::ffi::c_int as usize] = t;
    }
    if !(*h).ctx.is_null() && (*h).fast_mono == 0 {
        sws_scale(
            (*h).ctx as *mut SwsContext,
            (*output).img.plane.as_mut_ptr() as *const *const uint8_t,
            (*output).img.stride.as_mut_ptr() as *const ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (*output).img.height,
            (*h).buffer.img.plane.as_mut_ptr() as *const *mut uint8_t,
            (*h).buffer.img.stride.as_mut_ptr() as *const ::core::ffi::c_int,
        );
        (*output).img = (*h).buffer.img;
    } else {
        (*output).img.csp = (*h).dst_csp;
    }
    if (*h).post_swap_chroma != 0 {
        let mut t_0: *mut uint8_t = (*output)
            .img
            .plane[1 as ::core::ffi::c_int as usize];
        (*output).img.plane[1 as ::core::ffi::c_int as usize] = (*output)
            .img
            .plane[2 as ::core::ffi::c_int as usize];
        (*output).img.plane[2 as ::core::ffi::c_int as usize] = t_0;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "567:1"]
unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *mut resizer_hnd_t = handle as *mut resizer_hnd_t;
    return (*h)
        .prev_filter
        .release_frame
        .expect("non-null function pointer")((*h).prev_hnd, pic, frame);
}
#[c2rust::src_loc = "573:1"]
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut resizer_hnd_t = handle as *mut resizer_hnd_t;
    (*h).prev_filter.free.expect("non-null function pointer")((*h).prev_hnd);
    if !(*h).ctx.is_null() {
        sws_freeContext((*h).ctx as *mut SwsContext);
    }
    if (*h).buffer_allocated != 0 {
        x264_cli_pic_clean(&mut (*h).buffer);
    }
    free(h as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "612:18"]
pub static mut resize_filter: cli_vid_filter_t = unsafe {
    {
        let mut init = cli_vid_filter_t {
            name: NAME.as_ptr(),
            help: Some(help as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
            init: Some(
                init
                    as unsafe extern "C" fn(
                        *mut hnd_t,
                        *mut cli_vid_filter_t,
                        *mut video_info_t,
                        *mut x264_param_t,
                        *mut ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            ),
            get_frame: Some(
                get_frame
                    as unsafe extern "C" fn(
                        hnd_t,
                        *mut cli_pic_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            release_frame: Some(
                release_frame
                    as unsafe extern "C" fn(
                        hnd_t,
                        *mut cli_pic_t,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            free: Some(free_filter as unsafe extern "C" fn(hnd_t) -> ()),
            next: 0 as *const cli_vid_filter_t as *mut cli_vid_filter_t,
        };
        init
    }
};
