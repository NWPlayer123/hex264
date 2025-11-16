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
    #[c2rust::src_loc = "58:9"]
    pub const QP_BD_OFFSET: ::core::ffi::c_int = 6 as ::core::ffi::c_int
        * (BIT_DEPTH - 8 as ::core::ffi::c_int);
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
        #[c2rust::src_loc = "138:1"]
        pub fn x264_10_log(
            h: *mut x264_t,
            i_level: ::core::ffi::c_int,
            psz_fmt: *const ::core::ffi::c_char,
            ...
        );
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
    use super::pthreadtypes_h::{pthread_mutex_t, pthread_cond_t};
    use super::stdint_uintn_h::{uint8_t, uint16_t, uint32_t, uint64_t};
    use super::stdint_intn_h::{int64_t, int8_t, int16_t};
    use super::x264_h::{x264_param_t, x264_hrd_t, x264_sei_t};
    use super::common_h::pixel;
    use super::mc_h::x264_weight_t;
    use super::stdint_h::intptr_t;
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
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "633:16"]
    pub struct x264_level_t {
        pub level_idc: uint8_t,
        pub mbps: int32_t,
        pub frame_size: int32_t,
        pub dpb: int32_t,
        pub bitrate: int32_t,
        pub cpb: int32_t,
        pub mv_range: uint16_t,
        pub mvs_per_2mb: uint8_t,
        pub slice_rate: uint8_t,
        pub mincr: uint8_t,
        pub bipred8x8: uint8_t,
        pub direct8x8: uint8_t,
        pub frame_only: uint8_t,
    }
    #[c2rust::src_loc = "48:9"]
    pub const X264_BUILD: ::core::ffi::c_int = 165 as ::core::ffi::c_int;
    #[c2rust::src_loc = "211:9"]
    pub const X264_CQM_FLAT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "212:9"]
    pub const X264_CQM_JVT: ::core::ffi::c_int = 1;
    #[c2rust::src_loc = "213:9"]
    pub const X264_CQM_CUSTOM: ::core::ffi::c_int = 2;
    #[c2rust::src_loc = "214:9"]
    pub const X264_RC_CQP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "216:9"]
    pub const X264_RC_ABR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "229:9"]
    pub const X264_B_PYRAMID_STRICT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    #[c2rust::src_loc = "254:9"]
    pub const X264_CSP_MASK: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
    #[c2rust::src_loc = "257:9"]
    pub const X264_CSP_I420: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "261:9"]
    pub const X264_CSP_I422: ::core::ffi::c_int = 0x6 as ::core::ffi::c_int;
    #[c2rust::src_loc = "267:9"]
    pub const X264_CSP_I444: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;
    #[c2rust::src_loc = "269:9"]
    pub const X264_CSP_BGR: ::core::ffi::c_int = 0xe as ::core::ffi::c_int;
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    #[c2rust::src_loc = "290:9"]
    pub const X264_LOG_WARNING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    use super::stdint_uintn_h::{uint8_t, uint32_t, uint16_t};
    use super::internal::__va_list_tag;
    use super::common_h::x264_t;
    use super::stdint_intn_h::{int64_t, int32_t};
    extern "C" {
        #[c2rust::src_loc = "651:36"]
        pub static x264_levels: [x264_level_t; 0];
    }
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
    #[inline]
    #[c2rust::src_loc = "86:1"]
    pub unsafe extern "C" fn bs_init(
        mut s: *mut bs_t,
        mut p_data: *mut ::core::ffi::c_void,
        mut i_data: ::core::ffi::c_int,
    ) {
        let mut offset: ::core::ffi::c_int = (p_data as intptr_t & 3 as intptr_t)
            as ::core::ffi::c_int;
        (*s).p_start = (p_data as *mut uint8_t).offset(-(offset as isize));
        (*s).p = (*s).p_start;
        (*s).p_end = (p_data as *mut uint8_t).offset(i_data as isize);
        (*s).i_left = WORD_SIZE
            .wrapping_sub(offset as uint64_t)
            .wrapping_mul(8 as uint64_t) as ::core::ffi::c_int;
        if offset != 0 {
            (*s).cur_bits = endian_fix32((*((*s).p as *mut x264_union32_t)).i)
                as uintptr_t;
            (*s).cur_bits
                >>= (4 as ::core::ffi::c_int - offset) * 8 as ::core::ffi::c_int;
        } else {
            (*s).cur_bits = 0 as uintptr_t;
        };
    }
    #[inline]
    #[c2rust::src_loc = "100:1"]
    pub unsafe extern "C" fn bs_pos(mut s: *mut bs_t) -> ::core::ffi::c_int {
        return ((8 as ::core::ffi::c_long
            * (*s).p.offset_from((*s).p_start) as ::core::ffi::c_long) as uint64_t)
            .wrapping_add(WORD_SIZE.wrapping_mul(8 as uint64_t))
            .wrapping_sub((*s).i_left as uint64_t) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "106:1"]
    pub unsafe extern "C" fn bs_flush(mut s: *mut bs_t) {
        (*((*s).p as *mut x264_union32_t)).i = endian_fix32(
            ((*s).cur_bits << ((*s).i_left & 31 as ::core::ffi::c_int)) as uint32_t,
        );
        (*s).p = (*s)
            .p
            .offset(
                WORD_SIZE
                    .wrapping_sub(((*s).i_left >> 3 as ::core::ffi::c_int) as uint64_t)
                    as isize,
            );
        (*s).i_left = WORD_SIZE.wrapping_mul(8 as uint64_t) as ::core::ffi::c_int;
    }
    #[inline]
    #[c2rust::src_loc = "113:1"]
    pub unsafe extern "C" fn bs_realign(mut s: *mut bs_t) {
        let mut offset: ::core::ffi::c_int = ((*s).p as intptr_t & 3 as intptr_t)
            as ::core::ffi::c_int;
        if offset != 0 {
            (*s).p = (*s).p.offset(-(offset as isize));
            (*s).i_left = WORD_SIZE
                .wrapping_sub(offset as uint64_t)
                .wrapping_mul(8 as uint64_t) as ::core::ffi::c_int;
            (*s).cur_bits = endian_fix32((*((*s).p as *mut x264_union32_t)).i)
                as uintptr_t;
            (*s).cur_bits
                >>= (4 as ::core::ffi::c_int - offset) * 8 as ::core::ffi::c_int;
        }
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
                (*((*s).p as *mut x264_union32_t)).i = endian_fix(
                    (*s).cur_bits << (*s).i_left,
                ) as uint32_t;
                (*s).i_left += 32 as ::core::ffi::c_int;
                (*s).p = (*s).p.offset(4 as ::core::ffi::c_int as isize);
            }
        } else if i_count < (*s).i_left {
            (*s).cur_bits = (*s).cur_bits << i_count | i_bits as uintptr_t;
            (*s).i_left -= i_count;
        } else {
            i_count -= (*s).i_left;
            (*s).cur_bits = (*s).cur_bits << (*s).i_left
                | (i_bits >> i_count) as uintptr_t;
            (*((*s).p as *mut x264_union32_t)).i = endian_fix((*s).cur_bits) as uint32_t;
            (*s).p = (*s).p.offset(4 as ::core::ffi::c_int as isize);
            (*s).cur_bits = i_bits as uintptr_t;
            (*s).i_left = 32 as ::core::ffi::c_int - i_count;
        };
    }
    #[inline]
    #[c2rust::src_loc = "163:1"]
    pub unsafe extern "C" fn bs_write32(mut s: *mut bs_t, mut i_bits: uint32_t) {
        bs_write(s, 16 as ::core::ffi::c_int, i_bits >> 16 as ::core::ffi::c_int);
        bs_write(s, 16 as ::core::ffi::c_int, i_bits);
    }
    #[inline]
    #[c2rust::src_loc = "169:1"]
    pub unsafe extern "C" fn bs_write1(mut s: *mut bs_t, mut i_bit: uint32_t) {
        (*s).cur_bits <<= 1 as ::core::ffi::c_int;
        (*s).cur_bits |= i_bit as uintptr_t;
        (*s).i_left -= 1;
        if (*s).i_left as uint64_t
            == WORD_SIZE.wrapping_mul(8 as uint64_t).wrapping_sub(32 as uint64_t)
        {
            (*((*s).p as *mut x264_union32_t)).i = endian_fix32(
                (*s).cur_bits as uint32_t,
            );
            (*s).p = (*s).p.offset(4 as ::core::ffi::c_int as isize);
            (*s).i_left = WORD_SIZE.wrapping_mul(8 as uint64_t) as ::core::ffi::c_int;
        }
    }
    #[inline]
    #[c2rust::src_loc = "192:1"]
    pub unsafe extern "C" fn bs_align_10(mut s: *mut bs_t) {
        if (*s).i_left & 7 as ::core::ffi::c_int != 0 {
            bs_write(
                s,
                (*s).i_left & 7 as ::core::ffi::c_int,
                ((1 as ::core::ffi::c_int)
                    << ((*s).i_left & 7 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                    as uint32_t,
            );
        }
        bs_flush(s);
    }
    #[c2rust::src_loc = "201:22"]
    pub static mut x264_ue_size_tab: [uint8_t; 256] = [
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
    ];
    #[inline]
    #[c2rust::src_loc = "221:1"]
    pub unsafe extern "C" fn bs_write_ue_big(
        mut s: *mut bs_t,
        mut val: ::core::ffi::c_uint,
    ) {
        let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        val = val.wrapping_add(1);
        let mut tmp: ::core::ffi::c_int = val as ::core::ffi::c_int;
        if tmp >= 0x10000 as ::core::ffi::c_int {
            size = 32 as ::core::ffi::c_int;
            tmp >>= 16 as ::core::ffi::c_int;
        }
        if tmp >= 0x100 as ::core::ffi::c_int {
            size += 16 as ::core::ffi::c_int;
            tmp >>= 8 as ::core::ffi::c_int;
        }
        size += x264_ue_size_tab[tmp as usize] as ::core::ffi::c_int;
        bs_write(s, size >> 1 as ::core::ffi::c_int, 0 as uint32_t);
        bs_write(
            s,
            (size >> 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int,
            val as uint32_t,
        );
    }
    #[inline]
    #[c2rust::src_loc = "246:1"]
    pub unsafe extern "C" fn bs_write_se(mut s: *mut bs_t, mut val: ::core::ffi::c_int) {
        let mut size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut tmp: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            - val * 2 as ::core::ffi::c_int;
        if tmp < 0 as ::core::ffi::c_int {
            tmp = val * 2 as ::core::ffi::c_int;
        }
        val = tmp;
        if tmp >= 0x100 as ::core::ffi::c_int {
            size = 16 as ::core::ffi::c_int;
            tmp >>= 8 as ::core::ffi::c_int;
        }
        size += x264_ue_size_tab[tmp as usize] as ::core::ffi::c_int;
        bs_write(s, size, val as uint32_t);
    }
    #[inline]
    #[c2rust::src_loc = "272:1"]
    pub unsafe extern "C" fn bs_rbsp_trailing(mut s: *mut bs_t) {
        bs_write1(s, 1 as uint32_t);
        bs_write(s, (*s).i_left & 7 as ::core::ffi::c_int, 0 as uint32_t);
    }
    #[inline(always)]
    #[c2rust::src_loc = "291:1"]
    pub unsafe extern "C" fn bs_size_se(
        mut val: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut tmp: ::core::ffi::c_int = 1 as ::core::ffi::c_int
            - val * 2 as ::core::ffi::c_int;
        if tmp < 0 as ::core::ffi::c_int {
            tmp = val * 2 as ::core::ffi::c_int;
        }
        if tmp < 256 as ::core::ffi::c_int {
            return x264_ue_size_tab[tmp as usize] as ::core::ffi::c_int
        } else {
            return x264_ue_size_tab[(tmp >> 8 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int + 16 as ::core::ffi::c_int
        };
    }
    use super::stdint_uintn_h::{uint8_t, uint64_t, uint32_t};
    use super::common_h::dctcoef;
    use super::stdint_h::{intptr_t, uintptr_t};
    use super::cabac_h::x264_cabac_t;
    use super::stdint_intn_h::int32_t;
    use super::osdep_h::{WORD_SIZE, endian_fix32, endian_fix};
    use super::base_h::x264_union32_t;
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
    #[c2rust::src_loc = "30:1"]
    pub type cqm4_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "35:5"]
    pub const CQM_4PC: cqm4_e = 3;
    #[c2rust::src_loc = "34:5"]
    pub const CQM_4IC: cqm4_e = 2;
    #[c2rust::src_loc = "33:5"]
    pub const CQM_4PY: cqm4_e = 1;
    #[c2rust::src_loc = "32:5"]
    pub const CQM_4IY: cqm4_e = 0;
    #[c2rust::src_loc = "37:1"]
    pub type cqm8_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "42:5"]
    pub const CQM_8PC: cqm8_e = 3;
    #[c2rust::src_loc = "41:5"]
    pub const CQM_8IC: cqm8_e = 2;
    #[c2rust::src_loc = "40:5"]
    pub const CQM_8PY: cqm8_e = 1;
    #[c2rust::src_loc = "39:5"]
    pub const CQM_8IY: cqm8_e = 0;
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
    #[c2rust::src_loc = "93:1"]
    pub type profile_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "100:5"]
    pub const PROFILE_HIGH444_PREDICTIVE: profile_e = 244;
    #[c2rust::src_loc = "99:5"]
    pub const PROFILE_HIGH422: profile_e = 122;
    #[c2rust::src_loc = "98:5"]
    pub const PROFILE_HIGH10: profile_e = 110;
    #[c2rust::src_loc = "97:5"]
    pub const PROFILE_HIGH: profile_e = 100;
    #[c2rust::src_loc = "96:5"]
    pub const PROFILE_MAIN: profile_e = 77;
    #[c2rust::src_loc = "95:5"]
    pub const PROFILE_BASELINE: profile_e = 66;
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
    #[c2rust::src_loc = "120:1"]
    pub type sei_payload_type_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "133:5"]
    pub const SEI_ALTERNATIVE_TRANSFER: sei_payload_type_e = 147;
    #[c2rust::src_loc = "132:5"]
    pub const SEI_CONTENT_LIGHT_LEVEL: sei_payload_type_e = 144;
    #[c2rust::src_loc = "131:5"]
    pub const SEI_MASTERING_DISPLAY: sei_payload_type_e = 137;
    #[c2rust::src_loc = "130:5"]
    pub const SEI_FRAME_PACKING: sei_payload_type_e = 45;
    #[c2rust::src_loc = "129:5"]
    pub const SEI_DEC_REF_PIC_MARKING: sei_payload_type_e = 7;
    #[c2rust::src_loc = "128:5"]
    pub const SEI_RECOVERY_POINT: sei_payload_type_e = 6;
    #[c2rust::src_loc = "127:5"]
    pub const SEI_USER_DATA_UNREGISTERED: sei_payload_type_e = 5;
    #[c2rust::src_loc = "126:5"]
    pub const SEI_USER_DATA_REGISTERED: sei_payload_type_e = 4;
    #[c2rust::src_loc = "125:5"]
    pub const SEI_FILLER: sei_payload_type_e = 3;
    #[c2rust::src_loc = "124:5"]
    pub const SEI_PAN_SCAN_RECT: sei_payload_type_e = 2;
    #[c2rust::src_loc = "123:5"]
    pub const SEI_PIC_TIMING: sei_payload_type_e = 1;
    #[c2rust::src_loc = "122:5"]
    pub const SEI_BUFFERING_PERIOD: sei_payload_type_e = 0;
    use super::stdint_uintn_h::{uint32_t, uint16_t, uint8_t};
    use super::stdint_intn_h::int64_t;
    use super::x264_h::x264_param_t;
    extern "C" {
        #[c2rust::src_loc = "279:10"]
        pub fn x264_malloc(_: int64_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "280:10"]
        pub fn x264_free(_: *mut ::core::ffi::c_void);
        #[c2rust::src_loc = "291:10"]
        pub fn x264_param2string(
            p: *mut x264_param_t,
            b_res: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:27"]
pub mod osdep_h {
    #[c2rust::src_loc = "452:9"]
    pub const WORD_SIZE: uint64_t = ::core::mem::size_of::<*mut ::core::ffi::c_void>()
        as uint64_t;
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
            .wrapping_add(
                (endian_fix32(x as uint32_t) as uint64_t) << 32 as ::core::ffi::c_int,
            );
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
    use super::stdint_uintn_h::{uint32_t, uint64_t};
    use super::stdint_h::uintptr_t;
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
        #[c2rust::src_loc = "64:1"]
        pub fn memcmp(
            __s1: *const ::core::ffi::c_void,
            __s2: *const ::core::ffi::c_void,
            __n: size_t,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/tables.h:27"]
pub mod tables_h {
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "56:22"]
        pub static x264_cqm_flat16: [uint8_t; 64];
        #[c2rust::src_loc = "57:30"]
        pub static x264_cqm_jvt: [*const uint8_t; 8];
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/macroblock.h:27"]
pub mod macroblock_h {
    #[c2rust::src_loc = "181:22"]
    pub static mut x264_zigzag_scan4: [[uint8_t; 16]; 2] = [
        [
            0 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            15 as ::core::ffi::c_int as uint8_t,
        ],
        [
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            15 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    #[c2rust::src_loc = "188:22"]
    pub static mut x264_zigzag_scan8: [[uint8_t; 64]; 2] = [
        [
            0 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            17 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            25 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            33 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            19 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            27 as ::core::ffi::c_int as uint8_t,
            34 as ::core::ffi::c_int as uint8_t,
            41 as ::core::ffi::c_int as uint8_t,
            48 as ::core::ffi::c_int as uint8_t,
            56 as ::core::ffi::c_int as uint8_t,
            49 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            35 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            21 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            15 as ::core::ffi::c_int as uint8_t,
            22 as ::core::ffi::c_int as uint8_t,
            29 as ::core::ffi::c_int as uint8_t,
            36 as ::core::ffi::c_int as uint8_t,
            43 as ::core::ffi::c_int as uint8_t,
            50 as ::core::ffi::c_int as uint8_t,
            57 as ::core::ffi::c_int as uint8_t,
            58 as ::core::ffi::c_int as uint8_t,
            51 as ::core::ffi::c_int as uint8_t,
            44 as ::core::ffi::c_int as uint8_t,
            37 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            23 as ::core::ffi::c_int as uint8_t,
            31 as ::core::ffi::c_int as uint8_t,
            38 as ::core::ffi::c_int as uint8_t,
            45 as ::core::ffi::c_int as uint8_t,
            52 as ::core::ffi::c_int as uint8_t,
            59 as ::core::ffi::c_int as uint8_t,
            60 as ::core::ffi::c_int as uint8_t,
            53 as ::core::ffi::c_int as uint8_t,
            46 as ::core::ffi::c_int as uint8_t,
            39 as ::core::ffi::c_int as uint8_t,
            47 as ::core::ffi::c_int as uint8_t,
            54 as ::core::ffi::c_int as uint8_t,
            61 as ::core::ffi::c_int as uint8_t,
            62 as ::core::ffi::c_int as uint8_t,
            55 as ::core::ffi::c_int as uint8_t,
            63 as ::core::ffi::c_int as uint8_t,
        ],
        [
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            16 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            17 as ::core::ffi::c_int as uint8_t,
            24 as ::core::ffi::c_int as uint8_t,
            18 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            15 as ::core::ffi::c_int as uint8_t,
            19 as ::core::ffi::c_int as uint8_t,
            25 as ::core::ffi::c_int as uint8_t,
            32 as ::core::ffi::c_int as uint8_t,
            26 as ::core::ffi::c_int as uint8_t,
            20 as ::core::ffi::c_int as uint8_t,
            21 as ::core::ffi::c_int as uint8_t,
            22 as ::core::ffi::c_int as uint8_t,
            23 as ::core::ffi::c_int as uint8_t,
            27 as ::core::ffi::c_int as uint8_t,
            33 as ::core::ffi::c_int as uint8_t,
            40 as ::core::ffi::c_int as uint8_t,
            34 as ::core::ffi::c_int as uint8_t,
            28 as ::core::ffi::c_int as uint8_t,
            29 as ::core::ffi::c_int as uint8_t,
            30 as ::core::ffi::c_int as uint8_t,
            31 as ::core::ffi::c_int as uint8_t,
            35 as ::core::ffi::c_int as uint8_t,
            41 as ::core::ffi::c_int as uint8_t,
            48 as ::core::ffi::c_int as uint8_t,
            42 as ::core::ffi::c_int as uint8_t,
            36 as ::core::ffi::c_int as uint8_t,
            37 as ::core::ffi::c_int as uint8_t,
            38 as ::core::ffi::c_int as uint8_t,
            39 as ::core::ffi::c_int as uint8_t,
            43 as ::core::ffi::c_int as uint8_t,
            49 as ::core::ffi::c_int as uint8_t,
            50 as ::core::ffi::c_int as uint8_t,
            44 as ::core::ffi::c_int as uint8_t,
            45 as ::core::ffi::c_int as uint8_t,
            46 as ::core::ffi::c_int as uint8_t,
            47 as ::core::ffi::c_int as uint8_t,
            51 as ::core::ffi::c_int as uint8_t,
            56 as ::core::ffi::c_int as uint8_t,
            57 as ::core::ffi::c_int as uint8_t,
            52 as ::core::ffi::c_int as uint8_t,
            53 as ::core::ffi::c_int as uint8_t,
            54 as ::core::ffi::c_int as uint8_t,
            55 as ::core::ffi::c_int as uint8_t,
            58 as ::core::ffi::c_int as uint8_t,
            59 as ::core::ffi::c_int as uint8_t,
            60 as ::core::ffi::c_int as uint8_t,
            61 as ::core::ffi::c_int as uint8_t,
            62 as ::core::ffi::c_int as uint8_t,
            63 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    use super::stdint_uintn_h::uint8_t;
}
#[c2rust::header_src = "/usr/include/stdio.h:27"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "368:1"]
        pub fn sprintf(
            __s: *mut ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:27"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "170:1"]
        pub fn log2f(__x: ::core::ffi::c_float) -> ::core::ffi::c_float;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/config.h:27"]
pub mod config_h {
    #[c2rust::src_loc = "25:9"]
    pub const HAVE_GPL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264_config.h:27"]
pub mod x264_config_h {
    #[c2rust::src_loc = "7:9"]
    pub const X264_VERSION: [::core::ffi::c_char; 16] = unsafe {
        ::core::mem::transmute::<
            [u8; 16],
            [::core::ffi::c_char; 16],
        >(*b" r3223M 0480cb0\0")
    };
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
    C2RustUnnamed_12, C2RustUnnamed_13, C2RustUnnamed_17, C2RustUnnamed_18, QP_BD_OFFSET,
    x264_ratecontrol_t, x264_10_log,
};
pub use self::frame_h::{
    x264_sync_frame_list_t, x264_frame_t, x264_frame, x264_deblock_function_t,
    x264_deblock_intra_t, x264_deblock_inter_t,
};
pub use self::x264_h::{
    x264_sei_t, x264_sei_payload_t, x264_hrd_t, x264_param_t, x264_nal_t,
    C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, x264_zone_t,
    C2RustUnnamed_4, C2RustUnnamed_5, x264_level_t, X264_BUILD, X264_CQM_FLAT,
    X264_CQM_JVT, X264_CQM_CUSTOM, X264_RC_CQP, X264_RC_ABR, X264_B_PYRAMID_STRICT,
    X264_CSP_MASK, X264_CSP_I420, X264_CSP_I422, X264_CSP_I444, X264_CSP_BGR,
    X264_LOG_ERROR, X264_LOG_WARNING, x264_levels,
};
pub use self::mc_h::{x264_weight_t, weight_fn_t, x264_mc_functions_t};
pub use self::bitstream_h::{
    x264_bitstream_function_t, x264_run_level_t, bs_t, bs_s, bs_init, bs_pos, bs_flush,
    bs_realign, bs_write, bs_write32, bs_write1, bs_align_10, x264_ue_size_tab,
    bs_write_ue_big, bs_write_se, bs_rbsp_trailing, bs_size_se,
};
pub use self::cabac_h::x264_cabac_t;
pub use self::quant_h::x264_quant_function_t;
pub use self::dct_h::{x264_zigzag_function_t, x264_dct_function_t};
pub use self::pixel_h::{
    x264_pixel_function_t, x264_pixel_cmp_x4_t, x264_pixel_cmp_x3_t, x264_pixel_cmp_t,
};
pub use self::predict_h::{x264_predict_8x8_filter_t, x264_predict_t, x264_predict8x8_t};
pub use self::set_h::{
    x264_pps_t, x264_sps_t, C2RustUnnamed_14, C2RustUnnamed_15, C2RustUnnamed_16, cqm4_e,
    CQM_4PC, CQM_4IC, CQM_4PY, CQM_4IY, cqm8_e, CQM_8PC, CQM_8IC, CQM_8PY, CQM_8IY,
};
use self::threadpool_h::x264_threadpool_t;
pub use self::base_h::{
    x264_union32_t, profile_e, PROFILE_HIGH444_PREDICTIVE, PROFILE_HIGH422,
    PROFILE_HIGH10, PROFILE_HIGH, PROFILE_MAIN, PROFILE_BASELINE, chroma_format_e,
    CHROMA_444, CHROMA_422, CHROMA_420, CHROMA_400, sei_payload_type_e,
    SEI_ALTERNATIVE_TRANSFER, SEI_CONTENT_LIGHT_LEVEL, SEI_MASTERING_DISPLAY,
    SEI_FRAME_PACKING, SEI_DEC_REF_PIC_MARKING, SEI_RECOVERY_POINT,
    SEI_USER_DATA_UNREGISTERED, SEI_USER_DATA_REGISTERED, SEI_FILLER, SEI_PAN_SCAN_RECT,
    SEI_PIC_TIMING, SEI_BUFFERING_PERIOD, x264_malloc, x264_free, x264_param2string,
};
pub use self::osdep_h::{WORD_SIZE, endian_fix32, endian_fix64, endian_fix};
use self::string_h::{memcpy, memset, memcmp, strlen};
use self::tables_h::{x264_cqm_flat16, x264_cqm_jvt};
pub use self::macroblock_h::{x264_zigzag_scan4, x264_zigzag_scan8};
use self::stdio_h::sprintf;
use self::mathcalls_h::log2f;
pub use self::config_h::HAVE_GPL;
pub use self::x264_config_h::X264_VERSION;
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "381:26"]
pub struct C2RustUnnamed_19 {
    pub w: uint8_t,
    pub h: uint8_t,
    pub sar: uint8_t,
}
#[c2rust::src_loc = "33:22"]
static mut num_clock_ts: [uint8_t; 10] = [
    0 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
];
#[c2rust::src_loc = "34:22"]
static mut avcintra_uuid: [uint8_t; 16] = [
    0xf7 as ::core::ffi::c_int as uint8_t,
    0x49 as ::core::ffi::c_int as uint8_t,
    0x3e as ::core::ffi::c_int as uint8_t,
    0xb3 as ::core::ffi::c_int as uint8_t,
    0xd4 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0x47 as ::core::ffi::c_int as uint8_t,
    0x96 as ::core::ffi::c_int as uint8_t,
    0x86 as ::core::ffi::c_int as uint8_t,
    0x86 as ::core::ffi::c_int as uint8_t,
    0xc9 as ::core::ffi::c_int as uint8_t,
    0x70 as ::core::ffi::c_int as uint8_t,
    0x7b as ::core::ffi::c_int as uint8_t,
    0x64 as ::core::ffi::c_int as uint8_t,
    0x37 as ::core::ffi::c_int as uint8_t,
    0x2a as ::core::ffi::c_int as uint8_t,
];
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn transpose(mut buf: *mut uint8_t, mut w: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < w {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < i {
            let mut t: uint8_t = *buf.offset((w * i + j) as isize);
            *buf.offset((w * i + j) as isize) = *buf.offset((w * j + i) as isize);
            *buf.offset((w * j + i) as isize) = t;
            j += 1;
        }
        i += 1;
    }
}
#[c2rust::src_loc = "43:1"]
unsafe extern "C" fn scaling_list_write(
    mut s: *mut bs_t,
    mut sps: *mut x264_sps_t,
    mut idx: ::core::ffi::c_int,
) {
    let len: ::core::ffi::c_int = if idx < 4 as ::core::ffi::c_int {
        16 as ::core::ffi::c_int
    } else {
        64 as ::core::ffi::c_int
    };
    let mut zigzag: *const uint8_t = if idx < 4 as ::core::ffi::c_int {
        (*x264_zigzag_scan4.as_ptr().offset(0 as ::core::ffi::c_int as isize)).as_ptr()
    } else {
        (*x264_zigzag_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)).as_ptr()
    };
    let mut list: *const uint8_t = (*sps).scaling_list[idx as usize];
    let mut def_list: *const uint8_t = if idx == CQM_4IC as ::core::ffi::c_int {
        (*sps).scaling_list[CQM_4IY as ::core::ffi::c_int as usize]
    } else if idx == CQM_4PC as ::core::ffi::c_int {
        (*sps).scaling_list[CQM_4PY as ::core::ffi::c_int as usize]
    } else if idx == CQM_8IC as ::core::ffi::c_int + 4 as ::core::ffi::c_int {
        (*sps)
            .scaling_list[(CQM_8IY as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            as usize]
    } else if idx == CQM_8PC as ::core::ffi::c_int + 4 as ::core::ffi::c_int {
        (*sps)
            .scaling_list[(CQM_8PY as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            as usize]
    } else {
        x264_cqm_jvt[idx as usize]
    };
    if memcmp(
        list as *const ::core::ffi::c_void,
        def_list as *const ::core::ffi::c_void,
        len as size_t,
    ) == 0
    {
        bs_write1(s, 0 as uint32_t);
    } else if memcmp(
        list as *const ::core::ffi::c_void,
        x264_cqm_jvt[idx as usize] as *const ::core::ffi::c_void,
        len as size_t,
    ) == 0
    {
        bs_write1(s, 1 as uint32_t);
        bs_write_se(s, -(8 as ::core::ffi::c_int));
    } else {
        let mut run: ::core::ffi::c_int = 0;
        bs_write1(s, 1 as uint32_t);
        run = len;
        while run > 1 as ::core::ffi::c_int {
            if *list
                .offset(
                    *zigzag.offset((run - 1 as ::core::ffi::c_int) as isize) as isize,
                ) as ::core::ffi::c_int
                != *list
                    .offset(
                        *zigzag.offset((run - 2 as ::core::ffi::c_int) as isize) as isize,
                    ) as ::core::ffi::c_int
            {
                break;
            }
            run -= 1;
        }
        if run < len
            && len - run
                < bs_size_se(
                    -(*list.offset(*zigzag.offset(run as isize) as isize)
                        as ::core::ffi::c_int) as int8_t as ::core::ffi::c_int,
                )
        {
            run = len;
        }
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < run {
            bs_write_se(
                s,
                (*list.offset(*zigzag.offset(j as isize) as isize) as ::core::ffi::c_int
                    - (if j > 0 as ::core::ffi::c_int {
                        *list
                            .offset(
                                *zigzag.offset((j - 1 as ::core::ffi::c_int) as isize)
                                    as isize,
                            ) as ::core::ffi::c_int
                    } else {
                        8 as ::core::ffi::c_int
                    })) as int8_t as ::core::ffi::c_int,
            );
            j += 1;
        }
        if run < len {
            bs_write_se(
                s,
                -(*list.offset(*zigzag.offset(run as isize) as isize)
                    as ::core::ffi::c_int) as int8_t as ::core::ffi::c_int,
            );
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "80:1"]
pub unsafe extern "C" fn x264_10_sei_write(
    mut s: *mut bs_t,
    mut payload: *mut uint8_t,
    mut payload_size: ::core::ffi::c_int,
    mut payload_type: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    bs_realign(s);
    i = 0 as ::core::ffi::c_int;
    while i <= payload_type - 255 as ::core::ffi::c_int {
        bs_write(s, 8 as ::core::ffi::c_int, 255 as uint32_t);
        i += 255 as ::core::ffi::c_int;
    }
    bs_write(s, 8 as ::core::ffi::c_int, (payload_type - i) as uint32_t);
    i = 0 as ::core::ffi::c_int;
    while i <= payload_size - 255 as ::core::ffi::c_int {
        bs_write(s, 8 as ::core::ffi::c_int, 255 as uint32_t);
        i += 255 as ::core::ffi::c_int;
    }
    bs_write(s, 8 as ::core::ffi::c_int, (payload_size - i) as uint32_t);
    i = 0 as ::core::ffi::c_int;
    while i < payload_size {
        bs_write(s, 8 as ::core::ffi::c_int, *payload.offset(i as isize) as uint32_t);
        i += 1;
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
#[c2rust::src_loc = "101:1"]
pub unsafe extern "C" fn x264_10_sps_init(
    mut sps: *mut x264_sps_t,
    mut i_id: ::core::ffi::c_int,
    mut param: *mut x264_param_t,
) {
    let mut csp: ::core::ffi::c_int = (*param).i_csp & X264_CSP_MASK;
    (*sps).i_id = i_id;
    (*sps).i_mb_width = ((*param).i_width + 15 as ::core::ffi::c_int)
        / 16 as ::core::ffi::c_int;
    (*sps).i_mb_height = ((*param).i_height + 15 as ::core::ffi::c_int)
        / 16 as ::core::ffi::c_int;
    (*sps).b_frame_mbs_only = !((*param).b_interlaced != 0
        || (*param).b_fake_interlaced != 0) as ::core::ffi::c_int;
    if (*sps).b_frame_mbs_only == 0 {
        (*sps).i_mb_height = (*sps).i_mb_height + 1 as ::core::ffi::c_int
            & !(1 as ::core::ffi::c_int);
    }
    (*sps).i_chroma_format_idc = if csp >= X264_CSP_I444 {
        CHROMA_444 as ::core::ffi::c_int
    } else if csp >= X264_CSP_I422 {
        CHROMA_422 as ::core::ffi::c_int
    } else if csp >= X264_CSP_I420 {
        CHROMA_420 as ::core::ffi::c_int
    } else {
        CHROMA_400 as ::core::ffi::c_int
    };
    (*sps).b_qpprime_y_zero_transform_bypass = ((*param).rc.i_rc_method == X264_RC_CQP
        && (*param).rc.i_qp_constant == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if (*sps).b_qpprime_y_zero_transform_bypass != 0
        || (*sps).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int
    {
        (*sps).i_profile_idc = PROFILE_HIGH444_PREDICTIVE as ::core::ffi::c_int;
    } else if (*sps).i_chroma_format_idc == CHROMA_422 as ::core::ffi::c_int {
        (*sps).i_profile_idc = PROFILE_HIGH422 as ::core::ffi::c_int;
    } else if BIT_DEPTH > 8 as ::core::ffi::c_int {
        (*sps).i_profile_idc = PROFILE_HIGH10 as ::core::ffi::c_int;
    } else if (*param).analyse.b_transform_8x8 != 0
        || (*param).i_cqm_preset != X264_CQM_FLAT
        || (*sps).i_chroma_format_idc == CHROMA_400 as ::core::ffi::c_int
    {
        (*sps).i_profile_idc = PROFILE_HIGH as ::core::ffi::c_int;
    } else if (*param).b_cabac != 0 || (*param).i_bframe > 0 as ::core::ffi::c_int
        || (*param).b_interlaced != 0 || (*param).b_fake_interlaced != 0
        || (*param).analyse.i_weighted_pred > 0 as ::core::ffi::c_int
    {
        (*sps).i_profile_idc = PROFILE_MAIN as ::core::ffi::c_int;
    } else {
        (*sps).i_profile_idc = PROFILE_BASELINE as ::core::ffi::c_int;
    }
    (*sps).b_constraint_set0 = ((*sps).i_profile_idc
        == PROFILE_BASELINE as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*sps).b_constraint_set1 = ((*sps).i_profile_idc
        <= PROFILE_MAIN as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*sps).b_constraint_set2 = 0 as ::core::ffi::c_int;
    (*sps).b_constraint_set3 = 0 as ::core::ffi::c_int;
    (*sps).i_level_idc = (*param).i_level_idc;
    if (*param).i_level_idc == 9 as ::core::ffi::c_int
        && ((*sps).i_profile_idc == PROFILE_BASELINE as ::core::ffi::c_int
            || (*sps).i_profile_idc == PROFILE_MAIN as ::core::ffi::c_int)
    {
        (*sps).b_constraint_set3 = 1 as ::core::ffi::c_int;
        (*sps).i_level_idc = 11 as ::core::ffi::c_int;
    }
    if (*param).i_keyint_max == 1 as ::core::ffi::c_int
        && (*sps).i_profile_idc >= PROFILE_HIGH as ::core::ffi::c_int
    {
        (*sps).b_constraint_set3 = 1 as ::core::ffi::c_int;
    }
    (*sps).vui.i_num_reorder_frames = if (*param).i_bframe_pyramid != 0 {
        2 as ::core::ffi::c_int
    } else if (*param).i_bframe != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    (*sps).i_num_ref_frames = if (16 as ::core::ffi::c_int)
        < (if (*param).i_frame_reference
            > (if 1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
                > (if (if (*param).i_bframe_pyramid != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) > (*param).i_dpb_size
                {
                    (if (*param).i_bframe_pyramid != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })
                } else {
                    (*param).i_dpb_size
                })
            {
                1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
            } else {
                (if (if (*param).i_bframe_pyramid != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) > (*param).i_dpb_size
                {
                    (if (*param).i_bframe_pyramid != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })
                } else {
                    (*param).i_dpb_size
                })
            })
        {
            (*param).i_frame_reference
        } else {
            (if 1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
                > (if (if (*param).i_bframe_pyramid != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) > (*param).i_dpb_size
                {
                    (if (*param).i_bframe_pyramid != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })
                } else {
                    (*param).i_dpb_size
                })
            {
                1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
            } else {
                (if (if (*param).i_bframe_pyramid != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) > (*param).i_dpb_size
                {
                    (if (*param).i_bframe_pyramid != 0 {
                        4 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    })
                } else {
                    (*param).i_dpb_size
                })
            })
        })
    {
        16 as ::core::ffi::c_int
    } else if (*param).i_frame_reference
        > (if 1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
            > (if (if (*param).i_bframe_pyramid != 0 {
                4 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) > (*param).i_dpb_size
            {
                (if (*param).i_bframe_pyramid != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                })
            } else {
                (*param).i_dpb_size
            })
        {
            1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
        } else {
            (if (if (*param).i_bframe_pyramid != 0 {
                4 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) > (*param).i_dpb_size
            {
                (if (*param).i_bframe_pyramid != 0 {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                })
            } else {
                (*param).i_dpb_size
            })
        })
    {
        (*param).i_frame_reference
    } else if 1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
        > (if (if (*param).i_bframe_pyramid != 0 {
            4 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) > (*param).i_dpb_size
        {
            (if (*param).i_bframe_pyramid != 0 {
                4 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            })
        } else {
            (*param).i_dpb_size
        })
    {
        1 as ::core::ffi::c_int + (*sps).vui.i_num_reorder_frames
    } else if (if (*param).i_bframe_pyramid != 0 {
        4 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    }) > (*param).i_dpb_size
    {
        if (*param).i_bframe_pyramid != 0 {
            4 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }
    } else {
        (*param).i_dpb_size
    };
    (*sps).vui.i_max_dec_frame_buffering = (*sps).i_num_ref_frames;
    (*sps).i_num_ref_frames
        -= ((*param).i_bframe_pyramid == X264_B_PYRAMID_STRICT) as ::core::ffi::c_int;
    if (*param).i_keyint_max == 1 as ::core::ffi::c_int {
        (*sps).i_num_ref_frames = 0 as ::core::ffi::c_int;
        (*sps).vui.i_max_dec_frame_buffering = 0 as ::core::ffi::c_int;
    }
    let mut max_frame_num: ::core::ffi::c_int = (*sps).vui.i_max_dec_frame_buffering
        * (((*param).i_bframe_pyramid != 0) as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int;
    if (*param).b_intra_refresh != 0 {
        let mut time_to_recovery: ::core::ffi::c_int = (if ((*sps).i_mb_width
            - 1 as ::core::ffi::c_int) < (*param).i_keyint_max
        {
            (*sps).i_mb_width - 1 as ::core::ffi::c_int
        } else {
            (*param).i_keyint_max
        }) + (*param).i_bframe - 1 as ::core::ffi::c_int;
        max_frame_num = if max_frame_num > time_to_recovery + 1 as ::core::ffi::c_int {
            max_frame_num
        } else {
            time_to_recovery + 1 as ::core::ffi::c_int
        };
    }
    (*sps).i_log2_max_frame_num = 4 as ::core::ffi::c_int;
    while (1 as ::core::ffi::c_int) << (*sps).i_log2_max_frame_num <= max_frame_num {
        (*sps).i_log2_max_frame_num += 1;
    }
    (*sps).i_poc_type = if (*param).i_bframe != 0 || (*param).b_interlaced != 0
        || (*param).i_avcintra_class != 0
    {
        0 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    };
    if (*sps).i_poc_type == 0 as ::core::ffi::c_int {
        let mut max_delta_poc: ::core::ffi::c_int = ((*param).i_bframe
            + 2 as ::core::ffi::c_int)
            * (((*param).i_bframe_pyramid != 0) as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int) * 2 as ::core::ffi::c_int;
        (*sps).i_log2_max_poc_lsb = 4 as ::core::ffi::c_int;
        while (1 as ::core::ffi::c_int) << (*sps).i_log2_max_poc_lsb
            <= max_delta_poc * 2 as ::core::ffi::c_int
        {
            (*sps).i_log2_max_poc_lsb += 1;
        }
    }
    (*sps).b_vui = 1 as ::core::ffi::c_int;
    (*sps).b_gaps_in_frame_num_value_allowed = 0 as ::core::ffi::c_int;
    (*sps).b_mb_adaptive_frame_field = (*param).b_interlaced;
    (*sps).b_direct8x8_inference = 1 as ::core::ffi::c_int;
    x264_10_sps_init_reconfigurable(sps, param);
    (*sps).vui.b_overscan_info_present = ((*param).vui.i_overscan
        > 0 as ::core::ffi::c_int && (*param).vui.i_overscan <= 2 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    if (*sps).vui.b_overscan_info_present != 0 {
        (*sps).vui.b_overscan_info = if (*param).vui.i_overscan
            == 2 as ::core::ffi::c_int
        {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    }
    (*sps).vui.b_signal_type_present = 0 as ::core::ffi::c_int;
    (*sps).vui.i_vidformat = if (*param).vui.i_vidformat >= 0 as ::core::ffi::c_int
        && (*param).vui.i_vidformat <= 5 as ::core::ffi::c_int
    {
        (*param).vui.i_vidformat
    } else {
        5 as ::core::ffi::c_int
    };
    (*sps).vui.b_fullrange = if (*param).vui.b_fullrange >= 0 as ::core::ffi::c_int
        && (*param).vui.b_fullrange <= 1 as ::core::ffi::c_int
    {
        (*param).vui.b_fullrange
    } else if csp >= X264_CSP_BGR {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    (*sps).vui.b_color_description_present = 0 as ::core::ffi::c_int;
    (*sps).vui.i_colorprim = if (*param).vui.i_colorprim >= 0 as ::core::ffi::c_int
        && (*param).vui.i_colorprim <= 12 as ::core::ffi::c_int
    {
        (*param).vui.i_colorprim
    } else {
        2 as ::core::ffi::c_int
    };
    (*sps).vui.i_transfer = if (*param).vui.i_transfer >= 0 as ::core::ffi::c_int
        && (*param).vui.i_transfer <= 18 as ::core::ffi::c_int
    {
        (*param).vui.i_transfer
    } else {
        2 as ::core::ffi::c_int
    };
    (*sps).vui.i_colmatrix = if (*param).vui.i_colmatrix >= 0 as ::core::ffi::c_int
        && (*param).vui.i_colmatrix <= 14 as ::core::ffi::c_int
    {
        (*param).vui.i_colmatrix
    } else if csp >= X264_CSP_BGR {
        0 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    };
    if (*sps).vui.i_colorprim != 2 as ::core::ffi::c_int
        || (*sps).vui.i_transfer != 2 as ::core::ffi::c_int
        || (*sps).vui.i_colmatrix != 2 as ::core::ffi::c_int
    {
        (*sps).vui.b_color_description_present = 1 as ::core::ffi::c_int;
    }
    if (*sps).vui.i_vidformat != 5 as ::core::ffi::c_int || (*sps).vui.b_fullrange != 0
        || (*sps).vui.b_color_description_present != 0
    {
        (*sps).vui.b_signal_type_present = 1 as ::core::ffi::c_int;
    }
    (*sps).vui.b_chroma_loc_info_present = ((*param).vui.i_chroma_loc
        > 0 as ::core::ffi::c_int && (*param).vui.i_chroma_loc <= 5 as ::core::ffi::c_int
        && (*sps).i_chroma_format_idc == CHROMA_420 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    if (*sps).vui.b_chroma_loc_info_present != 0 {
        (*sps).vui.i_chroma_loc_top = (*param).vui.i_chroma_loc;
        (*sps).vui.i_chroma_loc_bottom = (*param).vui.i_chroma_loc;
    }
    (*sps).vui.b_timing_info_present = ((*param).i_timebase_num > 0 as uint32_t
        && (*param).i_timebase_den > 0 as uint32_t) as ::core::ffi::c_int;
    if (*sps).vui.b_timing_info_present != 0 {
        (*sps).vui.i_num_units_in_tick = (*param).i_timebase_num;
        (*sps).vui.i_time_scale = (*param).i_timebase_den.wrapping_mul(2 as uint32_t);
        (*sps).vui.b_fixed_frame_rate = ((*param).b_vfr_input == 0)
            as ::core::ffi::c_int;
    }
    (*sps).vui.b_vcl_hrd_parameters_present = 0 as ::core::ffi::c_int;
    (*sps).vui.b_nal_hrd_parameters_present = ((*param).i_nal_hrd != 0)
        as ::core::ffi::c_int;
    (*sps).vui.b_pic_struct_present = (*param).b_pic_struct;
    (*sps).vui.b_bitstream_restriction = !((*sps).b_constraint_set3 != 0
        && (*sps).i_profile_idc >= PROFILE_HIGH as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    if (*sps).vui.b_bitstream_restriction != 0 {
        (*sps).vui.b_motion_vectors_over_pic_boundaries = 1 as ::core::ffi::c_int;
        (*sps).vui.i_max_bytes_per_pic_denom = 0 as ::core::ffi::c_int;
        (*sps).vui.i_max_bits_per_mb_denom = 0 as ::core::ffi::c_int;
        (*sps).vui.i_log2_max_mv_length_vertical = log2f(
            (if 1 as ::core::ffi::c_int
                > (*param).analyse.i_mv_range * 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int
            {
                1 as ::core::ffi::c_int
            } else {
                (*param).analyse.i_mv_range * 4 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int
            }) as ::core::ffi::c_float,
        ) as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
        (*sps).vui.i_log2_max_mv_length_horizontal = (*sps)
            .vui
            .i_log2_max_mv_length_vertical;
    }
    (*sps).b_avcintra_hd = ((*param).i_avcintra_class != 0
        && (*param).i_avcintra_class <= 200 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*sps).b_avcintra_4k = ((*param).i_avcintra_class > 200 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    (*sps).i_cqm_preset = (*param).i_cqm_preset;
}
#[no_mangle]
#[c2rust::src_loc = "249:1"]
pub unsafe extern "C" fn x264_10_sps_init_reconfigurable(
    mut sps: *mut x264_sps_t,
    mut param: *mut x264_param_t,
) {
    (*sps).crop.i_left = (*param).crop_rect.i_left;
    (*sps).crop.i_top = (*param).crop_rect.i_top;
    (*sps).crop.i_right = (*param).crop_rect.i_right
        + (*sps).i_mb_width * 16 as ::core::ffi::c_int - (*param).i_width;
    (*sps).crop.i_bottom = (*param).crop_rect.i_bottom
        + (*sps).i_mb_height * 16 as ::core::ffi::c_int - (*param).i_height;
    (*sps).b_crop = ((*sps).crop.i_left != 0 || (*sps).crop.i_top != 0
        || (*sps).crop.i_right != 0 || (*sps).crop.i_bottom != 0) as ::core::ffi::c_int;
    (*sps).vui.b_aspect_ratio_info_present = 0 as ::core::ffi::c_int;
    if (*param).vui.i_sar_width > 0 as ::core::ffi::c_int
        && (*param).vui.i_sar_height > 0 as ::core::ffi::c_int
    {
        (*sps).vui.b_aspect_ratio_info_present = 1 as ::core::ffi::c_int;
        (*sps).vui.i_sar_width = (*param).vui.i_sar_width;
        (*sps).vui.i_sar_height = (*param).vui.i_sar_height;
    }
}
#[no_mangle]
#[c2rust::src_loc = "267:1"]
pub unsafe extern "C" fn x264_10_sps_init_scaling_list(
    mut sps: *mut x264_sps_t,
    mut param: *mut x264_param_t,
) {
    match (*sps).i_cqm_preset {
        X264_CQM_FLAT => {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 8 as ::core::ffi::c_int {
                (*sps).scaling_list[i as usize] = x264_cqm_flat16.as_ptr();
                i += 1;
            }
        }
        X264_CQM_JVT => {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 8 as ::core::ffi::c_int {
                (*sps).scaling_list[i_0 as usize] = x264_cqm_jvt[i_0 as usize];
                i_0 += 1;
            }
        }
        X264_CQM_CUSTOM => {
            transpose((*param).cqm_4iy.as_mut_ptr(), 4 as ::core::ffi::c_int);
            transpose((*param).cqm_4py.as_mut_ptr(), 4 as ::core::ffi::c_int);
            transpose((*param).cqm_4ic.as_mut_ptr(), 4 as ::core::ffi::c_int);
            transpose((*param).cqm_4pc.as_mut_ptr(), 4 as ::core::ffi::c_int);
            transpose((*param).cqm_8iy.as_mut_ptr(), 8 as ::core::ffi::c_int);
            transpose((*param).cqm_8py.as_mut_ptr(), 8 as ::core::ffi::c_int);
            transpose((*param).cqm_8ic.as_mut_ptr(), 8 as ::core::ffi::c_int);
            transpose((*param).cqm_8pc.as_mut_ptr(), 8 as ::core::ffi::c_int);
            (*sps).scaling_list[CQM_4IY as ::core::ffi::c_int as usize] = (*param)
                .cqm_4iy
                .as_mut_ptr();
            (*sps).scaling_list[CQM_4PY as ::core::ffi::c_int as usize] = (*param)
                .cqm_4py
                .as_mut_ptr();
            (*sps).scaling_list[CQM_4IC as ::core::ffi::c_int as usize] = (*param)
                .cqm_4ic
                .as_mut_ptr();
            (*sps).scaling_list[CQM_4PC as ::core::ffi::c_int as usize] = (*param)
                .cqm_4pc
                .as_mut_ptr();
            (*sps)
                .scaling_list[(CQM_8IY as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                as usize] = (*param).cqm_8iy.as_mut_ptr();
            (*sps)
                .scaling_list[(CQM_8PY as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                as usize] = (*param).cqm_8py.as_mut_ptr();
            (*sps)
                .scaling_list[(CQM_8IC as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                as usize] = (*param).cqm_8ic.as_mut_ptr();
            (*sps)
                .scaling_list[(CQM_8PC as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
                as usize] = (*param).cqm_8pc.as_mut_ptr();
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < 8 as ::core::ffi::c_int {
                let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while j
                    < (if i_1 < 4 as ::core::ffi::c_int {
                        16 as ::core::ffi::c_int
                    } else {
                        64 as ::core::ffi::c_int
                    })
                {
                    if *(*sps).scaling_list[i_1 as usize].offset(j as isize)
                        as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        (*sps).scaling_list[i_1 as usize] = x264_cqm_jvt[i_1 as usize];
                    }
                    j += 1;
                }
                i_1 += 1;
            }
        }
        _ => {}
    };
}
#[no_mangle]
#[c2rust::src_loc = "305:1"]
pub unsafe extern "C" fn x264_10_sps_write(mut s: *mut bs_t, mut sps: *mut x264_sps_t) {
    bs_realign(s);
    bs_write(s, 8 as ::core::ffi::c_int, (*sps).i_profile_idc as uint32_t);
    bs_write1(s, (*sps).b_constraint_set0 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set1 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set2 as uint32_t);
    bs_write1(s, (*sps).b_constraint_set3 as uint32_t);
    bs_write(s, 4 as ::core::ffi::c_int, 0 as uint32_t);
    bs_write(s, 8 as ::core::ffi::c_int, (*sps).i_level_idc as uint32_t);
    bs_write_ue_big(s, (*sps).i_id as ::core::ffi::c_uint);
    if (*sps).i_profile_idc >= PROFILE_HIGH as ::core::ffi::c_int {
        bs_write_ue_big(s, (*sps).i_chroma_format_idc as ::core::ffi::c_uint);
        if (*sps).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
            bs_write1(s, 0 as uint32_t);
        }
        bs_write_ue_big(s, (BIT_DEPTH - 8 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        bs_write_ue_big(s, (BIT_DEPTH - 8 as ::core::ffi::c_int) as ::core::ffi::c_uint);
        bs_write1(s, (*sps).b_qpprime_y_zero_transform_bypass as uint32_t);
        bs_write1(s, (*sps).b_avcintra_hd as uint32_t);
        if (*sps).b_avcintra_hd != 0 {
            scaling_list_write(s, sps, CQM_4IY as ::core::ffi::c_int);
            scaling_list_write(s, sps, CQM_4IC as ::core::ffi::c_int);
            scaling_list_write(s, sps, CQM_4IC as ::core::ffi::c_int);
            bs_write1(s, 0 as uint32_t);
            bs_write1(s, 0 as uint32_t);
            bs_write1(s, 0 as uint32_t);
            scaling_list_write(
                s,
                sps,
                CQM_8IY as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
            );
            bs_write1(s, 0 as uint32_t);
            if (*sps).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
                scaling_list_write(
                    s,
                    sps,
                    CQM_8IC as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
                );
                bs_write1(s, 0 as uint32_t);
                scaling_list_write(
                    s,
                    sps,
                    CQM_8IC as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
                );
                bs_write1(s, 0 as uint32_t);
            }
        }
    }
    bs_write_ue_big(
        s,
        ((*sps).i_log2_max_frame_num - 4 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    bs_write_ue_big(s, (*sps).i_poc_type as ::core::ffi::c_uint);
    if (*sps).i_poc_type == 0 as ::core::ffi::c_int {
        bs_write_ue_big(
            s,
            ((*sps).i_log2_max_poc_lsb - 4 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
    }
    bs_write_ue_big(s, (*sps).i_num_ref_frames as ::core::ffi::c_uint);
    bs_write1(s, (*sps).b_gaps_in_frame_num_value_allowed as uint32_t);
    bs_write_ue_big(
        s,
        ((*sps).i_mb_width - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    bs_write_ue_big(
        s,
        (((*sps).i_mb_height >> ((*sps).b_frame_mbs_only == 0) as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    bs_write1(s, (*sps).b_frame_mbs_only as uint32_t);
    if (*sps).b_frame_mbs_only == 0 {
        bs_write1(s, (*sps).b_mb_adaptive_frame_field as uint32_t);
    }
    bs_write1(s, (*sps).b_direct8x8_inference as uint32_t);
    bs_write1(s, (*sps).b_crop as uint32_t);
    if (*sps).b_crop != 0 {
        let mut h_shift: ::core::ffi::c_int = ((*sps).i_chroma_format_idc
            == CHROMA_420 as ::core::ffi::c_int
            || (*sps).i_chroma_format_idc == CHROMA_422 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut v_shift: ::core::ffi::c_int = ((*sps).i_chroma_format_idc
            == CHROMA_420 as ::core::ffi::c_int) as ::core::ffi::c_int
            + ((*sps).b_frame_mbs_only == 0) as ::core::ffi::c_int;
        bs_write_ue_big(s, ((*sps).crop.i_left >> h_shift) as ::core::ffi::c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_right >> h_shift) as ::core::ffi::c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_top >> v_shift) as ::core::ffi::c_uint);
        bs_write_ue_big(s, ((*sps).crop.i_bottom >> v_shift) as ::core::ffi::c_uint);
    }
    bs_write1(s, (*sps).b_vui as uint32_t);
    if (*sps).b_vui != 0 {
        bs_write1(s, (*sps).vui.b_aspect_ratio_info_present as uint32_t);
        if (*sps).vui.b_aspect_ratio_info_present != 0 {
            let mut i: ::core::ffi::c_int = 0;
            static mut sar: [C2RustUnnamed_19; 17] = [
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 1 as uint8_t,
                        h: 1 as uint8_t,
                        sar: 1 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 12 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 2 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 10 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 3 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 16 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 4 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 40 as uint8_t,
                        h: 33 as uint8_t,
                        sar: 5 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 24 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 6 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 20 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 7 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 32 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 8 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 80 as uint8_t,
                        h: 33 as uint8_t,
                        sar: 9 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 18 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 10 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 15 as uint8_t,
                        h: 11 as uint8_t,
                        sar: 11 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 64 as uint8_t,
                        h: 33 as uint8_t,
                        sar: 12 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 160 as uint8_t,
                        h: 99 as uint8_t,
                        sar: 13 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 4 as uint8_t,
                        h: 3 as uint8_t,
                        sar: 14 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 3 as uint8_t,
                        h: 2 as uint8_t,
                        sar: 15 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 2 as uint8_t,
                        h: 1 as uint8_t,
                        sar: 16 as uint8_t,
                    };
                    init
                },
                {
                    let mut init = C2RustUnnamed_19 {
                        w: 0 as uint8_t,
                        h: 0 as uint8_t,
                        sar: 255 as uint8_t,
                    };
                    init
                },
            ];
            i = 0 as ::core::ffi::c_int;
            while sar[i as usize].sar as ::core::ffi::c_int != 255 as ::core::ffi::c_int
            {
                if sar[i as usize].w as ::core::ffi::c_int == (*sps).vui.i_sar_width
                    && sar[i as usize].h as ::core::ffi::c_int == (*sps).vui.i_sar_height
                {
                    break;
                }
                i += 1;
            }
            bs_write(s, 8 as ::core::ffi::c_int, sar[i as usize].sar as uint32_t);
            if sar[i as usize].sar as ::core::ffi::c_int == 255 as ::core::ffi::c_int {
                bs_write(
                    s,
                    16 as ::core::ffi::c_int,
                    (*sps).vui.i_sar_width as uint32_t,
                );
                bs_write(
                    s,
                    16 as ::core::ffi::c_int,
                    (*sps).vui.i_sar_height as uint32_t,
                );
            }
        }
        bs_write1(s, (*sps).vui.b_overscan_info_present as uint32_t);
        if (*sps).vui.b_overscan_info_present != 0 {
            bs_write1(s, (*sps).vui.b_overscan_info as uint32_t);
        }
        bs_write1(s, (*sps).vui.b_signal_type_present as uint32_t);
        if (*sps).vui.b_signal_type_present != 0 {
            bs_write(s, 3 as ::core::ffi::c_int, (*sps).vui.i_vidformat as uint32_t);
            bs_write1(s, (*sps).vui.b_fullrange as uint32_t);
            bs_write1(s, (*sps).vui.b_color_description_present as uint32_t);
            if (*sps).vui.b_color_description_present != 0 {
                bs_write(s, 8 as ::core::ffi::c_int, (*sps).vui.i_colorprim as uint32_t);
                bs_write(s, 8 as ::core::ffi::c_int, (*sps).vui.i_transfer as uint32_t);
                bs_write(s, 8 as ::core::ffi::c_int, (*sps).vui.i_colmatrix as uint32_t);
            }
        }
        bs_write1(s, (*sps).vui.b_chroma_loc_info_present as uint32_t);
        if (*sps).vui.b_chroma_loc_info_present != 0 {
            bs_write_ue_big(s, (*sps).vui.i_chroma_loc_top as ::core::ffi::c_uint);
            bs_write_ue_big(s, (*sps).vui.i_chroma_loc_bottom as ::core::ffi::c_uint);
        }
        bs_write1(s, (*sps).vui.b_timing_info_present as uint32_t);
        if (*sps).vui.b_timing_info_present != 0 {
            bs_write32(s, (*sps).vui.i_num_units_in_tick);
            bs_write32(s, (*sps).vui.i_time_scale);
            bs_write1(s, (*sps).vui.b_fixed_frame_rate as uint32_t);
        }
        bs_write1(s, (*sps).vui.b_nal_hrd_parameters_present as uint32_t);
        if (*sps).vui.b_nal_hrd_parameters_present != 0 {
            bs_write_ue_big(
                s,
                ((*sps).vui.hrd.i_cpb_cnt - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            );
            bs_write(
                s,
                4 as ::core::ffi::c_int,
                (*sps).vui.hrd.i_bit_rate_scale as uint32_t,
            );
            bs_write(
                s,
                4 as ::core::ffi::c_int,
                (*sps).vui.hrd.i_cpb_size_scale as uint32_t,
            );
            bs_write_ue_big(
                s,
                ((*sps).vui.hrd.i_bit_rate_value - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            );
            bs_write_ue_big(
                s,
                ((*sps).vui.hrd.i_cpb_size_value - 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            );
            bs_write1(s, (*sps).vui.hrd.b_cbr_hrd as uint32_t);
            bs_write(
                s,
                5 as ::core::ffi::c_int,
                ((*sps).vui.hrd.i_initial_cpb_removal_delay_length
                    - 1 as ::core::ffi::c_int) as uint32_t,
            );
            bs_write(
                s,
                5 as ::core::ffi::c_int,
                ((*sps).vui.hrd.i_cpb_removal_delay_length - 1 as ::core::ffi::c_int)
                    as uint32_t,
            );
            bs_write(
                s,
                5 as ::core::ffi::c_int,
                ((*sps).vui.hrd.i_dpb_output_delay_length - 1 as ::core::ffi::c_int)
                    as uint32_t,
            );
            bs_write(
                s,
                5 as ::core::ffi::c_int,
                (*sps).vui.hrd.i_time_offset_length as uint32_t,
            );
        }
        bs_write1(s, (*sps).vui.b_vcl_hrd_parameters_present as uint32_t);
        if (*sps).vui.b_nal_hrd_parameters_present != 0
            || (*sps).vui.b_vcl_hrd_parameters_present != 0
        {
            bs_write1(s, 0 as uint32_t);
        }
        bs_write1(s, (*sps).vui.b_pic_struct_present as uint32_t);
        bs_write1(s, (*sps).vui.b_bitstream_restriction as uint32_t);
        if (*sps).vui.b_bitstream_restriction != 0 {
            bs_write1(s, (*sps).vui.b_motion_vectors_over_pic_boundaries as uint32_t);
            bs_write_ue_big(
                s,
                (*sps).vui.i_max_bytes_per_pic_denom as ::core::ffi::c_uint,
            );
            bs_write_ue_big(
                s,
                (*sps).vui.i_max_bits_per_mb_denom as ::core::ffi::c_uint,
            );
            bs_write_ue_big(
                s,
                (*sps).vui.i_log2_max_mv_length_horizontal as ::core::ffi::c_uint,
            );
            bs_write_ue_big(
                s,
                (*sps).vui.i_log2_max_mv_length_vertical as ::core::ffi::c_uint,
            );
            bs_write_ue_big(s, (*sps).vui.i_num_reorder_frames as ::core::ffi::c_uint);
            bs_write_ue_big(
                s,
                (*sps).vui.i_max_dec_frame_buffering as ::core::ffi::c_uint,
            );
        }
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
#[c2rust::src_loc = "479:1"]
pub unsafe extern "C" fn x264_10_pps_init(
    mut pps: *mut x264_pps_t,
    mut i_id: ::core::ffi::c_int,
    mut param: *mut x264_param_t,
    mut sps: *mut x264_sps_t,
) {
    (*pps).i_id = i_id;
    (*pps).i_sps_id = (*sps).i_id;
    (*pps).b_cabac = (*param).b_cabac;
    (*pps).b_pic_order = ((*param).i_avcintra_class == 0 && (*param).b_interlaced != 0)
        as ::core::ffi::c_int;
    (*pps).i_num_slice_groups = 1 as ::core::ffi::c_int;
    (*pps).i_num_ref_idx_l0_default_active = (*param).i_frame_reference;
    (*pps).i_num_ref_idx_l1_default_active = 1 as ::core::ffi::c_int;
    (*pps).b_weighted_pred = ((*param).analyse.i_weighted_pred > 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    (*pps).b_weighted_bipred = if (*param).analyse.b_weighted_bipred != 0 {
        2 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    (*pps).i_pic_init_qp = if (*param).rc.i_rc_method == X264_RC_ABR
        || (*param).b_stitchable != 0
    {
        26 as ::core::ffi::c_int + QP_BD_OFFSET
    } else if (*param).rc.i_qp_constant
        < 51 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
    {
        (*param).rc.i_qp_constant
    } else {
        51 as ::core::ffi::c_int
            + 6 as ::core::ffi::c_int
                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
    };
    (*pps).i_pic_init_qs = 26 as ::core::ffi::c_int + QP_BD_OFFSET;
    (*pps).i_chroma_qp_index_offset = (*param).analyse.i_chroma_qp_offset;
    (*pps).b_deblocking_filter_control = 1 as ::core::ffi::c_int;
    (*pps).b_constrained_intra_pred = (*param).b_constrained_intra;
    (*pps).b_redundant_pic_cnt = 0 as ::core::ffi::c_int;
    (*pps).b_transform_8x8_mode = if (*param).analyse.b_transform_8x8 != 0 {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
#[c2rust::src_loc = "505:1"]
pub unsafe extern "C" fn x264_10_pps_write(
    mut s: *mut bs_t,
    mut sps: *mut x264_sps_t,
    mut pps: *mut x264_pps_t,
) {
    bs_realign(s);
    bs_write_ue_big(s, (*pps).i_id as ::core::ffi::c_uint);
    bs_write_ue_big(s, (*pps).i_sps_id as ::core::ffi::c_uint);
    bs_write1(s, (*pps).b_cabac as uint32_t);
    bs_write1(s, (*pps).b_pic_order as uint32_t);
    bs_write_ue_big(
        s,
        ((*pps).i_num_slice_groups - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    bs_write_ue_big(
        s,
        ((*pps).i_num_ref_idx_l0_default_active - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
    bs_write_ue_big(
        s,
        ((*pps).i_num_ref_idx_l1_default_active - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_uint,
    );
    bs_write1(s, (*pps).b_weighted_pred as uint32_t);
    bs_write(s, 2 as ::core::ffi::c_int, (*pps).b_weighted_bipred as uint32_t);
    bs_write_se(s, (*pps).i_pic_init_qp - 26 as ::core::ffi::c_int - QP_BD_OFFSET);
    bs_write_se(s, (*pps).i_pic_init_qs - 26 as ::core::ffi::c_int - QP_BD_OFFSET);
    bs_write_se(s, (*pps).i_chroma_qp_index_offset);
    bs_write1(s, (*pps).b_deblocking_filter_control as uint32_t);
    bs_write1(s, (*pps).b_constrained_intra_pred as uint32_t);
    bs_write1(s, (*pps).b_redundant_pic_cnt as uint32_t);
    let mut b_scaling_list: ::core::ffi::c_int = ((*sps).b_avcintra_hd == 0
        && (*sps).i_cqm_preset != X264_CQM_FLAT) as ::core::ffi::c_int;
    if (*pps).b_transform_8x8_mode != 0 || b_scaling_list != 0 {
        bs_write1(s, (*pps).b_transform_8x8_mode as uint32_t);
        bs_write1(s, b_scaling_list as uint32_t);
        if b_scaling_list != 0 {
            scaling_list_write(s, sps, CQM_4IY as ::core::ffi::c_int);
            scaling_list_write(s, sps, CQM_4IC as ::core::ffi::c_int);
            if (*sps).b_avcintra_4k != 0 {
                scaling_list_write(s, sps, CQM_4IC as ::core::ffi::c_int);
                bs_write1(s, 0 as uint32_t);
                bs_write1(s, 0 as uint32_t);
                bs_write1(s, 0 as uint32_t);
            } else {
                bs_write1(s, 0 as uint32_t);
                scaling_list_write(s, sps, CQM_4PY as ::core::ffi::c_int);
                scaling_list_write(s, sps, CQM_4PC as ::core::ffi::c_int);
                bs_write1(s, 0 as uint32_t);
            }
            if (*pps).b_transform_8x8_mode != 0 {
                scaling_list_write(
                    s,
                    sps,
                    CQM_8IY as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
                );
                if (*sps).b_avcintra_4k != 0 {
                    bs_write1(s, 0 as uint32_t);
                } else {
                    scaling_list_write(
                        s,
                        sps,
                        CQM_8PY as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
                    );
                }
                if (*sps).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
                    scaling_list_write(
                        s,
                        sps,
                        CQM_8IC as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
                    );
                    scaling_list_write(
                        s,
                        sps,
                        CQM_8PC as ::core::ffi::c_int + 4 as ::core::ffi::c_int,
                    );
                    bs_write1(s, 0 as uint32_t);
                    bs_write1(s, 0 as uint32_t);
                }
            }
        }
        bs_write_se(s, (*pps).i_chroma_qp_index_offset);
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
#[c2rust::src_loc = "574:1"]
pub unsafe extern "C" fn x264_10_sei_recovery_point_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
    mut recovery_frame_cnt: ::core::ffi::c_int,
) {
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        100 as ::core::ffi::c_int,
    );
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, recovery_frame_cnt as ::core::ffi::c_uint);
    bs_write1(&mut q, 1 as uint32_t);
    bs_write1(&mut q, 0 as uint32_t);
    bs_write(&mut q, 2 as ::core::ffi::c_int, 0 as uint32_t);
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as ::core::ffi::c_int,
        SEI_RECOVERY_POINT as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "593:1"]
pub unsafe extern "C" fn x264_10_sei_version_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) -> ::core::ffi::c_int {
    static mut uuid: [uint8_t; 16] = [
        0xdc as ::core::ffi::c_int as uint8_t,
        0x45 as ::core::ffi::c_int as uint8_t,
        0xe9 as ::core::ffi::c_int as uint8_t,
        0xbd as ::core::ffi::c_int as uint8_t,
        0xe6 as ::core::ffi::c_int as uint8_t,
        0xd9 as ::core::ffi::c_int as uint8_t,
        0x48 as ::core::ffi::c_int as uint8_t,
        0xb7 as ::core::ffi::c_int as uint8_t,
        0x96 as ::core::ffi::c_int as uint8_t,
        0x2c as ::core::ffi::c_int as uint8_t,
        0xd8 as ::core::ffi::c_int as uint8_t,
        0x20 as ::core::ffi::c_int as uint8_t,
        0xd9 as ::core::ffi::c_int as uint8_t,
        0x23 as ::core::ffi::c_int as uint8_t,
        0xee as ::core::ffi::c_int as uint8_t,
        0xef as ::core::ffi::c_int as uint8_t,
    ];
    let mut opts: *mut ::core::ffi::c_char = x264_param2string(
        &mut (*h).param,
        0 as ::core::ffi::c_int,
    );
    let mut payload: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut length: ::core::ffi::c_int = 0;
    if opts.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    payload = x264_malloc((200 as size_t).wrapping_add(strlen(opts)) as int64_t)
        as *mut ::core::ffi::c_char;
    if payload.is_null() {
        x264_free(opts as *mut ::core::ffi::c_void);
        return -(1 as ::core::ffi::c_int);
    } else {
        memcpy(
            payload as *mut ::core::ffi::c_void,
            uuid.as_ptr() as *const ::core::ffi::c_void,
            16 as size_t,
        );
        sprintf(
            payload.offset(16 as ::core::ffi::c_int as isize),
            b"x264 - core %d%s - H.264/MPEG-4 AVC codec - Copy%s 2003-2025 - http://www.videolan.org/x264.html - options: %s\0"
                as *const u8 as *const ::core::ffi::c_char,
            X264_BUILD,
            X264_VERSION.as_ptr(),
            if HAVE_GPL != 0 {
                b"left\0" as *const u8 as *const ::core::ffi::c_char
            } else {
                b"right\0" as *const u8 as *const ::core::ffi::c_char
            },
            opts,
        );
        length = strlen(payload).wrapping_add(1 as size_t) as ::core::ffi::c_int;
        x264_10_sei_write(
            s,
            payload as *mut uint8_t,
            length,
            SEI_USER_DATA_UNREGISTERED as ::core::ffi::c_int,
        );
        x264_free(opts as *mut ::core::ffi::c_void);
        x264_free(payload as *mut ::core::ffi::c_void);
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
#[c2rust::src_loc = "625:1"]
pub unsafe extern "C" fn x264_10_sei_buffering_period_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut sps: *mut x264_sps_t = (*h).sps.as_mut_ptr();
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        100 as ::core::ffi::c_int,
    );
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, (*sps).i_id as ::core::ffi::c_uint);
    if (*sps).vui.b_nal_hrd_parameters_present != 0 {
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_initial_cpb_removal_delay_length,
            (*h).initial_cpb_removal_delay as uint32_t,
        );
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_initial_cpb_removal_delay_length,
            (*h).initial_cpb_removal_delay_offset as uint32_t,
        );
    }
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as ::core::ffi::c_int,
        SEI_BUFFERING_PERIOD as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "647:1"]
pub unsafe extern "C" fn x264_10_sei_pic_timing_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut sps: *mut x264_sps_t = (*h).sps.as_mut_ptr();
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        100 as ::core::ffi::c_int,
    );
    bs_realign(&mut q);
    if (*sps).vui.b_nal_hrd_parameters_present != 0
        || (*sps).vui.b_vcl_hrd_parameters_present != 0
    {
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_cpb_removal_delay_length,
            ((*(*h).fenc).i_cpb_delay - (*h).i_cpb_delay_pir_offset) as uint32_t,
        );
        bs_write(
            &mut q,
            (*sps).vui.hrd.i_dpb_output_delay_length,
            (*(*h).fenc).i_dpb_output_delay as uint32_t,
        );
    }
    if (*sps).vui.b_pic_struct_present != 0 {
        bs_write(
            &mut q,
            4 as ::core::ffi::c_int,
            ((*(*h).fenc).i_pic_struct - 1 as ::core::ffi::c_int) as uint32_t,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < num_clock_ts[(*(*h).fenc).i_pic_struct as usize] as ::core::ffi::c_int
        {
            bs_write1(&mut q, 0 as uint32_t);
            i += 1;
        }
    }
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as ::core::ffi::c_int,
        SEI_PIC_TIMING as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "678:1"]
pub unsafe extern "C" fn x264_10_sei_frame_packing_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut quincunx_sampling_flag: ::core::ffi::c_int = ((*h).param.i_frame_packing
        == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        100 as ::core::ffi::c_int,
    );
    bs_realign(&mut q);
    bs_write_ue_big(&mut q, 0 as ::core::ffi::c_uint);
    bs_write1(&mut q, 0 as uint32_t);
    bs_write(&mut q, 7 as ::core::ffi::c_int, (*h).param.i_frame_packing as uint32_t);
    bs_write1(&mut q, quincunx_sampling_flag as uint32_t);
    bs_write(
        &mut q,
        6 as ::core::ffi::c_int,
        ((*h).param.i_frame_packing != 6 as ::core::ffi::c_int) as ::core::ffi::c_int
            as uint32_t,
    );
    bs_write1(&mut q, 0 as uint32_t);
    bs_write1(&mut q, 0 as uint32_t);
    bs_write1(&mut q, 0 as uint32_t);
    bs_write1(
        &mut q,
        ((*h).param.i_frame_packing == 5 as ::core::ffi::c_int
            && (*(*h).fenc).i_frame & 1 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
            as uint32_t,
    );
    bs_write1(&mut q, 0 as uint32_t);
    bs_write1(&mut q, 0 as uint32_t);
    if quincunx_sampling_flag == 0 as ::core::ffi::c_int
        && (*h).param.i_frame_packing != 5 as ::core::ffi::c_int
    {
        bs_write(&mut q, 4 as ::core::ffi::c_int, 0 as uint32_t);
        bs_write(&mut q, 4 as ::core::ffi::c_int, 0 as uint32_t);
        bs_write(&mut q, 4 as ::core::ffi::c_int, 0 as uint32_t);
        bs_write(&mut q, 4 as ::core::ffi::c_int, 0 as uint32_t);
    }
    bs_write(&mut q, 8 as ::core::ffi::c_int, 0 as uint32_t);
    bs_write_ue_big(
        &mut q,
        ((*h).param.i_frame_packing != 5 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    bs_write1(&mut q, 0 as uint32_t);
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as ::core::ffi::c_int,
        SEI_FRAME_PACKING as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "720:1"]
pub unsafe extern "C" fn x264_10_sei_mastering_display_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        100 as ::core::ffi::c_int,
    );
    bs_realign(&mut q);
    bs_write(
        &mut q,
        16 as ::core::ffi::c_int,
        (*h).param.mastering_display.i_green_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as ::core::ffi::c_int,
        (*h).param.mastering_display.i_green_y as uint32_t,
    );
    bs_write(
        &mut q,
        16 as ::core::ffi::c_int,
        (*h).param.mastering_display.i_blue_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as ::core::ffi::c_int,
        (*h).param.mastering_display.i_blue_y as uint32_t,
    );
    bs_write(
        &mut q,
        16 as ::core::ffi::c_int,
        (*h).param.mastering_display.i_red_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as ::core::ffi::c_int,
        (*h).param.mastering_display.i_red_y as uint32_t,
    );
    bs_write(
        &mut q,
        16 as ::core::ffi::c_int,
        (*h).param.mastering_display.i_white_x as uint32_t,
    );
    bs_write(
        &mut q,
        16 as ::core::ffi::c_int,
        (*h).param.mastering_display.i_white_y as uint32_t,
    );
    bs_write32(&mut q, (*h).param.mastering_display.i_display_max as uint32_t);
    bs_write32(&mut q, (*h).param.mastering_display.i_display_min as uint32_t);
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as ::core::ffi::c_int,
        SEI_MASTERING_DISPLAY as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "745:1"]
pub unsafe extern "C" fn x264_10_sei_content_light_level_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        100 as ::core::ffi::c_int,
    );
    bs_realign(&mut q);
    bs_write(
        &mut q,
        16 as ::core::ffi::c_int,
        (*h).param.content_light_level.i_max_cll as uint32_t,
    );
    bs_write(
        &mut q,
        16 as ::core::ffi::c_int,
        (*h).param.content_light_level.i_max_fall as uint32_t,
    );
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as ::core::ffi::c_int,
        SEI_CONTENT_LIGHT_LEVEL as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "762:1"]
pub unsafe extern "C" fn x264_10_sei_alternative_transfer_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        100 as ::core::ffi::c_int,
    );
    bs_realign(&mut q);
    bs_write(
        &mut q,
        8 as ::core::ffi::c_int,
        (*h).param.i_alternative_transfer as uint32_t,
    );
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as ::core::ffi::c_int,
        SEI_ALTERNATIVE_TRANSFER as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "778:1"]
pub unsafe extern "C" fn x264_10_filler_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
    mut filler: ::core::ffi::c_int,
) {
    bs_realign(s);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < filler {
        bs_write(s, 8 as ::core::ffi::c_int, 0xff as uint32_t);
        i += 1;
    }
    bs_rbsp_trailing(s);
    bs_flush(s);
}
#[no_mangle]
#[c2rust::src_loc = "789:1"]
pub unsafe extern "C" fn x264_10_sei_dec_ref_pic_marking_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) {
    let mut sh: *mut x264_slice_header_t = &mut (*h).sh_backup;
    let mut q: bs_t = bs_s {
        p_start: 0 as *mut uint8_t,
        p: 0 as *mut uint8_t,
        p_end: 0 as *mut uint8_t,
        cur_bits: 0,
        i_left: 0,
        i_bits_encoded: 0,
    };
    let mut tmp_buf: [uint8_t; 100] = [0; 100];
    (*(tmp_buf.as_mut_ptr() as *mut x264_union32_t)).i = 0 as uint32_t;
    bs_init(
        &mut q,
        tmp_buf.as_mut_ptr() as *mut ::core::ffi::c_void,
        100 as ::core::ffi::c_int,
    );
    bs_realign(&mut q);
    bs_write1(&mut q, 0 as uint32_t);
    bs_write_ue_big(&mut q, (*sh).i_frame_num as ::core::ffi::c_uint);
    if (*(*h).sps.as_mut_ptr()).b_frame_mbs_only == 0 {
        bs_write1(&mut q, 0 as uint32_t);
    }
    bs_write1(
        &mut q,
        ((*sh).i_mmco_command_count > 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as uint32_t,
    );
    if (*sh).i_mmco_command_count > 0 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*sh).i_mmco_command_count {
            bs_write_ue_big(&mut q, 1 as ::core::ffi::c_uint);
            bs_write_ue_big(
                &mut q,
                ((*sh).mmco[i as usize].i_difference_of_pic_nums
                    - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            );
            i += 1;
        }
        bs_write_ue_big(&mut q, 0 as ::core::ffi::c_uint);
    }
    bs_align_10(&mut q);
    x264_10_sei_write(
        s,
        tmp_buf.as_mut_ptr(),
        bs_pos(&mut q) / 8 as ::core::ffi::c_int,
        SEI_DEC_REF_PIC_MARKING as ::core::ffi::c_int,
    );
}
#[no_mangle]
#[c2rust::src_loc = "821:1"]
pub unsafe extern "C" fn x264_10_sei_avcintra_umid_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
) -> ::core::ffi::c_int {
    let mut data: [uint8_t; 512] = [0; 512];
    let mut msg: *const ::core::ffi::c_char = b"UMID\0" as *const u8
        as *const ::core::ffi::c_char;
    let len: ::core::ffi::c_int = 497 as ::core::ffi::c_int;
    memset(
        data.as_mut_ptr() as *mut ::core::ffi::c_void,
        0xff as ::core::ffi::c_int,
        len as size_t,
    );
    memcpy(
        data.as_mut_ptr() as *mut ::core::ffi::c_void,
        avcintra_uuid.as_ptr() as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    memcpy(
        data.as_mut_ptr().offset(16 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        msg as *const ::core::ffi::c_void,
        strlen(msg),
    );
    data[20 as ::core::ffi::c_int as usize] = 0x13 as uint8_t;
    data[26 as ::core::ffi::c_int as usize] = 0 as uint8_t;
    data[25 as ::core::ffi::c_int as usize] = data[26 as ::core::ffi::c_int as usize];
    data[23 as ::core::ffi::c_int as usize] = data[25 as ::core::ffi::c_int as usize];
    data[22 as ::core::ffi::c_int as usize] = data[23 as ::core::ffi::c_int as usize];
    data[28 as ::core::ffi::c_int as usize] = 0x14 as uint8_t;
    data[34 as ::core::ffi::c_int as usize] = 0 as uint8_t;
    data[33 as ::core::ffi::c_int as usize] = data[34 as ::core::ffi::c_int as usize];
    data[31 as ::core::ffi::c_int as usize] = data[33 as ::core::ffi::c_int as usize];
    data[30 as ::core::ffi::c_int as usize] = data[31 as ::core::ffi::c_int as usize];
    data[36 as ::core::ffi::c_int as usize] = 0x60 as uint8_t;
    data[41 as ::core::ffi::c_int as usize] = 0x22 as uint8_t;
    data[60 as ::core::ffi::c_int as usize] = 0x62 as uint8_t;
    data[66 as ::core::ffi::c_int as usize] = 0 as uint8_t;
    data[65 as ::core::ffi::c_int as usize] = data[66 as ::core::ffi::c_int as usize];
    data[63 as ::core::ffi::c_int as usize] = data[65 as ::core::ffi::c_int as usize];
    data[62 as ::core::ffi::c_int as usize] = data[63 as ::core::ffi::c_int as usize];
    data[68 as ::core::ffi::c_int as usize] = 0x63 as uint8_t;
    data[74 as ::core::ffi::c_int as usize] = 0 as uint8_t;
    data[73 as ::core::ffi::c_int as usize] = data[74 as ::core::ffi::c_int as usize];
    data[71 as ::core::ffi::c_int as usize] = data[73 as ::core::ffi::c_int as usize];
    data[70 as ::core::ffi::c_int as usize] = data[71 as ::core::ffi::c_int as usize];
    x264_10_sei_write(
        &mut (*h).out.bs,
        data.as_mut_ptr(),
        len,
        SEI_USER_DATA_UNREGISTERED as ::core::ffi::c_int,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "849:1"]
pub unsafe extern "C" fn x264_10_sei_avcintra_vanc_write(
    mut h: *mut x264_t,
    mut s: *mut bs_t,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut data: [uint8_t; 6000] = [0; 6000];
    let mut msg: *const ::core::ffi::c_char = b"VANC\0" as *const u8
        as *const ::core::ffi::c_char;
    if len < 0 as ::core::ffi::c_int
        || len as ::core::ffi::c_uint as usize
            > ::core::mem::size_of::<[uint8_t; 6000]>() as usize
    {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"AVC-Intra SEI is too large (%d)\n\0" as *const u8
                as *const ::core::ffi::c_char,
            len,
        );
        return -(1 as ::core::ffi::c_int);
    }
    memset(
        data.as_mut_ptr() as *mut ::core::ffi::c_void,
        0xff as ::core::ffi::c_int,
        len as size_t,
    );
    memcpy(
        data.as_mut_ptr() as *mut ::core::ffi::c_void,
        avcintra_uuid.as_ptr() as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    memcpy(
        data.as_mut_ptr().offset(16 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        msg as *const ::core::ffi::c_void,
        strlen(msg),
    );
    x264_10_sei_write(
        &mut (*h).out.bs,
        data.as_mut_ptr(),
        len,
        SEI_USER_DATA_UNREGISTERED as ::core::ffi::c_int,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "876:1"]
pub unsafe extern "C" fn x264_10_validate_levels(
    mut h: *mut x264_t,
    mut verbose: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut mbs: ::core::ffi::c_int = (*(*h).sps.as_mut_ptr()).i_mb_width
        * (*(*h).sps.as_mut_ptr()).i_mb_height;
    let mut dpb: ::core::ffi::c_int = mbs
        * (*(*h).sps.as_mut_ptr()).vui.i_max_dec_frame_buffering;
    let mut cbp_factor: ::core::ffi::c_int = if (*(*h).sps.as_mut_ptr()).i_profile_idc
        >= PROFILE_HIGH422 as ::core::ffi::c_int
    {
        16 as ::core::ffi::c_int
    } else if (*(*h).sps.as_mut_ptr()).i_profile_idc
        == PROFILE_HIGH10 as ::core::ffi::c_int
    {
        12 as ::core::ffi::c_int
    } else if (*(*h).sps.as_mut_ptr()).i_profile_idc
        == PROFILE_HIGH as ::core::ffi::c_int
    {
        5 as ::core::ffi::c_int
    } else {
        4 as ::core::ffi::c_int
    };
    let mut l: *const x264_level_t = x264_levels.as_ptr();
    while (*l).level_idc as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && (*l).level_idc as ::core::ffi::c_int != (*h).param.i_level_idc
    {
        l = l.offset(1);
    }
    if (*l).frame_size < mbs as int32_t
        || ((*l).frame_size * 8 as int32_t)
            < (*(*h).sps.as_mut_ptr()).i_mb_width as int32_t
                * (*(*h).sps.as_mut_ptr()).i_mb_width as int32_t
        || ((*l).frame_size * 8 as int32_t)
            < (*(*h).sps.as_mut_ptr()).i_mb_height as int32_t
                * (*(*h).sps.as_mut_ptr()).i_mb_height as int32_t
    {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"frame MB size (%dx%d) > level limit (%d)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*(*h).sps.as_mut_ptr()).i_mb_width,
                (*(*h).sps.as_mut_ptr()).i_mb_height,
                (*l).frame_size,
            );
        }
        ret = 1 as ::core::ffi::c_int;
    }
    if dpb as int32_t > (*l).dpb {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"DPB size (%d frames, %d mbs) > level limit (%d frames, %d mbs)\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*(*h).sps.as_mut_ptr()).vui.i_max_dec_frame_buffering,
                dpb,
                (*l).dpb / mbs as int32_t,
                (*l).dpb,
            );
        }
        ret = 1 as ::core::ffi::c_int;
    }
    if (*h).param.rc.i_vbv_max_bitrate as int32_t
        > (*l).bitrate * cbp_factor as int32_t / 4 as int32_t
    {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV bitrate (%ld) > level limit (%d)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).param.rc.i_vbv_max_bitrate as int64_t,
                (*l).bitrate * cbp_factor as int32_t / 4 as int32_t,
            );
        }
        ret = 1 as ::core::ffi::c_int;
    }
    if (*h).param.rc.i_vbv_buffer_size as int32_t
        > (*l).cpb * cbp_factor as int32_t / 4 as int32_t
    {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"VBV buffer (%ld) > level limit (%d)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).param.rc.i_vbv_buffer_size as int64_t,
                (*l).cpb * cbp_factor as int32_t / 4 as int32_t,
            );
        }
        ret = 1 as ::core::ffi::c_int;
    }
    if (*h).param.analyse.i_mv_range > (*l).mv_range as ::core::ffi::c_int {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"MV range (%ld) > level limit (%d)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).param.analyse.i_mv_range as int64_t,
                (*l).mv_range as ::core::ffi::c_int,
            );
        }
        ret = 1 as ::core::ffi::c_int;
    }
    if (*h).param.b_interlaced > ((*l).frame_only == 0) as ::core::ffi::c_int {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"interlaced (%ld) > level limit (%d)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).param.b_interlaced as int64_t,
                ((*l).frame_only == 0) as ::core::ffi::c_int,
            );
        }
        ret = 1 as ::core::ffi::c_int;
    }
    if (*h).param.b_fake_interlaced > ((*l).frame_only == 0) as ::core::ffi::c_int {
        if verbose != 0 {
            x264_10_log(
                h,
                X264_LOG_WARNING,
                b"fake interlaced (%ld) > level limit (%d)\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*h).param.b_fake_interlaced as int64_t,
                ((*l).frame_only == 0) as ::core::ffi::c_int,
            );
        }
        ret = 1 as ::core::ffi::c_int;
    }
    if (*h).param.i_fps_den > 0 as uint32_t {
        if mbs as int64_t * (*h).param.i_fps_num as int64_t
            / (*h).param.i_fps_den as int64_t > (*l).mbps as int64_t
        {
            if verbose != 0 {
                x264_10_log(
                    h,
                    X264_LOG_WARNING,
                    b"MB rate (%ld) > level limit (%d)\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    mbs as int64_t * (*h).param.i_fps_num as int64_t
                        / (*h).param.i_fps_den as int64_t,
                    (*l).mbps,
                );
            }
            ret = 1 as ::core::ffi::c_int;
        }
    }
    return ret;
}
