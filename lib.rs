#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![deny(unsafe_op_in_unsafe_fn)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(label_break_value)]
#![feature(extern_types)]
#![feature(raw_ref_op)]
#![feature(register_tool)]
#![register_tool(c2rust)]
pub mod __stdarg___gnuc_va_list_h {
    pub type __gnuc_va_list = crate::internal::__builtin_va_list;
}
pub mod __stddef_size_t_h {
    pub type size_t = usize;
}
pub mod config_h {
    pub const HAVE_GPL: ::core::ffi::c_int = 1i32;
    pub const HAVE_INTERLACED: ::core::ffi::c_int = 1i32;
}
pub mod slicetype_c {
    pub const PAD_SIZE: ::core::ffi::c_int = 32i32;
    pub const NUM_INTS: ::core::ffi::c_int = 4i32;
    pub const COST_EST: ::core::ffi::c_int = 0i32;
    pub const COST_EST_AQ: ::core::ffi::c_int = 1i32;
    pub const INTRA_MBS: ::core::ffi::c_int = 2i32;
    pub const NUM_ROWS: ::core::ffi::c_int = 3i32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct x264_slicetype_slice_t {
        pub h: *mut crate::src::common::common::x264_t,
        pub a: *mut crate::src::encoder::analyse::x264_mb_analysis_t,
        pub frames: *mut *mut crate::src::common::frame::x264_frame_t,
        pub p0: ::core::ffi::c_int,
        pub p1: ::core::ffi::c_int,
        pub b: ::core::ffi::c_int,
        pub dist_scale_factor: ::core::ffi::c_int,
        pub do_search: *mut ::core::ffi::c_int,
        pub w: *const crate::src::common::mc::x264_weight_t,
        pub output_inter: *mut ::core::ffi::c_int,
        pub output_intra: *mut ::core::ffi::c_int,
    }
    pub const MBTREE_PRECISION: ::core::ffi::c_float = 0.5;
}
pub mod rdo_c {
    #[no_mangle]
    pub static mut x264_8_cabac_transition_unary: [[crate::stdlib::uint8_t; 128]; 15] =
        [[0; 128]; 15];
    #[no_mangle]
    pub static mut x264_8_cabac_size_unary: [[crate::stdlib::uint16_t; 128]; 15] = [[0; 128]; 15];
    pub const TRELLIS_SCORE_MAX: ::core::ffi::c_ulonglong = !(0u64);
    pub const TRELLIS_SCORE_BIAS: ::core::ffi::c_ulonglong = (1u64) << 60i32;
    pub const CABAC_SIZE_BITS: ::core::ffi::c_int = 8i32;
    pub const LAMBDA_BITS: ::core::ffi::c_int = 4i32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct trellis_node_t {
        pub score: crate::stdlib::uint64_t,
        pub level_idx: ::core::ffi::c_int,
        pub cabac_state: [crate::stdlib::uint8_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct trellis_level_t {
        pub next: crate::stdlib::uint16_t,
        pub abs_level: crate::stdlib::uint16_t,
    }
}
pub mod __stddef_null_h {
    pub const NULL: *mut ::core::ffi::c_void =
        ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
}
pub mod osdep_h {
    pub const NATIVE_ALIGN: ::core::ffi::c_int = 64i32;
    pub const WORD_SIZE: ::core::ffi::c_int = 8i32;
}
pub mod x264_config_h {
    pub const X264_CHROMA_FORMAT: ::core::ffi::c_int = crate::x264_h::X264_CSP_I444;
    pub const X264_VERSION: [::core::ffi::c_char; 16] = unsafe {
        ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b" r3223M 0480cb0\0")
    };
}
pub mod limits_h {
    pub const INT_MAX: ::core::ffi::c_int = crate::internal::__INT_MAX__;
}
pub mod x264_h {
    pub const X264_BUILD: ::core::ffi::c_int = 165i32;
    pub type nal_unit_type_e = ::core::ffi::c_uint;
    pub const NAL_UNKNOWN: crate::x264_h::nal_unit_type_e = 0;
    pub const NAL_SLICE: crate::x264_h::nal_unit_type_e = 1;
    pub const NAL_SLICE_DPA: crate::x264_h::nal_unit_type_e = 2;
    pub const NAL_SLICE_DPB: crate::x264_h::nal_unit_type_e = 3;
    pub const NAL_SLICE_DPC: crate::x264_h::nal_unit_type_e = 4;
    pub const NAL_SLICE_IDR: crate::x264_h::nal_unit_type_e = 5;
    pub const NAL_SEI: crate::x264_h::nal_unit_type_e = 6;
    pub const NAL_SPS: crate::x264_h::nal_unit_type_e = 7;
    pub const NAL_PPS: crate::x264_h::nal_unit_type_e = 8;
    pub const NAL_AUD: crate::x264_h::nal_unit_type_e = 9;
    pub const NAL_FILLER: crate::x264_h::nal_unit_type_e = 12;
    pub type nal_priority_e = ::core::ffi::c_uint;
    pub const NAL_PRIORITY_DISPOSABLE: crate::x264_h::nal_priority_e = 0;
    pub const NAL_PRIORITY_LOW: crate::x264_h::nal_priority_e = 1;
    pub const NAL_PRIORITY_HIGH: crate::x264_h::nal_priority_e = 2;
    pub const NAL_PRIORITY_HIGHEST: crate::x264_h::nal_priority_e = 3;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct x264_nal_t {
        pub i_ref_idc: ::core::ffi::c_int,
        pub i_type: ::core::ffi::c_int,
        pub b_long_startcode: ::core::ffi::c_int,
        pub i_first_mb: ::core::ffi::c_int,
        pub i_last_mb: ::core::ffi::c_int,
        pub i_payload: ::core::ffi::c_int,
        pub p_payload: *mut crate::stdlib::uint8_t,
        pub i_padding: ::core::ffi::c_int,
    }
    pub const X264_CPU_MMX: ::core::ffi::c_uint = (1u32) << 0i32;
    pub const X264_CPU_MMX2: ::core::ffi::c_uint = (1u32) << 1i32;
    pub const X264_CPU_SSE: ::core::ffi::c_uint = (1u32) << 2i32;
    pub const X264_CPU_SSE2: ::core::ffi::c_uint = (1u32) << 3i32;
    pub const X264_CPU_LZCNT: ::core::ffi::c_uint = (1u32) << 4i32;
    pub const X264_CPU_SSE3: ::core::ffi::c_uint = (1u32) << 5i32;
    pub const X264_CPU_SSSE3: ::core::ffi::c_uint = (1u32) << 6i32;
    pub const X264_CPU_SSE4: ::core::ffi::c_uint = (1u32) << 7i32;
    pub const X264_CPU_SSE42: ::core::ffi::c_uint = (1u32) << 8i32;
    pub const X264_CPU_AVX: ::core::ffi::c_uint = (1u32) << 9i32;
    pub const X264_CPU_XOP: ::core::ffi::c_uint = (1u32) << 10i32;
    pub const X264_CPU_FMA4: ::core::ffi::c_uint = (1u32) << 11i32;
    pub const X264_CPU_FMA3: ::core::ffi::c_uint = (1u32) << 12i32;
    pub const X264_CPU_BMI1: ::core::ffi::c_uint = (1u32) << 13i32;
    pub const X264_CPU_BMI2: ::core::ffi::c_uint = (1u32) << 14i32;
    pub const X264_CPU_AVX2: ::core::ffi::c_uint = (1u32) << 15i32;
    pub const X264_CPU_AVX512: ::core::ffi::c_uint = (1u32) << 16i32;
    pub const X264_CPU_CACHELINE_32: ::core::ffi::c_uint = (1u32) << 17i32;
    pub const X264_CPU_CACHELINE_64: ::core::ffi::c_uint = (1u32) << 18i32;
    pub const X264_CPU_SSE2_IS_SLOW: ::core::ffi::c_uint = (1u32) << 19i32;
    pub const X264_CPU_SSE2_IS_FAST: ::core::ffi::c_uint = (1u32) << 20i32;
    pub const X264_CPU_SLOW_SHUFFLE: ::core::ffi::c_uint = (1u32) << 21i32;
    pub const X264_CPU_STACK_MOD4: ::core::ffi::c_uint = (1u32) << 22i32;
    pub const X264_CPU_SLOW_ATOM: ::core::ffi::c_uint = (1u32) << 23i32;
    pub const X264_CPU_SLOW_PSHUFB: ::core::ffi::c_uint = (1u32) << 24i32;
    pub const X264_CPU_SLOW_PALIGNR: ::core::ffi::c_uint = (1u32) << 25i32;
    pub const X264_ANALYSE_I4x4: ::core::ffi::c_uint = 0x1u32;
    pub const X264_ANALYSE_I8x8: ::core::ffi::c_uint = 0x2u32;
    pub const X264_ANALYSE_PSUB16x16: ::core::ffi::c_uint = 0x10u32;
    pub const X264_ANALYSE_PSUB8x8: ::core::ffi::c_uint = 0x20u32;
    pub const X264_ANALYSE_BSUB16x16: ::core::ffi::c_uint = 0x100u32;
    pub const X264_DIRECT_PRED_NONE: ::core::ffi::c_int = 0i32;
    pub const X264_DIRECT_PRED_SPATIAL: ::core::ffi::c_int = 1i32;
    pub const X264_DIRECT_PRED_AUTO: ::core::ffi::c_int = 3i32;
    pub const X264_ME_DIA: ::core::ffi::c_int = 0i32;
    pub const X264_ME_DIA_1: ::core::ffi::c_int = 0;
    pub const X264_ME_HEX: ::core::ffi::c_int = 1i32;
    pub const X264_ME_HEX_1: ::core::ffi::c_int = 1;
    pub const X264_ME_UMH: ::core::ffi::c_int = 2i32;
    pub const X264_ME_UMH_1: ::core::ffi::c_int = 2;
    pub const X264_ME_ESA: ::core::ffi::c_int = 3i32;
    pub const X264_ME_ESA_1: ::core::ffi::c_int = 3;
    pub const X264_ME_TESA: ::core::ffi::c_int = 4i32;
    pub const X264_CQM_FLAT: ::core::ffi::c_int = 0i32;
    pub const X264_CQM_JVT: ::core::ffi::c_int = 1i32;
    pub const X264_CQM_JVT_1: ::core::ffi::c_int = 1;
    pub const X264_CQM_CUSTOM: ::core::ffi::c_int = 2i32;
    pub const X264_CQM_CUSTOM_1: ::core::ffi::c_int = 2;
    pub const X264_RC_CQP: ::core::ffi::c_int = 0i32;
    pub const X264_RC_CRF: ::core::ffi::c_int = 1i32;
    pub const X264_RC_ABR: ::core::ffi::c_int = 2i32;
    pub const X264_QP_AUTO: ::core::ffi::c_int = 0i32;
    pub const X264_AQ_NONE: ::core::ffi::c_int = 0i32;
    pub const X264_AQ_VARIANCE: ::core::ffi::c_int = 1i32;
    pub const X264_AQ_AUTOVARIANCE: ::core::ffi::c_int = 2i32;
    pub const X264_AQ_AUTOVARIANCE_BIASED: ::core::ffi::c_int = 3i32;
    pub const X264_B_ADAPT_NONE: ::core::ffi::c_int = 0i32;
    pub const X264_B_ADAPT_FAST: ::core::ffi::c_int = 1i32;
    pub const X264_B_ADAPT_TRELLIS: ::core::ffi::c_int = 2i32;
    pub const X264_WEIGHTP_NONE: ::core::ffi::c_int = 0i32;
    pub const X264_WEIGHTP_SIMPLE: ::core::ffi::c_int = 1i32;
    pub const X264_WEIGHTP_SMART: ::core::ffi::c_int = 2i32;
    pub const X264_B_PYRAMID_NONE: ::core::ffi::c_int = 0i32;
    pub const X264_B_PYRAMID_STRICT: ::core::ffi::c_int = 1i32;
    pub const X264_B_PYRAMID_NORMAL: ::core::ffi::c_int = 2i32;
    pub const X264_KEYINT_MIN_AUTO: ::core::ffi::c_int = 0i32;
    pub const X264_KEYINT_MAX_INFINITE: ::core::ffi::c_int = (1i32) << 30i32;
    pub const X264_AVCINTRA_FLAVOR_PANASONIC: ::core::ffi::c_int = 0i32;
    pub const X264_AVCINTRA_FLAVOR_SONY: ::core::ffi::c_int = 1i32;
    pub const X264_CSP_MASK: ::core::ffi::c_int = 0xffi32;
    pub const X264_CSP_NONE: ::core::ffi::c_int = 0i32;
    pub const X264_CSP_I400: ::core::ffi::c_int = 0x1i32;
    pub const X264_CSP_I420: ::core::ffi::c_int = 0x2i32;
    pub const X264_CSP_YV12: ::core::ffi::c_int = 0x3i32;
    pub const X264_CSP_NV12: ::core::ffi::c_int = 0x4i32;
    pub const X264_CSP_NV21: ::core::ffi::c_int = 0x5i32;
    pub const X264_CSP_I422: ::core::ffi::c_int = 0x6i32;
    pub const X264_CSP_YV16: ::core::ffi::c_int = 0x7i32;
    pub const X264_CSP_NV16: ::core::ffi::c_int = 0x8i32;
    pub const X264_CSP_YUYV: ::core::ffi::c_int = 0x9i32;
    pub const X264_CSP_UYVY: ::core::ffi::c_int = 0xai32;
    pub const X264_CSP_V210: ::core::ffi::c_int = 0xbi32;
    pub const X264_CSP_I444: ::core::ffi::c_int = 0xci32;
    pub const X264_CSP_YV24: ::core::ffi::c_int = 0xdi32;
    pub const X264_CSP_BGR: ::core::ffi::c_int = 0xei32;
    pub const X264_CSP_BGRA: ::core::ffi::c_int = 0xfi32;
    pub const X264_CSP_RGB: ::core::ffi::c_int = 0x10i32;
    pub const X264_CSP_MAX: ::core::ffi::c_int = 0x11i32;
    pub const X264_CSP_VFLIP: ::core::ffi::c_int = 0x1000i32;
    pub const X264_CSP_HIGH_DEPTH: ::core::ffi::c_int = 0x2000i32;
    pub const X264_TYPE_AUTO: ::core::ffi::c_int = 0i32;
    pub const X264_TYPE_IDR: ::core::ffi::c_int = 0x1i32;
    pub const X264_TYPE_I: ::core::ffi::c_int = 0x2i32;
    pub const X264_TYPE_P: ::core::ffi::c_int = 0x3i32;
    pub const X264_TYPE_BREF: ::core::ffi::c_int = 0x4i32;
    pub const X264_TYPE_B: ::core::ffi::c_int = 0x5i32;
    pub const X264_TYPE_KEYFRAME: ::core::ffi::c_int = 0x6i32;
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0;
    pub const X264_LOG_ERROR_1: ::core::ffi::c_int = 0i32;
    pub const X264_LOG_WARNING: ::core::ffi::c_int = 1;
    pub const X264_LOG_WARNING_1: ::core::ffi::c_int = 1i32;
    pub const X264_LOG_INFO: ::core::ffi::c_int = 2i32;
    pub const X264_LOG_DEBUG: ::core::ffi::c_int = 3;
    pub const X264_LOG_DEBUG_1: ::core::ffi::c_int = 3i32;
    pub const X264_THREADS_AUTO: ::core::ffi::c_int = 0i32;
    pub const X264_SYNC_LOOKAHEAD_AUTO: ::core::ffi::c_int = -(1i32);
    pub const X264_NAL_HRD_NONE: ::core::ffi::c_int = 0i32;
    pub const X264_NAL_HRD_VBR: ::core::ffi::c_int = 1i32;
    pub const X264_NAL_HRD_CBR: ::core::ffi::c_int = 2i32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct x264_zone_t {
        pub i_start: ::core::ffi::c_int,
        pub i_end: ::core::ffi::c_int,
        pub b_force_qp: ::core::ffi::c_int,
        pub i_qp: ::core::ffi::c_int,
        pub f_bitrate_factor: ::core::ffi::c_float,
        pub param: *mut crate::x264_h::x264_param_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct x264_param_t {
        pub cpu: crate::stdlib::uint32_t,
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
        pub vui: crate::x264_h::C2Rust_Unnamed_0,
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
        pub cqm_4iy: [crate::stdlib::uint8_t; 16],
        pub cqm_4py: [crate::stdlib::uint8_t; 16],
        pub cqm_4ic: [crate::stdlib::uint8_t; 16],
        pub cqm_4pc: [crate::stdlib::uint8_t; 16],
        pub cqm_8iy: [crate::stdlib::uint8_t; 64],
        pub cqm_8py: [crate::stdlib::uint8_t; 64],
        pub cqm_8ic: [crate::stdlib::uint8_t; 64],
        pub cqm_8pc: [crate::stdlib::uint8_t; 64],
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
        pub analyse: crate::x264_h::C2Rust_Unnamed_1,
        pub rc: crate::x264_h::C2Rust_Unnamed_2,
        pub crop_rect: crate::x264_h::C2Rust_Unnamed_3,
        pub i_frame_packing: ::core::ffi::c_int,
        pub mastering_display: crate::x264_h::C2Rust_Unnamed_4,
        pub content_light_level: crate::x264_h::C2Rust_Unnamed_5,
        pub i_alternative_transfer: ::core::ffi::c_int,
        pub b_aud: ::core::ffi::c_int,
        pub b_repeat_headers: ::core::ffi::c_int,
        pub b_annexb: ::core::ffi::c_int,
        pub i_sps_id: ::core::ffi::c_int,
        pub b_vfr_input: ::core::ffi::c_int,
        pub b_pulldown: ::core::ffi::c_int,
        pub i_fps_num: crate::stdlib::uint32_t,
        pub i_fps_den: crate::stdlib::uint32_t,
        pub i_timebase_num: crate::stdlib::uint32_t,
        pub i_timebase_den: crate::stdlib::uint32_t,
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
                *mut crate::src::common::common::x264_t,
                *mut crate::x264_h::x264_nal_t,
                *mut ::core::ffi::c_void,
            ) -> (),
        >,
        pub opaque: *mut ::core::ffi::c_void,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_0 {
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
    pub struct C2Rust_Unnamed_1 {
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
    pub struct C2Rust_Unnamed_2 {
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
        pub zones: *mut crate::x264_h::x264_zone_t,
        pub i_zones: ::core::ffi::c_int,
        pub psz_zones: *mut ::core::ffi::c_char,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_3 {
        pub i_left: ::core::ffi::c_int,
        pub i_top: ::core::ffi::c_int,
        pub i_right: ::core::ffi::c_int,
        pub i_bottom: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_4 {
        pub b_mastering_display: ::core::ffi::c_int,
        pub i_green_x: ::core::ffi::c_int,
        pub i_green_y: ::core::ffi::c_int,
        pub i_blue_x: ::core::ffi::c_int,
        pub i_blue_y: ::core::ffi::c_int,
        pub i_red_x: ::core::ffi::c_int,
        pub i_red_y: ::core::ffi::c_int,
        pub i_white_x: ::core::ffi::c_int,
        pub i_white_y: ::core::ffi::c_int,
        pub i_display_max: crate::stdlib::int64_t,
        pub i_display_min: crate::stdlib::int64_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_5 {
        pub b_cll: ::core::ffi::c_int,
        pub i_max_cll: ::core::ffi::c_int,
        pub i_max_fall: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct x264_level_t {
        pub level_idc: crate::stdlib::uint8_t,
        pub mbps: crate::stdlib::int32_t,
        pub frame_size: crate::stdlib::int32_t,
        pub dpb: crate::stdlib::int32_t,
        pub bitrate: crate::stdlib::int32_t,
        pub cpb: crate::stdlib::int32_t,
        pub mv_range: crate::stdlib::uint16_t,
        pub mvs_per_2mb: crate::stdlib::uint8_t,
        pub slice_rate: crate::stdlib::uint8_t,
        pub mincr: crate::stdlib::uint8_t,
        pub bipred8x8: crate::stdlib::uint8_t,
        pub direct8x8: crate::stdlib::uint8_t,
        pub frame_only: crate::stdlib::uint8_t,
    }
    pub const X264_PARAM_BAD_NAME: ::core::ffi::c_int = -(1i32);
    pub const X264_PARAM_BAD_VALUE: ::core::ffi::c_int = -(2i32);
    pub const X264_PARAM_ALLOC_FAILED: ::core::ffi::c_int = -(3i32);
    pub type pic_struct_e = ::core::ffi::c_uint;
    pub const PIC_STRUCT_AUTO: crate::x264_h::pic_struct_e = 0;
    pub const PIC_STRUCT_PROGRESSIVE: crate::x264_h::pic_struct_e = 1;
    pub const PIC_STRUCT_TOP_BOTTOM: crate::x264_h::pic_struct_e = 4;
    pub const PIC_STRUCT_BOTTOM_TOP: crate::x264_h::pic_struct_e = 5;
    pub const PIC_STRUCT_TOP_BOTTOM_TOP: crate::x264_h::pic_struct_e = 6;
    pub const PIC_STRUCT_BOTTOM_TOP_BOTTOM: crate::x264_h::pic_struct_e = 7;
    pub const PIC_STRUCT_DOUBLE: crate::x264_h::pic_struct_e = 8;
    pub const PIC_STRUCT_TRIPLE: crate::x264_h::pic_struct_e = 9;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct x264_hrd_t {
        pub cpb_initial_arrival_time: ::core::ffi::c_double,
        pub cpb_final_arrival_time: ::core::ffi::c_double,
        pub cpb_removal_time: ::core::ffi::c_double,
        pub dpb_output_time: ::core::ffi::c_double,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct x264_sei_payload_t {
        pub payload_size: ::core::ffi::c_int,
        pub payload_type: ::core::ffi::c_int,
        pub payload: *mut crate::stdlib::uint8_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct x264_sei_t {
        pub num_payloads: ::core::ffi::c_int,
        pub payloads: *mut crate::x264_h::x264_sei_payload_t,
        pub sei_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct x264_image_t {
        pub i_csp: ::core::ffi::c_int,
        pub i_plane: ::core::ffi::c_int,
        pub i_stride: [::core::ffi::c_int; 4],
        pub plane: [*mut crate::stdlib::uint8_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct x264_image_properties_t {
        pub quant_offsets: *mut ::core::ffi::c_float,
        pub quant_offsets_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub mb_info: *mut crate::stdlib::uint8_t,
        pub mb_info_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        pub f_ssim: ::core::ffi::c_double,
        pub f_psnr_avg: ::core::ffi::c_double,
        pub f_psnr: [::core::ffi::c_double; 3],
        pub f_crf_avg: ::core::ffi::c_double,
    }
    pub const X264_MBINFO_CONSTANT: ::core::ffi::c_uint = (1u32) << 0i32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct x264_picture_t {
        pub i_type: ::core::ffi::c_int,
        pub i_qpplus1: ::core::ffi::c_int,
        pub i_pic_struct: ::core::ffi::c_int,
        pub b_keyframe: ::core::ffi::c_int,
        pub i_pts: crate::stdlib::int64_t,
        pub i_dts: crate::stdlib::int64_t,
        pub param: *mut crate::x264_h::x264_param_t,
        pub img: crate::x264_h::x264_image_t,
        pub prop: crate::x264_h::x264_image_properties_t,
        pub hrd_timing: crate::x264_h::x264_hrd_t,
        pub extra_sei: crate::x264_h::x264_sei_t,
        pub opaque: *mut ::core::ffi::c_void,
    }
}
pub mod internal {
    pub type __builtin_va_list = [crate::internal::__va_list_tag; 1];
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __va_list_tag {
        pub gp_offset: ::core::ffi::c_uint,
        pub fp_offset: ::core::ffi::c_uint,
        pub overflow_arg_area: *mut ::core::ffi::c_void,
        pub reg_save_area: *mut ::core::ffi::c_void,
    }
    pub const BIT_DEPTH: ::core::ffi::c_int = 8i32;
    pub const __INT_MAX__: ::core::ffi::c_int = 2147483647i32;
}
pub mod stdlib {
    extern "C" {
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
        pub fn __assert_single_arg(_: bool) -> bool;
        pub fn __sched_cpucount(
            __setsize: crate::__stddef_size_t_h::size_t,
            __setp: *const crate::stdlib::cpu_set_t,
        ) -> ::core::ffi::c_int;
        pub fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
        pub fn malloc(__size: crate::__stddef_size_t_h::size_t) -> *mut ::core::ffi::c_void;
        pub fn realloc(
            __ptr: *mut ::core::ffi::c_void,
            __size: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn free(__ptr: *mut ::core::ffi::c_void);
        pub fn memalign(
            __alignment: crate::__stddef_size_t_h::size_t,
            __size: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn exp(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        pub fn log(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        pub fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        pub fn log2f(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
        pub fn log2(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        pub fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double)
            -> ::core::ffi::c_double;
        pub fn powf(__x: ::core::ffi::c_float, __y: ::core::ffi::c_float) -> ::core::ffi::c_float;
        pub fn sqrtf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
        pub fn sqrt(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        pub fn ceil(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        pub fn fabsf(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
        pub fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        pub fn round(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
        pub fn madvise(
            __addr: *mut ::core::ffi::c_void,
            __len: crate::__stddef_size_t_h::size_t,
            __advice: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        pub fn pthread_create(
            __newthread: *mut crate::stdlib::pthread_t,
            __attr: *const crate::stdlib::pthread_attr_t,
            __start_routine: Option<
                unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
            >,
            __arg: *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
        pub fn pthread_join(
            __th: crate::stdlib::pthread_t,
            __thread_return: *mut *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int;
        pub fn pthread_mutex_init(
            __mutex: *mut crate::stdlib::pthread_mutex_t,
            __mutexattr: *const crate::stdlib::pthread_mutexattr_t,
        ) -> ::core::ffi::c_int;
        pub fn pthread_mutex_destroy(
            __mutex: *mut crate::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;
        pub fn pthread_mutex_lock(
            __mutex: *mut crate::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;
        pub fn pthread_mutex_unlock(
            __mutex: *mut crate::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;
        pub fn pthread_cond_init(
            __cond: *mut crate::stdlib::pthread_cond_t,
            __cond_attr: *const crate::stdlib::pthread_condattr_t,
        ) -> ::core::ffi::c_int;
        pub fn pthread_cond_destroy(
            __cond: *mut crate::stdlib::pthread_cond_t,
        ) -> ::core::ffi::c_int;
        pub fn pthread_cond_broadcast(
            __cond: *mut crate::stdlib::pthread_cond_t,
        ) -> ::core::ffi::c_int;
        pub fn pthread_cond_wait(
            __cond: *mut crate::stdlib::pthread_cond_t,
            __mutex: *mut crate::stdlib::pthread_mutex_t,
        ) -> ::core::ffi::c_int;
        pub fn sched_getaffinity(
            __pid: crate::stdlib::__pid_t,
            __cpusetsize: crate::__stddef_size_t_h::size_t,
            __cpuset: *mut crate::stdlib::cpu_set_t,
        ) -> ::core::ffi::c_int;
        pub fn fstat(
            __fd: ::core::ffi::c_int,
            __buf: *mut crate::stdlib::stat,
        ) -> ::core::ffi::c_int;
        pub static mut stderr: *mut crate::stdlib::FILE;
        pub fn rename(
            __old: *const ::core::ffi::c_char,
            __new: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        pub fn fclose(__stream: *mut crate::stdlib::FILE) -> ::core::ffi::c_int;
        pub fn fopen(
            __filename: *const ::core::ffi::c_char,
            __modes: *const ::core::ffi::c_char,
        ) -> *mut crate::stdlib::FILE;
        pub fn fprintf(
            __stream: *mut crate::stdlib::FILE,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn sprintf(
            __s: *mut ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn vfprintf(
            __s: *mut crate::stdlib::FILE,
            __format: *const ::core::ffi::c_char,
            __arg: ::core::ffi::VaList,
        ) -> ::core::ffi::c_int;
        pub fn snprintf(
            __s: *mut ::core::ffi::c_char,
            __maxlen: crate::__stddef_size_t_h::size_t,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn sscanf(
            __s: *const ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
        pub fn fread(
            __ptr: *mut ::core::ffi::c_void,
            __size: crate::__stddef_size_t_h::size_t,
            __n: crate::__stddef_size_t_h::size_t,
            __stream: *mut crate::stdlib::FILE,
        ) -> ::core::ffi::c_ulong;
        pub fn fwrite(
            __ptr: *const ::core::ffi::c_void,
            __size: crate::__stddef_size_t_h::size_t,
            __n: crate::__stddef_size_t_h::size_t,
            __s: *mut crate::stdlib::FILE,
        ) -> ::core::ffi::c_ulong;
        pub fn fseeko(
            __stream: *mut crate::stdlib::FILE,
            __off: crate::stdlib::__off64_t,
            __whence: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        pub fn ftello(__stream: *mut crate::stdlib::FILE) -> crate::stdlib::__off64_t;
        pub fn fileno(__stream: *mut crate::stdlib::FILE) -> ::core::ffi::c_int;
        pub fn strtod(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_double;
        pub fn strtol(
            __nptr: *const ::core::ffi::c_char,
            __endptr: *mut *mut ::core::ffi::c_char,
            __base: ::core::ffi::c_int,
        ) -> ::core::ffi::c_long;
        pub fn calloc(
            __nmemb: crate::__stddef_size_t_h::size_t,
            __size: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
        pub fn memcpy(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn memmove(
            __dest: *mut ::core::ffi::c_void,
            __src: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn memset(
            __s: *mut ::core::ffi::c_void,
            __c: ::core::ffi::c_int,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> *mut ::core::ffi::c_void;
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> ::core::ffi::c_int;
        pub fn strcpy(
            __dest: *mut ::core::ffi::c_char,
            __src: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        pub fn strcat(
            __dest: *mut ::core::ffi::c_char,
            __src: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        pub fn strcmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        pub fn strncmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> ::core::ffi::c_int;
        pub fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        pub fn strcspn(
            __s: *const ::core::ffi::c_char,
            __reject: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;
        pub fn strspn(
            __s: *const ::core::ffi::c_char,
            __accept: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;
        pub fn strpbrk(
            __s: *const ::core::ffi::c_char,
            __accept: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        pub fn strstr(
            __haystack: *const ::core::ffi::c_char,
            __needle: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        pub fn strtok_r(
            __s: *mut ::core::ffi::c_char,
            __delim: *const ::core::ffi::c_char,
            __save_ptr: *mut *mut ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        pub fn strlen(__s: *const ::core::ffi::c_char) -> crate::__stddef_size_t_h::size_t;
        pub fn strcasecmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_int;
        pub fn strncasecmp(
            __s1: *const ::core::ffi::c_char,
            __s2: *const ::core::ffi::c_char,
            __n: crate::__stddef_size_t_h::size_t,
        ) -> ::core::ffi::c_int;
        pub type _IO_marker;
        pub type _IO_codecvt;
        pub type _IO_wide_data;
        pub fn clock_gettime(
            __clock_id: crate::stdlib::clockid_t,
            __tp: *mut crate::stdlib::timespec,
        ) -> ::core::ffi::c_int;
    }
    pub type FILE = crate::stdlib::_IO_FILE;
    pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 65] = unsafe {
        ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
            *b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
        )
    };
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union __atomic_wide_counter {
        pub __value64: ::core::ffi::c_ulonglong,
        pub __value32: crate::stdlib::C2Rust_Unnamed_6,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct C2Rust_Unnamed_6 {
        pub __low: ::core::ffi::c_uint,
        pub __high: ::core::ffi::c_uint,
    }
    pub const __S_IFMT: ::core::ffi::c_int = 0o170000i32;
    pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1i32;
    pub type clockid_t = crate::stdlib::__clockid_t;
    pub type __cpu_mask = ::core::ffi::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct cpu_set_t {
        pub __bits: [crate::stdlib::__cpu_mask; 16],
    }
    pub type C2Rust_Unnamed_7 = ::core::ffi::c_uint;
    pub const _ISupper: crate::stdlib::C2Rust_Unnamed_7 = 256;
    pub const _ISlower: crate::stdlib::C2Rust_Unnamed_7 = 512;
    pub const _ISalpha: crate::stdlib::C2Rust_Unnamed_7 = 1024;
    pub const _ISdigit: crate::stdlib::C2Rust_Unnamed_7 = 2048;
    pub const _ISxdigit: crate::stdlib::C2Rust_Unnamed_7 = 4096;
    pub const _ISspace: crate::stdlib::C2Rust_Unnamed_7 = 8192;
    pub const _ISprint: crate::stdlib::C2Rust_Unnamed_7 = 16384;
    pub const _ISgraph: crate::stdlib::C2Rust_Unnamed_7 = 32768;
    pub const _ISblank: crate::stdlib::C2Rust_Unnamed_7 = 1;
    pub const _IScntrl: crate::stdlib::C2Rust_Unnamed_7 = 2;
    pub const _ISpunct: crate::stdlib::C2Rust_Unnamed_7 = 4;
    pub const _ISalnum: crate::stdlib::C2Rust_Unnamed_7 = 8;
    pub const MADV_HUGEPAGE: ::core::ffi::c_int = 14i32;
    pub type pthread_t = ::core::ffi::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union pthread_mutexattr_t {
        pub __size: [::core::ffi::c_char; 4],
        pub __align: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union pthread_condattr_t {
        pub __size: [::core::ffi::c_char; 4],
        pub __align: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union pthread_attr_t {
        pub __size: [::core::ffi::c_char; 56],
        pub __align: ::core::ffi::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union pthread_mutex_t {
        pub __data: crate::stdlib::__pthread_mutex_s,
        pub __size: [::core::ffi::c_char; 40],
        pub __align: ::core::ffi::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub union pthread_cond_t {
        pub __data: crate::stdlib::__pthread_cond_s,
        pub __size: [::core::ffi::c_char; 48],
        pub __align: ::core::ffi::c_longlong,
    }
    pub type intptr_t = isize;
    pub type uintptr_t = usize;
    pub const INT16_MIN: ::core::ffi::c_int = -(32767i32) - 1i32;
    pub const INT16_MAX: ::core::ffi::c_int = 32767i32;
    pub const INT32_MAX: ::core::ffi::c_int = 2147483647i32;
    pub const UINT16_MAX: ::core::ffi::c_int = 65535i32;
    pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295u32;
    pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615u64;
    pub type int8_t = crate::stdlib::__int8_t;
    pub type int16_t = crate::stdlib::__int16_t;
    pub type int32_t = crate::stdlib::__int32_t;
    pub type int64_t = crate::stdlib::__int64_t;
    pub type uint8_t = crate::stdlib::__uint8_t;
    pub type uint16_t = crate::stdlib::__uint16_t;
    pub type uint32_t = crate::stdlib::__uint32_t;
    pub type uint64_t = crate::stdlib::__uint64_t;
    pub type va_list = crate::__stdarg___gnuc_va_list_h::__gnuc_va_list;
    pub const SEEK_SET: ::core::ffi::c_int = 0i32;
    pub const SEEK_END: ::core::ffi::c_int = 2i32;
    pub type _IO_lock_t = ();
    #[derive(Copy, Clone, BitfieldStruct)]
    #[repr(C)]
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
        pub _markers: *mut crate::stdlib::_IO_marker,
        pub _chain: *mut crate::stdlib::_IO_FILE,
        pub _fileno: ::core::ffi::c_int,
        #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
        pub _flags2: [u8; 3],
        pub _short_backupbuf: [::core::ffi::c_char; 1],
        pub _old_offset: crate::stdlib::__off_t,
        pub _cur_column: ::core::ffi::c_ushort,
        pub _vtable_offset: ::core::ffi::c_schar,
        pub _shortbuf: [::core::ffi::c_char; 1],
        pub _lock: *mut ::core::ffi::c_void,
        pub _offset: crate::stdlib::__off64_t,
        pub _codecvt: *mut crate::stdlib::_IO_codecvt,
        pub _wide_data: *mut crate::stdlib::_IO_wide_data,
        pub _freeres_list: *mut crate::stdlib::_IO_FILE,
        pub _freeres_buf: *mut ::core::ffi::c_void,
        pub _prevchain: *mut *mut crate::stdlib::_IO_FILE,
        pub _mode: ::core::ffi::c_int,
        pub _unused3: ::core::ffi::c_int,
        pub _total_written: crate::stdlib::__uint64_t,
        pub _unused2: [::core::ffi::c_char; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __pthread_mutex_s {
        pub __lock: ::core::ffi::c_int,
        pub __count: ::core::ffi::c_uint,
        pub __owner: ::core::ffi::c_int,
        pub __nusers: ::core::ffi::c_uint,
        pub __kind: ::core::ffi::c_int,
        pub __spins: ::core::ffi::c_short,
        pub __unused: ::core::ffi::c_short,
        pub __list: crate::stdlib::__pthread_list_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct stat {
        pub st_dev: crate::stdlib::__dev_t,
        pub st_ino: crate::stdlib::__ino_t,
        pub st_nlink: crate::stdlib::__nlink_t,
        pub st_mode: crate::stdlib::__mode_t,
        pub st_uid: crate::stdlib::__uid_t,
        pub st_gid: crate::stdlib::__gid_t,
        pub __pad0: ::core::ffi::c_int,
        pub st_rdev: crate::stdlib::__dev_t,
        pub st_size: crate::stdlib::__off_t,
        pub st_blksize: crate::stdlib::__blksize_t,
        pub st_blocks: crate::stdlib::__blkcnt_t,
        pub st_atim: crate::stdlib::timespec,
        pub st_mtim: crate::stdlib::timespec,
        pub st_ctim: crate::stdlib::timespec,
        pub __glibc_reserved: [crate::stdlib::__syscall_slong_t; 3],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct timespec {
        pub tv_sec: crate::stdlib::__time_t,
        pub tv_nsec: crate::stdlib::__syscall_slong_t,
    }
    pub type __pthread_list_t = crate::stdlib::__pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __pthread_internal_list {
        pub __prev: *mut crate::stdlib::__pthread_internal_list,
        pub __next: *mut crate::stdlib::__pthread_internal_list,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    pub struct __pthread_cond_s {
        pub __wseq: crate::stdlib::__atomic_wide_counter,
        pub __g1_start: crate::stdlib::__atomic_wide_counter,
        pub __g_size: [::core::ffi::c_uint; 2],
        pub __g1_orig_size: ::core::ffi::c_uint,
        pub __wrefs: ::core::ffi::c_uint,
        pub __g_signals: [::core::ffi::c_uint; 2],
        pub __unused_initialized_1: ::core::ffi::c_uint,
        pub __unused_initialized_2: ::core::ffi::c_uint,
    }
    pub type __int8_t = i8;
    pub type __uint8_t = u8;
    pub type __int16_t = i16;
    pub type __uint16_t = u16;
    pub type __int32_t = i32;
    pub type __uint32_t = u32;
    pub type __int64_t = i64;
    pub type __uint64_t = u64;
    pub type __dev_t = ::core::ffi::c_ulong;
    pub type __uid_t = ::core::ffi::c_uint;
    pub type __gid_t = ::core::ffi::c_uint;
    pub type __ino_t = ::core::ffi::c_ulong;
    pub type __mode_t = ::core::ffi::c_uint;
    pub type __nlink_t = ::core::ffi::c_ulong;
    pub type __off_t = ::core::ffi::c_long;
    pub type __off64_t = ::core::ffi::c_long;
    pub type __pid_t = ::core::ffi::c_int;
    pub type __time_t = ::core::ffi::c_long;
    pub type __clockid_t = ::core::ffi::c_int;
    pub type __blksize_t = ::core::ffi::c_long;
    pub type __blkcnt_t = ::core::ffi::c_long;
    pub type __syscall_slong_t = ::core::ffi::c_long;
}
#[macro_use]
extern crate c2rust_bitfields;
pub mod src {
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
} // mod src
