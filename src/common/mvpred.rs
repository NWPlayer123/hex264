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
    #[c2rust::src_loc = "106:10"]
    pub const INT16_MIN: ::core::ffi::c_int =
        -(32767 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "111:10"]
    pub const INT16_MAX: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
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
    use super::bitstream_h::{bs_t, x264_bitstream_function_t};
    use super::cabac_h::x264_cabac_t;
    use super::dct_h::{x264_dct_function_t, x264_zigzag_function_t};
    use super::frame_h::{x264_deblock_function_t, x264_frame_t, x264_sync_frame_list_t};
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
    #[c2rust::src_loc = "202:9"]
    pub const X264_DIRECT_PRED_NONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
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
    #[c2rust::src_loc = "232:1"]
    pub unsafe extern "C" fn x264_median(
        mut a: ::core::ffi::c_int,
        mut b: ::core::ffi::c_int,
        mut c: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut t: ::core::ffi::c_int = a - b & a - b >> 31 as ::core::ffi::c_int;
        a -= t;
        b += t;
        b -= b - c & b - c >> 31 as ::core::ffi::c_int;
        b += a - b & a - b >> 31 as ::core::ffi::c_int;
        return b;
    }
    #[inline(always)]
    #[c2rust::src_loc = "242:1"]
    pub unsafe extern "C" fn x264_median_mv(
        mut dst: *mut int16_t,
        mut a: *mut int16_t,
        mut b: *mut int16_t,
        mut c: *mut int16_t,
    ) {
        *dst.offset(0 as ::core::ffi::c_int as isize) = x264_median(
            *a.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *b.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *c.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) as int16_t;
        *dst.offset(1 as ::core::ffi::c_int as isize) = x264_median(
            *a.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *b.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *c.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        ) as int16_t;
    }
    use super::stdint_intn_h::int16_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/macroblock.h:28"]
