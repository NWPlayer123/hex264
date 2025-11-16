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
    #[c2rust::src_loc = "61:9"]
    pub const PIXEL_MAX: ::core::ffi::c_int = ((1 as ::core::ffi::c_int) << BIT_DEPTH)
        - 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "570:9"]
    pub const FENC_STRIDE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    #[c2rust::src_loc = "571:9"]
    pub const FDEC_STRIDE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
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
    use super::internal::BIT_DEPTH;
    extern "C" {
        #[c2rust::src_loc = "231:16"]
        pub type x264_ratecontrol_t;
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
    use super::stdint_uintn_h::{uint8_t, uint32_t};
    use super::internal::__va_list_tag;
    use super::common_h::x264_t;
    use super::stdint_intn_h::int64_t;
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
    #[c2rust::src_loc = "37:1"]
    pub type C2RustUnnamed_19 = ::core::ffi::c_uint;
    #[c2rust::src_loc = "52:5"]
    pub const PIXEL_2x2: C2RustUnnamed_19 = 11;
    #[c2rust::src_loc = "51:5"]
    pub const PIXEL_2x4: C2RustUnnamed_19 = 10;
    #[c2rust::src_loc = "50:5"]
    pub const PIXEL_2x8: C2RustUnnamed_19 = 9;
    #[c2rust::src_loc = "49:5"]
    pub const PIXEL_4x2: C2RustUnnamed_19 = 8;
    #[c2rust::src_loc = "48:5"]
    pub const PIXEL_4x16: C2RustUnnamed_19 = 7;
    #[c2rust::src_loc = "45:5"]
    pub const PIXEL_4x4: C2RustUnnamed_19 = 6;
    #[c2rust::src_loc = "44:5"]
    pub const PIXEL_4x8: C2RustUnnamed_19 = 5;
    #[c2rust::src_loc = "43:5"]
    pub const PIXEL_8x4: C2RustUnnamed_19 = 4;
    #[c2rust::src_loc = "42:5"]
    pub const PIXEL_8x8: C2RustUnnamed_19 = 3;
    #[c2rust::src_loc = "41:5"]
    pub const PIXEL_8x16: C2RustUnnamed_19 = 2;
    #[c2rust::src_loc = "40:5"]
    pub const PIXEL_16x8: C2RustUnnamed_19 = 1;
    #[c2rust::src_loc = "39:5"]
    pub const PIXEL_16x16: C2RustUnnamed_19 = 0;
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
    extern "C" {
        #[c2rust::src_loc = "113:1"]
        pub fn x264_10_predict_8x8_dc_c(src: *mut pixel, edge: *mut pixel);
        #[c2rust::src_loc = "115:1"]
        pub fn x264_10_predict_8x8_h_c(src: *mut pixel, edge: *mut pixel);
        #[c2rust::src_loc = "117:1"]
        pub fn x264_10_predict_8x8_v_c(src: *mut pixel, edge: *mut pixel);
        #[c2rust::src_loc = "119:1"]
        pub fn x264_10_predict_4x4_dc_c(src: *mut pixel);
        #[c2rust::src_loc = "121:1"]
        pub fn x264_10_predict_4x4_h_c(src: *mut pixel);
        #[c2rust::src_loc = "123:1"]
        pub fn x264_10_predict_4x4_v_c(src: *mut pixel);
        #[c2rust::src_loc = "125:1"]
        pub fn x264_10_predict_16x16_dc_c(src: *mut pixel);
        #[c2rust::src_loc = "127:1"]
        pub fn x264_10_predict_16x16_h_c(src: *mut pixel);
        #[c2rust::src_loc = "129:1"]
        pub fn x264_10_predict_16x16_v_c(src: *mut pixel);
        #[c2rust::src_loc = "133:1"]
        pub fn x264_10_predict_8x8c_dc_c(src: *mut pixel);
        #[c2rust::src_loc = "135:1"]
        pub fn x264_10_predict_8x8c_h_c(src: *mut pixel);
        #[c2rust::src_loc = "137:1"]
        pub fn x264_10_predict_8x8c_v_c(src: *mut pixel);
        #[c2rust::src_loc = "141:1"]
        pub fn x264_10_predict_8x16c_dc_c(src: *mut pixel);
        #[c2rust::src_loc = "143:1"]
        pub fn x264_10_predict_8x16c_h_c(src: *mut pixel);
        #[c2rust::src_loc = "145:1"]
        pub fn x264_10_predict_8x16c_v_c(src: *mut pixel);
    }
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
#[c2rust::header_src = "/usr/include/string.h:28"]
pub mod string_h {
    use super::__stddef_size_t_h::size_t;
    extern "C" {
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
pub use self::pthreadtypes_h::{pthread_t, pthread_mutex_t, pthread_cond_t};
pub use self::common_h::{
    x264_t, x264_lookahead_t, pixel, dctcoef, udctcoef, C2RustUnnamed_6,
    x264_frame_stat_t, C2RustUnnamed_7, C2RustUnnamed_8, C2RustUnnamed_9,
    x264_left_table_t, C2RustUnnamed_10, C2RustUnnamed_11, x264_slice_header_t,
    C2RustUnnamed_12, C2RustUnnamed_13, C2RustUnnamed_17, C2RustUnnamed_18, PIXEL_MAX,
    FENC_STRIDE, FDEC_STRIDE, x264_ratecontrol_t,
};
pub use self::frame_h::{
    x264_sync_frame_list_t, x264_frame_t, x264_frame, x264_deblock_function_t,
    x264_deblock_intra_t, x264_deblock_inter_t,
};
pub use self::x264_h::{
    x264_sei_t, x264_sei_payload_t, x264_hrd_t, x264_param_t, x264_nal_t,
    C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, x264_zone_t,
    C2RustUnnamed_4, C2RustUnnamed_5,
};
pub use self::mc_h::{x264_weight_t, weight_fn_t, x264_mc_functions_t};
pub use self::bitstream_h::{x264_bitstream_function_t, x264_run_level_t, bs_t, bs_s};
pub use self::cabac_h::x264_cabac_t;
pub use self::quant_h::x264_quant_function_t;
pub use self::dct_h::{x264_zigzag_function_t, x264_dct_function_t};
pub use self::pixel_h::{
    x264_pixel_function_t, x264_pixel_cmp_x4_t, x264_pixel_cmp_x3_t, x264_pixel_cmp_t,
    C2RustUnnamed_19, PIXEL_2x2, PIXEL_2x4, PIXEL_2x8, PIXEL_4x2, PIXEL_4x16, PIXEL_4x4,
    PIXEL_4x8, PIXEL_8x4, PIXEL_8x8, PIXEL_8x16, PIXEL_16x8, PIXEL_16x16,
};
pub use self::predict_h::{
    x264_predict_8x8_filter_t, x264_predict_t, x264_predict8x8_t,
    x264_10_predict_8x8_dc_c, x264_10_predict_8x8_h_c, x264_10_predict_8x8_v_c,
    x264_10_predict_4x4_dc_c, x264_10_predict_4x4_h_c, x264_10_predict_4x4_v_c,
    x264_10_predict_16x16_dc_c, x264_10_predict_16x16_h_c, x264_10_predict_16x16_v_c,
    x264_10_predict_8x8c_dc_c, x264_10_predict_8x8c_h_c, x264_10_predict_8x8c_v_c,
    x264_10_predict_8x16c_dc_c, x264_10_predict_8x16c_h_c, x264_10_predict_8x16c_v_c,
};
pub use self::set_h::{
    x264_pps_t, x264_sps_t, C2RustUnnamed_14, C2RustUnnamed_15, C2RustUnnamed_16,
};
use self::threadpool_h::x264_threadpool_t;
use self::string_h::memset;
use self::stdlib_h::abs;
#[c2rust::src_loc = "235:5"]
pub type sum2_t = uint64_t;
#[c2rust::src_loc = "234:5"]
pub type sum_t = uint32_t;
#[c2rust::src_loc = "73:1"]
unsafe extern "C" fn x264_pixel_sad_16x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 16 as ::core::ffi::c_int {
            i_sum
                += abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "74:1"]
unsafe extern "C" fn x264_pixel_sad_16x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 16 as ::core::ffi::c_int {
            i_sum
                += abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "75:1"]
unsafe extern "C" fn x264_pixel_sad_8x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            i_sum
                += abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn x264_pixel_sad_8x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            i_sum
                += abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "77:1"]
unsafe extern "C" fn x264_pixel_sad_8x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            i_sum
                += abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "78:1"]
unsafe extern "C" fn x264_pixel_sad_4x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            i_sum
                += abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn x264_pixel_sad_4x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            i_sum
                += abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "80:1"]
