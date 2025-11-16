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
    #[c2rust::src_loc = "3:9"]
    pub const BIT_DEPTH: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:28"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:28"]
pub mod types_h {
    #[c2rust::src_loc = "37:1"]
    pub type __int8_t = i8;
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
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:28"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "24:1"]
    pub type int8_t = __int8_t;
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int8_t, __int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:28"]
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
#[c2rust::header_src = "/usr/include/stdint.h:28"]
pub mod stdint_h {
    #[c2rust::src_loc = "76:1"]
    pub type intptr_t = isize;
    #[c2rust::src_loc = "79:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/atomic_wide_counter.h:28"]
pub mod atomic_wide_counter_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "25:9"]
    pub union __atomic_wide_counter {
        pub __value64: ::core::ffi::c_ulonglong,
        pub __value32: C2RustUnnamed,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "28:3"]
    pub struct C2RustUnnamed {
        pub __low: ::core::ffi::c_uint,
        pub __high: ::core::ffi::c_uint,
    }
}
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:28"]
pub mod thread_shared_types_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "51:16"]
    pub struct __pthread_internal_list {
        pub __prev: *mut __pthread_internal_list,
        pub __next: *mut __pthread_internal_list,
    }
    #[c2rust::src_loc = "51:1"]
    pub type __pthread_list_t = __pthread_internal_list;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "94:8"]
    pub struct __pthread_cond_s {
        pub __wseq: __atomic_wide_counter,
        pub __g1_start: __atomic_wide_counter,
        pub __g_size: [::core::ffi::c_uint; 2],
        pub __g1_orig_size: ::core::ffi::c_uint,
        pub __wrefs: ::core::ffi::c_uint,
        pub __g_signals: [::core::ffi::c_uint; 2],
        pub __unused_initialized_1: ::core::ffi::c_uint,
        pub __unused_initialized_2: ::core::ffi::c_uint,
    }
    use super::atomic_wide_counter_h::__atomic_wide_counter;
}
#[c2rust::header_src = "/usr/include/bits/struct_mutex.h:28"]
pub mod struct_mutex_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "22:8"]
    pub struct __pthread_mutex_s {
        pub __lock: ::core::ffi::c_int,
        pub __count: ::core::ffi::c_uint,
        pub __owner: ::core::ffi::c_int,
        pub __nusers: ::core::ffi::c_uint,
        pub __kind: ::core::ffi::c_int,
        pub __spins: ::core::ffi::c_short,
        pub __elision: ::core::ffi::c_short,
        pub __list: __pthread_list_t,
    }
    use super::thread_shared_types_h::__pthread_list_t;
}
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:28"]
pub mod pthreadtypes_h {
    #[c2rust::src_loc = "27:1"]
    pub type pthread_t = ::core::ffi::c_ulong;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "32:9"]
    pub union pthread_mutexattr_t {
        pub __size: [::core::ffi::c_char; 4],
        pub __align: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "41:9"]
    pub union pthread_condattr_t {
        pub __size: [::core::ffi::c_char; 4],
        pub __align: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "67:9"]
    pub union pthread_mutex_t {
        pub __data: __pthread_mutex_s,
        pub __size: [::core::ffi::c_char; 40],
        pub __align: ::core::ffi::c_long,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "75:9"]
    pub union pthread_cond_t {
        pub __data: __pthread_cond_s,
        pub __size: [::core::ffi::c_char; 48],
        pub __align: ::core::ffi::c_longlong,
    }
    use super::struct_mutex_h::__pthread_mutex_s;
    use super::thread_shared_types_h::__pthread_cond_s;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/common.h:28"]