pub mod macroblock_h {
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
    #[inline(always)]
    #[c2rust::src_loc = "395:1"]
    pub unsafe extern "C" fn pack16to32_mask(
        mut a: ::core::ffi::c_int,
        mut b: ::core::ffi::c_int,
    ) -> uint32_t {
        return ((a & 0xffff as ::core::ffi::c_int) as uint32_t)
            .wrapping_add((b as uint32_t) << 16 as ::core::ffi::c_int);
    }
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:28"]
pub mod osdep_h {
    #[c2rust::src_loc = "452:9"]
    pub const WORD_SIZE: uint64_t = ::core::mem::size_of::<*mut ::core::ffi::c_void>() as uint64_t;
    use super::stdint_uintn_h::uint64_t;
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
                    b"common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                    82 as ::core::ffi::c_uint,
                    __ASSERT_FUNCTION.as_ptr(),
                );
            }
            'c_20640: {
                if h != 1 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"h != 1\0" as *const u8 as *const ::core::ffi::c_char,
                        b"common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                108 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
            'c_20406: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
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
    #[c2rust::src_loc = "135:1"]
    pub unsafe extern "C" fn x264_macroblock_cache_ref(
        mut h: *mut x264_t,
        mut x: ::core::ffi::c_int,
        mut y: ::core::ffi::c_int,
        mut width: ::core::ffi::c_int,
        mut height: ::core::ffi::c_int,
        mut i_list: ::core::ffi::c_int,
        mut ref_0: int8_t,
    ) {
        let mut ref_cache: *mut ::core::ffi::c_void =
            &mut *(*(*h).mb.cache.ref_0.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((X264_SCAN8_0 + x + 8 as ::core::ffi::c_int * y) as isize)
                as *mut int8_t as *mut ::core::ffi::c_void;
        if 0 == 0 || 0 == 0 {
            x264_10_cache_ref_func_table
                [(width + (height << 1 as ::core::ffi::c_int) - 3 as ::core::ffi::c_int) as usize]
                .expect("non-null function pointer")(
                ref_cache, ref_0 as uint8_t as uint32_t
            );
        } else {
            x264_macroblock_cache_rect(
                ref_cache,
                width,
                height,
                1 as ::core::ffi::c_int,
                ref_0 as uint8_t as uint32_t,
            );
        };
    }
    use super::assert_h::{__assert_fail, __ASSERT_FUNCTION};
    use super::base_h::{x264_union16_t, x264_union32_t, x264_union64_t, X264_SCAN8_0};
    use super::common_h::x264_t;
    use super::osdep_h::WORD_SIZE;
    use super::stdint_intn_h::{int16_t, int8_t};
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "112:15"]
        pub static mut x264_10_cache_mv_func_table:
            [Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, uint32_t) -> ()>; 10];
        #[c2rust::src_loc = "116:15"]
        pub static mut x264_10_cache_ref_func_table:
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
    slice_type_e, x264_clip3, x264_median, x264_median_mv, x264_scan8, x264_union16_t,
    x264_union32_t, x264_union64_t, SLICE_TYPE_B, SLICE_TYPE_I, SLICE_TYPE_P, X264_SCAN8_0,
};
pub use self::bitstream_h::{bs_s, bs_t, x264_bitstream_function_t, x264_run_level_t};
pub use self::cabac_h::x264_cabac_t;
pub use self::common_h::{
    dctcoef, pixel, udctcoef, x264_frame_stat_t, x264_left_table_t, x264_lookahead_t,
    x264_ratecontrol_t, x264_slice_header_t, x264_t, C2RustUnnamed_10, C2RustUnnamed_11,
    C2RustUnnamed_12, C2RustUnnamed_13, C2RustUnnamed_17, C2RustUnnamed_18, C2RustUnnamed_6,
    C2RustUnnamed_7, C2RustUnnamed_8, C2RustUnnamed_9,
};
pub use self::dct_h::{x264_dct_function_t, x264_zigzag_function_t};
pub use self::frame_h::{
    x264_deblock_function_t, x264_deblock_inter_t, x264_deblock_intra_t, x264_frame, x264_frame_t,
    x264_sync_frame_list_t,
};
pub use self::internal::__va_list_tag;
pub use self::macroblock_h::{
    mb_class_e, mb_partition_e, pack16to32_mask, B_8x8, D_16x16, D_16x8, D_8x16, D_8x8, D_BI_4x4,
    D_BI_4x8, D_BI_8x4, D_BI_8x8, D_DIRECT_8x8, D_L0_4x4, D_L0_4x8, D_L0_8x4, D_L0_8x8, D_L1_4x4,
    D_L1_4x8, D_L1_8x4, D_L1_8x8, I_16x16, I_4x4, I_8x8, P_8x8, B_BI_BI, B_BI_L0, B_BI_L1,
    B_DIRECT, B_L0_BI, B_L0_L0, B_L0_L1, B_L1_BI, B_L1_L0, B_L1_L1, B_SKIP, I_PCM, P_L0, P_SKIP,
    X264_MBTYPE_MAX, X264_PARTTYPE_MAX,
};
pub use self::mc_h::{weight_fn_t, x264_mc_functions_t, x264_weight_t};
pub use self::osdep_h::WORD_SIZE;
pub use self::pixel_h::{
    x264_pixel_cmp_t, x264_pixel_cmp_x3_t, x264_pixel_cmp_x4_t, x264_pixel_function_t,
};
pub use self::predict_h::{x264_predict8x8_t, x264_predict_8x8_filter_t, x264_predict_t};
pub use self::pthreadtypes_h::{pthread_cond_t, pthread_mutex_t, pthread_t};
pub use self::quant_h::x264_quant_function_t;
pub use self::rectangle_h::{
    x264_10_cache_mv_func_table, x264_10_cache_ref_func_table, x264_macroblock_cache_mv,
    x264_macroblock_cache_rect, x264_macroblock_cache_ref,
};
pub use self::set_h::{
    x264_pps_t, x264_sps_t, C2RustUnnamed_14, C2RustUnnamed_15, C2RustUnnamed_16,
};
pub use self::stdint_h::{intptr_t, uintptr_t, INT16_MAX, INT16_MIN};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use self::stdlib_h::abs;
pub use self::struct_mutex_h::__pthread_mutex_s;
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
    C2RustUnnamed_5, X264_DIRECT_PRED_NONE,
};
#[no_mangle]
#[c2rust::src_loc = "30:1"]
pub unsafe extern "C" fn x264_10_mb_predict_mv(
    mut h: *mut x264_t,
    mut i_list: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut mvp: *mut int16_t,
) {
    let i8: ::core::ffi::c_int = x264_scan8[idx as usize] as ::core::ffi::c_int;
    let i_ref: ::core::ffi::c_int =
        (*h).mb.cache.ref_0[i_list as usize][i8 as usize] as ::core::ffi::c_int;
    let mut i_refa: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(i8 - 1 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let mut mv_a: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((i8 - 1 as ::core::ffi::c_int) as isize))
    .as_mut_ptr();
    let mut i_refb: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(i8 - 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let mut mv_b: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((i8 - 8 as ::core::ffi::c_int) as isize))
    .as_mut_ptr();
    let mut i_refc: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(i8 - 8 as ::core::ffi::c_int + i_width) as usize]
        as ::core::ffi::c_int;
    let mut mv_c: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((i8 - 8 as ::core::ffi::c_int + i_width) as isize))
    .as_mut_ptr();
    if idx & 3 as ::core::ffi::c_int
        >= 2 as ::core::ffi::c_int + (i_width & 1 as ::core::ffi::c_int)
        || i_refc == -(2 as ::core::ffi::c_int)
    {
        i_refc = (*h).mb.cache.ref_0[i_list as usize]
            [(i8 - 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        mv_c = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((i8 - 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
        .as_mut_ptr();
        if (*h).sh.b_mbaff != 0
            && (*h).mb.cache.ref_0[i_list as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int
                != -(2 as ::core::ffi::c_int)
            && (*h).mb.b_interlaced
                != *(*h)
                    .mb
                    .field
                    .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                    as ::core::ffi::c_int
        {
            if idx == 2 as ::core::ffi::c_int {
                mv_c = (*(*(*h)
                    .mb
                    .cache
                    .topright_mv
                    .as_mut_ptr()
                    .offset(i_list as isize))
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr();
                i_refc = (*h).mb.cache.topright_ref[i_list as usize]
                    [0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
            } else if idx == 8 as ::core::ffi::c_int {
                mv_c = (*(*(*h)
                    .mb
                    .cache
                    .topright_mv
                    .as_mut_ptr()
                    .offset(i_list as isize))
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr();
                i_refc = (*h).mb.cache.topright_ref[i_list as usize]
                    [1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
            } else if idx == 10 as ::core::ffi::c_int {
                mv_c = (*(*(*h)
                    .mb
                    .cache
                    .topright_mv
                    .as_mut_ptr()
                    .offset(i_list as isize))
                .as_mut_ptr()
                .offset(2 as ::core::ffi::c_int as isize))
                .as_mut_ptr();
                i_refc = (*h).mb.cache.topright_ref[i_list as usize]
                    [2 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
            }
        }
    }
    if (*h).mb.i_partition == D_16x8 as ::core::ffi::c_int {
        if idx == 0 as ::core::ffi::c_int {
            if i_refb == i_ref {
                (*(mvp as *mut x264_union32_t)).i = (*(mv_b as *mut x264_union32_t)).i;
                return;
            }
        } else if i_refa == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
            return;
        }
    } else if (*h).mb.i_partition == D_8x16 as ::core::ffi::c_int {
        if idx == 0 as ::core::ffi::c_int {
            if i_refa == i_ref {
                (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
                return;
            }
        } else if i_refc == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_c as *mut x264_union32_t)).i;
            return;
        }
    }
    let mut i_count: ::core::ffi::c_int = (i_refa == i_ref) as ::core::ffi::c_int
        + (i_refb == i_ref) as ::core::ffi::c_int
        + (i_refc == i_ref) as ::core::ffi::c_int;
    let mut current_block_51: u64;
    if i_count > 1 as ::core::ffi::c_int {
        current_block_51 = 12053149342108387725;
    } else if i_count == 1 as ::core::ffi::c_int {
        if i_refa == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
        } else if i_refb == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_b as *mut x264_union32_t)).i;
        } else {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_c as *mut x264_union32_t)).i;
        }
        current_block_51 = 10150597327160359210;
    } else if i_refb == -(2 as ::core::ffi::c_int)
        && i_refc == -(2 as ::core::ffi::c_int)
        && i_refa != -(2 as ::core::ffi::c_int)
    {
        (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
        current_block_51 = 10150597327160359210;
    } else {
        current_block_51 = 12053149342108387725;
    }
    match current_block_51 {
        12053149342108387725 => {
            x264_median_mv(mvp as *mut int16_t, mv_a, mv_b, mv_c);
        }
        _ => {}
    };
}
#[no_mangle]
#[c2rust::src_loc = "129:1"]
pub unsafe extern "C" fn x264_10_mb_predict_mv_16x16(
    mut h: *mut x264_t,
    mut i_list: ::core::ffi::c_int,
    mut i_ref: ::core::ffi::c_int,
    mut mvp: *mut int16_t,
) {
    let mut i_refa: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(X264_SCAN8_0 - 1 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let mut mv_a: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((X264_SCAN8_0 - 1 as ::core::ffi::c_int) as isize))
    .as_mut_ptr();
    let mut i_refb: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(X264_SCAN8_0 - 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let mut mv_b: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((X264_SCAN8_0 - 8 as ::core::ffi::c_int) as isize))
    .as_mut_ptr();
    let mut i_refc: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
        [(X264_SCAN8_0 - 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let mut mv_c: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
        .as_mut_ptr()
        .offset((X264_SCAN8_0 - 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize))
    .as_mut_ptr();
    if i_refc == -(2 as ::core::ffi::c_int) {
        i_refc = (*h).mb.cache.ref_0[i_list as usize]
            [(X264_SCAN8_0 - 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        mv_c = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((X264_SCAN8_0 - 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize))
        .as_mut_ptr();
    }
    let mut i_count: ::core::ffi::c_int = (i_refa == i_ref) as ::core::ffi::c_int
        + (i_refb == i_ref) as ::core::ffi::c_int
        + (i_refc == i_ref) as ::core::ffi::c_int;
    let mut current_block_11: u64;
    if i_count > 1 as ::core::ffi::c_int {
        current_block_11 = 2747653332422150726;
    } else if i_count == 1 as ::core::ffi::c_int {
        if i_refa == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
        } else if i_refb == i_ref {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_b as *mut x264_union32_t)).i;
        } else {
            (*(mvp as *mut x264_union32_t)).i = (*(mv_c as *mut x264_union32_t)).i;
        }
        current_block_11 = 13056961889198038528;
    } else if i_refb == -(2 as ::core::ffi::c_int)
        && i_refc == -(2 as ::core::ffi::c_int)
        && i_refa != -(2 as ::core::ffi::c_int)
    {
        (*(mvp as *mut x264_union32_t)).i = (*(mv_a as *mut x264_union32_t)).i;
        current_block_11 = 13056961889198038528;
    } else {
        current_block_11 = 2747653332422150726;
    }
    match current_block_11 {
        2747653332422150726 => {
            x264_median_mv(mvp as *mut int16_t, mv_a, mv_b, mv_c);
        }
        _ => {}
    };
}
#[no_mangle]
#[c2rust::src_loc = "166:1"]
pub unsafe extern "C" fn x264_10_mb_predict_mv_pskip(mut h: *mut x264_t, mut mv: *mut int16_t) {
    let mut i_refa: ::core::ffi::c_int = (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
        [(X264_SCAN8_0 - 1 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let mut i_refb: ::core::ffi::c_int = (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
        [(X264_SCAN8_0 - 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let mut mv_a: *mut int16_t = (*(*(*h)
        .mb
        .cache
        .mv
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
    .as_mut_ptr()
    .offset((X264_SCAN8_0 - 1 as ::core::ffi::c_int) as isize))
    .as_mut_ptr();
    let mut mv_b: *mut int16_t = (*(*(*h)
        .mb
        .cache
        .mv
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
    .as_mut_ptr()
    .offset((X264_SCAN8_0 - 8 as ::core::ffi::c_int) as isize))
    .as_mut_ptr();
    if i_refa == -(2 as ::core::ffi::c_int)
        || i_refb == -(2 as ::core::ffi::c_int)
        || i_refa as uint32_t | (*(mv_a as *mut x264_union32_t)).i == 0
        || i_refb as uint32_t | (*(mv_b as *mut x264_union32_t)).i == 0
    {
        (*(mv as *mut x264_union32_t)).i = 0 as uint32_t;
    } else {
        x264_10_mb_predict_mv_16x16(h, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int, mv);
    };
}
#[c2rust::src_loc = "183:1"]
unsafe extern "C" fn mb_predict_mv_direct16x16_temporal(mut h: *mut x264_t) -> ::core::ffi::c_int {
    let mut mb_x: ::core::ffi::c_int = (*h).mb.i_mb_x;
    let mut mb_y: ::core::ffi::c_int = (*h).mb.i_mb_y;
    let mut mb_xy: ::core::ffi::c_int = (*h).mb.i_mb_xy;
    let mut type_col: [::core::ffi::c_int; 2] = [
        *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .mb_type
            .offset(mb_xy as isize) as ::core::ffi::c_int,
        *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .mb_type
            .offset(mb_xy as isize) as ::core::ffi::c_int,
    ];
    let mut partition_col: [::core::ffi::c_int; 2] = [
        *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .mb_partition
            .offset(mb_xy as isize) as ::core::ffi::c_int,
        *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .mb_partition
            .offset(mb_xy as isize) as ::core::ffi::c_int,
    ];
    let mut preshift: ::core::ffi::c_int = (*h).mb.b_interlaced;
    let mut postshift: ::core::ffi::c_int = (*h).mb.b_interlaced;
    let mut offset: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut yshift: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    (*h).mb.i_partition = partition_col[0 as ::core::ffi::c_int as usize];
    if (*h).param.b_interlaced != 0
        && *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .field
            .offset(mb_xy as isize) as ::core::ffi::c_int
            != (*h).mb.b_interlaced
    {
        if (*h).mb.b_interlaced != 0 {
            mb_y = (*h).mb.i_mb_y & !(1 as ::core::ffi::c_int);
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[0 as ::core::ffi::c_int as usize] =
                *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .mb_type
                    .offset(mb_xy as isize) as ::core::ffi::c_int;
            type_col[1 as ::core::ffi::c_int as usize] = *(*(*h).fref
                [1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                .mb_type
                .offset((mb_xy + (*h).mb.i_mb_stride) as isize)
                as ::core::ffi::c_int;
            partition_col[0 as ::core::ffi::c_int as usize] =
                *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .mb_partition
                    .offset(mb_xy as isize) as ::core::ffi::c_int;
            partition_col[1 as ::core::ffi::c_int as usize] = *(*(*h).fref
                [1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                .mb_partition
                .offset((mb_xy + (*h).mb.i_mb_stride) as isize)
                as ::core::ffi::c_int;
            preshift = 0 as ::core::ffi::c_int;
            yshift = 0 as ::core::ffi::c_int;
            if (type_col[0 as ::core::ffi::c_int as usize] == I_4x4 as ::core::ffi::c_int
                || type_col[0 as ::core::ffi::c_int as usize] == I_8x8 as ::core::ffi::c_int
                || type_col[0 as ::core::ffi::c_int as usize] == I_16x16 as ::core::ffi::c_int
                || type_col[0 as ::core::ffi::c_int as usize] == I_PCM as ::core::ffi::c_int
                || partition_col[0 as ::core::ffi::c_int as usize] == D_16x16 as ::core::ffi::c_int)
                && (type_col[1 as ::core::ffi::c_int as usize] == I_4x4 as ::core::ffi::c_int
                    || type_col[1 as ::core::ffi::c_int as usize] == I_8x8 as ::core::ffi::c_int
                    || type_col[1 as ::core::ffi::c_int as usize] == I_16x16 as ::core::ffi::c_int
                    || type_col[1 as ::core::ffi::c_int as usize] == I_PCM as ::core::ffi::c_int
                    || partition_col[1 as ::core::ffi::c_int as usize]
                        == D_16x16 as ::core::ffi::c_int)
                && partition_col[0 as ::core::ffi::c_int as usize] != D_8x8 as ::core::ffi::c_int
            {
                (*h).mb.i_partition = D_16x8 as ::core::ffi::c_int;
            } else {
                (*h).mb.i_partition = D_8x8 as ::core::ffi::c_int;
            }
        } else {
            let mut cur_poc: ::core::ffi::c_int = (*(*h).fdec).i_poc
                + (*(*h).fdec).i_delta_poc
                    [((*h).mb.b_interlaced & (*h).mb.i_mb_y & 1 as ::core::ffi::c_int) as usize];
            let mut col_parity: ::core::ffi::c_int = (abs((*(*h).fref
                [1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                .i_poc
                + (*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .i_delta_poc[0 as ::core::ffi::c_int as usize]
                - cur_poc)
                >= abs((*(*h).fref[1 as ::core::ffi::c_int as usize]
                    [0 as ::core::ffi::c_int as usize])
                    .i_poc
                    + (*(*h).fref[1 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .i_delta_poc[1 as ::core::ffi::c_int as usize]
                    - cur_poc))
                as ::core::ffi::c_int;
            mb_y = ((*h).mb.i_mb_y & !(1 as ::core::ffi::c_int)) + col_parity;
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[1 as ::core::ffi::c_int as usize] =
                *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .mb_type
                    .offset(mb_xy as isize) as ::core::ffi::c_int;
            type_col[0 as ::core::ffi::c_int as usize] = type_col[1 as ::core::ffi::c_int as usize];
            partition_col[1 as ::core::ffi::c_int as usize] =
                *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .mb_partition
                    .offset(mb_xy as isize) as ::core::ffi::c_int;
            partition_col[0 as ::core::ffi::c_int as usize] =
                partition_col[1 as ::core::ffi::c_int as usize];
            preshift = 1 as ::core::ffi::c_int;
            yshift = 2 as ::core::ffi::c_int;
            (*h).mb.i_partition = partition_col[0 as ::core::ffi::c_int as usize];
        }
        offset = 0 as ::core::ffi::c_int;
    }
    let mut i_mb_4x4: ::core::ffi::c_int =
        16 as ::core::ffi::c_int * (*h).mb.i_mb_stride * mb_y + 4 as ::core::ffi::c_int * mb_x;
    let mut i_mb_8x8: ::core::ffi::c_int =
        4 as ::core::ffi::c_int * (*h).mb.i_mb_stride * mb_y + 2 as ::core::ffi::c_int * mb_x;
    x264_macroblock_cache_ref(
        h,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
        0 as int8_t,
    );
    let mut max_i8: ::core::ffi::c_int =
        D_16x16 as ::core::ffi::c_int - (*h).mb.i_partition + 1 as ::core::ffi::c_int;
    let mut step: ::core::ffi::c_int = ((*h).mb.i_partition == D_16x8 as ::core::ffi::c_int)
        as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int;
    let mut width: ::core::ffi::c_int = 4 as ::core::ffi::c_int
        >> (D_16x16 as ::core::ffi::c_int - (*h).mb.i_partition & 1 as ::core::ffi::c_int);
    let mut height: ::core::ffi::c_int = 4 as ::core::ffi::c_int
        >> (D_16x16 as ::core::ffi::c_int - (*h).mb.i_partition >> 1 as ::core::ffi::c_int);
    let mut i8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i8 < max_i8 {
        let mut x8: ::core::ffi::c_int = i8 & 1 as ::core::ffi::c_int;
        let mut y8: ::core::ffi::c_int = i8 >> 1 as ::core::ffi::c_int;
        let mut ypart: ::core::ffi::c_int = if (*h).sh.b_mbaff != 0
            && *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                .field
                .offset(mb_xy as isize) as ::core::ffi::c_int
                != (*h).mb.b_interlaced
        {
            if (*h).mb.b_interlaced != 0 {
                y8 * 6 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int * ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) + y8
            }
        } else {
            3 as ::core::ffi::c_int * y8
        };
        if type_col[y8 as usize] == I_4x4 as ::core::ffi::c_int
            || type_col[y8 as usize] == I_8x8 as ::core::ffi::c_int
            || type_col[y8 as usize] == I_16x16 as ::core::ffi::c_int
            || type_col[y8 as usize] == I_PCM as ::core::ffi::c_int
        {
            x264_macroblock_cache_ref(
                h,
                2 as ::core::ffi::c_int * x8,
                2 as ::core::ffi::c_int * y8,
                width,
                height,
                0 as ::core::ffi::c_int,
                0 as int8_t,
            );
            x264_macroblock_cache_mv(
                h,
                2 as ::core::ffi::c_int * x8,
                2 as ::core::ffi::c_int * y8,
                width,
                height,
                0 as ::core::ffi::c_int,
                0 as uint32_t,
            );
            x264_macroblock_cache_mv(
                h,
                2 as ::core::ffi::c_int * x8,
                2 as ::core::ffi::c_int * y8,
                width,
                height,
                1 as ::core::ffi::c_int,
                0 as uint32_t,
            );
        } else {
            let mut i_part_8x8: ::core::ffi::c_int =
                i_mb_8x8 + x8 + (ypart >> 1 as ::core::ffi::c_int) * (*h).mb.i_b8_stride;
            let mut i_ref1_ref: ::core::ffi::c_int =
                *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .ref_0[0 as ::core::ffi::c_int as usize]
                    .offset(i_part_8x8 as isize) as ::core::ffi::c_int;
            let mut i_ref: ::core::ffi::c_int = (*h).mb.map_col_to_list0
                [((i_ref1_ref >> preshift) + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                * ((1 as ::core::ffi::c_int) << postshift)
                + (offset & i_ref1_ref & (*h).mb.b_interlaced);
            if i_ref >= 0 as ::core::ffi::c_int {
                let mut dist_scale_factor: ::core::ffi::c_int =
                    (*(*h).mb.dist_scale_factor.offset(i_ref as isize))
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int;
                let mut mv_col: *mut int16_t = (*(*(**(*(*h)
                    .fref
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
                .mv
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
                .offset(
                    (i_mb_4x4 + 3 as ::core::ffi::c_int * x8 + ypart * (*h).mb.i_b4_stride)
                        as isize,
                ))
                .as_mut_ptr();
                let mut mv_y: int16_t = (*mv_col.offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    * ((1 as ::core::ffi::c_int) << yshift)
                    / 2 as ::core::ffi::c_int) as int16_t;
                let mut l0x: ::core::ffi::c_int = dist_scale_factor
                    * *mv_col.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + 128 as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int;
                let mut l0y: ::core::ffi::c_int = dist_scale_factor * mv_y as ::core::ffi::c_int
                    + 128 as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int;
                if (*h).param.i_threads > 1 as ::core::ffi::c_int
                    && (l0y > (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize]
                        || l0y - mv_y as ::core::ffi::c_int
                            > (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize])
                {
                    return 0 as ::core::ffi::c_int;
                }
                x264_macroblock_cache_ref(
                    h,
                    2 as ::core::ffi::c_int * x8,
                    2 as ::core::ffi::c_int * y8,
                    width,
                    height,
                    0 as ::core::ffi::c_int,
                    i_ref as int8_t,
                );
                x264_macroblock_cache_mv(
                    h,
                    2 as ::core::ffi::c_int * x8,
                    2 as ::core::ffi::c_int * y8,
                    width,
                    height,
                    0 as ::core::ffi::c_int,
                    pack16to32_mask(l0x, l0y),
                );
                x264_macroblock_cache_mv(
                    h,
                    2 as ::core::ffi::c_int * x8,
                    2 as ::core::ffi::c_int * y8,
                    width,
                    height,
                    1 as ::core::ffi::c_int,
                    pack16to32_mask(
                        l0x - *mv_col.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int,
                        l0y - mv_y as ::core::ffi::c_int,
                    ),
                );
            } else {
                return 0 as ::core::ffi::c_int;
            }
        }
        i8 += step;
    }
    return 1 as ::core::ffi::c_int;
}
#[inline(always)]
#[c2rust::src_loc = "289:1"]
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial(
    mut h: *mut x264_t,
    mut b_interlaced: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ref_0: [int8_t; 2] = [0; 2];
    let mut mv: [[int16_t; 2]; 2] = [[0; 2]; 2];
    let mut i_list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_list < 2 as ::core::ffi::c_int {
        let mut i_refa: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
            [(X264_SCAN8_0 - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        let mut mv_a: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((X264_SCAN8_0 - 1 as ::core::ffi::c_int) as isize))
        .as_mut_ptr();
        let mut i_refb: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
            [(X264_SCAN8_0 - 8 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        let mut mv_b: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((X264_SCAN8_0 - 8 as ::core::ffi::c_int) as isize))
        .as_mut_ptr();
        let mut i_refc: ::core::ffi::c_int = (*h).mb.cache.ref_0[i_list as usize]
            [(X264_SCAN8_0 - 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
        let mut mv_c: *mut int16_t = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
            .as_mut_ptr()
            .offset((X264_SCAN8_0 - 8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize))
        .as_mut_ptr();
        if i_refc == -(2 as ::core::ffi::c_int) {
            i_refc = (*h).mb.cache.ref_0[i_list as usize]
                [(X264_SCAN8_0 - 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int;
            mv_c = (*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset(
                    (X264_SCAN8_0 - 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize,
                ))
            .as_mut_ptr();
        }
        let mut i_ref: ::core::ffi::c_int = (if (i_refa as ::core::ffi::c_uint)
            < (if (i_refb as ::core::ffi::c_uint) < i_refc as ::core::ffi::c_uint {
                i_refb as ::core::ffi::c_uint
            } else {
                i_refc as ::core::ffi::c_uint
            }) {
            i_refa as ::core::ffi::c_uint
        } else if (i_refb as ::core::ffi::c_uint) < i_refc as ::core::ffi::c_uint {
            i_refb as ::core::ffi::c_uint
        } else {
            i_refc as ::core::ffi::c_uint
        }) as ::core::ffi::c_int;
        if i_ref < 0 as ::core::ffi::c_int {
            i_ref = -(1 as ::core::ffi::c_int);
            (*((*mv.as_mut_ptr().offset(i_list as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
                0 as uint32_t;
        } else {
            let mut i_count: ::core::ffi::c_int = (i_refa == i_ref) as ::core::ffi::c_int
                + (i_refb == i_ref) as ::core::ffi::c_int
                + (i_refc == i_ref) as ::core::ffi::c_int;
            if i_count > 1 as ::core::ffi::c_int {
                x264_median_mv(
                    (*mv.as_mut_ptr().offset(i_list as isize)).as_mut_ptr(),
                    mv_a,
                    mv_b,
                    mv_c,
                );
            } else if i_refa == i_ref {
                (*((*mv.as_mut_ptr().offset(i_list as isize)).as_mut_ptr()
                    as *mut x264_union32_t))
                    .i = (*(mv_a as *mut x264_union32_t)).i;
            } else if i_refb == i_ref {
                (*((*mv.as_mut_ptr().offset(i_list as isize)).as_mut_ptr()
                    as *mut x264_union32_t))
                    .i = (*(mv_b as *mut x264_union32_t)).i;
            } else {
                (*((*mv.as_mut_ptr().offset(i_list as isize)).as_mut_ptr()
                    as *mut x264_union32_t))
                    .i = (*(mv_c as *mut x264_union32_t)).i;
            }
        }
        x264_macroblock_cache_ref(
            h,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            i_list,
            i_ref as int8_t,
        );
        x264_macroblock_cache_mv(
            h,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            i_list,
            (*((*mv.as_mut_ptr().offset(i_list as isize)).as_mut_ptr() as *mut x264_union32_t)).i,
        );
        ref_0[i_list as usize] = i_ref as int8_t;
        i_list += 1;
    }
    let mut mb_x: ::core::ffi::c_int = (*h).mb.i_mb_x;
    let mut mb_y: ::core::ffi::c_int = (*h).mb.i_mb_y;
    let mut mb_xy: ::core::ffi::c_int = (*h).mb.i_mb_xy;
    let mut type_col: [::core::ffi::c_int; 2] = [
        *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .mb_type
            .offset(mb_xy as isize) as ::core::ffi::c_int,
        *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .mb_type
            .offset(mb_xy as isize) as ::core::ffi::c_int,
    ];
    let mut partition_col: [::core::ffi::c_int; 2] = [
        *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .mb_partition
            .offset(mb_xy as isize) as ::core::ffi::c_int,
        *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .mb_partition
            .offset(mb_xy as isize) as ::core::ffi::c_int,
    ];
    (*h).mb.i_partition = partition_col[0 as ::core::ffi::c_int as usize];
    if b_interlaced != 0
        && *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
            .field
            .offset(mb_xy as isize) as ::core::ffi::c_int
            != (*h).mb.b_interlaced
    {
        if (*h).mb.b_interlaced != 0 {
            mb_y = (*h).mb.i_mb_y & !(1 as ::core::ffi::c_int);
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[0 as ::core::ffi::c_int as usize] =
                *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .mb_type
                    .offset(mb_xy as isize) as ::core::ffi::c_int;
            type_col[1 as ::core::ffi::c_int as usize] = *(*(*h).fref
                [1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                .mb_type
                .offset((mb_xy + (*h).mb.i_mb_stride) as isize)
                as ::core::ffi::c_int;
            partition_col[0 as ::core::ffi::c_int as usize] =
                *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .mb_partition
                    .offset(mb_xy as isize) as ::core::ffi::c_int;
            partition_col[1 as ::core::ffi::c_int as usize] = *(*(*h).fref
                [1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                .mb_partition
                .offset((mb_xy + (*h).mb.i_mb_stride) as isize)
                as ::core::ffi::c_int;
            if (type_col[0 as ::core::ffi::c_int as usize] == I_4x4 as ::core::ffi::c_int
                || type_col[0 as ::core::ffi::c_int as usize] == I_8x8 as ::core::ffi::c_int
                || type_col[0 as ::core::ffi::c_int as usize] == I_16x16 as ::core::ffi::c_int
                || type_col[0 as ::core::ffi::c_int as usize] == I_PCM as ::core::ffi::c_int
                || partition_col[0 as ::core::ffi::c_int as usize] == D_16x16 as ::core::ffi::c_int)
                && (type_col[1 as ::core::ffi::c_int as usize] == I_4x4 as ::core::ffi::c_int
                    || type_col[1 as ::core::ffi::c_int as usize] == I_8x8 as ::core::ffi::c_int
                    || type_col[1 as ::core::ffi::c_int as usize] == I_16x16 as ::core::ffi::c_int
                    || type_col[1 as ::core::ffi::c_int as usize] == I_PCM as ::core::ffi::c_int
                    || partition_col[1 as ::core::ffi::c_int as usize]
                        == D_16x16 as ::core::ffi::c_int)
                && partition_col[0 as ::core::ffi::c_int as usize] != D_8x8 as ::core::ffi::c_int
            {
                (*h).mb.i_partition = D_16x8 as ::core::ffi::c_int;
            } else {
                (*h).mb.i_partition = D_8x8 as ::core::ffi::c_int;
            }
        } else {
            let mut cur_poc: ::core::ffi::c_int = (*(*h).fdec).i_poc
                + (*(*h).fdec).i_delta_poc
                    [((*h).mb.b_interlaced & (*h).mb.i_mb_y & 1 as ::core::ffi::c_int) as usize];
            let mut col_parity: ::core::ffi::c_int = (abs((*(*h).fref
                [1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                .i_poc
                + (*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .i_delta_poc[0 as ::core::ffi::c_int as usize]
                - cur_poc)
                >= abs((*(*h).fref[1 as ::core::ffi::c_int as usize]
                    [0 as ::core::ffi::c_int as usize])
                    .i_poc
                    + (*(*h).fref[1 as ::core::ffi::c_int as usize]
                        [0 as ::core::ffi::c_int as usize])
                        .i_delta_poc[1 as ::core::ffi::c_int as usize]
                    - cur_poc))
                as ::core::ffi::c_int;
            mb_y = ((*h).mb.i_mb_y & !(1 as ::core::ffi::c_int)) + col_parity;
            mb_xy = mb_x + (*h).mb.i_mb_stride * mb_y;
            type_col[1 as ::core::ffi::c_int as usize] =
                *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .mb_type
                    .offset(mb_xy as isize) as ::core::ffi::c_int;
            type_col[0 as ::core::ffi::c_int as usize] = type_col[1 as ::core::ffi::c_int as usize];
            partition_col[1 as ::core::ffi::c_int as usize] =
                *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .mb_partition
                    .offset(mb_xy as isize) as ::core::ffi::c_int;
            partition_col[0 as ::core::ffi::c_int as usize] =
                partition_col[1 as ::core::ffi::c_int as usize];
            (*h).mb.i_partition = partition_col[0 as ::core::ffi::c_int as usize];
        }
    }
    let mut i_mb_4x4: ::core::ffi::c_int = if b_interlaced != 0 {
        4 as ::core::ffi::c_int * ((*h).mb.i_b4_stride * mb_y + mb_x)
    } else {
        (*h).mb.i_b4_xy
    };
    let mut i_mb_8x8: ::core::ffi::c_int = if b_interlaced != 0 {
        2 as ::core::ffi::c_int * ((*h).mb.i_b8_stride * mb_y + mb_x)
    } else {
        (*h).mb.i_b8_xy
    };
    let mut l1ref0: *mut int8_t = &mut *(*(**(*(*h)
        .fref
        .as_mut_ptr()
        .offset(1 as ::core::ffi::c_int as isize))
    .as_mut_ptr()
    .offset(0 as ::core::ffi::c_int as isize))
    .ref_0
    .as_mut_ptr()
    .offset(0 as ::core::ffi::c_int as isize))
    .offset(i_mb_8x8 as isize) as *mut int8_t;
    let mut l1ref1: *mut int8_t = &mut *(*(**(*(*h)
        .fref
        .as_mut_ptr()
        .offset(1 as ::core::ffi::c_int as isize))
    .as_mut_ptr()
    .offset(0 as ::core::ffi::c_int as isize))
    .ref_0
    .as_mut_ptr()
    .offset(1 as ::core::ffi::c_int as isize))
    .offset(i_mb_8x8 as isize) as *mut int8_t;
    let mut l1mv: [*mut [int16_t; 2]; 2] = [
        &mut *(*(**(*(*h)
            .fref
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
        .mv
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
        .offset(i_mb_4x4 as isize) as *mut [int16_t; 2],
        &mut *(*(**(*(*h)
            .fref
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
        .mv
        .as_mut_ptr()
        .offset(1 as ::core::ffi::c_int as isize))
        .offset(i_mb_4x4 as isize) as *mut [int16_t; 2],
    ];
    if (*(ref_0.as_mut_ptr() as *mut x264_union16_t)).i as ::core::ffi::c_int
        & 0x8080 as ::core::ffi::c_int
        == 0x8080 as ::core::ffi::c_int
    {
        x264_macroblock_cache_ref(
            h,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as int8_t,
        );
        x264_macroblock_cache_ref(
            h,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            0 as int8_t,
        );
        return 1 as ::core::ffi::c_int;
    }
    if (*h).param.i_threads > 1 as ::core::ffi::c_int
        && (mv[0 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            > (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize]
            || mv[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                > (*h).mb.mv_max_spel[1 as ::core::ffi::c_int as usize])
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*(mv.as_mut_ptr() as *mut x264_union64_t)).i == 0
        || b_interlaced == 0
            && (type_col[0 as ::core::ffi::c_int as usize] == I_4x4 as ::core::ffi::c_int
                || type_col[0 as ::core::ffi::c_int as usize] == I_8x8 as ::core::ffi::c_int
                || type_col[0 as ::core::ffi::c_int as usize] == I_16x16 as ::core::ffi::c_int
                || type_col[0 as ::core::ffi::c_int as usize] == I_PCM as ::core::ffi::c_int)
        || ref_0[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
            && ref_0[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int != 0
    {
        return 1 as ::core::ffi::c_int;
    }
    let mut max_i8: ::core::ffi::c_int =
        D_16x16 as ::core::ffi::c_int - (*h).mb.i_partition + 1 as ::core::ffi::c_int;
    let mut step: ::core::ffi::c_int = ((*h).mb.i_partition == D_16x8 as ::core::ffi::c_int)
        as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int;
    let mut width: ::core::ffi::c_int = 4 as ::core::ffi::c_int
        >> (D_16x16 as ::core::ffi::c_int - (*h).mb.i_partition & 1 as ::core::ffi::c_int);
    let mut height: ::core::ffi::c_int = 4 as ::core::ffi::c_int
        >> (D_16x16 as ::core::ffi::c_int - (*h).mb.i_partition >> 1 as ::core::ffi::c_int);
    let mut current_block_59: u64;
    let mut i8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i8 < max_i8 {
        let x8: ::core::ffi::c_int = i8 & 1 as ::core::ffi::c_int;
        let y8: ::core::ffi::c_int = i8 >> 1 as ::core::ffi::c_int;
        let mut ypart: ::core::ffi::c_int = if b_interlaced != 0
            && *(*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                .field
                .offset(mb_xy as isize) as ::core::ffi::c_int
                != (*h).mb.b_interlaced
        {
            if (*h).mb.b_interlaced != 0 {
                y8 * 6 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int * ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) + y8
            }
        } else {
            3 as ::core::ffi::c_int * y8
        };
        let mut o8: ::core::ffi::c_int =
            x8 + (ypart >> 1 as ::core::ffi::c_int) * (*h).mb.i_b8_stride;
        let mut o4: ::core::ffi::c_int = 3 as ::core::ffi::c_int * x8 + ypart * (*h).mb.i_b4_stride;
        if !(b_interlaced != 0
            && (type_col[y8 as usize] == I_4x4 as ::core::ffi::c_int
                || type_col[y8 as usize] == I_8x8 as ::core::ffi::c_int
                || type_col[y8 as usize] == I_16x16 as ::core::ffi::c_int
                || type_col[y8 as usize] == I_PCM as ::core::ffi::c_int))
        {
            let mut idx: ::core::ffi::c_int = 0;
            if *l1ref0.offset(o8 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                idx = 0 as ::core::ffi::c_int;
                current_block_59 = 6528285054092551010;
            } else if (*l1ref0.offset(o8 as isize) as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                && *l1ref1.offset(o8 as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                idx = 1 as ::core::ffi::c_int;
                current_block_59 = 6528285054092551010;
            } else {
                current_block_59 = 1423531122933789233;
            }
            match current_block_59 {
                1423531122933789233 => {}
                _ => {
                    if abs((*l1mv[idx as usize].offset(o4 as isize))
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int)
                        <= 1 as ::core::ffi::c_int
                        && abs((*l1mv[idx as usize].offset(o4 as isize))
                            [1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int)
                            <= 1 as ::core::ffi::c_int
                    {
                        if ref_0[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            x264_macroblock_cache_mv(
                                h,
                                2 as ::core::ffi::c_int * x8,
                                2 as ::core::ffi::c_int * y8,
                                width,
                                height,
                                0 as ::core::ffi::c_int,
                                0 as uint32_t,
                            );
                        }
                        if ref_0[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            x264_macroblock_cache_mv(
                                h,
                                2 as ::core::ffi::c_int * x8,
                                2 as ::core::ffi::c_int * y8,
                                width,
                                height,
                                1 as ::core::ffi::c_int,
                                0 as uint32_t,
                            );
                        }
                    }
                }
            }
        }
        i8 += step;
    }
    return 1 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "444:1"]
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial_interlaced(
    mut h: *mut x264_t,
) -> ::core::ffi::c_int {
    return mb_predict_mv_direct16x16_spatial(h, 1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "449:1"]
unsafe extern "C" fn mb_predict_mv_direct16x16_spatial_progressive(
    mut h: *mut x264_t,
) -> ::core::ffi::c_int {
    return mb_predict_mv_direct16x16_spatial(h, 0 as ::core::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "454:1"]
pub unsafe extern "C" fn x264_10_mb_predict_mv_direct16x16(
    mut h: *mut x264_t,
    mut b_changed: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut b_available: ::core::ffi::c_int = 0;
    if (*h).param.analyse.i_direct_mv_pred == X264_DIRECT_PRED_NONE {
        return 0 as ::core::ffi::c_int;
    } else if (*h).sh.b_direct_spatial_mv_pred != 0 {
        if (*h).sh.b_mbaff != 0 {
            b_available = mb_predict_mv_direct16x16_spatial_interlaced(h);
        } else {
            b_available = mb_predict_mv_direct16x16_spatial_progressive(h);
        }
    } else {
        b_available = mb_predict_mv_direct16x16_temporal(h);
    }
    if !b_changed.is_null() && b_available != 0 {
        let mut changed: ::core::ffi::c_int = 0;
        changed = ((*((*(*(*h)
            .mb
            .cache
            .direct_mv
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
        .as_mut_ptr() as *mut x264_union32_t))
            .i
            ^ (*((*(*(*h)
                .mb
                .cache
                .mv
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i) as ::core::ffi::c_int;
        changed |= ((*((*(*(*h)
            .mb
            .cache
            .direct_mv
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
        .as_mut_ptr() as *mut x264_union32_t))
            .i
            ^ (*((*(*(*h)
                .mb
                .cache
                .mv
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i) as ::core::ffi::c_int;
        changed |= (*h).mb.cache.direct_ref[0 as ::core::ffi::c_int as usize]
            [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            ^ (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int;
        changed |= (*h).mb.cache.direct_ref[1 as ::core::ffi::c_int as usize]
            [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            ^ (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int;
        if changed == 0 && (*h).mb.i_partition != D_16x16 as ::core::ffi::c_int {
            changed |= ((*((*(*(*h)
                .mb
                .cache
                .direct_mv
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(3 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h)
                    .mb
                    .cache
                    .mv
                    .as_mut_ptr()
                    .offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    *x264_scan8
                        .as_ptr()
                        .offset(12 as ::core::ffi::c_int as isize) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as ::core::ffi::c_int;
            changed |= ((*((*(*(*h)
                .mb
                .cache
                .direct_mv
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(3 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h)
                    .mb
                    .cache
                    .mv
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    *x264_scan8
                        .as_ptr()
                        .offset(12 as ::core::ffi::c_int as isize) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as ::core::ffi::c_int;
            changed |= (*h).mb.cache.direct_ref[0 as ::core::ffi::c_int as usize]
                [3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                ^ (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int;
            changed |= (*h).mb.cache.direct_ref[1 as ::core::ffi::c_int as usize]
                [3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                ^ (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                    [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int;
        }
        if changed == 0 && (*h).mb.i_partition == D_8x8 as ::core::ffi::c_int {
            changed |= ((*((*(*(*h)
                .mb
                .cache
                .direct_mv
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h)
                    .mb
                    .cache
                    .mv
                    .as_mut_ptr()
                    .offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(4 as ::core::ffi::c_int as isize) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as ::core::ffi::c_int;
            changed |= ((*((*(*(*h)
                .mb
                .cache
                .direct_mv
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h)
                    .mb
                    .cache
                    .mv
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(4 as ::core::ffi::c_int as isize) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as ::core::ffi::c_int;
            changed |= ((*((*(*(*h)
                .mb
                .cache
                .direct_mv
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(2 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h)
                    .mb
                    .cache
                    .mv
                    .as_mut_ptr()
                    .offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(8 as ::core::ffi::c_int as isize) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as ::core::ffi::c_int;
            changed |= ((*((*(*(*h)
                .mb
                .cache
                .direct_mv
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(2 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i
                ^ (*((*(*(*h)
                    .mb
                    .cache
                    .mv
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(8 as ::core::ffi::c_int as isize) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i) as ::core::ffi::c_int;
            changed |= (*h).mb.cache.direct_ref[0 as ::core::ffi::c_int as usize]
                [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                ^ (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[4 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int;
            changed |= (*h).mb.cache.direct_ref[1 as ::core::ffi::c_int as usize]
                [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                ^ (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                    [x264_scan8[4 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int;
            changed |= (*h).mb.cache.direct_ref[0 as ::core::ffi::c_int as usize]
                [2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                ^ (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[8 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int;
            changed |= (*h).mb.cache.direct_ref[1 as ::core::ffi::c_int as usize]
                [2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                ^ (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                    [x264_scan8[8 as ::core::ffi::c_int as usize] as usize]
                    as ::core::ffi::c_int;
        }
        *b_changed = changed;
        if changed == 0 {
            return b_available;
        }
    }
    if b_available != 0 {
        let mut l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while l < 2 as ::core::ffi::c_int {
            (*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(4 as ::core::ffi::c_int as isize) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(2 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(8 as ::core::ffi::c_int as isize) as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*((*(*(*h).mb.cache.direct_mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(3 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                .as_mut_ptr()
                .offset(
                    *x264_scan8
                        .as_ptr()
                        .offset(12 as ::core::ffi::c_int as isize) as isize,
                ))
            .as_mut_ptr() as *mut x264_union32_t))
                .i;
            (*h).mb.cache.direct_ref[l as usize][0 as ::core::ffi::c_int as usize] =
                (*h).mb.cache.ref_0[l as usize]
                    [x264_scan8[0 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.direct_ref[l as usize][1 as ::core::ffi::c_int as usize] =
                (*h).mb.cache.ref_0[l as usize]
                    [x264_scan8[4 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.direct_ref[l as usize][2 as ::core::ffi::c_int as usize] =
                (*h).mb.cache.ref_0[l as usize]
                    [x264_scan8[8 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.direct_ref[l as usize][3 as ::core::ffi::c_int as usize] =
                (*h).mb.cache.ref_0[l as usize]
                    [x264_scan8[12 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.direct_partition = (*h).mb.i_partition;
            l += 1;
        }
    }
    return b_available;
}
#[no_mangle]
#[c2rust::src_loc = "519:1"]
pub unsafe extern "C" fn x264_10_mb_predict_mv_ref16x16(
    mut h: *mut x264_t,
    mut i_list: ::core::ffi::c_int,
    mut i_ref: ::core::ffi::c_int,
    mut mvc: *mut [int16_t; 2],
    mut i_mvc: *mut ::core::ffi::c_int,
) {
    let mut mvr: *mut [int16_t; 2] = (*h).mb.mvr[i_list as usize][i_ref as usize];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int
        && (*h).mb.cache.ref_0[i_list as usize]
            [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
            as ::core::ffi::c_int
            == i_ref
    {
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset(
                    *x264_scan8
                        .as_ptr()
                        .offset(12 as ::core::ffi::c_int as isize) as isize,
                ))
            .as_mut_ptr() as *mut x264_union32_t))
                .i;
        i += 1;
    }
    if i_ref == 0 as ::core::ffi::c_int && (*h).frames.b_have_lowres != 0 {
        let mut idx: ::core::ffi::c_int = if i_list != 0 {
            (*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]).i_frame
                - (*(*h).fenc).i_frame
                - 1 as ::core::ffi::c_int
        } else {
            (*(*h).fenc).i_frame
                - (*(*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .i_frame
                - 1 as ::core::ffi::c_int
        };
        if idx <= (*h).param.i_bframe {
            let mut lowres_mv: *mut [int16_t; 2] =
                (*(*h).fenc).lowres_mvs[i_list as usize][idx as usize];
            if (*lowres_mv.offset(0 as ::core::ffi::c_int as isize))
                [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                != 0x7fff as ::core::ffi::c_int
            {
                (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
                    (*((*lowres_mv.offset((*h).mb.i_mb_xy as isize)).as_mut_ptr()
                        as *mut x264_union32_t))
                        .i
                        .wrapping_mul(2 as uint32_t)
                        & 0xfffeffff as uint32_t;
                i += 1;
            }
        }
    }
    if (*h).sh.b_mbaff != 0 {
        if (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] >= 0 as ::core::ffi::c_int {
            let mut shift: ::core::ffi::c_int = 1 as ::core::ffi::c_int + (*h).mb.b_interlaced
                - *(*h)
                    .mb
                    .field
                    .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
                    as ::core::ffi::c_int;
            let mut mvp: *mut int16_t = (*(*(*(*h).mb.mvr.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((i_ref << 1 as ::core::ffi::c_int >> shift) as isize))
            .offset(
                *(*h)
                    .mb
                    .i_mb_left_xy
                    .as_mut_ptr()
                    .offset(0 as ::core::ffi::c_int as isize) as isize,
            ))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0 as ::core::ffi::c_int as usize] =
                *mvp.offset(0 as ::core::ffi::c_int as isize);
            (*mvc.offset(i as isize))[1 as ::core::ffi::c_int as usize] =
                (*mvp.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * 2 as ::core::ffi::c_int
                    >> shift) as int16_t;
            i += 1;
        }
        if (*h).mb.i_mb_top_xy >= 0 as ::core::ffi::c_int {
            let mut shift_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int + (*h).mb.b_interlaced
                - *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int;
            let mut mvp_0: *mut int16_t = (*(*(*(*h).mb.mvr.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((i_ref << 1 as ::core::ffi::c_int >> shift_0) as isize))
            .offset((*h).mb.i_mb_top_xy as isize))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0 as ::core::ffi::c_int as usize] =
                *mvp_0.offset(0 as ::core::ffi::c_int as isize);
            (*mvc.offset(i as isize))[1 as ::core::ffi::c_int as usize] =
                (*mvp_0.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * 2 as ::core::ffi::c_int
                    >> shift_0) as int16_t;
            i += 1;
        }
        if (*h).mb.i_mb_topleft_xy >= 0 as ::core::ffi::c_int {
            let mut shift_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int + (*h).mb.b_interlaced
                - *(*h).mb.field.offset((*h).mb.i_mb_topleft_xy as isize) as ::core::ffi::c_int;
            let mut mvp_1: *mut int16_t = (*(*(*(*h).mb.mvr.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((i_ref << 1 as ::core::ffi::c_int >> shift_1) as isize))
            .offset((*h).mb.i_mb_topleft_xy as isize))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0 as ::core::ffi::c_int as usize] =
                *mvp_1.offset(0 as ::core::ffi::c_int as isize);
            (*mvc.offset(i as isize))[1 as ::core::ffi::c_int as usize] =
                (*mvp_1.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * 2 as ::core::ffi::c_int
                    >> shift_1) as int16_t;
            i += 1;
        }
        if (*h).mb.i_mb_topright_xy >= 0 as ::core::ffi::c_int {
            let mut shift_2: ::core::ffi::c_int = 1 as ::core::ffi::c_int + (*h).mb.b_interlaced
                - *(*h).mb.field.offset((*h).mb.i_mb_topright_xy as isize) as ::core::ffi::c_int;
            let mut mvp_2: *mut int16_t = (*(*(*(*h).mb.mvr.as_mut_ptr().offset(i_list as isize))
                .as_mut_ptr()
                .offset((i_ref << 1 as ::core::ffi::c_int >> shift_2) as isize))
            .offset((*h).mb.i_mb_topright_xy as isize))
            .as_mut_ptr();
            (*mvc.offset(i as isize))[0 as ::core::ffi::c_int as usize] =
                *mvp_2.offset(0 as ::core::ffi::c_int as isize);
            (*mvc.offset(i as isize))[1 as ::core::ffi::c_int as usize] =
                (*mvp_2.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * 2 as ::core::ffi::c_int
                    >> shift_2) as int16_t;
            i += 1;
        }
    } else {
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i = (*((*mvr.offset(
            *(*h)
                .mb
                .i_mb_left_xy
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize) as isize,
        ))
        .as_mut_ptr()
            as *mut x264_union32_t))
            .i;
        i += 1;
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*((*mvr.offset((*h).mb.i_mb_top_xy as isize)).as_mut_ptr() as *mut x264_union32_t)).i;
        i += 1;
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*((*mvr.offset((*h).mb.i_mb_topleft_xy as isize)).as_mut_ptr()
                as *mut x264_union32_t))
                .i;
        i += 1;
        (*((*mvc.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i =
            (*((*mvr.offset((*h).mb.i_mb_topright_xy as isize)).as_mut_ptr()
                as *mut x264_union32_t))
                .i;
        i += 1;
    }
    if (*(*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]).i_ref
        [0 as ::core::ffi::c_int as usize]
        > 0 as ::core::ffi::c_int
    {
        let mut l0: *mut x264_frame_t =
            (*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize];
        let mut field: ::core::ffi::c_int = (*h).mb.i_mb_y & 1 as ::core::ffi::c_int;
        let mut curpoc: ::core::ffi::c_int =
            (*(*h).fdec).i_poc + (*(*h).fdec).i_delta_poc[field as usize];
        let mut refpoc: ::core::ffi::c_int =
            (*(*h).fref[i_list as usize][(i_ref >> (*h).sh.b_mbaff) as usize]).i_poc;
        refpoc += (*l0).i_delta_poc[(field ^ i_ref & 1 as ::core::ffi::c_int) as usize];
        let mut mb_index: ::core::ffi::c_int = (*h).mb.i_mb_xy
            + 0 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * (*h).mb.i_mb_stride;
        let mut scale: ::core::ffi::c_int = (curpoc - refpoc)
            * (*l0).inv_ref_poc[((*h).mb.b_interlaced & field) as usize] as ::core::ffi::c_int;
        (*mvc.offset(i as isize))[0 as ::core::ffi::c_int as usize] = x264_clip3(
            (*(*l0).mv16x16.offset(mb_index as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                * scale
                + 128 as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int,
            INT16_MIN,
            INT16_MAX,
        ) as int16_t;
        (*mvc.offset(i as isize))[1 as ::core::ffi::c_int as usize] = x264_clip3(
            (*(*l0).mv16x16.offset(mb_index as isize))[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                * scale
                + 128 as ::core::ffi::c_int
                >> 8 as ::core::ffi::c_int,
            INT16_MIN,
            INT16_MAX,
        ) as int16_t;
        i += 1;
        if (*h).mb.i_mb_x < (*h).mb.i_mb_width - 1 as ::core::ffi::c_int {
            let mut mb_index_0: ::core::ffi::c_int = (*h).mb.i_mb_xy
                + 1 as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * (*h).mb.i_mb_stride;
            let mut scale_0: ::core::ffi::c_int = (curpoc - refpoc)
                * (*l0).inv_ref_poc[((*h).mb.b_interlaced & field) as usize] as ::core::ffi::c_int;
            (*mvc.offset(i as isize))[0 as ::core::ffi::c_int as usize] = x264_clip3(
                (*(*l0).mv16x16.offset(mb_index_0 as isize))[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * scale_0
                    + 128 as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int,
                INT16_MIN,
                INT16_MAX,
            ) as int16_t;
            (*mvc.offset(i as isize))[1 as ::core::ffi::c_int as usize] = x264_clip3(
                (*(*l0).mv16x16.offset(mb_index_0 as isize))[1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * scale_0
                    + 128 as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int,
                INT16_MIN,
                INT16_MAX,
            ) as int16_t;
            i += 1;
        }
        if (*h).mb.i_mb_y < (*h).mb.i_mb_height - 1 as ::core::ffi::c_int {
            let mut mb_index_1: ::core::ffi::c_int = (*h).mb.i_mb_xy
                + 0 as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * (*h).mb.i_mb_stride;
            let mut scale_1: ::core::ffi::c_int = (curpoc - refpoc)
                * (*l0).inv_ref_poc[((*h).mb.b_interlaced & field) as usize] as ::core::ffi::c_int;
            (*mvc.offset(i as isize))[0 as ::core::ffi::c_int as usize] = x264_clip3(
                (*(*l0).mv16x16.offset(mb_index_1 as isize))[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * scale_1
                    + 128 as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int,
                INT16_MIN,
                INT16_MAX,
            ) as int16_t;
            (*mvc.offset(i as isize))[1 as ::core::ffi::c_int as usize] = x264_clip3(
                (*(*l0).mv16x16.offset(mb_index_1 as isize))[1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    * scale_1
                    + 128 as ::core::ffi::c_int
                    >> 8 as ::core::ffi::c_int,
                INT16_MIN,
                INT16_MAX,
            ) as int16_t;
            i += 1;
        }
    }
    *i_mvc = i;
}