unsafe extern "C" fn x264_pixel_sad_4x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            i_sum
                += abs(
                    *pix1.offset(x as isize) as ::core::ffi::c_int
                        - *pix2.offset(x as isize) as ::core::ffi::c_int,
                );
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "103:1"]
unsafe extern "C" fn x264_pixel_ssd_16x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 16 as ::core::ffi::c_int {
            let mut d: ::core::ffi::c_int = *pix1.offset(x as isize)
                as ::core::ffi::c_int - *pix2.offset(x as isize) as ::core::ffi::c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn x264_pixel_ssd_16x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 16 as ::core::ffi::c_int {
            let mut d: ::core::ffi::c_int = *pix1.offset(x as isize)
                as ::core::ffi::c_int - *pix2.offset(x as isize) as ::core::ffi::c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "105:1"]
unsafe extern "C" fn x264_pixel_ssd_8x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            let mut d: ::core::ffi::c_int = *pix1.offset(x as isize)
                as ::core::ffi::c_int - *pix2.offset(x as isize) as ::core::ffi::c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn x264_pixel_ssd_8x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            let mut d: ::core::ffi::c_int = *pix1.offset(x as isize)
                as ::core::ffi::c_int - *pix2.offset(x as isize) as ::core::ffi::c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn x264_pixel_ssd_8x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            let mut d: ::core::ffi::c_int = *pix1.offset(x as isize)
                as ::core::ffi::c_int - *pix2.offset(x as isize) as ::core::ffi::c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "108:1"]
unsafe extern "C" fn x264_pixel_ssd_4x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            let mut d: ::core::ffi::c_int = *pix1.offset(x as isize)
                as ::core::ffi::c_int - *pix2.offset(x as isize) as ::core::ffi::c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "109:1"]
unsafe extern "C" fn x264_pixel_ssd_4x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            let mut d: ::core::ffi::c_int = *pix1.offset(x as isize)
                as ::core::ffi::c_int - *pix2.offset(x as isize) as ::core::ffi::c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn x264_pixel_ssd_4x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut i_sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 4 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int {
            let mut d: ::core::ffi::c_int = *pix1.offset(x as isize)
                as ::core::ffi::c_int - *pix2.offset(x as isize) as ::core::ffi::c_int;
            i_sum += d * d;
            x += 1;
        }
        pix1 = pix1.offset(i_stride_pix1 as isize);
        pix2 = pix2.offset(i_stride_pix2 as isize);
        y += 1;
    }
    return i_sum;
}
#[no_mangle]
#[c2rust::src_loc = "112:1"]
pub unsafe extern "C" fn x264_10_pixel_ssd_wxh(
    mut pf: *mut x264_pixel_function_t,
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) -> uint64_t {
    let mut i_ssd: uint64_t = 0 as uint64_t;
    let mut y: ::core::ffi::c_int = 0;
    let mut align: ::core::ffi::c_int = ((pix1 as intptr_t | pix2 as intptr_t | i_pix1
        | i_pix2) & 15 as intptr_t == 0) as ::core::ffi::c_int;
    y = 0 as ::core::ffi::c_int;
    while y < i_height - 15 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if align != 0 {
            while x < i_width - 15 as ::core::ffi::c_int {
                i_ssd = i_ssd
                    .wrapping_add(
                        (*pf)
                            .ssd[PIXEL_16x16 as ::core::ffi::c_int as usize]
                            .expect(
                                "non-null function pointer",
                            )(
                            pix1
                                .offset((y as intptr_t * i_pix1) as isize)
                                .offset(x as isize),
                            i_pix1,
                            pix2
                                .offset((y as intptr_t * i_pix2) as isize)
                                .offset(x as isize),
                            i_pix2,
                        ) as uint64_t,
                    );
                x += 16 as ::core::ffi::c_int;
            }
        }
        while x < i_width - 7 as ::core::ffi::c_int {
            i_ssd = i_ssd
                .wrapping_add(
                    (*pf)
                        .ssd[PIXEL_8x16 as ::core::ffi::c_int as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        pix1
                            .offset((y as intptr_t * i_pix1) as isize)
                            .offset(x as isize),
                        i_pix1,
                        pix2
                            .offset((y as intptr_t * i_pix2) as isize)
                            .offset(x as isize),
                        i_pix2,
                    ) as uint64_t,
                );
            x += 8 as ::core::ffi::c_int;
        }
        y += 16 as ::core::ffi::c_int;
    }
    if y < i_height - 7 as ::core::ffi::c_int {
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < i_width - 7 as ::core::ffi::c_int {
            i_ssd = i_ssd
                .wrapping_add(
                    (*pf)
                        .ssd[PIXEL_8x8 as ::core::ffi::c_int as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        pix1
                            .offset((y as intptr_t * i_pix1) as isize)
                            .offset(x_0 as isize),
                        i_pix1,
                        pix2
                            .offset((y as intptr_t * i_pix2) as isize)
                            .offset(x_0 as isize),
                        i_pix2,
                    ) as uint64_t,
                );
            x_0 += 8 as ::core::ffi::c_int;
        }
    }
    if i_width & 7 as ::core::ffi::c_int != 0 {
        y = 0 as ::core::ffi::c_int;
        while y < i_height & !(7 as ::core::ffi::c_int) {
            let mut x_1: ::core::ffi::c_int = i_width & !(7 as ::core::ffi::c_int);
            while x_1 < i_width {
                let mut d: ::core::ffi::c_int = *pix1
                    .offset((y as intptr_t * i_pix1 + x_1 as intptr_t) as isize)
                    as ::core::ffi::c_int
                    - *pix2.offset((y as intptr_t * i_pix2 + x_1 as intptr_t) as isize)
                        as ::core::ffi::c_int;
                i_ssd = i_ssd.wrapping_add((d * d) as uint64_t);
                x_1 += 1;
            }
            y += 1;
        }
    }
    if i_height & 7 as ::core::ffi::c_int != 0 {
        y = i_height & !(7 as ::core::ffi::c_int);
        while y < i_height {
            let mut x_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_2 < i_width {
                let mut d_0: ::core::ffi::c_int = *pix1
                    .offset((y as intptr_t * i_pix1 + x_2 as intptr_t) as isize)
                    as ::core::ffi::c_int
                    - *pix2.offset((y as intptr_t * i_pix2 + x_2 as intptr_t) as isize)
                        as ::core::ffi::c_int;
                i_ssd = i_ssd.wrapping_add((d_0 * d_0) as uint64_t);
                x_2 += 1;
            }
            y += 1;
        }
    }
    return i_ssd;
}
#[c2rust::src_loc = "153:1"]
unsafe extern "C" fn pixel_ssd_nv12_core(
    mut pixuv1: *mut pixel,
    mut stride1: intptr_t,
    mut pixuv2: *mut pixel,
    mut stride2: intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut ssd_u: *mut uint64_t,
    mut ssd_v: *mut uint64_t,
) {
    *ssd_u = 0 as uint64_t;
    *ssd_v = 0 as uint64_t;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            let mut du: ::core::ffi::c_int = *pixuv1
                .offset((2 as ::core::ffi::c_int * x) as isize) as ::core::ffi::c_int
                - *pixuv2.offset((2 as ::core::ffi::c_int * x) as isize)
                    as ::core::ffi::c_int;
            let mut dv: ::core::ffi::c_int = *pixuv1
                .offset((2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - *pixuv2
                    .offset(
                        (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int;
            *ssd_u = (*ssd_u).wrapping_add((du * du) as uint64_t);
            *ssd_v = (*ssd_v).wrapping_add((dv * dv) as uint64_t);
            x += 1;
        }
        y += 1;
        pixuv1 = pixuv1.offset(stride1 as isize);
        pixuv2 = pixuv2.offset(stride2 as isize);
    }
}
#[no_mangle]
#[c2rust::src_loc = "167:1"]
pub unsafe extern "C" fn x264_10_pixel_ssd_nv12(
    mut pf: *mut x264_pixel_function_t,
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
    mut ssd_u: *mut uint64_t,
    mut ssd_v: *mut uint64_t,
) {
    (*pf)
        .ssd_nv12_core
        .expect(
            "non-null function pointer",
        )(
        pix1,
        i_pix1,
        pix2,
        i_pix2,
        i_width & !(7 as ::core::ffi::c_int),
        i_height,
        ssd_u,
        ssd_v,
    );
    if i_width & 7 as ::core::ffi::c_int != 0 {
        let mut tmp: [uint64_t; 2] = [0; 2];
        pixel_ssd_nv12_core(
            pix1.offset((i_width & !(7 as ::core::ffi::c_int)) as isize),
            i_pix1,
            pix2.offset((i_width & !(7 as ::core::ffi::c_int)) as isize),
            i_pix2,
            i_width & 7 as ::core::ffi::c_int,
            i_height,
            &mut *tmp.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize),
            &mut *tmp.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize),
        );
        *ssd_u = (*ssd_u).wrapping_add(tmp[0 as ::core::ffi::c_int as usize]);
        *ssd_v = (*ssd_v).wrapping_add(tmp[1 as ::core::ffi::c_int as usize]);
    }
}
#[c2rust::src_loc = "199:1"]
unsafe extern "C" fn pixel_var_16x16(
    mut pix: *mut pixel,
    mut i_stride: intptr_t,
) -> uint64_t {
    let mut sum: uint32_t = 0 as uint32_t;
    let mut sqr: uint32_t = 0 as uint32_t;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 16 as ::core::ffi::c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr
                .wrapping_add(
                    (*pix.offset(x as isize) as ::core::ffi::c_int
                        * *pix.offset(x as isize) as ::core::ffi::c_int) as uint32_t,
                );
            x += 1;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
    }
    return (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "200:1"]