pub mod common_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "270:8"]
    pub struct x264_t {
        pub param: x264_param_t,
        pub api: *mut ::core::ffi::c_void,
        pub thread: [*mut x264_t; 129],
        pub lookahead_thread: [*mut x264_t; 16],
        pub b_thread_active: ::core::ffi::c_int,
        pub i_thread_phase: ::core::ffi::c_int,
        pub i_thread_idx: ::core::ffi::c_int,
        pub i_threadslice_start: ::core::ffi::c_int,
        pub i_threadslice_end: ::core::ffi::c_int,
        pub i_threadslice_pass: ::core::ffi::c_int,
        pub threadpool: *mut x264_threadpool_t,
        pub lookaheadpool: *mut x264_threadpool_t,
        pub mutex: pthread_mutex_t,
        pub cv: pthread_cond_t,
        pub out: C2RustUnnamed_18,
        pub nal_buffer: *mut uint8_t,
        pub nal_buffer_size: ::core::ffi::c_int,
        pub reconfig_h: *mut x264_t,
        pub reconfig: ::core::ffi::c_int,
        pub i_frame: ::core::ffi::c_int,
        pub i_frame_num: ::core::ffi::c_int,
        pub i_thread_frames: ::core::ffi::c_int,
        pub i_nal_type: ::core::ffi::c_int,
        pub i_nal_ref_idc: ::core::ffi::c_int,
        pub i_disp_fields: int64_t,
        pub i_disp_fields_last_frame: ::core::ffi::c_int,
        pub i_prev_duration: int64_t,
        pub i_coded_fields: int64_t,
        pub i_cpb_delay: int64_t,
        pub i_coded_fields_lookahead: int64_t,
        pub i_cpb_delay_lookahead: int64_t,
        pub i_cpb_delay_pir_offset: int64_t,
        pub i_cpb_delay_pir_offset_next: int64_t,
        pub b_queued_intra_refresh: ::core::ffi::c_int,
        pub i_last_idr_pts: int64_t,
        pub i_idr_pic_id: ::core::ffi::c_int,
        pub dequant4_mf: [*mut [::core::ffi::c_int; 16]; 4],
        pub dequant8_mf: [*mut [::core::ffi::c_int; 64]; 4],
        pub unquant4_mf: [*mut [::core::ffi::c_int; 16]; 4],
        pub unquant8_mf: [*mut [::core::ffi::c_int; 64]; 4],
        pub quant4_mf: [*mut [udctcoef; 16]; 4],
        pub quant8_mf: [*mut [udctcoef; 64]; 4],
        pub quant4_bias: [*mut [udctcoef; 16]; 4],
        pub quant8_bias: [*mut [udctcoef; 64]; 4],
        pub quant4_bias0: [*mut [udctcoef; 16]; 4],
        pub quant8_bias0: [*mut [udctcoef; 64]; 4],
        pub nr_offset_emergency: *mut [[udctcoef; 64]; 4],
        pub cost_mv: [*mut uint16_t; 82],
        pub cost_mv_fpel: [[*mut uint16_t; 4]; 82],
        pub cost_table: *mut C2RustUnnamed_17,
        pub chroma_qp_table: *const uint8_t,
        pub sh: x264_slice_header_t,
        pub sps: [x264_sps_t; 1],
        pub pps: [x264_pps_t; 1],
        pub b_sh_backup: ::core::ffi::c_int,
        pub sh_backup: x264_slice_header_t,
        pub cabac: x264_cabac_t,
        pub frames: C2RustUnnamed_11,
        pub fenc: *mut x264_frame_t,
        pub fdec: *mut x264_frame_t,
        pub i_ref: [::core::ffi::c_int; 2],
        pub fref: [[*mut x264_frame_t; 19]; 2],
        pub fref_nearest: [*mut x264_frame_t; 2],
        pub b_ref_reorder: [::core::ffi::c_int; 2],
        pub initial_cpb_removal_delay: ::core::ffi::c_int,
        pub initial_cpb_removal_delay_offset: ::core::ffi::c_int,
        pub i_reordered_pts_delay: int64_t,
        pub dct: C2RustUnnamed_10,
        pub mb: C2RustUnnamed_7,
        pub rc: *mut x264_ratecontrol_t,
        pub stat: C2RustUnnamed_6,
        pub nr_offset: *mut [udctcoef; 64],
        pub nr_residual_sum: *mut [uint32_t; 64],
        pub nr_count: *mut uint32_t,
        pub nr_offset_denoise: [[udctcoef; 64]; 4],
        pub nr_residual_sum_buf: [[[uint32_t; 64]; 4]; 2],
        pub nr_count_buf: [[uint32_t; 4]; 2],
        pub luma2chroma_pixel: [uint8_t; 7],
        pub scratch_buffer: *mut ::core::ffi::c_void,
        pub scratch_buffer2: *mut ::core::ffi::c_void,
        pub intra_border_backup: [[*mut pixel; 3]; 5],
        pub deblock_strength: [*mut [[[uint8_t; 4]; 8]; 2]; 2],
        pub predict_16x16: [x264_predict_t; 7],
        pub predict_8x8: [x264_predict8x8_t; 12],
        pub predict_4x4: [x264_predict_t; 12],
        pub predict_chroma: [x264_predict_t; 7],
        pub predict_8x8c: [x264_predict_t; 7],
        pub predict_8x16c: [x264_predict_t; 7],
        pub predict_8x8_filter: x264_predict_8x8_filter_t,
        pub pixf: x264_pixel_function_t,
        pub mc: x264_mc_functions_t,
        pub dctf: x264_dct_function_t,
        pub zigzagf: x264_zigzag_function_t,
        pub zigzagf_interlaced: x264_zigzag_function_t,
        pub zigzagf_progressive: x264_zigzag_function_t,
        pub quantf: x264_quant_function_t,
        pub loopf: x264_deblock_function_t,
        pub bsf: x264_bitstream_function_t,
        pub lookahead: *mut x264_lookahead_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "217:16"]
    pub struct x264_lookahead_t {
        pub b_exit_thread: uint8_t,
        pub b_thread_active: uint8_t,
        pub b_analyse_keyframe: uint8_t,
        pub i_last_keyframe: ::core::ffi::c_int,
        pub i_slicetype_length: ::core::ffi::c_int,
        pub last_nonb: *mut x264_frame_t,
        pub thread_handle: pthread_t,
        pub ifbuf: x264_sync_frame_list_t,
        pub next: x264_sync_frame_list_t,
        pub ofbuf: x264_sync_frame_list_t,
    }
    #[c2rust::src_loc = "94:5"]
    pub type pixel = uint16_t;
    #[c2rust::src_loc = "96:5"]
    pub type dctcoef = int32_t;
    #[c2rust::src_loc = "97:5"]
    pub type udctcoef = uint32_t;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "685:5"]
    pub struct C2RustUnnamed_6 {
        pub i_frame_count: [::core::ffi::c_int; 3],
        pub i_frame_size: [int64_t; 3],
        pub f_frame_qp: [::core::ffi::c_double; 3],
        pub i_consecutive_bframes: [::core::ffi::c_int; 17],
        pub f_ssd_global: [::core::ffi::c_double; 3],
        pub f_psnr_average: [::core::ffi::c_double; 3],
        pub f_psnr_mean_y: [::core::ffi::c_double; 3],
        pub f_psnr_mean_u: [::core::ffi::c_double; 3],
        pub f_psnr_mean_v: [::core::ffi::c_double; 3],
        pub f_ssim_mean_y: [::core::ffi::c_double; 3],
        pub f_frame_duration: [::core::ffi::c_double; 3],
        pub i_mb_count: [[int64_t; 19]; 3],
        pub i_mb_partition: [[int64_t; 17]; 2],
        pub i_mb_count_8x8dct: [int64_t; 2],
        pub i_mb_count_ref: [[[int64_t; 32]; 2]; 2],
        pub i_mb_cbp: [int64_t; 6],
        pub i_mb_pred_mode: [[int64_t; 13]; 4],
        pub i_mb_field: [int64_t; 3],
        pub i_direct_score: [::core::ffi::c_int; 2],
        pub i_direct_frames: [::core::ffi::c_int; 2],
        pub i_wpred: [::core::ffi::c_int; 2],
        pub frame: x264_frame_stat_t,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "243:9"]
    pub struct x264_frame_stat_t {
        pub i_mv_bits: ::core::ffi::c_int,
        pub i_tex_bits: ::core::ffi::c_int,
        pub i_misc_bits: ::core::ffi::c_int,
        pub i_mb_count: [::core::ffi::c_int; 19],
        pub i_mb_count_i: ::core::ffi::c_int,
        pub i_mb_count_p: ::core::ffi::c_int,
        pub i_mb_count_skip: ::core::ffi::c_int,
        pub i_mb_count_8x8dct: [::core::ffi::c_int; 2],
        pub i_mb_count_ref: [[::core::ffi::c_int; 32]; 2],
        pub i_mb_partition: [::core::ffi::c_int; 17],
        pub i_mb_cbp: [::core::ffi::c_int; 6],
        pub i_mb_pred_mode: [[::core::ffi::c_int; 13]; 4],
        pub i_mb_field: [::core::ffi::c_int; 3],
        pub i_direct_score: [::core::ffi::c_int; 2],
        pub i_ssd: [int64_t; 3],
        pub f_ssim: ::core::ffi::c_double,
        pub i_ssim_cnt: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "438:5"]
    pub struct C2RustUnnamed_7 {
        pub i_mb_width: ::core::ffi::c_int,
        pub i_mb_height: ::core::ffi::c_int,
        pub i_mb_count: ::core::ffi::c_int,
        pub chroma_h_shift: ::core::ffi::c_int,
        pub chroma_v_shift: ::core::ffi::c_int,
        pub i_mb_stride: ::core::ffi::c_int,
        pub i_b8_stride: ::core::ffi::c_int,
        pub i_b4_stride: ::core::ffi::c_int,
        pub left_b8: [::core::ffi::c_int; 2],
        pub left_b4: [::core::ffi::c_int; 2],
        pub i_mb_x: ::core::ffi::c_int,
        pub i_mb_y: ::core::ffi::c_int,
        pub i_mb_xy: ::core::ffi::c_int,
        pub i_b8_xy: ::core::ffi::c_int,
        pub i_b4_xy: ::core::ffi::c_int,
        pub i_me_method: ::core::ffi::c_int,
        pub i_subpel_refine: ::core::ffi::c_int,
        pub b_chroma_me: ::core::ffi::c_int,
        pub b_trellis: ::core::ffi::c_int,
        pub b_noise_reduction: ::core::ffi::c_int,
        pub b_dct_decimate: ::core::ffi::c_int,
        pub i_psy_rd: ::core::ffi::c_int,
        pub i_psy_trellis: ::core::ffi::c_int,
        pub b_interlaced: ::core::ffi::c_int,
        pub b_adaptive_mbaff: ::core::ffi::c_int,
        pub mv_min: [::core::ffi::c_int; 2],
        pub mv_max: [::core::ffi::c_int; 2],
        pub mv_miny_row: [::core::ffi::c_int; 3],
        pub mv_maxy_row: [::core::ffi::c_int; 3],
        pub mv_min_spel: [::core::ffi::c_int; 2],
        pub mv_max_spel: [::core::ffi::c_int; 2],
        pub mv_miny_spel_row: [::core::ffi::c_int; 3],
        pub mv_maxy_spel_row: [::core::ffi::c_int; 3],
        pub mv_limit_fpel: [[int16_t; 2]; 2],
        pub mv_miny_fpel_row: [::core::ffi::c_int; 3],
        pub mv_maxy_fpel_row: [::core::ffi::c_int; 3],
        pub i_neighbour: ::core::ffi::c_uint,
        pub i_neighbour8: [::core::ffi::c_uint; 4],
        pub i_neighbour4: [::core::ffi::c_uint; 16],
        pub i_neighbour_intra: ::core::ffi::c_uint,
        pub i_neighbour_frame: ::core::ffi::c_uint,
        pub i_mb_type_top: ::core::ffi::c_int,
        pub i_mb_type_left: [::core::ffi::c_int; 2],
        pub i_mb_type_topleft: ::core::ffi::c_int,
        pub i_mb_type_topright: ::core::ffi::c_int,
        pub i_mb_prev_xy: ::core::ffi::c_int,
        pub i_mb_left_xy: [::core::ffi::c_int; 2],
        pub i_mb_top_xy: ::core::ffi::c_int,
        pub i_mb_topleft_xy: ::core::ffi::c_int,
        pub i_mb_topright_xy: ::core::ffi::c_int,
        pub i_mb_top_y: ::core::ffi::c_int,
        pub i_mb_topleft_y: ::core::ffi::c_int,
        pub i_mb_topright_y: ::core::ffi::c_int,
        pub left_index_table: *const x264_left_table_t,
        pub i_mb_top_mbpair_xy: ::core::ffi::c_int,
        pub topleft_partition: ::core::ffi::c_int,
        pub b_allow_skip: ::core::ffi::c_int,
        pub field_decoding_flag: ::core::ffi::c_int,
        pub base: *mut uint8_t,
        pub type_0: *mut int8_t,
        pub partition: *mut uint8_t,
        pub qp: *mut int8_t,
        pub cbp: *mut int16_t,
        pub intra4x4_pred_mode: *mut [int8_t; 8],
        pub non_zero_count: *mut [uint8_t; 48],
        pub chroma_pred_mode: *mut int8_t,
        pub mv: [*mut [int16_t; 2]; 2],
        pub mvd: [*mut [[uint8_t; 2]; 8]; 2],
        pub ref_0: [*mut int8_t; 2],
        pub mvr: [[*mut [int16_t; 2]; 32]; 2],
        pub skipbp: *mut int8_t,
        pub mb_transform_size: *mut int8_t,
        pub slice_table: *mut int32_t,
        pub field: *mut uint8_t,
        pub p_weight_buf: [*mut pixel; 16],
        pub i_type: ::core::ffi::c_int,
        pub i_partition: ::core::ffi::c_int,
        pub i_sub_partition: [uint8_t; 4],
        pub b_transform_8x8: ::core::ffi::c_int,
        pub i_cbp_luma: ::core::ffi::c_int,
        pub i_cbp_chroma: ::core::ffi::c_int,
        pub i_intra16x16_pred_mode: ::core::ffi::c_int,
        pub i_chroma_pred_mode: ::core::ffi::c_int,
        pub i_skip_intra: ::core::ffi::c_int,
        pub b_skip_mc: ::core::ffi::c_int,
        pub b_reencode_mb: ::core::ffi::c_int,
        pub ip_offset: ::core::ffi::c_int,
        pub b_deblock_rdo: ::core::ffi::c_int,
        pub b_overflow: ::core::ffi::c_int,
        pub pic: C2RustUnnamed_9,
        pub cache: C2RustUnnamed_8,
        pub i_qp: ::core::ffi::c_int,
        pub i_chroma_qp: ::core::ffi::c_int,
        pub i_last_qp: ::core::ffi::c_int,
        pub i_last_dqp: ::core::ffi::c_int,
        pub b_variable_qp: ::core::ffi::c_int,
        pub b_lossless: ::core::ffi::c_int,
        pub b_direct_auto_read: ::core::ffi::c_int,
        pub b_direct_auto_write: ::core::ffi::c_int,
        pub i_trellis_lambda2: [[::core::ffi::c_int; 2]; 2],
        pub i_psy_rd_lambda: ::core::ffi::c_int,
        pub i_chroma_lambda2_offset: ::core::ffi::c_int,
        pub dist_scale_factor_buf: [[[[int16_t; 4]; 32]; 2]; 2],
        pub dist_scale_factor: *mut [int16_t; 4],
        pub bipred_weight_buf: [[[[int8_t; 4]; 32]; 2]; 2],
        pub bipred_weight: *mut [int8_t; 4],
        pub map_col_to_list0: [int8_t; 18],
        pub ref_blind_dupe: ::core::ffi::c_int,
        pub deblock_ref_table: [int8_t; 34],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "614:9"]
    pub struct C2RustUnnamed_8 {
        pub intra4x4_pred_mode: [int8_t; 40],
        pub non_zero_count: [uint8_t; 120],
        pub ref_0: [[int8_t; 40]; 2],
        pub mv: [[[int16_t; 2]; 40]; 2],
        pub mvd: [[[uint8_t; 2]; 40]; 2],
        pub skip: [int8_t; 40],
        pub direct_mv: [[[int16_t; 2]; 4]; 2],
        pub direct_ref: [[int8_t; 4]; 2],
        pub direct_partition: ::core::ffi::c_int,
        pub pskip_mv: [int16_t; 2],
        pub i_neighbour_transform_size: ::core::ffi::c_int,
        pub i_neighbour_skip: ::core::ffi::c_int,
        pub i_cbp_top: ::core::ffi::c_int,
        pub i_cbp_left: ::core::ffi::c_int,
        pub topright_mv: [[[int16_t; 2]; 3]; 2],
        pub topright_ref: [[int8_t; 3]; 2],
        pub deblock_strength: *mut [[uint8_t; 4]; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "567:9"]
    pub struct C2RustUnnamed_9 {
        pub fenc_buf: [pixel; 768],
        pub fdec_buf: [pixel; 1728],
        pub i4x4_fdec_buf: [pixel; 256],
        pub i8x8_fdec_buf: [pixel; 256],
        pub i8x8_dct_buf: [[dctcoef; 64]; 3],
        pub i4x4_dct_buf: [[dctcoef; 16]; 15],
        pub i4x4_nnz_buf: [uint32_t; 4],
        pub i8x8_nnz_buf: [uint32_t; 4],
        pub fenc_dct8: [[dctcoef; 64]; 4],
        pub fenc_dct4: [[dctcoef; 16]; 16],
        pub fenc_satd_cache: [uint32_t; 32],
        pub fenc_hadamard_cache: [uint64_t; 9],
        pub i4x4_cbp: ::core::ffi::c_int,
        pub i8x8_cbp: ::core::ffi::c_int,
        pub p_fenc: [*mut pixel; 3],
        pub p_fenc_plane: [*mut pixel; 3],
        pub p_fdec: [*mut pixel; 3],
        pub i_fref: [::core::ffi::c_int; 2],
        pub p_fref: [[[*mut pixel; 12]; 32]; 2],
        pub p_fref_w: [*mut pixel; 32],
        pub p_integral: [[*mut uint16_t; 16]; 2],
        pub i_stride: [::core::ffi::c_int; 3],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "233:16"]
    pub struct x264_left_table_t {
        pub intra: [uint8_t; 4],
        pub nnz: [uint8_t; 4],
        pub nnz_chroma: [uint8_t; 4],
        pub mv: [uint8_t; 4],
        pub ref_0: [uint8_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "428:5"]
    pub struct C2RustUnnamed_10 {
        pub luma16x16_dc: [[dctcoef; 16]; 3],
        pub chroma_dc: [[dctcoef; 8]; 2],
        pub luma8x8: [[dctcoef; 64]; 12],
        pub luma4x4: [[dctcoef; 16]; 48],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "375:5"]
    pub struct C2RustUnnamed_11 {
        pub current: *mut *mut x264_frame_t,
        pub unused: [*mut *mut x264_frame_t; 2],
        pub blank_unused: *mut *mut x264_frame_t,
        pub reference: [*mut x264_frame_t; 18],
        pub i_last_keyframe: ::core::ffi::c_int,
        pub i_last_idr: ::core::ffi::c_int,
        pub i_poc_last_open_gop: ::core::ffi::c_int,
        pub i_input: ::core::ffi::c_int,
        pub i_max_dpb: ::core::ffi::c_int,
        pub i_max_ref0: ::core::ffi::c_int,
        pub i_max_ref1: ::core::ffi::c_int,
        pub i_delay: ::core::ffi::c_int,
        pub i_bframe_delay: ::core::ffi::c_int,
        pub i_bframe_delay_time: int64_t,
        pub i_first_pts: int64_t,
        pub i_prev_reordered_pts: [int64_t; 2],
        pub i_largest_pts: int64_t,
        pub i_second_largest_pts: int64_t,
        pub b_have_lowres: ::core::ffi::c_int,
        pub b_have_sub8x8_esa: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "153:9"]
    pub struct x264_slice_header_t {
        pub sps: *mut x264_sps_t,
        pub pps: *mut x264_pps_t,
        pub i_type: ::core::ffi::c_int,
        pub i_first_mb: ::core::ffi::c_int,
        pub i_last_mb: ::core::ffi::c_int,
        pub i_pps_id: ::core::ffi::c_int,
        pub i_frame_num: ::core::ffi::c_int,
        pub b_mbaff: ::core::ffi::c_int,
        pub b_field_pic: ::core::ffi::c_int,
        pub b_bottom_field: ::core::ffi::c_int,
        pub i_idr_pic_id: ::core::ffi::c_int,
        pub i_poc: ::core::ffi::c_int,
        pub i_delta_poc_bottom: ::core::ffi::c_int,
        pub i_delta_poc: [::core::ffi::c_int; 2],
        pub i_redundant_pic_cnt: ::core::ffi::c_int,
        pub b_direct_spatial_mv_pred: ::core::ffi::c_int,
        pub b_num_ref_idx_override: ::core::ffi::c_int,
        pub i_num_ref_idx_l0_active: ::core::ffi::c_int,
        pub i_num_ref_idx_l1_active: ::core::ffi::c_int,
        pub b_ref_pic_list_reordering: [::core::ffi::c_int; 2],
        pub ref_pic_list_order: [[C2RustUnnamed_13; 16]; 2],
        pub b_weighted_pred: ::core::ffi::c_int,
        pub weight: [[x264_weight_t; 3]; 32],
        pub i_mmco_remove_from_end: ::core::ffi::c_int,
        pub i_mmco_command_count: ::core::ffi::c_int,
        pub mmco: [C2RustUnnamed_12; 16],
        pub i_cabac_init_idc: ::core::ffi::c_int,
        pub i_qp: ::core::ffi::c_int,
        pub i_qp_delta: ::core::ffi::c_int,
        pub b_sp_for_swidth: ::core::ffi::c_int,
        pub i_qs_delta: ::core::ffi::c_int,
        pub i_disable_deblocking_filter_idc: ::core::ffi::c_int,
        pub i_alpha_c0_offset: ::core::ffi::c_int,
        pub i_beta_offset: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "197:5"]
    pub struct C2RustUnnamed_12 {
        pub i_difference_of_pic_nums: ::core::ffi::c_int,
        pub i_poc: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "185:5"]
    pub struct C2RustUnnamed_13 {
        pub idc: ::core::ffi::c_int,
        pub arg: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "353:5"]
    pub struct C2RustUnnamed_17 {
        pub ref_0: [[[uint16_t; 33]; 3]; 82],
        pub i4x4_mode: [[uint16_t; 17]; 82],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "291:5"]
    pub struct C2RustUnnamed_18 {
        pub i_nal: ::core::ffi::c_int,
        pub i_nals_allocated: ::core::ffi::c_int,
        pub nal: *mut x264_nal_t,
        pub i_bitstream: ::core::ffi::c_int,
        pub p_bitstream: *mut uint8_t,
        pub bs: bs_t,
    }
    #[c2rust::src_loc = "111:9"]
    pub const SIZEOF_PIXEL: ::core::ffi::c_int = ::core::mem::size_of::<pixel>()
        as ::core::ffi::c_int;
    use super::x264_h::{x264_param_t, x264_nal_t};
    use super::threadpool_h::x264_threadpool_t;
    use super::pthreadtypes_h::{pthread_mutex_t, pthread_cond_t, pthread_t};
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
    use super::stdint_intn_h::{int64_t, int32_t, int16_t, int8_t};
    use super::set_h::{x264_sps_t, x264_pps_t};
    use super::cabac_h::x264_cabac_t;
    use super::frame_h::{x264_frame_t, x264_deblock_function_t, x264_sync_frame_list_t};
    use super::predict_h::{x264_predict_t, x264_predict8x8_t, x264_predict_8x8_filter_t};
    use super::pixel_h::x264_pixel_function_t;
    use super::mc_h::{x264_mc_functions_t, x264_weight_t};
    use super::dct_h::{x264_dct_function_t, x264_zigzag_function_t};
    use super::quant_h::x264_quant_function_t;
    use super::bitstream_h::{x264_bitstream_function_t, bs_t};
    extern "C" {
        #[c2rust::src_loc = "231:16"]
        pub type x264_ratecontrol_t;
        #[c2rust::src_loc = "138:1"]
        pub fn x264_10_log(
            h: *mut x264_t,
            i_level: ::core::ffi::c_int,
            psz_fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/frame.h:28"]
pub mod frame_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "186:9"]
    pub struct x264_sync_frame_list_t {
        pub list: *mut *mut x264_frame_t,
        pub i_max_size: ::core::ffi::c_int,
        pub i_size: ::core::ffi::c_int,
        pub mutex: pthread_mutex_t,
        pub cv_fill: pthread_cond_t,
        pub cv_empty: pthread_cond_t,
    }
    #[c2rust::src_loc = "37:1"]
    pub type x264_frame_t = x264_frame;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "37:16"]
    pub struct x264_frame {
        pub base: *mut uint8_t,
        pub i_poc: ::core::ffi::c_int,
        pub i_delta_poc: [::core::ffi::c_int; 2],
        pub i_type: ::core::ffi::c_int,
        pub i_forced_type: ::core::ffi::c_int,
        pub i_qpplus1: ::core::ffi::c_int,
        pub i_pts: int64_t,
        pub i_dts: int64_t,
        pub i_reordered_pts: int64_t,
        pub i_duration: int64_t,
        pub f_duration: ::core::ffi::c_float,
        pub i_cpb_duration: int64_t,
        pub i_cpb_delay: int64_t,
        pub i_dpb_output_delay: int64_t,
        pub param: *mut x264_param_t,
        pub i_frame: ::core::ffi::c_int,
        pub i_coded: ::core::ffi::c_int,
        pub i_field_cnt: int64_t,
        pub i_frame_num: ::core::ffi::c_int,
        pub b_kept_as_ref: ::core::ffi::c_int,
        pub i_pic_struct: ::core::ffi::c_int,
        pub b_keyframe: ::core::ffi::c_int,
        pub b_fdec: uint8_t,
        pub b_last_minigop_bframe: uint8_t,
        pub i_bframes: uint8_t,
        pub f_qp_avg_rc: ::core::ffi::c_float,
        pub f_qp_avg_aq: ::core::ffi::c_float,
        pub f_crf_avg: ::core::ffi::c_float,
        pub i_poc_l0ref0: ::core::ffi::c_int,
        pub i_csp: ::core::ffi::c_int,
        pub i_plane: ::core::ffi::c_int,
        pub i_stride: [::core::ffi::c_int; 3],
        pub i_width: [::core::ffi::c_int; 3],
        pub i_lines: [::core::ffi::c_int; 3],
        pub i_stride_lowres: ::core::ffi::c_int,
        pub i_width_lowres: ::core::ffi::c_int,
        pub i_lines_lowres: ::core::ffi::c_int,
        pub plane: [*mut pixel; 3],
        pub plane_fld: [*mut pixel; 3],
        pub filtered: [[*mut pixel; 4]; 3],
        pub filtered_fld: [[*mut pixel; 4]; 3],
        pub lowres: [*mut pixel; 4],
        pub integral: *mut uint16_t,
        pub buffer: [*mut pixel; 4],
        pub buffer_fld: [*mut pixel; 4],
        pub buffer_lowres: *mut pixel,
        pub weight: [[x264_weight_t; 3]; 16],
        pub weighted: [*mut pixel; 16],
        pub b_duplicate: ::core::ffi::c_int,
        pub orig: *mut x264_frame,
        pub mb_type: *mut int8_t,
        pub mb_partition: *mut uint8_t,
        pub mv: [*mut [int16_t; 2]; 2],
        pub mv16x16: *mut [int16_t; 2],
        pub lowres_mvs: [[*mut [int16_t; 2]; 17]; 2],
        pub field: *mut uint8_t,
        pub effective_qp: *mut uint8_t,
        pub lowres_costs: [[*mut uint16_t; 18]; 18],
        pub lowres_mv_costs: [[*mut ::core::ffi::c_int; 17]; 2],
        pub ref_0: [*mut int8_t; 2],
        pub i_ref: [::core::ffi::c_int; 2],
        pub ref_poc: [[::core::ffi::c_int; 16]; 2],
        pub inv_ref_poc: [int16_t; 2],
        pub i_cost_est: [[::core::ffi::c_int; 18]; 18],
        pub i_cost_est_aq: [[::core::ffi::c_int; 18]; 18],
        pub i_satd: ::core::ffi::c_int,
        pub i_intra_mbs: [::core::ffi::c_int; 18],
        pub i_row_satds: [[*mut ::core::ffi::c_int; 18]; 18],
        pub i_row_satd: *mut ::core::ffi::c_int,
        pub i_row_bits: *mut ::core::ffi::c_int,
        pub f_row_qp: *mut ::core::ffi::c_float,
        pub f_row_qscale: *mut ::core::ffi::c_float,
        pub f_qp_offset: *mut ::core::ffi::c_float,
        pub f_qp_offset_aq: *mut ::core::ffi::c_float,
        pub b_intra_calculated: ::core::ffi::c_int,
        pub i_intra_cost: *mut uint16_t,
        pub i_propagate_cost: *mut uint16_t,
        pub i_inv_qscale_factor: *mut uint16_t,
        pub b_scenecut: ::core::ffi::c_int,
        pub f_weighted_cost_delta: [::core::ffi::c_float; 18],
        pub i_pixel_sum: [uint32_t; 3],
        pub i_pixel_ssd: [uint64_t; 3],
        pub hrd_timing: x264_hrd_t,
        pub i_planned_type: [uint8_t; 251],
        pub i_planned_satd: [::core::ffi::c_int; 251],
        pub f_planned_cpb_duration: [::core::ffi::c_double; 251],
        pub i_coded_fields_lookahead: int64_t,
        pub i_cpb_delay_lookahead: int64_t,
        pub i_lines_completed: ::core::ffi::c_int,
        pub i_lines_weighted: ::core::ffi::c_int,
        pub i_reference_count: ::core::ffi::c_int,
        pub mutex: pthread_mutex_t,
        pub cv: pthread_cond_t,
        pub i_slice_count: ::core::ffi::c_int,
        pub f_pir_position: ::core::ffi::c_float,
        pub i_pir_start_col: ::core::ffi::c_int,
        pub i_pir_end_col: ::core::ffi::c_int,
        pub i_frames_since_pir: ::core::ffi::c_int,
        pub b_corrupt: ::core::ffi::c_int,
        pub extra_sei: x264_sei_t,
        pub opaque: *mut ::core::ffi::c_void,
        pub mb_info: *mut uint8_t,
        pub mb_info_free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "198:9"]
    pub struct x264_deblock_function_t {
        pub deblock_luma: [x264_deblock_inter_t; 2],
        pub deblock_chroma: [x264_deblock_inter_t; 2],
        pub deblock_h_chroma_420: x264_deblock_inter_t,
        pub deblock_h_chroma_422: x264_deblock_inter_t,
        pub deblock_luma_intra: [x264_deblock_intra_t; 2],
        pub deblock_chroma_intra: [x264_deblock_intra_t; 2],
        pub deblock_h_chroma_420_intra: x264_deblock_intra_t,
        pub deblock_h_chroma_422_intra: x264_deblock_intra_t,
        pub deblock_luma_mbaff: x264_deblock_inter_t,
        pub deblock_chroma_mbaff: x264_deblock_inter_t,
        pub deblock_chroma_420_mbaff: x264_deblock_inter_t,
        pub deblock_chroma_422_mbaff: x264_deblock_inter_t,
        pub deblock_luma_intra_mbaff: x264_deblock_intra_t,
        pub deblock_chroma_intra_mbaff: x264_deblock_intra_t,
        pub deblock_chroma_420_intra_mbaff: x264_deblock_intra_t,
        pub deblock_chroma_422_intra_mbaff: x264_deblock_intra_t,
        pub deblock_strength: Option<
            unsafe extern "C" fn(
                *mut uint8_t,
                *mut [int8_t; 40],
                *mut [[int16_t; 2]; 40],
                *mut [[uint8_t; 4]; 8],
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
    }
    #[c2rust::src_loc = "197:1"]
    pub type x264_deblock_intra_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "196:1"]
    pub type x264_deblock_inter_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >;
    #[c2rust::src_loc = "32:9"]
    pub const PADH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    #[c2rust::src_loc = "33:9"]
    pub const PADV: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    use super::pthreadtypes_h::{pthread_mutex_t, pthread_cond_t};
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
    use super::stdint_intn_h::{int64_t, int8_t, int16_t};
    use super::x264_h::{x264_param_t, x264_hrd_t, x264_sei_t};
    use super::common_h::pixel;
    use super::mc_h::x264_weight_t;
    use super::stdint_h::intptr_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:28"]
pub mod x264_h {
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
        pub vui: C2RustUnnamed_5,
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
        pub analyse: C2RustUnnamed_4,
        pub rc: C2RustUnnamed_3,
        pub crop_rect: C2RustUnnamed_2,
        pub i_frame_packing: ::core::ffi::c_int,
        pub mastering_display: C2RustUnnamed_1,
        pub content_light_level: C2RustUnnamed_0,
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
    #[c2rust::src_loc = "517:5"]
    pub struct C2RustUnnamed_0 {
        pub b_cll: ::core::ffi::c_int,
        pub i_max_cll: ::core::ffi::c_int,
        pub i_max_fall: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "501:5"]
    pub struct C2RustUnnamed_1 {
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
    pub struct C2RustUnnamed_2 {
        pub i_left: ::core::ffi::c_int,
        pub i_top: ::core::ffi::c_int,
        pub i_right: ::core::ffi::c_int,
        pub i_bottom: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "443:5"]
    pub struct C2RustUnnamed_3 {
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
    #[c2rust::src_loc = "406:5"]
    pub struct C2RustUnnamed_4 {
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
    pub struct C2RustUnnamed_5 {
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
    #[c2rust::src_loc = "149:9"]
    pub const X264_CPU_AVX: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 9 as ::core::ffi::c_int;
    #[c2rust::src_loc = "156:9"]
    pub const X264_CPU_AVX512: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 16 as ::core::ffi::c_int;
    #[c2rust::src_loc = "158:9"]
    pub const X264_CPU_CACHELINE_32: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 17 as ::core::ffi::c_int;
    #[c2rust::src_loc = "159:9"]
    pub const X264_CPU_CACHELINE_64: ::core::ffi::c_uint = (1 as ::core::ffi::c_uint)
        << 18 as ::core::ffi::c_int;
    #[c2rust::src_loc = "209:9"]
    pub const X264_ME_ESA: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "217:9"]
    pub const X264_QP_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "254:9"]
    pub const X264_CSP_MASK: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
    #[c2rust::src_loc = "255:9"]
    pub const X264_CSP_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "256:9"]
    pub const X264_CSP_I400: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "257:9"]
    pub const X264_CSP_I420: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "258:9"]
    pub const X264_CSP_YV12: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "259:9"]
    pub const X264_CSP_NV12: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
    #[c2rust::src_loc = "260:9"]
    pub const X264_CSP_NV21: ::core::ffi::c_int = 0x5 as ::core::ffi::c_int;
    #[c2rust::src_loc = "261:9"]
    pub const X264_CSP_I422: ::core::ffi::c_int = 0x6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "262:9"]
    pub const X264_CSP_YV16: ::core::ffi::c_int = 0x7 as ::core::ffi::c_int;
    #[c2rust::src_loc = "263:9"]
    pub const X264_CSP_NV16: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
    #[c2rust::src_loc = "264:9"]
    pub const X264_CSP_YUYV: ::core::ffi::c_int = 0x9 as ::core::ffi::c_int;
    #[c2rust::src_loc = "265:9"]
    pub const X264_CSP_UYVY: ::core::ffi::c_int = 0xa as ::core::ffi::c_int;
    #[c2rust::src_loc = "266:9"]
    pub const X264_CSP_V210: ::core::ffi::c_int = 0xb as ::core::ffi::c_int;
    #[c2rust::src_loc = "267:9"]
    pub const X264_CSP_I444: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;
    #[c2rust::src_loc = "268:9"]
    pub const X264_CSP_YV24: ::core::ffi::c_int = 0xd as ::core::ffi::c_int;
    #[c2rust::src_loc = "269:9"]
    pub const X264_CSP_BGR: ::core::ffi::c_int = 0xe as ::core::ffi::c_int;
    #[c2rust::src_loc = "270:9"]
    pub const X264_CSP_BGRA: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;
    #[c2rust::src_loc = "271:9"]
    pub const X264_CSP_RGB: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
    #[c2rust::src_loc = "273:9"]
    pub const X264_CSP_VFLIP: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "274:9"]
    pub const X264_CSP_HIGH_DEPTH: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
    #[c2rust::src_loc = "277:9"]
    pub const X264_TYPE_AUTO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "283:9"]
    pub const X264_TYPE_KEYFRAME: ::core::ffi::c_int = 0x6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "290:9"]
    pub const X264_LOG_WARNING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::internal::__va_list_tag;
    use super::common_h::x264_t;
    use super::stdint_intn_h::int64_t;
    extern "C" {
        #[c2rust::src_loc = "679:10"]
        pub fn x264_param_cleanup(param: *mut x264_param_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/mc.h:28"]
pub mod mc_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "235:16"]
    pub struct x264_weight_t {
        pub cachea: [int16_t; 8],
        pub cacheb: [int16_t; 8],
        pub i_denom: int32_t,
        pub i_scale: int32_t,
        pub i_offset: int32_t,
        pub weightfn: *mut weight_fn_t,
    }
    #[c2rust::src_loc = "234:1"]
    pub type weight_fn_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
            *const x264_weight_t,
            ::core::ffi::c_int,
        ) -> (),
    >;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "267:9"]
    pub struct x264_mc_functions_t {
        pub mc_luma: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *const x264_weight_t,
            ) -> (),
        >,
        pub get_ref: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut intptr_t,
                *mut *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *const x264_weight_t,
            ) -> *mut pixel,
        >,
        pub mc_chroma: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub avg: [Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >; 12],
        pub copy: [Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >; 7],
        pub copy_16x16_unaligned: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub store_interleave_chroma: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                *mut pixel,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub load_deinterleave_chroma_fenc: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub load_deinterleave_chroma_fdec: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_swap: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_interleave: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_deinterleave: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_deinterleave_yuyv: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_deinterleave_rgb: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub plane_copy_deinterleave_v210: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut uint32_t,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub hpel_filter: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut int16_t,
            ) -> (),
        >,
        pub prefetch_fenc: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub prefetch_fenc_400: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub prefetch_fenc_420: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub prefetch_fenc_422: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub prefetch_ref: Option<
            unsafe extern "C" fn(*mut pixel, intptr_t, ::core::ffi::c_int) -> (),
        >,
        pub memcpy_aligned: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                size_t,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub memzero_aligned: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> (),
        >,
        pub integral_init4h: Option<
            unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> (),
        >,
        pub integral_init8h: Option<
            unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> (),
        >,
        pub integral_init4v: Option<
            unsafe extern "C" fn(*mut uint16_t, *mut uint16_t, intptr_t) -> (),
        >,
        pub integral_init8v: Option<unsafe extern "C" fn(*mut uint16_t, intptr_t) -> ()>,
        pub frame_init_lowres_core: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub weight: *mut weight_fn_t,
        pub offsetadd: *mut weight_fn_t,
        pub offsetsub: *mut weight_fn_t,
        pub weight_cache: Option<
            unsafe extern "C" fn(*mut x264_t, *mut x264_weight_t) -> (),
        >,
        pub mbtree_propagate_cost: Option<
            unsafe extern "C" fn(
                *mut int16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut ::core::ffi::c_float,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub mbtree_propagate_list: Option<
            unsafe extern "C" fn(
                *mut x264_t,
                *mut uint16_t,
                *mut [int16_t; 2],
                *mut int16_t,
                *mut uint16_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub mbtree_fix8_pack: Option<
            unsafe extern "C" fn(
                *mut uint16_t,
                *mut ::core::ffi::c_float,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub mbtree_fix8_unpack: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_float,
                *mut uint16_t,
                ::core::ffi::c_int,
            ) -> (),
        >,
    }
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::common_h::{pixel, x264_t};
    use super::stdint_h::intptr_t;
    use super::stdint_uintn_h::{uint32_t, uint16_t};
    use super::__stddef_size_t_h::size_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/bitstream.h:28"]
pub mod bitstream_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:9"]
    pub struct x264_bitstream_function_t {
        pub nal_escape: Option<
            unsafe extern "C" fn(
                *mut uint8_t,
                *mut uint8_t,
                *mut uint8_t,
            ) -> *mut uint8_t,
        >,
        pub cabac_block_residual_internal: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                ::core::ffi::c_int,
                intptr_t,
                *mut x264_cabac_t,
            ) -> (),
        >,
        pub cabac_block_residual_rd_internal: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                ::core::ffi::c_int,
                intptr_t,
                *mut x264_cabac_t,
            ) -> (),
        >,
        pub cabac_block_residual_8x8_rd_internal: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                ::core::ffi::c_int,
                intptr_t,
                *mut x264_cabac_t,
            ) -> (),
        >,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "50:9"]
    pub struct x264_run_level_t {
        pub last: int32_t,
        pub mask: int32_t,
        pub level: [dctcoef; 18],
    }
    #[c2rust::src_loc = "39:1"]
    pub type bs_t = bs_s;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "39:16"]
    pub struct bs_s {
        pub p_start: *mut uint8_t,
        pub p: *mut uint8_t,
        pub p_end: *mut uint8_t,
        pub cur_bits: uintptr_t,
        pub i_left: ::core::ffi::c_int,
        pub i_bits_encoded: ::core::ffi::c_int,
    }
    use super::stdint_uintn_h::uint8_t;
    use super::common_h::dctcoef;
    use super::stdint_h::{intptr_t, uintptr_t};
    use super::cabac_h::x264_cabac_t;
    use super::stdint_intn_h::int32_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/cabac.h:28"]
