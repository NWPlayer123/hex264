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
#[c2rust::header_src = "/usr/include/stdint.h:27"]
pub mod stdint_h {
    #[c2rust::src_loc = "76:1"]
    pub type intptr_t = isize;
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
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "291:9"]
    pub const X264_LOG_INFO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::internal::__va_list_tag;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "80:16"]
        pub type x264_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264cli.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/input/input.h:27"]
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
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "129:1"]
        pub fn x264_cli_csp_is_invalid(csp: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "130:1"]
        pub fn x264_cli_csp_depth_factor(csp: ::core::ffi::c_int) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "137:1"]
        pub fn x264_cli_get_csp(csp: ::core::ffi::c_int) -> *const x264_cli_csp_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/filters/video/video.h:27"]
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
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "366:1"]
        pub fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/filters/filters.h:27"]
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
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:27"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::filters_h::{x264_get_option, x264_otoi, x264_split_options};
pub use self::input_h::{
    cli_image_t, cli_pic_t, video_info_t, x264_cli_csp_depth_factor, x264_cli_csp_is_invalid,
    x264_cli_csp_t, x264_cli_get_csp,
};
pub use self::internal::__va_list_tag;
pub use self::stdint_h::intptr_t;
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::stdio_h::printf;
use self::stdlib_h::{calloc, free};
pub use self::types_h::{__int64_t, __uint32_t, __uint8_t};
pub use self::video_h::cli_vid_filter_t;
pub use self::x264_h::{
    x264_nal_t, x264_param_t, x264_t, x264_zone_t, C2RustUnnamed, C2RustUnnamed_0, C2RustUnnamed_1,
    C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4, X264_LOG_ERROR, X264_LOG_INFO,
};
pub use self::x264cli_h::{hnd_t, x264_cli_log};
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "34:9"]
pub struct crop_hnd_t {
    pub prev_hnd: hnd_t,
    pub prev_filter: cli_vid_filter_t,
    pub dims: [::core::ffi::c_int; 4],
    pub csp: *const x264_cli_csp_t,
}
#[c2rust::src_loc = "29:9"]
pub const NAME: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"crop\0") };
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn help(mut longhelp: ::core::ffi::c_int) {
    printf(b"      crop:left,top,right,bottom\n\0" as *const u8 as *const ::core::ffi::c_char);
    if longhelp == 0 {
        return;
    }
    printf(
        b"            removes pixels from the edges of the frame\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
}
#[c2rust::src_loc = "51:1"]
unsafe extern "C" fn handle_opts(
    mut h: *mut crop_hnd_t,
    mut info: *mut video_info_t,
    mut opts: *mut *mut ::core::ffi::c_char,
    mut optlist: *const *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let mut opt: *mut ::core::ffi::c_char = x264_get_option(*optlist.offset(i as isize), opts);
        if opt.is_null() {
            x264_cli_log(
                b"crop\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"%s crop value not specified\n\0" as *const u8 as *const ::core::ffi::c_char,
                *optlist.offset(i as isize),
            );
            return -(1 as ::core::ffi::c_int);
        }
        (*h).dims[i as usize] = x264_otoi(opt, -(1 as ::core::ffi::c_int));
        if (*h).dims[i as usize] < 0 as ::core::ffi::c_int {
            x264_cli_log(
                b"crop\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"%s crop value `%s' is less than 0\n\0" as *const u8 as *const ::core::ffi::c_char,
                *optlist.offset(i as isize),
                opt,
            );
            return -(1 as ::core::ffi::c_int);
        }
        let mut dim_mod: ::core::ffi::c_int = if i & 1 as ::core::ffi::c_int != 0 {
            (*(*h).csp).mod_height << (*info).interlaced
        } else {
            (*(*h).csp).mod_width
        };
        if (*h).dims[i as usize] % dim_mod != 0 {
            x264_cli_log(
                b"crop\0" as *const u8 as *const ::core::ffi::c_char,
                X264_LOG_ERROR,
                b"%s crop value `%s' is not a multiple of %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                *optlist.offset(i as isize),
                opt,
                dim_mod,
            );
            return -(1 as ::core::ffi::c_int);
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "65:1"]
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if x264_cli_csp_is_invalid((*info).csp) != 0 {
        x264_cli_log(
            b"crop\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"invalid csp %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*info).csp,
        );
        return -(1 as ::core::ffi::c_int);
    }
    let mut h: *mut crop_hnd_t =
        calloc(1 as size_t, ::core::mem::size_of::<crop_hnd_t>() as size_t) as *mut crop_hnd_t;
    if h.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*h).csp = x264_cli_get_csp((*info).csp);
    static mut optlist: [*const ::core::ffi::c_char; 5] = [
        b"left\0" as *const u8 as *const ::core::ffi::c_char,
        b"top\0" as *const u8 as *const ::core::ffi::c_char,
        b"right\0" as *const u8 as *const ::core::ffi::c_char,
        b"bottom\0" as *const u8 as *const ::core::ffi::c_char,
        0 as *const ::core::ffi::c_char,
    ];
    let mut opts: *mut *mut ::core::ffi::c_char = x264_split_options(opt_string, optlist.as_ptr());
    if opts.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut err: ::core::ffi::c_int = handle_opts(h, info, opts, optlist.as_ptr());
    free(opts as *mut ::core::ffi::c_void);
    if err != 0 {
        return -(1 as ::core::ffi::c_int);
    }
    (*h).dims[2 as ::core::ffi::c_int as usize] = (*info).width
        - (*h).dims[0 as ::core::ffi::c_int as usize]
        - (*h).dims[2 as ::core::ffi::c_int as usize];
    (*h).dims[3 as ::core::ffi::c_int as usize] = (*info).height
        - (*h).dims[1 as ::core::ffi::c_int as usize]
        - (*h).dims[3 as ::core::ffi::c_int as usize];
    if (*h).dims[2 as ::core::ffi::c_int as usize] <= 0 as ::core::ffi::c_int
        || (*h).dims[3 as ::core::ffi::c_int as usize] <= 0 as ::core::ffi::c_int
    {
        x264_cli_log(
            b"crop\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"invalid output resolution %dx%d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*h).dims[2 as ::core::ffi::c_int as usize],
            (*h).dims[3 as ::core::ffi::c_int as usize],
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*info).width != (*h).dims[2 as ::core::ffi::c_int as usize]
        || (*info).height != (*h).dims[3 as ::core::ffi::c_int as usize]
    {
        x264_cli_log(
            NAME.as_ptr(),
            X264_LOG_INFO,
            b"cropping to %dx%d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*h).dims[2 as ::core::ffi::c_int as usize],
            (*h).dims[3 as ::core::ffi::c_int as usize],
        );
    } else {
        free(h as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    }
    (*info).width = (*h).dims[2 as ::core::ffi::c_int as usize];
    (*info).height = (*h).dims[3 as ::core::ffi::c_int as usize];
    (*h).prev_filter = *filter;
    (*h).prev_hnd = *handle;
    *handle = h as hnd_t;
    *filter = crop_filter;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *mut crop_hnd_t = handle as *mut crop_hnd_t;
    if (*h)
        .prev_filter
        .get_frame
        .expect("non-null function pointer")((*h).prev_hnd, output, frame)
        != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    (*output).img.width = (*h).dims[2 as ::core::ffi::c_int as usize];
    (*output).img.height = (*h).dims[3 as ::core::ffi::c_int as usize];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*output).img.planes {
        let mut offset: intptr_t = (((*output).img.stride[i as usize]
            * (*h).dims[1 as ::core::ffi::c_int as usize])
            as ::core::ffi::c_float
            * (*(*h).csp).height[i as usize]) as intptr_t;
        offset = (offset as ::core::ffi::c_float
            + (*h).dims[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_float
                * (*(*h).csp).width[i as usize]
                * x264_cli_csp_depth_factor((*output).img.csp) as ::core::ffi::c_float)
            as intptr_t;
        (*output).img.plane[i as usize] = (*output).img.plane[i as usize].offset(offset as isize);
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "124:1"]
unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *mut crop_hnd_t = handle as *mut crop_hnd_t;
    return (*h)
        .prev_filter
        .release_frame
        .expect("non-null function pointer")((*h).prev_hnd, pic, frame);
}
#[c2rust::src_loc = "132:1"]
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut crop_hnd_t = handle as *mut crop_hnd_t;
    (*h).prev_filter.free.expect("non-null function pointer")((*h).prev_hnd);
    free(h as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "139:18"]
pub static mut crop_filter: cli_vid_filter_t = unsafe {
    {
        let mut init = cli_vid_filter_t {
            name: NAME.as_ptr(),
            help: Some(help as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
            init: Some(
                init as unsafe extern "C" fn(
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
