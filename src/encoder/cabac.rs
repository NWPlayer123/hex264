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
    use super::types_h::{__int16_t, __int32_t, __int64_t, __int8_t};
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
    use super::types_h::{__uint16_t, __uint32_t, __uint64_t, __uint8_t};
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
    #[c2rust::src_loc = "58:9"]
    pub const QP_BD_OFFSET: ::core::ffi::c_int =
        6 as ::core::ffi::c_int * (BIT_DEPTH - 8 as ::core::ffi::c_int);
    #[c2rust::src_loc = "59:9"]
    pub const QP_MAX_SPEC: ::core::ffi::c_int = 51 as ::core::ffi::c_int + QP_BD_OFFSET;
    #[c2rust::src_loc = "570:9"]
    pub const FENC_STRIDE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    use super::bitstream_h::{bs_t, x264_bitstream_function_t};
    use super::cabac_h::x264_cabac_t;
    use super::dct_h::{x264_dct_function_t, x264_zigzag_function_t};
    use super::frame_h::{x264_deblock_function_t, x264_frame_t, x264_sync_frame_list_t};
    use super::internal::BIT_DEPTH;
    use super::mc_h::{x264_mc_functions_t, x264_weight_t};
    use super::pixel_h::x264_pixel_function_t;
    use super::predict_h::{x264_predict8x8_t, x264_predict_8x8_filter_t, x264_predict_t};
    use super::pthreadtypes_h::{pthread_cond_t, pthread_mutex_t, pthread_t};
    use super::quant_h::x264_quant_function_t;
    use super::set_h::{x264_pps_t, x264_sps_t};
    use super::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
    use super::threadpool_h::x264_threadpool_t;
    use super::x264_h::{x264_nal_t, x264_param_t};
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
        unsafe extern "C" fn(*mut pixel, intptr_t, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
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
    use super::common_h::pixel;
    use super::mc_h::x264_weight_t;
    use super::pthreadtypes_h::{pthread_cond_t, pthread_mutex_t};
    use super::stdint_h::intptr_t;
    use super::stdint_intn_h::{int16_t, int64_t, int8_t};
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
    use super::x264_h::{x264_hrd_t, x264_param_t, x264_sei_t};
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
            unsafe extern "C" fn(*mut x264_t, *mut x264_nal_t, *mut ::core::ffi::c_void) -> (),
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
    use super::common_h::x264_t;
    use super::internal::__va_list_tag;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
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
            unsafe extern "C" fn(*mut pixel, *mut pixel, intptr_t, ::core::ffi::c_int) -> (),
        >,
        pub load_deinterleave_chroma_fdec: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, intptr_t, ::core::ffi::c_int) -> (),
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
        pub prefetch_ref:
            Option<unsafe extern "C" fn(*mut pixel, intptr_t, ::core::ffi::c_int) -> ()>,
        pub memcpy_aligned: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                size_t,
            ) -> *mut ::core::ffi::c_void,
        >,
        pub memzero_aligned: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>,
        pub integral_init4h:
            Option<unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> ()>,
        pub integral_init8h:
            Option<unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> ()>,
        pub integral_init4v:
            Option<unsafe extern "C" fn(*mut uint16_t, *mut uint16_t, intptr_t) -> ()>,
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
        pub weight_cache: Option<unsafe extern "C" fn(*mut x264_t, *mut x264_weight_t) -> ()>,
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
    use super::__stddef_size_t_h::size_t;
    use super::common_h::{pixel, x264_t};
    use super::stdint_h::intptr_t;
    use super::stdint_intn_h::{int16_t, int32_t};
    use super::stdint_uintn_h::{uint16_t, uint32_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/bitstream.h:28"]
pub mod bitstream_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "57:9"]
    pub struct x264_bitstream_function_t {
        pub nal_escape:
            Option<unsafe extern "C" fn(*mut uint8_t, *mut uint8_t, *mut uint8_t) -> *mut uint8_t>,
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
    #[inline]
    #[c2rust::src_loc = "86:1"]
    pub unsafe extern "C" fn bs_init(
        mut s: *mut bs_t,
        mut p_data: *mut ::core::ffi::c_void,
        mut i_data: ::core::ffi::c_int,
    ) {
        let mut offset: ::core::ffi::c_int =
            (p_data as intptr_t & 3 as intptr_t) as ::core::ffi::c_int;
        (*s).p_start = (p_data as *mut uint8_t).offset(-(offset as isize));
        (*s).p = (*s).p_start;
        (*s).p_end = (p_data as *mut uint8_t).offset(i_data as isize);
        (*s).i_left = WORD_SIZE
            .wrapping_sub(offset as uint64_t)
            .wrapping_mul(8 as uint64_t) as ::core::ffi::c_int;
        if offset != 0 {
            (*s).cur_bits = endian_fix32((*((*s).p as *mut x264_union32_t)).i) as uintptr_t;
            (*s).cur_bits >>= (4 as ::core::ffi::c_int - offset) * 8 as ::core::ffi::c_int;
        } else {
            (*s).cur_bits = 0 as uintptr_t;
        };
    }
    #[inline]
    #[c2rust::src_loc = "106:1"]
    pub unsafe extern "C" fn bs_flush(mut s: *mut bs_t) {
        (*((*s).p as *mut x264_union32_t)).i =
            endian_fix32(((*s).cur_bits << ((*s).i_left & 31 as ::core::ffi::c_int)) as uint32_t);
        (*s).p = (*s).p.offset(
            WORD_SIZE.wrapping_sub(((*s).i_left >> 3 as ::core::ffi::c_int) as uint64_t) as isize,
        );
        (*s).i_left = WORD_SIZE.wrapping_mul(8 as uint64_t) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "125:1"]
    pub unsafe extern "C" fn bs_write(
        mut s: *mut bs_t,
        mut i_count: ::core::ffi::c_int,
        mut i_bits: uint32_t,
    ) {
        if WORD_SIZE == 8 as uint64_t {
            (*s).cur_bits = (*s).cur_bits << i_count | i_bits as uintptr_t;
            (*s).i_left -= i_count;
            if (*s).i_left <= 32 as ::core::ffi::c_int {
                (*((*s).p as *mut x264_union32_t)).i =
                    endian_fix((*s).cur_bits << (*s).i_left) as uint32_t;
                (*s).i_left += 32 as ::core::ffi::c_int;
                (*s).p = (*s).p.offset(4 as ::core::ffi::c_int as isize);
            }
        } else if i_count < (*s).i_left {
            (*s).cur_bits = (*s).cur_bits << i_count | i_bits as uintptr_t;
            (*s).i_left -= i_count;
        } else {
            i_count -= (*s).i_left;
            (*s).cur_bits = (*s).cur_bits << (*s).i_left | (i_bits >> i_count) as uintptr_t;
            (*((*s).p as *mut x264_union32_t)).i = endian_fix((*s).cur_bits) as uint32_t;
            (*s).p = (*s).p.offset(4 as ::core::ffi::c_int as isize);
            (*s).cur_bits = i_bits as uintptr_t;
            (*s).i_left = 32 as ::core::ffi::c_int - i_count;
        };
    }
    use super::base_h::x264_union32_t;
    use super::cabac_h::x264_cabac_t;
    use super::common_h::dctcoef;
    use super::osdep_h::{endian_fix, endian_fix32, WORD_SIZE};
    use super::stdint_h::{intptr_t, uintptr_t};
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::{uint32_t, uint64_t, uint8_t};
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
    #[inline(always)]
    #[c2rust::src_loc = "94:1"]
    pub unsafe extern "C" fn x264_cabac_pos(mut cb: *mut x264_cabac_t) -> ::core::ffi::c_int {
        return (((*cb).p.offset_from((*cb).p_start) as ::core::ffi::c_long
            + (*cb).i_bytes_outstanding as ::core::ffi::c_long)
            * 8 as ::core::ffi::c_long
            + (*cb).i_queue as ::core::ffi::c_long) as ::core::ffi::c_int;
    }
    use super::common_h::x264_t;
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "59:1"]
        pub fn x264_10_cabac_encode_init_core(cb: *mut x264_cabac_t);
        #[c2rust::src_loc = "63:1"]
        pub fn x264_10_cabac_encode_decision_c(
            cb: *mut x264_cabac_t,
            i_ctx: ::core::ffi::c_int,
            b: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "67:1"]
        pub fn x264_10_cabac_encode_bypass_c(cb: *mut x264_cabac_t, b: ::core::ffi::c_int);
        #[c2rust::src_loc = "71:1"]
        pub fn x264_10_cabac_encode_terminal_c(cb: *mut x264_cabac_t);
        #[c2rust::src_loc = "75:1"]
        pub fn x264_10_cabac_encode_ue_bypass(
            cb: *mut x264_cabac_t,
            exp_bits: ::core::ffi::c_int,
            val: ::core::ffi::c_int,
        );
        #[c2rust::src_loc = "77:1"]
        pub fn x264_10_cabac_encode_flush(h: *mut x264_t, cb: *mut x264_cabac_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/quant.h:28"]
pub mod quant_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "30:9"]
    pub struct x264_quant_function_t {
        pub quant_8x8: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> ::core::ffi::c_int,
        >,
        pub quant_4x4: Option<
            unsafe extern "C" fn(*mut dctcoef, *mut udctcoef, *mut udctcoef) -> ::core::ffi::c_int,
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
        pub optimize_chroma_2x2_dc:
            Option<unsafe extern "C" fn(*mut dctcoef, ::core::ffi::c_int) -> ::core::ffi::c_int>,
        pub optimize_chroma_2x4_dc:
            Option<unsafe extern "C" fn(*mut dctcoef, ::core::ffi::c_int) -> ::core::ffi::c_int>,
        pub denoise_dct: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *mut uint32_t,
                *mut udctcoef,
                ::core::ffi::c_int,
            ) -> (),
        >,
        pub decimate_score15: Option<unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int>,
        pub decimate_score16: Option<unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int>,
        pub decimate_score64: Option<unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int>,
        pub coeff_last: [Option<unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int>; 14],
        pub coeff_last4: Option<unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int>,
        pub coeff_last8: Option<unsafe extern "C" fn(*mut dctcoef) -> ::core::ffi::c_int>,
        pub coeff_level_run: [Option<
            unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> ::core::ffi::c_int,
        >; 13],
        pub coeff_level_run4:
            Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> ::core::ffi::c_int>,
        pub coeff_level_run8:
            Option<unsafe extern "C" fn(*mut dctcoef, *mut x264_run_level_t) -> ::core::ffi::c_int>,
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
    use super::bitstream_h::x264_run_level_t;
    use super::common_h::{dctcoef, udctcoef};
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
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
            unsafe extern "C" fn(*mut dctcoef, *const pixel, *mut pixel) -> ::core::ffi::c_int,
        >,
        pub sub_4x4: Option<
            unsafe extern "C" fn(*mut dctcoef, *const pixel, *mut pixel) -> ::core::ffi::c_int,
        >,
        pub sub_4x4ac: Option<
            unsafe extern "C" fn(
                *mut dctcoef,
                *const pixel,
                *mut pixel,
                *mut dctcoef,
            ) -> ::core::ffi::c_int,
        >,
        pub interleave_8x8_cavlc:
            Option<unsafe extern "C" fn(*mut dctcoef, *mut dctcoef, *mut uint8_t) -> ()>,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "29:9"]
    pub struct x264_dct_function_t {
        pub sub4x4_dct: Option<unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ()>,
        pub add4x4_idct: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
        pub sub8x8_dct:
            Option<unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> ()>,
        pub sub8x8_dct_dc: Option<unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ()>,
        pub add8x8_idct: Option<unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> ()>,
        pub add8x8_idct_dc: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
        pub sub8x16_dct_dc:
            Option<unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ()>,
        pub sub16x16_dct:
            Option<unsafe extern "C" fn(*mut [dctcoef; 16], *mut pixel, *mut pixel) -> ()>,
        pub add16x16_idct: Option<unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 16]) -> ()>,
        pub add16x16_idct_dc: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
        pub sub8x8_dct8: Option<unsafe extern "C" fn(*mut dctcoef, *mut pixel, *mut pixel) -> ()>,
        pub add8x8_idct8: Option<unsafe extern "C" fn(*mut pixel, *mut dctcoef) -> ()>,
        pub sub16x16_dct8:
            Option<unsafe extern "C" fn(*mut [dctcoef; 64], *mut pixel, *mut pixel) -> ()>,
        pub add16x16_idct8: Option<unsafe extern "C" fn(*mut pixel, *mut [dctcoef; 64]) -> ()>,
        pub dct4x4dc: Option<unsafe extern "C" fn(*mut dctcoef) -> ()>,
        pub idct4x4dc: Option<unsafe extern "C" fn(*mut dctcoef) -> ()>,
        pub dct2x4dc: Option<unsafe extern "C" fn(*mut dctcoef, *mut [dctcoef; 16]) -> ()>,
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
            unsafe extern "C" fn(*mut pixel, intptr_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
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
        pub hadamard_ac: [Option<unsafe extern "C" fn(*mut pixel, intptr_t) -> uint64_t>; 4],
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
        pub intra_mbcmp_x3_16x16:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_satd_x3_16x16:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_sad_x3_16x16:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_mbcmp_x3_4x4:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_satd_x3_4x4:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_sad_x3_4x4:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_mbcmp_x3_chroma:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_satd_x3_chroma:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_sad_x3_chroma:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_mbcmp_x3_8x16c:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_satd_x3_8x16c:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_sad_x3_8x16c:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_mbcmp_x3_8x8c:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_satd_x3_8x8c:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_sad_x3_8x8c:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_mbcmp_x3_8x8:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_sa8d_x3_8x8:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_sad_x3_8x8:
            Option<unsafe extern "C" fn(*mut pixel, *mut pixel, *mut ::core::ffi::c_int) -> ()>,
        pub intra_mbcmp_x9_4x4: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut uint16_t) -> ::core::ffi::c_int,
        >,
        pub intra_satd_x9_4x4: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut uint16_t) -> ::core::ffi::c_int,
        >,
        pub intra_sad_x9_4x4: Option<
            unsafe extern "C" fn(*mut pixel, *mut pixel, *mut uint16_t) -> ::core::ffi::c_int,
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
        unsafe extern "C" fn(*mut pixel, intptr_t, *mut pixel, intptr_t) -> ::core::ffi::c_int,
    >;
    use super::common_h::pixel;
    use super::stdint_h::intptr_t;
    use super::stdint_intn_h::int16_t;
    use super::stdint_uintn_h::{uint16_t, uint64_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/predict.h:28"]
pub mod predict_h {
    #[c2rust::src_loc = "32:1"]
    pub type x264_predict_8x8_filter_t = Option<
        unsafe extern "C" fn(*mut pixel, *mut pixel, ::core::ffi::c_int, ::core::ffi::c_int) -> (),
    >;
    #[c2rust::src_loc = "30:1"]
    pub type x264_predict_t = Option<unsafe extern "C" fn(*mut pixel) -> ()>;
    #[c2rust::src_loc = "31:1"]
    pub type x264_predict8x8_t = Option<unsafe extern "C" fn(*mut pixel, *mut pixel) -> ()>;
    #[c2rust::src_loc = "34:1"]
    pub type intra_chroma_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "43:5"]
    pub const I_PRED_CHROMA_DC_128: intra_chroma_pred_e = 6;
    #[c2rust::src_loc = "42:5"]
    pub const I_PRED_CHROMA_DC_TOP: intra_chroma_pred_e = 5;
    #[c2rust::src_loc = "41:5"]
    pub const I_PRED_CHROMA_DC_LEFT: intra_chroma_pred_e = 4;
    #[c2rust::src_loc = "39:5"]
    pub const I_PRED_CHROMA_P: intra_chroma_pred_e = 3;
    #[c2rust::src_loc = "38:5"]
    pub const I_PRED_CHROMA_V: intra_chroma_pred_e = 2;
    #[c2rust::src_loc = "37:5"]
    pub const I_PRED_CHROMA_H: intra_chroma_pred_e = 1;
    #[c2rust::src_loc = "36:5"]
    pub const I_PRED_CHROMA_DC: intra_chroma_pred_e = 0;
    #[c2rust::src_loc = "51:1"]
    pub type intra16x16_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "60:5"]
    pub const I_PRED_16x16_DC_128: intra16x16_pred_e = 6;
    #[c2rust::src_loc = "59:5"]
    pub const I_PRED_16x16_DC_TOP: intra16x16_pred_e = 5;
    #[c2rust::src_loc = "58:5"]
    pub const I_PRED_16x16_DC_LEFT: intra16x16_pred_e = 4;
    #[c2rust::src_loc = "56:5"]
    pub const I_PRED_16x16_P: intra16x16_pred_e = 3;
    #[c2rust::src_loc = "55:5"]
    pub const I_PRED_16x16_DC: intra16x16_pred_e = 2;
    #[c2rust::src_loc = "54:5"]
    pub const I_PRED_16x16_H: intra16x16_pred_e = 1;
    #[c2rust::src_loc = "53:5"]
    pub const I_PRED_16x16_V: intra16x16_pred_e = 0;
    #[c2rust::src_loc = "68:1"]
    pub type intra4x4_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "82:5"]
    pub const I_PRED_4x4_DC_128: intra4x4_pred_e = 11;
    #[c2rust::src_loc = "81:5"]
    pub const I_PRED_4x4_DC_TOP: intra4x4_pred_e = 10;
    #[c2rust::src_loc = "80:5"]
    pub const I_PRED_4x4_DC_LEFT: intra4x4_pred_e = 9;
    #[c2rust::src_loc = "78:5"]
    pub const I_PRED_4x4_HU: intra4x4_pred_e = 8;
    #[c2rust::src_loc = "77:5"]
    pub const I_PRED_4x4_VL: intra4x4_pred_e = 7;
    #[c2rust::src_loc = "76:5"]
    pub const I_PRED_4x4_HD: intra4x4_pred_e = 6;
    #[c2rust::src_loc = "75:5"]
    pub const I_PRED_4x4_VR: intra4x4_pred_e = 5;
    #[c2rust::src_loc = "74:5"]
    pub const I_PRED_4x4_DDR: intra4x4_pred_e = 4;
    #[c2rust::src_loc = "73:5"]
    pub const I_PRED_4x4_DDL: intra4x4_pred_e = 3;
    #[c2rust::src_loc = "72:5"]
    pub const I_PRED_4x4_DC: intra4x4_pred_e = 2;
    #[c2rust::src_loc = "71:5"]
    pub const I_PRED_4x4_H: intra4x4_pred_e = 1;
    #[c2rust::src_loc = "70:5"]
    pub const I_PRED_4x4_V: intra4x4_pred_e = 0;
    #[c2rust::src_loc = "45:22"]
    pub static mut x264_mb_chroma_pred_mode_fix: [uint8_t; 7] = [
        I_PRED_CHROMA_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_H as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_V as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_P as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_CHROMA_DC as ::core::ffi::c_int as uint8_t,
    ];
    #[c2rust::src_loc = "62:22"]
    pub static mut x264_mb_pred_mode16x16_fix: [uint8_t; 7] = [
        I_PRED_16x16_V as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_H as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_P as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_DC as ::core::ffi::c_int as uint8_t,
        I_PRED_16x16_DC as ::core::ffi::c_int as uint8_t,
    ];
    #[c2rust::src_loc = "84:21"]
    pub static mut x264_mb_pred_mode4x4_fix: [int8_t; 13] = [
        -(1 as ::core::ffi::c_int) as int8_t,
        I_PRED_4x4_V as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_H as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DC as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DDL as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DDR as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_VR as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_HD as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_VL as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_HU as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DC as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DC as ::core::ffi::c_int as int8_t,
        I_PRED_4x4_DC as ::core::ffi::c_int as int8_t,
    ];
    use super::common_h::pixel;
    use super::stdint_intn_h::int8_t;
    use super::stdint_uintn_h::uint8_t;
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
    use super::stdint_uintn_h::{uint32_t, uint8_t};
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
    #[c2rust::src_loc = "111:1"]
    pub type slice_type_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "115:5"]
    pub const SLICE_TYPE_I: slice_type_e = 2;
    #[c2rust::src_loc = "114:5"]
    pub const SLICE_TYPE_B: slice_type_e = 1;
    #[c2rust::src_loc = "113:5"]
    pub const SLICE_TYPE_P: slice_type_e = 0;
    #[c2rust::src_loc = "155:9"]
    pub const X264_SCAN8_0: ::core::ffi::c_int =
        4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
    #[c2rust::src_loc = "177:9"]
    pub const LUMA_DC: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
    #[c2rust::src_loc = "180:22"]
    pub static mut x264_scan8: [uint8_t; 51] = [
        (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (4 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (4 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (4 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (4 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (4 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (5 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (6 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (7 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
        (0 as ::core::ffi::c_int + 10 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as uint8_t,
    ];
    #[inline(always)]
    #[c2rust::src_loc = "259:1"]
    pub unsafe extern "C" fn x264_cabac_mvd_sum(
        mut mvdleft: *mut uint8_t,
        mut mvdtop: *mut uint8_t,
    ) -> uint16_t {
        let mut amvd0: ::core::ffi::c_int = *mvdleft.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *mvdtop.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        let mut amvd1: ::core::ffi::c_int = *mvdleft.offset(1 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            + *mvdtop.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        amvd0 = (amvd0 > 2 as ::core::ffi::c_int) as ::core::ffi::c_int
            + (amvd0 > 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
        amvd1 = (amvd1 > 2 as ::core::ffi::c_int) as ::core::ffi::c_int
            + (amvd1 > 32 as ::core::ffi::c_int) as ::core::ffi::c_int;
        return (amvd0 + (amvd1 << 8 as ::core::ffi::c_int)) as uint16_t;
    }
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/macroblock.h:28"]
pub mod macroblock_h {
    #[c2rust::src_loc = "31:1"]
    pub type macroblock_position_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "40:5"]
    pub const ALL_NEIGHBORS: macroblock_position_e = 15;
    #[c2rust::src_loc = "38:5"]
    pub const MB_PRIVATE: macroblock_position_e = 16;
    #[c2rust::src_loc = "36:5"]
    pub const MB_TOPLEFT: macroblock_position_e = 8;
    #[c2rust::src_loc = "35:5"]
    pub const MB_TOPRIGHT: macroblock_position_e = 4;
    #[c2rust::src_loc = "34:5"]
    pub const MB_TOP: macroblock_position_e = 2;
    #[c2rust::src_loc = "33:5"]
    pub const MB_LEFT: macroblock_position_e = 1;
    #[c2rust::src_loc = "64:1"]
    pub type mb_class_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "88:5"]
    pub const X264_MBTYPE_MAX: mb_class_e = 19;
    #[c2rust::src_loc = "86:5"]
    pub const B_SKIP: mb_class_e = 18;
    #[c2rust::src_loc = "85:5"]
    pub const B_8x8: mb_class_e = 17;
    #[c2rust::src_loc = "84:5"]
    pub const B_BI_BI: mb_class_e = 16;
    #[c2rust::src_loc = "83:5"]
    pub const B_BI_L1: mb_class_e = 15;
    #[c2rust::src_loc = "82:5"]
    pub const B_BI_L0: mb_class_e = 14;
    #[c2rust::src_loc = "81:5"]
    pub const B_L1_BI: mb_class_e = 13;
    #[c2rust::src_loc = "80:5"]
    pub const B_L1_L1: mb_class_e = 12;
    #[c2rust::src_loc = "79:5"]
    pub const B_L1_L0: mb_class_e = 11;
    #[c2rust::src_loc = "78:5"]
    pub const B_L0_BI: mb_class_e = 10;
    #[c2rust::src_loc = "77:5"]
    pub const B_L0_L1: mb_class_e = 9;
    #[c2rust::src_loc = "76:5"]
    pub const B_L0_L0: mb_class_e = 8;
    #[c2rust::src_loc = "75:5"]
    pub const B_DIRECT: mb_class_e = 7;
    #[c2rust::src_loc = "73:5"]
    pub const P_SKIP: mb_class_e = 6;
    #[c2rust::src_loc = "72:5"]
    pub const P_8x8: mb_class_e = 5;
    #[c2rust::src_loc = "71:5"]
    pub const P_L0: mb_class_e = 4;
    #[c2rust::src_loc = "69:5"]
    pub const I_PCM: mb_class_e = 3;
    #[c2rust::src_loc = "68:5"]
    pub const I_16x16: mb_class_e = 2;
    #[c2rust::src_loc = "67:5"]
    pub const I_8x8: mb_class_e = 1;
    #[c2rust::src_loc = "66:5"]
    pub const I_4x4: mb_class_e = 0;
    #[c2rust::src_loc = "115:1"]
    pub type mb_partition_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "140:5"]
    pub const X264_PARTTYPE_MAX: mb_partition_e = 17;
    #[c2rust::src_loc = "139:5"]
    pub const D_16x16: mb_partition_e = 16;
    #[c2rust::src_loc = "138:5"]
    pub const D_8x16: mb_partition_e = 15;
    #[c2rust::src_loc = "137:5"]
    pub const D_16x8: mb_partition_e = 14;
    #[c2rust::src_loc = "136:5"]
    pub const D_8x8: mb_partition_e = 13;
    #[c2rust::src_loc = "133:5"]
    pub const D_DIRECT_8x8: mb_partition_e = 12;
    #[c2rust::src_loc = "132:5"]
    pub const D_BI_8x8: mb_partition_e = 11;
    #[c2rust::src_loc = "131:5"]
    pub const D_BI_4x8: mb_partition_e = 10;
    #[c2rust::src_loc = "130:5"]
    pub const D_BI_8x4: mb_partition_e = 9;
    #[c2rust::src_loc = "129:5"]
    pub const D_BI_4x4: mb_partition_e = 8;
    #[c2rust::src_loc = "127:5"]
    pub const D_L1_8x8: mb_partition_e = 7;
    #[c2rust::src_loc = "126:5"]
    pub const D_L1_4x8: mb_partition_e = 6;
    #[c2rust::src_loc = "125:5"]
    pub const D_L1_8x4: mb_partition_e = 5;
    #[c2rust::src_loc = "124:5"]
    pub const D_L1_4x4: mb_partition_e = 4;
    #[c2rust::src_loc = "121:5"]
    pub const D_L0_8x8: mb_partition_e = 3;
    #[c2rust::src_loc = "120:5"]
    pub const D_L0_4x8: mb_partition_e = 2;
    #[c2rust::src_loc = "119:5"]
    pub const D_L0_8x4: mb_partition_e = 1;
    #[c2rust::src_loc = "118:5"]
    pub const D_L0_4x4: mb_partition_e = 0;
    #[c2rust::src_loc = "273:1"]
    pub type cabac_ctx_block_cat_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "288:5"]
    pub const DCT_CHROMAV_8x8: cabac_ctx_block_cat_e = 13;
    #[c2rust::src_loc = "287:5"]
    pub const DCT_CHROMAV_4x4: cabac_ctx_block_cat_e = 12;
    #[c2rust::src_loc = "286:5"]
    pub const DCT_CHROMAV_AC: cabac_ctx_block_cat_e = 11;
    #[c2rust::src_loc = "285:5"]
    pub const DCT_CHROMAV_DC: cabac_ctx_block_cat_e = 10;
    #[c2rust::src_loc = "284:5"]
    pub const DCT_CHROMAU_8x8: cabac_ctx_block_cat_e = 9;
    #[c2rust::src_loc = "283:5"]
    pub const DCT_CHROMAU_4x4: cabac_ctx_block_cat_e = 8;
    #[c2rust::src_loc = "282:5"]
    pub const DCT_CHROMAU_AC: cabac_ctx_block_cat_e = 7;
    #[c2rust::src_loc = "281:5"]
    pub const DCT_CHROMAU_DC: cabac_ctx_block_cat_e = 6;
    #[c2rust::src_loc = "280:5"]
    pub const DCT_LUMA_8x8: cabac_ctx_block_cat_e = 5;
    #[c2rust::src_loc = "279:5"]
    pub const DCT_CHROMA_AC: cabac_ctx_block_cat_e = 4;
    #[c2rust::src_loc = "278:5"]
    pub const DCT_CHROMA_DC: cabac_ctx_block_cat_e = 3;
    #[c2rust::src_loc = "277:5"]
    pub const DCT_LUMA_4x4: cabac_ctx_block_cat_e = 2;
    #[c2rust::src_loc = "276:5"]
    pub const DCT_LUMA_AC: cabac_ctx_block_cat_e = 1;
    #[c2rust::src_loc = "275:5"]
    pub const DCT_LUMA_DC: cabac_ctx_block_cat_e = 0;
    #[c2rust::src_loc = "97:22"]
    pub static mut x264_mb_type_list_table: [[[uint8_t; 2]; 2]; 19] = [
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
            [
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
        [
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
            [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        ],
    ];
    #[c2rust::src_loc = "143:22"]
    pub static mut x264_mb_partition_listX_table: [[uint8_t; 17]; 2] = [
        [
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    #[c2rust::src_loc = "202:22"]
    pub static mut block_idx_x: [uint8_t; 16] = [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
    ];
    #[c2rust::src_loc = "206:22"]
    pub static mut block_idx_y: [uint8_t; 16] = [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
    ];
    #[c2rust::src_loc = "291:22"]
    pub static mut ctx_cat_plane: [[uint8_t; 3]; 6] = [
        [
            DCT_LUMA_DC as ::core::ffi::c_int as uint8_t,
            DCT_CHROMAU_DC as ::core::ffi::c_int as uint8_t,
            DCT_CHROMAV_DC as ::core::ffi::c_int as uint8_t,
        ],
        [
            DCT_LUMA_AC as ::core::ffi::c_int as uint8_t,
            DCT_CHROMAU_AC as ::core::ffi::c_int as uint8_t,
            DCT_CHROMAV_AC as ::core::ffi::c_int as uint8_t,
        ],
        [
            DCT_LUMA_4x4 as ::core::ffi::c_int as uint8_t,
            DCT_CHROMAU_4x4 as ::core::ffi::c_int as uint8_t,
            DCT_CHROMAV_4x4 as ::core::ffi::c_int as uint8_t,
        ],
        [0 as ::core::ffi::c_int as uint8_t, 0, 0],
        [0 as ::core::ffi::c_int as uint8_t, 0, 0],
        [
            DCT_LUMA_8x8 as ::core::ffi::c_int as uint8_t,
            DCT_CHROMAU_8x8 as ::core::ffi::c_int as uint8_t,
            DCT_CHROMAV_8x8 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    #[inline(always)]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn pack8to16(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
        return a.wrapping_add(b << 8 as ::core::ffi::c_int);
    }
    #[inline(always)]
    #[c2rust::src_loc = "420:1"]
    pub unsafe extern "C" fn x264_mb_predict_intra4x4_mode(
        mut h: *mut x264_t,
        mut idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let ma: ::core::ffi::c_int = (*h).mb.cache.intra4x4_pred_mode
            [(x264_scan8[idx as usize] as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        let mb: ::core::ffi::c_int = (*h).mb.cache.intra4x4_pred_mode
            [(x264_scan8[idx as usize] as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        let m: ::core::ffi::c_int = if (x264_mb_pred_mode4x4_fix
            [(ma + 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int)
            < x264_mb_pred_mode4x4_fix[(mb + 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
        {
            x264_mb_pred_mode4x4_fix[(ma + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
        } else {
            x264_mb_pred_mode4x4_fix[(mb + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
        };
        if m < 0 as ::core::ffi::c_int {
            return I_PRED_4x4_DC as ::core::ffi::c_int;
        }
        return m;
    }
    #[c2rust::src_loc = "445:22"]
    pub static mut x264_transform_allowed: [uint8_t; 19] = [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ];
    #[inline(always)]
    #[c2rust::src_loc = "454:1"]
    pub unsafe extern "C" fn x264_mb_transform_8x8_allowed(
        mut h: *mut x264_t,
    ) -> ::core::ffi::c_int {
        if (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode == 0 {
            return 0 as ::core::ffi::c_int;
        }
        if (*h).mb.i_type != P_8x8 as ::core::ffi::c_int {
            return x264_transform_allowed[(*h).mb.i_type as usize] as ::core::ffi::c_int;
        }
        return ((*((*h).mb.i_sub_partition.as_mut_ptr() as *mut x264_union32_t)).i
            == (D_L0_8x8 as ::core::ffi::c_int * 0x1010101 as ::core::ffi::c_int) as uint32_t)
            as ::core::ffi::c_int;
    }
    use super::base_h::{x264_scan8, x264_union32_t};
    use super::common_h::x264_t;
    use super::predict_h::{x264_mb_pred_mode4x4_fix, I_PRED_4x4_DC};
    use super::stdint_intn_h::{int16_t, int8_t};
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "350:1"]
        pub fn x264_10_mb_predict_mv(
            h: *mut x264_t,
            i_list: ::core::ffi::c_int,
            idx: ::core::ffi::c_int,
            i_width: ::core::ffi::c_int,
            mvp: *mut int16_t,
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:28"]
pub mod osdep_h {
    #[c2rust::src_loc = "452:9"]
    pub const WORD_SIZE: uint64_t = ::core::mem::size_of::<*mut ::core::ffi::c_void>() as uint64_t;
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
            .wrapping_add((endian_fix32(x as uint32_t) as uint64_t) << 32 as ::core::ffi::c_int);
    }
    #[inline(always)]
    #[c2rust::src_loc = "492:1"]
    pub unsafe extern "C" fn endian_fix(mut x: uintptr_t) -> uintptr_t {
        return if WORD_SIZE == 8 as uint64_t {
            endian_fix64(x as uint64_t) as uintptr_t
        } else {
            endian_fix32(x as uint32_t) as uintptr_t
        };
    }
    #[inline(always)]
    #[c2rust::src_loc = "503:1"]
    pub unsafe extern "C" fn x264_ctz_4bit(mut x: uint32_t) -> ::core::ffi::c_int {
        pub static mut lut: [uint8_t; 16] = [
            4 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ];
        return lut[x as usize] as ::core::ffi::c_int;
    }
    use super::stdint_h::uintptr_t;
    use super::stdint_uintn_h::{uint32_t, uint64_t, uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/tables.h:28"]
pub mod tables_h {
    use super::stdint_uintn_h::{uint16_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "85:23"]
        pub static x264_significant_coeff_flag_offset_8x8: [[uint8_t; 64]; 2];
        #[c2rust::src_loc = "86:23"]
        pub static x264_last_coeff_flag_offset_8x8: [uint8_t; 63];
        #[c2rust::src_loc = "87:23"]
        pub static x264_coeff_flag_offset_chroma_422_dc: [uint8_t; 7];
        #[c2rust::src_loc = "88:23"]
        pub static x264_significant_coeff_flag_offset: [[uint16_t; 16]; 2];
        #[c2rust::src_loc = "89:23"]
        pub static x264_last_coeff_flag_offset: [[uint16_t; 16]; 2];
        #[c2rust::src_loc = "90:23"]
        pub static x264_coeff_abs_level_m1_offset: [uint16_t; 16];
        #[c2rust::src_loc = "91:23"]
        pub static x264_count_cat_m1: [uint8_t; 14];
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/rectangle.h:28"]
pub mod rectangle_h {
    #[inline(always)]
    #[c2rust::src_loc = "28:1"]
    pub unsafe extern "C" fn x264_macroblock_cache_rect(
        mut dst: *mut ::core::ffi::c_void,
        mut w: ::core::ffi::c_int,
        mut h: ::core::ffi::c_int,
        mut s: ::core::ffi::c_int,
        mut v: uint32_t,
    ) {
        let mut d: *mut uint8_t = dst as *mut uint8_t;
        let mut v2: uint16_t = (if s >= 2 as ::core::ffi::c_int {
            v
        } else {
            v.wrapping_mul(0x101 as uint32_t)
        }) as uint16_t;
        let mut v4: uint32_t = if s >= 4 as ::core::ffi::c_int {
            v
        } else if s >= 2 as ::core::ffi::c_int {
            v.wrapping_mul(0x10001 as uint32_t)
        } else {
            v.wrapping_mul(0x1010101 as uint32_t)
        };
        let mut v8: uint64_t =
            (v4 as uint64_t).wrapping_add((v4 as uint64_t) << 32 as ::core::ffi::c_int);
        s *= 8 as ::core::ffi::c_int;
        if w == 2 as ::core::ffi::c_int {
            (*(d.offset((s * 0 as ::core::ffi::c_int) as isize) as *mut x264_union16_t)).i = v2;
            if h == 1 as ::core::ffi::c_int {
                return;
            }
            (*(d.offset((s * 1 as ::core::ffi::c_int) as isize) as *mut x264_union16_t)).i = v2;
            if h == 2 as ::core::ffi::c_int {
                return;
            }
            (*(d.offset((s * 2 as ::core::ffi::c_int) as isize) as *mut x264_union16_t)).i = v2;
            (*(d.offset((s * 3 as ::core::ffi::c_int) as isize) as *mut x264_union16_t)).i = v2;
        } else if w == 4 as ::core::ffi::c_int {
            (*(d.offset((s * 0 as ::core::ffi::c_int) as isize) as *mut x264_union32_t)).i = v4;
            if h == 1 as ::core::ffi::c_int {
                return;
            }
            (*(d.offset((s * 1 as ::core::ffi::c_int) as isize) as *mut x264_union32_t)).i = v4;
            if h == 2 as ::core::ffi::c_int {
                return;
            }
            (*(d.offset((s * 2 as ::core::ffi::c_int) as isize) as *mut x264_union32_t)).i = v4;
            (*(d.offset((s * 3 as ::core::ffi::c_int) as isize) as *mut x264_union32_t)).i = v4;
        } else if w == 8 as ::core::ffi::c_int {
            if WORD_SIZE == 8 as uint64_t {
                (*(d.offset((s * 0 as ::core::ffi::c_int) as isize) as *mut x264_union64_t)).i = v8;
                if h == 1 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 1 as ::core::ffi::c_int) as isize) as *mut x264_union64_t)).i = v8;
                if h == 2 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 2 as ::core::ffi::c_int) as isize) as *mut x264_union64_t)).i = v8;
                (*(d.offset((s * 3 as ::core::ffi::c_int) as isize) as *mut x264_union64_t)).i = v8;
            } else {
                (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                if h == 1 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                if h == 2 as ::core::ffi::c_int {
                    return;
                }
                (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 2 as ::core::ffi::c_int) as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
                (*(d.offset((s * 3 as ::core::ffi::c_int) as isize)
                    .offset(4 as ::core::ffi::c_int as isize)
                    as *mut x264_union32_t))
                    .i = v4;
            }
        } else if w == 16 as ::core::ffi::c_int {
            if h != 1 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"h != 1\0" as *const u8 as *const ::core::ffi::c_char,
                    b"./common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                    82 as ::core::ffi::c_uint,
                    ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
                        *b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
                    )
                    .as_ptr(),
                );
            }
            'c_27249: {
                if h != 1 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"h != 1\0" as *const u8 as *const ::core::ffi::c_char,
                        b"./common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                        82 as ::core::ffi::c_uint,
                        ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
                            *b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
                        )
                        .as_ptr(),
                    );
                }
            };
            if WORD_SIZE == 8 as uint64_t {
                loop {
                    (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut x264_union64_t))
                        .i = v8;
                    (*(d.offset((s * 0 as ::core::ffi::c_int) as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        as *mut x264_union64_t))
                        .i = v8;
                    (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *mut x264_union64_t))
                        .i = v8;
                    (*(d.offset((s * 1 as ::core::ffi::c_int) as isize)
                        .offset(8 as ::core::ffi::c_int as isize)
                        as *mut x264_union64_t))
                        .i = v8;
                    h -= 2 as ::core::ffi::c_int;
                    d = d.offset((s * 2 as ::core::ffi::c_int) as isize);
                    if !(h != 0) {
                        break;
                    }
                }
            } else {
                loop {
                    (*(d.offset(0 as ::core::ffi::c_int as isize) as *mut x264_union32_t)).i = v4;
                    (*(d.offset(4 as ::core::ffi::c_int as isize) as *mut x264_union32_t)).i = v4;
                    (*(d.offset(8 as ::core::ffi::c_int as isize) as *mut x264_union32_t)).i = v4;
                    (*(d.offset(12 as ::core::ffi::c_int as isize) as *mut x264_union32_t)).i = v4;
                    d = d.offset(s as isize);
                    h -= 1;
                    if !(h != 0) {
                        break;
                    }
                }
            }
        } else {
            __assert_fail(
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
                b"./common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                108 as ::core::ffi::c_uint,
                ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
                    *b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
                )
                .as_ptr(),
            );
            'c_27015: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"./common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                    108 as ::core::ffi::c_uint,
                    ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
                        *b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
                    )
                    .as_ptr(),
                );
            };
        };
    }
    #[inline(always)]
    #[c2rust::src_loc = "127:1"]
    pub unsafe extern "C" fn x264_macroblock_cache_mvd(
        mut h: *mut x264_t,
        mut x: ::core::ffi::c_int,
        mut y: ::core::ffi::c_int,
        mut width: ::core::ffi::c_int,
        mut height: ::core::ffi::c_int,
        mut i_list: ::core::ffi::c_int,
        mut mvd: uint16_t,
    ) {
        let mut mvd_cache: *mut ::core::ffi::c_void =
            &mut *(*(*h).mb.cache.mvd.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((X264_SCAN8_0 + x + 8 as ::core::ffi::c_int * y) as isize)
                as *mut [uint8_t; 2] as *mut ::core::ffi::c_void;
        if 0 == 0 || 0 == 0 {
            x264_10_cache_mvd_func_table
                [(width + (height << 1 as ::core::ffi::c_int) - 3 as ::core::ffi::c_int) as usize]
                .expect("non-null function pointer")(mvd_cache, mvd as uint32_t);
        } else {
            x264_macroblock_cache_rect(
                mvd_cache,
                width * 2 as ::core::ffi::c_int,
                height,
                2 as ::core::ffi::c_int,
                mvd as uint32_t,
            );
        };
    }
    use super::assert_h::__assert_fail;
    use super::base_h::{x264_union16_t, x264_union32_t, x264_union64_t, X264_SCAN8_0};
    use super::common_h::x264_t;
    use super::osdep_h::WORD_SIZE;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "114:15"]
        pub static mut x264_10_cache_mvd_func_table:
            [Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> ()>; 10];
    }
}
#[c2rust::header_src = "/usr/include/stdlib.h:28"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "980:1"]
        pub fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
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
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::atomic_wide_counter_h::{C2RustUnnamed, __atomic_wide_counter};
pub use self::base_h::{
    chroma_format_e, slice_type_e, x264_cabac_mvd_sum, x264_scan8, x264_union16_t, x264_union32_t,
    x264_union64_t, CHROMA_400, CHROMA_420, CHROMA_422, CHROMA_444, LUMA_DC, SLICE_TYPE_B,
    SLICE_TYPE_I, SLICE_TYPE_P, X264_SCAN8_0,
};
pub use self::bitstream_h::{
    bs_flush, bs_init, bs_s, bs_t, bs_write, x264_bitstream_function_t, x264_run_level_t,
};
pub use self::cabac_h::{
    x264_10_cabac_encode_bypass_c, x264_10_cabac_encode_decision_c, x264_10_cabac_encode_flush,
    x264_10_cabac_encode_init_core, x264_10_cabac_encode_terminal_c,
    x264_10_cabac_encode_ue_bypass, x264_cabac_pos, x264_cabac_t,
};
pub use self::common_h::{
    dctcoef, pixel, udctcoef, x264_frame_stat_t, x264_left_table_t, x264_lookahead_t,
    x264_ratecontrol_t, x264_slice_header_t, x264_t, C2RustUnnamed_10, C2RustUnnamed_11,
    C2RustUnnamed_12, C2RustUnnamed_13, C2RustUnnamed_17, C2RustUnnamed_18, C2RustUnnamed_6,
    C2RustUnnamed_7, C2RustUnnamed_8, C2RustUnnamed_9, FENC_STRIDE, QP_BD_OFFSET, QP_MAX_SPEC,
};
pub use self::dct_h::{x264_dct_function_t, x264_zigzag_function_t};
pub use self::frame_h::{
    x264_deblock_function_t, x264_deblock_inter_t, x264_deblock_intra_t, x264_frame, x264_frame_t,
    x264_sync_frame_list_t,
};
pub use self::internal::{__va_list_tag, BIT_DEPTH};
pub use self::macroblock_h::{
    block_idx_x, block_idx_y, cabac_ctx_block_cat_e, ctx_cat_plane, macroblock_position_e,
    mb_class_e, mb_partition_e, pack8to16, x264_10_mb_predict_mv, x264_mb_partition_listX_table,
    x264_mb_predict_intra4x4_mode, x264_mb_transform_8x8_allowed, x264_mb_type_list_table,
    x264_transform_allowed, B_8x8, DCT_CHROMAU_4x4, DCT_CHROMAU_8x8, DCT_CHROMAV_4x4,
    DCT_CHROMAV_8x8, DCT_LUMA_4x4, DCT_LUMA_8x8, D_16x16, D_16x8, D_8x16, D_8x8, D_BI_4x4,
    D_BI_4x8, D_BI_8x4, D_BI_8x8, D_DIRECT_8x8, D_L0_4x4, D_L0_4x8, D_L0_8x4, D_L0_8x8, D_L1_4x4,
    D_L1_4x8, D_L1_8x4, D_L1_8x8, I_16x16, I_4x4, I_8x8, P_8x8, ALL_NEIGHBORS, B_BI_BI, B_BI_L0,
    B_BI_L1, B_DIRECT, B_L0_BI, B_L0_L0, B_L0_L1, B_L1_BI, B_L1_L0, B_L1_L1, B_SKIP,
    DCT_CHROMAU_AC, DCT_CHROMAU_DC, DCT_CHROMAV_AC, DCT_CHROMAV_DC, DCT_CHROMA_AC, DCT_CHROMA_DC,
    DCT_LUMA_AC, DCT_LUMA_DC, I_PCM, MB_LEFT, MB_PRIVATE, MB_TOP, MB_TOPLEFT, MB_TOPRIGHT, P_L0,
    P_SKIP, X264_MBTYPE_MAX, X264_PARTTYPE_MAX,
};
pub use self::mc_h::{weight_fn_t, x264_mc_functions_t, x264_weight_t};
pub use self::osdep_h::{endian_fix, endian_fix32, endian_fix64, x264_ctz_4bit, WORD_SIZE};
pub use self::pixel_h::{
    x264_pixel_cmp_t, x264_pixel_cmp_x3_t, x264_pixel_cmp_x4_t, x264_pixel_function_t,
};
pub use self::predict_h::{
    intra16x16_pred_e, intra4x4_pred_e, intra_chroma_pred_e, x264_mb_chroma_pred_mode_fix,
    x264_mb_pred_mode16x16_fix, x264_mb_pred_mode4x4_fix, x264_predict8x8_t,
    x264_predict_8x8_filter_t, x264_predict_t, I_PRED_16x16_DC, I_PRED_16x16_DC_128,
    I_PRED_16x16_DC_LEFT, I_PRED_16x16_DC_TOP, I_PRED_16x16_H, I_PRED_16x16_P, I_PRED_16x16_V,
    I_PRED_4x4_DC, I_PRED_4x4_DC_128, I_PRED_4x4_DC_LEFT, I_PRED_4x4_DC_TOP, I_PRED_4x4_DDL,
    I_PRED_4x4_DDR, I_PRED_4x4_H, I_PRED_4x4_HD, I_PRED_4x4_HU, I_PRED_4x4_V, I_PRED_4x4_VL,
    I_PRED_4x4_VR, I_PRED_CHROMA_DC, I_PRED_CHROMA_DC_128, I_PRED_CHROMA_DC_LEFT,
    I_PRED_CHROMA_DC_TOP, I_PRED_CHROMA_H, I_PRED_CHROMA_P, I_PRED_CHROMA_V,
};
pub use self::pthreadtypes_h::{pthread_cond_t, pthread_mutex_t, pthread_t};
pub use self::quant_h::x264_quant_function_t;
pub use self::rectangle_h::{
    x264_10_cache_mvd_func_table, x264_macroblock_cache_mvd, x264_macroblock_cache_rect,
};
pub use self::set_h::{
    x264_pps_t, x264_sps_t, C2RustUnnamed_14, C2RustUnnamed_15, C2RustUnnamed_16,
};
pub use self::stdint_h::{intptr_t, uintptr_t};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use self::stdlib_h::abs;
pub use self::struct_mutex_h::__pthread_mutex_s;
use self::tables_h::{
    x264_coeff_abs_level_m1_offset, x264_coeff_flag_offset_chroma_422_dc, x264_count_cat_m1,
    x264_last_coeff_flag_offset, x264_last_coeff_flag_offset_8x8,
    x264_significant_coeff_flag_offset, x264_significant_coeff_flag_offset_8x8,
};
pub use self::thread_shared_types_h::{
    __pthread_cond_s, __pthread_internal_list, __pthread_list_t,
};
use self::threadpool_h::x264_threadpool_t;
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t,
};
pub use self::x264_h::{
    x264_hrd_t, x264_nal_t, x264_param_t, x264_sei_payload_t, x264_sei_t, x264_zone_t,
    C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4,
    C2RustUnnamed_5,
};
#[inline]
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn cabac_mb_type_intra(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: ::core::ffi::c_int,
    mut ctx0: ::core::ffi::c_int,
    mut ctx1: ::core::ffi::c_int,
    mut ctx2: ::core::ffi::c_int,
    mut ctx3: ::core::ffi::c_int,
    mut ctx4: ::core::ffi::c_int,
    mut ctx5: ::core::ffi::c_int,
) {
    if i_mb_type == I_4x4 as ::core::ffi::c_int || i_mb_type == I_8x8 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(cb, ctx0, 0 as ::core::ffi::c_int);
    } else if i_mb_type == I_PCM as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(cb, ctx0, 1 as ::core::ffi::c_int);
        x264_10_cabac_encode_flush(h, cb);
    } else {
        let mut i_pred: ::core::ffi::c_int = x264_mb_pred_mode16x16_fix
            [(*h).mb.i_intra16x16_pred_mode as usize]
            as ::core::ffi::c_int;
        x264_10_cabac_encode_decision_c(cb, ctx0, 1 as ::core::ffi::c_int);
        x264_10_cabac_encode_terminal_c(cb);
        x264_10_cabac_encode_decision_c(cb, ctx1, ((*h).mb.i_cbp_luma != 0) as ::core::ffi::c_int);
        if (*h).mb.i_cbp_chroma == 0 as ::core::ffi::c_int {
            x264_10_cabac_encode_decision_c(cb, ctx2, 0 as ::core::ffi::c_int);
        } else {
            x264_10_cabac_encode_decision_c(cb, ctx2, 1 as ::core::ffi::c_int);
            x264_10_cabac_encode_decision_c(
                cb,
                ctx3,
                (*h).mb.i_cbp_chroma >> 1 as ::core::ffi::c_int,
            );
        }
        x264_10_cabac_encode_decision_c(cb, ctx4, i_pred >> 1 as ::core::ffi::c_int);
        x264_10_cabac_encode_decision_c(cb, ctx5, i_pred & 1 as ::core::ffi::c_int);
    };
}
#[c2rust::src_loc = "70:1"]
unsafe extern "C" fn cabac_field_decoding_flag(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ctx += (*h).mb.field_decoding_flag & ((*h).mb.i_mb_x != 0) as ::core::ffi::c_int;
    ctx += ((*h).mb.i_mb_top_mbpair_xy >= 0 as ::core::ffi::c_int
        && *(*h)
            .mb
            .slice_table
            .offset((*h).mb.i_mb_top_mbpair_xy as isize)
            == (*h).sh.i_first_mb as int32_t
        && *(*h).mb.field.offset((*h).mb.i_mb_top_mbpair_xy as isize) as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
    x264_10_cabac_encode_decision_c(cb, 70 as ::core::ffi::c_int + ctx, (*h).mb.b_interlaced);
    (*h).mb.field_decoding_flag = (*h).mb.b_interlaced;
}
#[c2rust::src_loc = "83:1"]
unsafe extern "C" fn cabac_intra4x4_pred_mode(
    mut cb: *mut x264_cabac_t,
    mut i_pred: ::core::ffi::c_int,
    mut i_mode: ::core::ffi::c_int,
) {
    if i_pred == i_mode {
        x264_10_cabac_encode_decision_c(cb, 68 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
    } else {
        x264_10_cabac_encode_decision_c(cb, 68 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        if i_mode > i_pred {
            i_mode -= 1;
        }
        x264_10_cabac_encode_decision_c(
            cb,
            69 as ::core::ffi::c_int,
            i_mode & 0x1 as ::core::ffi::c_int,
        );
        x264_10_cabac_encode_decision_c(
            cb,
            69 as ::core::ffi::c_int,
            i_mode >> 1 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int,
        );
        x264_10_cabac_encode_decision_c(
            cb,
            69 as ::core::ffi::c_int,
            i_mode >> 2 as ::core::ffi::c_int,
        );
    };
}
#[c2rust::src_loc = "98:1"]
unsafe extern "C" fn cabac_intra_chroma_pred_mode(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut i_mode: ::core::ffi::c_int =
        x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize] as ::core::ffi::c_int;
    let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        && *(*h)
            .mb
            .chroma_pred_mode
            .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
            as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        ctx += 1;
    }
    if (*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        && *(*h)
            .mb
            .chroma_pred_mode
            .offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        ctx += 1;
    }
    x264_10_cabac_encode_decision_c(
        cb,
        64 as ::core::ffi::c_int + ctx,
        (i_mode > 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    if i_mode > 0 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(
            cb,
            64 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
            (i_mode > 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
        if i_mode > 1 as ::core::ffi::c_int {
            x264_10_cabac_encode_decision_c(
                cb,
                64 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
                (i_mode > 2 as ::core::ffi::c_int) as ::core::ffi::c_int,
            );
        }
    }
}
#[c2rust::src_loc = "118:1"]
unsafe extern "C" fn cabac_cbp_luma(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut cbp: ::core::ffi::c_int = (*h).mb.i_cbp_luma;
    let mut cbp_l: ::core::ffi::c_int = (*h).mb.cache.i_cbp_left;
    let mut cbp_t: ::core::ffi::c_int = (*h).mb.cache.i_cbp_top;
    x264_10_cabac_encode_decision_c(
        cb,
        76 as ::core::ffi::c_int
            - (cbp_l >> 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int)
            - (cbp_t >> 1 as ::core::ffi::c_int & 2 as ::core::ffi::c_int),
        cbp >> 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
    );
    x264_10_cabac_encode_decision_c(
        cb,
        76 as ::core::ffi::c_int
            - (cbp >> 0 as ::core::ffi::c_int & 1 as ::core::ffi::c_int)
            - (cbp_t >> 2 as ::core::ffi::c_int & 2 as ::core::ffi::c_int),
        cbp >> 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
    );
    x264_10_cabac_encode_decision_c(
        cb,
        76 as ::core::ffi::c_int
            - (cbp_l >> 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int)
            - (cbp << 1 as ::core::ffi::c_int & 2 as ::core::ffi::c_int),
        cbp >> 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
    );
    x264_10_cabac_encode_decision_c(
        cb,
        76 as ::core::ffi::c_int
            - (cbp >> 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int)
            - (cbp >> 0 as ::core::ffi::c_int & 2 as ::core::ffi::c_int),
        cbp >> 3 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
    );
}
#[c2rust::src_loc = "129:1"]
unsafe extern "C" fn cabac_cbp_chroma(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut cbp_a: ::core::ffi::c_int = (*h).mb.cache.i_cbp_left & 0x30 as ::core::ffi::c_int;
    let mut cbp_b: ::core::ffi::c_int = (*h).mb.cache.i_cbp_top & 0x30 as ::core::ffi::c_int;
    let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if cbp_a != 0 && (*h).mb.cache.i_cbp_left != -(1 as ::core::ffi::c_int) {
        ctx += 1;
    }
    if cbp_b != 0 && (*h).mb.cache.i_cbp_top != -(1 as ::core::ffi::c_int) {
        ctx += 2 as ::core::ffi::c_int;
    }
    if (*h).mb.i_cbp_chroma == 0 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(
            cb,
            77 as ::core::ffi::c_int + ctx,
            0 as ::core::ffi::c_int,
        );
    } else {
        x264_10_cabac_encode_decision_c(
            cb,
            77 as ::core::ffi::c_int + ctx,
            1 as ::core::ffi::c_int,
        );
        ctx = 4 as ::core::ffi::c_int;
        if cbp_a == 0x20 as ::core::ffi::c_int {
            ctx += 1;
        }
        if cbp_b == 0x20 as ::core::ffi::c_int {
            ctx += 2 as ::core::ffi::c_int;
        }
        x264_10_cabac_encode_decision_c(
            cb,
            77 as ::core::ffi::c_int + ctx,
            (*h).mb.i_cbp_chroma >> 1 as ::core::ffi::c_int,
        );
    };
}
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn cabac_qp_delta(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut i_dqp: ::core::ffi::c_int = (*h).mb.i_qp - (*h).mb.i_last_qp;
    let mut ctx: ::core::ffi::c_int = 0;
    if (*h).mb.i_type == I_16x16 as ::core::ffi::c_int
        && *(*h).mb.cbp.offset((*h).mb.i_mb_xy as isize) == 0
        && (*h).mb.i_qp > (*h).mb.i_last_qp
    {
        (*h).mb.i_qp = (*h).mb.i_last_qp;
        i_dqp = 0 as ::core::ffi::c_int;
    }
    ctx = ((*h).mb.i_last_dqp != 0
        && (*(*h).mb.type_0.offset((*h).mb.i_mb_prev_xy as isize) as ::core::ffi::c_int
            == I_16x16 as ::core::ffi::c_int
            || *(*h).mb.cbp.offset((*h).mb.i_mb_prev_xy as isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int
                != 0)) as ::core::ffi::c_int;
    if i_dqp != 0 as ::core::ffi::c_int {
        i_dqp *= 2 as ::core::ffi::c_int;
        let mut val: ::core::ffi::c_int = 1 as ::core::ffi::c_int - i_dqp;
        if val < 0 as ::core::ffi::c_int {
            val = i_dqp;
        }
        val -= 1;
        if val >= QP_MAX_SPEC && val != QP_MAX_SPEC + 1 as ::core::ffi::c_int {
            val = 2 as ::core::ffi::c_int * QP_MAX_SPEC + 1 as ::core::ffi::c_int - val;
        }
        loop {
            x264_10_cabac_encode_decision_c(
                cb,
                60 as ::core::ffi::c_int + ctx,
                1 as ::core::ffi::c_int,
            );
            ctx = 2 as ::core::ffi::c_int + (ctx >> 1 as ::core::ffi::c_int);
            val -= 1;
            if !(val != 0) {
                break;
            }
        }
    }
    x264_10_cabac_encode_decision_c(cb, 60 as ::core::ffi::c_int + ctx, 0 as ::core::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "189:1"]
pub unsafe extern "C" fn x264_10_cabac_mb_skip(mut h: *mut x264_t, mut b_skip: ::core::ffi::c_int) {
    let mut ctx: ::core::ffi::c_int = (*h).mb.cache.i_neighbour_skip + 11 as ::core::ffi::c_int;
    if (*h).sh.i_type != SLICE_TYPE_P as ::core::ffi::c_int {
        ctx += 13 as ::core::ffi::c_int;
    }
    x264_10_cabac_encode_decision_c(&mut (*h).cabac, ctx, b_skip);
}
#[inline]
#[c2rust::src_loc = "198:1"]
unsafe extern "C" fn cabac_subpartition_p(
    mut cb: *mut x264_cabac_t,
    mut i_sub: ::core::ffi::c_int,
) {
    if i_sub == D_L0_8x8 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(cb, 21 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
        return;
    }
    x264_10_cabac_encode_decision_c(cb, 21 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    if i_sub == D_L0_8x4 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(cb, 22 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    } else {
        x264_10_cabac_encode_decision_c(cb, 22 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
        x264_10_cabac_encode_decision_c(
            cb,
            23 as ::core::ffi::c_int,
            (i_sub == D_L0_4x8 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
    };
}
#[inline(always)]
#[c2rust::src_loc = "215:1"]
unsafe extern "C" fn cabac_subpartition_b(
    mut cb: *mut x264_cabac_t,
    mut i_sub: ::core::ffi::c_int,
) {
    if i_sub == D_DIRECT_8x8 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(cb, 36 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        return;
    }
    x264_10_cabac_encode_decision_c(cb, 36 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
    if i_sub == D_BI_8x8 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(cb, 37 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
        x264_10_cabac_encode_decision_c(cb, 38 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        x264_10_cabac_encode_decision_c(cb, 39 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        x264_10_cabac_encode_decision_c(cb, 39 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        return;
    }
    x264_10_cabac_encode_decision_c(cb, 37 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    x264_10_cabac_encode_decision_c(
        cb,
        39 as ::core::ffi::c_int,
        (i_sub == D_L1_8x8 as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
}
#[inline(always)]
#[c2rust::src_loc = "235:1"]
unsafe extern "C" fn cabac_transform_size(mut h: *mut x264_t, mut cb: *mut x264_cabac_t) {
    let mut ctx: ::core::ffi::c_int =
        399 as ::core::ffi::c_int + (*h).mb.cache.i_neighbour_transform_size;
    x264_10_cabac_encode_decision_c(cb, ctx, (*h).mb.b_transform_8x8);
}
#[inline(always)]
#[c2rust::src_loc = "241:1"]
unsafe extern "C" fn cabac_ref_internal(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_list: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut bframe: ::core::ffi::c_int,
) {
    let i8: ::core::ffi::c_int = x264_scan8[idx as usize] as ::core::ffi::c_int;
    let i_refa: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(i8 - 1 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let i_refb: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(i8 - 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if i_refa > 0 as ::core::ffi::c_int
        && (bframe == 0 || (*h).mb.cache.skip[(i8 - 1 as ::core::ffi::c_int) as usize] == 0)
    {
        ctx += 1;
    }
    if i_refb > 0 as ::core::ffi::c_int
        && (bframe == 0 || (*h).mb.cache.skip[(i8 - 8 as ::core::ffi::c_int) as usize] == 0)
    {
        ctx += 2 as ::core::ffi::c_int;
    }
    let mut i_ref: ::core::ffi::c_int =
        (*h).mb.cache.ref_0[i_list as usize][i8 as usize] as ::core::ffi::c_int;
    while i_ref > 0 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(
            cb,
            54 as ::core::ffi::c_int + ctx,
            1 as ::core::ffi::c_int,
        );
        ctx = (ctx >> 2 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int;
        i_ref -= 1;
    }
    x264_10_cabac_encode_decision_c(cb, 54 as ::core::ffi::c_int + ctx, 0 as ::core::ffi::c_int);
}
#[inline(never)]
#[c2rust::src_loc = "261:1"]
unsafe extern "C" fn cabac_ref_p(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut idx: ::core::ffi::c_int,
) {
    cabac_ref_internal(h, cb, 0 as ::core::ffi::c_int, idx, 0 as ::core::ffi::c_int);
}
#[inline(never)]
#[c2rust::src_loc = "265:1"]
unsafe extern "C" fn cabac_ref_b(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_list: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
) {
    cabac_ref_internal(h, cb, i_list, idx, 1 as ::core::ffi::c_int);
}
#[inline(always)]
#[c2rust::src_loc = "270:1"]
unsafe extern "C" fn cabac_mvd_cpn(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_list: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut l: ::core::ffi::c_int,
    mut mvd: ::core::ffi::c_int,
    mut ctx: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctxbase: ::core::ffi::c_int = if l != 0 {
        47 as ::core::ffi::c_int
    } else {
        40 as ::core::ffi::c_int
    };
    if mvd == 0 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(cb, ctxbase + ctx, 0 as ::core::ffi::c_int);
        return 0 as ::core::ffi::c_int;
    }
    let mut i_abs: ::core::ffi::c_int = abs(mvd);
    x264_10_cabac_encode_decision_c(cb, ctxbase + ctx, 1 as ::core::ffi::c_int);
    static mut ctxes: [uint8_t; 8] = [
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
    ];
    if i_abs < 9 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i < i_abs {
            x264_10_cabac_encode_decision_c(
                cb,
                ctxbase + ctxes[(i - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            i += 1;
        }
        x264_10_cabac_encode_decision_c(
            cb,
            ctxbase + ctxes[(i_abs - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    } else {
        let mut i_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i_0 < 9 as ::core::ffi::c_int {
            x264_10_cabac_encode_decision_c(
                cb,
                ctxbase + ctxes[(i_0 - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            i_0 += 1;
        }
        x264_10_cabac_encode_ue_bypass(
            cb,
            3 as ::core::ffi::c_int,
            i_abs - 9 as ::core::ffi::c_int,
        );
    }
    x264_10_cabac_encode_bypass_c(cb, mvd >> 31 as ::core::ffi::c_int);
    return if i_abs < 66 as ::core::ffi::c_int {
        i_abs
    } else {
        66 as ::core::ffi::c_int
    };
}
#[inline(never)]
#[c2rust::src_loc = "329:1"]
unsafe extern "C" fn cabac_mvd(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_list: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
) -> uint16_t {
    let mut mvp: [int16_t; 2] = [0; 2];
    let mut mdx: ::core::ffi::c_int = 0;
    let mut mdy: ::core::ffi::c_int = 0;
    x264_10_mb_predict_mv(h, i_list, idx, width, mvp.as_mut_ptr());
    mdx = (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize]
        [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        - mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    mdy = (*h).mb.cache.mv[i_list as usize][x264_scan8[idx as usize] as usize]
        [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        - mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    let mut amvd: uint16_t = x264_cabac_mvd_sum(
        (*(*(*h).mb.cache.mvd.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(idx as isize) as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as isize,
            ))
        .as_mut_ptr(),
        (*(*(*h).mb.cache.mvd.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(idx as isize) as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as isize,
            ))
        .as_mut_ptr(),
    );
    mdx = cabac_mvd_cpn(
        h,
        cb,
        i_list,
        idx,
        0 as ::core::ffi::c_int,
        mdx,
        amvd as ::core::ffi::c_int & 0xff as ::core::ffi::c_int,
    );
    mdy = cabac_mvd_cpn(
        h,
        cb,
        i_list,
        idx,
        1 as ::core::ffi::c_int,
        mdy,
        amvd as ::core::ffi::c_int >> 8 as ::core::ffi::c_int,
    );
    return pack8to16(mdx as uint32_t, mdy as uint32_t) as uint16_t;
}
#[inline]
#[c2rust::src_loc = "355:1"]
unsafe extern "C" fn cabac_8x8_mvd(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i: ::core::ffi::c_int,
) {
    match (*h).mb.i_sub_partition[i as usize] as ::core::ffi::c_int {
        3 => {
            let mut mvd: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int * i,
                2 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as ::core::ffi::c_int * i) as usize] as ::core::ffi::c_int,
                block_idx_y[(4 as ::core::ffi::c_int * i) as usize] as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd,
            );
        }
        1 => {
            let mut mvd_0: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                block_idx_y[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_0,
            );
            let mut mvd_1: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                block_idx_y[(4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_1,
            );
        }
        2 => {
            let mut mvd_2: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                block_idx_y[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_2,
            );
            let mut mvd_3: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                block_idx_y[(4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_3,
            );
        }
        0 => {
            let mut mvd_4: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                block_idx_y[(4 as ::core::ffi::c_int * i + 0 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_4,
            );
            let mut mvd_5: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                block_idx_y[(4 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_5,
            );
            let mut mvd_6: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                block_idx_y[(4 as ::core::ffi::c_int * i + 2 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_6,
            );
            let mut mvd_7: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[(4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                block_idx_y[(4 as ::core::ffi::c_int * i + 3 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_7,
            );
        }
        _ => {
            __assert_fail(
                b"0\0" as *const u8 as *const ::core::ffi::c_char,
                b"encoder/cabac.c\0" as *const u8 as *const ::core::ffi::c_char,
                377 as ::core::ffi::c_uint,
                ::core::mem::transmute::<[u8; 50], [::core::ffi::c_char; 50]>(
                    *b"void cabac_8x8_mvd(x264_t *, x264_cabac_t *, int)\0",
                )
                .as_ptr(),
            );
            'c_35309: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"encoder/cabac.c\0" as *const u8 as *const ::core::ffi::c_char,
                    377 as ::core::ffi::c_uint,
                    ::core::mem::transmute::<[u8; 50], [::core::ffi::c_char; 50]>(
                        *b"void cabac_8x8_mvd(x264_t *, x264_cabac_t *, int)\0",
                    )
                    .as_ptr(),
                );
            };
        }
    };
}
#[inline(always)]
#[c2rust::src_loc = "381:1"]
unsafe extern "C" fn cabac_mb_header_i(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: ::core::ffi::c_int,
    mut slice_type: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    if slice_type == SLICE_TYPE_I as ::core::ffi::c_int {
        let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
            && (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                != I_4x4 as ::core::ffi::c_int
        {
            ctx += 1;
        }
        if (*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
            && (*h).mb.i_mb_type_top != I_4x4 as ::core::ffi::c_int
        {
            ctx += 1;
        }
        cabac_mb_type_intra(
            h,
            cb,
            i_mb_type,
            3 as ::core::ffi::c_int + ctx,
            3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int + 6 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int + 7 as ::core::ffi::c_int,
        );
    } else if slice_type == SLICE_TYPE_P as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(cb, 14 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
        cabac_mb_type_intra(
            h,
            cb,
            i_mb_type,
            17 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            17 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
            17 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
            17 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
            17 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
            17 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
        );
    } else if slice_type == SLICE_TYPE_B as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        cabac_mb_type_intra(
            h,
            cb,
            i_mb_type,
            32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
            32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
            32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int,
            32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
            32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
        );
    }
    if i_mb_type == I_PCM as ::core::ffi::c_int {
        return;
    }
    if i_mb_type != I_16x16 as ::core::ffi::c_int {
        if (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0 {
            cabac_transform_size(h, cb);
        }
        let mut di: ::core::ffi::c_int = if (*h).mb.b_transform_8x8 != 0 {
            4 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            let i_pred: ::core::ffi::c_int =
                x264_mb_predict_intra4x4_mode(h, i) as ::core::ffi::c_int;
            let i_mode: ::core::ffi::c_int =
                x264_mb_pred_mode4x4_fix[((*h).mb.cache.intra4x4_pred_mode
                    [x264_scan8[i as usize] as usize]
                    as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
            cabac_intra4x4_pred_mode(cb, i_pred, i_mode);
            i += di;
        }
    }
    if chroma != 0 {
        cabac_intra_chroma_pred_mode(h, cb);
    }
}
#[inline(always)]
#[c2rust::src_loc = "435:1"]
unsafe extern "C" fn cabac_mb_header_p(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    if i_mb_type == P_L0 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(cb, 14 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        if (*h).mb.i_partition == D_16x16 as ::core::ffi::c_int {
            x264_10_cabac_encode_decision_c(cb, 15 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
            x264_10_cabac_encode_decision_c(cb, 16 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
            if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                cabac_ref_p(h, cb, 0 as ::core::ffi::c_int);
            }
            let mut mvd: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd,
            );
        } else if (*h).mb.i_partition == D_16x8 as ::core::ffi::c_int {
            x264_10_cabac_encode_decision_c(cb, 15 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
            x264_10_cabac_encode_decision_c(cb, 17 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
            if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                cabac_ref_p(h, cb, 0 as ::core::ffi::c_int);
                cabac_ref_p(h, cb, 8 as ::core::ffi::c_int);
            }
            let mut mvd_0: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_0,
            );
            let mut mvd_1: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                8 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                block_idx_y[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_1,
            );
        } else {
            x264_10_cabac_encode_decision_c(cb, 15 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
            x264_10_cabac_encode_decision_c(cb, 17 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
            if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
                cabac_ref_p(h, cb, 0 as ::core::ffi::c_int);
                cabac_ref_p(h, cb, 4 as ::core::ffi::c_int);
            }
            let mut mvd_2: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_2,
            );
            let mut mvd_3: uint16_t = cabac_mvd(
                h,
                cb,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
            );
            x264_macroblock_cache_mvd(
                h,
                block_idx_x[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                block_idx_y[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                mvd_3,
            );
        }
    } else if i_mb_type == P_8x8 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(cb, 14 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        x264_10_cabac_encode_decision_c(cb, 15 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
        x264_10_cabac_encode_decision_c(cb, 16 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            cabac_subpartition_p(
                cb,
                (*h).mb.i_sub_partition[i as usize] as ::core::ffi::c_int,
            );
            i += 1;
        }
        if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
            cabac_ref_p(h, cb, 0 as ::core::ffi::c_int);
            cabac_ref_p(h, cb, 4 as ::core::ffi::c_int);
            cabac_ref_p(h, cb, 8 as ::core::ffi::c_int);
            cabac_ref_p(h, cb, 12 as ::core::ffi::c_int);
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < 4 as ::core::ffi::c_int {
            cabac_8x8_mvd(h, cb, i_0);
            i_0 += 1;
        }
    } else {
        cabac_mb_header_i(h, cb, i_mb_type, SLICE_TYPE_P as ::core::ffi::c_int, chroma);
    };
}
#[inline(always)]
#[c2rust::src_loc = "499:1"]
unsafe extern "C" fn cabac_mb_header_b(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut i_mb_type: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        && (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize] != B_SKIP as ::core::ffi::c_int
        && (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
            != B_DIRECT as ::core::ffi::c_int
    {
        ctx += 1;
    }
    if (*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        && (*h).mb.i_mb_type_top != B_SKIP as ::core::ffi::c_int
        && (*h).mb.i_mb_type_top != B_DIRECT as ::core::ffi::c_int
    {
        ctx += 1;
    }
    if i_mb_type == B_DIRECT as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + ctx,
            0 as ::core::ffi::c_int,
        );
        return;
    }
    x264_10_cabac_encode_decision_c(cb, 27 as ::core::ffi::c_int + ctx, 1 as ::core::ffi::c_int);
    if i_mb_type == B_8x8 as ::core::ffi::c_int {
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            cabac_subpartition_b(
                cb,
                (*h).mb.i_sub_partition[i as usize] as ::core::ffi::c_int,
            );
            i += 1;
        }
        if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 4 as ::core::ffi::c_int {
                if x264_mb_partition_listX_table[0 as ::core::ffi::c_int as usize]
                    [(*h).mb.i_sub_partition[i_0 as usize] as usize]
                    != 0
                {
                    cabac_ref_b(
                        h,
                        cb,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int * i_0,
                    );
                }
                i_0 += 1;
            }
        }
        if (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < 4 as ::core::ffi::c_int {
                if x264_mb_partition_listX_table[1 as ::core::ffi::c_int as usize]
                    [(*h).mb.i_sub_partition[i_1 as usize] as usize]
                    != 0
                {
                    cabac_ref_b(
                        h,
                        cb,
                        1 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int * i_1,
                    );
                }
                i_1 += 1;
            }
        }
        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_2 < 4 as ::core::ffi::c_int {
            if x264_mb_partition_listX_table[0 as ::core::ffi::c_int as usize]
                [(*h).mb.i_sub_partition[i_2 as usize] as usize]
                != 0
            {
                let mut mvd: uint16_t = cabac_mvd(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i_2,
                    2 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as ::core::ffi::c_int * i_2) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4 as ::core::ffi::c_int * i_2) as usize] as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    mvd,
                );
            }
            i_2 += 1;
        }
        let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_3 < 4 as ::core::ffi::c_int {
            if x264_mb_partition_listX_table[1 as ::core::ffi::c_int as usize]
                [(*h).mb.i_sub_partition[i_3 as usize] as usize]
                != 0
            {
                let mut mvd_0: uint16_t = cabac_mvd(
                    h,
                    cb,
                    1 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * i_3,
                    2 as ::core::ffi::c_int,
                );
                x264_macroblock_cache_mvd(
                    h,
                    block_idx_x[(4 as ::core::ffi::c_int * i_3) as usize] as ::core::ffi::c_int,
                    block_idx_y[(4 as ::core::ffi::c_int * i_3) as usize] as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    mvd_0,
                );
            }
            i_3 += 1;
        }
    } else if i_mb_type >= B_L0_L0 as ::core::ffi::c_int
        && i_mb_type <= B_BI_BI as ::core::ffi::c_int
    {
        static mut i_mb_bits: [uint8_t; 27] = [
            0x31 as ::core::ffi::c_int as uint8_t,
            0x29 as ::core::ffi::c_int as uint8_t,
            0x4 as ::core::ffi::c_int as uint8_t,
            0x35 as ::core::ffi::c_int as uint8_t,
            0x2d as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x43 as ::core::ffi::c_int as uint8_t,
            0x63 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x3d as ::core::ffi::c_int as uint8_t,
            0x2f as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x39 as ::core::ffi::c_int as uint8_t,
            0x25 as ::core::ffi::c_int as uint8_t,
            0x6 as ::core::ffi::c_int as uint8_t,
            0x53 as ::core::ffi::c_int as uint8_t,
            0x73 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x4b as ::core::ffi::c_int as uint8_t,
            0x6b as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x5b as ::core::ffi::c_int as uint8_t,
            0x7b as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0x47 as ::core::ffi::c_int as uint8_t,
            0x67 as ::core::ffi::c_int as uint8_t,
            0x21 as ::core::ffi::c_int as uint8_t,
        ];
        let idx: ::core::ffi::c_int = (i_mb_type
            - B_L0_L0 as ::core::ffi::c_int as ::core::ffi::c_int)
            * 3 as ::core::ffi::c_int
            + ((*h).mb.i_partition - D_16x8 as ::core::ffi::c_int as ::core::ffi::c_int);
        let mut bits: ::core::ffi::c_int = i_mb_bits[idx as usize] as ::core::ffi::c_int;
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 3 as ::core::ffi::c_int,
            bits & 1 as ::core::ffi::c_int,
        );
        x264_10_cabac_encode_decision_c(
            cb,
            27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int - (bits & 1 as ::core::ffi::c_int),
            bits >> 1 as ::core::ffi::c_int & 1 as ::core::ffi::c_int,
        );
        bits >>= 2 as ::core::ffi::c_int;
        if bits != 1 as ::core::ffi::c_int {
            x264_10_cabac_encode_decision_c(
                cb,
                27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                bits & 1 as ::core::ffi::c_int,
            );
            bits >>= 1 as ::core::ffi::c_int;
            x264_10_cabac_encode_decision_c(
                cb,
                27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                bits & 1 as ::core::ffi::c_int,
            );
            bits >>= 1 as ::core::ffi::c_int;
            x264_10_cabac_encode_decision_c(
                cb,
                27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                bits & 1 as ::core::ffi::c_int,
            );
            bits >>= 1 as ::core::ffi::c_int;
            if bits != 1 as ::core::ffi::c_int {
                x264_10_cabac_encode_decision_c(
                    cb,
                    27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                    bits & 1 as ::core::ffi::c_int,
                );
            }
        }
        let mut b_list: *const [uint8_t; 2] =
            (*x264_mb_type_list_table.as_ptr().offset(i_mb_type as isize)).as_ptr()
                as *const [uint8_t; 2];
        if (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
            if (*b_list.offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                != 0
            {
                cabac_ref_b(h, cb, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
            }
            if (*b_list.offset(0 as ::core::ffi::c_int as isize))[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                != 0
                && (*h).mb.i_partition != D_16x16 as ::core::ffi::c_int
            {
                cabac_ref_b(
                    h,
                    cb,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int
                        >> ((*h).mb.i_partition == D_8x16 as ::core::ffi::c_int)
                            as ::core::ffi::c_int,
                );
            }
        }
        if (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] > 1 as ::core::ffi::c_int {
            if (*b_list.offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                != 0
            {
                cabac_ref_b(h, cb, 1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
            }
            if (*b_list.offset(1 as ::core::ffi::c_int as isize))[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                != 0
                && (*h).mb.i_partition != D_16x16 as ::core::ffi::c_int
            {
                cabac_ref_b(
                    h,
                    cb,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int
                        >> ((*h).mb.i_partition == D_8x16 as ::core::ffi::c_int)
                            as ::core::ffi::c_int,
                );
            }
        }
        let mut i_list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_list < 2 as ::core::ffi::c_int {
            if (*h).mb.i_partition == D_16x16 as ::core::ffi::c_int {
                if (*b_list.offset(i_list as isize))[0 as ::core::ffi::c_int as usize] != 0 {
                    let mut mvd_1: uint16_t = cabac_mvd(
                        h,
                        cb,
                        i_list,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                        block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        i_list,
                        mvd_1,
                    );
                }
            } else if (*h).mb.i_partition == D_16x8 as ::core::ffi::c_int {
                if (*b_list.offset(i_list as isize))[0 as ::core::ffi::c_int as usize] != 0 {
                    let mut mvd_2: uint16_t = cabac_mvd(
                        h,
                        cb,
                        i_list,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                        block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        i_list,
                        mvd_2,
                    );
                }
                if (*b_list.offset(i_list as isize))[1 as ::core::ffi::c_int as usize] != 0 {
                    let mut mvd_3: uint16_t = cabac_mvd(
                        h,
                        cb,
                        i_list,
                        8 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                        block_idx_y[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        i_list,
                        mvd_3,
                    );
                }
            } else {
                if (*b_list.offset(i_list as isize))[0 as ::core::ffi::c_int as usize] != 0 {
                    let mut mvd_4: uint16_t = cabac_mvd(
                        h,
                        cb,
                        i_list,
                        0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                        block_idx_y[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        i_list,
                        mvd_4,
                    );
                }
                if (*b_list.offset(i_list as isize))[1 as ::core::ffi::c_int as usize] != 0 {
                    let mut mvd_5: uint16_t = cabac_mvd(
                        h,
                        cb,
                        i_list,
                        4 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                    x264_macroblock_cache_mvd(
                        h,
                        block_idx_x[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                        block_idx_y[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        i_list,
                        mvd_5,
                    );
                }
            }
            i_list += 1;
        }
    } else {
        cabac_mb_header_i(h, cb, i_mb_type, SLICE_TYPE_B as ::core::ffi::c_int, chroma);
    };
}
#[inline(always)]
#[c2rust::src_loc = "612:1"]
unsafe extern "C" fn cabac_cbf_ctxidxinc(
    mut h: *mut x264_t,
    mut i_cat: ::core::ffi::c_int,
    mut i_idx: ::core::ffi::c_int,
    mut b_intra: ::core::ffi::c_int,
    mut b_dc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    static mut base_ctx: [uint16_t; 14] = [
        85 as ::core::ffi::c_int as uint16_t,
        89 as ::core::ffi::c_int as uint16_t,
        93 as ::core::ffi::c_int as uint16_t,
        97 as ::core::ffi::c_int as uint16_t,
        101 as ::core::ffi::c_int as uint16_t,
        1012 as ::core::ffi::c_int as uint16_t,
        460 as ::core::ffi::c_int as uint16_t,
        464 as ::core::ffi::c_int as uint16_t,
        468 as ::core::ffi::c_int as uint16_t,
        1016 as ::core::ffi::c_int as uint16_t,
        472 as ::core::ffi::c_int as uint16_t,
        476 as ::core::ffi::c_int as uint16_t,
        480 as ::core::ffi::c_int as uint16_t,
        1020 as ::core::ffi::c_int as uint16_t,
    ];
    if b_dc != 0 {
        i_idx -= LUMA_DC;
        if i_cat == DCT_CHROMA_DC as ::core::ffi::c_int {
            let mut i_nza: ::core::ffi::c_int =
                if (*h).mb.cache.i_cbp_left != -(1 as ::core::ffi::c_int) {
                    (*h).mb.cache.i_cbp_left >> 8 as ::core::ffi::c_int + i_idx
                        & 1 as ::core::ffi::c_int
                } else {
                    b_intra
                };
            let mut i_nzb: ::core::ffi::c_int = if (*h).mb.cache.i_cbp_top
                != -(1 as ::core::ffi::c_int)
            {
                (*h).mb.cache.i_cbp_top >> 8 as ::core::ffi::c_int + i_idx & 1 as ::core::ffi::c_int
            } else {
                b_intra
            };
            return base_ctx[i_cat as usize] as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * i_nzb
                + i_nza;
        } else {
            let mut i_nza_0: ::core::ffi::c_int = (*h).mb.cache.i_cbp_left
                >> 8 as ::core::ffi::c_int + i_idx
                & 1 as ::core::ffi::c_int;
            let mut i_nzb_0: ::core::ffi::c_int = (*h).mb.cache.i_cbp_top
                >> 8 as ::core::ffi::c_int + i_idx
                & 1 as ::core::ffi::c_int;
            return base_ctx[i_cat as usize] as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * i_nzb_0
                + i_nza_0;
        }
    } else {
        let mut i_nza_1: ::core::ffi::c_int = (*h).mb.cache.non_zero_count
            [(x264_scan8[i_idx as usize] as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        let mut i_nzb_1: ::core::ffi::c_int = (*h).mb.cache.non_zero_count
            [(x264_scan8[i_idx as usize] as ::core::ffi::c_int - 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        if 0 != 0 && b_intra == 0 {
            return base_ctx[i_cat as usize] as ::core::ffi::c_int
                + (2 as ::core::ffi::c_int * i_nzb_1 + i_nza_1 & 0x7f as ::core::ffi::c_int);
        } else {
            i_nza_1 &= 0x7f as ::core::ffi::c_int + (b_intra << 7 as ::core::ffi::c_int);
            i_nzb_1 &= 0x7f as ::core::ffi::c_int + (b_intra << 7 as ::core::ffi::c_int);
            return base_ctx[i_cat as usize] as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * (i_nzb_1 != 0) as ::core::ffi::c_int
                + (i_nza_1 != 0) as ::core::ffi::c_int;
        }
    };
}
#[c2rust::src_loc = "650:22"]
static mut coeff_abs_level1_ctx: [uint8_t; 8] = [
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
#[c2rust::src_loc = "652:22"]
static mut coeff_abs_levelgt1_ctx: [uint8_t; 8] = [
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
];
#[c2rust::src_loc = "655:22"]
static mut coeff_abs_levelgt1_ctx_chroma_dc: [uint8_t; 8] = [
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
];
#[c2rust::src_loc = "657:22"]
static mut coeff_abs_level_transition: [[uint8_t; 8]; 2] = [
    [
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
    ],
    [
        4 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
    ],
];
#[inline(always)]
#[c2rust::src_loc = "665:1"]
unsafe extern "C" fn cabac_block_residual_internal(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut ctx_block_cat: ::core::ffi::c_int,
    mut l: *mut dctcoef,
    mut chroma422dc: ::core::ffi::c_int,
) {
    let mut ctx_sig: ::core::ffi::c_int = x264_significant_coeff_flag_offset
        [(*h).mb.b_interlaced as usize][ctx_block_cat as usize]
        as ::core::ffi::c_int;
    let mut ctx_last: ::core::ffi::c_int = x264_last_coeff_flag_offset
        [(*h).mb.b_interlaced as usize][ctx_block_cat as usize]
        as ::core::ffi::c_int;
    let mut ctx_level: ::core::ffi::c_int =
        x264_coeff_abs_level_m1_offset[ctx_block_cat as usize] as ::core::ffi::c_int;
    let mut coeff_idx: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut node_ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut last: ::core::ffi::c_int =
        (*h).quantf.coeff_last[ctx_block_cat as usize].expect("non-null function pointer")(l);
    let mut levelgt1_ctx: *const uint8_t = if chroma422dc != 0 {
        coeff_abs_levelgt1_ctx_chroma_dc.as_ptr()
    } else {
        coeff_abs_levelgt1_ctx.as_ptr()
    };
    let mut coeffs: [dctcoef; 64] = [0; 64];
    if chroma422dc != 0 {
        let mut count_m1: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            if *l.offset(i as isize) != 0 {
                coeff_idx += 1;
                coeffs[coeff_idx as usize] = *l.offset(i as isize);
                x264_10_cabac_encode_decision_c(
                    cb,
                    ctx_sig
                        + x264_coeff_flag_offset_chroma_422_dc[i as usize] as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                if i == last {
                    x264_10_cabac_encode_decision_c(
                        cb,
                        ctx_last
                            + x264_coeff_flag_offset_chroma_422_dc[i as usize]
                                as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    break;
                } else {
                    x264_10_cabac_encode_decision_c(
                        cb,
                        ctx_last
                            + x264_coeff_flag_offset_chroma_422_dc[i as usize]
                                as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                }
            } else {
                x264_10_cabac_encode_decision_c(
                    cb,
                    ctx_sig
                        + x264_coeff_flag_offset_chroma_422_dc[i as usize] as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
            }
            i += 1;
            if !(i == count_m1) {
                continue;
            }
            coeff_idx += 1;
            coeffs[coeff_idx as usize] = *l.offset(i as isize);
            break;
        }
    } else {
        let mut count_m1_0: ::core::ffi::c_int =
            x264_count_cat_m1[ctx_block_cat as usize] as ::core::ffi::c_int;
        if count_m1_0 == 63 as ::core::ffi::c_int {
            let mut sig_offset: *const uint8_t = (*x264_significant_coeff_flag_offset_8x8
                .as_ptr()
                .offset((*h).mb.b_interlaced as isize))
            .as_ptr();
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            loop {
                if *l.offset(i_0 as isize) != 0 {
                    coeff_idx += 1;
                    coeffs[coeff_idx as usize] = *l.offset(i_0 as isize);
                    x264_10_cabac_encode_decision_c(
                        cb,
                        ctx_sig + *sig_offset.offset(i_0 as isize) as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    if i_0 == last {
                        x264_10_cabac_encode_decision_c(
                            cb,
                            ctx_last
                                + x264_last_coeff_flag_offset_8x8[i_0 as usize]
                                    as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                        break;
                    } else {
                        x264_10_cabac_encode_decision_c(
                            cb,
                            ctx_last
                                + x264_last_coeff_flag_offset_8x8[i_0 as usize]
                                    as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    x264_10_cabac_encode_decision_c(
                        cb,
                        ctx_sig + *sig_offset.offset(i_0 as isize) as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                }
                i_0 += 1;
                if !(i_0 == count_m1_0) {
                    continue;
                }
                coeff_idx += 1;
                coeffs[coeff_idx as usize] = *l.offset(i_0 as isize);
                break;
            }
        } else {
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            loop {
                if *l.offset(i_1 as isize) != 0 {
                    coeff_idx += 1;
                    coeffs[coeff_idx as usize] = *l.offset(i_1 as isize);
                    x264_10_cabac_encode_decision_c(cb, ctx_sig + i_1, 1 as ::core::ffi::c_int);
                    if i_1 == last {
                        x264_10_cabac_encode_decision_c(
                            cb,
                            ctx_last + i_1,
                            1 as ::core::ffi::c_int,
                        );
                        break;
                    } else {
                        x264_10_cabac_encode_decision_c(
                            cb,
                            ctx_last + i_1,
                            0 as ::core::ffi::c_int,
                        );
                    }
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctx_sig + i_1, 0 as ::core::ffi::c_int);
                }
                i_1 += 1;
                if !(i_1 == count_m1_0) {
                    continue;
                }
                coeff_idx += 1;
                coeffs[coeff_idx as usize] = *l.offset(i_1 as isize);
                break;
            }
        }
    }
    loop {
        let mut coeff: ::core::ffi::c_int = coeffs[coeff_idx as usize];
        let mut abs_coeff: ::core::ffi::c_int = abs(coeff);
        let mut coeff_sign: ::core::ffi::c_int = coeff >> 31 as ::core::ffi::c_int;
        let mut ctx: ::core::ffi::c_int =
            coeff_abs_level1_ctx[node_ctx as usize] as ::core::ffi::c_int + ctx_level;
        if abs_coeff > 1 as ::core::ffi::c_int {
            x264_10_cabac_encode_decision_c(cb, ctx, 1 as ::core::ffi::c_int);
            ctx = *levelgt1_ctx.offset(node_ctx as isize) as ::core::ffi::c_int + ctx_level;
            let mut i_2: ::core::ffi::c_int = (if abs_coeff < 15 as ::core::ffi::c_int {
                abs_coeff
            } else {
                15 as ::core::ffi::c_int
            }) - 2 as ::core::ffi::c_int;
            while i_2 > 0 as ::core::ffi::c_int {
                x264_10_cabac_encode_decision_c(cb, ctx, 1 as ::core::ffi::c_int);
                i_2 -= 1;
            }
            if abs_coeff < 15 as ::core::ffi::c_int {
                x264_10_cabac_encode_decision_c(cb, ctx, 0 as ::core::ffi::c_int);
            } else {
                x264_10_cabac_encode_ue_bypass(
                    cb,
                    0 as ::core::ffi::c_int,
                    abs_coeff - 15 as ::core::ffi::c_int,
                );
            }
            node_ctx = coeff_abs_level_transition[1 as ::core::ffi::c_int as usize]
                [node_ctx as usize] as ::core::ffi::c_int;
        } else {
            x264_10_cabac_encode_decision_c(cb, ctx, 0 as ::core::ffi::c_int);
            node_ctx = coeff_abs_level_transition[0 as ::core::ffi::c_int as usize]
                [node_ctx as usize] as ::core::ffi::c_int;
        }
        x264_10_cabac_encode_bypass_c(cb, coeff_sign);
        coeff_idx -= 1;
        if !(coeff_idx >= 0 as ::core::ffi::c_int) {
            break;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "750:1"]
pub unsafe extern "C" fn x264_10_cabac_block_residual_c(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut ctx_block_cat: ::core::ffi::c_int,
    mut l: *mut dctcoef,
) {
    cabac_block_residual_internal(h, cb, ctx_block_cat, l, 0 as ::core::ffi::c_int);
}
#[inline(always)]
#[c2rust::src_loc = "755:1"]
unsafe extern "C" fn cabac_block_residual(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut ctx_block_cat: ::core::ffi::c_int,
    mut l: *mut dctcoef,
) {
    x264_10_cabac_block_residual_c(h, cb, ctx_block_cat, l);
}
#[c2rust::src_loc = "763:1"]
unsafe extern "C" fn cabac_block_residual_422_dc(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut ctx_block_cat: ::core::ffi::c_int,
    mut l: *mut dctcoef,
) {
    cabac_block_residual_internal(
        h,
        cb,
        DCT_CHROMA_DC as ::core::ffi::c_int,
        l,
        1 as ::core::ffi::c_int,
    );
}
#[inline(always)]
#[c2rust::src_loc = "917:1"]
unsafe extern "C" fn macroblock_write_cabac_internal(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
    mut plane_count: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    let i_mb_type: ::core::ffi::c_int = (*h).mb.i_type;
    let i_mb_pos_start: ::core::ffi::c_int = x264_cabac_pos(cb) as ::core::ffi::c_int;
    let mut i_mb_pos_tex: ::core::ffi::c_int = 0;
    if (*h).sh.b_mbaff != 0
        && ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int == 0
            || (*(*h)
                .mb
                .type_0
                .offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
                as ::core::ffi::c_int
                == P_SKIP as ::core::ffi::c_int
                || *(*h)
                    .mb
                    .type_0
                    .offset(((*h).mb.i_mb_xy - (*h).mb.i_mb_stride) as isize)
                    as ::core::ffi::c_int
                    == B_SKIP as ::core::ffi::c_int))
    {
        cabac_field_decoding_flag(h, cb);
    }
    if (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int {
        cabac_mb_header_p(h, cb, i_mb_type, chroma);
    } else if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
        cabac_mb_header_b(h, cb, i_mb_type, chroma);
    } else {
        cabac_mb_header_i(h, cb, i_mb_type, SLICE_TYPE_I as ::core::ffi::c_int, chroma);
    }
    i_mb_pos_tex = x264_cabac_pos(cb);
    (*h).stat.frame.i_mv_bits += i_mb_pos_tex - i_mb_pos_start;
    if i_mb_type == I_PCM as ::core::ffi::c_int {
        let mut s: bs_t = bs_s {
            p_start: 0 as *mut uint8_t,
            p: 0 as *mut uint8_t,
            p_end: 0 as *mut uint8_t,
            cur_bits: 0,
            i_left: 0,
            i_bits_encoded: 0,
        };
        bs_init(
            &mut s,
            (*cb).p as *mut ::core::ffi::c_void,
            (*cb).p_end.offset_from((*cb).p) as ::core::ffi::c_long as ::core::ffi::c_int,
        );
        let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while p < plane_count {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 256 as ::core::ffi::c_int {
                bs_write(
                    &mut s,
                    BIT_DEPTH,
                    *(*h).mb.pic.p_fenc[p as usize].offset(i as isize) as uint32_t,
                );
                i += 1;
            }
            p += 1;
        }
        if chroma != 0 {
            let mut ch: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            while ch < 3 as ::core::ffi::c_int {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < 16 as ::core::ffi::c_int >> (*h).mb.chroma_v_shift {
                    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while j < 8 as ::core::ffi::c_int {
                        bs_write(
                            &mut s,
                            BIT_DEPTH,
                            *(*h).mb.pic.p_fenc[ch as usize]
                                .offset((i_0 * FENC_STRIDE + j) as isize)
                                as uint32_t,
                        );
                        j += 1;
                    }
                    i_0 += 1;
                }
                ch += 1;
            }
        }
        bs_flush(&mut s);
        (*cb).p = s.p;
        x264_10_cabac_encode_init_core(cb);
        (*h).stat.frame.i_tex_bits += x264_cabac_pos(cb) - i_mb_pos_tex;
        return;
    }
    if i_mb_type != I_16x16 as ::core::ffi::c_int {
        cabac_cbp_luma(h, cb);
        if chroma != 0 {
            cabac_cbp_chroma(h, cb);
        }
    }
    if x264_mb_transform_8x8_allowed(h) != 0 && (*h).mb.i_cbp_luma != 0 {
        cabac_transform_size(h, cb);
    }
    if (*h).mb.i_cbp_luma != 0
        || chroma != 0 && (*h).mb.i_cbp_chroma != 0
        || i_mb_type == I_16x16 as ::core::ffi::c_int
    {
        let b_intra: ::core::ffi::c_int = (i_mb_type == I_4x4 as ::core::ffi::c_int
            || i_mb_type == I_8x8 as ::core::ffi::c_int
            || i_mb_type == I_16x16 as ::core::ffi::c_int
            || i_mb_type == I_PCM as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        cabac_qp_delta(h, cb);
        if i_mb_type == I_16x16 as ::core::ffi::c_int {
            let mut p_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p_0 < plane_count {
                let mut ctxidxinc: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                    h,
                    ctx_cat_plane[DCT_LUMA_DC as ::core::ffi::c_int as usize][p_0 as usize]
                        as ::core::ffi::c_int,
                    48 as ::core::ffi::c_int + p_0,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                if (*h).mb.cache.non_zero_count
                    [x264_scan8[(48 as ::core::ffi::c_int + p_0) as usize] as usize]
                    != 0
                {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc, 1 as ::core::ffi::c_int);
                    cabac_block_residual(
                        h,
                        cb,
                        ctx_cat_plane[DCT_LUMA_DC as ::core::ffi::c_int as usize][p_0 as usize]
                            as ::core::ffi::c_int,
                        (*(*h).dct.luma16x16_dc.as_mut_ptr().offset(p_0 as isize)).as_mut_ptr(),
                    );
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc, 0 as ::core::ffi::c_int);
                }
                if (*h).mb.i_cbp_luma != 0 {
                    let mut i_1: ::core::ffi::c_int = p_0 * 16 as ::core::ffi::c_int;
                    while i_1 < p_0 * 16 as ::core::ffi::c_int + 16 as ::core::ffi::c_int {
                        let mut ctxidxinc_0: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                            h,
                            ctx_cat_plane[DCT_LUMA_AC as ::core::ffi::c_int as usize][p_0 as usize]
                                as ::core::ffi::c_int,
                            i_1,
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        );
                        if (*h).mb.cache.non_zero_count[x264_scan8[i_1 as usize] as usize] != 0 {
                            x264_10_cabac_encode_decision_c(
                                cb,
                                ctxidxinc_0,
                                1 as ::core::ffi::c_int,
                            );
                            cabac_block_residual(
                                h,
                                cb,
                                ctx_cat_plane[DCT_LUMA_AC as ::core::ffi::c_int as usize]
                                    [p_0 as usize]
                                    as ::core::ffi::c_int,
                                (*(*h).dct.luma4x4.as_mut_ptr().offset(i_1 as isize))
                                    .as_mut_ptr()
                                    .offset(1 as ::core::ffi::c_int as isize),
                            );
                        } else {
                            x264_10_cabac_encode_decision_c(
                                cb,
                                ctxidxinc_0,
                                0 as ::core::ffi::c_int,
                            );
                        }
                        i_1 += 1;
                    }
                }
                p_0 += 1;
            }
        } else if (*h).mb.b_transform_8x8 != 0 {
            if plane_count == 3 as ::core::ffi::c_int {
                let mut nnzbak: [[uint8_t; 8]; 3] = [[0; 8]; 3];
                if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                        == 0
                    && *(*h)
                        .mb
                        .cbp
                        .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                        as ::core::ffi::c_int
                        & 0x1000 as ::core::ffi::c_int
                        == 0
                {
                    nnzbak[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 0 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                    nnzbak[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                    nnzbak[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 1 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                    nnzbak[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                    nnzbak[2 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                    nnzbak[2 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                }
                if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
                        == 0
                    && *(*h)
                        .mb
                        .cbp
                        .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
                        as ::core::ffi::c_int
                        & 0x1000 as ::core::ffi::c_int
                        == 0
                {
                    nnzbak[0 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 0 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                    nnzbak[0 as ::core::ffi::c_int as usize][3 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 0 as ::core::ffi::c_int
                            + 10 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 0 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                    nnzbak[1 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 1 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                    nnzbak[1 as ::core::ffi::c_int as usize][3 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 1 as ::core::ffi::c_int
                            + 10 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 1 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                    nnzbak[2 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                    nnzbak[2 as ::core::ffi::c_int as usize][3 as ::core::ffi::c_int as usize] =
                        (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int
                            + 10 as ::core::ffi::c_int)
                            as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] = 0 as uint8_t;
                }
                if (*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_top_xy as isize)
                        == 0
                    && *(*h).mb.cbp.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
                        & 0x1000 as ::core::ffi::c_int
                        == 0
                {
                    (*(&mut *(*nnzbak.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut uint8_t as *mut x264_union32_t))
                        .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as ::core::ffi::c_uint as uint32_t;
                    (*(&mut *(*nnzbak.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut uint8_t as *mut x264_union32_t))
                        .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as ::core::ffi::c_uint as uint32_t;
                    (*(&mut *(*nnzbak.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(4 as ::core::ffi::c_int as isize)
                        as *mut uint8_t as *mut x264_union32_t))
                        .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i;
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as ::core::ffi::c_uint as uint32_t;
                }
                let mut p_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while p_1 < 3 as ::core::ffi::c_int {
                    let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut msk: ::core::ffi::c_int = (*h).mb.i_cbp_luma;
                    let mut skip: ::core::ffi::c_int = 0;
                    while msk != 0 && {
                        skip = x264_ctz_4bit(msk as uint32_t);
                        i_2 += skip;
                        msk >>= skip + 1 as ::core::ffi::c_int;
                        1 as ::core::ffi::c_int != 0
                    } {
                        let mut ctxidxinc_1: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                            h,
                            ctx_cat_plane[DCT_LUMA_8x8 as ::core::ffi::c_int as usize][p_1 as usize]
                                as ::core::ffi::c_int,
                            i_2 * 4 as ::core::ffi::c_int + p_1 * 16 as ::core::ffi::c_int,
                            b_intra,
                            0 as ::core::ffi::c_int,
                        );
                        if (*h).mb.cache.non_zero_count[x264_scan8[(i_2 * 4 as ::core::ffi::c_int
                            + p_1 * 16 as ::core::ffi::c_int)
                            as usize]
                            as usize]
                            != 0
                        {
                            x264_10_cabac_encode_decision_c(
                                cb,
                                ctxidxinc_1,
                                1 as ::core::ffi::c_int,
                            );
                            cabac_block_residual(
                                h,
                                cb,
                                ctx_cat_plane[DCT_LUMA_8x8 as ::core::ffi::c_int as usize]
                                    [p_1 as usize]
                                    as ::core::ffi::c_int,
                                (*(*h)
                                    .dct
                                    .luma8x8
                                    .as_mut_ptr()
                                    .offset((i_2 + p_1 * 4 as ::core::ffi::c_int) as isize))
                                .as_mut_ptr(),
                            );
                        } else {
                            x264_10_cabac_encode_decision_c(
                                cb,
                                ctxidxinc_1,
                                0 as ::core::ffi::c_int,
                            );
                        }
                        i_2 += 1;
                    }
                    p_1 += 1;
                }
                if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                        == 0
                    && *(*h)
                        .mb
                        .cbp
                        .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                        as ::core::ffi::c_int
                        & 0x1000 as ::core::ffi::c_int
                        == 0
                {
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 0 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 1 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[2 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[2 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize];
                }
                if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
                        == 0
                    && *(*h)
                        .mb
                        .cbp
                        .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
                        as ::core::ffi::c_int
                        & 0x1000 as ::core::ffi::c_int
                        == 0
                {
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 0 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[0 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 0 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[0 as ::core::ffi::c_int as usize][3 as ::core::ffi::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 1 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[1 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 1 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[1 as ::core::ffi::c_int as usize][3 as ::core::ffi::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[2 as ::core::ffi::c_int as usize][2 as ::core::ffi::c_int as usize];
                    (*h).mb.cache.non_zero_count[(x264_scan8[(16 as ::core::ffi::c_int
                        * 2 as ::core::ffi::c_int
                        + 10 as ::core::ffi::c_int)
                        as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] =
                        nnzbak[2 as ::core::ffi::c_int as usize][3 as ::core::ffi::c_int as usize];
                }
                if (*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                    && *(*h)
                        .mb
                        .mb_transform_size
                        .offset((*h).mb.i_mb_top_xy as isize)
                        == 0
                    && *(*h).mb.cbp.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
                        & 0x1000 as ::core::ffi::c_int
                        == 0
                {
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i =
                        (*(&mut *(*nnzbak.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(4 as ::core::ffi::c_int as isize)
                            as *mut uint8_t as *mut x264_union32_t))
                            .i;
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i =
                        (*(&mut *(*nnzbak.as_mut_ptr().offset(1 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(4 as ::core::ffi::c_int as isize)
                            as *mut uint8_t as *mut x264_union32_t))
                            .i;
                    (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                        .i =
                        (*(&mut *(*nnzbak.as_mut_ptr().offset(2 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(4 as ::core::ffi::c_int as isize)
                            as *mut uint8_t as *mut x264_union32_t))
                            .i;
                }
            } else {
                let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut msk_0: ::core::ffi::c_int = (*h).mb.i_cbp_luma;
                let mut skip_0: ::core::ffi::c_int = 0;
                while msk_0 != 0 && {
                    skip_0 = x264_ctz_4bit(msk_0 as uint32_t);
                    i_3 += skip_0;
                    msk_0 >>= skip_0 + 1 as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int != 0
                } {
                    cabac_block_residual(
                        h,
                        cb,
                        DCT_LUMA_8x8 as ::core::ffi::c_int,
                        (*(*h).dct.luma8x8.as_mut_ptr().offset(i_3 as isize)).as_mut_ptr(),
                    );
                    i_3 += 1;
                }
            }
        } else {
            let mut p_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p_2 < plane_count {
                let mut i8x8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut msk_1: ::core::ffi::c_int = (*h).mb.i_cbp_luma;
                let mut skip_1: ::core::ffi::c_int = 0;
                while msk_1 != 0 && {
                    skip_1 = x264_ctz_4bit(msk_1 as uint32_t);
                    i8x8 += skip_1;
                    msk_1 >>= skip_1 + 1 as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int != 0
                } {
                    let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i_4 < 4 as ::core::ffi::c_int {
                        let mut ctxidxinc_2: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                            h,
                            ctx_cat_plane[DCT_LUMA_4x4 as ::core::ffi::c_int as usize][p_2 as usize]
                                as ::core::ffi::c_int,
                            i_4 + i8x8 * 4 as ::core::ffi::c_int + p_2 * 16 as ::core::ffi::c_int,
                            b_intra,
                            0 as ::core::ffi::c_int,
                        );
                        if (*h).mb.cache.non_zero_count[x264_scan8[(i_4
                            + i8x8 * 4 as ::core::ffi::c_int
                            + p_2 * 16 as ::core::ffi::c_int)
                            as usize]
                            as usize]
                            != 0
                        {
                            x264_10_cabac_encode_decision_c(
                                cb,
                                ctxidxinc_2,
                                1 as ::core::ffi::c_int,
                            );
                            cabac_block_residual(
                                h,
                                cb,
                                ctx_cat_plane[DCT_LUMA_4x4 as ::core::ffi::c_int as usize]
                                    [p_2 as usize]
                                    as ::core::ffi::c_int,
                                (*(*h).dct.luma4x4.as_mut_ptr().offset(
                                    (i_4 + i8x8 * 4 as ::core::ffi::c_int
                                        + p_2 * 16 as ::core::ffi::c_int)
                                        as isize,
                                ))
                                .as_mut_ptr(),
                            );
                        } else {
                            x264_10_cabac_encode_decision_c(
                                cb,
                                ctxidxinc_2,
                                0 as ::core::ffi::c_int,
                            );
                        }
                        i_4 += 1;
                    }
                    i8x8 += 1;
                }
                p_2 += 1;
            }
        }
        if chroma != 0 && (*h).mb.i_cbp_chroma != 0 {
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as ::core::ffi::c_int {
                let mut ctxidxinc_3: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as ::core::ffi::c_int,
                    49 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
                    b_intra,
                    1 as ::core::ffi::c_int,
                );
                if (*h).mb.cache.non_zero_count[x264_scan8
                    [(49 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                    as usize]
                    != 0
                {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_3, 1 as ::core::ffi::c_int);
                    cabac_block_residual_422_dc(
                        h,
                        cb,
                        DCT_CHROMA_DC as ::core::ffi::c_int,
                        (*(*h)
                            .dct
                            .chroma_dc
                            .as_mut_ptr()
                            .offset(0 as ::core::ffi::c_int as isize))
                        .as_mut_ptr(),
                    );
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_3, 0 as ::core::ffi::c_int);
                }
                let mut ctxidxinc_4: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as ::core::ffi::c_int,
                    49 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                    b_intra,
                    1 as ::core::ffi::c_int,
                );
                if (*h).mb.cache.non_zero_count[x264_scan8
                    [(49 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                    as usize]
                    != 0
                {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_4, 1 as ::core::ffi::c_int);
                    cabac_block_residual_422_dc(
                        h,
                        cb,
                        DCT_CHROMA_DC as ::core::ffi::c_int,
                        (*(*h)
                            .dct
                            .chroma_dc
                            .as_mut_ptr()
                            .offset(1 as ::core::ffi::c_int as isize))
                        .as_mut_ptr(),
                    );
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_4, 0 as ::core::ffi::c_int);
                }
            } else {
                let mut ctxidxinc_5: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as ::core::ffi::c_int,
                    49 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
                    b_intra,
                    1 as ::core::ffi::c_int,
                );
                if (*h).mb.cache.non_zero_count[x264_scan8
                    [(49 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                    as usize]
                    != 0
                {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_5, 1 as ::core::ffi::c_int);
                    cabac_block_residual(
                        h,
                        cb,
                        DCT_CHROMA_DC as ::core::ffi::c_int,
                        (*(*h)
                            .dct
                            .chroma_dc
                            .as_mut_ptr()
                            .offset(0 as ::core::ffi::c_int as isize))
                        .as_mut_ptr(),
                    );
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_5, 0 as ::core::ffi::c_int);
                }
                let mut ctxidxinc_6: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                    h,
                    DCT_CHROMA_DC as ::core::ffi::c_int,
                    49 as ::core::ffi::c_int + 1 as ::core::ffi::c_int,
                    b_intra,
                    1 as ::core::ffi::c_int,
                );
                if (*h).mb.cache.non_zero_count[x264_scan8
                    [(49 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                    as usize]
                    != 0
                {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_6, 1 as ::core::ffi::c_int);
                    cabac_block_residual(
                        h,
                        cb,
                        DCT_CHROMA_DC as ::core::ffi::c_int,
                        (*(*h)
                            .dct
                            .chroma_dc
                            .as_mut_ptr()
                            .offset(1 as ::core::ffi::c_int as isize))
                        .as_mut_ptr(),
                    );
                } else {
                    x264_10_cabac_encode_decision_c(cb, ctxidxinc_6, 0 as ::core::ffi::c_int);
                }
            }
            if (*h).mb.i_cbp_chroma == 2 as ::core::ffi::c_int {
                let mut step: ::core::ffi::c_int =
                    (8 as ::core::ffi::c_int) << (*h).mb.chroma_v_shift;
                let mut i_5: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
                while i_5 < 3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int {
                    let mut j_0: ::core::ffi::c_int = i_5;
                    while j_0 < i_5 + 4 as ::core::ffi::c_int {
                        let mut ctxidxinc_7: ::core::ffi::c_int = cabac_cbf_ctxidxinc(
                            h,
                            DCT_CHROMA_AC as ::core::ffi::c_int,
                            j_0,
                            b_intra,
                            0 as ::core::ffi::c_int,
                        );
                        if (*h).mb.cache.non_zero_count[x264_scan8[j_0 as usize] as usize] != 0 {
                            x264_10_cabac_encode_decision_c(
                                cb,
                                ctxidxinc_7,
                                1 as ::core::ffi::c_int,
                            );
                            cabac_block_residual(
                                h,
                                cb,
                                DCT_CHROMA_AC as ::core::ffi::c_int,
                                (*(*h).dct.luma4x4.as_mut_ptr().offset(j_0 as isize))
                                    .as_mut_ptr()
                                    .offset(1 as ::core::ffi::c_int as isize),
                            );
                        } else {
                            x264_10_cabac_encode_decision_c(
                                cb,
                                ctxidxinc_7,
                                0 as ::core::ffi::c_int,
                            );
                        }
                        j_0 += 1;
                    }
                    i_5 += step;
                }
            }
        }
    }
    (*h).stat.frame.i_tex_bits += x264_cabac_pos(cb) - i_mb_pos_tex;
}
#[no_mangle]
#[c2rust::src_loc = "1088:1"]
pub unsafe extern "C" fn x264_10_macroblock_write_cabac(
    mut h: *mut x264_t,
    mut cb: *mut x264_cabac_t,
) {
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
        macroblock_write_cabac_internal(h, cb, 3 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        macroblock_write_cabac_internal(h, cb, 1 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
    } else {
        macroblock_write_cabac_internal(h, cb, 1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    };
}