pub mod cabac_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:9"]
    pub struct x264_cabac_t {
        pub i_low: ::core::ffi::c_int,
        pub i_range: ::core::ffi::c_int,
        pub i_queue: ::core::ffi::c_int,
        pub i_bytes_outstanding: ::core::ffi::c_int,
        pub p_start: *mut uint8_t,
        pub p: *mut uint8_t,
        pub p_end: *mut uint8_t,
        pub f8_bits_encoded: ::core::ffi::c_int,
        pub state: [uint8_t; 1024],
        pub padding: [uint8_t; 12],
    }
    use super::stdint_uintn_h::uint8_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/quant.h:28"]
pub mod quant_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:9"]
    pub struct x264_quant_function_t {
        pub quant_8x8: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut udctcoef,
                *mut udctcoef,
            ) -> ::core::ffi::c_int,
        >,
        pub quant_4x4: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut udctcoef,
                *mut udctcoef,
            ) -> ::core::ffi::c_int,
        >,
        pub quant_4x4x4: Option<
            unsafe extern "C" fn(
                *mut [dctcoef; 16],
                *mut udctcoef,
                *mut udctcoef,
            ) -> ::core::ffi::c_int,
        >,
        pub quant_4x4_dc: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub quant_2x2_dc: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub dequant_8x8: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut [::core::ffi::c_int; 64],
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub dequant_4x4: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut [::core::ffi::c_int; 16],
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub dequant_4x4_dc: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut [::core::ffi::c_int; 16],
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub idct_dequant_2x4_dc: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut [dctcoef; 16],
                *mut [::core::ffi::c_int; 16],
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub idct_dequant_2x4_dconly: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut [::core::ffi::c_int; 16],
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub optimize_chroma_2x2_dc: Option<
            unsafe extern "C" fn(*mut dctcoef, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub optimize_chroma_2x4_dc: Option<
            unsafe extern "C" fn(*mut dctcoef, ::core::ffi::c_int) -> ::core::ffi::c_int,
        >,
        pub denoise_dct: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut uint32_t,
                *mut udctcoef,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub decimate_score15: Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >,
        pub decimate_score16: Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >,
        pub decimate_score64: Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >,
        pub coeff_last: [Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >; 14],
        pub coeff_last4: Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >,
        pub coeff_last8: Option<
            unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int,
        >,
        pub coeff_level_run: [Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut x264_run_level_t,
            ) -> ::core::ffi::c_int,
        >; 13],
        pub coeff_level_run4: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut x264_run_level_t,
            ) -> ::core::ffi::c_int,
        >,
        pub coeff_level_run8: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut x264_run_level_t,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_4x4: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_8x8: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_4x4_psy: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
                ::core::ffi::c_int,
                *mut dctcoef,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_8x8_psy: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
                ::core::ffi::c_int,
                *mut dctcoef,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_dc: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub trellis_cabac_chroma_422_dc: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_int,
                *const uint8_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut dctcoef,
                *mut dctcoef,
                *mut dctcoef,
                *mut uint8_t,
                *mut uint8_t,
                uint64_t,
                uint16_t,
            ) -> ::core::ffi::c_int,
        >,
    }
    use super::common_h::{dctcoef, udctcoef};
    use super::stdint_uintn_h::{uint32_t, uint8_t, uint64_t, uint16_t};
    use super::bitstream_h::x264_run_level_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/dct.h:28"]
pub mod dct_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "61:9"]
    pub struct x264_zigzag_function_t {
        pub scan_8x8: Option<unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ()>,
        pub scan_4x4: Option<unsafe extern "C" fn(*mut dctcoef, *mut dctcoef) -> ()>,
        pub sub_8x8: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *const pixel,
                *mut pixel,
            ) -> ::core::ffi::c_int,
        >,
        pub sub_4x4: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *const pixel,
                *mut pixel,
            ) -> ::core::ffi::c_int,
        >,
        pub sub_4x4ac: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *const pixel,
                *mut pixel,
                *mut dctcoef,
            ) -> ::core::ffi::c_int,
        >,
        pub interleave_8x8_cavlc: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut dctcoef, *mut uint8_t) -> (),
        >,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:9"]
    pub struct x264_dct_function_t {
        pub sub4x4_dct: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
        >,
        pub add4x4_idct: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
        pub sub8x8_dct: Option<
            unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> (),
        >,
        pub sub8x8_dct_dc: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
        >,
        pub add8x8_idct: Option<
            unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> (),
        >,
        pub add8x8_idct_dc: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
        pub sub8x16_dct_dc: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
        >,
        pub sub16x16_dct: Option<
            unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> (),
        >,
        pub add16x16_idct: Option<
            unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> (),
        >,
        pub add16x16_idct_dc: Option<
            unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> (),
        >,
        pub sub8x8_dct8: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> (),
        >,
        pub add8x8_idct8: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
        pub sub16x16_dct8: Option<
            unsafe extern "C" fn(*mut [dctcoef; 64], *mut pixel, *mut pixel) -> (),
        >,
        pub add16x16_idct8: Option<
            unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 64]) -> (),
        >,
        pub dct4x4dc: Option<unsafe extern "C" fn(*mut dctcoef) -> ()>,
        pub idct4x4dc: Option<unsafe extern "C" fn(*mut dctcoef) -> ()>,
        pub dct2x4dc: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut [dctcoef; 16]) -> (),
        >,
    }
    use super::common_h::{dctcoef, pixel};
    use super::stdint_uintn_h::uint8_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/pixel.h:28"]
pub mod pixel_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "78:9"]
    pub struct x264_pixel_function_t {
        pub sad: [x264_pixel_cmp_t; 8],
        pub ssd: [x264_pixel_cmp_t; 8],
        pub satd: [x264_pixel_cmp_t; 8],
        pub ssim: [x264_pixel_cmp_t; 7],
        pub sa8d: [x264_pixel_cmp_t; 4],
        pub mbcmp: [x264_pixel_cmp_t; 8],
        pub mbcmp_unaligned: [x264_pixel_cmp_t; 8],
        pub fpelcmp: [x264_pixel_cmp_t; 8],
        pub fpelcmp_x3: [x264_pixel_cmp_x3_t; 7],
        pub fpelcmp_x4: [x264_pixel_cmp_x4_t; 7],
        pub sad_aligned: [x264_pixel_cmp_t; 8],
        pub vsad: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub asd8: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        pub sa8d_satd: [Option<
            unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> uint64_t,
        >; 1],
        pub var: [Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>; 4],
        pub var2: [Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >; 4],
        pub hadamard_ac: [Option<
            unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
        >; 4],
        pub ssd_nv12_core: Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint64_t,
                *mut uint64_t,
            ) -> (),
        >,
        pub ssim_4x4x2_core: Option<
            unsafe extern "C" fn(
                *const pixel,
                intptr_t,
                *const pixel,
                intptr_t,
                *mut [::core::ffi::c_int; 4],
            ) -> (),
        >,
        pub ssim_end4: Option<
            unsafe extern "C" fn(
                *mut [::core::ffi::c_int; 4],
                *mut [::core::ffi::c_int; 4],
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_float,
        >,
        pub sad_x3: [x264_pixel_cmp_x3_t; 7],
        pub sad_x4: [x264_pixel_cmp_x4_t; 7],
        pub satd_x3: [x264_pixel_cmp_x3_t; 7],
        pub satd_x4: [x264_pixel_cmp_x4_t; 7],
        pub ads: [Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_int,
                *mut uint16_t,
                ::core::ffi::c_int,
                *mut uint16_t,
                *mut int16_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >; 7],
        pub intra_mbcmp_x3_16x16: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_satd_x3_16x16: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_16x16: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x3_4x4: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_satd_x3_4x4: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_4x4: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x3_chroma: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_satd_x3_chroma: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_chroma: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x3_8x16c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_satd_x3_8x16c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_8x16c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x3_8x8c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_satd_x3_8x8c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_8x8c: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x3_8x8: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sa8d_x3_8x8: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_sad_x3_8x8: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >,
        pub intra_mbcmp_x9_4x4: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
        pub intra_satd_x9_4x4: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
        pub intra_sad_x9_4x4: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
        pub intra_mbcmp_x9_8x8: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
        pub intra_sa8d_x9_8x8: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
        pub intra_sad_x9_8x8: Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut uint16_t,
                *mut uint16_t,
            ) -> ::core::ffi::c_int,
        >,
    }
    #[c2rust::src_loc = "35:1"]
    pub type x264_pixel_cmp_x4_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut pixel,
            intptr_t,
            *mut ::core::ffi::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "34:1"]
    pub type x264_pixel_cmp_x3_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            *mut pixel,
            *mut pixel,
            intptr_t,
            *mut ::core::ffi::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "33:1"]
    pub type x264_pixel_cmp_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            intptr_t,
            *mut pixel,
            intptr_t,
        ) -> ::core::ffi::c_int,
    >;
    use super::common_h::pixel;
    use super::stdint_h::intptr_t;
    use super::stdint_uintn_h::{uint64_t, uint16_t};
    use super::stdint_intn_h::int16_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/predict.h:28"]
