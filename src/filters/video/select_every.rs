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
#[c2rust::header_src = "/usr/include/stdint.h:26"]
pub mod stdint_h {
    #[c2rust::src_loc = "76:1"]
    pub type intptr_t = isize;
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
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    use super::stdint_intn_h::int64_t;
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
    extern "C" {
        #[c2rust::src_loc = "60:1"]
        pub fn x264_init_vid_filter(
            name: *const ::core::ffi::c_char,
            handle: *mut hnd_t,
            filter: *mut cli_vid_filter_t,
            info: *mut video_info_t,
            param: *mut x264_param_t,
            opt_string: *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:26"]
pub mod stdlib_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
        #[c2rust::src_loc = "672:1"]
        pub fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
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
        #[c2rust::src_loc = "366:1"]
        pub fn strtok_r(
            __s: *mut ::core::ffi::c_char,
            __delim: *const ::core::ffi::c_char,
            __save_ptr: *mut *mut ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:26"]
pub mod base_h {
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "271:10"]
        pub fn x264_reduce_fraction(n: *mut uint32_t, d: *mut uint32_t);
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:26"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "366:1"]
        pub fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "368:1"]
        pub fn sprintf(
            __s: *mut ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/filters/filters.h:26"]
pub mod filters_h {
    extern "C" {
        #[c2rust::src_loc = "37:1"]
        pub fn x264_otoi(
            str: *const ::core::ffi::c_char,
            def: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:26"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
pub use self::internal::__va_list_tag;
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{__uint8_t, __uint32_t, __int64_t, __uint64_t};
pub use self::stdint_intn_h::int64_t;
pub use self::stdint_uintn_h::{uint8_t, uint32_t, uint64_t};
pub use self::stdint_h::intptr_t;
pub use self::x264_h::{
    x264_nal_t, x264_zone_t, x264_param_t, C2RustUnnamed, C2RustUnnamed_0,
    C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4, X264_LOG_ERROR,
    x264_t,
};
pub use self::x264cli_h::{hnd_t, x264_cli_log};
pub use self::input_h::{video_info_t, cli_image_t, cli_pic_t};
pub use self::video_h::{cli_vid_filter_t, x264_init_vid_filter};
use self::stdlib_h::{malloc, free};
use self::string_h::{memcpy, strtok_r};
use self::base_h::x264_reduce_fraction;
use self::stdio_h::{printf, sprintf};
use self::filters_h::x264_otoi;
pub use self::__stddef_null_h::NULL;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "33:9"]
pub struct selvry_hnd_t {
    pub prev_hnd: hnd_t,
    pub prev_filter: cli_vid_filter_t,
    pub pattern: *mut ::core::ffi::c_int,
    pub pattern_len: ::core::ffi::c_int,
    pub step_size: ::core::ffi::c_int,
    pub vfr: ::core::ffi::c_int,
    pub pts: int64_t,
}
#[c2rust::src_loc = "28:9"]
pub const NAME: [::core::ffi::c_char; 13] = unsafe {
    ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"select_every\0")
};
#[c2rust::src_loc = "47:1"]
unsafe extern "C" fn help(mut longhelp: ::core::ffi::c_int) {
    printf(
        b"      select_every:step,offset1[,...]\n\0" as *const u8
            as *const ::core::ffi::c_char,
    );
    if longhelp == 0 {
        return;
    }
    printf(
        b"            apply a selection pattern to input frames\n            step: the number of frames in the pattern\n            offsets: the offset into the step to select a frame\n            see: http://avisynth.nl/index.php/Select#SelectEvery\n\0"
            as *const u8 as *const ::core::ffi::c_char,
    );
}
#[c2rust::src_loc = "58:1"]
unsafe extern "C" fn init(
    mut handle: *mut hnd_t,
    mut filter: *mut cli_vid_filter_t,
    mut info: *mut video_info_t,
    mut param: *mut x264_param_t,
    mut opt_string: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut h: *mut selvry_hnd_t = malloc(
        ::core::mem::size_of::<selvry_hnd_t>() as size_t,
    ) as *mut selvry_hnd_t;
    if h.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    (*h).pattern_len = 0 as ::core::ffi::c_int;
    (*h).step_size = 0 as ::core::ffi::c_int;
    let mut offsets: [::core::ffi::c_int; 100] = [0; 100];
    let mut tok: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char = opt_string;
    let mut saveptr: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    loop {
        tok = strtok_r(
            p,
            b",\0" as *const u8 as *const ::core::ffi::c_char,
            &mut saveptr,
        );
        if tok.is_null() {
            break;
        }
        let mut val: ::core::ffi::c_int = x264_otoi(tok, -(1 as ::core::ffi::c_int));
        if !p.is_null() {
            if val <= 0 as ::core::ffi::c_int {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"invalid step `%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                    tok,
                );
                return -(1 as ::core::ffi::c_int);
            }
            (*h).step_size = val;
        } else {
            if val < 0 as ::core::ffi::c_int || val >= (*h).step_size {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"invalid offset `%s'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    tok,
                );
                return -(1 as ::core::ffi::c_int);
            }
            if (*h).pattern_len >= 100 as ::core::ffi::c_int {
                x264_cli_log(
                    b"select_every\0" as *const u8 as *const ::core::ffi::c_char,
                    X264_LOG_ERROR,
                    b"max pattern size %d reached\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    100 as ::core::ffi::c_int,
                );
                return -(1 as ::core::ffi::c_int);
            }
            let fresh0 = (*h).pattern_len;
            (*h).pattern_len = (*h).pattern_len + 1;
            offsets[fresh0 as usize] = val;
        }
        p = 0 as *mut ::core::ffi::c_char;
    }
    if (*h).step_size == 0 {
        x264_cli_log(
            b"select_every\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"no step size provided\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).pattern_len == 0 {
        x264_cli_log(
            b"select_every\0" as *const u8 as *const ::core::ffi::c_char,
            X264_LOG_ERROR,
            b"no offsets supplied\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*h).pattern = malloc(
        ((*h).pattern_len as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    ) as *mut ::core::ffi::c_int;
    if (*h).pattern.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    memcpy(
        (*h).pattern as *mut ::core::ffi::c_void,
        offsets.as_mut_ptr() as *const ::core::ffi::c_void,
        ((*h).pattern_len as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t),
    );
    let mut max_rewind: intptr_t = 0 as intptr_t;
    let mut min: ::core::ffi::c_int = (*h).step_size;
    let mut i: ::core::ffi::c_int = (*h).pattern_len - 1 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        min = if min < offsets[i as usize] { min } else { offsets[i as usize] };
        if i != 0 {
            max_rewind = if max_rewind
                > (offsets[(i - 1 as ::core::ffi::c_int) as usize] - min
                    + 1 as ::core::ffi::c_int) as intptr_t
            {
                max_rewind
            } else {
                (offsets[(i - 1 as ::core::ffi::c_int) as usize] - min
                    + 1 as ::core::ffi::c_int) as intptr_t
            };
        }
        if max_rewind == (*h).step_size as intptr_t {
            break;
        }
        i -= 1;
    }
    let mut name: [::core::ffi::c_char; 20] = [0; 20];
    sprintf(
        name.as_mut_ptr(),
        b"cache_%d\0" as *const u8 as *const ::core::ffi::c_char,
        (*param).i_bitdepth,
    );
    if x264_init_vid_filter(
        name.as_mut_ptr(),
        handle,
        filter,
        info,
        param,
        max_rewind as *mut ::core::ffi::c_void as *mut ::core::ffi::c_char,
    ) != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).step_size != (*h).pattern_len {
        (*info).num_frames = ((*info).num_frames as uint64_t)
            .wrapping_mul((*h).pattern_len as uint64_t)
            .wrapping_div((*h).step_size as uint64_t) as ::core::ffi::c_int;
        (*info).fps_den = (*info).fps_den.wrapping_mul((*h).step_size as uint32_t);
        (*info).fps_num = (*info).fps_num.wrapping_mul((*h).pattern_len as uint32_t);
        x264_reduce_fraction(&mut (*info).fps_num, &mut (*info).fps_den);
        if (*info).vfr != 0 {
            (*info).timebase_den = (*info)
                .timebase_den
                .wrapping_mul((*h).pattern_len as uint32_t);
            (*info).timebase_num = (*info)
                .timebase_num
                .wrapping_mul((*h).step_size as uint32_t);
            x264_reduce_fraction(&mut (*info).timebase_num, &mut (*info).timebase_den);
        }
    }
    (*h).pts = 0 as int64_t;
    (*h).vfr = (*info).vfr;
    (*h).prev_filter = *filter;
    (*h).prev_hnd = *handle;
    *filter = select_every_filter;
    *handle = h as hnd_t;
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "129:1"]
unsafe extern "C" fn get_frame(
    mut handle: hnd_t,
    mut output: *mut cli_pic_t,
    mut frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    let mut pat_frame: ::core::ffi::c_int = *(*h)
        .pattern
        .offset((frame % (*h).pattern_len) as isize)
        + frame / (*h).pattern_len * (*h).step_size;
    if (*h)
        .prev_filter
        .get_frame
        .expect("non-null function pointer")((*h).prev_hnd, output, pat_frame) != 0
    {
        return -(1 as ::core::ffi::c_int);
    }
    if (*h).vfr != 0 {
        (*output).pts = (*h).pts;
        (*h).pts += (*output).duration;
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "143:1"]
unsafe extern "C" fn release_frame(
    mut handle: hnd_t,
    mut pic: *mut cli_pic_t,
    mut frame: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    let mut pat_frame: ::core::ffi::c_int = *(*h)
        .pattern
        .offset((frame % (*h).pattern_len) as isize)
        + frame / (*h).pattern_len * (*h).step_size;
    return (*h)
        .prev_filter
        .release_frame
        .expect("non-null function pointer")((*h).prev_hnd, pic, pat_frame);
}
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn free_filter(mut handle: hnd_t) {
    let mut h: *mut selvry_hnd_t = handle as *mut selvry_hnd_t;
    (*h).prev_filter.free.expect("non-null function pointer")((*h).prev_hnd);
    free((*h).pattern as *mut ::core::ffi::c_void);
    free(h as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "158:18"]
pub static mut select_every_filter: cli_vid_filter_t = unsafe {
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