unsafe extern "C" fn pixel_var_8x16(
    mut pix: *mut pixel,
    mut i_stride: intptr_t,
) -> uint64_t {
    let mut sum: uint32_t = 0 as uint32_t;
    let mut sqr: uint32_t = 0 as uint32_t;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr
                .wrapping_add(
                    (*pix.offset(x as isize) as ::core::ffi::c_int
                        * *pix.offset(x as isize) as ::core::ffi::c_int) as uint32_t,
                );
            x += 1;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
    }
    return (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "201:1"]
unsafe extern "C" fn pixel_var_8x8(
    mut pix: *mut pixel,
    mut i_stride: intptr_t,
) -> uint64_t {
    let mut sum: uint32_t = 0 as uint32_t;
    let mut sqr: uint32_t = 0 as uint32_t;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            sum = sum.wrapping_add(*pix.offset(x as isize) as uint32_t);
            sqr = sqr
                .wrapping_add(
                    (*pix.offset(x as isize) as ::core::ffi::c_int
                        * *pix.offset(x as isize) as ::core::ffi::c_int) as uint32_t,
                );
            x += 1;
        }
        pix = pix.offset(i_stride as isize);
        y += 1;
    }
    return (sum as uint64_t).wrapping_add((sqr as uint64_t) << 32 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "230:1"]