pub mod predict_h {
    #[c2rust::src_loc = "32:1"]
    pub type x264_predict_8x8_filter_t = Option<
        unsafe extern "C" fn(
            *mut pixel,
            *mut pixel,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >;
    #[c2rust::src_loc = "30:1"]
    pub type x264_predict_t = Option<unsafe extern "C" fn(*mut pixel) -> ()>;
    #[c2rust::src_loc = "31:1"]
    pub type x264_predict8x8_t = Option<
        unsafe extern "C" fn(*mut pixel, *mut pixel) -> (),
    >;
    use super::common_h::pixel;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/set.h:28"]
pub mod set_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "150:9"]
    pub struct x264_pps_t {
        pub i_id: ::core::ffi::c_int,
        pub i_sps_id: ::core::ffi::c_int,
        pub b_cabac: ::core::ffi::c_int,
        pub b_pic_order: ::core::ffi::c_int,
        pub i_num_slice_groups: ::core::ffi::c_int,
        pub i_num_ref_idx_l0_default_active: ::core::ffi::c_int,
        pub i_num_ref_idx_l1_default_active: ::core::ffi::c_int,
        pub b_weighted_pred: ::core::ffi::c_int,
        pub b_weighted_bipred: ::core::ffi::c_int,
        pub i_pic_init_qp: ::core::ffi::c_int,
        pub i_pic_init_qs: ::core::ffi::c_int,
        pub i_chroma_qp_index_offset: ::core::ffi::c_int,
        pub b_deblocking_filter_control: ::core::ffi::c_int,
        pub b_constrained_intra_pred: ::core::ffi::c_int,
        pub b_redundant_pic_cnt: ::core::ffi::c_int,
        pub b_transform_8x8_mode: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "45:9"]
    pub struct x264_sps_t {
        pub i_id: ::core::ffi::c_int,
        pub i_profile_idc: ::core::ffi::c_int,
        pub i_level_idc: ::core::ffi::c_int,
        pub b_constraint_set0: ::core::ffi::c_int,
        pub b_constraint_set1: ::core::ffi::c_int,
        pub b_constraint_set2: ::core::ffi::c_int,
        pub b_constraint_set3: ::core::ffi::c_int,
        pub i_log2_max_frame_num: ::core::ffi::c_int,
        pub i_poc_type: ::core::ffi::c_int,
        pub i_log2_max_poc_lsb: ::core::ffi::c_int,
        pub i_num_ref_frames: ::core::ffi::c_int,
        pub b_gaps_in_frame_num_value_allowed: ::core::ffi::c_int,
        pub i_mb_width: ::core::ffi::c_int,
        pub i_mb_height: ::core::ffi::c_int,
        pub b_frame_mbs_only: ::core::ffi::c_int,
        pub b_mb_adaptive_frame_field: ::core::ffi::c_int,
        pub b_direct8x8_inference: ::core::ffi::c_int,
        pub b_crop: ::core::ffi::c_int,
        pub crop: C2RustUnnamed_16,
        pub b_vui: ::core::ffi::c_int,
        pub vui: C2RustUnnamed_14,
        pub b_qpprime_y_zero_transform_bypass: ::core::ffi::c_int,
        pub i_chroma_format_idc: ::core::ffi::c_int,
        pub b_avcintra_hd: ::core::ffi::c_int,
        pub b_avcintra_4k: ::core::ffi::c_int,
        pub i_cqm_preset: ::core::ffi::c_int,
        pub scaling_list: [*const uint8_t; 8],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "81:5"]
    pub struct C2RustUnnamed_14 {
        pub b_aspect_ratio_info_present: ::core::ffi::c_int,
        pub i_sar_width: ::core::ffi::c_int,
        pub i_sar_height: ::core::ffi::c_int,
        pub b_overscan_info_present: ::core::ffi::c_int,
        pub b_overscan_info: ::core::ffi::c_int,
        pub b_signal_type_present: ::core::ffi::c_int,
        pub i_vidformat: ::core::ffi::c_int,
        pub b_fullrange: ::core::ffi::c_int,
        pub b_color_description_present: ::core::ffi::c_int,
        pub i_colorprim: ::core::ffi::c_int,
        pub i_transfer: ::core::ffi::c_int,
        pub i_colmatrix: ::core::ffi::c_int,
        pub b_chroma_loc_info_present: ::core::ffi::c_int,
        pub i_chroma_loc_top: ::core::ffi::c_int,
        pub i_chroma_loc_bottom: ::core::ffi::c_int,
        pub b_timing_info_present: ::core::ffi::c_int,
        pub i_num_units_in_tick: uint32_t,
        pub i_time_scale: uint32_t,
        pub b_fixed_frame_rate: ::core::ffi::c_int,
        pub b_nal_hrd_parameters_present: ::core::ffi::c_int,
        pub b_vcl_hrd_parameters_present: ::core::ffi::c_int,
        pub hrd: C2RustUnnamed_15,
        pub b_pic_struct_present: ::core::ffi::c_int,
        pub b_bitstream_restriction: ::core::ffi::c_int,
        pub b_motion_vectors_over_pic_boundaries: ::core::ffi::c_int,
        pub i_max_bytes_per_pic_denom: ::core::ffi::c_int,
        pub i_max_bits_per_mb_denom: ::core::ffi::c_int,
        pub i_log2_max_mv_length_horizontal: ::core::ffi::c_int,
        pub i_log2_max_mv_length_vertical: ::core::ffi::c_int,
        pub i_num_reorder_frames: ::core::ffi::c_int,
        pub i_max_dec_frame_buffering: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "110:9"]
    pub struct C2RustUnnamed_15 {
        pub i_cpb_cnt: ::core::ffi::c_int,
        pub i_bit_rate_scale: ::core::ffi::c_int,
        pub i_cpb_size_scale: ::core::ffi::c_int,
        pub i_bit_rate_value: ::core::ffi::c_int,
        pub i_cpb_size_value: ::core::ffi::c_int,
        pub i_bit_rate_unscaled: ::core::ffi::c_int,
        pub i_cpb_size_unscaled: ::core::ffi::c_int,
        pub b_cbr_hrd: ::core::ffi::c_int,
        pub i_initial_cpb_removal_delay_length: ::core::ffi::c_int,
        pub i_cpb_removal_delay_length: ::core::ffi::c_int,
        pub i_dpb_output_delay_length: ::core::ffi::c_int,
        pub i_time_offset_length: ::core::ffi::c_int,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "72:5"]
    pub struct C2RustUnnamed_16 {
        pub i_left: ::core::ffi::c_int,
        pub i_right: ::core::ffi::c_int,
        pub i_top: ::core::ffi::c_int,
        pub i_bottom: ::core::ffi::c_int,
    }
    use super::stdint_uintn_h::{uint8_t, uint32_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/threadpool.h:28"]
pub mod threadpool_h {
    extern "C" {
        #[c2rust::src_loc = "29:16"]
        pub type x264_threadpool_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:28"]
pub mod base_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "64:9"]
    pub union x264_union16_t {
        pub i: uint16_t,
        pub b: [uint8_t; 2],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:9"]
    pub union x264_union32_t {
        pub i: uint32_t,
        pub w: [uint16_t; 2],
        pub b: [uint8_t; 4],
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "66:9"]
    pub union x264_union64_t {
        pub i: uint64_t,
        pub d: [uint32_t; 2],
        pub w: [uint16_t; 4],
        pub b: [uint8_t; 8],
    }
    #[c2rust::src_loc = "103:1"]
    pub type chroma_format_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "108:5"]
    pub const CHROMA_444: chroma_format_e = 3;
    #[c2rust::src_loc = "107:5"]
    pub const CHROMA_422: chroma_format_e = 2;
    #[c2rust::src_loc = "106:5"]
    pub const CHROMA_420: chroma_format_e = 1;
    #[c2rust::src_loc = "105:5"]
    pub const CHROMA_400: chroma_format_e = 0;
    use super::stdint_uintn_h::{uint16_t, uint8_t, uint32_t, uint64_t};
    use super::stdint_intn_h::int64_t;
    extern "C" {
        #[c2rust::src_loc = "279:10"]
        pub fn x264_malloc(_: int64_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "280:10"]
        pub fn x264_free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/usr/include/pthread.h:28"]
pub mod pthread_h {
    use super::pthreadtypes_h::{
        pthread_mutex_t, pthread_mutexattr_t, pthread_cond_t, pthread_condattr_t,
    };
    extern "C" {
        #[c2rust::src_loc = "781:1"]
        pub fn pthread_mutex_init(
            __mutex: *mut pthread_mutex_t,
            __mutexattr: *const pthread_mutexattr_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "786:1"]
        pub fn pthread_mutex_destroy(
            __mutex: *mut pthread_mutex_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "794:1"]
        pub fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "835:1"]
        pub fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1112:1"]
        pub fn pthread_cond_init(
            __cond: *mut pthread_cond_t,
            __cond_attr: *const pthread_condattr_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1117:1"]
        pub fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1125:1"]
        pub fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "1133:1"]
        pub fn pthread_cond_wait(
            __cond: *mut pthread_cond_t,
            __mutex: *mut pthread_mutex_t,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:28"]
pub mod osdep_h {
    #[c2rust::src_loc = "317:9"]
    pub const NATIVE_ALIGN: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
    #[inline(always)]
    #[c2rust::src_loc = "433:1"]
    pub unsafe extern "C" fn x264_pthread_fetch_and_add(
        mut val: *mut ::core::ffi::c_int,
        mut add: ::core::ffi::c_int,
        mut mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int {
        return ::core::intrinsics::atomic_xadd_seqcst(val, add);
    }
    #[c2rust::src_loc = "452:9"]
    pub const WORD_SIZE: uint64_t = ::core::mem::size_of::<*mut ::core::ffi::c_void>()
        as uint64_t;
    use super::pthreadtypes_h::pthread_mutex_t;
    use super::stdint_uintn_h::uint64_t;
}
#[c2rust::header_src = "/usr/include/string.h:28"]
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
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "980:1"]
        pub fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:28"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/assert.h:28"]
pub mod assert_h {
    extern "C" {
        #[c2rust::src_loc = "67:1"]
        pub fn __assert_fail(
            __assertion: *const ::core::ffi::c_char,
            __file: *const ::core::ffi::c_char,
            __line: ::core::ffi::c_uint,
            __function: *const ::core::ffi::c_char,
        ) -> !;
    }
}
pub use self::internal::{__va_list_tag, BIT_DEPTH};
pub use self::__stddef_size_t_h::size_t;
pub use self::types_h::{
    __int8_t, __uint8_t, __int16_t, __uint16_t, __int32_t, __uint32_t, __int64_t,
    __uint64_t,
};
pub use self::stdint_intn_h::{int8_t, int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
pub use self::stdint_h::{intptr_t, uintptr_t};
pub use self::atomic_wide_counter_h::{__atomic_wide_counter, C2RustUnnamed};
pub use self::thread_shared_types_h::{
    __pthread_internal_list, __pthread_list_t, __pthread_cond_s,
};
pub use self::struct_mutex_h::__pthread_mutex_s;
pub use self::pthreadtypes_h::{
    pthread_t, pthread_mutexattr_t, pthread_condattr_t, pthread_mutex_t, pthread_cond_t,
};
pub use self::common_h::{
    x264_t, x264_lookahead_t, pixel, dctcoef, udctcoef, C2RustUnnamed_6,
    x264_frame_stat_t, C2RustUnnamed_7, C2RustUnnamed_8, C2RustUnnamed_9,
    x264_left_table_t, C2RustUnnamed_10, C2RustUnnamed_11, x264_slice_header_t,
    C2RustUnnamed_12, C2RustUnnamed_13, C2RustUnnamed_17, C2RustUnnamed_18, SIZEOF_PIXEL,
    x264_ratecontrol_t, x264_10_log,
};
pub use self::frame_h::{
    x264_sync_frame_list_t, x264_frame_t, x264_frame, x264_deblock_function_t,
    x264_deblock_intra_t, x264_deblock_inter_t, PADH, PADV,
};
pub use self::x264_h::{
    x264_sei_t, x264_sei_payload_t, x264_hrd_t, x264_param_t, x264_nal_t,
    C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, x264_zone_t,
    C2RustUnnamed_4, C2RustUnnamed_5, pic_struct_e, PIC_STRUCT_TRIPLE, PIC_STRUCT_DOUBLE,
    PIC_STRUCT_BOTTOM_TOP_BOTTOM, PIC_STRUCT_TOP_BOTTOM_TOP, PIC_STRUCT_BOTTOM_TOP,
    PIC_STRUCT_TOP_BOTTOM, PIC_STRUCT_PROGRESSIVE, PIC_STRUCT_AUTO, x264_image_t,
    x264_image_properties_t, x264_picture_t, X264_CPU_AVX, X264_CPU_AVX512,
    X264_CPU_CACHELINE_32, X264_CPU_CACHELINE_64, X264_ME_ESA, X264_QP_AUTO,
    X264_CSP_MASK, X264_CSP_NONE, X264_CSP_I400, X264_CSP_I420, X264_CSP_YV12,
    X264_CSP_NV12, X264_CSP_NV21, X264_CSP_I422, X264_CSP_YV16, X264_CSP_NV16,
    X264_CSP_YUYV, X264_CSP_UYVY, X264_CSP_V210, X264_CSP_I444, X264_CSP_YV24,
    X264_CSP_BGR, X264_CSP_BGRA, X264_CSP_RGB, X264_CSP_VFLIP, X264_CSP_HIGH_DEPTH,
    X264_TYPE_AUTO, X264_TYPE_KEYFRAME, X264_LOG_ERROR, X264_LOG_WARNING,
    x264_param_cleanup,
};
pub use self::mc_h::{x264_weight_t, weight_fn_t, x264_mc_functions_t};
pub use self::bitstream_h::{x264_bitstream_function_t, x264_run_level_t, bs_t, bs_s};
pub use self::cabac_h::x264_cabac_t;
pub use self::quant_h::x264_quant_function_t;
pub use self::dct_h::{x264_zigzag_function_t, x264_dct_function_t};
pub use self::pixel_h::{
    x264_pixel_function_t, x264_pixel_cmp_x4_t, x264_pixel_cmp_x3_t, x264_pixel_cmp_t,
};
pub use self::predict_h::{x264_predict_8x8_filter_t, x264_predict_t, x264_predict8x8_t};
pub use self::set_h::{
    x264_pps_t, x264_sps_t, C2RustUnnamed_14, C2RustUnnamed_15, C2RustUnnamed_16,
};
use self::threadpool_h::x264_threadpool_t;
pub use self::base_h::{
    x264_union16_t, x264_union32_t, x264_union64_t, chroma_format_e, CHROMA_444,
    CHROMA_422, CHROMA_420, CHROMA_400, x264_malloc, x264_free,
};
use self::pthread_h::{
    pthread_mutex_init, pthread_mutex_destroy, pthread_mutex_lock, pthread_mutex_unlock,
    pthread_cond_init, pthread_cond_destroy, pthread_cond_broadcast, pthread_cond_wait,
};
pub use self::osdep_h::{NATIVE_ALIGN, x264_pthread_fetch_and_add, WORD_SIZE};
use self::string_h::{memcpy, memset};
use self::stdlib_h::abs;
pub use self::__stddef_null_h::NULL;
use self::assert_h::__assert_fail;
#[c2rust::src_loc = "30:1"]
unsafe extern "C" fn align_stride(
    mut x: ::core::ffi::c_int,
    mut align: ::core::ffi::c_int,
    mut disalign: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    x = x + (align - 1 as ::core::ffi::c_int) & !(align - 1 as ::core::ffi::c_int);
    if x & disalign - 1 as ::core::ffi::c_int == 0 {
        x += align;
    }
    return x;
}
#[c2rust::src_loc = "38:1"]
unsafe extern "C" fn align_plane_size(
    mut x: ::core::ffi::c_int,
    mut disalign: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if x & disalign - 1 as ::core::ffi::c_int == 0 {
        x
            += (if 128 as ::core::ffi::c_int > 64 as ::core::ffi::c_int {
                128 as ::core::ffi::c_int
            } else {
                64 as ::core::ffi::c_int
            }) / SIZEOF_PIXEL;
    }
    return x;
}
#[c2rust::src_loc = "45:1"]
unsafe extern "C" fn frame_internal_csp(
    mut external_csp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut csp: ::core::ffi::c_int = external_csp & X264_CSP_MASK;
    if csp == X264_CSP_I400 {
        return X264_CSP_I400;
    }
    if csp >= X264_CSP_I420 && csp < X264_CSP_I422 {
        return X264_CSP_NV12;
    }
    if csp >= X264_CSP_I422 && csp < X264_CSP_I444 {
        return X264_CSP_NV16;
    }
    if csp >= X264_CSP_I444 && csp <= X264_CSP_RGB {
        return X264_CSP_I444;
    }
    return X264_CSP_NONE;
}
#[c2rust::src_loc = "59:1"]
unsafe extern "C" fn frame_new(
    mut h: *mut x264_t,
    mut b_fdec: ::core::ffi::c_int,
) -> *mut x264_frame_t {
    let mut prealloc_idx: ::core::ffi::c_int = 0;
    let mut prealloc_size: int64_t = 0;
    let mut preallocs: [*mut *mut uint8_t; 1024] = [0 as *mut *mut uint8_t; 1024];
    let mut current_block: u64;
    let mut frame: *mut x264_frame_t = 0 as *mut x264_frame_t;
    let mut i_csp: ::core::ffi::c_int = frame_internal_csp((*h).param.i_csp);
    let mut i_mb_count: ::core::ffi::c_int = (*h).mb.i_mb_count;
    let mut i_stride: ::core::ffi::c_int = 0;
    let mut i_width: ::core::ffi::c_int = 0;
    let mut i_lines: ::core::ffi::c_int = 0;
    let mut luma_plane_count: ::core::ffi::c_int = 0;
    let mut i_padv: ::core::ffi::c_int = PADV << (*h).param.b_interlaced;
    let mut align: ::core::ffi::c_int = NATIVE_ALIGN / SIZEOF_PIXEL;
    if (*h).param.cpu & X264_CPU_CACHELINE_64 as uint32_t != 0
        || (*h).param.cpu & X264_CPU_AVX512 as uint32_t != 0
    {
        align = 64 as ::core::ffi::c_int / SIZEOF_PIXEL;
    } else if (*h).param.cpu & X264_CPU_CACHELINE_32 as uint32_t != 0
        || (*h).param.cpu & X264_CPU_AVX as uint32_t != 0
    {
        align = 32 as ::core::ffi::c_int / SIZEOF_PIXEL;
    } else {
        align = 16 as ::core::ffi::c_int / SIZEOF_PIXEL;
    }
    let mut disalign: ::core::ffi::c_int = ((1 as ::core::ffi::c_int)
        << 10 as ::core::ffi::c_int) / SIZEOF_PIXEL;
    frame = x264_malloc(::core::mem::size_of::<x264_frame_t>() as int64_t)
        as *mut x264_frame_t;
    if !frame.is_null() {
        memset(
            frame as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<x264_frame_t>() as size_t,
        );
        prealloc_idx = 0 as ::core::ffi::c_int;
        prealloc_size = 0 as int64_t;
        preallocs = [0 as *mut *mut uint8_t; 1024];
        i_width = (*h).mb.i_mb_width * 16 as ::core::ffi::c_int;
        i_lines = (*h).mb.i_mb_height * 16 as ::core::ffi::c_int;
        i_stride = align_stride(
            i_width
                + ((if 32 as ::core::ffi::c_int
                    > 64 as ::core::ffi::c_int
                        / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                {
                    32 as ::core::ffi::c_int
                } else {
                    64 as ::core::ffi::c_int
                        / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                }) + PADH),
            align,
            disalign,
        );
        if i_csp == X264_CSP_NV12 || i_csp == X264_CSP_NV16 {
            luma_plane_count = 1 as ::core::ffi::c_int;
            (*frame).i_plane = 2 as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 2 as ::core::ffi::c_int {
                (*frame).i_width[i as usize] = i_width >> i;
                (*frame).i_lines[i as usize] = i_lines
                    >> (i != 0 && i_csp == X264_CSP_NV12) as ::core::ffi::c_int;
                (*frame).i_stride[i as usize] = i_stride;
                i += 1;
            }
            current_block = 7245201122033322888;
        } else if i_csp == X264_CSP_I444 {
            luma_plane_count = 3 as ::core::ffi::c_int;
            (*frame).i_plane = 3 as ::core::ffi::c_int;
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 3 as ::core::ffi::c_int {
                (*frame).i_width[i_0 as usize] = i_width;
                (*frame).i_lines[i_0 as usize] = i_lines;
                (*frame).i_stride[i_0 as usize] = i_stride;
                i_0 += 1;
            }
            current_block = 7245201122033322888;
        } else if i_csp == X264_CSP_I400 {
            luma_plane_count = 1 as ::core::ffi::c_int;
            (*frame).i_plane = 1 as ::core::ffi::c_int;
            (*frame).i_width[0 as ::core::ffi::c_int as usize] = i_width;
            (*frame).i_lines[0 as ::core::ffi::c_int as usize] = i_lines;
            (*frame).i_stride[0 as ::core::ffi::c_int as usize] = i_stride;
            current_block = 7245201122033322888;
        } else {
            current_block = 18021720757857092697;
        }
        match current_block {
            18021720757857092697 => {}
            _ => {
                (*frame).i_csp = i_csp;
                (*frame).i_width_lowres = (*frame)
                    .i_width[0 as ::core::ffi::c_int as usize] / 2 as ::core::ffi::c_int;
                (*frame).i_lines_lowres = (*frame)
                    .i_lines[0 as ::core::ffi::c_int as usize] / 2 as ::core::ffi::c_int;
                (*frame).i_stride_lowres = align_stride(
                    (*frame).i_width_lowres
                        + ((if 32 as ::core::ffi::c_int
                            > 64 as ::core::ffi::c_int
                                / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                        {
                            32 as ::core::ffi::c_int
                        } else {
                            64 as ::core::ffi::c_int
                                / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                        }) + PADH),
                    align,
                    disalign << 1 as ::core::ffi::c_int,
                );
                let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_1 < (*h).param.i_bframe + 2 as ::core::ffi::c_int {
                    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while j < (*h).param.i_bframe + 2 as ::core::ffi::c_int {
                        (*frame).i_row_satds[i_1 as usize][j as usize] = prealloc_size
                            as intptr_t as *mut ::core::ffi::c_void
                            as *mut ::core::ffi::c_int;
                        let fresh8 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh8 as usize] = &mut *(*(*frame)
                            .i_row_satds
                            .as_mut_ptr()
                            .offset(i_1 as isize))
                            .as_mut_ptr()
                            .offset(j as isize) as *mut *mut ::core::ffi::c_int
                            as *mut *mut uint8_t;
                        prealloc_size
                            += ((i_lines / 16 as ::core::ffi::c_int) as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                ) as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                        j += 1;
                    }
                    i_1 += 1;
                }
                (*frame).i_poc = -(1 as ::core::ffi::c_int);
                (*frame).i_type = X264_TYPE_AUTO;
                (*frame).i_qpplus1 = X264_QP_AUTO;
                (*frame).i_pts = -(1 as ::core::ffi::c_int) as int64_t;
                (*frame).i_frame = -(1 as ::core::ffi::c_int);
                (*frame).i_frame_num = -(1 as ::core::ffi::c_int);
                (*frame).i_lines_completed = -(1 as ::core::ffi::c_int);
                (*frame).b_fdec = b_fdec as uint8_t;
                (*frame).i_pic_struct = PIC_STRUCT_AUTO as ::core::ffi::c_int;
                (*frame).i_field_cnt = -(1 as ::core::ffi::c_int) as int64_t;
                (*frame).i_cpb_delay = 0 as int64_t;
                (*frame).i_dpb_output_delay = (*frame).i_cpb_delay;
                (*frame).i_cpb_duration = (*frame).i_dpb_output_delay;
                (*frame).i_duration = (*frame).i_cpb_duration;
                (*frame).i_cpb_delay_lookahead = -(1 as ::core::ffi::c_int) as int64_t;
                (*frame).i_coded_fields_lookahead = (*frame).i_cpb_delay_lookahead;
                (*frame).orig = frame as *mut x264_frame;
                if i_csp == X264_CSP_NV12 || i_csp == X264_CSP_NV16 {
                    let mut chroma_padv: ::core::ffi::c_int = i_padv
                        >> (i_csp == X264_CSP_NV12) as ::core::ffi::c_int;
                    let mut chroma_plane_size: ::core::ffi::c_int = (*frame)
                        .i_stride[1 as ::core::ffi::c_int as usize]
                        * ((*frame).i_lines[1 as ::core::ffi::c_int as usize]
                            + 2 as ::core::ffi::c_int * chroma_padv);
                    (*frame).buffer[1 as ::core::ffi::c_int as usize] = prealloc_size
                        as intptr_t as *mut ::core::ffi::c_void as *mut pixel;
                    let fresh9 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh9 as usize] = &mut *(*frame)
                        .buffer
                        .as_mut_ptr()
                        .offset(1 as ::core::ffi::c_int as isize) as *mut *mut pixel
                        as *mut *mut uint8_t;
                    prealloc_size
                        += (chroma_plane_size
                            * ::core::mem::size_of::<pixel>() as ::core::ffi::c_int)
                            as int64_t
                            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t
                            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t;
                    if (*h).param.b_interlaced != 0 {
                        (*frame).buffer_fld[1 as ::core::ffi::c_int as usize] = prealloc_size
                            as intptr_t as *mut ::core::ffi::c_void as *mut pixel;
                        let fresh10 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh10 as usize] = &mut *(*frame)
                            .buffer_fld
                            .as_mut_ptr()
                            .offset(1 as ::core::ffi::c_int as isize) as *mut *mut pixel
                            as *mut *mut uint8_t;
                        prealloc_size
                            += (chroma_plane_size
                                * ::core::mem::size_of::<pixel>() as ::core::ffi::c_int)
                                as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                    }
                }
                let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while p < luma_plane_count {
                    let mut luma_plane_size: int64_t = align_plane_size(
                        (*frame).i_stride[p as usize]
                            * ((*frame).i_lines[p as usize]
                                + 2 as ::core::ffi::c_int * i_padv),
                        disalign,
                    ) as int64_t;
                    if (*h).param.analyse.i_subpel_refine != 0 && b_fdec != 0 {
                        luma_plane_size *= 4 as int64_t;
                    }
                    (*frame).buffer[p as usize] = prealloc_size as intptr_t
                        as *mut ::core::ffi::c_void as *mut pixel;
                    let fresh11 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh11 as usize] = &mut *(*frame)
                        .buffer
                        .as_mut_ptr()
                        .offset(p as isize) as *mut *mut pixel as *mut *mut uint8_t;
                    prealloc_size
                        += luma_plane_size
                            * ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                as int64_t
                            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t
                            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t;
                    if (*h).param.b_interlaced != 0 {
                        (*frame).buffer_fld[p as usize] = prealloc_size as intptr_t
                            as *mut ::core::ffi::c_void as *mut pixel;
                        let fresh12 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh12 as usize] = &mut *(*frame)
                            .buffer_fld
                            .as_mut_ptr()
                            .offset(p as isize) as *mut *mut pixel as *mut *mut uint8_t;
                        prealloc_size
                            += luma_plane_size
                                * ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                    }
                    p += 1;
                }
                (*frame).b_duplicate = 0 as ::core::ffi::c_int;
                if b_fdec != 0 {
                    (*frame).mb_type = prealloc_size as intptr_t
                        as *mut ::core::ffi::c_void as *mut int8_t;
                    let fresh13 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh13 as usize] = &mut (*frame).mb_type
                        as *mut *mut int8_t as *mut *mut uint8_t;
                    prealloc_size
                        += (i_mb_count as usize)
                            .wrapping_mul(::core::mem::size_of::<int8_t>() as usize)
                            as int64_t
                            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t
                            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t;
                    (*frame).mb_partition = prealloc_size as intptr_t
                        as *mut ::core::ffi::c_void as *mut uint8_t;
                    let fresh14 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh14 as usize] = &mut (*frame).mb_partition
                        as *mut *mut uint8_t;
                    prealloc_size
                        += (i_mb_count as usize)
                            .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                            as int64_t
                            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t
                            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t;
                    (*frame).mv[0 as ::core::ffi::c_int as usize] = prealloc_size
                        as intptr_t as *mut ::core::ffi::c_void as *mut [int16_t; 2];
                    let fresh15 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh15 as usize] = &mut *(*frame)
                        .mv
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut *mut [int16_t; 2] as *mut *mut uint8_t;
                    prealloc_size
                        += ((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                            * i_mb_count) as usize)
                            .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                            as int64_t
                            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t
                            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t;
                    (*frame).mv16x16 = prealloc_size as intptr_t
                        as *mut ::core::ffi::c_void as *mut [int16_t; 2];
                    let fresh16 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh16 as usize] = &mut (*frame).mv16x16
                        as *mut *mut [int16_t; 2] as *mut *mut uint8_t;
                    prealloc_size
                        += ((2 as ::core::ffi::c_int
                            * (i_mb_count + 1 as ::core::ffi::c_int)) as usize)
                            .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                            as int64_t
                            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t
                            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t;
                    (*frame).ref_0[0 as ::core::ffi::c_int as usize] = prealloc_size
                        as intptr_t as *mut ::core::ffi::c_void as *mut int8_t;
                    let fresh17 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh17 as usize] = &mut *(*frame)
                        .ref_0
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize) as *mut *mut int8_t
                        as *mut *mut uint8_t;
                    prealloc_size
                        += ((4 as ::core::ffi::c_int * i_mb_count) as usize)
                            .wrapping_mul(::core::mem::size_of::<int8_t>() as usize)
                            as int64_t
                            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t
                            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t;
                    if (*h).param.i_bframe != 0 {
                        (*frame).mv[1 as ::core::ffi::c_int as usize] = prealloc_size
                            as intptr_t as *mut ::core::ffi::c_void as *mut [int16_t; 2];
                        let fresh18 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh18 as usize] = &mut *(*frame)
                            .mv
                            .as_mut_ptr()
                            .offset(1 as ::core::ffi::c_int as isize)
                            as *mut *mut [int16_t; 2] as *mut *mut uint8_t;
                        prealloc_size
                            += ((2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                * i_mb_count) as usize)
                                .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                                as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                        (*frame).ref_0[1 as ::core::ffi::c_int as usize] = prealloc_size
                            as intptr_t as *mut ::core::ffi::c_void as *mut int8_t;
                        let fresh19 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh19 as usize] = &mut *(*frame)
                            .ref_0
                            .as_mut_ptr()
                            .offset(1 as ::core::ffi::c_int as isize) as *mut *mut int8_t
                            as *mut *mut uint8_t;
                        prealloc_size
                            += ((4 as ::core::ffi::c_int * i_mb_count) as usize)
                                .wrapping_mul(::core::mem::size_of::<int8_t>() as usize)
                                as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                    } else {
                        (*frame).mv[1 as ::core::ffi::c_int as usize] = 0
                            as *mut [int16_t; 2];
                        (*frame).ref_0[1 as ::core::ffi::c_int as usize] = 0
                            as *mut int8_t;
                    }
                    (*frame).i_row_bits = prealloc_size as intptr_t
                        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_int;
                    let fresh20 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh20 as usize] = &mut (*frame).i_row_bits
                        as *mut *mut ::core::ffi::c_int as *mut *mut uint8_t;
                    prealloc_size
                        += ((i_lines / 16 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            ) as int64_t
                            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t
                            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t;
                    (*frame).f_row_qp = prealloc_size as intptr_t
                        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_float;
                    let fresh21 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh21 as usize] = &mut (*frame).f_row_qp
                        as *mut *mut ::core::ffi::c_float as *mut *mut uint8_t;
                    prealloc_size
                        += ((i_lines / 16 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_float>() as usize,
                            ) as int64_t
                            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t
                            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t;
                    (*frame).f_row_qscale = prealloc_size as intptr_t
                        as *mut ::core::ffi::c_void as *mut ::core::ffi::c_float;
                    let fresh22 = prealloc_idx;
                    prealloc_idx = prealloc_idx + 1;
                    preallocs[fresh22 as usize] = &mut (*frame).f_row_qscale
                        as *mut *mut ::core::ffi::c_float as *mut *mut uint8_t;
                    prealloc_size
                        += ((i_lines / 16 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_float>() as usize,
                            ) as int64_t
                            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t
                            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                as int64_t;
                    if (*h).param.analyse.i_me_method >= X264_ME_ESA {
                        (*frame).buffer[3 as ::core::ffi::c_int as usize] = prealloc_size
                            as intptr_t as *mut ::core::ffi::c_void as *mut pixel;
                        let fresh23 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh23 as usize] = &mut *(*frame)
                            .buffer
                            .as_mut_ptr()
                            .offset(3 as ::core::ffi::c_int as isize) as *mut *mut pixel
                            as *mut *mut uint8_t;
                        prealloc_size
                            += ((((*frame).i_stride[0 as ::core::ffi::c_int as usize]
                                * ((*frame).i_lines[0 as ::core::ffi::c_int as usize]
                                    + 2 as ::core::ffi::c_int * i_padv)) as usize)
                                .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize)
                                << (*h).frames.b_have_sub8x8_esa) as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                    }
                    if (*h).param.b_interlaced != 0 {
                        (*frame).field = prealloc_size as intptr_t
                            as *mut ::core::ffi::c_void as *mut uint8_t;
                        let fresh24 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh24 as usize] = &mut (*frame).field
                            as *mut *mut uint8_t;
                        prealloc_size
                            += (i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                    }
                    if (*h).param.analyse.b_mb_info != 0 {
                        (*frame).effective_qp = prealloc_size as intptr_t
                            as *mut ::core::ffi::c_void as *mut uint8_t;
                        let fresh25 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh25 as usize] = &mut (*frame).effective_qp
                            as *mut *mut uint8_t;
                        prealloc_size
                            += (i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize)
                                as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                    }
                } else {
                    if (*h).frames.b_have_lowres != 0 {
                        let mut luma_plane_size_0: int64_t = align_plane_size(
                            (*frame).i_stride_lowres
                                * ((*frame).i_lines[0 as ::core::ffi::c_int as usize]
                                    / 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * PADV),
                            disalign,
                        ) as int64_t;
                        (*frame).buffer_lowres = prealloc_size as intptr_t
                            as *mut ::core::ffi::c_void as *mut pixel;
                        let fresh26 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh26 as usize] = &mut (*frame).buffer_lowres
                            as *mut *mut pixel as *mut *mut uint8_t;
                        prealloc_size
                            += 4 as int64_t * luma_plane_size_0
                                * ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                        let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while j_0 <= ((*h).param.i_bframe != 0) as ::core::ffi::c_int {
                            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_2 <= (*h).param.i_bframe {
                                (*frame).lowres_mvs[j_0 as usize][i_2 as usize] = prealloc_size
                                    as intptr_t as *mut ::core::ffi::c_void
                                    as *mut [int16_t; 2];
                                let fresh27 = prealloc_idx;
                                prealloc_idx = prealloc_idx + 1;
                                preallocs[fresh27 as usize] = &mut *(*(*frame)
                                    .lowres_mvs
                                    .as_mut_ptr()
                                    .offset(j_0 as isize))
                                    .as_mut_ptr()
                                    .offset(i_2 as isize) as *mut *mut [int16_t; 2]
                                    as *mut *mut uint8_t;
                                prealloc_size
                                    += ((2 as ::core::ffi::c_int * i_mb_count) as usize)
                                        .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                                        as int64_t
                                        + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                            as int64_t
                                        & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                            as int64_t;
                                (*frame).lowres_mv_costs[j_0 as usize][i_2 as usize] = prealloc_size
                                    as intptr_t as *mut ::core::ffi::c_void
                                    as *mut ::core::ffi::c_int;
                                let fresh28 = prealloc_idx;
                                prealloc_idx = prealloc_idx + 1;
                                preallocs[fresh28 as usize] = &mut *(*(*frame)
                                    .lowres_mv_costs
                                    .as_mut_ptr()
                                    .offset(j_0 as isize))
                                    .as_mut_ptr()
                                    .offset(i_2 as isize) as *mut *mut ::core::ffi::c_int
                                    as *mut *mut uint8_t;
                                prealloc_size
                                    += (i_mb_count as usize)
                                        .wrapping_mul(
                                            ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                                        ) as int64_t
                                        + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                            as int64_t
                                        & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                            as int64_t;
                                i_2 += 1;
                            }
                            j_0 += 1;
                        }
                        (*frame).i_propagate_cost = prealloc_size as intptr_t
                            as *mut ::core::ffi::c_void as *mut uint16_t;
                        let fresh29 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh29 as usize] = &mut (*frame).i_propagate_cost
                            as *mut *mut uint16_t as *mut *mut uint8_t;
                        prealloc_size
                            += (i_mb_count as usize)
                                .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize)
                                as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                        let mut j_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while j_1 <= (*h).param.i_bframe + 1 as ::core::ffi::c_int {
                            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_3 <= (*h).param.i_bframe + 1 as ::core::ffi::c_int {
                                (*frame).lowres_costs[j_1 as usize][i_3 as usize] = prealloc_size
                                    as intptr_t as *mut ::core::ffi::c_void as *mut uint16_t;
                                let fresh30 = prealloc_idx;
                                prealloc_idx = prealloc_idx + 1;
                                preallocs[fresh30 as usize] = &mut *(*(*frame)
                                    .lowres_costs
                                    .as_mut_ptr()
                                    .offset(j_1 as isize))
                                    .as_mut_ptr()
                                    .offset(i_3 as isize) as *mut *mut uint16_t
                                    as *mut *mut uint8_t;
                                prealloc_size
                                    += (i_mb_count as usize)
                                        .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize)
                                        as int64_t
                                        + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                            as int64_t
                                        & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                            as int64_t;
                                i_3 += 1;
                            }
                            j_1 += 1;
                        }
                    }
                    if (*h).param.rc.i_aq_mode != 0 {
                        (*frame).f_qp_offset = prealloc_size as intptr_t
                            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_float;
                        let fresh31 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh31 as usize] = &mut (*frame).f_qp_offset
                            as *mut *mut ::core::ffi::c_float as *mut *mut uint8_t;
                        prealloc_size
                            += (i_mb_count as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_float>() as usize,
                                ) as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                        (*frame).f_qp_offset_aq = prealloc_size as intptr_t
                            as *mut ::core::ffi::c_void as *mut ::core::ffi::c_float;
                        let fresh32 = prealloc_idx;
                        prealloc_idx = prealloc_idx + 1;
                        preallocs[fresh32 as usize] = &mut (*frame).f_qp_offset_aq
                            as *mut *mut ::core::ffi::c_float as *mut *mut uint8_t;
                        prealloc_size
                            += (i_mb_count as usize)
                                .wrapping_mul(
                                    ::core::mem::size_of::<::core::ffi::c_float>() as usize,
                                ) as int64_t
                                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t
                                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                    as int64_t;
                        if (*h).frames.b_have_lowres != 0 {
                            (*frame).i_inv_qscale_factor = prealloc_size as intptr_t
                                as *mut ::core::ffi::c_void as *mut uint16_t;
                            let fresh33 = prealloc_idx;
                            prealloc_idx = prealloc_idx + 1;
                            preallocs[fresh33 as usize] = &mut (*frame)
                                .i_inv_qscale_factor as *mut *mut uint16_t
                                as *mut *mut uint8_t;
                            prealloc_size
                                += (i_mb_count as usize)
                                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as usize)
                                    as int64_t
                                    + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                        as int64_t
                                    & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                                        as int64_t;
                        }
                    }
                    if (*h).frames.b_have_lowres != 0 {
                        prealloc_size += NATIVE_ALIGN as int64_t;
                    }
                }
                (*frame).base = x264_malloc(prealloc_size) as *mut uint8_t;
                if !(*frame).base.is_null() {
                    loop {
                        let fresh34 = prealloc_idx;
                        prealloc_idx = prealloc_idx - 1;
                        if !(fresh34 != 0) {
                            break;
                        }
                        *preallocs[prealloc_idx as usize] = (*preallocs[prealloc_idx
                            as usize] as intptr_t + (*frame).base as intptr_t)
                            as *mut uint8_t;
                    }
                    if i_csp == X264_CSP_NV12 || i_csp == X264_CSP_NV16 {
                        let mut chroma_padv_0: ::core::ffi::c_int = i_padv
                            >> (i_csp == X264_CSP_NV12) as ::core::ffi::c_int;
                        (*frame).plane[1 as ::core::ffi::c_int as usize] = (*frame)
                            .buffer[1 as ::core::ffi::c_int as usize]
                            .offset(
                                ((*frame).i_stride[1 as ::core::ffi::c_int as usize]
                                    * chroma_padv_0) as isize,
                            )
                            .offset(
                                (if 32 as ::core::ffi::c_int
                                    > 64 as ::core::ffi::c_int
                                        / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                {
                                    32 as ::core::ffi::c_int
                                } else {
                                    64 as ::core::ffi::c_int
                                        / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                }) as isize,
                            );
                        if (*h).param.b_interlaced != 0 {
                            (*frame).plane_fld[1 as ::core::ffi::c_int as usize] = (*frame)
                                .buffer_fld[1 as ::core::ffi::c_int as usize]
                                .offset(
                                    ((*frame).i_stride[1 as ::core::ffi::c_int as usize]
                                        * chroma_padv_0) as isize,
                                )
                                .offset(
                                    (if 32 as ::core::ffi::c_int
                                        > 64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    {
                                        32 as ::core::ffi::c_int
                                    } else {
                                        64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    }) as isize,
                                );
                        }
                    }
                    let mut p_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while p_0 < luma_plane_count {
                        let mut luma_plane_size_1: int64_t = align_plane_size(
                            (*frame).i_stride[p_0 as usize]
                                * ((*frame).i_lines[p_0 as usize]
                                    + 2 as ::core::ffi::c_int * i_padv),
                            disalign,
                        ) as int64_t;
                        if (*h).param.analyse.i_subpel_refine != 0 && b_fdec != 0 {
                            let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_4 < 4 as ::core::ffi::c_int {
                                (*frame).filtered[p_0 as usize][i_4 as usize] = (*frame)
                                    .buffer[p_0 as usize]
                                    .offset((i_4 as int64_t * luma_plane_size_1) as isize)
                                    .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                    .offset(
                                        (if 32 as ::core::ffi::c_int
                                            > 64 as ::core::ffi::c_int
                                                / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                        {
                                            32 as ::core::ffi::c_int
                                        } else {
                                            64 as ::core::ffi::c_int
                                                / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                        }) as isize,
                                    );
                                if (*h).param.b_interlaced != 0 {
                                    (*frame).filtered_fld[p_0 as usize][i_4 as usize] = (*frame)
                                        .buffer_fld[p_0 as usize]
                                        .offset((i_4 as int64_t * luma_plane_size_1) as isize)
                                        .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                        .offset(
                                            (if 32 as ::core::ffi::c_int
                                                > 64 as ::core::ffi::c_int
                                                    / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                            {
                                                32 as ::core::ffi::c_int
                                            } else {
                                                64 as ::core::ffi::c_int
                                                    / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                            }) as isize,
                                        );
                                }
                                i_4 += 1;
                            }
                            (*frame).plane[p_0 as usize] = (*frame)
                                .filtered[p_0 as usize][0 as ::core::ffi::c_int as usize];
                            (*frame).plane_fld[p_0 as usize] = (*frame)
                                .filtered_fld[p_0
                                as usize][0 as ::core::ffi::c_int as usize];
                        } else {
                            (*frame).plane[p_0 as usize] = (*frame)
                                .buffer[p_0 as usize]
                                .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                .offset(
                                    (if 32 as ::core::ffi::c_int
                                        > 64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    {
                                        32 as ::core::ffi::c_int
                                    } else {
                                        64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    }) as isize,
                                );
                            (*frame)
                                .filtered[p_0 as usize][0 as ::core::ffi::c_int as usize] = (*frame)
                                .plane[p_0 as usize];
                            if (*h).param.b_interlaced != 0 {
                                (*frame).plane_fld[p_0 as usize] = (*frame)
                                    .buffer_fld[p_0 as usize]
                                    .offset(((*frame).i_stride[p_0 as usize] * i_padv) as isize)
                                    .offset(
                                        (if 32 as ::core::ffi::c_int
                                            > 64 as ::core::ffi::c_int
                                                / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                        {
                                            32 as ::core::ffi::c_int
                                        } else {
                                            64 as ::core::ffi::c_int
                                                / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                        }) as isize,
                                    );
                                (*frame)
                                    .filtered_fld[p_0
                                    as usize][0 as ::core::ffi::c_int as usize] = (*frame)
                                    .plane_fld[p_0 as usize];
                            }
                        }
                        p_0 += 1;
                    }
                    if b_fdec != 0 {
                        (*((*(*frame).mv16x16.offset(0 as ::core::ffi::c_int as isize))
                            .as_mut_ptr() as *mut x264_union32_t))
                            .i = 0 as uint32_t;
                        (*frame).mv16x16 = (*frame).mv16x16.offset(1);
                        if (*h).param.analyse.i_me_method >= X264_ME_ESA {
                            (*frame).integral = ((*frame)
                                .buffer[3 as ::core::ffi::c_int as usize] as *mut uint16_t)
                                .offset(
                                    ((*frame).i_stride[0 as ::core::ffi::c_int as usize]
                                        * i_padv) as isize,
                                )
                                .offset(
                                    (if 32 as ::core::ffi::c_int
                                        > 64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    {
                                        32 as ::core::ffi::c_int
                                    } else {
                                        64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    }) as isize,
                                );
                        }
                    } else if (*h).frames.b_have_lowres != 0 {
                        let mut luma_plane_size_2: int64_t = align_plane_size(
                            (*frame).i_stride_lowres
                                * ((*frame).i_lines[0 as ::core::ffi::c_int as usize]
                                    / 2 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * PADV),
                            disalign,
                        ) as int64_t;
                        let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_5 < 4 as ::core::ffi::c_int {
                            (*frame).lowres[i_5 as usize] = (*frame)
                                .buffer_lowres
                                .offset(((*frame).i_stride_lowres * PADV) as isize)
                                .offset(
                                    (if 32 as ::core::ffi::c_int
                                        > 64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    {
                                        32 as ::core::ffi::c_int
                                    } else {
                                        64 as ::core::ffi::c_int
                                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                                    }) as isize,
                                )
                                .offset((i_5 as int64_t * luma_plane_size_2) as isize);
                            i_5 += 1;
                        }
                        let mut j_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while j_2 <= ((*h).param.i_bframe != 0) as ::core::ffi::c_int {
                            let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_6 <= (*h).param.i_bframe {
                                memset(
                                    (*frame).lowres_mvs[j_2 as usize][i_6 as usize]
                                        as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    ((2 as ::core::ffi::c_int * i_mb_count) as size_t)
                                        .wrapping_mul(::core::mem::size_of::<int16_t>() as size_t),
                                );
                                i_6 += 1;
                            }
                            j_2 += 1;
                        }
                        (*frame).i_intra_cost = (*frame)
                            .lowres_costs[0 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int as usize] as *mut uint16_t;
                        memset(
                            (*frame).i_intra_cost as *mut ::core::ffi::c_void,
                            -(1 as ::core::ffi::c_int),
                            (i_mb_count as size_t)
                                .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
                        );
                        if (*h).param.rc.i_aq_mode != 0 {
                            memset(
                                (*frame).i_inv_qscale_factor as *mut ::core::ffi::c_void,
                                0 as ::core::ffi::c_int,
                                (i_mb_count as size_t)
                                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
                            );
                        }
                    }
                    if !(pthread_mutex_init(
                        &mut (*frame).mutex,
                        0 as *const pthread_mutexattr_t,
                    ) != 0)
                    {
                        if !(pthread_cond_init(
                            &mut (*frame).cv,
                            0 as *const pthread_condattr_t,
                        ) != 0)
                        {
                            return frame;
                        }
                    }
                }
            }
        }
    }
    x264_free(frame as *mut ::core::ffi::c_void);
    return 0 as *mut x264_frame_t;
}
#[no_mangle]
#[c2rust::src_loc = "312:1"]
pub unsafe extern "C" fn x264_10_frame_delete(mut frame: *mut x264_frame_t) {
    if (*frame).b_duplicate == 0 {
        x264_free((*frame).base as *mut ::core::ffi::c_void);
        if !(*frame).param.is_null() && (*(*frame).param).param_free.is_some() {
            x264_param_cleanup((*frame).param);
            (*(*frame).param)
                .param_free
                .expect(
                    "non-null function pointer",
                )((*frame).param as *mut ::core::ffi::c_void);
        }
        if (*frame).mb_info_free.is_some() {
            (*frame)
                .mb_info_free
                .expect(
                    "non-null function pointer",
                )((*frame).mb_info as *mut ::core::ffi::c_void);
        }
        if (*frame).extra_sei.sei_free.is_some() {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*frame).extra_sei.num_payloads {
                (*frame)
                    .extra_sei
                    .sei_free
                    .expect(
                        "non-null function pointer",
                    )(
                    (*(*frame).extra_sei.payloads.offset(i as isize)).payload
                        as *mut ::core::ffi::c_void,
                );
                i += 1;
            }
            (*frame)
                .extra_sei
                .sei_free
                .expect(
                    "non-null function pointer",
                )((*frame).extra_sei.payloads as *mut ::core::ffi::c_void);
        }
        pthread_mutex_destroy(&mut (*frame).mutex);
        pthread_cond_destroy(&mut (*frame).cv);
    }
    x264_free(frame as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "342:1"]
