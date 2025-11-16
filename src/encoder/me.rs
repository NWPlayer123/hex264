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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "765:9"]
    pub struct mvsad_t {
        pub sad: ::core::ffi::c_int,
        pub mv: [int16_t; 2],
    }
    #[c2rust::src_loc = "570:9"]
    pub const FENC_STRIDE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    #[c2rust::src_loc = "571:9"]
    pub const FDEC_STRIDE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    #[inline(always)]
    #[c2rust::src_loc = "774:1"]
    pub unsafe extern "C" fn x264_predictor_roundclip(
        mut dst: *mut [int16_t; 2],
        mut mvc: *mut [int16_t; 2],
        mut i_mvc: ::core::ffi::c_int,
        mut mv_limit: *mut [int16_t; 2],
        mut pmv: uint32_t,
    ) -> ::core::ffi::c_int {
        let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < i_mvc {
            let mut mx: ::core::ffi::c_int =
                (*mvc.offset(i as isize))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int;
            let mut my: ::core::ffi::c_int =
                (*mvc.offset(i as isize))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int;
            let mut mv: uint32_t = pack16to32_mask(mx, my);
            if !(mv == 0 || mv == pmv) {
                (*dst.offset(cnt as isize))[0 as ::core::ffi::c_int as usize] = x264_clip3(
                    mx,
                    (*mv_limit.offset(0 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    (*mv_limit.offset(1 as ::core::ffi::c_int as isize))
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                )
                    as int16_t;
                (*dst.offset(cnt as isize))[1 as ::core::ffi::c_int as usize] = x264_clip3(
                    my,
                    (*mv_limit.offset(0 as ::core::ffi::c_int as isize))
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    (*mv_limit.offset(1 as ::core::ffi::c_int as isize))
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                )
                    as int16_t;
                cnt += 1;
            }
            i += 1;
        }
        return cnt;
    }
    #[inline(always)]
    #[c2rust::src_loc = "790:1"]
    pub unsafe extern "C" fn x264_predictor_clip(
        mut dst: *mut [int16_t; 2],
        mut mvc: *mut [int16_t; 2],
        mut i_mvc: ::core::ffi::c_int,
        mut mv_limit: *mut [int16_t; 2],
        mut pmv: uint32_t,
    ) -> ::core::ffi::c_int {
        let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut qpel_limit: [::core::ffi::c_int; 4] = [
            ((*mv_limit.offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int,
            ((*mv_limit.offset(0 as ::core::ffi::c_int as isize))[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int,
            ((*mv_limit.offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int,
            ((*mv_limit.offset(1 as ::core::ffi::c_int as isize))[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int,
        ];
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < i_mvc {
            let mut mv: uint32_t =
                (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i;
            let mut mx: ::core::ffi::c_int =
                (*mvc.offset(i as isize))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
            let mut my: ::core::ffi::c_int =
                (*mvc.offset(i as isize))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
            if !(mv == 0 || mv == pmv) {
                (*dst.offset(cnt as isize))[0 as ::core::ffi::c_int as usize] = x264_clip3(
                    mx,
                    qpel_limit[0 as ::core::ffi::c_int as usize],
                    qpel_limit[2 as ::core::ffi::c_int as usize],
                )
                    as int16_t;
                (*dst.offset(cnt as isize))[1 as ::core::ffi::c_int as usize] = x264_clip3(
                    my,
                    qpel_limit[1 as ::core::ffi::c_int as usize],
                    qpel_limit[3 as ::core::ffi::c_int as usize],
                )
                    as int16_t;
                cnt += 1;
            }
            i += 1;
        }
        return cnt;
    }
    use super::base_h::{x264_clip3, x264_union32_t};
    use super::bitstream_h::{bs_t, x264_bitstream_function_t};
    use super::cabac_h::x264_cabac_t;
    use super::dct_h::{x264_dct_function_t, x264_zigzag_function_t};
    use super::frame_h::{x264_deblock_function_t, x264_frame_t, x264_sync_frame_list_t};
    use super::macroblock_h::pack16to32_mask;
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
    #[c2rust::src_loc = "33:9"]
    pub const PADV: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
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
    #[c2rust::src_loc = "206:9"]
    pub const X264_ME_DIA: ::core::ffi::c_int = 0;
    #[c2rust::src_loc = "207:9"]
    pub const X264_ME_HEX: ::core::ffi::c_int = 1;
    #[c2rust::src_loc = "208:9"]
    pub const X264_ME_UMH: ::core::ffi::c_int = 2;
    #[c2rust::src_loc = "209:9"]
    pub const X264_ME_ESA: ::core::ffi::c_int = 3;
    #[c2rust::src_loc = "210:9"]
    pub const X264_ME_TESA: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
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
    use super::cabac_h::x264_cabac_t;
    use super::common_h::dctcoef;
    use super::stdint_h::{intptr_t, uintptr_t};
    use super::stdint_intn_h::int32_t;
    use super::stdint_uintn_h::uint8_t;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "55:14"]
    pub struct C2RustUnnamed_20 {
        pub w: uint8_t,
        pub h: uint8_t,
    }
    #[c2rust::src_loc = "55:39"]
    pub static mut x264_pixel_size: [C2RustUnnamed_20; 12] = [
        {
            let mut init = C2RustUnnamed_20 {
                w: 16 as uint8_t,
                h: 16 as uint8_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_20 {
                w: 16 as uint8_t,
                h: 8 as uint8_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_20 {
                w: 8 as uint8_t,
                h: 16 as uint8_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_20 {
                w: 8 as uint8_t,
                h: 8 as uint8_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_20 {
                w: 8 as uint8_t,
                h: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_20 {
                w: 4 as uint8_t,
                h: 8 as uint8_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_20 {
                w: 4 as uint8_t,
                h: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_20 {
                w: 4 as uint8_t,
                h: 16 as uint8_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_20 {
                w: 4 as uint8_t,
                h: 2 as uint8_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_20 {
                w: 2 as uint8_t,
                h: 8 as uint8_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_20 {
                w: 2 as uint8_t,
                h: 4 as uint8_t,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_20 {
                w: 2 as uint8_t,
                h: 2 as uint8_t,
            };
            init
        },
    ];
    use super::common_h::pixel;
    use super::stdint_h::intptr_t;
    use super::stdint_intn_h::int16_t;
    use super::stdint_uintn_h::{uint16_t, uint64_t, uint8_t};
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
    #[c2rust::src_loc = "155:9"]
    pub const X264_SCAN8_0: ::core::ffi::c_int =
        4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
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
    #[inline(always)]
    #[c2rust::src_loc = "248:1"]
    pub unsafe extern "C" fn x264_predictor_difference(
        mut mvc: *mut [int16_t; 2],
        mut i_mvc: intptr_t,
    ) -> ::core::ffi::c_int {
        let mut sum: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while (i as intptr_t) < i_mvc - 1 as intptr_t {
            sum += abs((*mvc.offset(i as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - (*mvc.offset((i + 1 as ::core::ffi::c_int) as isize))
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                + abs((*mvc.offset(i as isize))[1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - (*mvc.offset((i + 1 as ::core::ffi::c_int) as isize))
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int);
            i += 1;
        }
        return sum;
    }
    use super::stdint_h::intptr_t;
    use super::stdint_intn_h::int16_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
    use super::stdlib_h::abs;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/encoder/me.h:28"]
pub mod me_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "33:9"]
    pub struct x264_me_t {
        pub i_pixel: ::core::ffi::c_int,
        pub p_cost_mv: *mut uint16_t,
        pub i_ref_cost: ::core::ffi::c_int,
        pub i_ref: ::core::ffi::c_int,
        pub weight: *const x264_weight_t,
        pub p_fref: [*mut pixel; 12],
        pub p_fref_w: *mut pixel,
        pub p_fenc: [*mut pixel; 3],
        pub integral: *mut uint16_t,
        pub i_stride: [::core::ffi::c_int; 3],
        pub mvp: [int16_t; 2],
        pub cost_mv: ::core::ffi::c_int,
        pub cost: ::core::ffi::c_int,
        pub mv: [int16_t; 2],
    }
    #[c2rust::src_loc = "30:9"]
    pub const COST_MAX: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int;
    #[c2rust::src_loc = "31:9"]
    pub const COST_MAX64: ::core::ffi::c_ulonglong =
        (1 as ::core::ffi::c_ulonglong) << 60 as ::core::ffi::c_int;
    use super::common_h::{pixel, x264_t};
    use super::mc_h::x264_weight_t;
    use super::stdint_intn_h::int16_t;
    use super::stdint_uintn_h::{uint16_t, uint64_t};
    extern "C" {
        #[c2rust::src_loc = "74:1"]
        pub fn x264_10_rd_cost_part(
            h: *mut x264_t,
            i_lambda2: ::core::ffi::c_int,
            i8: ::core::ffi::c_int,
            i_pixel: ::core::ffi::c_int,
        ) -> uint64_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:28"]
pub mod osdep_h {
    #[c2rust::src_loc = "452:9"]
    pub const WORD_SIZE: uint64_t = ::core::mem::size_of::<*mut ::core::ffi::c_void>() as uint64_t;
    use super::stdint_uintn_h::uint64_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/tables.h:28"]
pub mod tables_h {
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "100:16"]
        pub static mut x264_zero: [uint8_t; 1024];
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/macroblock.h:28"]
pub mod macroblock_h {
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
    #[c2rust::src_loc = "236:23"]
    pub static mut block_idx_xy_fdec: [uint16_t; 16] = [
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE) as uint16_t,
    ];
    #[inline(always)]
    #[c2rust::src_loc = "379:1"]
    pub unsafe extern "C" fn pack8to16(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
        return a.wrapping_add(b << 8 as ::core::ffi::c_int);
    }
    #[inline(always)]
    #[c2rust::src_loc = "395:1"]
    pub unsafe extern "C" fn pack16to32_mask(
        mut a: ::core::ffi::c_int,
        mut b: ::core::ffi::c_int,
    ) -> uint32_t {
        return ((a & 0xffff as ::core::ffi::c_int) as uint32_t)
            .wrapping_add((b as uint32_t) << 16 as ::core::ffi::c_int);
    }
    use super::common_h::{x264_t, FDEC_STRIDE};
    use super::stdint_intn_h::int16_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint8_t};
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
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
            'c_27249: {
                if h != 1 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"h != 1\0" as *const u8 as *const ::core::ffi::c_char,
                        b"./common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                        82 as ::core::ffi::c_uint,
                        __ASSERT_FUNCTION.as_ptr(),
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
                __ASSERT_FUNCTION.as_ptr(),
            );
            'c_27015: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"./common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                    108 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            };
        };
    }
    #[inline(always)]
    #[c2rust::src_loc = "119:1"]
    pub unsafe extern "C" fn x264_macroblock_cache_mv(
        mut h: *mut x264_t,
        mut x: ::core::ffi::c_int,
        mut y: ::core::ffi::c_int,
        mut width: ::core::ffi::c_int,
        mut height: ::core::ffi::c_int,
        mut i_list: ::core::ffi::c_int,
        mut mv: uint32_t,
    ) {
        let mut mv_cache: *mut ::core::ffi::c_void =
            &mut *(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((X264_SCAN8_0 + x + 8 as ::core::ffi::c_int * y) as isize)
                as *mut [int16_t; 2] as *mut ::core::ffi::c_void;
        if 0 == 0 || 0 == 0 {
            x264_10_cache_mv_func_table
                [(width + (height << 1 as ::core::ffi::c_int) - 3 as ::core::ffi::c_int) as usize]
                .expect("non-null function pointer")(mv_cache, mv);
        } else {
            x264_macroblock_cache_rect(
                mv_cache,
                width * 4 as ::core::ffi::c_int,
                height,
                4 as ::core::ffi::c_int,
                mv,
            );
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
    use super::assert_h::{__assert_fail, __ASSERT_FUNCTION};
    use super::base_h::{x264_union16_t, x264_union32_t, x264_union64_t, X264_SCAN8_0};
    use super::common_h::x264_t;
    use super::osdep_h::WORD_SIZE;
    use super::stdint_intn_h::int16_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "112:15"]
        pub static mut x264_10_cache_mv_func_table:
            [Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> ()>; 10];
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
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:28"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/assert.h:28"]
pub mod assert_h {
    #[c2rust::src_loc = "137:12"]
    pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 65] = unsafe {
        ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
            *b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
        )
    };
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
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
pub use self::assert_h::{__assert_fail, __ASSERT_FUNCTION};
pub use self::atomic_wide_counter_h::{C2RustUnnamed, __atomic_wide_counter};
pub use self::base_h::{
    chroma_format_e, x264_clip3, x264_predictor_difference, x264_scan8, x264_union16_t,
    x264_union32_t, x264_union64_t, CHROMA_400, CHROMA_420, CHROMA_422, CHROMA_444, X264_SCAN8_0,
};
pub use self::bitstream_h::{bs_s, bs_t, x264_bitstream_function_t, x264_run_level_t};
pub use self::cabac_h::x264_cabac_t;
pub use self::common_h::{
    dctcoef, mvsad_t, pixel, udctcoef, x264_frame_stat_t, x264_left_table_t, x264_lookahead_t,
    x264_predictor_clip, x264_predictor_roundclip, x264_ratecontrol_t, x264_slice_header_t, x264_t,
    C2RustUnnamed_10, C2RustUnnamed_11, C2RustUnnamed_12, C2RustUnnamed_13, C2RustUnnamed_17,
    C2RustUnnamed_18, C2RustUnnamed_6, C2RustUnnamed_7, C2RustUnnamed_8, C2RustUnnamed_9,
    FDEC_STRIDE, FENC_STRIDE,
};
pub use self::dct_h::{x264_dct_function_t, x264_zigzag_function_t};
pub use self::frame_h::{
    x264_deblock_function_t, x264_deblock_inter_t, x264_deblock_intra_t, x264_frame, x264_frame_t,
    x264_sync_frame_list_t, PADV,
};
pub use self::internal::__va_list_tag;
pub use self::macroblock_h::{
    block_idx_x, block_idx_xy_fdec, block_idx_y, pack16to32_mask, pack8to16, x264_10_mb_predict_mv,
};
pub use self::mc_h::{weight_fn_t, x264_mc_functions_t, x264_weight_t};
pub use self::me_h::{x264_10_rd_cost_part, x264_me_t, COST_MAX, COST_MAX64};
pub use self::osdep_h::WORD_SIZE;
pub use self::pixel_h::{
    x264_pixel_cmp_t, x264_pixel_cmp_x3_t, x264_pixel_cmp_x4_t, x264_pixel_function_t,
    x264_pixel_size, C2RustUnnamed_19, C2RustUnnamed_20, PIXEL_16x16, PIXEL_16x8, PIXEL_2x2,
    PIXEL_2x4, PIXEL_2x8, PIXEL_4x16, PIXEL_4x2, PIXEL_4x4, PIXEL_4x8, PIXEL_8x16, PIXEL_8x4,
    PIXEL_8x8,
};
pub use self::predict_h::{x264_predict8x8_t, x264_predict_8x8_filter_t, x264_predict_t};
pub use self::pthreadtypes_h::{pthread_cond_t, pthread_mutex_t, pthread_t};
pub use self::quant_h::x264_quant_function_t;
pub use self::rectangle_h::{
    x264_10_cache_mv_func_table, x264_10_cache_mvd_func_table, x264_macroblock_cache_mv,
    x264_macroblock_cache_mvd, x264_macroblock_cache_rect,
};
pub use self::set_h::{
    x264_pps_t, x264_sps_t, C2RustUnnamed_14, C2RustUnnamed_15, C2RustUnnamed_16,
};
pub use self::stdint_h::{intptr_t, uintptr_t};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use self::stdlib_h::abs;
pub use self::struct_mutex_h::__pthread_mutex_s;
use self::tables_h::x264_zero;
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
    C2RustUnnamed_5, X264_ME_DIA, X264_ME_ESA, X264_ME_HEX, X264_ME_TESA, X264_ME_UMH,
};
#[c2rust::src_loc = "38:22"]
static mut subpel_iterations: [[uint8_t; 4]; 12] = [
    [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
    ],
    [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
    ],
];
#[c2rust::src_loc = "53:22"]
static mut mod6m1: [uint8_t; 8] = [
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
#[c2rust::src_loc = "55:21"]
static mut hex2: [[int8_t; 2]; 8] = [
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        -(2 as ::core::ffi::c_int) as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        2 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        2 as ::core::ffi::c_int as int8_t,
    ],
    [
        2 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        -(2 as ::core::ffi::c_int) as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        -(2 as ::core::ffi::c_int) as int8_t,
    ],
    [
        -(2 as ::core::ffi::c_int) as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
];
#[c2rust::src_loc = "56:21"]
static mut square1: [[int8_t; 2]; 9] = [
    [
        0 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        -(1 as ::core::ffi::c_int) as int8_t,
    ],
    [
        0 as ::core::ffi::c_int as int8_t,
        1 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        0 as ::core::ffi::c_int as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        -(1 as ::core::ffi::c_int) as int8_t,
    ],
    [
        -(1 as ::core::ffi::c_int) as int8_t,
        1 as ::core::ffi::c_int as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        -(1 as ::core::ffi::c_int) as int8_t,
    ],
    [
        1 as ::core::ffi::c_int as int8_t,
        1 as ::core::ffi::c_int as int8_t,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "182:1"]
pub unsafe extern "C" fn x264_10_me_search_ref(
    mut h: *mut x264_t,
    mut m: *mut x264_me_t,
    mut mvc: *mut [int16_t; 2],
    mut i_mvc: ::core::ffi::c_int,
    mut p_halfpel_thresh: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let bw: ::core::ffi::c_int = x264_pixel_size[(*m).i_pixel as usize].w as ::core::ffi::c_int;
    let bh: ::core::ffi::c_int = x264_pixel_size[(*m).i_pixel as usize].h as ::core::ffi::c_int;
    let i_pixel: ::core::ffi::c_int = (*m).i_pixel;
    let stride: ::core::ffi::c_int = (*m).i_stride[0 as ::core::ffi::c_int as usize];
    let mut i_me_range: ::core::ffi::c_int = (*h).param.analyse.i_me_range;
    let mut bmx: ::core::ffi::c_int = 0;
    let mut bmy: ::core::ffi::c_int = 0;
    let mut bcost: ::core::ffi::c_int = COST_MAX;
    let mut bpred_cost: ::core::ffi::c_int = COST_MAX;
    let mut omx: ::core::ffi::c_int = 0;
    let mut omy: ::core::ffi::c_int = 0;
    let mut pmx: ::core::ffi::c_int = 0;
    let mut pmy: ::core::ffi::c_int = 0;
    let mut p_fenc: *mut pixel = (*m).p_fenc[0 as ::core::ffi::c_int as usize];
    let mut p_fref_w: *mut pixel = (*m).p_fref_w;
    let mut pix: [pixel; 256] = [0; 256];
    let mut mvc_temp: [[int16_t; 2]; 16] = [[0; 2]; 16];
    let mut costs: [::core::ffi::c_int; 16] = [0; 16];
    let mut mv_x_min: ::core::ffi::c_int = (*h).mb.mv_limit_fpel[0 as ::core::ffi::c_int as usize]
        [0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int;
    let mut mv_y_min: ::core::ffi::c_int = (*h).mb.mv_limit_fpel[0 as ::core::ffi::c_int as usize]
        [1 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int;
    let mut mv_x_max: ::core::ffi::c_int = (*h).mb.mv_limit_fpel[1 as ::core::ffi::c_int as usize]
        [0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int;
    let mut mv_y_max: ::core::ffi::c_int = (*h).mb.mv_limit_fpel[1 as ::core::ffi::c_int as usize]
        [1 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int;
    let mut mv_min: uint32_t = (-mv_x_min as uint32_t) << 16 as ::core::ffi::c_int
        | -mv_y_min as uint32_t & 0x7fff as uint32_t;
    let mut mv_max: uint32_t = (mv_x_max as uint32_t) << 16 as ::core::ffi::c_int
        | mv_y_max as uint32_t & 0x7fff as uint32_t
        | 0x8000 as uint32_t;
    let mut pmv: uint32_t = 0;
    let mut bpred_mv: uint32_t = 0 as uint32_t;
    let mut p_cost_mvx: *const uint16_t = (*m)
        .p_cost_mv
        .offset(-((*m).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize));
    let mut p_cost_mvy: *const uint16_t = (*m)
        .p_cost_mv
        .offset(-((*m).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize));
    if (*h).mb.i_subpel_refine >= 3 as ::core::ffi::c_int {
        let mut bpred_mx: ::core::ffi::c_int = x264_clip3(
            (*m).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            mv_x_min * 4 as ::core::ffi::c_int,
            mv_x_max * 4 as ::core::ffi::c_int,
        );
        let mut bpred_my: ::core::ffi::c_int = x264_clip3(
            (*m).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            mv_y_min * 4 as ::core::ffi::c_int,
            mv_y_max * 4 as ::core::ffi::c_int,
        );
        pmv = pack16to32_mask(bpred_mx, bpred_my);
        pmx = bpred_mx + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
        pmy = bpred_my + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
        let mut stride2: intptr_t = 16 as intptr_t;
        let mut src: *mut pixel = (*h).mc.get_ref.expect("non-null function pointer")(
            pix.as_mut_ptr(),
            &mut stride2,
            (*m).p_fref.as_mut_ptr(),
            stride as intptr_t,
            bpred_mx,
            bpred_my,
            bw,
            bh,
            &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
        );
        bpred_cost = (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
            p_fenc,
            FENC_STRIDE as intptr_t,
            src,
            stride2,
        ) + *p_cost_mvx.offset(bpred_mx as isize) as ::core::ffi::c_int
            + *p_cost_mvy.offset(bpred_my as isize) as ::core::ffi::c_int;
        let mut pmv_cost: ::core::ffi::c_int = bpred_cost;
        if i_mvc > 0 as ::core::ffi::c_int {
            let mut valid_mvcs: ::core::ffi::c_int = x264_predictor_clip(
                mvc_temp
                    .as_mut_ptr()
                    .offset(2 as ::core::ffi::c_int as isize),
                mvc,
                i_mvc,
                (*h).mb.mv_limit_fpel.as_mut_ptr(),
                pmv,
            );
            if valid_mvcs > 0 as ::core::ffi::c_int {
                let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                let mut cost: ::core::ffi::c_int = 0;
                (*((*mvc_temp
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = pmv;
                bpred_cost <<= 4 as ::core::ffi::c_int;
                loop {
                    let mut mx: ::core::ffi::c_int = mvc_temp
                        [(i + 1 as ::core::ffi::c_int) as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                    let mut my: ::core::ffi::c_int = mvc_temp
                        [(i + 1 as ::core::ffi::c_int) as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                    let mut stride2_0: intptr_t = 16 as intptr_t;
                    let mut src_0: *mut pixel = (*h).mc.get_ref.expect("non-null function pointer")(
                        pix.as_mut_ptr(),
                        &mut stride2_0,
                        (*m).p_fref.as_mut_ptr(),
                        stride as intptr_t,
                        mx,
                        my,
                        bw,
                        bh,
                        &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
                    );
                    cost = (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                        p_fenc,
                        FENC_STRIDE as intptr_t,
                        src_0,
                        stride2_0,
                    ) + *p_cost_mvx.offset(mx as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(my as isize) as ::core::ffi::c_int;
                    if (cost << 4 as ::core::ffi::c_int) + i < bpred_cost {
                        bpred_cost = (cost << 4 as ::core::ffi::c_int) + i;
                    }
                    i += 1;
                    if !(i <= valid_mvcs) {
                        break;
                    }
                }
                bpred_mx = mvc_temp
                    [((bpred_cost & 15 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as usize]
                    [0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                bpred_my = mvc_temp
                    [((bpred_cost & 15 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as usize]
                    [1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                bpred_cost >>= 4 as ::core::ffi::c_int;
            }
        }
        bmx = bpred_mx + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
        bmy = bpred_my + 2 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int;
        bpred_mv = pack16to32_mask(bpred_mx, bpred_my);
        if bpred_mv & 0x30003 as uint32_t != 0 {
            let mut cost_0: ::core::ffi::c_int =
                (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                    p_fenc,
                    FENC_STRIDE as intptr_t,
                    &mut *p_fref_w.offset((bmy * stride + bmx) as isize),
                    stride as intptr_t,
                ) + (*p_cost_mvx.offset((bmx * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset((bmy * 4 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int);
            if cost_0 < bcost {
                bcost = cost_0;
                bmx = bmx;
                bmy = bmy;
            }
        } else {
            bcost = bpred_cost;
        }
        if pmv != 0 {
            if bmx | bmy != 0 {
                let mut cost_1: ::core::ffi::c_int = (*h).pixf.fpelcmp[i_pixel as usize]
                    .expect("non-null function pointer")(
                    p_fenc,
                    FENC_STRIDE as intptr_t,
                    &mut *p_fref_w.offset(
                        (0 as ::core::ffi::c_int * stride + 0 as ::core::ffi::c_int) as isize,
                    ),
                    stride as intptr_t,
                ) + (*p_cost_mvx
                    .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy
                        .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int);
                if cost_1 < bcost {
                    bcost = cost_1;
                    bmx = 0 as ::core::ffi::c_int;
                    bmy = 0 as ::core::ffi::c_int;
                }
            }
        } else if pmv_cost < bcost {
            bcost = pmv_cost;
            bmx = 0 as ::core::ffi::c_int;
            bmy = 0 as ::core::ffi::c_int;
        }
    } else {
        pmx = x264_clip3(
            (*m).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int,
            mv_x_min,
            mv_x_max,
        );
        bmx = pmx;
        pmy = x264_clip3(
            (*m).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int,
            mv_y_min,
            mv_y_max,
        );
        bmy = pmy;
        pmv = pack16to32_mask(bmx, bmy);
        bcost = (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
            p_fenc,
            FENC_STRIDE as intptr_t,
            &mut *p_fref_w.offset((bmy * stride + bmx) as isize),
            stride as intptr_t,
        );
        if i_mvc > 0 as ::core::ffi::c_int {
            let mut valid_mvcs_0: ::core::ffi::c_int = x264_predictor_roundclip(
                mvc_temp
                    .as_mut_ptr()
                    .offset(2 as ::core::ffi::c_int as isize),
                mvc,
                i_mvc,
                (*h).mb.mv_limit_fpel.as_mut_ptr(),
                pmv,
            );
            if valid_mvcs_0 > 0 as ::core::ffi::c_int {
                let mut i_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                let mut cost_2: ::core::ffi::c_int = 0;
                (*((*mvc_temp
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = pmv;
                bcost <<= 4 as ::core::ffi::c_int;
                loop {
                    let mut mx_0: ::core::ffi::c_int = mvc_temp
                        [(i_0 + 1 as ::core::ffi::c_int) as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                    let mut my_0: ::core::ffi::c_int = mvc_temp
                        [(i_0 + 1 as ::core::ffi::c_int) as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                    cost_2 = (*h).pixf.fpelcmp[i_pixel as usize]
                        .expect("non-null function pointer")(
                        p_fenc,
                        FENC_STRIDE as intptr_t,
                        &mut *p_fref_w.offset((my_0 * stride + mx_0) as isize),
                        stride as intptr_t,
                    ) + (*p_cost_mvx.offset((mx_0 * 4 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset((my_0 * 4 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int);
                    if (cost_2 << 4 as ::core::ffi::c_int) + i_0 < bcost {
                        bcost = (cost_2 << 4 as ::core::ffi::c_int) + i_0;
                    }
                    i_0 += 1;
                    if !(i_0 <= valid_mvcs_0) {
                        break;
                    }
                }
                bmx = mvc_temp
                    [((bcost & 15 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as usize]
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                bmy = mvc_temp
                    [((bcost & 15 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as usize]
                    [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                bcost >>= 4 as ::core::ffi::c_int;
            }
        }
        if pmv != 0 {
            let mut cost_3: ::core::ffi::c_int = (*h).pixf.fpelcmp[i_pixel as usize]
                .expect("non-null function pointer")(
                p_fenc,
                FENC_STRIDE as intptr_t,
                &mut *p_fref_w
                    .offset((0 as ::core::ffi::c_int * stride + 0 as ::core::ffi::c_int) as isize),
                stride as intptr_t,
            ) + (*p_cost_mvx
                .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy.offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int);
            if cost_3 < bcost {
                bcost = cost_3;
                bmx = 0 as ::core::ffi::c_int;
                bmy = 0 as ::core::ffi::c_int;
            }
        }
    }
    match (*h).mb.i_me_method {
        X264_ME_DIA => {
            bcost <<= 4 as ::core::ffi::c_int;
            let mut i_1: ::core::ffi::c_int = i_me_range;
            loop {
                let mut pix_base: *mut pixel = p_fref_w
                    .offset(bmx as isize)
                    .offset((bmy * stride) as isize);
                (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                    p_fenc,
                    pix_base
                        .offset(0 as ::core::ffi::c_int as isize)
                        .offset((-(1 as ::core::ffi::c_int) * stride) as isize),
                    pix_base
                        .offset(0 as ::core::ffi::c_int as isize)
                        .offset((1 as ::core::ffi::c_int * stride) as isize),
                    pix_base
                        .offset(-(1 as ::core::ffi::c_int) as isize)
                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                    pix_base
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                    stride as intptr_t,
                    costs.as_mut_ptr(),
                );
                costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx
                    .offset(((bmx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset(
                        ((bmy + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int;
                costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx
                    .offset(((bmx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset(
                        ((bmy + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int;
                costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx
                    .offset(((bmx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset(
                        ((bmy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int;
                costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx
                    .offset(((bmx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset(
                        ((bmy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int;
                if ((costs[0 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int)
                    < bcost
                {
                    bcost = (costs[0 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int;
                }
                if ((costs[1 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 3 as ::core::ffi::c_int)
                    < bcost
                {
                    bcost = (costs[1 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                        + 3 as ::core::ffi::c_int;
                }
                if ((costs[2 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 4 as ::core::ffi::c_int)
                    < bcost
                {
                    bcost = (costs[2 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                        + 4 as ::core::ffi::c_int;
                }
                if ((costs[3 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 12 as ::core::ffi::c_int)
                    < bcost
                {
                    bcost = (costs[3 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                        + 12 as ::core::ffi::c_int;
                }
                if bcost & 15 as ::core::ffi::c_int == 0 {
                    break;
                }
                bmx -= (((bcost as uint32_t) << 28 as ::core::ffi::c_int) as int32_t
                    >> 30 as ::core::ffi::c_int) as ::core::ffi::c_int;
                bmy -= (((bcost as uint32_t) << 30 as ::core::ffi::c_int) as int32_t
                    >> 30 as ::core::ffi::c_int) as ::core::ffi::c_int;
                bcost &= !(15 as ::core::ffi::c_int);
                i_1 -= 1;
                if !(i_1 != 0
                    && (((bmx as uint32_t) << 16 as ::core::ffi::c_int
                        | bmy as uint32_t & 0x7fff as uint32_t)
                        .wrapping_add(mv_min)
                        | mv_max.wrapping_sub(
                            (bmx as uint32_t) << 16 as ::core::ffi::c_int
                                | bmy as uint32_t & 0x7fff as uint32_t,
                        ))
                        & 0x80004000 as uint32_t
                        == 0)
                {
                    break;
                }
            }
            bcost >>= 4 as ::core::ffi::c_int;
            current_block = 14127502640287082657;
        }
        X264_ME_HEX => {
            current_block = 14690580863265192683;
        }
        X264_ME_UMH => {
            static mut pixel_size_shift: [uint8_t; 7] = [
                0 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
                2 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
                4 as ::core::ffi::c_int as uint8_t,
            ];
            let mut ucost1: ::core::ffi::c_int = 0;
            let mut ucost2: ::core::ffi::c_int = 0;
            let mut cross_start: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            ucost1 = bcost;
            omx = pmx;
            omy = pmy;
            let mut pix_base_5: *mut pixel = p_fref_w
                .offset(omx as isize)
                .offset((omy * stride) as isize);
            (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                p_fenc,
                pix_base_5
                    .offset(0 as ::core::ffi::c_int as isize)
                    .offset((-(1 as ::core::ffi::c_int) * stride) as isize),
                pix_base_5
                    .offset(0 as ::core::ffi::c_int as isize)
                    .offset((1 as ::core::ffi::c_int * stride) as isize),
                pix_base_5
                    .offset(-(1 as ::core::ffi::c_int) as isize)
                    .offset((0 as ::core::ffi::c_int * stride) as isize),
                pix_base_5
                    .offset(1 as ::core::ffi::c_int as isize)
                    .offset((0 as ::core::ffi::c_int * stride) as isize),
                stride as intptr_t,
                costs.as_mut_ptr(),
            );
            costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((omy + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((omy + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((omx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((omx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            if costs[0 as ::core::ffi::c_int as usize] < bcost {
                bcost = costs[0 as ::core::ffi::c_int as usize];
                bmx = omx + 0 as ::core::ffi::c_int;
                bmy = omy + -(1 as ::core::ffi::c_int);
            }
            if costs[1 as ::core::ffi::c_int as usize] < bcost {
                bcost = costs[1 as ::core::ffi::c_int as usize];
                bmx = omx + 0 as ::core::ffi::c_int;
                bmy = omy + 1 as ::core::ffi::c_int;
            }
            if costs[2 as ::core::ffi::c_int as usize] < bcost {
                bcost = costs[2 as ::core::ffi::c_int as usize];
                bmx = omx + -(1 as ::core::ffi::c_int);
                bmy = omy + 0 as ::core::ffi::c_int;
            }
            if costs[3 as ::core::ffi::c_int as usize] < bcost {
                bcost = costs[3 as ::core::ffi::c_int as usize];
                bmx = omx + 1 as ::core::ffi::c_int;
                bmy = omy + 0 as ::core::ffi::c_int;
            }
            if pmx | pmy != 0 {
                omx = 0 as ::core::ffi::c_int;
                omy = 0 as ::core::ffi::c_int;
                let mut pix_base_6: *mut pixel = p_fref_w
                    .offset(omx as isize)
                    .offset((omy * stride) as isize);
                (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                    p_fenc,
                    pix_base_6
                        .offset(0 as ::core::ffi::c_int as isize)
                        .offset((-(1 as ::core::ffi::c_int) * stride) as isize),
                    pix_base_6
                        .offset(0 as ::core::ffi::c_int as isize)
                        .offset((1 as ::core::ffi::c_int * stride) as isize),
                    pix_base_6
                        .offset(-(1 as ::core::ffi::c_int) as isize)
                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                    pix_base_6
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                    stride as intptr_t,
                    costs.as_mut_ptr(),
                );
                costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx
                    .offset(((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset(
                        ((omy + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int;
                costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx
                    .offset(((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset(
                        ((omy + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int;
                costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx
                    .offset(((omx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset(
                        ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int;
                costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx
                    .offset(((omx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset(
                        ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int;
                if costs[0 as ::core::ffi::c_int as usize] < bcost {
                    bcost = costs[0 as ::core::ffi::c_int as usize];
                    bmx = omx + 0 as ::core::ffi::c_int;
                    bmy = omy + -(1 as ::core::ffi::c_int);
                }
                if costs[1 as ::core::ffi::c_int as usize] < bcost {
                    bcost = costs[1 as ::core::ffi::c_int as usize];
                    bmx = omx + 0 as ::core::ffi::c_int;
                    bmy = omy + 1 as ::core::ffi::c_int;
                }
                if costs[2 as ::core::ffi::c_int as usize] < bcost {
                    bcost = costs[2 as ::core::ffi::c_int as usize];
                    bmx = omx + -(1 as ::core::ffi::c_int);
                    bmy = omy + 0 as ::core::ffi::c_int;
                }
                if costs[3 as ::core::ffi::c_int as usize] < bcost {
                    bcost = costs[3 as ::core::ffi::c_int as usize];
                    bmx = omx + 1 as ::core::ffi::c_int;
                    bmy = omy + 0 as ::core::ffi::c_int;
                }
            }
            if i_pixel == PIXEL_4x4 as ::core::ffi::c_int {
                current_block = 14690580863265192683;
            } else {
                ucost2 = bcost;
                if bmx | bmy != 0 && bmx - pmx | bmy - pmy != 0 {
                    omx = bmx;
                    omy = bmy;
                    let mut pix_base_7: *mut pixel = p_fref_w
                        .offset(omx as isize)
                        .offset((omy * stride) as isize);
                    (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                        p_fenc,
                        pix_base_7
                            .offset(0 as ::core::ffi::c_int as isize)
                            .offset((-(1 as ::core::ffi::c_int) * stride) as isize),
                        pix_base_7
                            .offset(0 as ::core::ffi::c_int as isize)
                            .offset((1 as ::core::ffi::c_int * stride) as isize),
                        pix_base_7
                            .offset(-(1 as ::core::ffi::c_int) as isize)
                            .offset((0 as ::core::ffi::c_int * stride) as isize),
                        pix_base_7
                            .offset(1 as ::core::ffi::c_int as isize)
                            .offset((0 as ::core::ffi::c_int * stride) as isize),
                        stride as intptr_t,
                        costs.as_mut_ptr(),
                    );
                    costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    if costs[0 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[0 as ::core::ffi::c_int as usize];
                        bmx = omx + 0 as ::core::ffi::c_int;
                        bmy = omy + -(1 as ::core::ffi::c_int);
                    }
                    if costs[1 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[1 as ::core::ffi::c_int as usize];
                        bmx = omx + 0 as ::core::ffi::c_int;
                        bmy = omy + 1 as ::core::ffi::c_int;
                    }
                    if costs[2 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[2 as ::core::ffi::c_int as usize];
                        bmx = omx + -(1 as ::core::ffi::c_int);
                        bmy = omy + 0 as ::core::ffi::c_int;
                    }
                    if costs[3 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[3 as ::core::ffi::c_int as usize];
                        bmx = omx + 1 as ::core::ffi::c_int;
                        bmy = omy + 0 as ::core::ffi::c_int;
                    }
                }
                if bcost == ucost2 {
                    cross_start = 3 as ::core::ffi::c_int;
                }
                omx = bmx;
                omy = bmy;
                if bcost == ucost2
                    && bcost
                        < 2000 as ::core::ffi::c_int
                            >> pixel_size_shift[i_pixel as usize] as ::core::ffi::c_int
                {
                    let mut pix_base_8: *mut pixel = p_fref_w
                        .offset(omx as isize)
                        .offset((omy * stride) as isize);
                    (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                        p_fenc,
                        pix_base_8
                            .offset(0 as ::core::ffi::c_int as isize)
                            .offset((-(2 as ::core::ffi::c_int) * stride) as isize),
                        pix_base_8
                            .offset(-(1 as ::core::ffi::c_int) as isize)
                            .offset((-(1 as ::core::ffi::c_int) * stride) as isize),
                        pix_base_8
                            .offset(1 as ::core::ffi::c_int as isize)
                            .offset((-(1 as ::core::ffi::c_int) * stride) as isize),
                        pix_base_8
                            .offset(-(2 as ::core::ffi::c_int) as isize)
                            .offset((0 as ::core::ffi::c_int * stride) as isize),
                        stride as intptr_t,
                        costs.as_mut_ptr(),
                    );
                    costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    if costs[0 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[0 as ::core::ffi::c_int as usize];
                        bmx = omx + 0 as ::core::ffi::c_int;
                        bmy = omy + -(2 as ::core::ffi::c_int);
                    }
                    if costs[1 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[1 as ::core::ffi::c_int as usize];
                        bmx = omx + -(1 as ::core::ffi::c_int);
                        bmy = omy + -(1 as ::core::ffi::c_int);
                    }
                    if costs[2 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[2 as ::core::ffi::c_int as usize];
                        bmx = omx + 1 as ::core::ffi::c_int;
                        bmy = omy + -(1 as ::core::ffi::c_int);
                    }
                    if costs[3 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[3 as ::core::ffi::c_int as usize];
                        bmx = omx + -(2 as ::core::ffi::c_int);
                        bmy = omy + 0 as ::core::ffi::c_int;
                    }
                    let mut pix_base_9: *mut pixel = p_fref_w
                        .offset(omx as isize)
                        .offset((omy * stride) as isize);
                    (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                        p_fenc,
                        pix_base_9
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset((0 as ::core::ffi::c_int * stride) as isize),
                        pix_base_9
                            .offset(-(1 as ::core::ffi::c_int) as isize)
                            .offset((1 as ::core::ffi::c_int * stride) as isize),
                        pix_base_9
                            .offset(1 as ::core::ffi::c_int as isize)
                            .offset((1 as ::core::ffi::c_int * stride) as isize),
                        pix_base_9
                            .offset(0 as ::core::ffi::c_int as isize)
                            .offset((2 as ::core::ffi::c_int * stride) as isize),
                        stride as intptr_t,
                        costs.as_mut_ptr(),
                    );
                    costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((omy + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    if costs[0 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[0 as ::core::ffi::c_int as usize];
                        bmx = omx + 2 as ::core::ffi::c_int;
                        bmy = omy + 0 as ::core::ffi::c_int;
                    }
                    if costs[1 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[1 as ::core::ffi::c_int as usize];
                        bmx = omx + -(1 as ::core::ffi::c_int);
                        bmy = omy + 1 as ::core::ffi::c_int;
                    }
                    if costs[2 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[2 as ::core::ffi::c_int as usize];
                        bmx = omx + 1 as ::core::ffi::c_int;
                        bmy = omy + 1 as ::core::ffi::c_int;
                    }
                    if costs[3 as ::core::ffi::c_int as usize] < bcost {
                        bcost = costs[3 as ::core::ffi::c_int as usize];
                        bmx = omx + 0 as ::core::ffi::c_int;
                        bmy = omy + 2 as ::core::ffi::c_int;
                    }
                    if bcost == ucost1
                        && bcost
                            < 500 as ::core::ffi::c_int
                                >> pixel_size_shift[i_pixel as usize] as ::core::ffi::c_int
                    {
                        current_block = 14127502640287082657;
                    } else if bcost == ucost2 {
                        let mut range: ::core::ffi::c_int =
                            i_me_range >> 1 as ::core::ffi::c_int | 1 as ::core::ffi::c_int;
                        let mut i_3: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
                        if range
                            <= (if mv_x_max - omx < omx - mv_x_min {
                                mv_x_max - omx
                            } else {
                                omx - mv_x_min
                            })
                        {
                            while i_3 < range - 2 as ::core::ffi::c_int {
                                let mut pix_base_10: *mut pixel = p_fref_w
                                    .offset(omx as isize)
                                    .offset((omy * stride) as isize);
                                (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_10
                                        .offset(i_3 as isize)
                                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                                    pix_base_10
                                        .offset(-i_3 as isize)
                                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                                    pix_base_10
                                        .offset((i_3 + 2 as ::core::ffi::c_int) as isize)
                                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                                    pix_base_10
                                        .offset((-i_3 - 2 as ::core::ffi::c_int) as isize)
                                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                                    stride as intptr_t,
                                    costs.as_mut_ptr(),
                                );
                                costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + i_3) * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + -i_3) * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + (i_3 + 2 as ::core::ffi::c_int))
                                        * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + (-i_3 - 2 as ::core::ffi::c_int))
                                        * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                if costs[0 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[0 as ::core::ffi::c_int as usize];
                                    bmx = omx + i_3;
                                    bmy = omy + 0 as ::core::ffi::c_int;
                                }
                                if costs[1 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[1 as ::core::ffi::c_int as usize];
                                    bmx = omx + -i_3;
                                    bmy = omy + 0 as ::core::ffi::c_int;
                                }
                                if costs[2 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[2 as ::core::ffi::c_int as usize];
                                    bmx = omx + (i_3 + 2 as ::core::ffi::c_int);
                                    bmy = omy + 0 as ::core::ffi::c_int;
                                }
                                if costs[3 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[3 as ::core::ffi::c_int as usize];
                                    bmx = omx + (-i_3 - 2 as ::core::ffi::c_int);
                                    bmy = omy + 0 as ::core::ffi::c_int;
                                }
                                i_3 += 4 as ::core::ffi::c_int;
                            }
                        }
                        while i_3 < range {
                            if omx + i_3 <= mv_x_max {
                                let mut cost_4: ::core::ffi::c_int = (*h).pixf.fpelcmp
                                    [i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    FENC_STRIDE as intptr_t,
                                    &mut *p_fref_w.offset((omy * stride + (omx + i_3)) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset(((omx + i_3) * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset((omy * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int);
                                if cost_4 < bcost {
                                    bcost = cost_4;
                                    bmx = omx + i_3;
                                    bmy = omy;
                                }
                            }
                            if omx - i_3 >= mv_x_min {
                                let mut cost_5: ::core::ffi::c_int = (*h).pixf.fpelcmp
                                    [i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    FENC_STRIDE as intptr_t,
                                    &mut *p_fref_w.offset((omy * stride + (omx - i_3)) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset(((omx - i_3) * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset((omy * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int);
                                if cost_5 < bcost {
                                    bcost = cost_5;
                                    bmx = omx - i_3;
                                    bmy = omy;
                                }
                            }
                            i_3 += 2 as ::core::ffi::c_int;
                        }
                        i_3 = 3 as ::core::ffi::c_int;
                        if range
                            <= (if mv_y_max - omy < omy - mv_y_min {
                                mv_y_max - omy
                            } else {
                                omy - mv_y_min
                            })
                        {
                            while i_3 < range - 2 as ::core::ffi::c_int {
                                let mut pix_base_11: *mut pixel = p_fref_w
                                    .offset(omx as isize)
                                    .offset((omy * stride) as isize);
                                (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_11
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        .offset((i_3 * stride) as isize),
                                    pix_base_11
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        .offset((-i_3 * stride) as isize),
                                    pix_base_11.offset(0 as ::core::ffi::c_int as isize).offset(
                                        ((i_3 + 2 as ::core::ffi::c_int) * stride) as isize,
                                    ),
                                    pix_base_11.offset(0 as ::core::ffi::c_int as isize).offset(
                                        ((-i_3 - 2 as ::core::ffi::c_int) * stride) as isize,
                                    ),
                                    stride as intptr_t,
                                    costs.as_mut_ptr(),
                                );
                                costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy
                                        .offset(((omy + i_3) * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int;
                                costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy
                                        .offset(((omy + -i_3) * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int;
                                costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + (i_3 + 2 as ::core::ffi::c_int))
                                            * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + (-i_3 - 2 as ::core::ffi::c_int))
                                            * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                if costs[0 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[0 as ::core::ffi::c_int as usize];
                                    bmx = omx + 0 as ::core::ffi::c_int;
                                    bmy = omy + i_3;
                                }
                                if costs[1 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[1 as ::core::ffi::c_int as usize];
                                    bmx = omx + 0 as ::core::ffi::c_int;
                                    bmy = omy + -i_3;
                                }
                                if costs[2 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[2 as ::core::ffi::c_int as usize];
                                    bmx = omx + 0 as ::core::ffi::c_int;
                                    bmy = omy + (i_3 + 2 as ::core::ffi::c_int);
                                }
                                if costs[3 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[3 as ::core::ffi::c_int as usize];
                                    bmx = omx + 0 as ::core::ffi::c_int;
                                    bmy = omy + (-i_3 - 2 as ::core::ffi::c_int);
                                }
                                i_3 += 4 as ::core::ffi::c_int;
                            }
                        }
                        while i_3 < range {
                            if omy + i_3 <= mv_y_max {
                                let mut cost_6: ::core::ffi::c_int = (*h).pixf.fpelcmp
                                    [i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    FENC_STRIDE as intptr_t,
                                    &mut *p_fref_w.offset(((omy + i_3) * stride + omx) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset((omx * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy
                                        .offset(((omy + i_3) * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int);
                                if cost_6 < bcost {
                                    bcost = cost_6;
                                    bmx = omx;
                                    bmy = omy + i_3;
                                }
                            }
                            if omy - i_3 >= mv_y_min {
                                let mut cost_7: ::core::ffi::c_int = (*h).pixf.fpelcmp
                                    [i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    FENC_STRIDE as intptr_t,
                                    &mut *p_fref_w.offset(((omy - i_3) * stride + omx) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset((omx * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy
                                        .offset(((omy - i_3) * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int);
                                if cost_7 < bcost {
                                    bcost = cost_7;
                                    bmx = omx;
                                    bmy = omy - i_3;
                                }
                            }
                            i_3 += 2 as ::core::ffi::c_int;
                        }
                        let mut pix_base_12: *mut pixel = p_fref_w
                            .offset(omx as isize)
                            .offset((omy * stride) as isize);
                        (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                            p_fenc,
                            pix_base_12
                                .offset(-(1 as ::core::ffi::c_int) as isize)
                                .offset((-(2 as ::core::ffi::c_int) * stride) as isize),
                            pix_base_12
                                .offset(1 as ::core::ffi::c_int as isize)
                                .offset((-(2 as ::core::ffi::c_int) * stride) as isize),
                            pix_base_12
                                .offset(-(2 as ::core::ffi::c_int) as isize)
                                .offset((-(1 as ::core::ffi::c_int) * stride) as isize),
                            pix_base_12
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset((-(1 as ::core::ffi::c_int) * stride) as isize),
                            stride as intptr_t,
                            costs.as_mut_ptr(),
                        );
                        costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        if costs[0 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[0 as ::core::ffi::c_int as usize];
                            bmx = omx + -(1 as ::core::ffi::c_int);
                            bmy = omy + -(2 as ::core::ffi::c_int);
                        }
                        if costs[1 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[1 as ::core::ffi::c_int as usize];
                            bmx = omx + 1 as ::core::ffi::c_int;
                            bmy = omy + -(2 as ::core::ffi::c_int);
                        }
                        if costs[2 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[2 as ::core::ffi::c_int as usize];
                            bmx = omx + -(2 as ::core::ffi::c_int);
                            bmy = omy + -(1 as ::core::ffi::c_int);
                        }
                        if costs[3 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[3 as ::core::ffi::c_int as usize];
                            bmx = omx + 2 as ::core::ffi::c_int;
                            bmy = omy + -(1 as ::core::ffi::c_int);
                        }
                        let mut pix_base_13: *mut pixel = p_fref_w
                            .offset(omx as isize)
                            .offset((omy * stride) as isize);
                        (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                            p_fenc,
                            pix_base_13
                                .offset(-(2 as ::core::ffi::c_int) as isize)
                                .offset((1 as ::core::ffi::c_int * stride) as isize),
                            pix_base_13
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset((1 as ::core::ffi::c_int * stride) as isize),
                            pix_base_13
                                .offset(-(1 as ::core::ffi::c_int) as isize)
                                .offset((2 as ::core::ffi::c_int * stride) as isize),
                            pix_base_13
                                .offset(1 as ::core::ffi::c_int as isize)
                                .offset((2 as ::core::ffi::c_int * stride) as isize),
                            stride as intptr_t,
                            costs.as_mut_ptr(),
                        );
                        costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        if costs[0 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[0 as ::core::ffi::c_int as usize];
                            bmx = omx + -(2 as ::core::ffi::c_int);
                            bmy = omy + 1 as ::core::ffi::c_int;
                        }
                        if costs[1 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[1 as ::core::ffi::c_int as usize];
                            bmx = omx + 2 as ::core::ffi::c_int;
                            bmy = omy + 1 as ::core::ffi::c_int;
                        }
                        if costs[2 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[2 as ::core::ffi::c_int as usize];
                            bmx = omx + -(1 as ::core::ffi::c_int);
                            bmy = omy + 2 as ::core::ffi::c_int;
                        }
                        if costs[3 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[3 as ::core::ffi::c_int as usize];
                            bmx = omx + 1 as ::core::ffi::c_int;
                            bmy = omy + 2 as ::core::ffi::c_int;
                        }
                        if bcost == ucost2 {
                            current_block = 14127502640287082657;
                        } else {
                            cross_start = range + 2 as ::core::ffi::c_int;
                            current_block = 3988218931164863998;
                        }
                    } else {
                        current_block = 3988218931164863998;
                    }
                } else {
                    current_block = 3988218931164863998;
                }
                match current_block {
                    14127502640287082657 => {}
                    _ => {
                        if i_mvc != 0 {
                            static mut range_mul: [[uint8_t; 4]; 4] = [
                                [
                                    3 as ::core::ffi::c_int as uint8_t,
                                    3 as ::core::ffi::c_int as uint8_t,
                                    4 as ::core::ffi::c_int as uint8_t,
                                    4 as ::core::ffi::c_int as uint8_t,
                                ],
                                [
                                    3 as ::core::ffi::c_int as uint8_t,
                                    4 as ::core::ffi::c_int as uint8_t,
                                    4 as ::core::ffi::c_int as uint8_t,
                                    4 as ::core::ffi::c_int as uint8_t,
                                ],
                                [
                                    4 as ::core::ffi::c_int as uint8_t,
                                    4 as ::core::ffi::c_int as uint8_t,
                                    4 as ::core::ffi::c_int as uint8_t,
                                    5 as ::core::ffi::c_int as uint8_t,
                                ],
                                [
                                    4 as ::core::ffi::c_int as uint8_t,
                                    4 as ::core::ffi::c_int as uint8_t,
                                    5 as ::core::ffi::c_int as uint8_t,
                                    6 as ::core::ffi::c_int as uint8_t,
                                ],
                            ];
                            let mut mvd: ::core::ffi::c_int = 0;
                            let mut sad_ctx: ::core::ffi::c_int = 0;
                            let mut mvd_ctx: ::core::ffi::c_int = 0;
                            let mut denom: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                            if i_mvc == 1 as ::core::ffi::c_int {
                                if i_pixel == PIXEL_16x16 as ::core::ffi::c_int {
                                    mvd = 25 as ::core::ffi::c_int;
                                } else {
                                    mvd = abs((*m).mvp[0 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        - (*mvc.offset(0 as ::core::ffi::c_int as isize))
                                            [0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int)
                                        + abs((*m).mvp[1 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int
                                            - (*mvc.offset(0 as ::core::ffi::c_int as isize))
                                                [1 as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int);
                                }
                            } else {
                                denom = i_mvc - 1 as ::core::ffi::c_int;
                                mvd = 0 as ::core::ffi::c_int;
                                if i_pixel != PIXEL_16x16 as ::core::ffi::c_int {
                                    mvd = abs((*m).mvp[0 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                        - (*mvc.offset(0 as ::core::ffi::c_int as isize))
                                            [0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int)
                                        + abs((*m).mvp[1 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int
                                            - (*mvc.offset(0 as ::core::ffi::c_int as isize))
                                                [1 as ::core::ffi::c_int as usize]
                                                as ::core::ffi::c_int);
                                    denom += 1;
                                }
                                mvd += x264_predictor_difference(mvc, i_mvc as intptr_t);
                            }
                            sad_ctx = if bcost
                                < 1000 as ::core::ffi::c_int
                                    >> pixel_size_shift[i_pixel as usize] as ::core::ffi::c_int
                            {
                                0 as ::core::ffi::c_int
                            } else if bcost
                                < 2000 as ::core::ffi::c_int
                                    >> pixel_size_shift[i_pixel as usize] as ::core::ffi::c_int
                            {
                                1 as ::core::ffi::c_int
                            } else if bcost
                                < 4000 as ::core::ffi::c_int
                                    >> pixel_size_shift[i_pixel as usize] as ::core::ffi::c_int
                            {
                                2 as ::core::ffi::c_int
                            } else {
                                3 as ::core::ffi::c_int
                            };
                            mvd_ctx = if mvd < 10 as ::core::ffi::c_int * denom {
                                0 as ::core::ffi::c_int
                            } else if mvd < 20 as ::core::ffi::c_int * denom {
                                1 as ::core::ffi::c_int
                            } else if mvd < 40 as ::core::ffi::c_int * denom {
                                2 as ::core::ffi::c_int
                            } else {
                                3 as ::core::ffi::c_int
                            };
                            i_me_range = i_me_range
                                * range_mul[mvd_ctx as usize][sad_ctx as usize]
                                    as ::core::ffi::c_int
                                >> 2 as ::core::ffi::c_int;
                        }
                        let mut i_4: ::core::ffi::c_int = cross_start;
                        if i_me_range
                            <= (if mv_x_max - omx < omx - mv_x_min {
                                mv_x_max - omx
                            } else {
                                omx - mv_x_min
                            })
                        {
                            while i_4 < i_me_range - 2 as ::core::ffi::c_int {
                                let mut pix_base_14: *mut pixel = p_fref_w
                                    .offset(omx as isize)
                                    .offset((omy * stride) as isize);
                                (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_14
                                        .offset(i_4 as isize)
                                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                                    pix_base_14
                                        .offset(-i_4 as isize)
                                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                                    pix_base_14
                                        .offset((i_4 + 2 as ::core::ffi::c_int) as isize)
                                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                                    pix_base_14
                                        .offset((-i_4 - 2 as ::core::ffi::c_int) as isize)
                                        .offset((0 as ::core::ffi::c_int * stride) as isize),
                                    stride as intptr_t,
                                    costs.as_mut_ptr(),
                                );
                                costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + i_4) * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx
                                    .offset(((omx + -i_4) * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + (i_4 + 2 as ::core::ffi::c_int))
                                        * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + (-i_4 - 2 as ::core::ffi::c_int))
                                        * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                if costs[0 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[0 as ::core::ffi::c_int as usize];
                                    bmx = omx + i_4;
                                    bmy = omy + 0 as ::core::ffi::c_int;
                                }
                                if costs[1 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[1 as ::core::ffi::c_int as usize];
                                    bmx = omx + -i_4;
                                    bmy = omy + 0 as ::core::ffi::c_int;
                                }
                                if costs[2 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[2 as ::core::ffi::c_int as usize];
                                    bmx = omx + (i_4 + 2 as ::core::ffi::c_int);
                                    bmy = omy + 0 as ::core::ffi::c_int;
                                }
                                if costs[3 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[3 as ::core::ffi::c_int as usize];
                                    bmx = omx + (-i_4 - 2 as ::core::ffi::c_int);
                                    bmy = omy + 0 as ::core::ffi::c_int;
                                }
                                i_4 += 4 as ::core::ffi::c_int;
                            }
                        }
                        while i_4 < i_me_range {
                            if omx + i_4 <= mv_x_max {
                                let mut cost_8: ::core::ffi::c_int = (*h).pixf.fpelcmp
                                    [i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    FENC_STRIDE as intptr_t,
                                    &mut *p_fref_w.offset((omy * stride + (omx + i_4)) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset(((omx + i_4) * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset((omy * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int);
                                if cost_8 < bcost {
                                    bcost = cost_8;
                                    bmx = omx + i_4;
                                    bmy = omy;
                                }
                            }
                            if omx - i_4 >= mv_x_min {
                                let mut cost_9: ::core::ffi::c_int = (*h).pixf.fpelcmp
                                    [i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    FENC_STRIDE as intptr_t,
                                    &mut *p_fref_w.offset((omy * stride + (omx - i_4)) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset(((omx - i_4) * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset((omy * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int);
                                if cost_9 < bcost {
                                    bcost = cost_9;
                                    bmx = omx - i_4;
                                    bmy = omy;
                                }
                            }
                            i_4 += 2 as ::core::ffi::c_int;
                        }
                        i_4 = cross_start;
                        if i_me_range >> 1 as ::core::ffi::c_int
                            <= (if mv_y_max - omy < omy - mv_y_min {
                                mv_y_max - omy
                            } else {
                                omy - mv_y_min
                            })
                        {
                            while i_4
                                < (i_me_range >> 1 as ::core::ffi::c_int) - 2 as ::core::ffi::c_int
                            {
                                let mut pix_base_15: *mut pixel = p_fref_w
                                    .offset(omx as isize)
                                    .offset((omy * stride) as isize);
                                (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_15
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        .offset((i_4 * stride) as isize),
                                    pix_base_15
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        .offset((-i_4 * stride) as isize),
                                    pix_base_15.offset(0 as ::core::ffi::c_int as isize).offset(
                                        ((i_4 + 2 as ::core::ffi::c_int) * stride) as isize,
                                    ),
                                    pix_base_15.offset(0 as ::core::ffi::c_int as isize).offset(
                                        ((-i_4 - 2 as ::core::ffi::c_int) * stride) as isize,
                                    ),
                                    stride as intptr_t,
                                    costs.as_mut_ptr(),
                                );
                                costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy
                                        .offset(((omy + i_4) * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int;
                                costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy
                                        .offset(((omy + -i_4) * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int;
                                costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + (i_4 + 2 as ::core::ffi::c_int))
                                            * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                    ((omx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy.offset(
                                        ((omy + (-i_4 - 2 as ::core::ffi::c_int))
                                            * 4 as ::core::ffi::c_int)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                if costs[0 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[0 as ::core::ffi::c_int as usize];
                                    bmx = omx + 0 as ::core::ffi::c_int;
                                    bmy = omy + i_4;
                                }
                                if costs[1 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[1 as ::core::ffi::c_int as usize];
                                    bmx = omx + 0 as ::core::ffi::c_int;
                                    bmy = omy + -i_4;
                                }
                                if costs[2 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[2 as ::core::ffi::c_int as usize];
                                    bmx = omx + 0 as ::core::ffi::c_int;
                                    bmy = omy + (i_4 + 2 as ::core::ffi::c_int);
                                }
                                if costs[3 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[3 as ::core::ffi::c_int as usize];
                                    bmx = omx + 0 as ::core::ffi::c_int;
                                    bmy = omy + (-i_4 - 2 as ::core::ffi::c_int);
                                }
                                i_4 += 4 as ::core::ffi::c_int;
                            }
                        }
                        while i_4 < i_me_range >> 1 as ::core::ffi::c_int {
                            if omy + i_4 <= mv_y_max {
                                let mut cost_10: ::core::ffi::c_int = (*h).pixf.fpelcmp
                                    [i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    FENC_STRIDE as intptr_t,
                                    &mut *p_fref_w.offset(((omy + i_4) * stride + omx) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset((omx * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy
                                        .offset(((omy + i_4) * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int);
                                if cost_10 < bcost {
                                    bcost = cost_10;
                                    bmx = omx;
                                    bmy = omy + i_4;
                                }
                            }
                            if omy - i_4 >= mv_y_min {
                                let mut cost_11: ::core::ffi::c_int = (*h).pixf.fpelcmp
                                    [i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    FENC_STRIDE as intptr_t,
                                    &mut *p_fref_w.offset(((omy - i_4) * stride + omx) as isize),
                                    stride as intptr_t,
                                ) + (*p_cost_mvx
                                    .offset((omx * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int
                                    + *p_cost_mvy
                                        .offset(((omy - i_4) * 4 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int);
                                if cost_11 < bcost {
                                    bcost = cost_11;
                                    bmx = omx;
                                    bmy = omy - i_4;
                                }
                            }
                            i_4 += 2 as ::core::ffi::c_int;
                        }
                        let mut pix_base_16: *mut pixel = p_fref_w
                            .offset(omx as isize)
                            .offset((omy * stride) as isize);
                        (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                            p_fenc,
                            pix_base_16
                                .offset(-(2 as ::core::ffi::c_int) as isize)
                                .offset((-(2 as ::core::ffi::c_int) * stride) as isize),
                            pix_base_16
                                .offset(-(2 as ::core::ffi::c_int) as isize)
                                .offset((2 as ::core::ffi::c_int * stride) as isize),
                            pix_base_16
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset((-(2 as ::core::ffi::c_int) * stride) as isize),
                            pix_base_16
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset((2 as ::core::ffi::c_int * stride) as isize),
                            stride as intptr_t,
                            costs.as_mut_ptr(),
                        );
                        costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                            ((omx + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
                        )
                            as ::core::ffi::c_int
                            + *p_cost_mvy.offset(
                                ((omy + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        if costs[0 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[0 as ::core::ffi::c_int as usize];
                            bmx = omx + -(2 as ::core::ffi::c_int);
                            bmy = omy + -(2 as ::core::ffi::c_int);
                        }
                        if costs[1 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[1 as ::core::ffi::c_int as usize];
                            bmx = omx + -(2 as ::core::ffi::c_int);
                            bmy = omy + 2 as ::core::ffi::c_int;
                        }
                        if costs[2 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[2 as ::core::ffi::c_int as usize];
                            bmx = omx + 2 as ::core::ffi::c_int;
                            bmy = omy + -(2 as ::core::ffi::c_int);
                        }
                        if costs[3 as ::core::ffi::c_int as usize] < bcost {
                            bcost = costs[3 as ::core::ffi::c_int as usize];
                            bmx = omx + 2 as ::core::ffi::c_int;
                            bmy = omy + 2 as ::core::ffi::c_int;
                        }
                        omx = bmx;
                        omy = bmy;
                        let mut p_cost_omvx: *const uint16_t =
                            p_cost_mvx.offset((omx * 4 as ::core::ffi::c_int) as isize);
                        let mut p_cost_omvy: *const uint16_t =
                            p_cost_mvy.offset((omy * 4 as ::core::ffi::c_int) as isize);
                        let mut i_5: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        loop {
                            static mut hex4: [[int8_t; 2]; 16] = [
                                [
                                    0 as ::core::ffi::c_int as int8_t,
                                    -(4 as ::core::ffi::c_int) as int8_t,
                                ],
                                [
                                    0 as ::core::ffi::c_int as int8_t,
                                    4 as ::core::ffi::c_int as int8_t,
                                ],
                                [
                                    -(2 as ::core::ffi::c_int) as int8_t,
                                    -(3 as ::core::ffi::c_int) as int8_t,
                                ],
                                [
                                    2 as ::core::ffi::c_int as int8_t,
                                    -(3 as ::core::ffi::c_int) as int8_t,
                                ],
                                [
                                    -(4 as ::core::ffi::c_int) as int8_t,
                                    -(2 as ::core::ffi::c_int) as int8_t,
                                ],
                                [
                                    4 as ::core::ffi::c_int as int8_t,
                                    -(2 as ::core::ffi::c_int) as int8_t,
                                ],
                                [
                                    -(4 as ::core::ffi::c_int) as int8_t,
                                    -(1 as ::core::ffi::c_int) as int8_t,
                                ],
                                [
                                    4 as ::core::ffi::c_int as int8_t,
                                    -(1 as ::core::ffi::c_int) as int8_t,
                                ],
                                [
                                    -(4 as ::core::ffi::c_int) as int8_t,
                                    0 as ::core::ffi::c_int as int8_t,
                                ],
                                [
                                    4 as ::core::ffi::c_int as int8_t,
                                    0 as ::core::ffi::c_int as int8_t,
                                ],
                                [
                                    -(4 as ::core::ffi::c_int) as int8_t,
                                    1 as ::core::ffi::c_int as int8_t,
                                ],
                                [
                                    4 as ::core::ffi::c_int as int8_t,
                                    1 as ::core::ffi::c_int as int8_t,
                                ],
                                [
                                    -(4 as ::core::ffi::c_int) as int8_t,
                                    2 as ::core::ffi::c_int as int8_t,
                                ],
                                [
                                    4 as ::core::ffi::c_int as int8_t,
                                    2 as ::core::ffi::c_int as int8_t,
                                ],
                                [
                                    -(2 as ::core::ffi::c_int) as int8_t,
                                    3 as ::core::ffi::c_int as int8_t,
                                ],
                                [
                                    2 as ::core::ffi::c_int as int8_t,
                                    3 as ::core::ffi::c_int as int8_t,
                                ],
                            ];
                            if 4 as ::core::ffi::c_int * i_5
                                > (if mv_x_max - omx
                                    < (if omx - mv_x_min
                                        < (if mv_y_max - omy < omy - mv_y_min {
                                            mv_y_max - omy
                                        } else {
                                            omy - mv_y_min
                                        })
                                    {
                                        omx - mv_x_min
                                    } else {
                                        (if mv_y_max - omy < omy - mv_y_min {
                                            mv_y_max - omy
                                        } else {
                                            omy - mv_y_min
                                        })
                                    })
                                {
                                    mv_x_max - omx
                                } else {
                                    (if omx - mv_x_min
                                        < (if mv_y_max - omy < omy - mv_y_min {
                                            mv_y_max - omy
                                        } else {
                                            omy - mv_y_min
                                        })
                                    {
                                        omx - mv_x_min
                                    } else {
                                        (if mv_y_max - omy < omy - mv_y_min {
                                            mv_y_max - omy
                                        } else {
                                            omy - mv_y_min
                                        })
                                    })
                                })
                            {
                                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while j < 16 as ::core::ffi::c_int {
                                    let mut mx_1: ::core::ffi::c_int = omx
                                        + hex4[j as usize][0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int
                                            * i_5;
                                    let mut my_1: ::core::ffi::c_int = omy
                                        + hex4[j as usize][1 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int
                                            * i_5;
                                    if (((mx_1 as uint32_t) << 16 as ::core::ffi::c_int
                                        | my_1 as uint32_t & 0x7fff as uint32_t)
                                        .wrapping_add(mv_min)
                                        | mv_max.wrapping_sub(
                                            (mx_1 as uint32_t) << 16 as ::core::ffi::c_int
                                                | my_1 as uint32_t & 0x7fff as uint32_t,
                                        ))
                                        & 0x80004000 as uint32_t
                                        == 0
                                    {
                                        let mut cost_12: ::core::ffi::c_int = (*h).pixf.fpelcmp
                                            [i_pixel as usize]
                                            .expect("non-null function pointer")(
                                            p_fenc,
                                            FENC_STRIDE as intptr_t,
                                            &mut *p_fref_w.offset((my_1 * stride + mx_1) as isize),
                                            stride as intptr_t,
                                        ) + (*p_cost_mvx
                                            .offset((mx_1 * 4 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int
                                            + *p_cost_mvy
                                                .offset((my_1 * 4 as ::core::ffi::c_int) as isize)
                                                as ::core::ffi::c_int);
                                        if cost_12 < bcost {
                                            bcost = cost_12;
                                            bmx = mx_1;
                                            bmy = my_1;
                                        }
                                    }
                                    j += 1;
                                }
                            } else {
                                let mut dir_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut pix_base_17: *mut pixel =
                                    p_fref_w.offset(omx as isize).offset(
                                        ((omy - 4 as ::core::ffi::c_int * i_5) * stride) as isize,
                                    );
                                let mut dy: ::core::ffi::c_int = i_5 * stride;
                                (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_17
                                        .offset((0 as ::core::ffi::c_int * i_5) as isize)
                                        .offset(
                                            ((-(4 as ::core::ffi::c_int)
                                                - 2 as ::core::ffi::c_int
                                                    * 0 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((0 as ::core::ffi::c_int * i_5) as isize)
                                        .offset(
                                            ((4 as ::core::ffi::c_int
                                                - 2 as ::core::ffi::c_int
                                                    * 0 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset(-((2 as ::core::ffi::c_int * i_5) as isize))
                                        .offset(
                                            ((-(3 as ::core::ffi::c_int)
                                                - 2 as ::core::ffi::c_int
                                                    * 0 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((2 as ::core::ffi::c_int * i_5) as isize)
                                        .offset(
                                            ((-(3 as ::core::ffi::c_int)
                                                - 2 as ::core::ffi::c_int
                                                    * 0 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    stride as intptr_t,
                                    costs.as_mut_ptr().offset(
                                        (4 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
                                            as isize,
                                    ),
                                );
                                pix_base_17 =
                                    pix_base_17.offset((2 as ::core::ffi::c_int * dy) as isize);
                                (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_17
                                        .offset(-((4 as ::core::ffi::c_int * i_5) as isize))
                                        .offset(
                                            ((-(2 as ::core::ffi::c_int)
                                                - 2 as ::core::ffi::c_int
                                                    * 1 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((4 as ::core::ffi::c_int * i_5) as isize)
                                        .offset(
                                            ((-(2 as ::core::ffi::c_int)
                                                - 2 as ::core::ffi::c_int
                                                    * 1 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset(-((4 as ::core::ffi::c_int * i_5) as isize))
                                        .offset(
                                            ((-(1 as ::core::ffi::c_int)
                                                - 2 as ::core::ffi::c_int
                                                    * 1 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((4 as ::core::ffi::c_int * i_5) as isize)
                                        .offset(
                                            ((-(1 as ::core::ffi::c_int)
                                                - 2 as ::core::ffi::c_int
                                                    * 1 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    stride as intptr_t,
                                    costs.as_mut_ptr().offset(
                                        (4 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                            as isize,
                                    ),
                                );
                                pix_base_17 =
                                    pix_base_17.offset((2 as ::core::ffi::c_int * dy) as isize);
                                (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_17
                                        .offset(-((4 as ::core::ffi::c_int * i_5) as isize))
                                        .offset(
                                            ((0 as ::core::ffi::c_int
                                                - 2 as ::core::ffi::c_int
                                                    * 2 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((4 as ::core::ffi::c_int * i_5) as isize)
                                        .offset(
                                            ((0 as ::core::ffi::c_int
                                                - 2 as ::core::ffi::c_int
                                                    * 2 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset(-((4 as ::core::ffi::c_int * i_5) as isize))
                                        .offset(
                                            ((1 as ::core::ffi::c_int
                                                - 2 as ::core::ffi::c_int
                                                    * 2 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((4 as ::core::ffi::c_int * i_5) as isize)
                                        .offset(
                                            ((1 as ::core::ffi::c_int
                                                - 2 as ::core::ffi::c_int
                                                    * 2 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    stride as intptr_t,
                                    costs.as_mut_ptr().offset(
                                        (4 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                            as isize,
                                    ),
                                );
                                pix_base_17 =
                                    pix_base_17.offset((2 as ::core::ffi::c_int * dy) as isize);
                                (*h).pixf.fpelcmp_x4[i_pixel as usize]
                                    .expect("non-null function pointer")(
                                    p_fenc,
                                    pix_base_17
                                        .offset(-((4 as ::core::ffi::c_int * i_5) as isize))
                                        .offset(
                                            ((2 as ::core::ffi::c_int
                                                - 2 as ::core::ffi::c_int
                                                    * 3 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((4 as ::core::ffi::c_int * i_5) as isize)
                                        .offset(
                                            ((2 as ::core::ffi::c_int
                                                - 2 as ::core::ffi::c_int
                                                    * 3 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset(-((2 as ::core::ffi::c_int * i_5) as isize))
                                        .offset(
                                            ((3 as ::core::ffi::c_int
                                                - 2 as ::core::ffi::c_int
                                                    * 3 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    pix_base_17
                                        .offset((2 as ::core::ffi::c_int * i_5) as isize)
                                        .offset(
                                            ((3 as ::core::ffi::c_int
                                                - 2 as ::core::ffi::c_int
                                                    * 3 as ::core::ffi::c_int
                                                + 4 as ::core::ffi::c_int)
                                                * dy)
                                                as isize,
                                        ),
                                    stride as intptr_t,
                                    costs.as_mut_ptr().offset(
                                        (4 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                                            as isize,
                                    ),
                                );
                                pix_base_17 =
                                    pix_base_17.offset((2 as ::core::ffi::c_int * dy) as isize);
                                costs[0 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (-(4 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[1 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[2 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (-(2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (-(3 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[3 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (-(3 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[4 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (-(4 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (-(2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[5 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (-(2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[6 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (-(4 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (-(1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[7 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (-(1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[8 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (-(4 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[9 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[10 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (-(4 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[11 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[12 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (-(4 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[13 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[14 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (-(2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                costs[15 as ::core::ffi::c_int as usize] += *p_cost_omvx.offset(
                                    (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                        as isize,
                                )
                                    as ::core::ffi::c_int
                                    + *p_cost_omvy.offset(
                                        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * i_5)
                                            as isize,
                                    ) as ::core::ffi::c_int;
                                if costs[0 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[0 as ::core::ffi::c_int as usize];
                                    dir_0 = 0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                        + (-(4 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int);
                                }
                                if costs[1 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[1 as ::core::ffi::c_int as usize];
                                    dir_0 = 0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                        + (4 as ::core::ffi::c_int & 15 as ::core::ffi::c_int);
                                }
                                if costs[2 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[2 as ::core::ffi::c_int as usize];
                                    dir_0 = -(2 as ::core::ffi::c_int) * 16 as ::core::ffi::c_int
                                        + (-(3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int);
                                }
                                if costs[3 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[3 as ::core::ffi::c_int as usize];
                                    dir_0 = 2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                        + (-(3 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int);
                                }
                                if costs[4 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[4 as ::core::ffi::c_int as usize];
                                    dir_0 = -(4 as ::core::ffi::c_int) * 16 as ::core::ffi::c_int
                                        + (-(2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int);
                                }
                                if costs[5 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[5 as ::core::ffi::c_int as usize];
                                    dir_0 = 4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                        + (-(2 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int);
                                }
                                if costs[6 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[6 as ::core::ffi::c_int as usize];
                                    dir_0 = -(4 as ::core::ffi::c_int) * 16 as ::core::ffi::c_int
                                        + (-(1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int);
                                }
                                if costs[7 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[7 as ::core::ffi::c_int as usize];
                                    dir_0 = 4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                        + (-(1 as ::core::ffi::c_int) & 15 as ::core::ffi::c_int);
                                }
                                if costs[8 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[8 as ::core::ffi::c_int as usize];
                                    dir_0 = -(4 as ::core::ffi::c_int) * 16 as ::core::ffi::c_int
                                        + (0 as ::core::ffi::c_int & 15 as ::core::ffi::c_int);
                                }
                                if costs[9 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[9 as ::core::ffi::c_int as usize];
                                    dir_0 = 4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                        + (0 as ::core::ffi::c_int & 15 as ::core::ffi::c_int);
                                }
                                if costs[10 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[10 as ::core::ffi::c_int as usize];
                                    dir_0 = -(4 as ::core::ffi::c_int) * 16 as ::core::ffi::c_int
                                        + (1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int);
                                }
                                if costs[11 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[11 as ::core::ffi::c_int as usize];
                                    dir_0 = 4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                        + (1 as ::core::ffi::c_int & 15 as ::core::ffi::c_int);
                                }
                                if costs[12 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[12 as ::core::ffi::c_int as usize];
                                    dir_0 = -(4 as ::core::ffi::c_int) * 16 as ::core::ffi::c_int
                                        + (2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int);
                                }
                                if costs[13 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[13 as ::core::ffi::c_int as usize];
                                    dir_0 = 4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                        + (2 as ::core::ffi::c_int & 15 as ::core::ffi::c_int);
                                }
                                if costs[14 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[14 as ::core::ffi::c_int as usize];
                                    dir_0 = -(2 as ::core::ffi::c_int) * 16 as ::core::ffi::c_int
                                        + (3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int);
                                }
                                if costs[15 as ::core::ffi::c_int as usize] < bcost {
                                    bcost = costs[15 as ::core::ffi::c_int as usize];
                                    dir_0 = 2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                        + (3 as ::core::ffi::c_int & 15 as ::core::ffi::c_int);
                                }
                                if dir_0 != 0 {
                                    bmx = omx + i_5 * (dir_0 >> 4 as ::core::ffi::c_int);
                                    bmy = (omy as int32_t
                                        + i_5 as int32_t
                                            * (((dir_0 as uint32_t) << 28 as ::core::ffi::c_int)
                                                as int32_t
                                                >> 28 as ::core::ffi::c_int))
                                        as ::core::ffi::c_int;
                                }
                            }
                            i_5 += 1;
                            if !(i_5 <= i_me_range >> 2 as ::core::ffi::c_int) {
                                break;
                            }
                        }
                        if bmy <= mv_y_max && bmy >= mv_y_min && bmx <= mv_x_max && bmx >= mv_x_min
                        {
                            current_block = 14690580863265192683;
                        } else {
                            current_block = 14127502640287082657;
                        }
                    }
                }
            }
        }
        X264_ME_ESA | X264_ME_TESA => {
            let min_x: ::core::ffi::c_int = if bmx - i_me_range > mv_x_min {
                bmx - i_me_range
            } else {
                mv_x_min
            };
            let min_y: ::core::ffi::c_int = if bmy - i_me_range > mv_y_min {
                bmy - i_me_range
            } else {
                mv_y_min
            };
            let max_x: ::core::ffi::c_int = if bmx + i_me_range < mv_x_max {
                bmx + i_me_range
            } else {
                mv_x_max
            };
            let max_y: ::core::ffi::c_int = if bmy + i_me_range < mv_y_max {
                bmy + i_me_range
            } else {
                mv_y_max
            };
            let width: ::core::ffi::c_int =
                max_x - min_x + 3 as ::core::ffi::c_int & !(3 as ::core::ffi::c_int);
            let mut sums_base: *mut uint16_t = (*m).integral;
            let mut enc_dc: [::core::ffi::c_int; 4] = [0; 4];
            let mut sad_size: ::core::ffi::c_int = if i_pixel <= PIXEL_8x8 as ::core::ffi::c_int {
                PIXEL_8x8 as ::core::ffi::c_int
            } else {
                PIXEL_4x4 as ::core::ffi::c_int
            };
            let mut delta: ::core::ffi::c_int =
                x264_pixel_size[sad_size as usize].w as ::core::ffi::c_int;
            let mut xs: *mut int16_t = (*h).scratch_buffer as *mut int16_t;
            let mut xn: ::core::ffi::c_int = 0;
            let mut cost_fpel_mvx: *mut uint16_t = (*h).cost_mv_fpel[(*h).mb.i_qp as usize][(-((*m)
                .mvp[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int)
                & 3 as ::core::ffi::c_int)
                as usize]
                .offset(
                    (-((*m).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                        >> 2 as ::core::ffi::c_int) as isize,
                );
            (*h).pixf.sad_x4[sad_size as usize].expect("non-null function pointer")(
                x264_zero.as_mut_ptr() as *mut pixel,
                p_fenc,
                p_fenc.offset(delta as isize),
                p_fenc.offset((delta * FENC_STRIDE) as isize),
                p_fenc
                    .offset(delta as isize)
                    .offset((delta * FENC_STRIDE) as isize),
                FENC_STRIDE as intptr_t,
                enc_dc.as_mut_ptr(),
            );
            if delta == 4 as ::core::ffi::c_int {
                sums_base = sums_base.offset(
                    (stride
                        * ((*(*h).fenc).i_lines[0 as ::core::ffi::c_int as usize]
                            + PADV * 2 as ::core::ffi::c_int)) as isize,
                );
            }
            if i_pixel == PIXEL_16x16 as ::core::ffi::c_int
                || i_pixel == PIXEL_8x16 as ::core::ffi::c_int
                || i_pixel == PIXEL_4x8 as ::core::ffi::c_int
            {
                delta *= stride;
            }
            if i_pixel == PIXEL_8x16 as ::core::ffi::c_int
                || i_pixel == PIXEL_4x8 as ::core::ffi::c_int
            {
                enc_dc[1 as ::core::ffi::c_int as usize] = enc_dc[2 as ::core::ffi::c_int as usize];
            }
            if (*h).mb.i_me_method == X264_ME_TESA {
                let mut mvsads: *mut mvsad_t =
                    xs.offset(
                        (width + 31 as ::core::ffi::c_int & !(31 as ::core::ffi::c_int)) as isize,
                    )
                    .offset(4 as ::core::ffi::c_int as isize) as *mut mvsad_t;
                let mut nmvsad: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut limit: ::core::ffi::c_int = 0;
                let mut sad_thresh: ::core::ffi::c_int = if i_me_range <= 16 as ::core::ffi::c_int {
                    10 as ::core::ffi::c_int
                } else if i_me_range <= 24 as ::core::ffi::c_int {
                    11 as ::core::ffi::c_int
                } else {
                    12 as ::core::ffi::c_int
                };
                let mut bsad: ::core::ffi::c_int =
                    (*h).pixf.sad[i_pixel as usize].expect("non-null function pointer")(
                        p_fenc,
                        FENC_STRIDE as intptr_t,
                        p_fref_w
                            .offset((bmy * stride) as isize)
                            .offset(bmx as isize),
                        stride as intptr_t,
                    ) + (*p_cost_mvx.offset((bmx * 4 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset((bmy * 4 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int);
                let mut my_2: ::core::ffi::c_int = min_y;
                while my_2 <= max_y {
                    let mut i_6: ::core::ffi::c_int = 0;
                    let mut ycost: ::core::ffi::c_int = *p_cost_mvy
                        .offset((my_2 * 4 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int;
                    if !(bsad <= ycost) {
                        bsad -= ycost;
                        xn = (*h).pixf.ads[i_pixel as usize].expect("non-null function pointer")(
                            enc_dc.as_mut_ptr(),
                            sums_base
                                .offset(min_x as isize)
                                .offset((my_2 * stride) as isize),
                            delta,
                            cost_fpel_mvx.offset(min_x as isize),
                            xs,
                            width,
                            bsad * 17 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                        );
                        i_6 = 0 as ::core::ffi::c_int;
                        while i_6 < xn - 2 as ::core::ffi::c_int {
                            let mut ref_0: *mut pixel = p_fref_w
                                .offset(min_x as isize)
                                .offset((my_2 * stride) as isize);
                            let mut sads: [::core::ffi::c_int; 4] = [0; 4];
                            (*h)
                                .pixf
                                .sad_x3[i_pixel as usize]
                                .expect(
                                    "non-null function pointer",
                                )(
                                p_fenc,
                                ref_0
                                    .offset(
                                        *xs.offset(i_6 as isize) as ::core::ffi::c_int as isize,
                                    ),
                                ref_0
                                    .offset(
                                        *xs.offset((i_6 + 1 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int as isize,
                                    ),
                                ref_0
                                    .offset(
                                        *xs.offset((i_6 + 2 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int as isize,
                                    ),
                                stride as intptr_t,
                                sads.as_mut_ptr(),
                            );
                            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while j_0 < 3 as ::core::ffi::c_int {
                                let mut sad: ::core::ffi::c_int = sads[j_0 as usize]
                                    + *cost_fpel_mvx
                                        .offset(*xs.offset((i_6 + j_0) as isize) as isize)
                                        as ::core::ffi::c_int;
                                if sad < bsad * sad_thresh >> 3 as ::core::ffi::c_int {
                                    if sad < bsad {
                                        bsad = sad;
                                    }
                                    (*mvsads.offset(nmvsad as isize)).sad = sad + ycost;
                                    (*mvsads.offset(nmvsad as isize)).mv
                                        [0 as ::core::ffi::c_int as usize] = (min_x
                                        + *xs.offset((i_6 + j_0) as isize) as ::core::ffi::c_int)
                                        as int16_t;
                                    (*mvsads.offset(nmvsad as isize)).mv
                                        [1 as ::core::ffi::c_int as usize] = my_2 as int16_t;
                                    nmvsad += 1;
                                }
                                j_0 += 1;
                            }
                            i_6 += 3 as ::core::ffi::c_int;
                        }
                        while i_6 < xn {
                            let mut mx_2: ::core::ffi::c_int =
                                min_x + *xs.offset(i_6 as isize) as ::core::ffi::c_int;
                            let mut sad_0: ::core::ffi::c_int = (*h).pixf.sad[i_pixel as usize]
                                .expect("non-null function pointer")(
                                p_fenc,
                                FENC_STRIDE as intptr_t,
                                p_fref_w
                                    .offset(mx_2 as isize)
                                    .offset((my_2 * stride) as isize),
                                stride as intptr_t,
                            ) + *cost_fpel_mvx
                                .offset(*xs.offset(i_6 as isize) as isize)
                                as ::core::ffi::c_int;
                            if sad_0 < bsad * sad_thresh >> 3 as ::core::ffi::c_int {
                                if sad_0 < bsad {
                                    bsad = sad_0;
                                }
                                (*mvsads.offset(nmvsad as isize)).sad = sad_0 + ycost;
                                (*mvsads.offset(nmvsad as isize)).mv
                                    [0 as ::core::ffi::c_int as usize] = mx_2 as int16_t;
                                (*mvsads.offset(nmvsad as isize)).mv
                                    [1 as ::core::ffi::c_int as usize] = my_2 as int16_t;
                                nmvsad += 1;
                            }
                            i_6 += 1;
                        }
                        bsad += ycost;
                    }
                    my_2 += 1;
                }
                limit = i_me_range >> 1 as ::core::ffi::c_int;
                sad_thresh = bsad * sad_thresh >> 3 as ::core::ffi::c_int;
                while nmvsad > limit * 2 as ::core::ffi::c_int && sad_thresh > bsad {
                    let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    sad_thresh = sad_thresh + bsad >> 1 as ::core::ffi::c_int;
                    while i_7 < nmvsad && (*mvsads.offset(i_7 as isize)).sad <= sad_thresh {
                        i_7 += 1;
                    }
                    let mut j_1: ::core::ffi::c_int = i_7;
                    while j_1 < nmvsad {
                        let mut sad_1: uint32_t = 0;
                        if WORD_SIZE == 8 as uint64_t
                            && ::core::mem::size_of::<mvsad_t>() as usize == 8 as usize
                        {
                            let ref mut fresh0 = (*(&mut *mvsads.offset(i_7 as isize)
                                as *mut mvsad_t
                                as *mut x264_union64_t))
                                .i;
                            *fresh0 = (*(&mut *mvsads.offset(j_1 as isize) as *mut mvsad_t
                                as *mut x264_union64_t))
                                .i;
                            let mut mvsad: uint64_t = *fresh0;
                            sad_1 = mvsad as uint32_t;
                        } else {
                            sad_1 = (*mvsads.offset(j_1 as isize)).sad as uint32_t;
                            (*((*mvsads.offset(i_7 as isize)).mv.as_mut_ptr()
                                as *mut x264_union32_t))
                                .i = (*((*mvsads.offset(j_1 as isize)).mv.as_mut_ptr()
                                as *mut x264_union32_t))
                                .i;
                            (*mvsads.offset(i_7 as isize)).sad = sad_1 as ::core::ffi::c_int;
                        }
                        i_7 = (i_7 as uint32_t).wrapping_add(
                            sad_1.wrapping_sub((sad_thresh + 1 as ::core::ffi::c_int) as uint32_t)
                                >> 31 as ::core::ffi::c_int,
                        ) as ::core::ffi::c_int as ::core::ffi::c_int;
                        j_1 += 1;
                    }
                    nmvsad = i_7;
                }
                while nmvsad > limit {
                    let mut bi: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut i_8: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    while i_8 < nmvsad {
                        if (*mvsads.offset(i_8 as isize)).sad > (*mvsads.offset(bi as isize)).sad {
                            bi = i_8;
                        }
                        i_8 += 1;
                    }
                    nmvsad -= 1;
                    if ::core::mem::size_of::<mvsad_t>() as usize
                        == ::core::mem::size_of::<uint64_t>() as usize
                    {
                        (*(&mut *mvsads.offset(bi as isize) as *mut mvsad_t
                            as *mut x264_union64_t))
                            .i = (*(&mut *mvsads.offset(nmvsad as isize) as *mut mvsad_t
                            as *mut x264_union64_t))
                            .i;
                    } else {
                        *mvsads.offset(bi as isize) = *mvsads.offset(nmvsad as isize);
                    }
                }
                let mut i_9: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_9 < nmvsad {
                    let mut cost_13: ::core::ffi::c_int = (*h).pixf.fpelcmp[i_pixel as usize]
                        .expect("non-null function pointer")(
                        p_fenc,
                        FENC_STRIDE as intptr_t,
                        &mut *p_fref_w.offset(
                            (*(*mvsads.offset(i_9 as isize))
                                .mv
                                .as_mut_ptr()
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                * stride
                                + *(*mvsads.offset(i_9 as isize))
                                    .mv
                                    .as_mut_ptr()
                                    .offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int) as isize,
                        ),
                        stride as intptr_t,
                    ) + (*p_cost_mvx.offset(
                        ((*mvsads.offset(i_9 as isize)).mv[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((*mvsads.offset(i_9 as isize)).mv[1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int);
                    if cost_13 < bcost {
                        bcost = cost_13;
                        bmx = (*mvsads.offset(i_9 as isize)).mv[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                        bmy = (*mvsads.offset(i_9 as isize)).mv[1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                    }
                    i_9 += 1;
                }
            } else {
                let mut my_3: ::core::ffi::c_int = min_y;
                while my_3 <= max_y {
                    let mut i_10: ::core::ffi::c_int = 0;
                    let mut ycost_0: ::core::ffi::c_int = *p_cost_mvy
                        .offset((my_3 * 4 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int;
                    if !(bcost <= ycost_0) {
                        bcost -= ycost_0;
                        xn = (*h).pixf.ads[i_pixel as usize].expect("non-null function pointer")(
                            enc_dc.as_mut_ptr(),
                            sums_base
                                .offset(min_x as isize)
                                .offset((my_3 * stride) as isize),
                            delta,
                            cost_fpel_mvx.offset(min_x as isize),
                            xs,
                            width,
                            bcost,
                        );
                        i_10 = 0 as ::core::ffi::c_int;
                        while i_10 < xn - 2 as ::core::ffi::c_int {
                            (*h).pixf.fpelcmp_x3[i_pixel as usize]
                                .expect("non-null function pointer")(
                                p_fenc,
                                p_fref_w
                                    .offset(
                                        (min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int)
                                            as isize,
                                    )
                                    .offset((my_3 * stride) as isize),
                                p_fref_w
                                    .offset(
                                        (min_x
                                            + *xs.offset((i_10 + 1 as ::core::ffi::c_int) as isize)
                                                as ::core::ffi::c_int)
                                            as isize,
                                    )
                                    .offset((my_3 * stride) as isize),
                                p_fref_w
                                    .offset(
                                        (min_x
                                            + *xs.offset((i_10 + 2 as ::core::ffi::c_int) as isize)
                                                as ::core::ffi::c_int)
                                            as isize,
                                    )
                                    .offset((my_3 * stride) as isize),
                                stride as intptr_t,
                                costs.as_mut_ptr(),
                            );
                            costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                ((min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int)
                                    * 4 as ::core::ffi::c_int)
                                    as isize,
                            )
                                as ::core::ffi::c_int;
                            costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                ((min_x
                                    + *xs.offset((i_10 + 1 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int)
                                    * 4 as ::core::ffi::c_int)
                                    as isize,
                            )
                                as ::core::ffi::c_int;
                            costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                                ((min_x
                                    + *xs.offset((i_10 + 2 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int)
                                    * 4 as ::core::ffi::c_int)
                                    as isize,
                            )
                                as ::core::ffi::c_int;
                            if costs[0 as ::core::ffi::c_int as usize] < bcost {
                                bcost = costs[0 as ::core::ffi::c_int as usize];
                                bmx = min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int;
                                bmy = my_3;
                            }
                            if costs[1 as ::core::ffi::c_int as usize] < bcost {
                                bcost = costs[1 as ::core::ffi::c_int as usize];
                                bmx = min_x
                                    + *xs.offset((i_10 + 1 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int;
                                bmy = my_3;
                            }
                            if costs[2 as ::core::ffi::c_int as usize] < bcost {
                                bcost = costs[2 as ::core::ffi::c_int as usize];
                                bmx = min_x
                                    + *xs.offset((i_10 + 2 as ::core::ffi::c_int) as isize)
                                        as ::core::ffi::c_int;
                                bmy = my_3;
                            }
                            i_10 += 3 as ::core::ffi::c_int;
                        }
                        bcost += ycost_0;
                        while i_10 < xn {
                            let mut cost_14: ::core::ffi::c_int = (*h).pixf.fpelcmp
                                [i_pixel as usize]
                                .expect("non-null function pointer")(
                                p_fenc,
                                FENC_STRIDE as intptr_t,
                                &mut *p_fref_w.offset(
                                    (my_3 * stride
                                        + (min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int))
                                        as isize,
                                ),
                                stride as intptr_t,
                            ) + (*p_cost_mvx.offset(
                                ((min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int)
                                    * 4 as ::core::ffi::c_int)
                                    as isize,
                            )
                                as ::core::ffi::c_int
                                + *p_cost_mvy.offset((my_3 * 4 as ::core::ffi::c_int) as isize)
                                    as ::core::ffi::c_int);
                            if cost_14 < bcost {
                                bcost = cost_14;
                                bmx = min_x + *xs.offset(i_10 as isize) as ::core::ffi::c_int;
                                bmy = my_3;
                            }
                            i_10 += 1;
                        }
                    }
                    my_3 += 1;
                }
            }
            current_block = 14127502640287082657;
        }
        _ => {
            current_block = 14127502640287082657;
        }
    }
    match current_block {
        14690580863265192683 => {
            let mut pix_base_0: *mut pixel = p_fref_w
                .offset(bmx as isize)
                .offset((bmy * stride) as isize);
            (*h).pixf.fpelcmp_x3[i_pixel as usize].expect("non-null function pointer")(
                p_fenc,
                pix_base_0
                    .offset(-(2 as ::core::ffi::c_int) as isize)
                    .offset((0 as ::core::ffi::c_int * stride) as isize),
                pix_base_0
                    .offset(-(1 as ::core::ffi::c_int) as isize)
                    .offset((2 as ::core::ffi::c_int * stride) as isize),
                pix_base_0
                    .offset(1 as ::core::ffi::c_int as isize)
                    .offset((2 as ::core::ffi::c_int * stride) as isize),
                stride as intptr_t,
                costs.as_mut_ptr(),
            );
            costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((bmx + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((bmx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((bmx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            let mut pix_base_1: *mut pixel = p_fref_w
                .offset(bmx as isize)
                .offset((bmy * stride) as isize);
            (*h).pixf.fpelcmp_x3[i_pixel as usize].expect("non-null function pointer")(
                p_fenc,
                pix_base_1
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset((0 as ::core::ffi::c_int * stride) as isize),
                pix_base_1
                    .offset(1 as ::core::ffi::c_int as isize)
                    .offset((-(2 as ::core::ffi::c_int) * stride) as isize),
                pix_base_1
                    .offset(-(1 as ::core::ffi::c_int) as isize)
                    .offset((-(2 as ::core::ffi::c_int) * stride) as isize),
                stride as intptr_t,
                costs.as_mut_ptr().offset(4 as ::core::ffi::c_int as isize),
            );
            *costs
                .as_mut_ptr()
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) += *p_cost_mvx
                .offset(((bmx + 2 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            *costs
                .as_mut_ptr()
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(1 as ::core::ffi::c_int as isize) += *p_cost_mvx
                .offset(((bmx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            *costs
                .as_mut_ptr()
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize) += *p_cost_mvx
                .offset(((bmx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + -(2 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            bcost <<= 3 as ::core::ffi::c_int;
            if ((costs[0 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                + 2 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[0 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_int;
            }
            if ((costs[1 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                + 3 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[1 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                    + 3 as ::core::ffi::c_int;
            }
            if ((costs[2 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                + 4 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[2 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                    + 4 as ::core::ffi::c_int;
            }
            if ((costs[4 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                + 5 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[4 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                    + 5 as ::core::ffi::c_int;
            }
            if ((costs[5 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                + 6 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[5 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                    + 6 as ::core::ffi::c_int;
            }
            if ((costs[6 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                + 7 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[6 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                    + 7 as ::core::ffi::c_int;
            }
            if bcost & 7 as ::core::ffi::c_int != 0 {
                let mut dir: ::core::ffi::c_int =
                    (bcost & 7 as ::core::ffi::c_int) - 2 as ::core::ffi::c_int;
                bmx += hex2[(dir + 1 as ::core::ffi::c_int) as usize]
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                bmy += hex2[(dir + 1 as ::core::ffi::c_int) as usize]
                    [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                let mut i_2: ::core::ffi::c_int =
                    (i_me_range >> 1 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
                while i_2 > 0 as ::core::ffi::c_int
                    && (((bmx as uint32_t) << 16 as ::core::ffi::c_int
                        | bmy as uint32_t & 0x7fff as uint32_t)
                        .wrapping_add(mv_min)
                        | mv_max.wrapping_sub(
                            (bmx as uint32_t) << 16 as ::core::ffi::c_int
                                | bmy as uint32_t & 0x7fff as uint32_t,
                        ))
                        & 0x80004000 as uint32_t
                        == 0
                {
                    let mut pix_base_2: *mut pixel = p_fref_w
                        .offset(bmx as isize)
                        .offset((bmy * stride) as isize);
                    (*h).pixf.fpelcmp_x3[i_pixel as usize].expect("non-null function pointer")(
                        p_fenc,
                        pix_base_2
                            .offset(
                                hex2[(dir + 0 as ::core::ffi::c_int) as usize]
                                    [0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int as isize,
                            )
                            .offset(
                                (hex2[(dir + 0 as ::core::ffi::c_int) as usize]
                                    [1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int
                                    * stride) as isize,
                            ),
                        pix_base_2
                            .offset(
                                hex2[(dir + 1 as ::core::ffi::c_int) as usize]
                                    [0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int as isize,
                            )
                            .offset(
                                (hex2[(dir + 1 as ::core::ffi::c_int) as usize]
                                    [1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int
                                    * stride) as isize,
                            ),
                        pix_base_2
                            .offset(
                                hex2[(dir + 2 as ::core::ffi::c_int) as usize]
                                    [0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int as isize,
                            )
                            .offset(
                                (hex2[(dir + 2 as ::core::ffi::c_int) as usize]
                                    [1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int
                                    * stride) as isize,
                            ),
                        stride as intptr_t,
                        costs.as_mut_ptr(),
                    );
                    costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((bmx
                            + hex2[(dir + 0 as ::core::ffi::c_int) as usize]
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                            * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((bmy
                                + hex2[(dir + 0 as ::core::ffi::c_int) as usize]
                                    [1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int)
                                * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((bmx
                            + hex2[(dir + 1 as ::core::ffi::c_int) as usize]
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                            * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((bmy
                                + hex2[(dir + 1 as ::core::ffi::c_int) as usize]
                                    [1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int)
                                * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(
                        ((bmx
                            + hex2[(dir + 2 as ::core::ffi::c_int) as usize]
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                            * 4 as ::core::ffi::c_int) as isize,
                    )
                        as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            ((bmy
                                + hex2[(dir + 2 as ::core::ffi::c_int) as usize]
                                    [1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int)
                                * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    bcost &= !(7 as ::core::ffi::c_int);
                    if ((costs[0 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int)
                        < bcost
                    {
                        bcost = (costs[0 as ::core::ffi::c_int as usize]
                            << 3 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_int;
                    }
                    if ((costs[1 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                        + 2 as ::core::ffi::c_int)
                        < bcost
                    {
                        bcost = (costs[1 as ::core::ffi::c_int as usize]
                            << 3 as ::core::ffi::c_int)
                            + 2 as ::core::ffi::c_int;
                    }
                    if ((costs[2 as ::core::ffi::c_int as usize] << 3 as ::core::ffi::c_int)
                        + 3 as ::core::ffi::c_int)
                        < bcost
                    {
                        bcost = (costs[2 as ::core::ffi::c_int as usize]
                            << 3 as ::core::ffi::c_int)
                            + 3 as ::core::ffi::c_int;
                    }
                    if bcost & 7 as ::core::ffi::c_int == 0 {
                        break;
                    }
                    dir += (bcost & 7 as ::core::ffi::c_int) - 2 as ::core::ffi::c_int;
                    dir = mod6m1[(dir + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
                    bmx += hex2[(dir + 1 as ::core::ffi::c_int) as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                    bmy += hex2[(dir + 1 as ::core::ffi::c_int) as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                    i_2 -= 1;
                }
            }
            bcost >>= 3 as ::core::ffi::c_int;
            bcost <<= 4 as ::core::ffi::c_int;
            let mut pix_base_3: *mut pixel = p_fref_w
                .offset(bmx as isize)
                .offset((bmy * stride) as isize);
            (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                p_fenc,
                pix_base_3
                    .offset(0 as ::core::ffi::c_int as isize)
                    .offset((-(1 as ::core::ffi::c_int) * stride) as isize),
                pix_base_3
                    .offset(0 as ::core::ffi::c_int as isize)
                    .offset((1 as ::core::ffi::c_int * stride) as isize),
                pix_base_3
                    .offset(-(1 as ::core::ffi::c_int) as isize)
                    .offset((0 as ::core::ffi::c_int * stride) as isize),
                pix_base_3
                    .offset(1 as ::core::ffi::c_int as isize)
                    .offset((0 as ::core::ffi::c_int * stride) as isize),
                stride as intptr_t,
                costs.as_mut_ptr(),
            );
            costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((bmx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((bmx + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((bmx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((bmx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + 0 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            if ((costs[0 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[0 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int;
            }
            if ((costs[1 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 2 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[1 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_int;
            }
            if ((costs[2 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 3 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[2 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 3 as ::core::ffi::c_int;
            }
            if ((costs[3 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 4 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[3 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 4 as ::core::ffi::c_int;
            }
            let mut pix_base_4: *mut pixel = p_fref_w
                .offset(bmx as isize)
                .offset((bmy * stride) as isize);
            (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                p_fenc,
                pix_base_4
                    .offset(-(1 as ::core::ffi::c_int) as isize)
                    .offset((-(1 as ::core::ffi::c_int) * stride) as isize),
                pix_base_4
                    .offset(-(1 as ::core::ffi::c_int) as isize)
                    .offset((1 as ::core::ffi::c_int * stride) as isize),
                pix_base_4
                    .offset(1 as ::core::ffi::c_int as isize)
                    .offset((-(1 as ::core::ffi::c_int) * stride) as isize),
                pix_base_4
                    .offset(1 as ::core::ffi::c_int as isize)
                    .offset((1 as ::core::ffi::c_int * stride) as isize),
                stride as intptr_t,
                costs.as_mut_ptr(),
            );
            costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((bmx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((bmx + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[2 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((bmx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + -(1 as ::core::ffi::c_int)) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[3 as ::core::ffi::c_int as usize] += *p_cost_mvx
                .offset(((bmx + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy
                    .offset(((bmy + 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            if ((costs[0 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 5 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[0 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 5 as ::core::ffi::c_int;
            }
            if ((costs[1 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 6 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[1 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 6 as ::core::ffi::c_int;
            }
            if ((costs[2 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 7 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[2 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 7 as ::core::ffi::c_int;
            }
            if ((costs[3 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 8 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[3 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                    + 8 as ::core::ffi::c_int;
            }
            bmx += square1[(bcost & 15 as ::core::ffi::c_int) as usize]
                [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
            bmy += square1[(bcost & 15 as ::core::ffi::c_int) as usize]
                [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
            bcost >>= 4 as ::core::ffi::c_int;
        }
        _ => {}
    }
    let mut bmv: uint32_t = pack16to32_mask(bmx, bmy);
    let mut bmv_spel: uint32_t = bmv.wrapping_mul(4 as uint32_t) & 0xfffcfffc as uint32_t;
    if (*h).mb.i_subpel_refine < 3 as ::core::ffi::c_int {
        (*m).cost_mv = *p_cost_mvx.offset((bmx * 4 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            + *p_cost_mvy.offset((bmy * 4 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
        (*m).cost = bcost;
        if bmv == pmv {
            (*m).cost += (*m).cost_mv;
        }
        (*((*m).mv.as_mut_ptr() as *mut x264_union32_t)).i = bmv_spel;
    } else {
        (*((*m).mv.as_mut_ptr() as *mut x264_union32_t)).i = if bpred_cost < bcost {
            bpred_mv
        } else {
            bmv_spel
        };
        (*m).cost = if bpred_cost < bcost {
            bpred_cost
        } else {
            bcost
        };
    }
    if (*h).mb.i_subpel_refine >= 2 as ::core::ffi::c_int {
        let mut hpel: ::core::ffi::c_int = subpel_iterations[(*h).mb.i_subpel_refine as usize]
            [2 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int;
        let mut qpel: ::core::ffi::c_int = subpel_iterations[(*h).mb.i_subpel_refine as usize]
            [3 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int;
        refine_subpel(h, m, hpel, qpel, p_halfpel_thresh, 0 as ::core::ffi::c_int);
    }
}
#[no_mangle]
#[c2rust::src_loc = "801:1"]
pub unsafe extern "C" fn x264_10_me_refine_qpel(mut h: *mut x264_t, mut m: *mut x264_me_t) {
    let mut hpel: ::core::ffi::c_int = subpel_iterations[(*h).mb.i_subpel_refine as usize]
        [0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int;
    let mut qpel: ::core::ffi::c_int = subpel_iterations[(*h).mb.i_subpel_refine as usize]
        [1 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int;
    if (*m).i_pixel <= PIXEL_8x8 as ::core::ffi::c_int {
        (*m).cost -= (*m).i_ref_cost;
    }
    refine_subpel(
        h,
        m,
        hpel,
        qpel,
        0 as *mut ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "812:1"]
pub unsafe extern "C" fn x264_10_me_refine_qpel_refdupe(
    mut h: *mut x264_t,
    mut m: *mut x264_me_t,
    mut p_halfpel_thresh: *mut ::core::ffi::c_int,
) {
    refine_subpel(
        h,
        m,
        0 as ::core::ffi::c_int,
        if (2 as ::core::ffi::c_int)
            < subpel_iterations[(*h).mb.i_subpel_refine as usize][3 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
        {
            2 as ::core::ffi::c_int
        } else {
            subpel_iterations[(*h).mb.i_subpel_refine as usize][3 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
        },
        p_halfpel_thresh,
        0 as ::core::ffi::c_int,
    );
}
#[c2rust::src_loc = "865:1"]
unsafe extern "C" fn refine_subpel(
    mut h: *mut x264_t,
    mut m: *mut x264_me_t,
    mut hpel_iters: ::core::ffi::c_int,
    mut qpel_iters: ::core::ffi::c_int,
    mut p_halfpel_thresh: *mut ::core::ffi::c_int,
    mut b_refine_qpel: ::core::ffi::c_int,
) {
    let bw: ::core::ffi::c_int = x264_pixel_size[(*m).i_pixel as usize].w as ::core::ffi::c_int;
    let bh: ::core::ffi::c_int = x264_pixel_size[(*m).i_pixel as usize].h as ::core::ffi::c_int;
    let mut p_cost_mvx: *const uint16_t = (*m)
        .p_cost_mv
        .offset(-((*m).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize));
    let mut p_cost_mvy: *const uint16_t = (*m)
        .p_cost_mv
        .offset(-((*m).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize));
    let i_pixel: ::core::ffi::c_int = (*m).i_pixel;
    let b_chroma_me: ::core::ffi::c_int = ((*h).mb.b_chroma_me != 0
        && (i_pixel <= PIXEL_8x8 as ::core::ffi::c_int
            || (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int))
        as ::core::ffi::c_int;
    let mut chromapix: ::core::ffi::c_int =
        (*h).luma2chroma_pixel[i_pixel as usize] as ::core::ffi::c_int;
    let mut chroma_v_shift: ::core::ffi::c_int = (*h).mb.chroma_v_shift;
    let mut mvy_offset: ::core::ffi::c_int =
        if chroma_v_shift & (*h).mb.b_interlaced & (*m).i_ref != 0 {
            ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    let mut pix: [pixel; 1152] = [0; 1152];
    let mut costs: [::core::ffi::c_int; 4] = [0; 4];
    let mut bmx: ::core::ffi::c_int =
        (*m).mv[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    let mut bmy: ::core::ffi::c_int =
        (*m).mv[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    let mut bcost: ::core::ffi::c_int = (*m).cost;
    let mut odir: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut bdir: ::core::ffi::c_int = 0;
    if hpel_iters != 0 {
        if (*h).mb.i_subpel_refine < 3 as ::core::ffi::c_int {
            let mut mx: ::core::ffi::c_int = x264_clip3(
                (*m).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                (*h).mb.mv_min_spel[0 as ::core::ffi::c_int as usize] + 2 as ::core::ffi::c_int,
                (*h).mb.mv_max_spel[0 as ::core::ffi::c_int as usize] - 2 as ::core::ffi::c_int,
            );
            let mut my: ::core::ffi::c_int = x264_clip3(
                (*m).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                (*h).mb.mv_min_spel[1 as ::core::ffi::c_int as usize] + 2 as ::core::ffi::c_int,
                (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize] - 2 as ::core::ffi::c_int,
            );
            if mx - bmx | my - bmy != 0 {
                let mut stride: intptr_t = 16 as intptr_t;
                let mut src: *mut pixel = (*h).mc.get_ref.expect("non-null function pointer")(
                    pix.as_mut_ptr(),
                    &mut stride,
                    (*m).p_fref.as_mut_ptr(),
                    (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                    mx,
                    my,
                    bw,
                    bh,
                    &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
                );
                let mut cost: ::core::ffi::c_int =
                    (*h).pixf.fpelcmp[i_pixel as usize].expect("non-null function pointer")(
                        (*m).p_fenc[0 as ::core::ffi::c_int as usize],
                        FENC_STRIDE as intptr_t,
                        src,
                        stride,
                    ) + *p_cost_mvx.offset(mx as isize) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(my as isize) as ::core::ffi::c_int;
                if cost < bcost {
                    bcost = cost;
                    bmx = mx;
                    bmy = my;
                }
            }
        }
        bcost <<= 6 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = hpel_iters;
        while i > 0 as ::core::ffi::c_int {
            let mut omx: ::core::ffi::c_int = bmx;
            let mut omy: ::core::ffi::c_int = bmy;
            let mut stride_0: intptr_t = 64 as intptr_t;
            let mut src0: *mut pixel = 0 as *mut pixel;
            let mut src1: *mut pixel = 0 as *mut pixel;
            let mut src2: *mut pixel = 0 as *mut pixel;
            let mut src3: *mut pixel = 0 as *mut pixel;
            src0 = (*h).mc.get_ref.expect("non-null function pointer")(
                pix.as_mut_ptr(),
                &mut stride_0,
                (*m).p_fref.as_mut_ptr(),
                (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                omx,
                omy - 2 as ::core::ffi::c_int,
                bw,
                bh + 1 as ::core::ffi::c_int,
                &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
            );
            src2 = (*h).mc.get_ref.expect("non-null function pointer")(
                pix.as_mut_ptr().offset(32 as ::core::ffi::c_int as isize),
                &mut stride_0,
                (*m).p_fref.as_mut_ptr(),
                (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                omx - 2 as ::core::ffi::c_int,
                omy,
                bw + 4 as ::core::ffi::c_int,
                bh,
                &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
            );
            src1 = src0.offset(stride_0 as isize);
            src3 = src2.offset(1 as ::core::ffi::c_int as isize);
            (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
                (*m).p_fenc[0 as ::core::ffi::c_int as usize],
                src0,
                src1,
                src2,
                src3,
                stride_0,
                costs.as_mut_ptr(),
            );
            costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(omx as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy.offset((omy - 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(omx as isize)
                as ::core::ffi::c_int
                + *p_cost_mvy.offset((omy + 2 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int;
            costs[2 as ::core::ffi::c_int as usize] +=
                *p_cost_mvx.offset((omx - 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(omy as isize) as ::core::ffi::c_int;
            costs[3 as ::core::ffi::c_int as usize] +=
                *p_cost_mvx.offset((omx + 2 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(omy as isize) as ::core::ffi::c_int;
            if ((costs[0 as ::core::ffi::c_int as usize] << 6 as ::core::ffi::c_int)
                + 2 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[0 as ::core::ffi::c_int as usize] << 6 as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_int;
            }
            if ((costs[1 as ::core::ffi::c_int as usize] << 6 as ::core::ffi::c_int)
                + 6 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[1 as ::core::ffi::c_int as usize] << 6 as ::core::ffi::c_int)
                    + 6 as ::core::ffi::c_int;
            }
            if ((costs[2 as ::core::ffi::c_int as usize] << 6 as ::core::ffi::c_int)
                + 16 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[2 as ::core::ffi::c_int as usize] << 6 as ::core::ffi::c_int)
                    + 16 as ::core::ffi::c_int;
            }
            if ((costs[3 as ::core::ffi::c_int as usize] << 6 as ::core::ffi::c_int)
                + 48 as ::core::ffi::c_int)
                < bcost
            {
                bcost = (costs[3 as ::core::ffi::c_int as usize] << 6 as ::core::ffi::c_int)
                    + 48 as ::core::ffi::c_int;
            }
            if bcost & 63 as ::core::ffi::c_int == 0 {
                break;
            }
            bmx -= (((bcost as uint32_t) << 26 as ::core::ffi::c_int) as int32_t
                >> 29 as ::core::ffi::c_int) as ::core::ffi::c_int;
            bmy -= (((bcost as uint32_t) << 29 as ::core::ffi::c_int) as int32_t
                >> 29 as ::core::ffi::c_int) as ::core::ffi::c_int;
            bcost &= !(63 as ::core::ffi::c_int);
            i -= 1;
        }
        bcost >>= 6 as ::core::ffi::c_int;
    }
    if b_refine_qpel == 0
        && ((*h).pixf.mbcmp_unaligned[0 as ::core::ffi::c_int as usize]
            != (*h).pixf.fpelcmp[0 as ::core::ffi::c_int as usize]
            || b_chroma_me != 0)
    {
        bcost = COST_MAX;
        if b_refine_qpel != 0 || -(1 as ::core::ffi::c_int) ^ 1 as ::core::ffi::c_int != odir {
            let mut stride_1: intptr_t = 16 as intptr_t;
            let mut src_0: *mut pixel = (*h).mc.get_ref.expect("non-null function pointer")(
                pix.as_mut_ptr(),
                &mut stride_1,
                &mut *(*m)
                    .p_fref
                    .as_mut_ptr()
                    .offset(0 as ::core::ffi::c_int as isize),
                (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                bmx,
                bmy,
                bw,
                bh,
                &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
            );
            let mut cost_0: ::core::ffi::c_int =
                (*h).pixf.mbcmp_unaligned[i_pixel as usize].expect("non-null function pointer")(
                    (*m).p_fenc[0 as ::core::ffi::c_int as usize],
                    FENC_STRIDE as intptr_t,
                    src_0,
                    stride_1,
                ) + *p_cost_mvx.offset(bmx as isize) as ::core::ffi::c_int
                    + *p_cost_mvy.offset(bmy as isize) as ::core::ffi::c_int;
            if b_chroma_me != 0 && cost_0 < bcost {
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int
                {
                    stride_1 = 16 as intptr_t;
                    src_0 = (*h).mc.get_ref.expect("non-null function pointer")(
                        pix.as_mut_ptr(),
                        &mut stride_1,
                        &mut *(*m)
                            .p_fref
                            .as_mut_ptr()
                            .offset(4 as ::core::ffi::c_int as isize),
                        (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                        bmx,
                        bmy,
                        bw,
                        bh,
                        &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                    );
                    cost_0 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                        .expect("non-null function pointer")(
                        (*m).p_fenc[1 as ::core::ffi::c_int as usize],
                        FENC_STRIDE as intptr_t,
                        src_0,
                        stride_1,
                    );
                    if cost_0 < bcost {
                        stride_1 = 16 as intptr_t;
                        src_0 = (*h).mc.get_ref.expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            &mut stride_1,
                            &mut *(*m)
                                .p_fref
                                .as_mut_ptr()
                                .offset(8 as ::core::ffi::c_int as isize),
                            (*m).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                            bmx,
                            bmy,
                            bw,
                            bh,
                            &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                        );
                        cost_0 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                            .expect("non-null function pointer")(
                            (*m).p_fenc[2 as ::core::ffi::c_int as usize],
                            FENC_STRIDE as intptr_t,
                            src_0,
                            stride_1,
                        );
                    }
                } else {
                    (*h).mc.mc_chroma.expect("non-null function pointer")(
                        pix.as_mut_ptr(),
                        pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                        16 as intptr_t,
                        (*m).p_fref[4 as ::core::ffi::c_int as usize],
                        (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                        bmx,
                        2 as ::core::ffi::c_int * (bmy + mvy_offset) >> chroma_v_shift,
                        bw >> 1 as ::core::ffi::c_int,
                        bh >> chroma_v_shift,
                    );
                    if !(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                        .weightfn
                        .is_null()
                    {
                        (*(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                            .weightfn
                            .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                        .expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            16 as intptr_t,
                            pix.as_mut_ptr(),
                            16 as intptr_t,
                            &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                            bh >> chroma_v_shift,
                        );
                    }
                    cost_0 += (*h).pixf.mbcmp[chromapix as usize]
                        .expect("non-null function pointer")(
                        (*m).p_fenc[1 as ::core::ffi::c_int as usize],
                        FENC_STRIDE as intptr_t,
                        pix.as_mut_ptr(),
                        16 as intptr_t,
                    );
                    if cost_0 < bcost {
                        if !(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                            .weightfn
                            .is_null()
                        {
                            (*(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                                .weightfn
                                .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                            .expect("non-null function pointer")(
                                pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                16 as intptr_t,
                                pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                16 as intptr_t,
                                &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        cost_0 += (*h).pixf.mbcmp[chromapix as usize]
                            .expect("non-null function pointer")(
                            (*m).p_fenc[2 as ::core::ffi::c_int as usize],
                            FENC_STRIDE as intptr_t,
                            pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                            16 as intptr_t,
                        );
                    }
                }
            }
            if cost_0 < bcost {
                bcost = cost_0;
                bmx = bmx;
                bmy = bmy;
                bdir = -(1 as ::core::ffi::c_int);
            }
        }
    }
    if !p_halfpel_thresh.is_null() {
        if bcost * 7 as ::core::ffi::c_int >> 3 as ::core::ffi::c_int > *p_halfpel_thresh {
            (*m).cost = bcost;
            (*m).mv[0 as ::core::ffi::c_int as usize] = bmx as int16_t;
            (*m).mv[1 as ::core::ffi::c_int as usize] = bmy as int16_t;
            return;
        } else if bcost < *p_halfpel_thresh {
            *p_halfpel_thresh = bcost;
        }
    }
    if (*h).mb.i_subpel_refine != 1 as ::core::ffi::c_int {
        bdir = -(1 as ::core::ffi::c_int);
        let mut i_0: ::core::ffi::c_int = qpel_iters;
        while i_0 > 0 as ::core::ffi::c_int {
            if bmy <= (*h).mb.mv_min_spel[1 as ::core::ffi::c_int as usize]
                || bmy >= (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize]
                || bmx <= (*h).mb.mv_min_spel[0 as ::core::ffi::c_int as usize]
                || bmx >= (*h).mb.mv_max_spel[0 as ::core::ffi::c_int as usize]
            {
                break;
            }
            odir = bdir;
            let mut omx_0: ::core::ffi::c_int = bmx;
            let mut omy_0: ::core::ffi::c_int = bmy;
            if b_refine_qpel != 0 || 0 as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int != odir {
                let mut stride_2: intptr_t = 16 as intptr_t;
                let mut src_1: *mut pixel = (*h).mc.get_ref.expect("non-null function pointer")(
                    pix.as_mut_ptr(),
                    &mut stride_2,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize),
                    (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                    omx_0,
                    omy_0 - 1 as ::core::ffi::c_int,
                    bw,
                    bh,
                    &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
                );
                let mut cost_1: ::core::ffi::c_int = (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                    .expect("non-null function pointer")(
                    (*m).p_fenc[0 as ::core::ffi::c_int as usize],
                    FENC_STRIDE as intptr_t,
                    src_1,
                    stride_2,
                ) + *p_cost_mvx.offset(omx_0 as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset((omy_0 - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int;
                if b_chroma_me != 0 && cost_1 < bcost {
                    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                        == CHROMA_444 as ::core::ffi::c_int
                    {
                        stride_2 = 16 as intptr_t;
                        src_1 = (*h).mc.get_ref.expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            &mut stride_2,
                            &mut *(*m)
                                .p_fref
                                .as_mut_ptr()
                                .offset(4 as ::core::ffi::c_int as isize),
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            omx_0,
                            omy_0 - 1 as ::core::ffi::c_int,
                            bw,
                            bh,
                            &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                        );
                        cost_1 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as ::core::ffi::c_int as usize],
                            FENC_STRIDE as intptr_t,
                            src_1,
                            stride_2,
                        );
                        if cost_1 < bcost {
                            stride_2 = 16 as intptr_t;
                            src_1 = (*h).mc.get_ref.expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                &mut stride_2,
                                &mut *(*m)
                                    .p_fref
                                    .as_mut_ptr()
                                    .offset(8 as ::core::ffi::c_int as isize),
                                (*m).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                                omx_0,
                                omy_0 - 1 as ::core::ffi::c_int,
                                bw,
                                bh,
                                &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                            );
                            cost_1 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as ::core::ffi::c_int as usize],
                                FENC_STRIDE as intptr_t,
                                src_1,
                                stride_2,
                            );
                        }
                    } else {
                        (*h).mc.mc_chroma.expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                            16 as intptr_t,
                            (*m).p_fref[4 as ::core::ffi::c_int as usize],
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            omx_0,
                            2 as ::core::ffi::c_int
                                * (omy_0 - 1 as ::core::ffi::c_int + mvy_offset)
                                >> chroma_v_shift,
                            bw >> 1 as ::core::ffi::c_int,
                            bh >> chroma_v_shift,
                        );
                        if !(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                            .weightfn
                            .is_null()
                        {
                            (*(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                                .weightfn
                                .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                            .expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                16 as intptr_t,
                                pix.as_mut_ptr(),
                                16 as intptr_t,
                                &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        cost_1 += (*h).pixf.mbcmp[chromapix as usize]
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as ::core::ffi::c_int as usize],
                            FENC_STRIDE as intptr_t,
                            pix.as_mut_ptr(),
                            16 as intptr_t,
                        );
                        if cost_1 < bcost {
                            if !(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                                .weightfn
                                .is_null()
                            {
                                (*(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                                    .weightfn
                                    .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                                .expect("non-null function pointer")(
                                    pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                    16 as intptr_t,
                                    pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                    16 as intptr_t,
                                    &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_1 += (*h).pixf.mbcmp[chromapix as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as ::core::ffi::c_int as usize],
                                FENC_STRIDE as intptr_t,
                                pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                16 as intptr_t,
                            );
                        }
                    }
                }
                if cost_1 < bcost {
                    bcost = cost_1;
                    bmx = omx_0;
                    bmy = omy_0 - 1 as ::core::ffi::c_int;
                    bdir = 0 as ::core::ffi::c_int;
                }
            }
            if b_refine_qpel != 0 || 1 as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int != odir {
                let mut stride_3: intptr_t = 16 as intptr_t;
                let mut src_2: *mut pixel = (*h).mc.get_ref.expect("non-null function pointer")(
                    pix.as_mut_ptr(),
                    &mut stride_3,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize),
                    (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                    omx_0,
                    omy_0 + 1 as ::core::ffi::c_int,
                    bw,
                    bh,
                    &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
                );
                let mut cost_2: ::core::ffi::c_int = (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                    .expect("non-null function pointer")(
                    (*m).p_fenc[0 as ::core::ffi::c_int as usize],
                    FENC_STRIDE as intptr_t,
                    src_2,
                    stride_3,
                ) + *p_cost_mvx.offset(omx_0 as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset((omy_0 + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int;
                if b_chroma_me != 0 && cost_2 < bcost {
                    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                        == CHROMA_444 as ::core::ffi::c_int
                    {
                        stride_3 = 16 as intptr_t;
                        src_2 = (*h).mc.get_ref.expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            &mut stride_3,
                            &mut *(*m)
                                .p_fref
                                .as_mut_ptr()
                                .offset(4 as ::core::ffi::c_int as isize),
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            omx_0,
                            omy_0 + 1 as ::core::ffi::c_int,
                            bw,
                            bh,
                            &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                        );
                        cost_2 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as ::core::ffi::c_int as usize],
                            FENC_STRIDE as intptr_t,
                            src_2,
                            stride_3,
                        );
                        if cost_2 < bcost {
                            stride_3 = 16 as intptr_t;
                            src_2 = (*h).mc.get_ref.expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                &mut stride_3,
                                &mut *(*m)
                                    .p_fref
                                    .as_mut_ptr()
                                    .offset(8 as ::core::ffi::c_int as isize),
                                (*m).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                                omx_0,
                                omy_0 + 1 as ::core::ffi::c_int,
                                bw,
                                bh,
                                &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                            );
                            cost_2 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as ::core::ffi::c_int as usize],
                                FENC_STRIDE as intptr_t,
                                src_2,
                                stride_3,
                            );
                        }
                    } else {
                        (*h).mc.mc_chroma.expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                            16 as intptr_t,
                            (*m).p_fref[4 as ::core::ffi::c_int as usize],
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            omx_0,
                            2 as ::core::ffi::c_int
                                * (omy_0 + 1 as ::core::ffi::c_int + mvy_offset)
                                >> chroma_v_shift,
                            bw >> 1 as ::core::ffi::c_int,
                            bh >> chroma_v_shift,
                        );
                        if !(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                            .weightfn
                            .is_null()
                        {
                            (*(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                                .weightfn
                                .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                            .expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                16 as intptr_t,
                                pix.as_mut_ptr(),
                                16 as intptr_t,
                                &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        cost_2 += (*h).pixf.mbcmp[chromapix as usize]
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as ::core::ffi::c_int as usize],
                            FENC_STRIDE as intptr_t,
                            pix.as_mut_ptr(),
                            16 as intptr_t,
                        );
                        if cost_2 < bcost {
                            if !(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                                .weightfn
                                .is_null()
                            {
                                (*(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                                    .weightfn
                                    .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                                .expect("non-null function pointer")(
                                    pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                    16 as intptr_t,
                                    pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                    16 as intptr_t,
                                    &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_2 += (*h).pixf.mbcmp[chromapix as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as ::core::ffi::c_int as usize],
                                FENC_STRIDE as intptr_t,
                                pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                16 as intptr_t,
                            );
                        }
                    }
                }
                if cost_2 < bcost {
                    bcost = cost_2;
                    bmx = omx_0;
                    bmy = omy_0 + 1 as ::core::ffi::c_int;
                    bdir = 1 as ::core::ffi::c_int;
                }
            }
            if b_refine_qpel != 0 || 2 as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int != odir {
                let mut stride_4: intptr_t = 16 as intptr_t;
                let mut src_3: *mut pixel = (*h).mc.get_ref.expect("non-null function pointer")(
                    pix.as_mut_ptr(),
                    &mut stride_4,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize),
                    (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                    omx_0 - 1 as ::core::ffi::c_int,
                    omy_0,
                    bw,
                    bh,
                    &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
                );
                let mut cost_3: ::core::ffi::c_int = (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                    .expect("non-null function pointer")(
                    (*m).p_fenc[0 as ::core::ffi::c_int as usize],
                    FENC_STRIDE as intptr_t,
                    src_3,
                    stride_4,
                ) + *p_cost_mvx
                    .offset((omx_0 - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset(omy_0 as isize) as ::core::ffi::c_int;
                if b_chroma_me != 0 && cost_3 < bcost {
                    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                        == CHROMA_444 as ::core::ffi::c_int
                    {
                        stride_4 = 16 as intptr_t;
                        src_3 = (*h).mc.get_ref.expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            &mut stride_4,
                            &mut *(*m)
                                .p_fref
                                .as_mut_ptr()
                                .offset(4 as ::core::ffi::c_int as isize),
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            omx_0 - 1 as ::core::ffi::c_int,
                            omy_0,
                            bw,
                            bh,
                            &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                        );
                        cost_3 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as ::core::ffi::c_int as usize],
                            FENC_STRIDE as intptr_t,
                            src_3,
                            stride_4,
                        );
                        if cost_3 < bcost {
                            stride_4 = 16 as intptr_t;
                            src_3 = (*h).mc.get_ref.expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                &mut stride_4,
                                &mut *(*m)
                                    .p_fref
                                    .as_mut_ptr()
                                    .offset(8 as ::core::ffi::c_int as isize),
                                (*m).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                                omx_0 - 1 as ::core::ffi::c_int,
                                omy_0,
                                bw,
                                bh,
                                &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                            );
                            cost_3 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as ::core::ffi::c_int as usize],
                                FENC_STRIDE as intptr_t,
                                src_3,
                                stride_4,
                            );
                        }
                    } else {
                        (*h).mc.mc_chroma.expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                            16 as intptr_t,
                            (*m).p_fref[4 as ::core::ffi::c_int as usize],
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            omx_0 - 1 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int * (omy_0 + mvy_offset) >> chroma_v_shift,
                            bw >> 1 as ::core::ffi::c_int,
                            bh >> chroma_v_shift,
                        );
                        if !(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                            .weightfn
                            .is_null()
                        {
                            (*(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                                .weightfn
                                .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                            .expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                16 as intptr_t,
                                pix.as_mut_ptr(),
                                16 as intptr_t,
                                &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        cost_3 += (*h).pixf.mbcmp[chromapix as usize]
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as ::core::ffi::c_int as usize],
                            FENC_STRIDE as intptr_t,
                            pix.as_mut_ptr(),
                            16 as intptr_t,
                        );
                        if cost_3 < bcost {
                            if !(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                                .weightfn
                                .is_null()
                            {
                                (*(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                                    .weightfn
                                    .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                                .expect("non-null function pointer")(
                                    pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                    16 as intptr_t,
                                    pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                    16 as intptr_t,
                                    &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_3 += (*h).pixf.mbcmp[chromapix as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as ::core::ffi::c_int as usize],
                                FENC_STRIDE as intptr_t,
                                pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                16 as intptr_t,
                            );
                        }
                    }
                }
                if cost_3 < bcost {
                    bcost = cost_3;
                    bmx = omx_0 - 1 as ::core::ffi::c_int;
                    bmy = omy_0;
                    bdir = 2 as ::core::ffi::c_int;
                }
            }
            if b_refine_qpel != 0 || 3 as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int != odir {
                let mut stride_5: intptr_t = 16 as intptr_t;
                let mut src_4: *mut pixel = (*h).mc.get_ref.expect("non-null function pointer")(
                    pix.as_mut_ptr(),
                    &mut stride_5,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize),
                    (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                    omx_0 + 1 as ::core::ffi::c_int,
                    omy_0,
                    bw,
                    bh,
                    &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
                );
                let mut cost_4: ::core::ffi::c_int = (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                    .expect("non-null function pointer")(
                    (*m).p_fenc[0 as ::core::ffi::c_int as usize],
                    FENC_STRIDE as intptr_t,
                    src_4,
                    stride_5,
                ) + *p_cost_mvx
                    .offset((omx_0 + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    + *p_cost_mvy.offset(omy_0 as isize) as ::core::ffi::c_int;
                if b_chroma_me != 0 && cost_4 < bcost {
                    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                        == CHROMA_444 as ::core::ffi::c_int
                    {
                        stride_5 = 16 as intptr_t;
                        src_4 = (*h).mc.get_ref.expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            &mut stride_5,
                            &mut *(*m)
                                .p_fref
                                .as_mut_ptr()
                                .offset(4 as ::core::ffi::c_int as isize),
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            omx_0 + 1 as ::core::ffi::c_int,
                            omy_0,
                            bw,
                            bh,
                            &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                        );
                        cost_4 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as ::core::ffi::c_int as usize],
                            FENC_STRIDE as intptr_t,
                            src_4,
                            stride_5,
                        );
                        if cost_4 < bcost {
                            stride_5 = 16 as intptr_t;
                            src_4 = (*h).mc.get_ref.expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                &mut stride_5,
                                &mut *(*m)
                                    .p_fref
                                    .as_mut_ptr()
                                    .offset(8 as ::core::ffi::c_int as isize),
                                (*m).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                                omx_0 + 1 as ::core::ffi::c_int,
                                omy_0,
                                bw,
                                bh,
                                &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                            );
                            cost_4 += (*h).pixf.mbcmp_unaligned[i_pixel as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as ::core::ffi::c_int as usize],
                                FENC_STRIDE as intptr_t,
                                src_4,
                                stride_5,
                            );
                        }
                    } else {
                        (*h).mc.mc_chroma.expect("non-null function pointer")(
                            pix.as_mut_ptr(),
                            pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                            16 as intptr_t,
                            (*m).p_fref[4 as ::core::ffi::c_int as usize],
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            omx_0 + 1 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int * (omy_0 + mvy_offset) >> chroma_v_shift,
                            bw >> 1 as ::core::ffi::c_int,
                            bh >> chroma_v_shift,
                        );
                        if !(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                            .weightfn
                            .is_null()
                        {
                            (*(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                                .weightfn
                                .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                            .expect("non-null function pointer")(
                                pix.as_mut_ptr(),
                                16 as intptr_t,
                                pix.as_mut_ptr(),
                                16 as intptr_t,
                                &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        cost_4 += (*h).pixf.mbcmp[chromapix as usize]
                            .expect("non-null function pointer")(
                            (*m).p_fenc[1 as ::core::ffi::c_int as usize],
                            FENC_STRIDE as intptr_t,
                            pix.as_mut_ptr(),
                            16 as intptr_t,
                        );
                        if cost_4 < bcost {
                            if !(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                                .weightfn
                                .is_null()
                            {
                                (*(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                                    .weightfn
                                    .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                                .expect("non-null function pointer")(
                                    pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                    16 as intptr_t,
                                    pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                    16 as intptr_t,
                                    &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                                    bh >> chroma_v_shift,
                                );
                            }
                            cost_4 += (*h).pixf.mbcmp[chromapix as usize]
                                .expect("non-null function pointer")(
                                (*m).p_fenc[2 as ::core::ffi::c_int as usize],
                                FENC_STRIDE as intptr_t,
                                pix.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
                                16 as intptr_t,
                            );
                        }
                    }
                }
                if cost_4 < bcost {
                    bcost = cost_4;
                    bmx = omx_0 + 1 as ::core::ffi::c_int;
                    bmy = omy_0;
                    bdir = 3 as ::core::ffi::c_int;
                }
            }
            if (bmx == omx_0) as ::core::ffi::c_int & (bmy == omy_0) as ::core::ffi::c_int != 0 {
                break;
            }
            i_0 -= 1;
        }
    } else if bmy > (*h).mb.mv_min_spel[1 as ::core::ffi::c_int as usize]
        && bmy < (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize]
        && bmx > (*h).mb.mv_min_spel[0 as ::core::ffi::c_int as usize]
        && bmx < (*h).mb.mv_max_spel[0 as ::core::ffi::c_int as usize]
    {
        let mut omx_1: ::core::ffi::c_int = bmx;
        let mut omy_1: ::core::ffi::c_int = bmy;
        (*h).mc.mc_luma.expect("non-null function pointer")(
            pix.as_mut_ptr(),
            64 as intptr_t,
            (*m).p_fref.as_mut_ptr(),
            (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
            omx_1,
            omy_1 - 1 as ::core::ffi::c_int,
            bw,
            bh,
            &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
        );
        (*h).mc.mc_luma.expect("non-null function pointer")(
            pix.as_mut_ptr().offset(16 as ::core::ffi::c_int as isize),
            64 as intptr_t,
            (*m).p_fref.as_mut_ptr(),
            (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
            omx_1,
            omy_1 + 1 as ::core::ffi::c_int,
            bw,
            bh,
            &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
        );
        (*h).mc.mc_luma.expect("non-null function pointer")(
            pix.as_mut_ptr().offset(32 as ::core::ffi::c_int as isize),
            64 as intptr_t,
            (*m).p_fref.as_mut_ptr(),
            (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
            omx_1 - 1 as ::core::ffi::c_int,
            omy_1,
            bw,
            bh,
            &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
        );
        (*h).mc.mc_luma.expect("non-null function pointer")(
            pix.as_mut_ptr().offset(48 as ::core::ffi::c_int as isize),
            64 as intptr_t,
            (*m).p_fref.as_mut_ptr(),
            (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
            omx_1 + 1 as ::core::ffi::c_int,
            omy_1,
            bw,
            bh,
            &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
        );
        (*h).pixf.fpelcmp_x4[i_pixel as usize].expect("non-null function pointer")(
            (*m).p_fenc[0 as ::core::ffi::c_int as usize],
            pix.as_mut_ptr(),
            pix.as_mut_ptr().offset(16 as ::core::ffi::c_int as isize),
            pix.as_mut_ptr().offset(32 as ::core::ffi::c_int as isize),
            pix.as_mut_ptr().offset(48 as ::core::ffi::c_int as isize),
            64 as intptr_t,
            costs.as_mut_ptr(),
        );
        costs[0 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(omx_1 as isize)
            as ::core::ffi::c_int
            + *p_cost_mvy.offset((omy_1 - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
        costs[1 as ::core::ffi::c_int as usize] += *p_cost_mvx.offset(omx_1 as isize)
            as ::core::ffi::c_int
            + *p_cost_mvy.offset((omy_1 + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int;
        costs[2 as ::core::ffi::c_int as usize] +=
            *p_cost_mvx.offset((omx_1 - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *p_cost_mvy.offset(omy_1 as isize) as ::core::ffi::c_int;
        costs[3 as ::core::ffi::c_int as usize] +=
            *p_cost_mvx.offset((omx_1 + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                + *p_cost_mvy.offset(omy_1 as isize) as ::core::ffi::c_int;
        bcost <<= 4 as ::core::ffi::c_int;
        if ((costs[0 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int)
            < bcost
        {
            bcost = (costs[0 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int;
        }
        if ((costs[1 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
            + 3 as ::core::ffi::c_int)
            < bcost
        {
            bcost = (costs[1 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 3 as ::core::ffi::c_int;
        }
        if ((costs[2 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
            + 4 as ::core::ffi::c_int)
            < bcost
        {
            bcost = (costs[2 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 4 as ::core::ffi::c_int;
        }
        if ((costs[3 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
            + 12 as ::core::ffi::c_int)
            < bcost
        {
            bcost = (costs[3 as ::core::ffi::c_int as usize] << 4 as ::core::ffi::c_int)
                + 12 as ::core::ffi::c_int;
        }
        bmx -= (((bcost as uint32_t) << 28 as ::core::ffi::c_int) as int32_t
            >> 30 as ::core::ffi::c_int) as ::core::ffi::c_int;
        bmy -= (((bcost as uint32_t) << 30 as ::core::ffi::c_int) as int32_t
            >> 30 as ::core::ffi::c_int) as ::core::ffi::c_int;
        bcost >>= 4 as ::core::ffi::c_int;
    }
    (*m).cost = bcost;
    (*m).mv[0 as ::core::ffi::c_int as usize] = bmx as int16_t;
    (*m).mv[1 as ::core::ffi::c_int as usize] = bmy as int16_t;
    (*m).cost_mv = *p_cost_mvx.offset(bmx as isize) as ::core::ffi::c_int
        + *p_cost_mvy.offset(bmy as isize) as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1025:5"]
pub static mut x264_10_iter_kludge: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline(always)]
#[c2rust::src_loc = "1027:1"]
unsafe extern "C" fn me_refine_bidir(
    mut h: *mut x264_t,
    mut m0: *mut x264_me_t,
    mut m1: *mut x264_me_t,
    mut i_weight: ::core::ffi::c_int,
    mut i8: ::core::ffi::c_int,
    mut i_lambda2: ::core::ffi::c_int,
    mut rd: ::core::ffi::c_int,
) {
    let mut x: ::core::ffi::c_int = i8 & 1 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = i8 >> 1 as ::core::ffi::c_int;
    let mut s8: ::core::ffi::c_int =
        X264_SCAN8_0 + 2 as ::core::ffi::c_int * x + 16 as ::core::ffi::c_int * y;
    let mut cache0_mv: *mut int16_t = (*(*(*h)
        .mb
        .cache
        .mv
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
    .as_mut_ptr()
    .offset(s8 as isize))
    .as_mut_ptr();
    let mut cache1_mv: *mut int16_t = (*(*(*h)
        .mb
        .cache
        .mv
        .as_mut_ptr()
        .offset(1 as ::core::ffi::c_int as isize))
    .as_mut_ptr()
    .offset(s8 as isize))
    .as_mut_ptr();
    let i_pixel: ::core::ffi::c_int = (*m0).i_pixel;
    let bw: ::core::ffi::c_int = x264_pixel_size[i_pixel as usize].w as ::core::ffi::c_int;
    let bh: ::core::ffi::c_int = x264_pixel_size[i_pixel as usize].h as ::core::ffi::c_int;
    let mut pixy_buf: [[[pixel; 256]; 9]; 2] = [[[0; 256]; 9]; 2];
    let mut pixu_buf: [[[pixel; 256]; 9]; 2] = [[[0; 256]; 9]; 2];
    let mut pixv_buf: [[[pixel; 256]; 9]; 2] = [[[0; 256]; 9]; 2];
    let mut src: [[[*mut pixel; 9]; 2]; 3] = [[[0 as *mut pixel; 9]; 2]; 3];
    let mut chromapix: ::core::ffi::c_int =
        (*h).luma2chroma_pixel[i_pixel as usize] as ::core::ffi::c_int;
    let mut chroma_v_shift: ::core::ffi::c_int = (*h).mb.chroma_v_shift;
    let mut chroma_x: ::core::ffi::c_int = (8 as ::core::ffi::c_int >> (*h).mb.chroma_h_shift) * x;
    let mut chroma_y: ::core::ffi::c_int = (8 as ::core::ffi::c_int >> chroma_v_shift) * y;
    let mut pix: *mut pixel = &mut *(*(*h)
        .mb
        .pic
        .p_fdec
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
    .offset((8 as ::core::ffi::c_int * x + 8 as ::core::ffi::c_int * y * FDEC_STRIDE) as isize)
        as *mut pixel;
    let mut pixu: *mut pixel = if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
        .offset((chroma_x + chroma_y * FDEC_STRIDE) as isize) as *mut pixel
    } else {
        0 as *mut pixel
    };
    let mut pixv: *mut pixel = if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(2 as ::core::ffi::c_int as isize))
        .offset((chroma_x + chroma_y * FDEC_STRIDE) as isize) as *mut pixel
    } else {
        0 as *mut pixel
    };
    let mut ref0: ::core::ffi::c_int =
        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize][s8 as usize] as ::core::ffi::c_int;
    let mut ref1: ::core::ffi::c_int =
        (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize][s8 as usize] as ::core::ffi::c_int;
    let mv0y_offset: ::core::ffi::c_int = if chroma_v_shift & (*h).mb.b_interlaced & ref0 != 0 {
        ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let mv1y_offset: ::core::ffi::c_int = if chroma_v_shift & (*h).mb.b_interlaced & ref1 != 0 {
        ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let mut stride: [[[intptr_t; 9]; 2]; 3] = [[[0; 9]; 2]; 3];
    let mut bm0x: ::core::ffi::c_int =
        (*m0).mv[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    let mut bm0y: ::core::ffi::c_int =
        (*m0).mv[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    let mut bm1x: ::core::ffi::c_int =
        (*m1).mv[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    let mut bm1y: ::core::ffi::c_int =
        (*m1).mv[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    let mut bcost: ::core::ffi::c_int = COST_MAX;
    let mut mc_list0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut mc_list1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut bcostrd: uint64_t = COST_MAX64 as uint64_t;
    let mut amvd: uint16_t = 0;
    let mut visited: [[[uint8_t; 8]; 8]; 8] = [[[0; 8]; 8]; 8];
    static mut dia4d: [[int8_t; 4]; 33] = [
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
        ],
        [
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
        ],
        [
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
        [
            1 as ::core::ffi::c_int as int8_t,
            0 as ::core::ffi::c_int as int8_t,
            -(1 as ::core::ffi::c_int) as int8_t,
            0 as ::core::ffi::c_int as int8_t,
        ],
    ];
    if bm0y < (*h).mb.mv_min_spel[1 as ::core::ffi::c_int as usize] + 8 as ::core::ffi::c_int
        || bm1y < (*h).mb.mv_min_spel[1 as ::core::ffi::c_int as usize] + 8 as ::core::ffi::c_int
        || bm0y > (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize] - 8 as ::core::ffi::c_int
        || bm1y > (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize] - 8 as ::core::ffi::c_int
        || bm0x < (*h).mb.mv_min_spel[0 as ::core::ffi::c_int as usize] + 8 as ::core::ffi::c_int
        || bm1x < (*h).mb.mv_min_spel[0 as ::core::ffi::c_int as usize] + 8 as ::core::ffi::c_int
        || bm0x > (*h).mb.mv_max_spel[0 as ::core::ffi::c_int as usize] - 8 as ::core::ffi::c_int
        || bm1x > (*h).mb.mv_max_spel[0 as ::core::ffi::c_int as usize] - 8 as ::core::ffi::c_int
    {
        return;
    }
    if rd != 0
        && (*m0).i_pixel != PIXEL_16x16 as ::core::ffi::c_int
        && i8 != 0 as ::core::ffi::c_int
    {
        x264_10_mb_predict_mv(
            h,
            0 as ::core::ffi::c_int,
            i8 << 2 as ::core::ffi::c_int,
            bw >> 2 as ::core::ffi::c_int,
            (*m0).mvp.as_mut_ptr(),
        );
        x264_10_mb_predict_mv(
            h,
            1 as ::core::ffi::c_int,
            i8 << 2 as ::core::ffi::c_int,
            bw >> 2 as ::core::ffi::c_int,
            (*m1).mvp.as_mut_ptr(),
        );
    }
    let mut p_cost_m0x: *const uint16_t = (*m0)
        .p_cost_mv
        .offset(-((*m0).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize));
    let mut p_cost_m0y: *const uint16_t = (*m0)
        .p_cost_mv
        .offset(-((*m0).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize));
    let mut p_cost_m1x: *const uint16_t = (*m1)
        .p_cost_mv
        .offset(-((*m1).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize));
    let mut p_cost_m1y: *const uint16_t = (*m1)
        .p_cost_mv
        .offset(-((*m1).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int as isize));
    (*h).mc.memzero_aligned.expect("non-null function pointer")(
        visited.as_mut_ptr() as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[[[uint8_t; 8]; 8]; 8]>() as size_t,
    );
    let mut pass: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while pass < 8 as ::core::ffi::c_int {
        let mut bestj: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if mc_list0 != 0 {
            let mut j: ::core::ffi::c_int = x264_10_iter_kludge;
            while j < 9 as ::core::ffi::c_int {
                let mut m: *mut x264_me_t = m0;
                let mut i: ::core::ffi::c_int = 4 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int
                        * square1[j as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                    + square1[j as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                let mut mvx: ::core::ffi::c_int = bm0x
                    + square1[j as usize][0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                let mut mvy: ::core::ffi::c_int = bm0y
                    + square1[j as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                stride[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                    [i as usize] = bw as intptr_t;
                src[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                    [i as usize] = (*h).mc.get_ref.expect("non-null function pointer")(
                    (*(*pixy_buf
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(i as isize))
                    .as_mut_ptr(),
                    &mut *(*(*stride.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(i as isize),
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize),
                    (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                    mvx,
                    mvy,
                    bw,
                    bh,
                    x264_zero.as_mut_ptr() as *const x264_weight_t,
                );
                if rd != 0 {
                    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                        == CHROMA_444 as ::core::ffi::c_int
                    {
                        stride[1 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize][i as usize] = bw as intptr_t;
                        src[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                            [i as usize] = (*h).mc.get_ref.expect("non-null function pointer")(
                            (*(*pixu_buf
                                .as_mut_ptr()
                                .offset(0 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i as isize))
                            .as_mut_ptr(),
                            &mut *(*(*stride
                                .as_mut_ptr()
                                .offset(1 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i as isize),
                            &mut *(*m)
                                .p_fref
                                .as_mut_ptr()
                                .offset(4 as ::core::ffi::c_int as isize),
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            mvx,
                            mvy,
                            bw,
                            bh,
                            x264_zero.as_mut_ptr() as *const x264_weight_t,
                        );
                        stride[2 as ::core::ffi::c_int as usize]
                            [0 as ::core::ffi::c_int as usize][i as usize] = bw as intptr_t;
                        src[2 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                            [i as usize] = (*h).mc.get_ref.expect("non-null function pointer")(
                            (*(*pixv_buf
                                .as_mut_ptr()
                                .offset(0 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i as isize))
                            .as_mut_ptr(),
                            &mut *(*(*stride
                                .as_mut_ptr()
                                .offset(2 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(0 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i as isize),
                            &mut *(*m)
                                .p_fref
                                .as_mut_ptr()
                                .offset(8 as ::core::ffi::c_int as isize),
                            (*m).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                            mvx,
                            mvy,
                            bw,
                            bh,
                            x264_zero.as_mut_ptr() as *const x264_weight_t,
                        );
                    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                        (*h).mc.mc_chroma.expect("non-null function pointer")(
                            (*(*pixu_buf
                                .as_mut_ptr()
                                .offset(0 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i as isize))
                            .as_mut_ptr(),
                            (*(*pixv_buf
                                .as_mut_ptr()
                                .offset(0 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i as isize))
                            .as_mut_ptr(),
                            8 as intptr_t,
                            (*m).p_fref[4 as ::core::ffi::c_int as usize],
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            mvx,
                            2 as ::core::ffi::c_int * (mvy + mv0y_offset) >> chroma_v_shift,
                            bw >> 1 as ::core::ffi::c_int,
                            bh >> chroma_v_shift,
                        );
                    }
                }
                j += 1;
            }
        }
        if mc_list1 != 0 {
            let mut j_0: ::core::ffi::c_int = x264_10_iter_kludge;
            while j_0 < 9 as ::core::ffi::c_int {
                let mut m_0: *mut x264_me_t = m1;
                let mut i_0: ::core::ffi::c_int = 4 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int
                        * square1[j_0 as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                    + square1[j_0 as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                let mut mvx_0: ::core::ffi::c_int = bm1x
                    + square1[j_0 as usize][0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                let mut mvy_0: ::core::ffi::c_int = bm1y
                    + square1[j_0 as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                stride[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                    [i_0 as usize] = bw as intptr_t;
                src[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                    [i_0 as usize] = (*h).mc.get_ref.expect("non-null function pointer")(
                    (*(*pixy_buf
                        .as_mut_ptr()
                        .offset(1 as ::core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(i_0 as isize))
                    .as_mut_ptr(),
                    &mut *(*(*stride.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(1 as ::core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(i_0 as isize),
                    &mut *(*m_0)
                        .p_fref
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize),
                    (*m_0).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                    mvx_0,
                    mvy_0,
                    bw,
                    bh,
                    x264_zero.as_mut_ptr() as *const x264_weight_t,
                );
                if rd != 0 {
                    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                        == CHROMA_444 as ::core::ffi::c_int
                    {
                        stride[1 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize][i_0 as usize] = bw as intptr_t;
                        src[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                            [i_0 as usize] = (*h).mc.get_ref.expect("non-null function pointer")(
                            (*(*pixu_buf
                                .as_mut_ptr()
                                .offset(1 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i_0 as isize))
                            .as_mut_ptr(),
                            &mut *(*(*stride
                                .as_mut_ptr()
                                .offset(1 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(1 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i_0 as isize),
                            &mut *(*m_0)
                                .p_fref
                                .as_mut_ptr()
                                .offset(4 as ::core::ffi::c_int as isize),
                            (*m_0).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            mvx_0,
                            mvy_0,
                            bw,
                            bh,
                            x264_zero.as_mut_ptr() as *const x264_weight_t,
                        );
                        stride[2 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize][i_0 as usize] = bw as intptr_t;
                        src[2 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                            [i_0 as usize] = (*h).mc.get_ref.expect("non-null function pointer")(
                            (*(*pixv_buf
                                .as_mut_ptr()
                                .offset(1 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i_0 as isize))
                            .as_mut_ptr(),
                            &mut *(*(*stride
                                .as_mut_ptr()
                                .offset(2 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(1 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i_0 as isize),
                            &mut *(*m_0)
                                .p_fref
                                .as_mut_ptr()
                                .offset(8 as ::core::ffi::c_int as isize),
                            (*m_0).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                            mvx_0,
                            mvy_0,
                            bw,
                            bh,
                            x264_zero.as_mut_ptr() as *const x264_weight_t,
                        );
                    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                        (*h).mc.mc_chroma.expect("non-null function pointer")(
                            (*(*pixu_buf
                                .as_mut_ptr()
                                .offset(1 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i_0 as isize))
                            .as_mut_ptr(),
                            (*(*pixv_buf
                                .as_mut_ptr()
                                .offset(1 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(i_0 as isize))
                            .as_mut_ptr(),
                            8 as intptr_t,
                            (*m_0).p_fref[4 as ::core::ffi::c_int as usize],
                            (*m_0).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            mvx_0,
                            2 as ::core::ffi::c_int * (mvy_0 + mv1y_offset) >> chroma_v_shift,
                            bw >> 1 as ::core::ffi::c_int,
                            bh >> chroma_v_shift,
                        );
                    }
                }
                j_0 += 1;
            }
        }
        let mut j_1: ::core::ffi::c_int = (pass != 0) as ::core::ffi::c_int;
        while j_1 < 33 as ::core::ffi::c_int {
            let mut m0x: ::core::ffi::c_int =
                dia4d[j_1 as usize][0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int + bm0x;
            let mut m0y: ::core::ffi::c_int =
                dia4d[j_1 as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int + bm0y;
            let mut m1x: ::core::ffi::c_int =
                dia4d[j_1 as usize][2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int + bm1x;
            let mut m1y: ::core::ffi::c_int =
                dia4d[j_1 as usize][3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int + bm1y;
            if pass == 0
                || visited[(m0x & 7 as ::core::ffi::c_int) as usize]
                    [(m0y & 7 as ::core::ffi::c_int) as usize]
                    [(m1x & 7 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << (m1y & 7 as ::core::ffi::c_int)
                    == 0
            {
                let mut i0: ::core::ffi::c_int = 4 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int
                        * dia4d[j_1 as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                    + dia4d[j_1 as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                let mut i1: ::core::ffi::c_int = 4 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int
                        * dia4d[j_1 as usize][2 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                    + dia4d[j_1 as usize][3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                visited[(m0x & 7 as ::core::ffi::c_int) as usize]
                    [(m0y & 7 as ::core::ffi::c_int) as usize]
                    [(m1x & 7 as ::core::ffi::c_int) as usize] = (visited
                    [(m0x & 7 as ::core::ffi::c_int) as usize]
                    [(m0y & 7 as ::core::ffi::c_int) as usize]
                    [(m1x & 7 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << (m1y & 7 as ::core::ffi::c_int))
                    as uint8_t;
                (*h).mc.avg[i_pixel as usize].expect("non-null function pointer")(
                    pix,
                    FDEC_STRIDE as intptr_t,
                    src[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                        [i0 as usize],
                    stride[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]
                        [i0 as usize],
                    src[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                        [i1 as usize],
                    stride[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                        [i1 as usize],
                    i_weight,
                );
                let mut cost: ::core::ffi::c_int =
                    (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
                        (*m0).p_fenc[0 as ::core::ffi::c_int as usize],
                        FENC_STRIDE as intptr_t,
                        pix,
                        FDEC_STRIDE as intptr_t,
                    ) + *p_cost_m0x.offset(m0x as isize) as ::core::ffi::c_int
                        + *p_cost_m0y.offset(m0y as isize) as ::core::ffi::c_int
                        + *p_cost_m1x.offset(m1x as isize) as ::core::ffi::c_int
                        + *p_cost_m1y.offset(m1y as isize) as ::core::ffi::c_int;
                if rd != 0 {
                    if cost < bcost + (bcost >> 4 as ::core::ffi::c_int) {
                        bcost = if cost < bcost { cost } else { bcost };
                        (*(cache0_mv as *mut x264_union32_t)).i = pack16to32_mask(m0x, m0y);
                        (*(cache1_mv as *mut x264_union32_t)).i = pack16to32_mask(m1x, m1y);
                        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                            == CHROMA_444 as ::core::ffi::c_int
                        {
                            (*h).mc.avg[i_pixel as usize].expect("non-null function pointer")(
                                pixu,
                                FDEC_STRIDE as intptr_t,
                                src[1 as ::core::ffi::c_int as usize]
                                    [0 as ::core::ffi::c_int as usize]
                                    [i0 as usize],
                                stride[1 as ::core::ffi::c_int as usize]
                                    [0 as ::core::ffi::c_int as usize]
                                    [i0 as usize],
                                src[1 as ::core::ffi::c_int as usize]
                                    [1 as ::core::ffi::c_int as usize]
                                    [i1 as usize],
                                stride[1 as ::core::ffi::c_int as usize]
                                    [1 as ::core::ffi::c_int as usize]
                                    [i1 as usize],
                                i_weight,
                            );
                            (*h).mc.avg[i_pixel as usize].expect("non-null function pointer")(
                                pixv,
                                FDEC_STRIDE as intptr_t,
                                src[2 as ::core::ffi::c_int as usize]
                                    [0 as ::core::ffi::c_int as usize]
                                    [i0 as usize],
                                stride[2 as ::core::ffi::c_int as usize]
                                    [0 as ::core::ffi::c_int as usize]
                                    [i0 as usize],
                                src[2 as ::core::ffi::c_int as usize]
                                    [1 as ::core::ffi::c_int as usize]
                                    [i1 as usize],
                                stride[2 as ::core::ffi::c_int as usize]
                                    [1 as ::core::ffi::c_int as usize]
                                    [i1 as usize],
                                i_weight,
                            );
                        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                            (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
                                pixu,
                                FDEC_STRIDE as intptr_t,
                                (*(*pixu_buf
                                    .as_mut_ptr()
                                    .offset(0 as ::core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(i0 as isize))
                                .as_mut_ptr(),
                                8 as intptr_t,
                                (*(*pixu_buf
                                    .as_mut_ptr()
                                    .offset(1 as ::core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(i1 as isize))
                                .as_mut_ptr(),
                                8 as intptr_t,
                                i_weight,
                            );
                            (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
                                pixv,
                                FDEC_STRIDE as intptr_t,
                                (*(*pixv_buf
                                    .as_mut_ptr()
                                    .offset(0 as ::core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(i0 as isize))
                                .as_mut_ptr(),
                                8 as intptr_t,
                                (*(*pixv_buf
                                    .as_mut_ptr()
                                    .offset(1 as ::core::ffi::c_int as isize))
                                .as_mut_ptr()
                                .offset(i1 as isize))
                                .as_mut_ptr(),
                                8 as intptr_t,
                                i_weight,
                            );
                        }
                        let mut costrd: uint64_t = x264_10_rd_cost_part(
                            h,
                            i_lambda2,
                            i8 * 4 as ::core::ffi::c_int,
                            (*m0).i_pixel,
                        );
                        if costrd < bcostrd {
                            bcostrd = costrd;
                            bestj = j_1;
                        }
                    }
                } else if cost < bcost {
                    bcost = cost;
                    bestj = j_1;
                }
            }
            j_1 += 1;
        }
        if bestj == 0 {
            break;
        }
        bm0x += dia4d[bestj as usize][0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        bm0y += dia4d[bestj as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        bm1x += dia4d[bestj as usize][2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        bm1y += dia4d[bestj as usize][3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        mc_list0 = (*(&*(*dia4d.as_ptr().offset(bestj as isize))
            .as_ptr()
            .offset(0 as ::core::ffi::c_int as isize) as *const int8_t
            as *mut x264_union16_t))
            .i as ::core::ffi::c_int;
        mc_list1 = (*(&*(*dia4d.as_ptr().offset(bestj as isize))
            .as_ptr()
            .offset(2 as ::core::ffi::c_int as isize) as *const int8_t
            as *mut x264_union16_t))
            .i as ::core::ffi::c_int;
        pass += 1;
    }
    if rd != 0 {
        x264_macroblock_cache_mv(
            h,
            2 as ::core::ffi::c_int * x,
            2 as ::core::ffi::c_int * y,
            bw >> 2 as ::core::ffi::c_int,
            bh >> 2 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            pack16to32_mask(bm0x, bm0y),
        );
        amvd = pack8to16(
            (if abs(bm0x - (*m0).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                < 33 as ::core::ffi::c_int
            {
                abs(bm0x - (*m0).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
            } else {
                33 as ::core::ffi::c_int
            }) as uint32_t,
            (if abs(bm0y - (*m0).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                < 33 as ::core::ffi::c_int
            {
                abs(bm0y - (*m0).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
            } else {
                33 as ::core::ffi::c_int
            }) as uint32_t,
        ) as uint16_t;
        x264_macroblock_cache_mvd(
            h,
            2 as ::core::ffi::c_int * x,
            2 as ::core::ffi::c_int * y,
            bw >> 2 as ::core::ffi::c_int,
            bh >> 2 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            amvd,
        );
        x264_macroblock_cache_mv(
            h,
            2 as ::core::ffi::c_int * x,
            2 as ::core::ffi::c_int * y,
            bw >> 2 as ::core::ffi::c_int,
            bh >> 2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            pack16to32_mask(bm1x, bm1y),
        );
        amvd = pack8to16(
            (if abs(bm1x - (*m1).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                < 33 as ::core::ffi::c_int
            {
                abs(bm1x - (*m1).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
            } else {
                33 as ::core::ffi::c_int
            }) as uint32_t,
            (if abs(bm1y - (*m1).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                < 33 as ::core::ffi::c_int
            {
                abs(bm1y - (*m1).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
            } else {
                33 as ::core::ffi::c_int
            }) as uint32_t,
        ) as uint16_t;
        x264_macroblock_cache_mvd(
            h,
            2 as ::core::ffi::c_int * x,
            2 as ::core::ffi::c_int * y,
            bw >> 2 as ::core::ffi::c_int,
            bh >> 2 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            amvd,
        );
    }
    (*m0).mv[0 as ::core::ffi::c_int as usize] = bm0x as int16_t;
    (*m0).mv[1 as ::core::ffi::c_int as usize] = bm0y as int16_t;
    (*m1).mv[0 as ::core::ffi::c_int as usize] = bm1x as int16_t;
    (*m1).mv[1 as ::core::ffi::c_int as usize] = bm1y as int16_t;
}
#[no_mangle]
#[c2rust::src_loc = "1180:1"]
pub unsafe extern "C" fn x264_10_me_refine_bidir_satd(
    mut h: *mut x264_t,
    mut m0: *mut x264_me_t,
    mut m1: *mut x264_me_t,
    mut i_weight: ::core::ffi::c_int,
) {
    me_refine_bidir(
        h,
        m0,
        m1,
        i_weight,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "1185:1"]
pub unsafe extern "C" fn x264_10_me_refine_bidir_rd(
    mut h: *mut x264_t,
    mut m0: *mut x264_me_t,
    mut m1: *mut x264_me_t,
    mut i_weight: ::core::ffi::c_int,
    mut i8: ::core::ffi::c_int,
    mut i_lambda2: ::core::ffi::c_int,
) {
    (*h).mb.b_skip_mc = 1 as ::core::ffi::c_int;
    me_refine_bidir(h, m0, m1, i_weight, i8, i_lambda2, 1 as ::core::ffi::c_int);
    (*h).mb.b_skip_mc = 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1233:1"]
pub unsafe extern "C" fn x264_10_me_refine_qpel_rd(
    mut h: *mut x264_t,
    mut m: *mut x264_me_t,
    mut i_lambda2: ::core::ffi::c_int,
    mut i4: ::core::ffi::c_int,
    mut i_list: ::core::ffi::c_int,
) {
    let mut cache_mv: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(i4 as isize) as isize))
    .as_mut_ptr();
    let mut p_cost_mvx: *const uint16_t = 0 as *const uint16_t;
    let mut p_cost_mvy: *const uint16_t = 0 as *const uint16_t;
    let bw: ::core::ffi::c_int = x264_pixel_size[(*m).i_pixel as usize].w as ::core::ffi::c_int;
    let bh: ::core::ffi::c_int = x264_pixel_size[(*m).i_pixel as usize].h as ::core::ffi::c_int;
    let i_pixel: ::core::ffi::c_int = (*m).i_pixel;
    let mut chroma_v_shift: ::core::ffi::c_int = (*h).mb.chroma_v_shift;
    let mut mvy_offset: ::core::ffi::c_int =
        if chroma_v_shift & (*h).mb.b_interlaced & (*m).i_ref != 0 {
            ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    let mut bcost: uint64_t = COST_MAX64 as uint64_t;
    let mut bmx: ::core::ffi::c_int =
        (*m).mv[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    let mut bmy: ::core::ffi::c_int =
        (*m).mv[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    let mut omx: ::core::ffi::c_int = 0;
    let mut omy: ::core::ffi::c_int = 0;
    let mut pmx: ::core::ffi::c_int = 0;
    let mut pmy: ::core::ffi::c_int = 0;
    let mut satd: ::core::ffi::c_int = 0;
    let mut bsatd: ::core::ffi::c_int = 0;
    let mut dir: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
    let mut i8: ::core::ffi::c_int = i4 >> 2 as ::core::ffi::c_int;
    let mut amvd: uint16_t = 0;
    let mut pix: *mut pixel = &mut *(*(*h)
        .mb
        .pic
        .p_fdec
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
    .offset(*block_idx_xy_fdec.as_ptr().offset(i4 as isize) as isize)
        as *mut pixel;
    let mut pixu: *mut pixel = 0 as *mut pixel;
    let mut pixv: *mut pixel = 0 as *mut pixel;
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
        pixu = &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
        .offset(*block_idx_xy_fdec.as_ptr().offset(i4 as isize) as isize)
            as *mut pixel;
        pixv = &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(2 as ::core::ffi::c_int as isize))
        .offset(*block_idx_xy_fdec.as_ptr().offset(i4 as isize) as isize)
            as *mut pixel;
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        pixu = &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
        .offset(
            ((i8 >> 1 as ::core::ffi::c_int)
                * (8 as ::core::ffi::c_int * FDEC_STRIDE >> chroma_v_shift)
                + (i8 & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
        ) as *mut pixel;
        pixv = &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(2 as ::core::ffi::c_int as isize))
        .offset(
            ((i8 >> 1 as ::core::ffi::c_int)
                * (8 as ::core::ffi::c_int * FDEC_STRIDE >> chroma_v_shift)
                + (i8 & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int) as isize,
        ) as *mut pixel;
    } else {
        pixu = 0 as *mut pixel;
        pixv = 0 as *mut pixel;
    }
    (*h).mb.b_skip_mc = 1 as ::core::ffi::c_int;
    if (*m).i_pixel != PIXEL_16x16 as ::core::ffi::c_int && i4 != 0 as ::core::ffi::c_int {
        x264_10_mb_predict_mv(
            h,
            i_list,
            i4,
            bw >> 2 as ::core::ffi::c_int,
            (*m).mvp.as_mut_ptr(),
        );
    }
    pmx = (*m).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    pmy = (*m).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
    p_cost_mvx = (*m).p_cost_mv.offset(-(pmx as isize));
    p_cost_mvy = (*m).p_cost_mv.offset(-(pmy as isize));
    if 0 as ::core::ffi::c_int == 0 || !(bmx == pmx && bmy == pmy) {
        (*h).mc.mc_luma.expect("non-null function pointer")(
            pix,
            FDEC_STRIDE as intptr_t,
            (*m).p_fref.as_mut_ptr(),
            (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
            bmx,
            bmy,
            bw,
            bh,
            &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
        );
        bsatd = (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
            (*m).p_fenc[0 as ::core::ffi::c_int as usize],
            FENC_STRIDE as intptr_t,
            pix,
            FDEC_STRIDE as intptr_t,
        ) + *p_cost_mvx.offset(bmx as isize) as ::core::ffi::c_int
            + *p_cost_mvy.offset(bmy as isize) as ::core::ffi::c_int;
        if bsatd < bsatd {
            bsatd = bsatd;
        }
    } else {
        bsatd = COST_MAX;
    }
    if (*m).i_pixel != PIXEL_16x16 as ::core::ffi::c_int {
        if 0 as ::core::ffi::c_int <= bsatd + (bsatd >> 4 as ::core::ffi::c_int) {
            let mut cost: uint64_t = 0;
            (*(cache_mv as *mut x264_union32_t)).i = pack16to32_mask(bmx, bmy);
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    pixu,
                    FDEC_STRIDE as intptr_t,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(4 as ::core::ffi::c_int as isize),
                    (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    bmx,
                    bmy,
                    bw,
                    bh,
                    &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                );
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    pixv,
                    FDEC_STRIDE as intptr_t,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(8 as ::core::ffi::c_int as isize),
                    (*m).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                    bmx,
                    bmy,
                    bw,
                    bh,
                    &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                );
            } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0
                && (*m).i_pixel <= PIXEL_8x8 as ::core::ffi::c_int
            {
                (*h).mc.mc_chroma.expect("non-null function pointer")(
                    pixu,
                    pixv,
                    FDEC_STRIDE as intptr_t,
                    (*m).p_fref[4 as ::core::ffi::c_int as usize],
                    (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    bmx,
                    2 as ::core::ffi::c_int * (bmy + mvy_offset) >> chroma_v_shift,
                    bw >> 1 as ::core::ffi::c_int,
                    bh >> chroma_v_shift,
                );
                if !(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                    .weightfn
                    .is_null()
                {
                    (*(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                        .weightfn
                        .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixu,
                        FDEC_STRIDE as intptr_t,
                        pixu,
                        FDEC_STRIDE as intptr_t,
                        &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
                if !(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                    .weightfn
                    .is_null()
                {
                    (*(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                        .weightfn
                        .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixv,
                        FDEC_STRIDE as intptr_t,
                        pixv,
                        FDEC_STRIDE as intptr_t,
                        &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
            }
            cost = x264_10_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
            if cost < bcost {
                bcost = cost;
                bmx = bmx;
                bmy = bmy;
                dir = if 0 as ::core::ffi::c_int != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    dir
                };
            }
        }
    } else {
        bcost = (*m).cost as uint64_t;
    }
    if (bmx != pmx || bmy != pmy)
        && pmx >= (*h).mb.mv_min_spel[0 as ::core::ffi::c_int as usize]
        && pmx <= (*h).mb.mv_max_spel[0 as ::core::ffi::c_int as usize]
        && pmy >= (*h).mb.mv_min_spel[1 as ::core::ffi::c_int as usize]
        && pmy <= (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize]
    {
        if 0 as ::core::ffi::c_int == 0 || !(pmx == pmx && pmy == pmy) {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                pix,
                FDEC_STRIDE as intptr_t,
                (*m).p_fref.as_mut_ptr(),
                (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                pmx,
                pmy,
                bw,
                bh,
                &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
            );
            satd = (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
                (*m).p_fenc[0 as ::core::ffi::c_int as usize],
                FENC_STRIDE as intptr_t,
                pix,
                FDEC_STRIDE as intptr_t,
            ) + *p_cost_mvx.offset(pmx as isize) as ::core::ffi::c_int
                + *p_cost_mvy.offset(pmy as isize) as ::core::ffi::c_int;
            if satd < bsatd {
                bsatd = satd;
            }
        } else {
            satd = COST_MAX;
        }
        if satd <= bsatd + (bsatd >> 4 as ::core::ffi::c_int) {
            let mut cost_0: uint64_t = 0;
            (*(cache_mv as *mut x264_union32_t)).i = pack16to32_mask(pmx, pmy);
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    pixu,
                    FDEC_STRIDE as intptr_t,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(4 as ::core::ffi::c_int as isize),
                    (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    pmx,
                    pmy,
                    bw,
                    bh,
                    &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                );
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    pixv,
                    FDEC_STRIDE as intptr_t,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(8 as ::core::ffi::c_int as isize),
                    (*m).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                    pmx,
                    pmy,
                    bw,
                    bh,
                    &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                );
            } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0
                && (*m).i_pixel <= PIXEL_8x8 as ::core::ffi::c_int
            {
                (*h).mc.mc_chroma.expect("non-null function pointer")(
                    pixu,
                    pixv,
                    FDEC_STRIDE as intptr_t,
                    (*m).p_fref[4 as ::core::ffi::c_int as usize],
                    (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    pmx,
                    2 as ::core::ffi::c_int * (pmy + mvy_offset) >> chroma_v_shift,
                    bw >> 1 as ::core::ffi::c_int,
                    bh >> chroma_v_shift,
                );
                if !(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                    .weightfn
                    .is_null()
                {
                    (*(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                        .weightfn
                        .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixu,
                        FDEC_STRIDE as intptr_t,
                        pixu,
                        FDEC_STRIDE as intptr_t,
                        &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
                if !(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                    .weightfn
                    .is_null()
                {
                    (*(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                        .weightfn
                        .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixv,
                        FDEC_STRIDE as intptr_t,
                        pixv,
                        FDEC_STRIDE as intptr_t,
                        &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
            }
            cost_0 = x264_10_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
            if cost_0 < bcost {
                bcost = cost_0;
                bmx = pmx;
                bmy = pmy;
                dir = if 0 as ::core::ffi::c_int != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    dir
                };
            }
        }
        if bmx == pmx && bmy == pmy {
            pmx = (*m).mv[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
            pmy = (*m).mv[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
        }
    }
    if bmy < (*h).mb.mv_min_spel[1 as ::core::ffi::c_int as usize] + 3 as ::core::ffi::c_int
        || bmy > (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize] - 3 as ::core::ffi::c_int
        || bmx < (*h).mb.mv_min_spel[0 as ::core::ffi::c_int as usize] + 3 as ::core::ffi::c_int
        || bmx > (*h).mb.mv_max_spel[0 as ::core::ffi::c_int as usize] - 3 as ::core::ffi::c_int
    {
        (*h).mb.b_skip_mc = 0 as ::core::ffi::c_int;
        return;
    }
    dir = -(2 as ::core::ffi::c_int);
    omx = bmx;
    omy = bmy;
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j < 6 as ::core::ffi::c_int {
        if 1 as ::core::ffi::c_int == 0
            || !(omx
                + hex2[(j + 1 as ::core::ffi::c_int) as usize][0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                == pmx
                && omy
                    + hex2[(j + 1 as ::core::ffi::c_int) as usize][1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                    == pmy)
        {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                pix,
                FDEC_STRIDE as intptr_t,
                (*m).p_fref.as_mut_ptr(),
                (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                omx + hex2[(j + 1 as ::core::ffi::c_int) as usize][0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int,
                omy + hex2[(j + 1 as ::core::ffi::c_int) as usize][1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int,
                bw,
                bh,
                &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
            );
            satd = (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
                (*m).p_fenc[0 as ::core::ffi::c_int as usize],
                FENC_STRIDE as intptr_t,
                pix,
                FDEC_STRIDE as intptr_t,
            ) + *p_cost_mvx.offset(
                (omx + hex2[(j + 1 as ::core::ffi::c_int) as usize]
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
                + *p_cost_mvy.offset(
                    (omy + hex2[(j + 1 as ::core::ffi::c_int) as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int;
            if satd < bsatd {
                bsatd = satd;
            }
        } else {
            satd = COST_MAX;
        }
        if satd <= bsatd + (bsatd >> 4 as ::core::ffi::c_int) {
            let mut cost_1: uint64_t = 0;
            (*(cache_mv as *mut x264_union32_t)).i = pack16to32_mask(
                omx + hex2[(j + 1 as ::core::ffi::c_int) as usize][0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int,
                omy + hex2[(j + 1 as ::core::ffi::c_int) as usize][1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int,
            );
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    pixu,
                    FDEC_STRIDE as intptr_t,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(4 as ::core::ffi::c_int as isize),
                    (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    omx + hex2[(j + 1 as ::core::ffi::c_int) as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    omy + hex2[(j + 1 as ::core::ffi::c_int) as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    bw,
                    bh,
                    &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                );
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    pixv,
                    FDEC_STRIDE as intptr_t,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(8 as ::core::ffi::c_int as isize),
                    (*m).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                    omx + hex2[(j + 1 as ::core::ffi::c_int) as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    omy + hex2[(j + 1 as ::core::ffi::c_int) as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    bw,
                    bh,
                    &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                );
            } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0
                && (*m).i_pixel <= PIXEL_8x8 as ::core::ffi::c_int
            {
                (*h).mc.mc_chroma.expect("non-null function pointer")(
                    pixu,
                    pixv,
                    FDEC_STRIDE as intptr_t,
                    (*m).p_fref[4 as ::core::ffi::c_int as usize],
                    (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    omx + hex2[(j + 1 as ::core::ffi::c_int) as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int
                        * (omy
                            + hex2[(j + 1 as ::core::ffi::c_int) as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                            + mvy_offset)
                        >> chroma_v_shift,
                    bw >> 1 as ::core::ffi::c_int,
                    bh >> chroma_v_shift,
                );
                if !(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                    .weightfn
                    .is_null()
                {
                    (*(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                        .weightfn
                        .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixu,
                        FDEC_STRIDE as intptr_t,
                        pixu,
                        FDEC_STRIDE as intptr_t,
                        &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
                if !(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                    .weightfn
                    .is_null()
                {
                    (*(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                        .weightfn
                        .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixv,
                        FDEC_STRIDE as intptr_t,
                        pixv,
                        FDEC_STRIDE as intptr_t,
                        &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
            }
            cost_1 = x264_10_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
            if cost_1 < bcost {
                bcost = cost_1;
                bmx = omx
                    + hex2[(j + 1 as ::core::ffi::c_int) as usize][0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                bmy = omy
                    + hex2[(j + 1 as ::core::ffi::c_int) as usize][1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                dir = if 1 as ::core::ffi::c_int != 0 { j } else { dir };
            }
        }
        j += 1;
    }
    if dir != -(2 as ::core::ffi::c_int) {
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i < 10 as ::core::ffi::c_int {
            let odir: ::core::ffi::c_int =
                mod6m1[(dir + 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
            if bmy < (*h).mb.mv_min_spel[1 as ::core::ffi::c_int as usize] + 3 as ::core::ffi::c_int
                || bmy
                    > (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize]
                        - 3 as ::core::ffi::c_int
            {
                break;
            }
            dir = -(2 as ::core::ffi::c_int);
            omx = bmx;
            omy = bmy;
            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_0 < 3 as ::core::ffi::c_int {
                if 1 as ::core::ffi::c_int == 0
                    || !(omx
                        + hex2[(odir + j_0) as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                        == pmx
                        && omy
                            + hex2[(odir + j_0) as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                            == pmy)
                {
                    (*h).mc.mc_luma.expect("non-null function pointer")(
                        pix,
                        FDEC_STRIDE as intptr_t,
                        (*m).p_fref.as_mut_ptr(),
                        (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                        omx + hex2[(odir + j_0) as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int,
                        omy + hex2[(odir + j_0) as usize][1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int,
                        bw,
                        bh,
                        &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
                    );
                    satd = (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
                        (*m).p_fenc[0 as ::core::ffi::c_int as usize],
                        FENC_STRIDE as intptr_t,
                        pix,
                        FDEC_STRIDE as intptr_t,
                    ) + *p_cost_mvx.offset(
                        (omx + hex2[(odir + j_0) as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                        + *p_cost_mvy.offset(
                            (omy + hex2[(odir + j_0) as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int;
                    if satd < bsatd {
                        bsatd = satd;
                    }
                } else {
                    satd = COST_MAX;
                }
                if satd <= bsatd + (bsatd >> 4 as ::core::ffi::c_int) {
                    let mut cost_2: uint64_t = 0;
                    (*(cache_mv as *mut x264_union32_t)).i = pack16to32_mask(
                        omx + hex2[(odir + j_0) as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int,
                        omy + hex2[(odir + j_0) as usize][1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int,
                    );
                    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                        == CHROMA_444 as ::core::ffi::c_int
                    {
                        (*h).mc.mc_luma.expect("non-null function pointer")(
                            pixu,
                            FDEC_STRIDE as intptr_t,
                            &mut *(*m)
                                .p_fref
                                .as_mut_ptr()
                                .offset(4 as ::core::ffi::c_int as isize),
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            omx + hex2[(odir + j_0) as usize][0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            omy + hex2[(odir + j_0) as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            bw,
                            bh,
                            &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                        );
                        (*h).mc.mc_luma.expect("non-null function pointer")(
                            pixv,
                            FDEC_STRIDE as intptr_t,
                            &mut *(*m)
                                .p_fref
                                .as_mut_ptr()
                                .offset(8 as ::core::ffi::c_int as isize),
                            (*m).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                            omx + hex2[(odir + j_0) as usize][0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            omy + hex2[(odir + j_0) as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            bw,
                            bh,
                            &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                        );
                    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0
                        && (*m).i_pixel <= PIXEL_8x8 as ::core::ffi::c_int
                    {
                        (*h).mc.mc_chroma.expect("non-null function pointer")(
                            pixu,
                            pixv,
                            FDEC_STRIDE as intptr_t,
                            (*m).p_fref[4 as ::core::ffi::c_int as usize],
                            (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                            omx + hex2[(odir + j_0) as usize][0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int
                                * (omy
                                    + hex2[(odir + j_0) as usize][1 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int
                                    + mvy_offset)
                                >> chroma_v_shift,
                            bw >> 1 as ::core::ffi::c_int,
                            bh >> chroma_v_shift,
                        );
                        if !(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                            .weightfn
                            .is_null()
                        {
                            (*(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                                .weightfn
                                .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                            .expect("non-null function pointer")(
                                pixu,
                                FDEC_STRIDE as intptr_t,
                                pixu,
                                FDEC_STRIDE as intptr_t,
                                &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                        if !(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                            .weightfn
                            .is_null()
                        {
                            (*(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                                .weightfn
                                .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                            .expect("non-null function pointer")(
                                pixv,
                                FDEC_STRIDE as intptr_t,
                                pixv,
                                FDEC_STRIDE as intptr_t,
                                &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                                bh >> chroma_v_shift,
                            );
                        }
                    }
                    cost_2 = x264_10_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
                    if cost_2 < bcost {
                        bcost = cost_2;
                        bmx = omx
                            + hex2[(odir + j_0) as usize][0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int;
                        bmy = omy
                            + hex2[(odir + j_0) as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int;
                        dir = if 1 as ::core::ffi::c_int != 0 {
                            odir - 1 as ::core::ffi::c_int + j_0
                        } else {
                            dir
                        };
                    }
                }
                j_0 += 1;
            }
            if dir == -(2 as ::core::ffi::c_int) {
                break;
            }
            i += 1;
        }
    }
    omx = bmx;
    omy = bmy;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 8 as ::core::ffi::c_int {
        if 1 as ::core::ffi::c_int == 0
            || !(omx
                + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                == pmx
                && omy
                    + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                    == pmy)
        {
            (*h).mc.mc_luma.expect("non-null function pointer")(
                pix,
                FDEC_STRIDE as intptr_t,
                (*m).p_fref.as_mut_ptr(),
                (*m).i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
                omx + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                omy + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                    [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                bw,
                bh,
                &*(*m).weight.offset(0 as ::core::ffi::c_int as isize),
            );
            satd = (*h).pixf.mbcmp[i_pixel as usize].expect("non-null function pointer")(
                (*m).p_fenc[0 as ::core::ffi::c_int as usize],
                FENC_STRIDE as intptr_t,
                pix,
                FDEC_STRIDE as intptr_t,
            ) + *p_cost_mvx.offset(
                (omx + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
                    as isize,
            ) as ::core::ffi::c_int
                + *p_cost_mvy.offset(
                    (omy + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int) as isize,
                ) as ::core::ffi::c_int;
            if satd < bsatd {
                bsatd = satd;
            }
        } else {
            satd = COST_MAX;
        }
        if satd <= bsatd + (bsatd >> 4 as ::core::ffi::c_int) {
            let mut cost_3: uint64_t = 0;
            (*(cache_mv as *mut x264_union32_t)).i = pack16to32_mask(
                omx + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                    [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                omy + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                    [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            );
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    pixu,
                    FDEC_STRIDE as intptr_t,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(4 as ::core::ffi::c_int as isize),
                    (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    omx + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    omy + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    bw,
                    bh,
                    &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                );
                (*h).mc.mc_luma.expect("non-null function pointer")(
                    pixv,
                    FDEC_STRIDE as intptr_t,
                    &mut *(*m)
                        .p_fref
                        .as_mut_ptr()
                        .offset(8 as ::core::ffi::c_int as isize),
                    (*m).i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
                    omx + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    omy + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    bw,
                    bh,
                    &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                );
            } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0
                && (*m).i_pixel <= PIXEL_8x8 as ::core::ffi::c_int
            {
                (*h).mc.mc_chroma.expect("non-null function pointer")(
                    pixu,
                    pixv,
                    FDEC_STRIDE as intptr_t,
                    (*m).p_fref[4 as ::core::ffi::c_int as usize],
                    (*m).i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    omx + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int
                        * (omy
                            + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                            + mvy_offset)
                        >> chroma_v_shift,
                    bw >> 1 as ::core::ffi::c_int,
                    bh >> chroma_v_shift,
                );
                if !(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                    .weightfn
                    .is_null()
                {
                    (*(*(*m).weight.offset(1 as ::core::ffi::c_int as isize))
                        .weightfn
                        .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixu,
                        FDEC_STRIDE as intptr_t,
                        pixu,
                        FDEC_STRIDE as intptr_t,
                        &*(*m).weight.offset(1 as ::core::ffi::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
                if !(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                    .weightfn
                    .is_null()
                {
                    (*(*(*m).weight.offset(2 as ::core::ffi::c_int as isize))
                        .weightfn
                        .offset((bw >> 3 as ::core::ffi::c_int) as isize))
                    .expect("non-null function pointer")(
                        pixv,
                        FDEC_STRIDE as intptr_t,
                        pixv,
                        FDEC_STRIDE as intptr_t,
                        &*(*m).weight.offset(2 as ::core::ffi::c_int as isize),
                        bh >> chroma_v_shift,
                    );
                }
            }
            cost_3 = x264_10_rd_cost_part(h, i_lambda2, i4, (*m).i_pixel);
            if cost_3 < bcost {
                bcost = cost_3;
                bmx = omx
                    + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                bmy = omy
                    + square1[(i_0 + 1 as ::core::ffi::c_int) as usize]
                        [1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                dir = if 0 as ::core::ffi::c_int != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    dir
                };
            }
        }
        i_0 += 1;
    }
    (*m).cost = bcost as ::core::ffi::c_int;
    (*m).mv[0 as ::core::ffi::c_int as usize] = bmx as int16_t;
    (*m).mv[1 as ::core::ffi::c_int as usize] = bmy as int16_t;
    x264_macroblock_cache_mv(
        h,
        block_idx_x[i4 as usize] as ::core::ffi::c_int,
        block_idx_y[i4 as usize] as ::core::ffi::c_int,
        bw >> 2 as ::core::ffi::c_int,
        bh >> 2 as ::core::ffi::c_int,
        i_list,
        pack16to32_mask(bmx, bmy),
    );
    amvd = pack8to16(
        (if abs(bmx - (*m).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
            < 66 as ::core::ffi::c_int
        {
            abs(bmx - (*m).mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        } else {
            66 as ::core::ffi::c_int
        }) as uint32_t,
        (if abs(bmy - (*m).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
            < 66 as ::core::ffi::c_int
        {
            abs(bmy - (*m).mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int)
        } else {
            66 as ::core::ffi::c_int
        }) as uint32_t,
    ) as uint16_t;
    x264_macroblock_cache_mvd(
        h,
        block_idx_x[i4 as usize] as ::core::ffi::c_int,
        block_idx_y[i4 as usize] as ::core::ffi::c_int,
        bw >> 2 as ::core::ffi::c_int,
        bh >> 2 as ::core::ffi::c_int,
        i_list,
        amvd,
    );
    (*h).mb.b_skip_mc = 0 as ::core::ffi::c_int;
}
