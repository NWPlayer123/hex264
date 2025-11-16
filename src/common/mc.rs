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
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:27"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:27"]
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
#[c2rust::header_src = "/usr/include/stdint.h:27"]
pub mod stdint_h {
    #[c2rust::src_loc = "76:1"]
    pub type intptr_t = isize;
    #[c2rust::src_loc = "79:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/atomic_wide_counter.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/struct_mutex.h:27"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/common.h:27"]
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
    #[c2rust::src_loc = "111:9"]
    pub const SIZEOF_PIXEL: ::core::ffi::c_int = ::core::mem::size_of::<pixel>()
        as ::core::ffi::c_int;
    #[inline(always)]
    #[c2rust::src_loc = "145:1"]
    pub unsafe extern "C" fn x264_clip_pixel(mut x: ::core::ffi::c_int) -> pixel {
        return (if x & !PIXEL_MAX != 0 {
            -x >> 31 as ::core::ffi::c_int & PIXEL_MAX
        } else {
            x
        }) as pixel;
    }
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/frame.h:27"]
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
    #[c2rust::src_loc = "33:9"]
    pub const PADV: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
    #[c2rust::src_loc = "112:13"]
    pub const LOWRES_COST_SHIFT: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
    use super::pthreadtypes_h::{pthread_mutex_t, pthread_cond_t};
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
    use super::stdint_intn_h::{int64_t, int8_t, int16_t};
    use super::x264_h::{x264_param_t, x264_hrd_t, x264_sei_t};
    use super::common_h::pixel;
    use super::mc_h::x264_weight_t;
    use super::stdint_h::intptr_t;
    extern "C" {
        #[c2rust::src_loc = "232:1"]
        pub fn x264_10_frame_expand_border_lowres(frame: *mut x264_frame_t);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/mc.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/bitstream.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/cabac.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/quant.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/dct.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/pixel.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/predict.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/set.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/threadpool.h:27"]
pub mod threadpool_h {
    extern "C" {
        #[c2rust::src_loc = "29:16"]
        pub type x264_threadpool_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:27"]
pub mod base_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "65:9"]
    pub union x264_union32_t {
        pub i: uint32_t,
        pub w: [uint16_t; 2],
        pub b: [uint8_t; 4],
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
    use super::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:27"]
pub mod osdep_h {
    #[inline(always)]
    #[c2rust::src_loc = "496:1"]
    pub unsafe extern "C" fn endian_fix16(mut x: uint16_t) -> uint16_t {
        return ((x as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | x as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint16_t;
    }
    use super::stdint_uintn_h::uint16_t;
}
#[c2rust::header_src = "/usr/include/string.h:27"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/tables.h:27"]
pub mod tables_h {
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "49:22"]
        pub static x264_hpel_ref0: [uint8_t; 16];
        #[c2rust::src_loc = "50:22"]
        pub static x264_hpel_ref1: [uint8_t; 16];
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
    SIZEOF_PIXEL, x264_clip_pixel, FENC_STRIDE, FDEC_STRIDE, x264_ratecontrol_t,
};
pub use self::frame_h::{
    x264_sync_frame_list_t, x264_frame_t, x264_frame, x264_deblock_function_t,
    x264_deblock_intra_t, x264_deblock_inter_t, PADV, LOWRES_COST_SHIFT,
    x264_10_frame_expand_border_lowres,
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
pub use self::predict_h::{x264_predict_8x8_filter_t, x264_predict_t, x264_predict8x8_t};
pub use self::set_h::{
    x264_pps_t, x264_sps_t, C2RustUnnamed_14, C2RustUnnamed_15, C2RustUnnamed_16,
};
use self::threadpool_h::x264_threadpool_t;
pub use self::base_h::{
    x264_union32_t, chroma_format_e, CHROMA_444, CHROMA_422, CHROMA_420, CHROMA_400,
};
pub use self::osdep_h::endian_fix16;
use self::string_h::{memcpy, memset};
use self::tables_h::{x264_hpel_ref0, x264_hpel_ref1};
#[inline]
#[c2rust::src_loc = "49:1"]
unsafe extern "C" fn pixel_avg(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src1: *mut pixel,
    mut i_src1_stride: intptr_t,
    mut src2: *mut pixel,
    mut i_src2_stride: intptr_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < i_height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < i_width {
            *dst.offset(x as isize) = (*src1.offset(x as isize) as ::core::ffi::c_int
                + *src2.offset(x as isize) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
            x += 1;
        }
        dst = dst.offset(i_dst_stride as isize);
        src1 = src1.offset(i_src1_stride as isize);
        src2 = src2.offset(i_src2_stride as isize);
        y += 1;
    }
}
#[inline]
#[c2rust::src_loc = "63:1"]
unsafe extern "C" fn pixel_avg_wxh(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src1: *mut pixel,
    mut i_src1: intptr_t,
    mut src2: *mut pixel,
    mut i_src2: intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            *dst.offset(x as isize) = (*src1.offset(x as isize) as ::core::ffi::c_int
                + *src2.offset(x as isize) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as pixel;
            x += 1;
        }
        src1 = src1.offset(i_src1 as isize);
        src2 = src2.offset(i_src2 as isize);
        dst = dst.offset(i_dst as isize);
        y += 1;
    }
}
#[inline]
#[c2rust::src_loc = "79:1"]
unsafe extern "C" fn pixel_avg_weight_wxh(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src1: *mut pixel,
    mut i_src1: intptr_t,
    mut src2: *mut pixel,
    mut i_src2: intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut i_weight1: ::core::ffi::c_int,
) {
    let mut i_weight2: ::core::ffi::c_int = 64 as ::core::ffi::c_int - i_weight1;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            *dst.offset(x as isize) = x264_clip_pixel(
                *src1.offset(x as isize) as ::core::ffi::c_int * i_weight1
                    + *src2.offset(x as isize) as ::core::ffi::c_int * i_weight2
                    + ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int)
                    >> 6 as ::core::ffi::c_int,
            );
            x += 1;
        }
        y += 1;
        dst = dst.offset(i_dst as isize);
        src1 = src1.offset(i_src1 as isize);
        src2 = src2.offset(i_src2 as isize);
    }
}
#[c2rust::src_loc = "100:1"]
unsafe extern "C" fn pixel_avg_16x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "101:1"]
unsafe extern "C" fn pixel_avg_16x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "102:1"]
unsafe extern "C" fn pixel_avg_8x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "103:1"]
unsafe extern "C" fn pixel_avg_8x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "104:1"]
unsafe extern "C" fn pixel_avg_8x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            8 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "105:1"]
unsafe extern "C" fn pixel_avg_4x16(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "106:1"]
unsafe extern "C" fn pixel_avg_4x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "107:1"]
unsafe extern "C" fn pixel_avg_4x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "108:1"]
unsafe extern "C" fn pixel_avg_4x2(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "109:1"]
unsafe extern "C" fn pixel_avg_2x8(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "110:1"]
unsafe extern "C" fn pixel_avg_2x4(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "111:1"]
unsafe extern "C" fn pixel_avg_2x2(
    mut pix1: *mut pixel,
    mut i_stride_pix1: intptr_t,
    mut pix2: *mut pixel,
    mut i_stride_pix2: intptr_t,
    mut pix3: *mut pixel,
    mut i_stride_pix3: intptr_t,
    mut weight: ::core::ffi::c_int,
) {
    if weight == 32 as ::core::ffi::c_int {
        pixel_avg_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
    } else {
        pixel_avg_weight_wxh(
            pix1,
            i_stride_pix1,
            pix2,
            i_stride_pix2,
            pix3,
            i_stride_pix3,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            weight,
        );
    };
}
#[c2rust::src_loc = "113:1"]
unsafe extern "C" fn weight_cache(mut h: *mut x264_t, mut w: *mut x264_weight_t) {
    (*w).weightfn = (*h).mc.weight;
}
#[c2rust::src_loc = "119:1"]
unsafe extern "C" fn mc_weight(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) {
    let mut offset: ::core::ffi::c_int = (*weight).i_offset as ::core::ffi::c_int
        * ((1 as ::core::ffi::c_int) << BIT_DEPTH - 8 as ::core::ffi::c_int);
    let mut scale: ::core::ffi::c_int = (*weight).i_scale as ::core::ffi::c_int;
    let mut denom: ::core::ffi::c_int = (*weight).i_denom as ::core::ffi::c_int;
    if denom >= 1 as ::core::ffi::c_int {
        let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y < i_height {
            let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x < i_width {
                *dst.offset(x as isize) = x264_clip_pixel(
                    (*src.offset(x as isize) as ::core::ffi::c_int * scale
                        + ((1 as ::core::ffi::c_int) << denom - 1 as ::core::ffi::c_int)
                        >> denom) + offset,
                );
                x += 1;
            }
            y += 1;
            dst = dst.offset(i_dst_stride as isize);
            src = src.offset(i_src_stride as isize);
        }
    } else {
        let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while y_0 < i_height {
            let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while x_0 < i_width {
                *dst.offset(x_0 as isize) = x264_clip_pixel(
                    *src.offset(x_0 as isize) as ::core::ffi::c_int * scale + offset,
                );
                x_0 += 1;
            }
            y_0 += 1;
            dst = dst.offset(i_dst_stride as isize);
            src = src.offset(i_src_stride as isize);
        }
    };
}
#[c2rust::src_loc = "145:1"]
unsafe extern "C" fn mc_weight_w20(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        20 as ::core::ffi::c_int,
        height,
    );
}
#[c2rust::src_loc = "146:1"]
unsafe extern "C" fn mc_weight_w16(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        16 as ::core::ffi::c_int,
        height,
    );
}
#[c2rust::src_loc = "147:1"]
unsafe extern "C" fn mc_weight_w12(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        12 as ::core::ffi::c_int,
        height,
    );
}
#[c2rust::src_loc = "148:1"]
unsafe extern "C" fn mc_weight_w8(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        8 as ::core::ffi::c_int,
        height,
    );
}
#[c2rust::src_loc = "149:1"]
unsafe extern "C" fn mc_weight_w4(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        4 as ::core::ffi::c_int,
        height,
    );
}
#[c2rust::src_loc = "150:1"]
unsafe extern "C" fn mc_weight_w2(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut weight: *const x264_weight_t,
    mut height: ::core::ffi::c_int,
) {
    mc_weight(
        dst,
        i_dst_stride,
        src,
        i_src_stride,
        weight,
        2 as ::core::ffi::c_int,
        height,
    );
}
#[c2rust::src_loc = "152:20"]
static mut mc_weight_wtab: [weight_fn_t; 6] = unsafe {
    [
        Some(
            mc_weight_w2
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        Some(
            mc_weight_w4
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        Some(
            mc_weight_w8
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        Some(
            mc_weight_w12
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        Some(
            mc_weight_w16
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    ::core::ffi::c_int,
                ) -> (),
        ),
        Some(
            mc_weight_w20
                as unsafe extern "C" fn(
                    *mut pixel,
                    intptr_t,
                    *mut pixel,
                    intptr_t,
                    *const x264_weight_t,
                    ::core::ffi::c_int,
                ) -> (),
        ),
    ]
};
#[c2rust::src_loc = "162:1"]
unsafe extern "C" fn mc_copy(
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < i_height {
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (i_width * SIZEOF_PIXEL) as size_t,
        );
        src = src.offset(i_src_stride as isize);
        dst = dst.offset(i_dst_stride as isize);
        y += 1;
    }
}
#[c2rust::src_loc = "174:1"]
unsafe extern "C" fn hpel_filter(
    mut dsth: *mut pixel,
    mut dstv: *mut pixel,
    mut dstc: *mut pixel,
    mut src: *mut pixel,
    mut stride: intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut buf: *mut int16_t,
) {
    let pad: ::core::ffi::c_int = if BIT_DEPTH > 9 as ::core::ffi::c_int {
        -(10 as ::core::ffi::c_int) * PIXEL_MAX
    } else {
        0 as ::core::ffi::c_int
    };
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < height {
        let mut x: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
        while x < width + 3 as ::core::ffi::c_int {
            let mut v: ::core::ffi::c_int = *src
                .offset((x as intptr_t - 2 as intptr_t * stride) as isize)
                as ::core::ffi::c_int
                + *src.offset((x as intptr_t + 3 as intptr_t * stride) as isize)
                    as ::core::ffi::c_int
                - 5 as ::core::ffi::c_int
                    * (*src.offset((x as intptr_t - stride) as isize)
                        as ::core::ffi::c_int
                        + *src.offset((x as intptr_t + 2 as intptr_t * stride) as isize)
                            as ::core::ffi::c_int)
                + 20 as ::core::ffi::c_int
                    * (*src.offset(x as isize) as ::core::ffi::c_int
                        + *src.offset((x as intptr_t + stride) as isize)
                            as ::core::ffi::c_int);
            *dstv.offset(x as isize) = x264_clip_pixel(
                v + 16 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int,
            );
            *buf.offset((x + 2 as ::core::ffi::c_int) as isize) = (v + pad) as int16_t;
            x += 1;
        }
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < width {
            *dstc.offset(x_0 as isize) = x264_clip_pixel(
                *buf
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(
                        (x_0 - 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + *buf
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(
                            (x_0 + 3 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    - 5 as ::core::ffi::c_int
                        * (*buf
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset((x_0 - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            + *buf
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(
                                    (x_0 + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                        as isize,
                                ) as ::core::ffi::c_int)
                    + 20 as ::core::ffi::c_int
                        * (*buf
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset(x_0 as isize) as ::core::ffi::c_int
                            + *buf
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset((x_0 + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int) - 32 as ::core::ffi::c_int * pad
                    + 512 as ::core::ffi::c_int >> 10 as ::core::ffi::c_int,
            );
            x_0 += 1;
        }
        let mut x_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_1 < width {
            *dsth.offset(x_1 as isize) = x264_clip_pixel(
                *src
                    .offset(
                        (x_1 - 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                            as isize,
                    ) as ::core::ffi::c_int
                    + *src
                        .offset(
                            (x_1 + 3 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                    - 5 as ::core::ffi::c_int
                        * (*src.offset((x_1 - 1 as ::core::ffi::c_int) as isize)
                            as ::core::ffi::c_int
                            + *src
                                .offset(
                                    (x_1 + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                                        as isize,
                                ) as ::core::ffi::c_int)
                    + 20 as ::core::ffi::c_int
                        * (*src.offset(x_1 as isize) as ::core::ffi::c_int
                            + *src.offset((x_1 + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int) + 16 as ::core::ffi::c_int
                    >> 5 as ::core::ffi::c_int,
            );
            x_1 += 1;
        }
        dsth = dsth.offset(stride as isize);
        dstv = dstv.offset(stride as isize);
        dstc = dstc.offset(stride as isize);
        src = src.offset(stride as isize);
        y += 1;
    }
}
#[c2rust::src_loc = "198:1"]
unsafe extern "C" fn mc_luma(
    mut dst: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut *mut pixel,
    mut i_src_stride: intptr_t,
    mut mvx: ::core::ffi::c_int,
    mut mvy: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
    mut weight: *const x264_weight_t,
) {
    let mut qpel_idx: ::core::ffi::c_int = ((mvy & 3 as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int) + (mvx & 3 as ::core::ffi::c_int);
    let mut offset: ::core::ffi::c_int = ((mvy >> 2 as ::core::ffi::c_int) as intptr_t
        * i_src_stride + (mvx >> 2 as ::core::ffi::c_int) as intptr_t)
        as ::core::ffi::c_int;
    let mut src1: *mut pixel = (*src.offset(x264_hpel_ref0[qpel_idx as usize] as isize))
        .offset(offset as isize)
        .offset(
            ((mvy & 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int)
                as ::core::ffi::c_int as intptr_t * i_src_stride) as isize,
        );
    if qpel_idx & 5 as ::core::ffi::c_int != 0 {
        let mut src2: *mut pixel = (*src
            .offset(x264_hpel_ref1[qpel_idx as usize] as isize))
            .offset(offset as isize)
            .offset(
                (mvx & 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as isize,
            );
        pixel_avg(
            dst,
            i_dst_stride,
            src1,
            i_src_stride,
            src2,
            i_src_stride,
            i_width,
            i_height,
        );
        if !(*weight).weightfn.is_null() {
            mc_weight(dst, i_dst_stride, dst, i_dst_stride, weight, i_width, i_height);
        }
    } else if !(*weight).weightfn.is_null() {
        mc_weight(dst, i_dst_stride, src1, i_src_stride, weight, i_width, i_height);
    } else {
        mc_copy(src1, i_src_stride, dst, i_dst_stride, i_width, i_height);
    };
}
#[c2rust::src_loc = "221:1"]
unsafe extern "C" fn get_ref(
    mut dst: *mut pixel,
    mut i_dst_stride: *mut intptr_t,
    mut src: *mut *mut pixel,
    mut i_src_stride: intptr_t,
    mut mvx: ::core::ffi::c_int,
    mut mvy: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
    mut weight: *const x264_weight_t,
) -> *mut pixel {
    let mut qpel_idx: ::core::ffi::c_int = ((mvy & 3 as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int) + (mvx & 3 as ::core::ffi::c_int);
    let mut offset: ::core::ffi::c_int = ((mvy >> 2 as ::core::ffi::c_int) as intptr_t
        * i_src_stride + (mvx >> 2 as ::core::ffi::c_int) as intptr_t)
        as ::core::ffi::c_int;
    let mut src1: *mut pixel = (*src.offset(x264_hpel_ref0[qpel_idx as usize] as isize))
        .offset(offset as isize)
        .offset(
            ((mvy & 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int)
                as ::core::ffi::c_int as intptr_t * i_src_stride) as isize,
        );
    if qpel_idx & 5 as ::core::ffi::c_int != 0 {
        let mut src2: *mut pixel = (*src
            .offset(x264_hpel_ref1[qpel_idx as usize] as isize))
            .offset(offset as isize)
            .offset(
                (mvx & 3 as ::core::ffi::c_int == 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_int as isize,
            );
        pixel_avg(
            dst,
            *i_dst_stride,
            src1,
            i_src_stride,
            src2,
            i_src_stride,
            i_width,
            i_height,
        );
        if !(*weight).weightfn.is_null() {
            mc_weight(dst, *i_dst_stride, dst, *i_dst_stride, weight, i_width, i_height);
        }
        return dst;
    } else if !(*weight).weightfn.is_null() {
        mc_weight(dst, *i_dst_stride, src1, i_src_stride, weight, i_width, i_height);
        return dst;
    } else {
        *i_dst_stride = i_src_stride;
        return src1;
    };
}
#[c2rust::src_loc = "252:1"]
unsafe extern "C" fn mc_chroma(
    mut dstu: *mut pixel,
    mut dstv: *mut pixel,
    mut i_dst_stride: intptr_t,
    mut src: *mut pixel,
    mut i_src_stride: intptr_t,
    mut mvx: ::core::ffi::c_int,
    mut mvy: ::core::ffi::c_int,
    mut i_width: ::core::ffi::c_int,
    mut i_height: ::core::ffi::c_int,
) {
    let mut srcp: *mut pixel = 0 as *mut pixel;
    let mut d8x: ::core::ffi::c_int = mvx & 0x7 as ::core::ffi::c_int;
    let mut d8y: ::core::ffi::c_int = mvy & 0x7 as ::core::ffi::c_int;
    let mut cA: ::core::ffi::c_int = (8 as ::core::ffi::c_int - d8x)
        * (8 as ::core::ffi::c_int - d8y);
    let mut cB: ::core::ffi::c_int = d8x * (8 as ::core::ffi::c_int - d8y);
    let mut cC: ::core::ffi::c_int = (8 as ::core::ffi::c_int - d8x) * d8y;
    let mut cD: ::core::ffi::c_int = d8x * d8y;
    src = src
        .offset(
            ((mvy >> 3 as ::core::ffi::c_int) as intptr_t * i_src_stride
                + ((mvx >> 3 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int)
                    as intptr_t) as isize,
        );
    srcp = &mut *src.offset(i_src_stride as isize) as *mut pixel;
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < i_height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < i_width {
            *dstu.offset(x as isize) = (cA
                * *src.offset((2 as ::core::ffi::c_int * x) as isize)
                    as ::core::ffi::c_int
                + cB
                    * *src
                        .offset(
                            (2 as ::core::ffi::c_int * x + 2 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                + cC
                    * *srcp.offset((2 as ::core::ffi::c_int * x) as isize)
                        as ::core::ffi::c_int
                + cD
                    * *srcp
                        .offset(
                            (2 as ::core::ffi::c_int * x + 2 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as pixel;
            *dstv.offset(x as isize) = (cA
                * *src
                    .offset(
                        (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                + cB
                    * *src
                        .offset(
                            (2 as ::core::ffi::c_int * x + 3 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                + cC
                    * *srcp
                        .offset(
                            (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int
                + cD
                    * *srcp
                        .offset(
                            (2 as ::core::ffi::c_int * x + 3 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int) as pixel;
            x += 1;
        }
        dstu = dstu.offset(i_dst_stride as isize);
        dstv = dstv.offset(i_dst_stride as isize);
        src = srcp;
        srcp = srcp.offset(i_src_stride as isize);
        y += 1;
    }
}
#[c2rust::src_loc = "290:1"]
unsafe extern "C" fn mc_copy_w16(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut i_height: ::core::ffi::c_int,
) {
    mc_copy(src, i_src, dst, i_dst, 16 as ::core::ffi::c_int, i_height);
}
#[c2rust::src_loc = "291:1"]
unsafe extern "C" fn mc_copy_w8(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut i_height: ::core::ffi::c_int,
) {
    mc_copy(src, i_src, dst, i_dst, 8 as ::core::ffi::c_int, i_height);
}
#[c2rust::src_loc = "292:1"]
unsafe extern "C" fn mc_copy_w4(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut i_height: ::core::ffi::c_int,
) {
    mc_copy(src, i_src, dst, i_dst, 4 as ::core::ffi::c_int, i_height);
}
#[no_mangle]
#[c2rust::src_loc = "294:1"]
pub unsafe extern "C" fn x264_10_plane_copy_c(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    loop {
        let fresh0 = h;
        h = h - 1;
        if !(fresh0 != 0) {
            break;
        }
        memcpy(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            (w * SIZEOF_PIXEL) as size_t,
        );
        dst = dst.offset(i_dst as isize);
        src = src.offset(i_src as isize);
    };
}
#[no_mangle]
#[c2rust::src_loc = "305:1"]
pub unsafe extern "C" fn x264_10_plane_copy_swap_c(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < h {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 2 as ::core::ffi::c_int * w {
            *dst.offset(x as isize) = *src
                .offset((x + 1 as ::core::ffi::c_int) as isize);
            *dst.offset((x + 1 as ::core::ffi::c_int) as isize) = *src
                .offset(x as isize);
            x += 2 as ::core::ffi::c_int;
        }
        y += 1;
        dst = dst.offset(i_dst as isize);
        src = src.offset(i_src as isize);
    }
}
#[no_mangle]
#[c2rust::src_loc = "316:1"]
pub unsafe extern "C" fn x264_10_plane_copy_interleave_c(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut srcu: *mut pixel,
    mut i_srcu: intptr_t,
    mut srcv: *mut pixel,
    mut i_srcv: intptr_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < h {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < w {
            *dst.offset((2 as ::core::ffi::c_int * x) as isize) = *srcu
                .offset(x as isize);
            *dst
                .offset(
                    (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize,
                ) = *srcv.offset(x as isize);
            x += 1;
        }
        y += 1;
        dst = dst.offset(i_dst as isize);
        srcu = srcu.offset(i_srcu as isize);
        srcv = srcv.offset(i_srcv as isize);
    }
}
#[no_mangle]
#[c2rust::src_loc = "328:1"]
pub unsafe extern "C" fn x264_10_plane_copy_deinterleave_c(
    mut dsta: *mut pixel,
    mut i_dsta: intptr_t,
    mut dstb: *mut pixel,
    mut i_dstb: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < h {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < w {
            *dsta.offset(x as isize) = *src
                .offset((2 as ::core::ffi::c_int * x) as isize);
            *dstb.offset(x as isize) = *src
                .offset(
                    (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize,
                );
            x += 1;
        }
        y += 1;
        dsta = dsta.offset(i_dsta as isize);
        dstb = dstb.offset(i_dstb as isize);
        src = src.offset(i_src as isize);
    }
}
#[c2rust::src_loc = "339:1"]
unsafe extern "C" fn plane_copy_deinterleave_rgb_c(
    mut dsta: *mut pixel,
    mut i_dsta: intptr_t,
    mut dstb: *mut pixel,
    mut i_dstb: intptr_t,
    mut dstc: *mut pixel,
    mut i_dstc: intptr_t,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut pw: ::core::ffi::c_int,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < h {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < w {
            *dsta.offset(x as isize) = *src.offset((x * pw) as isize);
            *dstb.offset(x as isize) = *src
                .offset((x * pw + 1 as ::core::ffi::c_int) as isize);
            *dstc.offset(x as isize) = *src
                .offset((x * pw + 2 as ::core::ffi::c_int) as isize);
            x += 1;
        }
        y += 1;
        dsta = dsta.offset(i_dsta as isize);
        dstb = dstb.offset(i_dstb as isize);
        dstc = dstc.offset(i_dstc as isize);
        src = src.offset(i_src as isize);
    }
}
#[c2rust::src_loc = "364:1"]
unsafe extern "C" fn plane_copy_deinterleave_v210_c(
    mut dsty: *mut pixel,
    mut i_dsty: intptr_t,
    mut dstc: *mut pixel,
    mut i_dstc: intptr_t,
    mut src: *mut uint32_t,
    mut i_src: intptr_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
) {
    let mut l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while l < h {
        let mut dsty0: *mut pixel = dsty;
        let mut dstc0: *mut pixel = dstc;
        let mut src0: *mut uint32_t = src;
        let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while n < w {
            let fresh1 = src0;
            src0 = src0.offset(1);
            let mut s: uint32_t = *fresh1;
            let fresh2 = dstc0;
            dstc0 = dstc0.offset(1);
            *fresh2 = (s & 0x3ff as uint32_t) as pixel;
            let fresh3 = dsty0;
            dsty0 = dsty0.offset(1);
            *fresh3 = (s >> 10 as ::core::ffi::c_int & 0x3ff as uint32_t) as pixel;
            let fresh4 = dstc0;
            dstc0 = dstc0.offset(1);
            *fresh4 = (s >> 20 as ::core::ffi::c_int & 0x3ff as uint32_t) as pixel;
            let fresh5 = src0;
            src0 = src0.offset(1);
            s = *fresh5;
            let fresh6 = dsty0;
            dsty0 = dsty0.offset(1);
            *fresh6 = (s & 0x3ff as uint32_t) as pixel;
            let fresh7 = dstc0;
            dstc0 = dstc0.offset(1);
            *fresh7 = (s >> 10 as ::core::ffi::c_int & 0x3ff as uint32_t) as pixel;
            let fresh8 = dsty0;
            dsty0 = dsty0.offset(1);
            *fresh8 = (s >> 20 as ::core::ffi::c_int & 0x3ff as uint32_t) as pixel;
            n += 3 as ::core::ffi::c_int;
        }
        dsty = dsty.offset(i_dsty as isize);
        dstc = dstc.offset(i_dstc as isize);
        src = src.offset(i_src as isize);
        l += 1;
    }
}
#[c2rust::src_loc = "392:1"]
unsafe extern "C" fn store_interleave_chroma(
    mut dst: *mut pixel,
    mut i_dst: intptr_t,
    mut srcu: *mut pixel,
    mut srcv: *mut pixel,
    mut height: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < height {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 8 as ::core::ffi::c_int {
            *dst.offset((2 as ::core::ffi::c_int * x) as isize) = *srcu
                .offset(x as isize);
            *dst
                .offset(
                    (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize,
                ) = *srcv.offset(x as isize);
            x += 1;
        }
        y += 1;
        dst = dst.offset(i_dst as isize);
        srcu = srcu.offset(FDEC_STRIDE as isize);
        srcv = srcv.offset(FDEC_STRIDE as isize);
    }
}
#[c2rust::src_loc = "402:1"]
unsafe extern "C" fn load_deinterleave_chroma_fenc(
    mut dst: *mut pixel,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut height: ::core::ffi::c_int,
) {
    x264_10_plane_copy_deinterleave_c(
        dst,
        FENC_STRIDE as intptr_t,
        dst.offset((FENC_STRIDE / 2 as ::core::ffi::c_int) as isize),
        FENC_STRIDE as intptr_t,
        src,
        i_src,
        8 as ::core::ffi::c_int,
        height,
    );
}
#[c2rust::src_loc = "407:1"]
unsafe extern "C" fn load_deinterleave_chroma_fdec(
    mut dst: *mut pixel,
    mut src: *mut pixel,
    mut i_src: intptr_t,
    mut height: ::core::ffi::c_int,
) {
    x264_10_plane_copy_deinterleave_c(
        dst,
        FDEC_STRIDE as intptr_t,
        dst.offset((FDEC_STRIDE / 2 as ::core::ffi::c_int) as isize),
        FDEC_STRIDE as intptr_t,
        src,
        i_src,
        8 as ::core::ffi::c_int,
        height,
    );
}
#[c2rust::src_loc = "412:1"]
unsafe extern "C" fn prefetch_fenc_null(
    mut pix_y: *mut pixel,
    mut stride_y: intptr_t,
    mut pix_uv: *mut pixel,
    mut stride_uv: intptr_t,
    mut mb_x: ::core::ffi::c_int,
) {}
#[c2rust::src_loc = "416:1"]
unsafe extern "C" fn prefetch_ref_null(
    mut pix: *mut pixel,
    mut stride: intptr_t,
    mut parity: ::core::ffi::c_int,
) {}
#[c2rust::src_loc = "419:1"]
unsafe extern "C" fn memzero_aligned(mut dst: *mut ::core::ffi::c_void, mut n: size_t) {
    memset(dst, 0 as ::core::ffi::c_int, n);
}
#[c2rust::src_loc = "424:1"]
unsafe extern "C" fn integral_init4h(
    mut sum: *mut uint16_t,
    mut pix: *mut pixel,
    mut stride: intptr_t,
) {
    let mut v: ::core::ffi::c_int = *pix.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        + *pix.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + *pix.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + *pix.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (x as intptr_t) < stride - 4 as intptr_t {
        *sum.offset(x as isize) = (v
            + *sum.offset((x as intptr_t - stride) as isize) as ::core::ffi::c_int)
            as uint16_t;
        v
            += *pix.offset((x + 4 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                - *pix.offset(x as isize) as ::core::ffi::c_int;
        x += 1;
    }
}
#[c2rust::src_loc = "434:1"]
unsafe extern "C" fn integral_init8h(
    mut sum: *mut uint16_t,
    mut pix: *mut pixel,
    mut stride: intptr_t,
) {
    let mut v: ::core::ffi::c_int = *pix.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        + *pix.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + *pix.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + *pix.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + *pix.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + *pix.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + *pix.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        + *pix.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (x as intptr_t) < stride - 8 as intptr_t {
        *sum.offset(x as isize) = (v
            + *sum.offset((x as intptr_t - stride) as isize) as ::core::ffi::c_int)
            as uint16_t;
        v
            += *pix.offset((x + 8 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                - *pix.offset(x as isize) as ::core::ffi::c_int;
        x += 1;
    }
}
#[c2rust::src_loc = "444:1"]
unsafe extern "C" fn integral_init4v(
    mut sum8: *mut uint16_t,
    mut sum4: *mut uint16_t,
    mut stride: intptr_t,
) {
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (x as intptr_t) < stride - 8 as intptr_t {
        *sum4.offset(x as isize) = (*sum8
            .offset((x as intptr_t + 4 as intptr_t * stride) as isize)
            as ::core::ffi::c_int - *sum8.offset(x as isize) as ::core::ffi::c_int)
            as uint16_t;
        x += 1;
    }
    let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (x_0 as intptr_t) < stride - 8 as intptr_t {
        *sum8.offset(x_0 as isize) = (*sum8
            .offset((x_0 as intptr_t + 8 as intptr_t * stride) as isize)
            as ::core::ffi::c_int
            + *sum8
                .offset(
                    (x_0 as intptr_t + 8 as intptr_t * stride + 4 as intptr_t) as isize,
                ) as ::core::ffi::c_int
            - *sum8.offset(x_0 as isize) as ::core::ffi::c_int
            - *sum8.offset((x_0 + 4 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int) as uint16_t;
        x_0 += 1;
    }
}
#[c2rust::src_loc = "452:1"]
unsafe extern "C" fn integral_init8v(mut sum8: *mut uint16_t, mut stride: intptr_t) {
    let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (x as intptr_t) < stride - 8 as intptr_t {
        *sum8.offset(x as isize) = (*sum8
            .offset((x as intptr_t + 8 as intptr_t * stride) as isize)
            as ::core::ffi::c_int - *sum8.offset(x as isize) as ::core::ffi::c_int)
            as uint16_t;
        x += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "458:1"]
pub unsafe extern "C" fn x264_10_frame_init_lowres(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
) {
    let mut src: *mut pixel = (*frame).plane[0 as ::core::ffi::c_int as usize];
    let mut i_stride: ::core::ffi::c_int = (*frame)
        .i_stride[0 as ::core::ffi::c_int as usize];
    let mut i_height: ::core::ffi::c_int = (*frame)
        .i_lines[0 as ::core::ffi::c_int as usize];
    let mut i_width: ::core::ffi::c_int = (*frame)
        .i_width[0 as ::core::ffi::c_int as usize];
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < i_height {
        *src.offset((i_width + y * i_stride) as isize) = *src
            .offset((i_width - 1 as ::core::ffi::c_int + y * i_stride) as isize);
        y += 1;
    }
    memcpy(
        src.offset((i_stride * i_height) as isize) as *mut ::core::ffi::c_void,
        src.offset((i_stride * (i_height - 1 as ::core::ffi::c_int)) as isize)
            as *const ::core::ffi::c_void,
        ((i_width + 1 as ::core::ffi::c_int) * SIZEOF_PIXEL) as size_t,
    );
    (*h)
        .mc
        .frame_init_lowres_core
        .expect(
            "non-null function pointer",
        )(
        src,
        (*frame).lowres[0 as ::core::ffi::c_int as usize],
        (*frame).lowres[1 as ::core::ffi::c_int as usize],
        (*frame).lowres[2 as ::core::ffi::c_int as usize],
        (*frame).lowres[3 as ::core::ffi::c_int as usize],
        i_stride as intptr_t,
        (*frame).i_stride_lowres as intptr_t,
        (*frame).i_width_lowres,
        (*frame).i_lines_lowres,
    );
    x264_10_frame_expand_border_lowres(frame);
    memset(
        (*frame).i_cost_est.as_mut_ptr() as *mut ::core::ffi::c_void,
        -(1 as ::core::ffi::c_int),
        ::core::mem::size_of::<[[::core::ffi::c_int; 18]; 18]>() as size_t,
    );
    let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y_0 < (*h).param.i_bframe + 2 as ::core::ffi::c_int {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < (*h).param.i_bframe + 2 as ::core::ffi::c_int {
            *(*frame)
                .i_row_satds[y_0 as usize][x as usize]
                .offset(0 as ::core::ffi::c_int as isize) = -(1 as ::core::ffi::c_int);
            x += 1;
        }
        y_0 += 1;
    }
    let mut y_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y_1 <= ((*h).param.i_bframe != 0) as ::core::ffi::c_int {
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 <= (*h).param.i_bframe {
            (*(*frame)
                .lowres_mvs[y_1 as usize][x_0 as usize]
                .offset(
                    0 as ::core::ffi::c_int as isize,
                ))[0 as ::core::ffi::c_int as usize] = 0x7fff as int16_t;
            x_0 += 1;
        }
        y_1 += 1;
    }
}
#[c2rust::src_loc = "484:1"]
unsafe extern "C" fn frame_init_lowres_core(
    mut src0: *mut pixel,
    mut dst0: *mut pixel,
    mut dsth: *mut pixel,
    mut dstv: *mut pixel,
    mut dstc: *mut pixel,
    mut src_stride: intptr_t,
    mut dst_stride: intptr_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y < height {
        let mut src1: *mut pixel = src0.offset(src_stride as isize);
        let mut src2: *mut pixel = src1.offset(src_stride as isize);
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < width {
            *dst0.offset(x as isize) = ((*src0
                .offset((2 as ::core::ffi::c_int * x) as isize) as ::core::ffi::c_int
                + *src1.offset((2 as ::core::ffi::c_int * x) as isize)
                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                + (*src0
                    .offset(
                        (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    + *src1
                        .offset(
                            (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as pixel;
            *dsth.offset(x as isize) = ((*src0
                .offset((2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *src1
                    .offset(
                        (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                + (*src0
                    .offset(
                        (2 as ::core::ffi::c_int * x + 2 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    + *src1
                        .offset(
                            (2 as ::core::ffi::c_int * x + 2 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as pixel;
            *dstv.offset(x as isize) = ((*src1
                .offset((2 as ::core::ffi::c_int * x) as isize) as ::core::ffi::c_int
                + *src2.offset((2 as ::core::ffi::c_int * x) as isize)
                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                + (*src1
                    .offset(
                        (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    + *src2
                        .offset(
                            (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as pixel;
            *dstc.offset(x as isize) = ((*src1
                .offset((2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                + *src2
                    .offset(
                        (2 as ::core::ffi::c_int * x + 1 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int)
                + (*src1
                    .offset(
                        (2 as ::core::ffi::c_int * x + 2 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int
                    + *src2
                        .offset(
                            (2 as ::core::ffi::c_int * x + 2 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as pixel;
            x += 1;
        }
        src0 = src0.offset((src_stride * 2 as intptr_t) as isize);
        dst0 = dst0.offset(dst_stride as isize);
        dsth = dsth.offset(dst_stride as isize);
        dstv = dstv.offset(dst_stride as isize);
        dstc = dstc.offset(dst_stride as isize);
        y += 1;
    }
}
#[c2rust::src_loc = "511:1"]
unsafe extern "C" fn mbtree_propagate_cost(
    mut dst: *mut int16_t,
    mut propagate_in: *mut uint16_t,
    mut intra_costs: *mut uint16_t,
    mut inter_costs: *mut uint16_t,
    mut inv_qscales: *mut uint16_t,
    mut fps_factor: *mut ::core::ffi::c_float,
    mut len: ::core::ffi::c_int,
) {
    let mut fps: ::core::ffi::c_float = *fps_factor;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        let mut intra_cost: ::core::ffi::c_int = *intra_costs.offset(i as isize)
            as ::core::ffi::c_int;
        let mut inter_cost: ::core::ffi::c_int = if (*intra_costs.offset(i as isize)
            as ::core::ffi::c_int)
            < *inter_costs.offset(i as isize) as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int
        {
            *intra_costs.offset(i as isize) as ::core::ffi::c_int
        } else {
            *inter_costs.offset(i as isize) as ::core::ffi::c_int
                & ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int
        };
        let mut propagate_intra: ::core::ffi::c_float = (intra_cost
            * *inv_qscales.offset(i as isize) as ::core::ffi::c_int)
            as ::core::ffi::c_float;
        let mut propagate_amount: ::core::ffi::c_float = *propagate_in.offset(i as isize)
            as ::core::ffi::c_int as ::core::ffi::c_float + propagate_intra * fps;
        let mut propagate_num: ::core::ffi::c_float = (intra_cost - inter_cost)
            as ::core::ffi::c_float;
        let mut propagate_denom: ::core::ffi::c_float = intra_cost
            as ::core::ffi::c_float;
        *dst.offset(i as isize) = (if ((propagate_amount * propagate_num
            / propagate_denom + 0.5f32) as ::core::ffi::c_int)
            < 32767 as ::core::ffi::c_int
        {
            (propagate_amount * propagate_num / propagate_denom + 0.5f32)
                as ::core::ffi::c_int
        } else {
            32767 as ::core::ffi::c_int
        }) as int16_t;
        i += 1;
    }
}
#[c2rust::src_loc = "527:1"]
unsafe extern "C" fn mbtree_propagate_list(
    mut h: *mut x264_t,
    mut ref_costs: *mut uint16_t,
    mut mvs: *mut [int16_t; 2],
    mut propagate_amount: *mut int16_t,
    mut lowres_costs: *mut uint16_t,
    mut bipred_weight: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut len: ::core::ffi::c_int,
    mut list: ::core::ffi::c_int,
) {
    let mut stride: ::core::ffi::c_uint = (*h).mb.i_mb_stride as ::core::ffi::c_uint;
    let mut width: ::core::ffi::c_uint = (*h).mb.i_mb_width as ::core::ffi::c_uint;
    let mut height: ::core::ffi::c_uint = (*h).mb.i_mb_height as ::core::ffi::c_uint;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < len {
        let mut lists_used: ::core::ffi::c_int = *lowres_costs.offset(i as isize)
            as ::core::ffi::c_int >> LOWRES_COST_SHIFT;
        if !(lists_used & (1 as ::core::ffi::c_int) << list == 0) {
            let mut listamount: ::core::ffi::c_int = *propagate_amount.offset(i as isize)
                as ::core::ffi::c_int;
            if lists_used == 3 as ::core::ffi::c_int {
                listamount = listamount * bipred_weight + 32 as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int;
            }
            if (*((*mvs.offset(i as isize)).as_mut_ptr() as *mut x264_union32_t)).i == 0
            {
                *ref_costs
                    .offset(
                        (mb_y as ::core::ffi::c_uint)
                            .wrapping_mul(stride)
                            .wrapping_add(i as ::core::ffi::c_uint) as isize,
                    ) = (if *ref_costs
                    .offset(
                        (mb_y as ::core::ffi::c_uint)
                            .wrapping_mul(stride)
                            .wrapping_add(i as ::core::ffi::c_uint) as isize,
                    ) as ::core::ffi::c_int + listamount
                    < ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int
                {
                    *ref_costs
                        .offset(
                            (mb_y as ::core::ffi::c_uint)
                                .wrapping_mul(stride)
                                .wrapping_add(i as ::core::ffi::c_uint) as isize,
                        ) as ::core::ffi::c_int + listamount
                } else {
                    ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int
                }) as uint16_t;
            } else {
                let mut x: ::core::ffi::c_int = (*mvs
                    .offset(i as isize))[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                let mut y: ::core::ffi::c_int = (*mvs
                    .offset(i as isize))[1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                let mut mbx: ::core::ffi::c_uint = ((x >> 5 as ::core::ffi::c_int) + i)
                    as ::core::ffi::c_uint;
                let mut mby: ::core::ffi::c_uint = ((y >> 5 as ::core::ffi::c_int)
                    + mb_y) as ::core::ffi::c_uint;
                let mut idx0: ::core::ffi::c_uint = mbx
                    .wrapping_add(mby.wrapping_mul(stride));
                let mut idx2: ::core::ffi::c_uint = idx0.wrapping_add(stride);
                x &= 31 as ::core::ffi::c_int;
                y &= 31 as ::core::ffi::c_int;
                let mut idx0weight: ::core::ffi::c_int = (32 as ::core::ffi::c_int - y)
                    * (32 as ::core::ffi::c_int - x);
                let mut idx1weight: ::core::ffi::c_int = (32 as ::core::ffi::c_int - y)
                    * x;
                let mut idx2weight: ::core::ffi::c_int = y
                    * (32 as ::core::ffi::c_int - x);
                let mut idx3weight: ::core::ffi::c_int = y * x;
                idx0weight = idx0weight * listamount + 512 as ::core::ffi::c_int
                    >> 10 as ::core::ffi::c_int;
                idx1weight = idx1weight * listamount + 512 as ::core::ffi::c_int
                    >> 10 as ::core::ffi::c_int;
                idx2weight = idx2weight * listamount + 512 as ::core::ffi::c_int
                    >> 10 as ::core::ffi::c_int;
                idx3weight = idx3weight * listamount + 512 as ::core::ffi::c_int
                    >> 10 as ::core::ffi::c_int;
                if mbx < width.wrapping_sub(1 as ::core::ffi::c_uint)
                    && mby < height.wrapping_sub(1 as ::core::ffi::c_uint)
                {
                    *ref_costs
                        .offset(idx0.wrapping_add(0 as ::core::ffi::c_uint) as isize) = (if *ref_costs
                        .offset(idx0.wrapping_add(0 as ::core::ffi::c_uint) as isize)
                        as ::core::ffi::c_int + idx0weight
                        < ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int
                    {
                        *ref_costs
                            .offset(idx0.wrapping_add(0 as ::core::ffi::c_uint) as isize)
                            as ::core::ffi::c_int + idx0weight
                    } else {
                        ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int
                    }) as uint16_t;
                    *ref_costs
                        .offset(idx0.wrapping_add(1 as ::core::ffi::c_uint) as isize) = (if *ref_costs
                        .offset(idx0.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                        as ::core::ffi::c_int + idx1weight
                        < ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int
                    {
                        *ref_costs
                            .offset(idx0.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                            as ::core::ffi::c_int + idx1weight
                    } else {
                        ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int
                    }) as uint16_t;
                    *ref_costs
                        .offset(idx2.wrapping_add(0 as ::core::ffi::c_uint) as isize) = (if *ref_costs
                        .offset(idx2.wrapping_add(0 as ::core::ffi::c_uint) as isize)
                        as ::core::ffi::c_int + idx2weight
                        < ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int
                    {
                        *ref_costs
                            .offset(idx2.wrapping_add(0 as ::core::ffi::c_uint) as isize)
                            as ::core::ffi::c_int + idx2weight
                    } else {
                        ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int
                    }) as uint16_t;
                    *ref_costs
                        .offset(idx2.wrapping_add(1 as ::core::ffi::c_uint) as isize) = (if *ref_costs
                        .offset(idx2.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                        as ::core::ffi::c_int + idx3weight
                        < ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int
                    {
                        *ref_costs
                            .offset(idx2.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                            as ::core::ffi::c_int + idx3weight
                    } else {
                        ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int
                    }) as uint16_t;
                } else {
                    if mby < height {
                        if mbx < width {
                            *ref_costs
                                .offset(
                                    idx0.wrapping_add(0 as ::core::ffi::c_uint) as isize,
                                ) = (if *ref_costs
                                .offset(
                                    idx0.wrapping_add(0 as ::core::ffi::c_uint) as isize,
                                ) as ::core::ffi::c_int + idx0weight
                                < ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int
                            {
                                *ref_costs
                                    .offset(
                                        idx0.wrapping_add(0 as ::core::ffi::c_uint) as isize,
                                    ) as ::core::ffi::c_int + idx0weight
                            } else {
                                ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int
                            }) as uint16_t;
                        }
                        if mbx.wrapping_add(1 as ::core::ffi::c_uint) < width {
                            *ref_costs
                                .offset(
                                    idx0.wrapping_add(1 as ::core::ffi::c_uint) as isize,
                                ) = (if *ref_costs
                                .offset(
                                    idx0.wrapping_add(1 as ::core::ffi::c_uint) as isize,
                                ) as ::core::ffi::c_int + idx1weight
                                < ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int
                            {
                                *ref_costs
                                    .offset(
                                        idx0.wrapping_add(1 as ::core::ffi::c_uint) as isize,
                                    ) as ::core::ffi::c_int + idx1weight
                            } else {
                                ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int
                            }) as uint16_t;
                        }
                    }
                    if mby.wrapping_add(1 as ::core::ffi::c_uint) < height {
                        if mbx < width {
                            *ref_costs
                                .offset(
                                    idx2.wrapping_add(0 as ::core::ffi::c_uint) as isize,
                                ) = (if *ref_costs
                                .offset(
                                    idx2.wrapping_add(0 as ::core::ffi::c_uint) as isize,
                                ) as ::core::ffi::c_int + idx2weight
                                < ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int
                            {
                                *ref_costs
                                    .offset(
                                        idx2.wrapping_add(0 as ::core::ffi::c_uint) as isize,
                                    ) as ::core::ffi::c_int + idx2weight
                            } else {
                                ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int
                            }) as uint16_t;
                        }
                        if mbx.wrapping_add(1 as ::core::ffi::c_uint) < width {
                            *ref_costs
                                .offset(
                                    idx2.wrapping_add(1 as ::core::ffi::c_uint) as isize,
                                ) = (if *ref_costs
                                .offset(
                                    idx2.wrapping_add(1 as ::core::ffi::c_uint) as isize,
                                ) as ::core::ffi::c_int + idx3weight
                                < ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int
                            {
                                *ref_costs
                                    .offset(
                                        idx2.wrapping_add(1 as ::core::ffi::c_uint) as isize,
                                    ) as ::core::ffi::c_int + idx3weight
                            } else {
                                ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int
                            }) as uint16_t;
                        }
                    }
                }
            }
        }
        i += 1;
    }
}
#[c2rust::src_loc = "601:1"]
unsafe extern "C" fn mbtree_fix8_pack(
    mut dst: *mut uint16_t,
    mut src: *mut ::core::ffi::c_float,
    mut count: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        *dst.offset(i as isize) = endian_fix16(
            (*src.offset(i as isize) * 256.0f32) as int16_t as uint16_t,
        );
        i += 1;
    }
}
#[c2rust::src_loc = "607:1"]
unsafe extern "C" fn mbtree_fix8_unpack(
    mut dst: *mut ::core::ffi::c_float,
    mut src: *mut uint16_t,
    mut count: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        *dst.offset(i as isize) = endian_fix16(*src.offset(i as isize)) as int16_t
            as ::core::ffi::c_int as ::core::ffi::c_float * (1.0f32 / 256.0f32);
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "613:1"]
pub unsafe extern "C" fn x264_10_mc_init(
    mut cpu: uint32_t,
    mut pf: *mut x264_mc_functions_t,
    mut cpu_independent: ::core::ffi::c_int,
) {
    (*pf).mc_luma = Some(
        mc_luma
            as unsafe extern "C" fn(
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
    )
        as Option<
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
        >;
    (*pf).get_ref = Some(
        get_ref
            as unsafe extern "C" fn(
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
    )
        as Option<
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
        >;
    (*pf).mc_chroma = Some(
        mc_chroma
            as unsafe extern "C" fn(
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
    )
        as Option<
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
        >;
    (*pf).avg[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_16x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).avg[PIXEL_16x8 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_16x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).avg[PIXEL_8x16 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_8x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).avg[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_8x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).avg[PIXEL_8x4 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_8x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).avg[PIXEL_4x16 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_4x16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).avg[PIXEL_4x8 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_4x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).avg[PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_4x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).avg[PIXEL_4x2 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_4x2
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).avg[PIXEL_2x8 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_2x8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).avg[PIXEL_2x4 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_2x4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).avg[PIXEL_2x2 as ::core::ffi::c_int as usize] = Some(
        pixel_avg_2x2
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).weight = mc_weight_wtab.as_mut_ptr();
    (*pf).offsetadd = mc_weight_wtab.as_mut_ptr();
    (*pf).offsetsub = mc_weight_wtab.as_mut_ptr();
    (*pf).weight_cache = Some(
        weight_cache as unsafe extern "C" fn(*mut x264_t, *mut x264_weight_t) -> (),
    ) as Option<unsafe extern "C" fn(*mut x264_t, *mut x264_weight_t) -> ()>;
    (*pf).copy_16x16_unaligned = Some(
        mc_copy_w16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).copy[PIXEL_16x16 as ::core::ffi::c_int as usize] = Some(
        mc_copy_w16
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).copy[PIXEL_8x8 as ::core::ffi::c_int as usize] = Some(
        mc_copy_w8
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).copy[PIXEL_4x4 as ::core::ffi::c_int as usize] = Some(
        mc_copy_w4
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).store_interleave_chroma = Some(
        store_interleave_chroma
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                *mut pixel,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                *mut pixel,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).load_deinterleave_chroma_fenc = Some(
        load_deinterleave_chroma_fenc
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).load_deinterleave_chroma_fdec = Some(
        load_deinterleave_chroma_fdec
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).plane_copy = Some(
        x264_10_plane_copy_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
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
            ) -> (),
        >;
    (*pf).plane_copy_swap = Some(
        x264_10_plane_copy_swap_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
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
            ) -> (),
        >;
    (*pf).plane_copy_interleave = Some(
        x264_10_plane_copy_interleave_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
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
        >;
    (*pf).plane_copy_deinterleave = Some(
        x264_10_plane_copy_deinterleave_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
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
        >;
    (*pf).plane_copy_deinterleave_yuyv = Some(
        x264_10_plane_copy_deinterleave_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
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
        >;
    (*pf).plane_copy_deinterleave_rgb = Some(
        plane_copy_deinterleave_rgb_c
            as unsafe extern "C" fn(
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
    )
        as Option<
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
        >;
    (*pf).plane_copy_deinterleave_v210 = Some(
        plane_copy_deinterleave_v210_c
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                *mut uint32_t,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
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
        >;
    (*pf).hpel_filter = Some(
        hpel_filter
            as unsafe extern "C" fn(
                *mut pixel,
                *mut pixel,
                *mut pixel,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
                *mut int16_t,
            ) -> (),
    )
        as Option<
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
        >;
    (*pf).prefetch_fenc_400 = Some(
        prefetch_fenc_null
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).prefetch_fenc_420 = Some(
        prefetch_fenc_null
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).prefetch_fenc_422 = Some(
        prefetch_fenc_null
            as unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut pixel,
                intptr_t,
                *mut pixel,
                intptr_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).prefetch_ref = Some(
        prefetch_ref_null
            as unsafe extern "C" fn(*mut pixel, intptr_t, ::core::ffi::c_int) -> (),
    ) as Option<unsafe extern "C" fn(*mut pixel, intptr_t, ::core::ffi::c_int) -> ()>;
    (*pf).memcpy_aligned = Some(
        memcpy
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                size_t,
            ) -> *mut ::core::ffi::c_void,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                size_t,
            ) -> *mut ::core::ffi::c_void,
        >;
    (*pf).memzero_aligned = Some(
        memzero_aligned as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> (),
    ) as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
    (*pf).frame_init_lowres_core = Some(
        frame_init_lowres_core
            as unsafe extern "C" fn(
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
    )
        as Option<
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
        >;
    (*pf).integral_init4h = Some(
        integral_init4h
            as unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> (),
    ) as Option<unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> ()>;
    (*pf).integral_init8h = Some(
        integral_init8h
            as unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> (),
    ) as Option<unsafe extern "C" fn(*mut uint16_t, *mut pixel, intptr_t) -> ()>;
    (*pf).integral_init4v = Some(
        integral_init4v
            as unsafe extern "C" fn(*mut uint16_t, *mut uint16_t, intptr_t) -> (),
    ) as Option<unsafe extern "C" fn(*mut uint16_t, *mut uint16_t, intptr_t) -> ()>;
    (*pf).integral_init8v = Some(
        integral_init8v as unsafe extern "C" fn(*mut uint16_t, intptr_t) -> (),
    ) as Option<unsafe extern "C" fn(*mut uint16_t, intptr_t) -> ()>;
    (*pf).mbtree_propagate_cost = Some(
        mbtree_propagate_cost
            as unsafe extern "C" fn(
                *mut int16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut ::core::ffi::c_float,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut int16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut uint16_t,
                *mut ::core::ffi::c_float,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).mbtree_propagate_list = Some(
        mbtree_propagate_list
            as unsafe extern "C" fn(
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
    )
        as Option<
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
        >;
    (*pf).mbtree_fix8_pack = Some(
        mbtree_fix8_pack
            as unsafe extern "C" fn(
                *mut uint16_t,
                *mut ::core::ffi::c_float,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut uint16_t,
                *mut ::core::ffi::c_float,
                ::core::ffi::c_int,
            ) -> (),
        >;
    (*pf).mbtree_fix8_unpack = Some(
        mbtree_fix8_unpack
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_float,
                *mut uint16_t,
                ::core::ffi::c_int,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_float,
                *mut uint16_t,
                ::core::ffi::c_int,
            ) -> (),
        >;
    if cpu_independent != 0 {
        (*pf).mbtree_propagate_cost = Some(
            mbtree_propagate_cost
                as unsafe extern "C" fn(
                    *mut int16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut ::core::ffi::c_float,
                    ::core::ffi::c_int,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut int16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut uint16_t,
                    *mut ::core::ffi::c_float,
                    ::core::ffi::c_int,
                ) -> (),
            >;
        (*pf).mbtree_propagate_list = Some(
            mbtree_propagate_list
                as unsafe extern "C" fn(
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
        )
            as Option<
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
            >;
    }
}
#[no_mangle]
#[c2rust::src_loc = "704:1"]
pub unsafe extern "C" fn x264_10_frame_filter(
    mut h: *mut x264_t,
    mut frame: *mut x264_frame_t,
    mut mb_y: ::core::ffi::c_int,
    mut b_end: ::core::ffi::c_int,
) {
    let b_interlaced: ::core::ffi::c_int = (*h).param.b_interlaced;
    let mut start: ::core::ffi::c_int = mb_y * 16 as ::core::ffi::c_int
        - 8 as ::core::ffi::c_int;
    let mut height: ::core::ffi::c_int = (if b_end != 0 {
        (*frame).i_lines[0 as ::core::ffi::c_int as usize]
            + 16 as ::core::ffi::c_int * (*h).param.b_interlaced
    } else {
        (mb_y + b_interlaced) * 16 as ::core::ffi::c_int
    }) + 8 as ::core::ffi::c_int;
    if mb_y & b_interlaced != 0 {
        return;
    }
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
        let mut stride: ::core::ffi::c_int = (*frame).i_stride[p as usize];
        let width: ::core::ffi::c_int = (*frame).i_width[p as usize];
        let mut offs: ::core::ffi::c_int = start * stride - 8 as ::core::ffi::c_int;
        if b_interlaced == 0 || (*h).mb.b_adaptive_mbaff != 0 {
            (*h)
                .mc
                .hpel_filter
                .expect(
                    "non-null function pointer",
                )(
                (*frame)
                    .filtered[p as usize][1 as ::core::ffi::c_int as usize]
                    .offset(offs as isize),
                (*frame)
                    .filtered[p as usize][2 as ::core::ffi::c_int as usize]
                    .offset(offs as isize),
                (*frame)
                    .filtered[p as usize][3 as ::core::ffi::c_int as usize]
                    .offset(offs as isize),
                (*frame).plane[p as usize].offset(offs as isize),
                stride as intptr_t,
                width + 16 as ::core::ffi::c_int,
                height - start,
                (*h).scratch_buffer as *mut int16_t,
            );
        }
        if b_interlaced != 0 {
            stride = (*frame).i_stride[p as usize] << 1 as ::core::ffi::c_int;
            start = (mb_y * 16 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                - 8 as ::core::ffi::c_int;
            let mut height_fld: ::core::ffi::c_int = ((if b_end != 0 {
                (*frame).i_lines[p as usize]
            } else {
                mb_y * 16 as ::core::ffi::c_int
            }) >> 1 as ::core::ffi::c_int) + 8 as ::core::ffi::c_int;
            offs = start * stride - 8 as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 2 as ::core::ffi::c_int {
                (*h)
                    .mc
                    .hpel_filter
                    .expect(
                        "non-null function pointer",
                    )(
                    (*frame)
                        .filtered_fld[p as usize][1 as ::core::ffi::c_int as usize]
                        .offset(offs as isize),
                    (*frame)
                        .filtered_fld[p as usize][2 as ::core::ffi::c_int as usize]
                        .offset(offs as isize),
                    (*frame)
                        .filtered_fld[p as usize][3 as ::core::ffi::c_int as usize]
                        .offset(offs as isize),
                    (*frame).plane_fld[p as usize].offset(offs as isize),
                    stride as intptr_t,
                    width + 16 as ::core::ffi::c_int,
                    height_fld - start,
                    (*h).scratch_buffer as *mut int16_t,
                );
                i += 1;
                offs += (*frame).i_stride[p as usize];
            }
        }
        p += 1;
    }
    if !(*frame).integral.is_null() {
        let mut stride_0: ::core::ffi::c_int = (*frame)
            .i_stride[0 as ::core::ffi::c_int as usize];
        if start < 0 as ::core::ffi::c_int {
            memset(
                (*frame)
                    .integral
                    .offset(-((PADV * stride_0) as isize))
                    .offset(
                        -((if 32 as ::core::ffi::c_int
                            > 64 as ::core::ffi::c_int
                                / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                        {
                            32 as ::core::ffi::c_int
                        } else {
                            64 as ::core::ffi::c_int
                                / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                        }) as isize),
                    ) as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (stride_0 as size_t)
                    .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            );
            start = -PADV;
        }
        if b_end != 0 {
            height += PADV - 9 as ::core::ffi::c_int;
        }
        let mut y: ::core::ffi::c_int = start;
        while y < height {
            let mut pix: *mut pixel = (*frame)
                .plane[0 as ::core::ffi::c_int as usize]
                .offset((y * stride_0) as isize)
                .offset(
                    -((if 32 as ::core::ffi::c_int
                        > 64 as ::core::ffi::c_int
                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                    {
                        32 as ::core::ffi::c_int
                    } else {
                        64 as ::core::ffi::c_int
                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                    }) as isize),
                );
            let mut sum8: *mut uint16_t = (*frame)
                .integral
                .offset(((y + 1 as ::core::ffi::c_int) * stride_0) as isize)
                .offset(
                    -((if 32 as ::core::ffi::c_int
                        > 64 as ::core::ffi::c_int
                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                    {
                        32 as ::core::ffi::c_int
                    } else {
                        64 as ::core::ffi::c_int
                            / ::core::mem::size_of::<pixel>() as ::core::ffi::c_int
                    }) as isize),
                );
            let mut sum4: *mut uint16_t = 0 as *mut uint16_t;
            if (*h).frames.b_have_sub8x8_esa != 0 {
                (*h)
                    .mc
                    .integral_init4h
                    .expect(
                        "non-null function pointer",
                    )(sum8, pix, stride_0 as intptr_t);
                sum8 = sum8.offset(-((8 as ::core::ffi::c_int * stride_0) as isize));
                sum4 = sum8
                    .offset(
                        (stride_0
                            * ((*frame).i_lines[0 as ::core::ffi::c_int as usize]
                                + PADV * 2 as ::core::ffi::c_int)) as isize,
                    );
                if y >= 8 as ::core::ffi::c_int - PADV {
                    (*h)
                        .mc
                        .integral_init4v
                        .expect(
                            "non-null function pointer",
                        )(sum8, sum4, stride_0 as intptr_t);
                }
            } else {
                (*h)
                    .mc
                    .integral_init8h
                    .expect(
                        "non-null function pointer",
                    )(sum8, pix, stride_0 as intptr_t);
                if y >= 8 as ::core::ffi::c_int - PADV {
                    (*h)
                        .mc
                        .integral_init8v
                        .expect(
                            "non-null function pointer",
                        )(
                        sum8.offset(-((8 as ::core::ffi::c_int * stride_0) as isize)),
                        stride_0 as intptr_t,
                    );
                }
            }
            y += 1;
        }
    }
}