unsafe extern "C" fn get_plane_ptr(
    mut h: *mut x264_t,
    mut src: *mut x264_picture_t,
    mut pix: *mut *mut uint8_t,
    mut stride: *mut ::core::ffi::c_int,
    mut plane: ::core::ffi::c_int,
    mut xshift: ::core::ffi::c_int,
    mut yshift: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut width: ::core::ffi::c_int = (*h).param.i_width >> xshift;
    let mut height: ::core::ffi::c_int = (*h).param.i_height >> yshift;
    *pix = (*src).img.plane[plane as usize];
    *stride = (*src).img.i_stride[plane as usize];
    if (*src).img.i_csp & X264_CSP_VFLIP != 0 {
        *pix = (*pix).offset(((height - 1 as ::core::ffi::c_int) * *stride) as isize);
        *stride = -*stride;
    }
    if width > abs(*stride) {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"Input picture width (%d) is greater than stride (%d)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            width,
            *stride,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "363:1"]
pub unsafe extern "C" fn x264_10_frame_copy_picture(
    mut h: *mut x264_t,
    mut dst: *mut x264_frame_t,
    mut src: *mut x264_picture_t,
) -> ::core::ffi::c_int {
    let mut i_csp: ::core::ffi::c_int = (*src).img.i_csp & X264_CSP_MASK;
    if (*dst).i_csp != frame_internal_csp(i_csp) {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"Invalid input colorspace\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*src).img.i_csp & X264_CSP_HIGH_DEPTH == 0 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"This build of x264 requires high depth input. Rebuild to support 8-bit input.\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if BIT_DEPTH != 10 as ::core::ffi::c_int && i_csp == X264_CSP_V210 {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"v210 input is only compatible with bit-depth of 10 bits\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if (*src).i_type < X264_TYPE_AUTO || (*src).i_type > X264_TYPE_KEYFRAME {
        x264_10_log(
            h,
            X264_LOG_WARNING,
            b"forced frame type (%d) at %d is unknown\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*src).i_type,
            (*h).frames.i_input,
        );
        (*dst).i_forced_type = X264_TYPE_AUTO;
    } else {
        (*dst).i_forced_type = (*src).i_type;
    }
    (*dst).i_type = (*dst).i_forced_type;
    (*dst).i_qpplus1 = (*src).i_qpplus1;
    (*dst).i_reordered_pts = (*src).i_pts;
    (*dst).i_pts = (*dst).i_reordered_pts;
    (*dst).param = (*src).param;
    (*dst).i_pic_struct = (*src).i_pic_struct;
    (*dst).extra_sei = (*src).extra_sei;
    (*dst).opaque = (*src).opaque;
    (*dst).mb_info = if (*h).param.analyse.b_mb_info != 0 {
        (*src).prop.mb_info
    } else {
        0 as *mut uint8_t
    };
    (*dst).mb_info_free = if (*h).param.analyse.b_mb_info != 0 {
        (*src).prop.mb_info_free
    } else {
        None
    };
    let mut pix: [*mut uint8_t; 3] = [0 as *mut uint8_t; 3];
    let mut stride: [::core::ffi::c_int; 3] = [0; 3];
    if i_csp == X264_CSP_YUYV || i_csp == X264_CSP_UYVY {
        let mut p: ::core::ffi::c_int = (i_csp == X264_CSP_UYVY) as ::core::ffi::c_int;
        (*h)
            .mc
            .plane_copy_deinterleave_yuyv
            .expect(
                "non-null function pointer",
            )(
            (*dst).plane[p as usize],
            (*dst).i_stride[p as usize] as intptr_t,
            (*dst).plane[(p ^ 1 as ::core::ffi::c_int) as usize],
            (*dst).i_stride[(p ^ 1 as ::core::ffi::c_int) as usize] as intptr_t,
            (*src).img.plane[0 as ::core::ffi::c_int as usize] as *mut pixel,
            ((*src).img.i_stride[0 as ::core::ffi::c_int as usize] / SIZEOF_PIXEL)
                as intptr_t,
            (*h).param.i_width,
            (*h).param.i_height,
        );
    } else if i_csp == X264_CSP_V210 {
        stride[0 as ::core::ffi::c_int as usize] = (*src)
            .img
            .i_stride[0 as ::core::ffi::c_int as usize];
        pix[0 as ::core::ffi::c_int as usize] = (*src)
            .img
            .plane[0 as ::core::ffi::c_int as usize];
        (*h)
            .mc
            .plane_copy_deinterleave_v210
            .expect(
                "non-null function pointer",
            )(
            (*dst).plane[0 as ::core::ffi::c_int as usize],
            (*dst).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
            (*dst).plane[1 as ::core::ffi::c_int as usize],
            (*dst).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
            pix[0 as ::core::ffi::c_int as usize] as *mut uint32_t,
            (stride[0 as ::core::ffi::c_int as usize]
                / ::core::mem::size_of::<uint32_t>() as ::core::ffi::c_int) as intptr_t,
            (*h).param.i_width,
            (*h).param.i_height,
        );
    } else if i_csp >= X264_CSP_BGR {
        stride[0 as ::core::ffi::c_int as usize] = (*src)
            .img
            .i_stride[0 as ::core::ffi::c_int as usize];
        pix[0 as ::core::ffi::c_int as usize] = (*src)
            .img
            .plane[0 as ::core::ffi::c_int as usize];
        if (*src).img.i_csp & X264_CSP_VFLIP != 0 {
            pix[0 as ::core::ffi::c_int as usize] = pix[0 as ::core::ffi::c_int as usize]
                .offset(
                    (((*h).param.i_height - 1 as ::core::ffi::c_int)
                        * stride[0 as ::core::ffi::c_int as usize]) as isize,
                );
            stride[0 as ::core::ffi::c_int as usize] = -stride[0 as ::core::ffi::c_int
                as usize];
        }
        let mut b: ::core::ffi::c_int = (i_csp == X264_CSP_RGB) as ::core::ffi::c_int;
        (*h)
            .mc
            .plane_copy_deinterleave_rgb
            .expect(
                "non-null function pointer",
            )(
            (*dst).plane[(1 as ::core::ffi::c_int + b) as usize],
            (*dst).i_stride[(1 as ::core::ffi::c_int + b) as usize] as intptr_t,
            (*dst).plane[0 as ::core::ffi::c_int as usize],
            (*dst).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
            (*dst).plane[(2 as ::core::ffi::c_int - b) as usize],
            (*dst).i_stride[(2 as ::core::ffi::c_int - b) as usize] as intptr_t,
            pix[0 as ::core::ffi::c_int as usize] as *mut pixel,
            (stride[0 as ::core::ffi::c_int as usize] / SIZEOF_PIXEL) as intptr_t,
            if i_csp == X264_CSP_BGRA {
                4 as ::core::ffi::c_int
            } else {
                3 as ::core::ffi::c_int
            },
            (*h).param.i_width,
            (*h).param.i_height,
        );
    } else {
        let mut v_shift: ::core::ffi::c_int = (*h).mb.chroma_v_shift;
        if get_plane_ptr(
            h,
            src,
            &mut *pix.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize),
            &mut *stride.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize),
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            return -(1 as ::core::ffi::c_int);
        }
        (*h)
            .mc
            .plane_copy
            .expect(
                "non-null function pointer",
            )(
            (*dst).plane[0 as ::core::ffi::c_int as usize],
            (*dst).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
            pix[0 as ::core::ffi::c_int as usize] as *mut pixel,
            (stride[0 as ::core::ffi::c_int as usize] / SIZEOF_PIXEL) as intptr_t,
            (*h).param.i_width,
            (*h).param.i_height,
        );
        if i_csp == X264_CSP_NV12 || i_csp == X264_CSP_NV16 {
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize),
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                v_shift,
            ) < 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            (*h)
                .mc
                .plane_copy
                .expect(
                    "non-null function pointer",
                )(
                (*dst).plane[1 as ::core::ffi::c_int as usize],
                (*dst).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                pix[1 as ::core::ffi::c_int as usize] as *mut pixel,
                (stride[1 as ::core::ffi::c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                (*h).param.i_width,
                (*h).param.i_height >> v_shift,
            );
        } else if i_csp == X264_CSP_NV21 {
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize),
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                v_shift,
            ) < 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            (*h)
                .mc
                .plane_copy_swap
                .expect(
                    "non-null function pointer",
                )(
                (*dst).plane[1 as ::core::ffi::c_int as usize],
                (*dst).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                pix[1 as ::core::ffi::c_int as usize] as *mut pixel,
                (stride[1 as ::core::ffi::c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                (*h).param.i_width >> 1 as ::core::ffi::c_int,
                (*h).param.i_height >> v_shift,
            );
        } else if i_csp == X264_CSP_I420 || i_csp == X264_CSP_I422
            || i_csp == X264_CSP_YV12 || i_csp == X264_CSP_YV16
        {
            let mut uv_swap: ::core::ffi::c_int = (i_csp == X264_CSP_YV12
                || i_csp == X264_CSP_YV16) as ::core::ffi::c_int;
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize),
                (if uv_swap != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }),
                1 as ::core::ffi::c_int,
                v_shift,
            ) < 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize),
                &mut *stride.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize),
                (if uv_swap != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }),
                1 as ::core::ffi::c_int,
                v_shift,
            ) < 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            (*h)
                .mc
                .plane_copy_interleave
                .expect(
                    "non-null function pointer",
                )(
                (*dst).plane[1 as ::core::ffi::c_int as usize],
                (*dst).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                pix[1 as ::core::ffi::c_int as usize] as *mut pixel,
                (stride[1 as ::core::ffi::c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                pix[2 as ::core::ffi::c_int as usize] as *mut pixel,
                (stride[2 as ::core::ffi::c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                (*h).param.i_width >> 1 as ::core::ffi::c_int,
                (*h).param.i_height >> v_shift,
            );
        } else if i_csp == X264_CSP_I444 || i_csp == X264_CSP_YV24 {
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize),
                &mut *stride.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize),
                (if i_csp == 0xc as ::core::ffi::c_int {
                    1 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }),
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            if get_plane_ptr(
                h,
                src,
                &mut *pix.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize),
                &mut *stride.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize),
                (if i_csp == 0xc as ::core::ffi::c_int {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }),
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
            (*h)
                .mc
                .plane_copy
                .expect(
                    "non-null function pointer",
                )(
                (*dst).plane[1 as ::core::ffi::c_int as usize],
                (*dst).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                pix[1 as ::core::ffi::c_int as usize] as *mut pixel,
                (stride[1 as ::core::ffi::c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                (*h).param.i_width,
                (*h).param.i_height,
            );
            (*h)
                .mc
                .plane_copy
                .expect(
                    "non-null function pointer",
                )(
                (*dst).plane[2 as ::core::ffi::c_int as usize],
                (*dst).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                pix[2 as ::core::ffi::c_int as usize] as *mut pixel,
                (stride[2 as ::core::ffi::c_int as usize] / SIZEOF_PIXEL) as intptr_t,
                (*h).param.i_width,
                (*h).param.i_height,
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
#[c2rust::src_loc = "483:1"]
unsafe extern "C" fn pixel_memset(
    mut dst: *mut pixel,
    mut src: *mut pixel,
    mut len: ::core::ffi::c_int,
    mut size: ::core::ffi::c_int,
) {
    let mut dstp: *mut uint8_t = dst as *mut uint8_t;
    let mut v1: uint32_t = *src as uint32_t;
    let mut v2: uint32_t = if size == 1 as ::core::ffi::c_int {
        v1.wrapping_add(v1 << 8 as ::core::ffi::c_int)
    } else {
        (*(src as *mut x264_union16_t)).i as uint32_t
    };
    let mut v4: uint32_t = if size <= 2 as ::core::ffi::c_int {
        v2.wrapping_add(v2 << 16 as ::core::ffi::c_int)
    } else {
        (*(src as *mut x264_union32_t)).i
    };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    len *= size;
    if dstp as intptr_t as uint64_t & WORD_SIZE.wrapping_sub(1 as uint64_t) != 0 {
        if size <= 2 as ::core::ffi::c_int && dstp as intptr_t & 3 as intptr_t != 0 {
            if size == 1 as ::core::ffi::c_int && dstp as intptr_t & 1 as intptr_t != 0 {
                let fresh0 = i;
                i = i + 1;
                *dstp.offset(fresh0 as isize) = v1 as uint8_t;
            }
            if dstp as intptr_t & 2 as intptr_t != 0 {
                (*(dstp.offset(i as isize) as *mut x264_union16_t)).i = v2 as uint16_t;
                i += 2 as ::core::ffi::c_int;
            }
        }
        if WORD_SIZE == 8 as uint64_t && dstp as intptr_t & 4 as intptr_t != 0 {
            (*(dstp.offset(i as isize) as *mut x264_union32_t)).i = v4;
            i += 4 as ::core::ffi::c_int;
        }
    }
    if WORD_SIZE == 8 as uint64_t {
        let mut v8: uint64_t = (v4 as uint64_t)
            .wrapping_add((v4 as uint64_t) << 32 as ::core::ffi::c_int);
        while i < len - 7 as ::core::ffi::c_int {
            (*(dstp.offset(i as isize) as *mut x264_union64_t)).i = v8;
            i += 8 as ::core::ffi::c_int;
        }
    }
    while i < len - 3 as ::core::ffi::c_int {
        (*(dstp.offset(i as isize) as *mut x264_union32_t)).i = v4;
        i += 4 as ::core::ffi::c_int;
    }
    if size <= 2 as ::core::ffi::c_int {
        if i < len - 1 as ::core::ffi::c_int {
            (*(dstp.offset(i as isize) as *mut x264_union16_t)).i = v2 as uint16_t;
            i += 2 as ::core::ffi::c_int;
        }
        if size == 1 as ::core::ffi::c_int && i != len {
            *dstp.offset(i as isize) = v1 as uint8_t;
        }
    }
}
#[inline(always)]
#[c2rust::src_loc = "535:1"]
unsafe extern "C" fn plane_expand_border(
    mut pix: *mut pixel,
    mut i_stride: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
    mut i_padh: ::core::ffi::c_int,
    mut i_padv: ::core::ffi::c_int,
    mut b_pad_top: ::core::ffi::c_int,
    mut b_pad_bottom: ::core::ffi::c_int,
    mut b_chroma: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < i_height {
        pixel_memset(
            pix.offset(-i_padh as isize).offset((y * i_stride) as isize),
            pix.offset(0 as ::core::ffi::c_int as isize).offset((y * i_stride) as isize),
            i_padh >> b_chroma,
            SIZEOF_PIXEL << b_chroma,
        );
        pixel_memset(
            pix.offset(i_width as isize).offset((y * i_stride) as isize),
            pix
                .offset((i_width - 1 as ::core::ffi::c_int - b_chroma) as isize)
                .offset((y * i_stride) as isize),
            i_padh >> b_chroma,
            SIZEOF_PIXEL << b_chroma,
        );
        y += 1;
    }
    if b_pad_top != 0 {
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < i_padv {
            memcpy(
                pix
                    .offset(-i_padh as isize)
                    .offset(((-y_0 - 1 as ::core::ffi::c_int) * i_stride) as isize)
                    as *mut ::core::ffi::c_void,
                pix
                    .offset(-i_padh as isize)
                    .offset((0 as ::core::ffi::c_int * i_stride) as isize)
                    as *const ::core::ffi::c_void,
                ((i_width + 2 as ::core::ffi::c_int * i_padh) * SIZEOF_PIXEL) as size_t,
            );
            y_0 += 1;
        }
    }
    if b_pad_bottom != 0 {
        let mut y_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_1 < i_padv {
            memcpy(
                pix
                    .offset(-i_padh as isize)
                    .offset(((i_height + y_1) * i_stride) as isize)
                    as *mut ::core::ffi::c_void,
                pix
                    .offset(-i_padh as isize)
                    .offset(((i_height - 1 as ::core::ffi::c_int) * i_stride) as isize)
                    as *const ::core::ffi::c_void,
                ((i_width + 2 as ::core::ffi::c_int * i_padh) * SIZEOF_PIXEL) as size_t,
            );
            y_1 += 1;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "556:1"]
pub unsafe extern "C" fn x264_10_frame_expand_border(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut mb_y: ::core::ffi::c_int,
) {
    let mut pad_top: ::core::ffi::c_int = (mb_y == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    let mut pad_bot: ::core::ffi::c_int = (mb_y
        == (*h).mb.i_mb_height - ((1 as ::core::ffi::c_int) << (*h).sh.b_mbaff))
        as ::core::ffi::c_int;
    let mut b_start: ::core::ffi::c_int = (mb_y == (*h).i_threadslice_start)
        as ::core::ffi::c_int;
    let mut b_end: ::core::ffi::c_int = (mb_y
        == (*h).i_threadslice_end - ((1 as ::core::ffi::c_int) << (*h).sh.b_mbaff))
        as ::core::ffi::c_int;
    if mb_y & (*h).sh.b_mbaff != 0 {
        return;
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*frame).i_plane {
        let mut h_shift: ::core::ffi::c_int = (i != 0 && (*h).mb.chroma_h_shift != 0)
            as ::core::ffi::c_int;
        let mut v_shift: ::core::ffi::c_int = (i != 0 && (*h).mb.chroma_v_shift != 0)
            as ::core::ffi::c_int;
        let mut stride: ::core::ffi::c_int = (*frame).i_stride[i as usize];
        let mut width: ::core::ffi::c_int = 16 as ::core::ffi::c_int
            * (*h).mb.i_mb_width;
        let mut height: ::core::ffi::c_int = (if pad_bot != 0 {
            16 as ::core::ffi::c_int * ((*h).mb.i_mb_height - mb_y) >> (*h).sh.b_mbaff
        } else {
            16 as ::core::ffi::c_int
        }) >> v_shift;
        let mut padh: ::core::ffi::c_int = PADH;
        let mut padv: ::core::ffi::c_int = PADV >> v_shift;
        if b_end != 0 && b_start == 0 {
            height += 4 as ::core::ffi::c_int >> v_shift + (*h).sh.b_mbaff;
        }
        let mut pix: *mut pixel = 0 as *mut pixel;
        let mut starty: ::core::ffi::c_int = 16 as ::core::ffi::c_int * mb_y
            - 4 as ::core::ffi::c_int * (b_start == 0) as ::core::ffi::c_int;
        if (*h).sh.b_mbaff != 0 {
            pix = (*frame)
                .plane_fld[i as usize]
                .offset((starty * stride >> v_shift) as isize);
            plane_expand_border(
                pix,
                stride * 2 as ::core::ffi::c_int,
                width,
                height,
                padh,
                padv,
                pad_top,
                pad_bot,
                h_shift,
            );
            plane_expand_border(
                pix.offset(stride as isize),
                stride * 2 as ::core::ffi::c_int,
                width,
                height,
                padh,
                padv,
                pad_top,
                pad_bot,
                h_shift,
            );
            height = (if pad_bot != 0 {
                16 as ::core::ffi::c_int * ((*h).mb.i_mb_height - mb_y)
            } else {
                32 as ::core::ffi::c_int
            }) >> v_shift;
            if b_end != 0 && b_start == 0 {
                height += 4 as ::core::ffi::c_int >> v_shift;
            }
            pix = (*frame)
                .plane[i as usize]
                .offset((starty * stride >> v_shift) as isize);
            plane_expand_border(
                pix,
                stride,
                width,
                height,
                padh,
                padv,
                pad_top,
                pad_bot,
                h_shift,
            );
        } else {
            pix = (*frame)
                .plane[i as usize]
                .offset((starty * stride >> v_shift) as isize);
            plane_expand_border(
                pix,
                stride,
                width,
                height,
                padh,
                padv,
                pad_top,
                pad_bot,
                h_shift,
            );
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "599:1"]
pub unsafe extern "C" fn x264_10_frame_expand_border_filtered(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut mb_y: ::core::ffi::c_int,
    mut b_end: ::core::ffi::c_int,
) {
    let mut b_start: ::core::ffi::c_int = (mb_y == 0) as ::core::ffi::c_int;
    let mut width: ::core::ffi::c_int = 16 as ::core::ffi::c_int * (*h).mb.i_mb_width
        + 8 as ::core::ffi::c_int;
    let mut height: ::core::ffi::c_int = if b_end != 0 {
        (16 as ::core::ffi::c_int * ((*h).mb.i_mb_height - mb_y) >> (*h).sh.b_mbaff)
            + 16 as ::core::ffi::c_int
    } else {
        16 as ::core::ffi::c_int
    };
    let mut padh: ::core::ffi::c_int = PADH - 4 as ::core::ffi::c_int;
    let mut padv: ::core::ffi::c_int = PADV - 8 as ::core::ffi::c_int;
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while p
        < (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
            == CHROMA_444 as ::core::ffi::c_int
        {
            3 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        })
    {
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let mut stride: ::core::ffi::c_int = (*frame).i_stride[p as usize];
            let mut pix: *mut pixel = 0 as *mut pixel;
            if (*h).sh.b_mbaff != 0 {
                pix = (*frame)
                    .filtered_fld[p as usize][i as usize]
                    .offset(
                        ((16 as ::core::ffi::c_int * mb_y - 16 as ::core::ffi::c_int)
                            * stride) as isize,
                    )
                    .offset(-(4 as ::core::ffi::c_int as isize));
                plane_expand_border(
                    pix,
                    stride * 2 as ::core::ffi::c_int,
                    width,
                    height,
                    padh,
                    padv,
                    b_start,
                    b_end,
                    0 as ::core::ffi::c_int,
                );
                plane_expand_border(
                    pix.offset(stride as isize),
                    stride * 2 as ::core::ffi::c_int,
                    width,
                    height,
                    padh,
                    padv,
                    b_start,
                    b_end,
                    0 as ::core::ffi::c_int,
                );
            }
            pix = (*frame)
                .filtered[p as usize][i as usize]
                .offset(
                    ((16 as ::core::ffi::c_int * mb_y - 8 as ::core::ffi::c_int)
                        * stride) as isize,
                )
                .offset(-(4 as ::core::ffi::c_int as isize));
            plane_expand_border(
                pix,
                stride,
                width,
                height << (*h).sh.b_mbaff,
                padh,
                padv,
                b_start,
                b_end,
                0 as ::core::ffi::c_int,
            );
            i += 1;
        }
        p += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "627:1"]
pub unsafe extern "C" fn x264_10_frame_expand_border_lowres(
    mut frame: *mut x264_frame_t,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        plane_expand_border(
            (*frame).lowres[i as usize],
            (*frame).i_stride_lowres,
            (*frame).i_width_lowres,
            (*frame).i_lines_lowres,
            PADH,
            PADV,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "633:1"]
pub unsafe extern "C" fn x264_10_frame_expand_border_chroma(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut plane: ::core::ffi::c_int,
) {
    let mut v_shift: ::core::ffi::c_int = (*h).mb.chroma_v_shift;
    plane_expand_border(
        (*frame).plane[plane as usize],
        (*frame).i_stride[plane as usize],
        16 as ::core::ffi::c_int * (*h).mb.i_mb_width,
        16 as ::core::ffi::c_int * (*h).mb.i_mb_height >> v_shift,
        PADH,
        PADV >> v_shift,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        (*h).mb.chroma_h_shift,
    );
}
#[no_mangle]
#[c2rust::src_loc = "640:1"]
pub unsafe extern "C" fn x264_10_frame_expand_border_mod16(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*frame).i_plane {
        let mut i_width: ::core::ffi::c_int = (*h).param.i_width;
        let mut h_shift: ::core::ffi::c_int = (i != 0 && (*h).mb.chroma_h_shift != 0)
            as ::core::ffi::c_int;
        let mut v_shift: ::core::ffi::c_int = (i != 0 && (*h).mb.chroma_v_shift != 0)
            as ::core::ffi::c_int;
        let mut i_height: ::core::ffi::c_int = (*h).param.i_height >> v_shift;
        let mut i_padx: ::core::ffi::c_int = (*h).mb.i_mb_width
            * 16 as ::core::ffi::c_int - (*h).param.i_width;
        let mut i_pady: ::core::ffi::c_int = (*h).mb.i_mb_height
            * 16 as ::core::ffi::c_int - (*h).param.i_height >> v_shift;
        if i_padx != 0 {
            let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y < i_height {
                pixel_memset(
                    &mut *(*(*frame).plane.as_mut_ptr().offset(i as isize))
                        .offset(
                            (y * *(*frame).i_stride.as_mut_ptr().offset(i as isize)
                                + i_width) as isize,
                        ),
                    &mut *(*(*frame).plane.as_mut_ptr().offset(i as isize))
                        .offset(
                            (y * *(*frame).i_stride.as_mut_ptr().offset(i as isize)
                                + i_width - 1 as ::core::ffi::c_int - h_shift) as isize,
                        ),
                    i_padx >> h_shift,
                    SIZEOF_PIXEL << h_shift,
                );
                y += 1;
            }
        }
        if i_pady != 0 {
            let mut y_0: ::core::ffi::c_int = i_height;
            while y_0 < i_height + i_pady {
                memcpy(
                    &mut *(*(*frame).plane.as_mut_ptr().offset(i as isize))
                        .offset(
                            (y_0 * *(*frame).i_stride.as_mut_ptr().offset(i as isize))
                                as isize,
                        ) as *mut pixel as *mut ::core::ffi::c_void,
                    &mut *(*(*frame).plane.as_mut_ptr().offset(i as isize))
                        .offset(
                            ((i_height - (!y_0 & (*h).param.b_interlaced)
                                - 1 as ::core::ffi::c_int)
                                * *(*frame).i_stride.as_mut_ptr().offset(i as isize))
                                as isize,
                        ) as *mut pixel as *const ::core::ffi::c_void,
                    ((i_width + i_padx) * SIZEOF_PIXEL) as size_t,
                );
                y_0 += 1;
            }
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "668:1"]
pub unsafe extern "C" fn x264_10_expand_border_mbpair(
    mut h: *mut x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*(*h).fenc).i_plane {
        let mut v_shift: ::core::ffi::c_int = (i != 0 && (*h).mb.chroma_v_shift != 0)
            as ::core::ffi::c_int;
        let mut stride: ::core::ffi::c_int = (*(*h).fenc).i_stride[i as usize];
        let mut height: ::core::ffi::c_int = (*h).param.i_height >> v_shift;
        let mut pady: ::core::ffi::c_int = (*h).mb.i_mb_height * 16 as ::core::ffi::c_int
            - (*h).param.i_height >> v_shift;
        let mut fenc: *mut pixel = (*(*h).fenc)
            .plane[i as usize]
            .offset((16 as ::core::ffi::c_int * mb_x) as isize);
        let mut y: ::core::ffi::c_int = height;
        while y < height + pady {
            memcpy(
                fenc.offset((y * stride) as isize) as *mut ::core::ffi::c_void,
                fenc.offset(((height - 1 as ::core::ffi::c_int) * stride) as isize)
                    as *const ::core::ffi::c_void,
                (16 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
            );
            y += 1;
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "683:1"]
pub unsafe extern "C" fn x264_10_frame_cond_broadcast(
    mut frame: *mut x264_frame_t,
    mut i_lines_completed: ::core::ffi::c_int,
) {
    pthread_mutex_lock(&mut (*frame).mutex);
    (*frame).i_lines_completed = i_lines_completed;
    pthread_cond_broadcast(&mut (*frame).cv);
    pthread_mutex_unlock(&mut (*frame).mutex);
}
#[no_mangle]
#[c2rust::src_loc = "691:1"]
pub unsafe extern "C" fn x264_10_frame_cond_wait(
    mut frame: *mut x264_frame_t,
    mut i_lines_completed: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut completed: ::core::ffi::c_int = 0;
    pthread_mutex_lock(&mut (*frame).mutex);
    loop {
        completed = (*frame).i_lines_completed;
        if !(completed < i_lines_completed
            && i_lines_completed >= 0 as ::core::ffi::c_int)
        {
            break;
        }
        pthread_cond_wait(&mut (*frame).cv, &mut (*frame).mutex);
    }
    pthread_mutex_unlock(&mut (*frame).mutex);
    return completed;
}
#[no_mangle]
#[c2rust::src_loc = "701:1"]
pub unsafe extern "C" fn x264_10_threadslice_cond_broadcast(
    mut h: *mut x264_t,
    mut pass: ::core::ffi::c_int,
) {
    pthread_mutex_lock(&mut (*h).mutex);
    (*h).i_threadslice_pass = pass;
    if pass > 0 as ::core::ffi::c_int {
        pthread_cond_broadcast(&mut (*h).cv);
    }
    pthread_mutex_unlock(&mut (*h).mutex);
}
#[no_mangle]
#[c2rust::src_loc = "710:1"]
pub unsafe extern "C" fn x264_10_threadslice_cond_wait(
    mut h: *mut x264_t,
    mut pass: ::core::ffi::c_int,
) {
    pthread_mutex_lock(&mut (*h).mutex);
    while (*h).i_threadslice_pass < pass {
        pthread_cond_wait(&mut (*h).cv, &mut (*h).mutex);
    }
    pthread_mutex_unlock(&mut (*h).mutex);
}
#[no_mangle]
#[c2rust::src_loc = "718:1"]
pub unsafe extern "C" fn x264_10_frame_new_slice(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) -> ::core::ffi::c_int {
    if (*h).param.i_slice_count_max != 0 {
        let mut slice_count: ::core::ffi::c_int = 0;
        if (*h).param.b_sliced_threads != 0 {
            slice_count = x264_pthread_fetch_and_add(
                &mut (*frame).i_slice_count,
                1 as ::core::ffi::c_int,
                &mut (*frame).mutex,
            );
        } else {
            let fresh1 = (*frame).i_slice_count;
            (*frame).i_slice_count = (*frame).i_slice_count + 1;
            slice_count = fresh1;
        }
        if slice_count >= (*h).param.i_slice_count_max {
            return -(1 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "735:1"]
pub unsafe extern "C" fn x264_10_frame_push(
    mut list: *mut *mut x264_frame_t,
    mut frame: *mut x264_frame_t,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*list.offset(i as isize)).is_null() {
        i += 1;
    }
    let ref mut fresh2 = *list.offset(i as isize);
    *fresh2 = frame;
}
#[no_mangle]
#[c2rust::src_loc = "742:1"]
pub unsafe extern "C" fn x264_10_frame_pop(
    mut list: *mut *mut x264_frame_t,
) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = 0 as *mut x264_frame_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*list.offset(0 as ::core::ffi::c_int as isize)).is_null() {} else {
        __assert_fail(
            b"list[0]\0" as *const u8 as *const ::core::ffi::c_char,
            b"common/frame.c\0" as *const u8 as *const ::core::ffi::c_char,
            746 as ::core::ffi::c_uint,
            ::core::mem::transmute::<
                [u8; 49],
                [::core::ffi::c_char; 49],
            >(*b"x264_frame_t *x264_10_frame_pop(x264_frame_t **)\0")
                .as_ptr(),
        );
    }
    'c_18603: {
        if !(*list.offset(0 as ::core::ffi::c_int as isize)).is_null() {} else {
            __assert_fail(
                b"list[0]\0" as *const u8 as *const ::core::ffi::c_char,
                b"common/frame.c\0" as *const u8 as *const ::core::ffi::c_char,
                746 as ::core::ffi::c_uint,
                ::core::mem::transmute::<
                    [u8; 49],
                    [::core::ffi::c_char; 49],
                >(*b"x264_frame_t *x264_10_frame_pop(x264_frame_t **)\0")
                    .as_ptr(),
            );
        }
    };
    while !(*list.offset((i + 1 as ::core::ffi::c_int) as isize)).is_null() {
        i += 1;
    }
    frame = *list.offset(i as isize);
    let ref mut fresh3 = *list.offset(i as isize);
    *fresh3 = 0 as *mut x264_frame_t;
    return frame;
}
#[no_mangle]
#[c2rust::src_loc = "753:1"]
pub unsafe extern "C" fn x264_10_frame_unshift(
    mut list: *mut *mut x264_frame_t,
    mut frame: *mut x264_frame_t,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !(*list.offset(i as isize)).is_null() {
        i += 1;
    }
    loop {
        let fresh4 = i;
        i = i - 1;
        if !(fresh4 != 0) {
            break;
        }
        let ref mut fresh5 = *list.offset((i + 1 as ::core::ffi::c_int) as isize);
        *fresh5 = *list.offset(i as isize);
    }
    let ref mut fresh6 = *list.offset(0 as ::core::ffi::c_int as isize);
    *fresh6 = frame;
}
#[no_mangle]
#[c2rust::src_loc = "762:1"]
pub unsafe extern "C" fn x264_10_frame_shift(
    mut list: *mut *mut x264_frame_t,
) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = *list.offset(0 as ::core::ffi::c_int as isize);
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !(*list.offset(i as isize)).is_null() {
        let ref mut fresh7 = *list.offset(i as isize);
        *fresh7 = *list.offset((i + 1 as ::core::ffi::c_int) as isize);
        i += 1;
    }
    if !frame.is_null() {} else {
        __assert_fail(
            b"frame\0" as *const u8 as *const ::core::ffi::c_char,
            b"common/frame.c\0" as *const u8 as *const ::core::ffi::c_char,
            768 as ::core::ffi::c_uint,
            ::core::mem::transmute::<
                [u8; 51],
                [::core::ffi::c_char; 51],
            >(*b"x264_frame_t *x264_10_frame_shift(x264_frame_t **)\0")
                .as_ptr(),
        );
    }
    'c_18705: {
        if !frame.is_null() {} else {
            __assert_fail(
                b"frame\0" as *const u8 as *const ::core::ffi::c_char,
                b"common/frame.c\0" as *const u8 as *const ::core::ffi::c_char,
                768 as ::core::ffi::c_uint,
                ::core::mem::transmute::<
                    [u8; 51],
                    [::core::ffi::c_char; 51],
                >(*b"x264_frame_t *x264_10_frame_shift(x264_frame_t **)\0")
                    .as_ptr(),
            );
        }
    };
    return frame;
}
#[no_mangle]
#[c2rust::src_loc = "772:1"]
pub unsafe extern "C" fn x264_10_frame_push_unused(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    if (*frame).i_reference_count > 0 as ::core::ffi::c_int {} else {
        __assert_fail(
            b"frame->i_reference_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"common/frame.c\0" as *const u8 as *const ::core::ffi::c_char,
            774 as ::core::ffi::c_uint,
            ::core::mem::transmute::<
                [u8; 57],
                [::core::ffi::c_char; 57],
            >(*b"void x264_10_frame_push_unused(x264_t *, x264_frame_t *)\0")
                .as_ptr(),
        );
    }
    'c_18802: {
        if (*frame).i_reference_count > 0 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"frame->i_reference_count > 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"common/frame.c\0" as *const u8 as *const ::core::ffi::c_char,
                774 as ::core::ffi::c_uint,
                ::core::mem::transmute::<
                    [u8; 57],
                    [::core::ffi::c_char; 57],
                >(*b"void x264_10_frame_push_unused(x264_t *, x264_frame_t *)\0")
                    .as_ptr(),
            );
        }
    };
    (*frame).i_reference_count -= 1;
    if (*frame).i_reference_count == 0 as ::core::ffi::c_int {
        x264_10_frame_push((*h).frames.unused[(*frame).b_fdec as usize], frame);
    }
}
#[no_mangle]
#[c2rust::src_loc = "780:1"]
pub unsafe extern "C" fn x264_10_frame_pop_unused(
    mut h: *mut x264_t,
    mut b_fdec: ::core::ffi::c_int,
) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = 0 as *mut x264_frame_t;
    if !(*(*h).frames.unused[b_fdec as usize].offset(0 as ::core::ffi::c_int as isize))
        .is_null()
    {
        frame = x264_10_frame_pop((*h).frames.unused[b_fdec as usize]);
    } else {
        frame = frame_new(h, b_fdec);
    }
    if frame.is_null() {
        return 0 as *mut x264_frame_t;
    }
    (*frame).b_last_minigop_bframe = 0 as uint8_t;
    (*frame).i_reference_count = 1 as ::core::ffi::c_int;
    (*frame).b_intra_calculated = 0 as ::core::ffi::c_int;
    (*frame).b_scenecut = 1 as ::core::ffi::c_int;
    (*frame).b_keyframe = 0 as ::core::ffi::c_int;
    (*frame).b_corrupt = 0 as ::core::ffi::c_int;
    (*frame).i_slice_count = if (*h).param.b_sliced_threads != 0 {
        (*h).param.i_threads
    } else {
        1 as ::core::ffi::c_int
    };
    memset(
        (*frame).weight.as_mut_ptr() as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[[x264_weight_t; 3]; 16]>() as size_t,
    );
    memset(
        (*frame).f_weighted_cost_delta.as_mut_ptr() as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_float; 18]>() as size_t,
    );
    return frame;
}
#[no_mangle]
#[c2rust::src_loc = "803:1"]
pub unsafe extern "C" fn x264_10_frame_push_blank_unused(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    if (*frame).i_reference_count > 0 as ::core::ffi::c_int {} else {
        __assert_fail(
            b"frame->i_reference_count > 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"common/frame.c\0" as *const u8 as *const ::core::ffi::c_char,
            805 as ::core::ffi::c_uint,
            ::core::mem::transmute::<
                [u8; 63],
                [::core::ffi::c_char; 63],
            >(*b"void x264_10_frame_push_blank_unused(x264_t *, x264_frame_t *)\0")
                .as_ptr(),
        );
    }
    'c_18871: {
        if (*frame).i_reference_count > 0 as ::core::ffi::c_int {} else {
            __assert_fail(
                b"frame->i_reference_count > 0\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"common/frame.c\0" as *const u8 as *const ::core::ffi::c_char,
                805 as ::core::ffi::c_uint,
                ::core::mem::transmute::<
                    [u8; 63],
                    [::core::ffi::c_char; 63],
                >(*b"void x264_10_frame_push_blank_unused(x264_t *, x264_frame_t *)\0")
                    .as_ptr(),
            );
        }
    };
    (*frame).i_reference_count -= 1;
    if (*frame).i_reference_count == 0 as ::core::ffi::c_int {
        x264_10_frame_push((*h).frames.blank_unused, frame);
    }
}
#[no_mangle]
#[c2rust::src_loc = "811:1"]
pub unsafe extern "C" fn x264_10_frame_pop_blank_unused(
    mut h: *mut x264_t,
) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = 0 as *mut x264_frame_t;
    if !(*(*h).frames.blank_unused.offset(0 as ::core::ffi::c_int as isize)).is_null() {
        frame = x264_10_frame_pop((*h).frames.blank_unused);
    } else {
        frame = x264_malloc(::core::mem::size_of::<x264_frame_t>() as int64_t)
            as *mut x264_frame_t;
    }
    if frame.is_null() {
        return 0 as *mut x264_frame_t;
    }
    (*frame).b_duplicate = 1 as ::core::ffi::c_int;
    (*frame).i_reference_count = 1 as ::core::ffi::c_int;
    return frame;
}
#[no_mangle]
#[c2rust::src_loc = "825:1"]
pub unsafe extern "C" fn x264_10_weight_scale_plane(
    mut h: *mut x264_t,
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
    mut w: *mut x264_weight_t,
) {
    while i_height > 0 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0;
        x = 0 as ::core::ffi::c_int;
        while x < i_width - 8 as ::core::ffi::c_int {
            (*(*w)
                .weightfn
                .offset((16 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as isize))
                .expect(
                    "non-null function pointer",
                )(
                dst.offset(x as isize),
                i_dst_stride,
                src.offset(x as isize),
                i_src_stride,
                w,
                if i_height < 16 as ::core::ffi::c_int {
                    i_height
                } else {
                    16 as ::core::ffi::c_int
                },
            );
            x += 16 as ::core::ffi::c_int;
        }
        if x < i_width {
            (*(*w)
                .weightfn
                .offset((8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as isize))
                .expect(
                    "non-null function pointer",
                )(
                dst.offset(x as isize),
                i_dst_stride,
                src.offset(x as isize),
                i_src_stride,
                w,
                if i_height < 16 as ::core::ffi::c_int {
                    i_height
                } else {
                    16 as ::core::ffi::c_int
                },
            );
        }
        i_height -= 16 as ::core::ffi::c_int;
        dst = dst.offset((16 as intptr_t * i_dst_stride) as isize);
        src = src.offset((16 as intptr_t * i_src_stride) as isize);
    }
}
#[no_mangle]
#[c2rust::src_loc = "843:1"]
pub unsafe extern "C" fn x264_10_frame_delete_list(mut list: *mut *mut x264_frame_t) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if list.is_null() {
        return;
    }
    while !(*list.offset(i as isize)).is_null() {
        let fresh35 = i;
        i = i + 1;
        x264_10_frame_delete(*list.offset(fresh35 as isize));
    }
    x264_free(list as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "853:1"]
pub unsafe extern "C" fn x264_10_sync_frame_list_init(
    mut slist: *mut x264_sync_frame_list_t,
    mut max_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if max_size < 0 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    (*slist).i_max_size = max_size;
    (*slist).i_size = 0 as ::core::ffi::c_int;
    (*slist).list = x264_malloc(
        ((max_size + 1 as ::core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<*mut x264_frame_t>() as usize)
            as int64_t,
    ) as *mut *mut x264_frame_t;
    if (*slist).list.is_null() {
        return -(1 as ::core::ffi::c_int)
    } else {
        memset(
            (*slist).list as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((max_size + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut x264_frame_t>() as size_t),
        );
        if pthread_mutex_init(&mut (*slist).mutex, 0 as *const pthread_mutexattr_t) != 0
            || pthread_cond_init(&mut (*slist).cv_fill, 0 as *const pthread_condattr_t)
                != 0
            || pthread_cond_init(&mut (*slist).cv_empty, 0 as *const pthread_condattr_t)
                != 0
        {
            return -(1 as ::core::ffi::c_int);
        }
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
#[c2rust::src_loc = "869:1"]
pub unsafe extern "C" fn x264_10_sync_frame_list_delete(
    mut slist: *mut x264_sync_frame_list_t,
) {
    pthread_mutex_destroy(&mut (*slist).mutex);
    pthread_cond_destroy(&mut (*slist).cv_fill);
    pthread_cond_destroy(&mut (*slist).cv_empty);
    x264_10_frame_delete_list((*slist).list);
}
#[no_mangle]
#[c2rust::src_loc = "877:1"]
pub unsafe extern "C" fn x264_10_sync_frame_list_push(
    mut slist: *mut x264_sync_frame_list_t,
    mut frame: *mut x264_frame_t,
) {
    pthread_mutex_lock(&mut (*slist).mutex);
    while (*slist).i_size == (*slist).i_max_size {
        pthread_cond_wait(&mut (*slist).cv_empty, &mut (*slist).mutex);
    }
    let fresh36 = (*slist).i_size;
    (*slist).i_size = (*slist).i_size + 1;
    let ref mut fresh37 = *(*slist).list.offset(fresh36 as isize);
    *fresh37 = frame;
    pthread_mutex_unlock(&mut (*slist).mutex);
    pthread_cond_broadcast(&mut (*slist).cv_fill);
}
#[no_mangle]
#[c2rust::src_loc = "887:1"]
pub unsafe extern "C" fn x264_10_sync_frame_list_pop(
    mut slist: *mut x264_sync_frame_list_t,
) -> *mut x264_frame_t {
    let mut frame: *mut x264_frame_t = 0 as *mut x264_frame_t;
    pthread_mutex_lock(&mut (*slist).mutex);
    while (*slist).i_size == 0 {
        pthread_cond_wait(&mut (*slist).cv_fill, &mut (*slist).mutex);
    }
    (*slist).i_size -= 1;
    frame = *(*slist).list.offset((*slist).i_size as isize);
    let ref mut fresh38 = *(*slist).list.offset((*slist).i_size as isize);
    *fresh38 = 0 as *mut x264_frame_t;
    pthread_cond_broadcast(&mut (*slist).cv_empty);
    pthread_mutex_unlock(&mut (*slist).mutex);
    return frame;
}