unsafe extern "C" fn pixel_var2_8x16(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut ssd: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sum_u: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sum_v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sqr_u: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sqr_v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 16 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            let mut diff_u: ::core::ffi::c_int = *fenc.offset(x as isize)
                as ::core::ffi::c_int - *fdec.offset(x as isize) as ::core::ffi::c_int;
            let mut diff_v: ::core::ffi::c_int = *fenc
                .offset((x + FENC_STRIDE / 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - *fdec.offset((x + FDEC_STRIDE / 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            sum_u += diff_u;
            sum_v += diff_v;
            sqr_u += diff_u * diff_u;
            sqr_v += diff_v * diff_v;
            x += 1;
        }
        fenc = fenc.offset(FENC_STRIDE as isize);
        fdec = fdec.offset(FDEC_STRIDE as isize);
        y += 1;
    }
    *ssd.offset(0 as ::core::ffi::c_int as isize) = sqr_u;
    *ssd.offset(1 as ::core::ffi::c_int as isize) = sqr_v;
    return (sqr_u as int64_t
        - (sum_u as int64_t * sum_u as int64_t >> 7 as ::core::ffi::c_int)
        + sqr_v as int64_t
        - (sum_v as int64_t * sum_v as int64_t >> 7 as ::core::ffi::c_int))
        as ::core::ffi::c_int;
}
#[c2rust::src_loc = "231:1"]
unsafe extern "C" fn pixel_var2_8x8(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut ssd: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sum_u: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sum_v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sqr_u: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sqr_v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < 8 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            let mut diff_u: ::core::ffi::c_int = *fenc.offset(x as isize)
                as ::core::ffi::c_int - *fdec.offset(x as isize) as ::core::ffi::c_int;
            let mut diff_v: ::core::ffi::c_int = *fenc
                .offset((x + FENC_STRIDE / 2 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                - *fdec.offset((x + FDEC_STRIDE / 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            sum_u += diff_u;
            sum_v += diff_v;
            sqr_u += diff_u * diff_u;
            sqr_v += diff_v * diff_v;
            x += 1;
        }
        fenc = fenc.offset(FENC_STRIDE as isize);
        fdec = fdec.offset(FDEC_STRIDE as isize);
        y += 1;
    }
    *ssd.offset(0 as ::core::ffi::c_int as isize) = sqr_u;
    *ssd.offset(1 as ::core::ffi::c_int as isize) = sqr_v;
    return (sqr_u as int64_t
        - (sum_u as int64_t * sum_u as int64_t >> 6 as ::core::ffi::c_int)
        + sqr_v as int64_t
        - (sum_v as int64_t * sum_v as int64_t >> 6 as ::core::ffi::c_int))
        as ::core::ffi::c_int;
}
#[c2rust::src_loc = "240:9"]
pub const BITS_PER_SUM: usize = (8 as usize)
    .wrapping_mul(::core::mem::size_of::<sum_t>() as usize);
#[inline(always)]
#[c2rust::src_loc = "255:1"]
unsafe extern "C" fn abs2(mut a: sum2_t) -> sum2_t {
    let mut s: sum2_t = (a >> BITS_PER_SUM.wrapping_sub(1 as usize)
        & ((1 as ::core::ffi::c_int as sum2_t) << BITS_PER_SUM)
            .wrapping_add(1 as sum2_t))
        .wrapping_mul(-(1 as ::core::ffi::c_int) as sum_t as sum2_t);
    return a.wrapping_add(s) ^ s;
}
#[inline(never)]
#[c2rust::src_loc = "265:1"]
unsafe extern "C" fn x264_pixel_satd_4x4(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut tmp: [[sum2_t; 2]; 4] = [[0; 2]; 4];
    let mut a0: sum2_t = 0;
    let mut a1: sum2_t = 0;
    let mut a2: sum2_t = 0;
    let mut a3: sum2_t = 0;
    let mut b0: sum2_t = 0;
    let mut b1: sum2_t = 0;
    let mut sum: sum2_t = 0 as sum2_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        a0 = (*pix1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        a1 = (*pix1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        b0 = a0.wrapping_add(a1).wrapping_add(a0.wrapping_sub(a1) << BITS_PER_SUM);
        a2 = (*pix1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        a3 = (*pix1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        b1 = a2.wrapping_add(a3).wrapping_add(a2.wrapping_sub(a3) << BITS_PER_SUM);
        tmp[i as usize][0 as ::core::ffi::c_int as usize] = b0.wrapping_add(b1);
        tmp[i as usize][1 as ::core::ffi::c_int as usize] = b0.wrapping_sub(b1);
        i += 1;
        pix1 = pix1.offset(i_pix1 as isize);
        pix2 = pix2.offset(i_pix2 as isize);
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 2 as ::core::ffi::c_int {
        let mut t0: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_add(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t1: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t2: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_add(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t3: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
        a0 = t0.wrapping_add(t2);
        a2 = t0.wrapping_sub(t2);
        a1 = t1.wrapping_add(t3);
        a3 = t1.wrapping_sub(t3);
        a0 = abs2(a0)
            .wrapping_add(abs2(a1))
            .wrapping_add(abs2(a2))
            .wrapping_add(abs2(a3));
        sum = sum.wrapping_add((a0 as sum_t as sum2_t).wrapping_add(a0 >> BITS_PER_SUM));
        i_0 += 1;
    }
    return (sum >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[inline(never)]
#[c2rust::src_loc = "290:1"]
unsafe extern "C" fn x264_pixel_satd_8x4(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut tmp: [[sum2_t; 4]; 4] = [[0; 4]; 4];
    let mut a0: sum2_t = 0;
    let mut a1: sum2_t = 0;
    let mut a2: sum2_t = 0;
    let mut a3: sum2_t = 0;
    let mut sum: sum2_t = 0 as sum2_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        a0 = ((*pix1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t)
            .wrapping_add(
                ((*pix1.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *pix2.offset(4 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as sum2_t) << BITS_PER_SUM,
            );
        a1 = ((*pix1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t)
            .wrapping_add(
                ((*pix1.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *pix2.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as sum2_t) << BITS_PER_SUM,
            );
        a2 = ((*pix1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t)
            .wrapping_add(
                ((*pix1.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *pix2.offset(6 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as sum2_t) << BITS_PER_SUM,
            );
        a3 = ((*pix1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t)
            .wrapping_add(
                ((*pix1.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *pix2.offset(7 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as sum2_t) << BITS_PER_SUM,
            );
        let mut t0: sum2_t = a0.wrapping_add(a1);
        let mut t1: sum2_t = a0.wrapping_sub(a1);
        let mut t2: sum2_t = a2.wrapping_add(a3);
        let mut t3: sum2_t = a2.wrapping_sub(a3);
        tmp[i as usize][0 as ::core::ffi::c_int as usize] = t0.wrapping_add(t2);
        tmp[i as usize][2 as ::core::ffi::c_int as usize] = t0.wrapping_sub(t2);
        tmp[i as usize][1 as ::core::ffi::c_int as usize] = t1.wrapping_add(t3);
        tmp[i as usize][3 as ::core::ffi::c_int as usize] = t1.wrapping_sub(t3);
        i += 1;
        pix1 = pix1.offset(i_pix1 as isize);
        pix2 = pix2.offset(i_pix2 as isize);
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 4 as ::core::ffi::c_int {
        let mut t0_0: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_add(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t1_0: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t2_0: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_add(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t3_0: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
        a0 = t0_0.wrapping_add(t2_0);
        a2 = t0_0.wrapping_sub(t2_0);
        a1 = t1_0.wrapping_add(t3_0);
        a3 = t1_0.wrapping_sub(t3_0);
        sum = sum
            .wrapping_add(
                abs2(a0)
                    .wrapping_add(abs2(a1))
                    .wrapping_add(abs2(a2))
                    .wrapping_add(abs2(a3)),
            );
        i_0 += 1;
    }
    return ((sum as sum_t as sum2_t).wrapping_add(sum >> BITS_PER_SUM)
        >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[c2rust::src_loc = "327:1"]
unsafe extern "C" fn x264_pixel_satd_16x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut sum: ::core::ffi::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1.offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        && 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
    {
        sum
            += x264_pixel_satd_8x4(
                pix1
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
}
#[c2rust::src_loc = "328:1"]
unsafe extern "C" fn x264_pixel_satd_16x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut sum: ::core::ffi::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1.offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        && 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
    {
        sum
            += x264_pixel_satd_8x4(
                pix1
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
}
#[c2rust::src_loc = "329:1"]
unsafe extern "C" fn x264_pixel_satd_8x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut sum: ::core::ffi::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1.offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        && 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
    {
        sum
            += x264_pixel_satd_8x4(
                pix1
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
}
#[c2rust::src_loc = "330:1"]
unsafe extern "C" fn x264_pixel_satd_8x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut sum: ::core::ffi::c_int = x264_pixel_satd_8x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_8x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_8x4(
                pix1.offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1.offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        && 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
    {
        sum
            += x264_pixel_satd_8x4(
                pix1
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_8x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
}
#[c2rust::src_loc = "331:1"]
unsafe extern "C" fn x264_pixel_satd_4x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut sum: ::core::ffi::c_int = x264_pixel_satd_4x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_4x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 4 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_4x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_4x4(
                pix1.offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1.offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 4 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        && 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
    {
        sum
            += x264_pixel_satd_4x4(
                pix1
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
}
#[c2rust::src_loc = "332:1"]
unsafe extern "C" fn x264_pixel_satd_4x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut sum: ::core::ffi::c_int = x264_pixel_satd_4x4(pix1, i_pix1, pix2, i_pix2)
        + x264_pixel_satd_4x4(
            pix1.offset((4 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((4 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    if 4 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_4x4(
                pix1.offset(8 as ::core::ffi::c_int as isize),
                i_pix1,
                pix2.offset(8 as ::core::ffi::c_int as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((4 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum
            += x264_pixel_satd_4x4(
                pix1.offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2.offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1.offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2.offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    if 4 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        && 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
    {
        sum
            += x264_pixel_satd_4x4(
                pix1
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix1) as isize),
                i_pix1,
                pix2
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset((8 as intptr_t * i_pix2) as isize),
                i_pix2,
            )
                + x264_pixel_satd_4x4(
                    pix1
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix1) as isize),
                    i_pix1,
                    pix2
                        .offset(8 as ::core::ffi::c_int as isize)
                        .offset((12 as intptr_t * i_pix2) as isize),
                    i_pix2,
                );
    }
    return sum;
}
#[inline(never)]
#[c2rust::src_loc = "334:1"]
unsafe extern "C" fn sa8d_8x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut tmp: [[sum2_t; 4]; 8] = [[0; 4]; 8];
    let mut a0: sum2_t = 0;
    let mut a1: sum2_t = 0;
    let mut a2: sum2_t = 0;
    let mut a3: sum2_t = 0;
    let mut a4: sum2_t = 0;
    let mut a5: sum2_t = 0;
    let mut a6: sum2_t = 0;
    let mut a7: sum2_t = 0;
    let mut b0: sum2_t = 0;
    let mut b1: sum2_t = 0;
    let mut b2: sum2_t = 0;
    let mut b3: sum2_t = 0;
    let mut sum: sum2_t = 0 as sum2_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        a0 = (*pix1.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        a1 = (*pix1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        b0 = a0.wrapping_add(a1).wrapping_add(a0.wrapping_sub(a1) << BITS_PER_SUM);
        a2 = (*pix1.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        a3 = (*pix1.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        b1 = a2.wrapping_add(a3).wrapping_add(a2.wrapping_sub(a3) << BITS_PER_SUM);
        a4 = (*pix1.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        a5 = (*pix1.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        b2 = a4.wrapping_add(a5).wrapping_add(a4.wrapping_sub(a5) << BITS_PER_SUM);
        a6 = (*pix1.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        a7 = (*pix1.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            - *pix2.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t;
        b3 = a6.wrapping_add(a7).wrapping_add(a6.wrapping_sub(a7) << BITS_PER_SUM);
        let mut t0: sum2_t = b0.wrapping_add(b1);
        let mut t1: sum2_t = b0.wrapping_sub(b1);
        let mut t2: sum2_t = b2.wrapping_add(b3);
        let mut t3: sum2_t = b2.wrapping_sub(b3);
        tmp[i as usize][0 as ::core::ffi::c_int as usize] = t0.wrapping_add(t2);
        tmp[i as usize][2 as ::core::ffi::c_int as usize] = t0.wrapping_sub(t2);
        tmp[i as usize][1 as ::core::ffi::c_int as usize] = t1.wrapping_add(t3);
        tmp[i as usize][3 as ::core::ffi::c_int as usize] = t1.wrapping_sub(t3);
        i += 1;
        pix1 = pix1.offset(i_pix1 as isize);
        pix2 = pix2.offset(i_pix2 as isize);
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 4 as ::core::ffi::c_int {
        let mut t0_0: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_add(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t1_0: sum2_t = tmp[0 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[1 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t2_0: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_add(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t3_0: sum2_t = tmp[2 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[3 as ::core::ffi::c_int as usize][i_0 as usize]);
        a0 = t0_0.wrapping_add(t2_0);
        a2 = t0_0.wrapping_sub(t2_0);
        a1 = t1_0.wrapping_add(t3_0);
        a3 = t1_0.wrapping_sub(t3_0);
        let mut t0_1: sum2_t = tmp[4 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_add(tmp[5 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t1_1: sum2_t = tmp[4 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[5 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t2_1: sum2_t = tmp[6 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_add(tmp[7 as ::core::ffi::c_int as usize][i_0 as usize]);
        let mut t3_1: sum2_t = tmp[6 as ::core::ffi::c_int as usize][i_0 as usize]
            .wrapping_sub(tmp[7 as ::core::ffi::c_int as usize][i_0 as usize]);
        a4 = t0_1.wrapping_add(t2_1);
        a6 = t0_1.wrapping_sub(t2_1);
        a5 = t1_1.wrapping_add(t3_1);
        a7 = t1_1.wrapping_sub(t3_1);
        b0 = abs2(a0.wrapping_add(a4)).wrapping_add(abs2(a0.wrapping_sub(a4)));
        b0 = b0
            .wrapping_add(
                abs2(a1.wrapping_add(a5)).wrapping_add(abs2(a1.wrapping_sub(a5))),
            );
        b0 = b0
            .wrapping_add(
                abs2(a2.wrapping_add(a6)).wrapping_add(abs2(a2.wrapping_sub(a6))),
            );
        b0 = b0
            .wrapping_add(
                abs2(a3.wrapping_add(a7)).wrapping_add(abs2(a3.wrapping_sub(a7))),
            );
        sum = sum.wrapping_add((b0 as sum_t as sum2_t).wrapping_add(b0 >> BITS_PER_SUM));
        i_0 += 1;
    }
    return sum as ::core::ffi::c_int;
}
#[c2rust::src_loc = "368:1"]
unsafe extern "C" fn x264_pixel_sa8d_8x8(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut sum: ::core::ffi::c_int = sa8d_8x8(pix1, i_pix1, pix2, i_pix2);
    return sum + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "374:1"]
unsafe extern "C" fn x264_pixel_sa8d_16x16(
    mut pix1: *mut pixel,
    mut i_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_pix2: intptr_t,
) -> ::core::ffi::c_int {
    let mut sum: ::core::ffi::c_int = sa8d_8x8(pix1, i_pix1, pix2, i_pix2)
        + sa8d_8x8(
            pix1.offset(8 as ::core::ffi::c_int as isize),
            i_pix1,
            pix2.offset(8 as ::core::ffi::c_int as isize),
            i_pix2,
        )
        + sa8d_8x8(
            pix1.offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2.offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        )
        + sa8d_8x8(
            pix1
                .offset(8 as ::core::ffi::c_int as isize)
                .offset((8 as intptr_t * i_pix1) as isize),
            i_pix1,
            pix2
                .offset(8 as ::core::ffi::c_int as isize)
                .offset((8 as intptr_t * i_pix2) as isize),
            i_pix2,
        );
    return sum + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
}
#[inline(never)]
#[c2rust::src_loc = "383:1"]
unsafe extern "C" fn pixel_hadamard_ac(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut tmp: [sum2_t; 32] = [0; 32];
    let mut a0: sum2_t = 0;
    let mut a1: sum2_t = 0;
    let mut a2: sum2_t = 0;
    let mut a3: sum2_t = 0;
    let mut dc: sum2_t = 0;
    let mut sum4: sum2_t = 0 as sum2_t;
    let mut sum8: sum2_t = 0 as sum2_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        let mut t: *mut sum2_t = tmp
            .as_mut_ptr()
            .offset((i & 3 as ::core::ffi::c_int) as isize)
            .offset(((i & 4 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize);
        a0 = ((*pix.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *pix.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t)
            .wrapping_add(
                ((*pix.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *pix.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as sum2_t) << BITS_PER_SUM,
            );
        a1 = ((*pix.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *pix.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t)
            .wrapping_add(
                ((*pix.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *pix.offset(3 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as sum2_t) << BITS_PER_SUM,
            );
        *t.offset(0 as ::core::ffi::c_int as isize) = a0.wrapping_add(a1);
        *t.offset(4 as ::core::ffi::c_int as isize) = a0.wrapping_sub(a1);
        a2 = ((*pix.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *pix.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t)
            .wrapping_add(
                ((*pix.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *pix.offset(5 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as sum2_t) << BITS_PER_SUM,
            );
        a3 = ((*pix.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            + *pix.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as sum2_t)
            .wrapping_add(
                ((*pix.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - *pix.offset(7 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int) as sum2_t) << BITS_PER_SUM,
            );
        *t.offset(8 as ::core::ffi::c_int as isize) = a2.wrapping_add(a3);
        *t.offset(12 as ::core::ffi::c_int as isize) = a2.wrapping_sub(a3);
        i += 1;
        pix = pix.offset(stride as isize);
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 8 as ::core::ffi::c_int {
        let mut t0: sum2_t = tmp[(i_0 * 4 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                tmp[(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize],
            );
        let mut t1: sum2_t = tmp[(i_0 * 4 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int) as usize]
            .wrapping_sub(
                tmp[(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize],
            );
        let mut t2: sum2_t = tmp[(i_0 * 4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize]
            .wrapping_add(
                tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize],
            );
        let mut t3: sum2_t = tmp[(i_0 * 4 as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize]
            .wrapping_sub(
                tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize],
            );
        a0 = t0.wrapping_add(t2);
        a2 = t0.wrapping_sub(t2);
        a1 = t1.wrapping_add(t3);
        a3 = t1.wrapping_sub(t3);
        tmp[(i_0 * 4 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize] = a0;
        tmp[(i_0 * 4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] = a1;
        tmp[(i_0 * 4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize] = a2;
        tmp[(i_0 * 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as usize] = a3;
        sum4 = sum4
            .wrapping_add(
                abs2(a0)
                    .wrapping_add(abs2(a1))
                    .wrapping_add(abs2(a2))
                    .wrapping_add(abs2(a3)),
            );
        i_0 += 1;
    }
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_1 < 8 as ::core::ffi::c_int {
        let mut t0_0: sum2_t = tmp[i_1 as usize]
            .wrapping_add(tmp[(8 as ::core::ffi::c_int + i_1) as usize]);
        let mut t1_0: sum2_t = tmp[i_1 as usize]
            .wrapping_sub(tmp[(8 as ::core::ffi::c_int + i_1) as usize]);
        let mut t2_0: sum2_t = tmp[(16 as ::core::ffi::c_int + i_1) as usize]
            .wrapping_add(tmp[(24 as ::core::ffi::c_int + i_1) as usize]);
        let mut t3_0: sum2_t = tmp[(16 as ::core::ffi::c_int + i_1) as usize]
            .wrapping_sub(tmp[(24 as ::core::ffi::c_int + i_1) as usize]);
        a0 = t0_0.wrapping_add(t2_0);
        a2 = t0_0.wrapping_sub(t2_0);
        a1 = t1_0.wrapping_add(t3_0);
        a3 = t1_0.wrapping_sub(t3_0);
        sum8 = sum8
            .wrapping_add(
                abs2(a0)
                    .wrapping_add(abs2(a1))
                    .wrapping_add(abs2(a2))
                    .wrapping_add(abs2(a3)),
            );
        i_1 += 1;
    }
    dc = tmp[0 as ::core::ffi::c_int as usize]
        .wrapping_add(tmp[8 as ::core::ffi::c_int as usize])
        .wrapping_add(tmp[16 as ::core::ffi::c_int as usize])
        .wrapping_add(tmp[24 as ::core::ffi::c_int as usize]) as sum_t as sum2_t;
    sum4 = (sum4 as sum_t as sum2_t).wrapping_add(sum4 >> BITS_PER_SUM).wrapping_sub(dc);
    sum8 = (sum8 as sum_t as sum2_t).wrapping_add(sum8 >> BITS_PER_SUM).wrapping_sub(dc);
    return (sum8 << 32 as ::core::ffi::c_int).wrapping_add(sum4 as uint64_t);
}
#[c2rust::src_loc = "432:1"]
unsafe extern "C" fn x264_pixel_hadamard_ac_16x16(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset(8 as ::core::ffi::c_int as isize), stride),
            );
    }
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset((8 as intptr_t * stride) as isize), stride),
            );
    }
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        && 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
    {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix
                        .offset((8 as intptr_t * stride) as isize)
                        .offset(8 as ::core::ffi::c_int as isize),
                    stride,
                ),
            );
    }
    return ((sum >> 34 as ::core::ffi::c_int) << 32 as ::core::ffi::c_int)
        .wrapping_add((sum as uint32_t >> 1 as ::core::ffi::c_int) as uint64_t);
}
#[c2rust::src_loc = "433:1"]
unsafe extern "C" fn x264_pixel_hadamard_ac_16x8(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset(8 as ::core::ffi::c_int as isize), stride),
            );
    }
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset((8 as intptr_t * stride) as isize), stride),
            );
    }
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        && 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
    {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix
                        .offset((8 as intptr_t * stride) as isize)
                        .offset(8 as ::core::ffi::c_int as isize),
                    stride,
                ),
            );
    }
    return ((sum >> 34 as ::core::ffi::c_int) << 32 as ::core::ffi::c_int)
        .wrapping_add((sum as uint32_t >> 1 as ::core::ffi::c_int) as uint64_t);
}
#[c2rust::src_loc = "434:1"]
unsafe extern "C" fn x264_pixel_hadamard_ac_8x16(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset(8 as ::core::ffi::c_int as isize), stride),
            );
    }
    if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset((8 as intptr_t * stride) as isize), stride),
            );
    }
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        && 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
    {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix
                        .offset((8 as intptr_t * stride) as isize)
                        .offset(8 as ::core::ffi::c_int as isize),
                    stride,
                ),
            );
    }
    return ((sum >> 34 as ::core::ffi::c_int) << 32 as ::core::ffi::c_int)
        .wrapping_add((sum as uint32_t >> 1 as ::core::ffi::c_int) as uint64_t);
}
#[c2rust::src_loc = "435:1"]
unsafe extern "C" fn x264_pixel_hadamard_ac_8x8(
    mut pix: *mut pixel,
    mut stride: intptr_t,
) -> uint64_t {
    let mut sum: uint64_t = pixel_hadamard_ac(pix, stride);
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset(8 as ::core::ffi::c_int as isize), stride),
            );
    }
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(pix.offset((8 as intptr_t * stride) as isize), stride),
            );
    }
    if 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
        && 8 as ::core::ffi::c_int == 16 as ::core::ffi::c_int
    {
        sum = sum
            .wrapping_add(
                pixel_hadamard_ac(
                    pix
                        .offset((8 as intptr_t * stride) as isize)
                        .offset(8 as ::core::ffi::c_int as isize),
                    stride,
                ),
            );
    }
    return ((sum >> 34 as ::core::ffi::c_int) << 32 as ::core::ffi::c_int)
        .wrapping_add((sum as uint32_t >> 1 as ::core::ffi::c_int) as uint64_t);
}
#[c2rust::src_loc = "458:1"]
unsafe extern "C" fn x264_pixel_sad_x3_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "458:1"]
unsafe extern "C" fn x264_pixel_sad_x4_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "459:1"]
unsafe extern "C" fn x264_pixel_sad_x4_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "459:1"]
unsafe extern "C" fn x264_pixel_sad_x3_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "460:1"]
unsafe extern "C" fn x264_pixel_sad_x4_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "460:1"]
unsafe extern "C" fn x264_pixel_sad_x3_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "461:1"]
unsafe extern "C" fn x264_pixel_sad_x3_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "461:1"]
unsafe extern "C" fn x264_pixel_sad_x4_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "462:1"]
unsafe extern "C" fn x264_pixel_sad_x3_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "462:1"]
unsafe extern "C" fn x264_pixel_sad_x4_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "463:1"]
unsafe extern "C" fn x264_pixel_sad_x3_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "463:1"]
unsafe extern "C" fn x264_pixel_sad_x4_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "464:1"]
unsafe extern "C" fn x264_pixel_sad_x4_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "464:1"]
unsafe extern "C" fn x264_pixel_sad_x3_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_8x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_4x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x4_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut pix3: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
    *scores.offset(3 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix3,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_4x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_8x4(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x4(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_8x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_16x8(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x8(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "498:1"]
unsafe extern "C" fn x264_pixel_satd_x3_16x16(
    mut fenc: *mut pixel,
    mut pix0: *mut pixel,
    mut pix1: *mut pixel,
    mut pix2: *mut pixel,
    mut i_stride: intptr_t,
    mut scores: *mut ::core::ffi::c_int,
) {
    *scores.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix0,
        i_stride,
    );
    *scores.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix1,
        i_stride,
    );
    *scores.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
        fenc,
        FENC_STRIDE as intptr_t,
        pix2,
        i_stride,
    );
}
#[c2rust::src_loc = "530:1"]
unsafe extern "C" fn intra_sad_x3_8x8(
    mut fenc: *mut pixel,
    mut edge: *mut pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    let mut pix: [pixel; 256] = [0; 256];
    x264_10_predict_8x8_v_c(pix.as_mut_ptr(), edge);
    *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8_h_c(pix.as_mut_ptr(), edge);
    *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8_dc_c(pix.as_mut_ptr(), edge);
    *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "531:1"]
unsafe extern "C" fn intra_sa8d_x3_8x8(
    mut fenc: *mut pixel,
    mut edge: *mut pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    let mut pix: [pixel; 256] = [0; 256];
    x264_10_predict_8x8_v_c(pix.as_mut_ptr(), edge);
    *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sa8d_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8_h_c(pix.as_mut_ptr(), edge);
    *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sa8d_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8_dc_c(pix.as_mut_ptr(), edge);
    *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sa8d_8x8(
        pix.as_mut_ptr(),
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "553:1"]
unsafe extern "C" fn intra_sad_x3_4x4(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    x264_10_predict_4x4_v_c(fdec);
    *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_4x4_h_c(fdec);
    *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_4x4_dc_c(fdec);
    *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_4x4(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "554:1"]
unsafe extern "C" fn intra_satd_x3_4x4(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    x264_10_predict_4x4_v_c(fdec);
    *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_4x4_h_c(fdec);
    *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_4x4_dc_c(fdec);
    *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_4x4(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "555:1"]
unsafe extern "C" fn intra_sad_x3_8x8c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    x264_10_predict_8x8c_dc_c(fdec);
    *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8c_h_c(fdec);
    *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8c_v_c(fdec);
    *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x8(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "556:1"]
unsafe extern "C" fn intra_satd_x3_8x8c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    x264_10_predict_8x8c_dc_c(fdec);
    *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8c_h_c(fdec);
    *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x8c_v_c(fdec);
    *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x8(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "557:1"]
unsafe extern "C" fn intra_sad_x3_8x16c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    x264_10_predict_8x16c_dc_c(fdec);
    *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x16c_h_c(fdec);
    *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x16c_v_c(fdec);
    *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_8x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "558:1"]
unsafe extern "C" fn intra_satd_x3_8x16c(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    x264_10_predict_8x16c_dc_c(fdec);
    *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x16c_h_c(fdec);
    *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_8x16c_v_c(fdec);
    *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_8x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "559:1"]
unsafe extern "C" fn intra_sad_x3_16x16(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    x264_10_predict_16x16_v_c(fdec);
    *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_16x16_h_c(fdec);
    *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_16x16_dc_c(fdec);
    *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_sad_16x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "560:1"]
unsafe extern "C" fn intra_satd_x3_16x16(
    mut fenc: *mut pixel,
    mut fdec: *mut pixel,
    mut res: *mut ::core::ffi::c_int,
) {
    x264_10_predict_16x16_v_c(fdec);
    *res.offset(0 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_16x16_h_c(fdec);
    *res.offset(1 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
    x264_10_predict_16x16_dc_c(fdec);
    *res.offset(2 as ::core::ffi::c_int as isize) = x264_pixel_satd_16x16(
        fdec,
        FDEC_STRIDE as intptr_t,
        fenc,
        FENC_STRIDE as intptr_t,
    );
}
#[c2rust::src_loc = "627:1"]
unsafe extern "C" fn ssim_4x4x2_core(
    mut pix1: *const pixel,
    mut stride1: intptr_t,
    mut pix2: *const pixel,
    mut stride2: intptr_t,
    mut sums: *mut [::core::ffi::c_int; 4],
) {
    let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while z < 2 as ::core::ffi::c_int {
        let mut s1: uint32_t = 0 as uint32_t;
        let mut s2: uint32_t = 0 as uint32_t;
        let mut ss: uint32_t = 0 as uint32_t;
        let mut s12: uint32_t = 0 as uint32_t;
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < 4 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < 4 as ::core::ffi::c_int {
                let mut a: ::core::ffi::c_int = *pix1
                    .offset((x as intptr_t + y as intptr_t * stride1) as isize)
                    as ::core::ffi::c_int;
                let mut b: ::core::ffi::c_int = *pix2
                    .offset((x as intptr_t + y as intptr_t * stride2) as isize)
                    as ::core::ffi::c_int;
                s1 = s1.wrapping_add(a as uint32_t);
                s2 = s2.wrapping_add(b as uint32_t);
                ss = ss.wrapping_add((a * a) as uint32_t);
                ss = ss.wrapping_add((b * b) as uint32_t);
                s12 = s12.wrapping_add((a * b) as uint32_t);
                x += 1;
            }
            y += 1;
        }
        (*sums.offset(z as isize))[0 as ::core::ffi::c_int as usize] = s1
            as ::core::ffi::c_int;
        (*sums.offset(z as isize))[1 as ::core::ffi::c_int as usize] = s2
            as ::core::ffi::c_int;
        (*sums.offset(z as isize))[2 as ::core::ffi::c_int as usize] = ss
            as ::core::ffi::c_int;
        (*sums.offset(z as isize))[3 as ::core::ffi::c_int as usize] = s12
            as ::core::ffi::c_int;
        pix1 = pix1.offset(4 as ::core::ffi::c_int as isize);
        pix2 = pix2.offset(4 as ::core::ffi::c_int as isize);
        z += 1;
    }
}
#[c2rust::src_loc = "654:1"]
unsafe extern "C" fn ssim_end1(
    mut s1: ::core::ffi::c_int,
    mut s2: ::core::ffi::c_int,
    mut ss: ::core::ffi::c_int,
    mut s12: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    static mut ssim_c1: ::core::ffi::c_float = (0.01f64 * 0.01f64
        * PIXEL_MAX as ::core::ffi::c_double * PIXEL_MAX as ::core::ffi::c_double
        * 64 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_float;
    static mut ssim_c2: ::core::ffi::c_float = (0.03f64 * 0.03f64
        * PIXEL_MAX as ::core::ffi::c_double * PIXEL_MAX as ::core::ffi::c_double
        * 64 as ::core::ffi::c_int as ::core::ffi::c_double
        * 63 as ::core::ffi::c_int as ::core::ffi::c_double) as ::core::ffi::c_float;
    let mut fs1: ::core::ffi::c_float = s1 as ::core::ffi::c_float;
    let mut fs2: ::core::ffi::c_float = s2 as ::core::ffi::c_float;
    let mut fss: ::core::ffi::c_float = ss as ::core::ffi::c_float;
    let mut fs12: ::core::ffi::c_float = s12 as ::core::ffi::c_float;
    let mut vars: ::core::ffi::c_float = fss
        * 64 as ::core::ffi::c_int as ::core::ffi::c_float - fs1 * fs1 - fs2 * fs2;
    let mut covar: ::core::ffi::c_float = fs12
        * 64 as ::core::ffi::c_int as ::core::ffi::c_float - fs1 * fs2;
    return (2 as ::core::ffi::c_int as ::core::ffi::c_float * fs1 * fs2 + ssim_c1)
        * (2 as ::core::ffi::c_int as ::core::ffi::c_float * covar + ssim_c2)
        / ((fs1 * fs1 + fs2 * fs2 + ssim_c1) * (vars + ssim_c2));
}
#[c2rust::src_loc = "679:1"]
unsafe extern "C" fn ssim_end4(
    mut sum0: *mut [::core::ffi::c_int; 4],
    mut sum1: *mut [::core::ffi::c_int; 4],
    mut width: ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    let mut ssim: ::core::ffi::c_float = 0.0f32;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < width {
        ssim
            += ssim_end1(
                (*sum0.offset(i as isize))[0 as ::core::ffi::c_int as usize]
                    + (*sum0
                        .offset(
                            (i + 1 as ::core::ffi::c_int) as isize,
                        ))[0 as ::core::ffi::c_int as usize]
                    + (*sum1.offset(i as isize))[0 as ::core::ffi::c_int as usize]
                    + (*sum1
                        .offset(
                            (i + 1 as ::core::ffi::c_int) as isize,
                        ))[0 as ::core::ffi::c_int as usize],
                (*sum0.offset(i as isize))[1 as ::core::ffi::c_int as usize]
                    + (*sum0
                        .offset(
                            (i + 1 as ::core::ffi::c_int) as isize,
                        ))[1 as ::core::ffi::c_int as usize]
                    + (*sum1.offset(i as isize))[1 as ::core::ffi::c_int as usize]
                    + (*sum1
                        .offset(
                            (i + 1 as ::core::ffi::c_int) as isize,
                        ))[1 as ::core::ffi::c_int as usize],
                (*sum0.offset(i as isize))[2 as ::core::ffi::c_int as usize]
                    + (*sum0
                        .offset(
                            (i + 1 as ::core::ffi::c_int) as isize,
                        ))[2 as ::core::ffi::c_int as usize]
                    + (*sum1.offset(i as isize))[2 as ::core::ffi::c_int as usize]
                    + (*sum1
                        .offset(
                            (i + 1 as ::core::ffi::c_int) as isize,
                        ))[2 as ::core::ffi::c_int as usize],
                (*sum0.offset(i as isize))[3 as ::core::ffi::c_int as usize]
                    + (*sum0
                        .offset(
                            (i + 1 as ::core::ffi::c_int) as isize,
                        ))[3 as ::core::ffi::c_int as usize]
                    + (*sum1.offset(i as isize))[3 as ::core::ffi::c_int as usize]
                    + (*sum1
                        .offset(
                            (i + 1 as ::core::ffi::c_int) as isize,
                        ))[3 as ::core::ffi::c_int as usize],
            );
        i += 1;
    }
    return ssim;
}
#[no_mangle]
#[c2rust::src_loc = "690:1"]
pub unsafe extern "C" fn x264_10_pixel_ssim_wxh(
    mut pf: *mut x264_pixel_function_t,
    mut pix1: *mut pixel,
    mut stride1: intptr_t,
    mut pix2: *mut pixel,
    mut stride2: intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_void,
    mut cnt: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_float {
    let mut z: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ssim: ::core::ffi::c_float = 0.0f32;
    let mut sum0: *mut [::core::ffi::c_int; 4] = buf as *mut [::core::ffi::c_int; 4];
    let mut sum1: *mut [::core::ffi::c_int; 4] = sum0
        .offset((width >> 2 as ::core::ffi::c_int) as isize)
        .offset(3 as ::core::ffi::c_int as isize);
    width >>= 2 as ::core::ffi::c_int;
    height >>= 2 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while y < height {
        while z <= y {
            let mut t: *mut ::core::ffi::c_void = sum0 as *mut ::core::ffi::c_void;
            sum0 = sum1;
            sum1 = t as *mut [::core::ffi::c_int; 4];
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < width {
                (*pf)
                    .ssim_4x4x2_core
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *pix1
                        .offset(
                            (4 as intptr_t * (x as intptr_t + z as intptr_t * stride1))
                                as isize,
                        ),
                    stride1,
                    &mut *pix2
                        .offset(
                            (4 as intptr_t * (x as intptr_t + z as intptr_t * stride2))
                                as isize,
                        ),
                    stride2,
                    &mut *sum0.offset(x as isize),
                );
                x += 2 as ::core::ffi::c_int;
            }
            z += 1;
        }
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < width - 1 as ::core::ffi::c_int {
            ssim
                += (*pf)
                    .ssim_end4
                    .expect(
                        "non-null function pointer",
                    )(
                    sum0.offset(x_0 as isize),
                    sum1.offset(x_0 as isize),
                    if (4 as ::core::ffi::c_int) < width - x_0 - 1 as ::core::ffi::c_int
                    {
                        4 as ::core::ffi::c_int
                    } else {
                        width - x_0 - 1 as ::core::ffi::c_int
                    },
                );
            x_0 += 4 as ::core::ffi::c_int;
        }
        y += 1;
    }
    *cnt = (height - 1 as ::core::ffi::c_int) * (width - 1 as ::core::ffi::c_int);
    return ssim;
}
#[c2rust::src_loc = "716:1"]
unsafe extern "C" fn pixel_vsad(
    mut src: *mut pixel,
    mut stride: intptr_t,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut score: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < height {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < 16 as ::core::ffi::c_int {
            score
                += abs(
                    *src.offset(j as isize) as ::core::ffi::c_int
                        - *src.offset((j as intptr_t + stride) as isize)
                            as ::core::ffi::c_int,
                );
            j += 1;
        }
        i += 1;
        src = src.offset(stride as isize);
    }
    return score;
}
#[no_mangle]
#[c2rust::src_loc = "725:1"]
pub unsafe extern "C" fn x264_10_field_vsad(
    mut h: *mut x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut score_field: ::core::ffi::c_int = 0;
    let mut score_frame: ::core::ffi::c_int = 0;
    let mut stride: ::core::ffi::c_int = (*(*h).fenc)
        .i_stride[0 as ::core::ffi::c_int as usize];
    let mut mb_stride: ::core::ffi::c_int = (*h).mb.i_mb_stride;
    let mut fenc: *mut pixel = (*(*h).fenc)
        .plane[0 as ::core::ffi::c_int as usize]
        .offset((16 as ::core::ffi::c_int * (mb_x + mb_y * stride)) as isize);
    let mut mb_xy: ::core::ffi::c_int = mb_x + mb_y * mb_stride;
    let mut mbpair_height: ::core::ffi::c_int = if ((*h).param.i_height
        - mb_y * 16 as ::core::ffi::c_int) < 32 as ::core::ffi::c_int
    {
        (*h).param.i_height - mb_y * 16 as ::core::ffi::c_int
    } else {
        32 as ::core::ffi::c_int
    };
    score_frame = (*h)
        .pixf
        .vsad
        .expect("non-null function pointer")(fenc, stride as intptr_t, mbpair_height);
    score_field = (*h)
        .pixf
        .vsad
        .expect(
            "non-null function pointer",
        )(
        fenc,
        (stride * 2 as ::core::ffi::c_int) as intptr_t,
        mbpair_height >> 1 as ::core::ffi::c_int,
    );
    score_field
        += (*h)
            .pixf
            .vsad
            .expect(
                "non-null function pointer",
            )(
            fenc.offset(stride as isize),
            (stride * 2 as ::core::ffi::c_int) as intptr_t,
            mbpair_height >> 1 as ::core::ffi::c_int,
        );
    if mb_x > 0 as ::core::ffi::c_int {
        score_field
            += 512 as ::core::ffi::c_int
                - *(*h).mb.field.offset((mb_xy - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
    }
    if mb_y > 0 as ::core::ffi::c_int {
        score_field
            += 512 as ::core::ffi::c_int
                - *(*h).mb.field.offset((mb_xy - mb_stride) as isize)
                    as ::core::ffi::c_int * 1024 as ::core::ffi::c_int;
    }
    return (score_field < score_frame) as ::core::ffi::c_int;
}
#[c2rust::src_loc = "747:1"]
unsafe extern "C" fn pixel_asd8(
    mut pix1: *mut pixel,
    mut stride1: intptr_t,
    mut pix2: *mut pixel,
    mut stride2: intptr_t,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            sum
                += *pix1.offset(x as isize) as ::core::ffi::c_int
                    - *pix2.offset(x as isize) as ::core::ffi::c_int;
            x += 1;
        }
        y += 1;
        pix1 = pix1.offset(stride1 as isize);
        pix2 = pix2.offset(stride2 as isize);
    }
    return abs(sum);
}
#[c2rust::src_loc = "759:1"]
unsafe extern "C" fn x264_pixel_ads4(
    mut enc_dc: *mut ::core::ffi::c_int,
    mut sums: *mut uint16_t,
    mut delta: ::core::ffi::c_int,
    mut cost_mvx: *mut uint16_t,
    mut mvs: *mut int16_t,
    mut width: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nmv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < width {
        let mut ads: ::core::ffi::c_int = abs(
            *enc_dc.offset(0 as ::core::ffi::c_int as isize)
                - *sums.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        )
            + abs(
                *enc_dc.offset(1 as ::core::ffi::c_int as isize)
                    - *sums.offset(8 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int,
            )
            + abs(
                *enc_dc.offset(2 as ::core::ffi::c_int as isize)
                    - *sums.offset(delta as isize) as ::core::ffi::c_int,
            )
            + abs(
                *enc_dc.offset(3 as ::core::ffi::c_int as isize)
                    - *sums.offset((delta + 8 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int,
            ) + *cost_mvx.offset(i as isize) as ::core::ffi::c_int;
        if ads < thresh {
            let fresh2 = nmv;
            nmv = nmv + 1;
            *mvs.offset(fresh2 as isize) = i as int16_t;
        }
        i += 1;
        sums = sums.offset(1);
    }
    return nmv;
}
#[c2rust::src_loc = "776:1"]
unsafe extern "C" fn x264_pixel_ads2(
    mut enc_dc: *mut ::core::ffi::c_int,
    mut sums: *mut uint16_t,
    mut delta: ::core::ffi::c_int,
    mut cost_mvx: *mut uint16_t,
    mut mvs: *mut int16_t,
    mut width: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nmv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < width {
        let mut ads: ::core::ffi::c_int = abs(
            *enc_dc.offset(0 as ::core::ffi::c_int as isize)
                - *sums.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        )
            + abs(
                *enc_dc.offset(1 as ::core::ffi::c_int as isize)
                    - *sums.offset(delta as isize) as ::core::ffi::c_int,
            ) + *cost_mvx.offset(i as isize) as ::core::ffi::c_int;
        if ads < thresh {
            let fresh1 = nmv;
            nmv = nmv + 1;
            *mvs.offset(fresh1 as isize) = i as int16_t;
        }
        i += 1;
        sums = sums.offset(1);
    }
    return nmv;
}
#[c2rust::src_loc = "791:1"]
unsafe extern "C" fn x264_pixel_ads1(
    mut enc_dc: *mut ::core::ffi::c_int,
    mut sums: *mut uint16_t,
    mut delta: ::core::ffi::c_int,
    mut cost_mvx: *mut uint16_t,
    mut mvs: *mut int16_t,
    mut width: ::core::ffi::c_int,
    mut thresh: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nmv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < width {
        let mut ads: ::core::ffi::c_int = abs(
            *enc_dc.offset(0 as ::core::ffi::c_int as isize)
                - *sums.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) + *cost_mvx.offset(i as isize) as ::core::ffi::c_int;
        if ads < thresh {
            let fresh0 = nmv;
            nmv = nmv + 1;
            *mvs.offset(fresh0 as isize) = i as int16_t;
        }
        i += 1;
        sums = sums.offset(1);
    }
    return nmv;
}
#[no_mangle]
#[c2rust::src_loc = "809:1"]
pub unsafe extern "C" fn x264_10_pixel_init(
    mut cpu: uint32_t,
    mut pixf: *mut x264_pixel_function_t,
) {
    memset(
        pixf as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<x264_pixel_function_t>() as size_t,
    );
    (*pixf).sad[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad[PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_4x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_aligned[PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_4x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sad_x3[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x3_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x3_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x3_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x3_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x3_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x3_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x3[PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x3_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).sad_x4[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x4_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x4_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x4_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x4_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x4_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x4_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).sad_x4[PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sad_x4_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).ssd[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_ssd_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_ssd_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_ssd_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_ssd_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_ssd_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_ssd_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_ssd_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).ssd[PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_ssd_4x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd[PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_4x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).satd_x3[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x3_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x3_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x3_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x3_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x3_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x3_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x3[PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x3_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x3_t;
    (*pixf).satd_x4[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x4_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x4_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x4_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x4_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x4_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x4_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).satd_x4[PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_satd_x4_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                *mut ::core::ffi::c_int,
            ) -> (),
    ) as x264_pixel_cmp_x4_t;
    (*pixf).hadamard_ac[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_hadamard_ac_16x16
            as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    ) as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).hadamard_ac[PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_hadamard_ac_16x8
            as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    ) as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).hadamard_ac[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_hadamard_ac_8x16
            as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    ) as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).hadamard_ac[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_hadamard_ac_8x8
            as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    ) as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).ads[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_ads4
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_int,
                *mut uint16_t,
                ::core::ffi::c_int,
                *mut uint16_t,
                *mut int16_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_int,
                *mut uint16_t,
                ::core::ffi::c_int,
                *mut uint16_t,
                *mut int16_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    (*pixf).ads[PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_ads2
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_int,
                *mut uint16_t,
                ::core::ffi::c_int,
                *mut uint16_t,
                *mut int16_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_int,
                *mut uint16_t,
                ::core::ffi::c_int,
                *mut uint16_t,
                *mut int16_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    (*pixf).ads[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_ads1
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_int,
                *mut uint16_t,
                ::core::ffi::c_int,
                *mut uint16_t,
                *mut int16_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_int,
                *mut uint16_t,
                ::core::ffi::c_int,
                *mut uint16_t,
                *mut int16_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    (*pixf).sa8d[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sa8d_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).sa8d[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        x264_pixel_sa8d_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
            ) -> ::core::ffi::c_int,
    ) as x264_pixel_cmp_t;
    (*pixf).var[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        pixel_var_16x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    ) as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).var[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        pixel_var_8x16 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    ) as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).var[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        pixel_var_8x8 as unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t,
    ) as Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>;
    (*pixf).var2[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        pixel_var2_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    (*pixf).var2[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        pixel_var2_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    (*pixf).ssd_nv12_core = Some(
        pixel_ssd_nv12_core
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut uint64_t,
                *mut uint64_t,
            ) -> (),
    )
        as Option<
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
        >;
    (*pixf).ssim_4x4x2_core = Some(
        ssim_4x4x2_core
            as unsafe extern "C" fn(
                *const pixel,
                intptr_t,
                *const pixel,
                intptr_t,
                *mut [::core::ffi::c_int; 4],
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *const pixel,
                intptr_t,
                *const pixel,
                intptr_t,
                *mut [::core::ffi::c_int; 4],
            ) -> (),
        >;
    (*pixf).ssim_end4 = Some(
        ssim_end4
            as unsafe extern "C" fn(
                *mut [::core::ffi::c_int; 4],
                *mut [::core::ffi::c_int; 4],
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_float,
    )
        as Option<
            unsafe extern "C" fn(
                *mut [::core::ffi::c_int; 4],
                *mut [::core::ffi::c_int; 4],
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_float,
        >;
    (*pixf).vsad = Some(
        pixel_vsad
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    (*pixf).asd8 = Some(
        pixel_asd8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    (*pixf).intra_sad_x3_4x4 = Some(
        intra_sad_x3_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >;
    (*pixf).intra_satd_x3_4x4 = Some(
        intra_satd_x3_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >;
    (*pixf).intra_sad_x3_8x8 = Some(
        intra_sad_x3_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >;
    (*pixf).intra_sa8d_x3_8x8 = Some(
        intra_sa8d_x3_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >;
    (*pixf).intra_sad_x3_8x8c = Some(
        intra_sad_x3_8x8c
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >;
    (*pixf).intra_satd_x3_8x8c = Some(
        intra_satd_x3_8x8c
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >;
    (*pixf).intra_sad_x3_8x16c = Some(
        intra_sad_x3_8x16c
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >;
    (*pixf).intra_satd_x3_8x16c = Some(
        intra_satd_x3_8x16c
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >;
    (*pixf).intra_sad_x3_16x16 = Some(
        intra_sad_x3_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >;
    (*pixf).intra_satd_x3_16x16 = Some(
        intra_satd_x3_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> (),
        >;
    (*pixf).ads[PIXEL_4x8 as ::core::ffi::c_int as usize] = (*pixf)
        .ads[PIXEL_16x8 as ::core::ffi::c_int as usize];
    (*pixf).ads[PIXEL_8x4 as ::core::ffi::c_int as usize] = (*pixf)
        .ads[PIXEL_4x8 as ::core::ffi::c_int as usize];
    (*pixf).ads[PIXEL_8x16 as ::core::ffi::c_int as usize] = (*pixf)
        .ads[PIXEL_8x4 as ::core::ffi::c_int as usize];
    (*pixf).ads[PIXEL_4x4 as ::core::ffi::c_int as usize] = (*pixf)
        .ads[PIXEL_8x8 as ::core::ffi::c_int as usize];
}
