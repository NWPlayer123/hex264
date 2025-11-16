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
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:29"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:29"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:29"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:29"]
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
#[c2rust::header_src = "/usr/include/stdint.h:29"]
pub mod stdint_h {
    #[c2rust::src_loc = "76:1"]
    pub type intptr_t = isize;
    #[c2rust::src_loc = "79:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/atomic_wide_counter.h:29"]
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
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:29"]
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
#[c2rust::header_src = "/usr/include/bits/struct_mutex.h:29"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:29"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/common.h:29"]
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
    #[c2rust::src_loc = "111:9"]
    pub const SIZEOF_PIXEL: ::core::ffi::c_int =
        ::core::mem::size_of::<pixel>() as ::core::ffi::c_int;
    #[c2rust::src_loc = "570:9"]
    pub const FENC_STRIDE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    #[c2rust::src_loc = "571:9"]
    pub const FDEC_STRIDE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/frame.h:29"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:29"]
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
    #[c2rust::src_loc = "209:9"]
    pub const X264_ME_ESA: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    #[c2rust::src_loc = "227:9"]
    pub const X264_WEIGHTP_SMART: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    use super::common_h::x264_t;
    use super::internal::__va_list_tag;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/mc.h:29"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/bitstream.h:29"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/cabac.h:29"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/quant.h:29"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/dct.h:29"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/pixel.h:29"]
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
    #[c2rust::src_loc = "61:22"]
    pub static mut x264_size2pixel: [[uint8_t; 5]; 5] = [
        [0 as ::core::ffi::c_int as uint8_t, 0, 0, 0, 0],
        [
            0 as ::core::ffi::c_int as uint8_t,
            PIXEL_4x4 as ::core::ffi::c_int as uint8_t,
            PIXEL_8x4 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        [
            0 as ::core::ffi::c_int as uint8_t,
            PIXEL_4x8 as ::core::ffi::c_int as uint8_t,
            PIXEL_8x8 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            PIXEL_16x8 as ::core::ffi::c_int as uint8_t,
        ],
        [0 as ::core::ffi::c_int as uint8_t, 0, 0, 0, 0],
        [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            PIXEL_8x16 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            PIXEL_16x16 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    use super::common_h::pixel;
    use super::stdint_h::intptr_t;
    use super::stdint_intn_h::int16_t;
    use super::stdint_uintn_h::{uint16_t, uint64_t, uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/predict.h:29"]
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
    use super::common_h::pixel;
    use super::stdint_uintn_h::uint8_t;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/set.h:29"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/threadpool.h:29"]
pub mod threadpool_h {
    extern "C" {
        #[c2rust::src_loc = "29:16"]
        pub type x264_threadpool_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:29"]
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
    #[c2rust::src_loc = "151:9"]
    pub const X264_WEIGHTP_FAKE: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
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
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "279:10"]
        pub fn x264_malloc(_: int64_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "280:10"]
        pub fn x264_free(_: *mut ::core::ffi::c_void);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/macroblock.h:29"]
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
    #[c2rust::src_loc = "90:22"]
    pub static mut x264_mb_type_fix: [uint8_t; 19] = [
        I_4x4 as ::core::ffi::c_int as uint8_t,
        I_4x4 as ::core::ffi::c_int as uint8_t,
        I_16x16 as ::core::ffi::c_int as uint8_t,
        I_PCM as ::core::ffi::c_int as uint8_t,
        P_L0 as ::core::ffi::c_int as uint8_t,
        P_8x8 as ::core::ffi::c_int as uint8_t,
        P_SKIP as ::core::ffi::c_int as uint8_t,
        B_DIRECT as ::core::ffi::c_int as uint8_t,
        B_L0_L0 as ::core::ffi::c_int as uint8_t,
        B_L0_L1 as ::core::ffi::c_int as uint8_t,
        B_L0_BI as ::core::ffi::c_int as uint8_t,
        B_L1_L0 as ::core::ffi::c_int as uint8_t,
        B_L1_L1 as ::core::ffi::c_int as uint8_t,
        B_L1_BI as ::core::ffi::c_int as uint8_t,
        B_BI_L0 as ::core::ffi::c_int as uint8_t,
        B_BI_L1 as ::core::ffi::c_int as uint8_t,
        B_BI_BI as ::core::ffi::c_int as uint8_t,
        B_8x8 as ::core::ffi::c_int as uint8_t,
        B_SKIP as ::core::ffi::c_int as uint8_t,
    ];
    #[inline(always)]
    #[c2rust::src_loc = "371:1"]
    pub unsafe extern "C" fn pack16to32(mut a: uint32_t, mut b: uint32_t) -> uint32_t {
        return a.wrapping_add(b << 16 as ::core::ffi::c_int);
    }
    #[inline(always)]
    #[c2rust::src_loc = "387:1"]
    pub unsafe extern "C" fn pack8to32(
        mut a: uint32_t,
        mut b: uint32_t,
        mut c: uint32_t,
        mut d: uint32_t,
    ) -> uint32_t {
        return a
            .wrapping_add(b << 8 as ::core::ffi::c_int)
            .wrapping_add(c << 16 as ::core::ffi::c_int)
            .wrapping_add(d << 24 as ::core::ffi::c_int);
    }
    use super::common_h::x264_t;
    use super::stdint_intn_h::int16_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
    extern "C" {
        #[c2rust::src_loc = "344:1"]
        pub fn x264_10_mb_predict_mv_pskip(h: *mut x264_t, mv: *mut int16_t);
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/xmmintrin.h:29"]
pub mod xmmintrin_h {
    #[cfg(target_arch = "x86")]
    pub use ::core::arch::x86::__m128;
    #[cfg(target_arch = "x86_64")]
    pub use ::core::arch::x86_64::__m128;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/x86/util.h:29"]
pub mod util_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "36:9"]
    pub union x264_union128_sse_t {
        pub i: __m128,
        pub q: [uint64_t; 2],
        pub d: [uint32_t; 4],
        pub w: [uint16_t; 8],
        pub b: [uint8_t; 16],
    }
    #[c2rust::src_loc = "34:9"]
    pub const M128_ZERO: __m128 = unsafe { core::mem::transmute([0.0f32, 0.0f32, 0.0f32, 0.0f32]) };
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
    use super::xmmintrin_h::__m128;
    #[cfg(target_arch = "x86")]
    pub use ::core::arch::x86::_mm_setr_ps;
    #[cfg(target_arch = "x86_64")]
    pub use ::core::arch::x86_64::_mm_setr_ps;
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:29"]
pub mod osdep_h {
    #[c2rust::src_loc = "452:9"]
    pub const WORD_SIZE: uint64_t = ::core::mem::size_of::<*mut ::core::ffi::c_void>() as uint64_t;
    use super::stdint_uintn_h::uint64_t;
}
#[c2rust::header_src = "/usr/include/string.h:29"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/tables.h:29"]
pub mod tables_h {
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "100:16"]
        pub static mut x264_zero: [uint8_t; 1024];
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/rectangle.h:29"]
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
                    ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
                        *b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
                    )
                    .as_ptr(),
                );
            }
            'c_21741: {
                if h != 1 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"h != 1\0" as *const u8 as *const ::core::ffi::c_char,
                        b"common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
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
                b"common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
                108 as ::core::ffi::c_uint,
                ::core::mem::transmute::<[u8; 65], [::core::ffi::c_char; 65]>(
                    *b"void x264_macroblock_cache_rect(void *, int, int, int, uint32_t)\0",
                )
                .as_ptr(),
            );
            'c_21507: {
                __assert_fail(
                    b"0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"common/rectangle.h\0" as *const u8 as *const ::core::ffi::c_char,
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
    #[c2rust::src_loc = "143:1"]
    pub unsafe extern "C" fn x264_macroblock_cache_skip(
        mut h: *mut x264_t,
        mut x: ::core::ffi::c_int,
        mut y: ::core::ffi::c_int,
        mut width: ::core::ffi::c_int,
        mut height: ::core::ffi::c_int,
        mut b_skip: ::core::ffi::c_int,
    ) {
        x264_macroblock_cache_rect(
            &mut *(*h)
                .mb
                .cache
                .skip
                .as_mut_ptr()
                .offset((X264_SCAN8_0 + x + 8 as ::core::ffi::c_int * y) as isize)
                as *mut int8_t as *mut ::core::ffi::c_void,
            width,
            height,
            1 as ::core::ffi::c_int,
            b_skip as uint32_t,
        );
    }
    use super::assert_h::__assert_fail;
    use super::base_h::{x264_union16_t, x264_union32_t, x264_union64_t, X264_SCAN8_0};
    use super::common_h::x264_t;
    use super::osdep_h::WORD_SIZE;
    use super::stdint_intn_h::int8_t;
    use super::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
}
#[c2rust::header_src = "/usr/include/stdlib.h:29"]
pub mod stdlib_h {
    extern "C" {
        #[c2rust::src_loc = "980:1"]
        pub fn abs(__x: ::core::ffi::c_int) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:29"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
#[c2rust::header_src = "/usr/include/assert.h:29"]
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
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
use self::assert_h::__assert_fail;
pub use self::atomic_wide_counter_h::{C2RustUnnamed, __atomic_wide_counter};
pub use self::base_h::{
    chroma_format_e, slice_type_e, x264_clip3, x264_free, x264_malloc, x264_scan8, x264_union16_t,
    x264_union32_t, x264_union64_t, CHROMA_400, CHROMA_420, CHROMA_422, CHROMA_444, SLICE_TYPE_B,
    SLICE_TYPE_I, SLICE_TYPE_P, X264_SCAN8_0, X264_WEIGHTP_FAKE,
};
pub use self::bitstream_h::{bs_s, bs_t, x264_bitstream_function_t, x264_run_level_t};
pub use self::cabac_h::x264_cabac_t;
pub use self::common_h::{
    dctcoef, mvsad_t, pixel, udctcoef, x264_frame_stat_t, x264_left_table_t, x264_lookahead_t,
    x264_ratecontrol_t, x264_slice_header_t, x264_t, C2RustUnnamed_10, C2RustUnnamed_11,
    C2RustUnnamed_12, C2RustUnnamed_13, C2RustUnnamed_17, C2RustUnnamed_18, C2RustUnnamed_6,
    C2RustUnnamed_7, C2RustUnnamed_8, C2RustUnnamed_9, FDEC_STRIDE, FENC_STRIDE, SIZEOF_PIXEL,
};
pub use self::dct_h::{x264_dct_function_t, x264_zigzag_function_t};
pub use self::frame_h::{
    x264_deblock_function_t, x264_deblock_inter_t, x264_deblock_intra_t, x264_frame, x264_frame_t,
    x264_sync_frame_list_t, PADV,
};
pub use self::internal::{__va_list_tag, BIT_DEPTH};
pub use self::macroblock_h::{
    macroblock_position_e, mb_class_e, mb_partition_e, pack16to32, pack8to32,
    x264_10_mb_predict_mv_pskip, x264_mb_type_fix, B_8x8, D_16x16, D_16x8, D_8x16, D_8x8, D_BI_4x4,
    D_BI_4x8, D_BI_8x4, D_BI_8x8, D_DIRECT_8x8, D_L0_4x4, D_L0_4x8, D_L0_8x4, D_L0_8x8, D_L1_4x4,
    D_L1_4x8, D_L1_8x4, D_L1_8x8, I_16x16, I_4x4, I_8x8, P_8x8, ALL_NEIGHBORS, B_BI_BI, B_BI_L0,
    B_BI_L1, B_DIRECT, B_L0_BI, B_L0_L0, B_L0_L1, B_L1_BI, B_L1_L0, B_L1_L1, B_SKIP, I_PCM,
    MB_LEFT, MB_PRIVATE, MB_TOP, MB_TOPLEFT, MB_TOPRIGHT, P_L0, P_SKIP, X264_MBTYPE_MAX,
    X264_PARTTYPE_MAX,
};
pub use self::mc_h::{weight_fn_t, x264_mc_functions_t, x264_weight_t};
pub use self::osdep_h::WORD_SIZE;
pub use self::pixel_h::{
    x264_pixel_cmp_t, x264_pixel_cmp_x3_t, x264_pixel_cmp_x4_t, x264_pixel_function_t,
    x264_size2pixel, C2RustUnnamed_19, PIXEL_16x16, PIXEL_16x8, PIXEL_2x2, PIXEL_2x4, PIXEL_2x8,
    PIXEL_4x16, PIXEL_4x2, PIXEL_4x4, PIXEL_4x8, PIXEL_8x16, PIXEL_8x4, PIXEL_8x8,
};
pub use self::predict_h::{
    intra4x4_pred_e, intra_chroma_pred_e, x264_mb_chroma_pred_mode_fix, x264_predict8x8_t,
    x264_predict_8x8_filter_t, x264_predict_t, I_PRED_4x4_DC, I_PRED_4x4_DC_128,
    I_PRED_4x4_DC_LEFT, I_PRED_4x4_DC_TOP, I_PRED_4x4_DDL, I_PRED_4x4_DDR, I_PRED_4x4_H,
    I_PRED_4x4_HD, I_PRED_4x4_HU, I_PRED_4x4_V, I_PRED_4x4_VL, I_PRED_4x4_VR, I_PRED_CHROMA_DC,
    I_PRED_CHROMA_DC_128, I_PRED_CHROMA_DC_LEFT, I_PRED_CHROMA_DC_TOP, I_PRED_CHROMA_H,
    I_PRED_CHROMA_P, I_PRED_CHROMA_V,
};
pub use self::pthreadtypes_h::{pthread_cond_t, pthread_mutex_t, pthread_t};
pub use self::quant_h::x264_quant_function_t;
pub use self::rectangle_h::{x264_macroblock_cache_rect, x264_macroblock_cache_skip};
pub use self::set_h::{
    x264_pps_t, x264_sps_t, C2RustUnnamed_14, C2RustUnnamed_15, C2RustUnnamed_16,
};
pub use self::stdint_h::{intptr_t, uintptr_t};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use self::stdlib_h::abs;
use self::string_h::{memcpy, memset};
pub use self::struct_mutex_h::__pthread_mutex_s;
use self::tables_h::x264_zero;
pub use self::thread_shared_types_h::{
    __pthread_cond_s, __pthread_internal_list, __pthread_list_t,
};
use self::threadpool_h::x264_threadpool_t;
pub use self::types_h::{
    __int16_t, __int32_t, __int64_t, __int8_t, __uint16_t, __uint32_t, __uint64_t, __uint8_t,
};
pub use self::util_h::{x264_union128_sse_t, M128_ZERO};
pub use self::x264_h::{
    x264_hrd_t, x264_nal_t, x264_param_t, x264_sei_payload_t, x264_sei_t, x264_zone_t,
    C2RustUnnamed_0, C2RustUnnamed_1, C2RustUnnamed_2, C2RustUnnamed_3, C2RustUnnamed_4,
    C2RustUnnamed_5, X264_ME_ESA, X264_WEIGHTP_SMART,
};
#[cfg(target_arch = "x86")]
pub use ::core::arch::x86::_mm_setr_ps;
#[cfg(target_arch = "x86_64")]
pub use ::core::arch::x86_64::_mm_setr_ps;
#[inline(never)]
#[c2rust::src_loc = "37:1"]
unsafe extern "C" fn mb_mc_0xywh(
    mut h: *mut x264_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    let mut i8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int
        + x
        + 8 as ::core::ffi::c_int * y;
    let mut i_ref: ::core::ffi::c_int =
        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize][i8 as usize] as ::core::ffi::c_int;
    let mut mvx: ::core::ffi::c_int = x264_clip3(
        (*h).mb.cache.mv[0 as ::core::ffi::c_int as usize][i8 as usize]
            [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*h).mb.mv_min[0 as ::core::ffi::c_int as usize],
        (*h).mb.mv_max[0 as ::core::ffi::c_int as usize],
    ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * x;
    let mut mvy: ::core::ffi::c_int = x264_clip3(
        (*h).mb.cache.mv[0 as ::core::ffi::c_int as usize][i8 as usize]
            [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*h).mb.mv_min[1 as ::core::ffi::c_int as usize],
        (*h).mb.mv_max[1 as ::core::ffi::c_int as usize],
    ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * y;
    (*h).mc.mc_luma.expect("non-null function pointer")(
        &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .offset((4 as ::core::ffi::c_int * y * FDEC_STRIDE + 4 as ::core::ffi::c_int * x) as isize),
        FDEC_STRIDE as intptr_t,
        &mut *(*(*(*h)
            .mb
            .pic
            .p_fref
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(i_ref as isize))
        .as_mut_ptr()
        .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
        (*h).mb.pic.i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
        mvx,
        mvy,
        4 as ::core::ffi::c_int * width,
        4 as ::core::ffi::c_int * height,
        if 0 as ::core::ffi::c_int != 0 {
            x264_zero.as_mut_ptr() as *const x264_weight_t
        } else {
            &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize) as *mut x264_weight_t
                as *const x264_weight_t
        },
    );
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
        (*h).mc.mc_luma.expect("non-null function pointer")(
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .offset(
                (4 as ::core::ffi::c_int * y * FDEC_STRIDE + 4 as ::core::ffi::c_int * x) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            &mut *(*(*(*h)
                .mb
                .pic
                .p_fref
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref as isize))
            .as_mut_ptr()
            .offset((1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
            (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            if 0 as ::core::ffi::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
        (*h).mc.mc_luma.expect("non-null function pointer")(
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(2 as ::core::ffi::c_int as isize))
            .offset(
                (4 as ::core::ffi::c_int * y * FDEC_STRIDE + 4 as ::core::ffi::c_int * x) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            &mut *(*(*(*h)
                .mb
                .pic
                .p_fref
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref as isize))
            .as_mut_ptr()
            .offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
            (*h).mb.pic.i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            if 0 as ::core::ffi::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2 as ::core::ffi::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut v_shift: ::core::ffi::c_int = (*h).mb.chroma_v_shift;
        if v_shift & (*h).mb.b_interlaced & i_ref != 0 {
            mvy += ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int;
        }
        let mut offset: ::core::ffi::c_int =
            (4 as ::core::ffi::c_int * FDEC_STRIDE >> v_shift) * y + 2 as ::core::ffi::c_int * x;
        height = 4 as ::core::ffi::c_int * height >> v_shift;
        (*h).mc.mc_chroma.expect("non-null function pointer")(
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .offset(offset as isize),
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(2 as ::core::ffi::c_int as isize))
            .offset(offset as isize),
            FDEC_STRIDE as intptr_t,
            (*h).mb.pic.p_fref[0 as ::core::ffi::c_int as usize][i_ref as usize]
                [4 as ::core::ffi::c_int as usize],
            (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
            mvx,
            2 as ::core::ffi::c_int * mvy >> v_shift,
            2 as ::core::ffi::c_int * width,
            height,
        );
        if !(*h).sh.weight[i_ref as usize][1 as ::core::ffi::c_int as usize]
            .weightfn
            .is_null()
        {
            (*(*h).sh.weight[i_ref as usize][1 as ::core::ffi::c_int as usize]
                .weightfn
                .offset((width >> 1 as ::core::ffi::c_int) as isize))
            .expect("non-null function pointer")(
                &mut *(*(*h)
                    .mb
                    .pic
                    .p_fdec
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(offset as isize),
                FDEC_STRIDE as intptr_t,
                &mut *(*(*h)
                    .mb
                    .pic
                    .p_fdec
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(offset as isize),
                FDEC_STRIDE as intptr_t,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize),
                height,
            );
        }
        if !(*h).sh.weight[i_ref as usize][2 as ::core::ffi::c_int as usize]
            .weightfn
            .is_null()
        {
            (*(*h).sh.weight[i_ref as usize][2 as ::core::ffi::c_int as usize]
                .weightfn
                .offset((width >> 1 as ::core::ffi::c_int) as isize))
            .expect("non-null function pointer")(
                &mut *(*(*h)
                    .mb
                    .pic
                    .p_fdec
                    .as_mut_ptr()
                    .offset(2 as ::core::ffi::c_int as isize))
                .offset(offset as isize),
                FDEC_STRIDE as intptr_t,
                &mut *(*(*h)
                    .mb
                    .pic
                    .p_fdec
                    .as_mut_ptr()
                    .offset(2 as ::core::ffi::c_int as isize))
                .offset(offset as isize),
                FDEC_STRIDE as intptr_t,
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2 as ::core::ffi::c_int as isize),
                height,
            );
        }
    }
}
#[inline(never)]
#[c2rust::src_loc = "76:1"]
unsafe extern "C" fn mb_mc_1xywh(
    mut h: *mut x264_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    let mut i8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int
        + x
        + 8 as ::core::ffi::c_int * y;
    let mut i_ref: ::core::ffi::c_int =
        (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize][i8 as usize] as ::core::ffi::c_int;
    let mut mvx: ::core::ffi::c_int = x264_clip3(
        (*h).mb.cache.mv[1 as ::core::ffi::c_int as usize][i8 as usize]
            [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*h).mb.mv_min[0 as ::core::ffi::c_int as usize],
        (*h).mb.mv_max[0 as ::core::ffi::c_int as usize],
    ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * x;
    let mut mvy: ::core::ffi::c_int = x264_clip3(
        (*h).mb.cache.mv[1 as ::core::ffi::c_int as usize][i8 as usize]
            [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*h).mb.mv_min[1 as ::core::ffi::c_int as usize],
        (*h).mb.mv_max[1 as ::core::ffi::c_int as usize],
    ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * y;
    (*h).mc.mc_luma.expect("non-null function pointer")(
        &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .offset((4 as ::core::ffi::c_int * y * FDEC_STRIDE + 4 as ::core::ffi::c_int * x) as isize),
        FDEC_STRIDE as intptr_t,
        &mut *(*(*(*h)
            .mb
            .pic
            .p_fref
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(i_ref as isize))
        .as_mut_ptr()
        .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
        (*h).mb.pic.i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
        mvx,
        mvy,
        4 as ::core::ffi::c_int * width,
        4 as ::core::ffi::c_int * height,
        if 1 as ::core::ffi::c_int != 0 {
            x264_zero.as_mut_ptr() as *const x264_weight_t
        } else {
            &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize) as *mut x264_weight_t
                as *const x264_weight_t
        },
    );
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
        (*h).mc.mc_luma.expect("non-null function pointer")(
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .offset(
                (4 as ::core::ffi::c_int * y * FDEC_STRIDE + 4 as ::core::ffi::c_int * x) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            &mut *(*(*(*h)
                .mb
                .pic
                .p_fref
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref as isize))
            .as_mut_ptr()
            .offset((1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
            (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            if 1 as ::core::ffi::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
        (*h).mc.mc_luma.expect("non-null function pointer")(
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(2 as ::core::ffi::c_int as isize))
            .offset(
                (4 as ::core::ffi::c_int * y * FDEC_STRIDE + 4 as ::core::ffi::c_int * x) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            &mut *(*(*(*h)
                .mb
                .pic
                .p_fref
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref as isize))
            .as_mut_ptr()
            .offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
            (*h).mb.pic.i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
            mvx,
            mvy,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            if 1 as ::core::ffi::c_int != 0 {
                x264_zero.as_mut_ptr() as *const x264_weight_t
            } else {
                &mut *(*(*h).sh.weight.as_mut_ptr().offset(i_ref as isize))
                    .as_mut_ptr()
                    .offset(2 as ::core::ffi::c_int as isize) as *mut x264_weight_t
                    as *const x264_weight_t
            },
        );
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut v_shift: ::core::ffi::c_int = (*h).mb.chroma_v_shift;
        if v_shift & (*h).mb.b_interlaced & i_ref != 0 {
            mvy += ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int;
        }
        let mut offset: ::core::ffi::c_int =
            (4 as ::core::ffi::c_int * FDEC_STRIDE >> v_shift) * y + 2 as ::core::ffi::c_int * x;
        (*h).mc.mc_chroma.expect("non-null function pointer")(
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .offset(offset as isize),
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(2 as ::core::ffi::c_int as isize))
            .offset(offset as isize),
            FDEC_STRIDE as intptr_t,
            (*h).mb.pic.p_fref[1 as ::core::ffi::c_int as usize][i_ref as usize]
                [4 as ::core::ffi::c_int as usize],
            (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
            mvx,
            2 as ::core::ffi::c_int * mvy >> v_shift,
            2 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height >> v_shift,
        );
    }
}
#[inline(never)]
#[c2rust::src_loc = "112:1"]
unsafe extern "C" fn mb_mc_01xywh(
    mut h: *mut x264_t,
    mut x: ::core::ffi::c_int,
    mut y: ::core::ffi::c_int,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    let mut i8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int
        + x
        + 8 as ::core::ffi::c_int * y;
    let mut i_ref0: ::core::ffi::c_int =
        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize][i8 as usize] as ::core::ffi::c_int;
    let mut i_ref1: ::core::ffi::c_int =
        (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize][i8 as usize] as ::core::ffi::c_int;
    let mut weight: ::core::ffi::c_int =
        (*(*h).mb.bipred_weight.offset(i_ref0 as isize))[i_ref1 as usize] as ::core::ffi::c_int;
    let mut mvx0: ::core::ffi::c_int = x264_clip3(
        (*h).mb.cache.mv[0 as ::core::ffi::c_int as usize][i8 as usize]
            [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*h).mb.mv_min[0 as ::core::ffi::c_int as usize],
        (*h).mb.mv_max[0 as ::core::ffi::c_int as usize],
    ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * x;
    let mut mvx1: ::core::ffi::c_int = x264_clip3(
        (*h).mb.cache.mv[1 as ::core::ffi::c_int as usize][i8 as usize]
            [0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*h).mb.mv_min[0 as ::core::ffi::c_int as usize],
        (*h).mb.mv_max[0 as ::core::ffi::c_int as usize],
    ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * x;
    let mut mvy0: ::core::ffi::c_int = x264_clip3(
        (*h).mb.cache.mv[0 as ::core::ffi::c_int as usize][i8 as usize]
            [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*h).mb.mv_min[1 as ::core::ffi::c_int as usize],
        (*h).mb.mv_max[1 as ::core::ffi::c_int as usize],
    ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * y;
    let mut mvy1: ::core::ffi::c_int = x264_clip3(
        (*h).mb.cache.mv[1 as ::core::ffi::c_int as usize][i8 as usize]
            [1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        (*h).mb.mv_min[1 as ::core::ffi::c_int as usize],
        (*h).mb.mv_max[1 as ::core::ffi::c_int as usize],
    ) + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * y;
    let mut i_mode: ::core::ffi::c_int =
        x264_size2pixel[height as usize][width as usize] as ::core::ffi::c_int;
    let mut i_stride0: intptr_t = 16 as intptr_t;
    let mut i_stride1: intptr_t = 16 as intptr_t;
    let mut tmp0: [pixel; 256] = [0; 256];
    let mut tmp1: [pixel; 256] = [0; 256];
    let mut src0: *mut pixel = 0 as *mut pixel;
    let mut src1: *mut pixel = 0 as *mut pixel;
    src0 = (*h).mc.get_ref.expect("non-null function pointer")(
        tmp0.as_mut_ptr(),
        &mut i_stride0,
        &mut *(*(*(*h)
            .mb
            .pic
            .p_fref
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(i_ref0 as isize))
        .as_mut_ptr()
        .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
        (*h).mb.pic.i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
        mvx0,
        mvy0,
        4 as ::core::ffi::c_int * width,
        4 as ::core::ffi::c_int * height,
        x264_zero.as_mut_ptr() as *const x264_weight_t,
    );
    src1 = (*h).mc.get_ref.expect("non-null function pointer")(
        tmp1.as_mut_ptr(),
        &mut i_stride1,
        &mut *(*(*(*h)
            .mb
            .pic
            .p_fref
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(i_ref1 as isize))
        .as_mut_ptr()
        .offset((0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
        (*h).mb.pic.i_stride[0 as ::core::ffi::c_int as usize] as intptr_t,
        mvx1,
        mvy1,
        4 as ::core::ffi::c_int * width,
        4 as ::core::ffi::c_int * height,
        x264_zero.as_mut_ptr() as *const x264_weight_t,
    );
    (*h).mc.avg[i_mode as usize].expect("non-null function pointer")(
        &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .offset((4 as ::core::ffi::c_int * y * FDEC_STRIDE + 4 as ::core::ffi::c_int * x) as isize),
        FDEC_STRIDE as intptr_t,
        src0,
        i_stride0,
        src1,
        i_stride1,
        weight,
    );
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
        src0 = (*h).mc.get_ref.expect("non-null function pointer")(
            tmp0.as_mut_ptr(),
            &mut i_stride0,
            &mut *(*(*(*h)
                .mb
                .pic
                .p_fref
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref0 as isize))
            .as_mut_ptr()
            .offset((1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
            (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
            mvx0,
            mvy0,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        src1 = (*h).mc.get_ref.expect("non-null function pointer")(
            tmp1.as_mut_ptr(),
            &mut i_stride1,
            &mut *(*(*(*h)
                .mb
                .pic
                .p_fref
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref1 as isize))
            .as_mut_ptr()
            .offset((1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
            (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
            mvx1,
            mvy1,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        (*h).mc.avg[i_mode as usize].expect("non-null function pointer")(
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .offset(
                (4 as ::core::ffi::c_int * y * FDEC_STRIDE + 4 as ::core::ffi::c_int * x) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            src0,
            i_stride0,
            src1,
            i_stride1,
            weight,
        );
        src0 = (*h).mc.get_ref.expect("non-null function pointer")(
            tmp0.as_mut_ptr(),
            &mut i_stride0,
            &mut *(*(*(*h)
                .mb
                .pic
                .p_fref
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref0 as isize))
            .as_mut_ptr()
            .offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
            (*h).mb.pic.i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
            mvx0,
            mvy0,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        src1 = (*h).mc.get_ref.expect("non-null function pointer")(
            tmp1.as_mut_ptr(),
            &mut i_stride1,
            &mut *(*(*(*h)
                .mb
                .pic
                .p_fref
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(i_ref1 as isize))
            .as_mut_ptr()
            .offset((2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize),
            (*h).mb.pic.i_stride[2 as ::core::ffi::c_int as usize] as intptr_t,
            mvx1,
            mvy1,
            4 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height,
            x264_zero.as_mut_ptr() as *const x264_weight_t,
        );
        (*h).mc.avg[i_mode as usize].expect("non-null function pointer")(
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(2 as ::core::ffi::c_int as isize))
            .offset(
                (4 as ::core::ffi::c_int * y * FDEC_STRIDE + 4 as ::core::ffi::c_int * x) as isize,
            ),
            FDEC_STRIDE as intptr_t,
            src0,
            i_stride0,
            src1,
            i_stride1,
            weight,
        );
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut v_shift: ::core::ffi::c_int = (*h).mb.chroma_v_shift;
        if v_shift & (*h).mb.b_interlaced & i_ref0 != 0 {
            mvy0 += ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int;
        }
        if v_shift & (*h).mb.b_interlaced & i_ref1 != 0 {
            mvy1 += ((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int;
        }
        (*h).mc.mc_chroma.expect("non-null function pointer")(
            tmp0.as_mut_ptr(),
            tmp0.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
            16 as intptr_t,
            (*h).mb.pic.p_fref[0 as ::core::ffi::c_int as usize][i_ref0 as usize]
                [4 as ::core::ffi::c_int as usize],
            (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
            mvx0,
            2 as ::core::ffi::c_int * mvy0 >> v_shift,
            2 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height >> v_shift,
        );
        (*h).mc.mc_chroma.expect("non-null function pointer")(
            tmp1.as_mut_ptr(),
            tmp1.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
            16 as intptr_t,
            (*h).mb.pic.p_fref[1 as ::core::ffi::c_int as usize][i_ref1 as usize]
                [4 as ::core::ffi::c_int as usize],
            (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
            mvx1,
            2 as ::core::ffi::c_int * mvy1 >> v_shift,
            2 as ::core::ffi::c_int * width,
            4 as ::core::ffi::c_int * height >> v_shift,
        );
        let mut chromapix: ::core::ffi::c_int =
            (*h).luma2chroma_pixel[i_mode as usize] as ::core::ffi::c_int;
        let mut offset: ::core::ffi::c_int =
            (4 as ::core::ffi::c_int * FDEC_STRIDE >> v_shift) * y + 2 as ::core::ffi::c_int * x;
        (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .offset(offset as isize),
            FDEC_STRIDE as intptr_t,
            tmp0.as_mut_ptr(),
            16 as intptr_t,
            tmp1.as_mut_ptr(),
            16 as intptr_t,
            weight,
        );
        (*h).mc.avg[chromapix as usize].expect("non-null function pointer")(
            &mut *(*(*h)
                .mb
                .pic
                .p_fdec
                .as_mut_ptr()
                .offset(2 as ::core::ffi::c_int as isize))
            .offset(offset as isize),
            FDEC_STRIDE as intptr_t,
            tmp0.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
            16 as intptr_t,
            tmp1.as_mut_ptr().offset(8 as ::core::ffi::c_int as isize),
            16 as intptr_t,
            weight,
        );
    }
}
#[no_mangle]
#[c2rust::src_loc = "158:1"]
pub unsafe extern "C" fn x264_10_mb_mc_8x8(mut h: *mut x264_t, mut i8: ::core::ffi::c_int) {
    let mut x: ::core::ffi::c_int = 2 as ::core::ffi::c_int * (i8 & 1 as ::core::ffi::c_int);
    let mut y: ::core::ffi::c_int = 2 as ::core::ffi::c_int * (i8 >> 1 as ::core::ffi::c_int);
    if (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int {
        match (*h).mb.i_sub_partition[i8 as usize] as ::core::ffi::c_int {
            3 => {
                mb_mc_0xywh(h, x, y, 2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int);
            }
            1 => {
                mb_mc_0xywh(
                    h,
                    x,
                    y + 0 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x,
                    y + 1 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            }
            2 => {
                mb_mc_0xywh(
                    h,
                    x + 0 as ::core::ffi::c_int,
                    y,
                    1 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 1 as ::core::ffi::c_int,
                    y,
                    1 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
            }
            0 => {
                mb_mc_0xywh(
                    h,
                    x + 0 as ::core::ffi::c_int,
                    y + 0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 1 as ::core::ffi::c_int,
                    y + 0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 0 as ::core::ffi::c_int,
                    y + 1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
                mb_mc_0xywh(
                    h,
                    x + 1 as ::core::ffi::c_int,
                    y + 1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            }
            _ => {}
        }
    } else {
        let mut scan8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            + x
            + 8 as ::core::ffi::c_int * y;
        if (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize][scan8 as usize]
            as ::core::ffi::c_int
            >= 0 as ::core::ffi::c_int
        {
            if (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize][scan8 as usize]
                as ::core::ffi::c_int
                >= 0 as ::core::ffi::c_int
            {
                mb_mc_01xywh(h, x, y, 2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int);
            } else {
                mb_mc_0xywh(h, x, y, 2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int);
            }
        } else {
            mb_mc_1xywh(h, x, y, 2 as ::core::ffi::c_int, 2 as ::core::ffi::c_int);
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "200:1"]
pub unsafe extern "C" fn x264_10_mb_mc(mut h: *mut x264_t) {
    if (*h).mb.i_partition == D_8x8 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            x264_10_mb_mc_8x8(h, i);
            i += 1;
        }
    } else {
        let mut ref0a: ::core::ffi::c_int = (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
            as ::core::ffi::c_int;
        let mut ref0b: ::core::ffi::c_int = (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
            as ::core::ffi::c_int;
        let mut ref1a: ::core::ffi::c_int = (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
            [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
            as ::core::ffi::c_int;
        let mut ref1b: ::core::ffi::c_int = (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
            [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
            as ::core::ffi::c_int;
        if (*h).mb.i_partition == D_16x16 as ::core::ffi::c_int {
            if ref0a >= 0 as ::core::ffi::c_int {
                if ref1a >= 0 as ::core::ffi::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                );
            }
        } else if (*h).mb.i_partition == D_16x8 as ::core::ffi::c_int {
            if ref0a >= 0 as ::core::ffi::c_int {
                if ref1a >= 0 as ::core::ffi::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
            }
            if ref0b >= 0 as ::core::ffi::c_int {
                if ref1b >= 0 as ::core::ffi::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                );
            }
        } else if (*h).mb.i_partition == D_8x16 as ::core::ffi::c_int {
            if ref0a >= 0 as ::core::ffi::c_int {
                if ref1a >= 0 as ::core::ffi::c_int {
                    mb_mc_01xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                );
            }
            if ref0b >= 0 as ::core::ffi::c_int {
                if ref1b >= 0 as ::core::ffi::c_int {
                    mb_mc_01xywh(
                        h,
                        2 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                } else {
                    mb_mc_0xywh(
                        h,
                        2 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                    );
                }
            } else {
                mb_mc_1xywh(
                    h,
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                );
            }
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "248:1"]
pub unsafe extern "C" fn x264_10_macroblock_cache_allocate(
    mut h: *mut x264_t,
) -> ::core::ffi::c_int {
    let mut i_mb_count: ::core::ffi::c_int = (*h).mb.i_mb_count;
    (*h).mb.i_mb_stride = (*h).mb.i_mb_width;
    (*h).mb.i_b8_stride = (*h).mb.i_mb_width * 2 as ::core::ffi::c_int;
    (*h).mb.i_b4_stride = (*h).mb.i_mb_width * 4 as ::core::ffi::c_int;
    (*h).mb.b_interlaced = (*h).param.b_interlaced;
    let mut prealloc_idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut prealloc_size: int64_t = 0 as int64_t;
    let mut preallocs: [*mut *mut uint8_t; 1024] = [0 as *mut *mut uint8_t; 1024];
    (*h).mb.qp = prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut int8_t;
    let fresh0 = prealloc_idx;
    prealloc_idx = prealloc_idx + 1;
    preallocs[fresh0 as usize] = &mut (*h).mb.qp as *mut *mut int8_t as *mut *mut uint8_t;
    prealloc_size += (i_mb_count as usize).wrapping_mul(::core::mem::size_of::<int8_t>() as usize)
        as int64_t
        + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
        & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
    (*h).mb.cbp = prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut int16_t;
    let fresh1 = prealloc_idx;
    prealloc_idx = prealloc_idx + 1;
    preallocs[fresh1 as usize] = &mut (*h).mb.cbp as *mut *mut int16_t as *mut *mut uint8_t;
    prealloc_size += (i_mb_count as usize).wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
        as int64_t
        + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
        & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
    (*h).mb.mb_transform_size =
        prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut int8_t;
    let fresh2 = prealloc_idx;
    prealloc_idx = prealloc_idx + 1;
    preallocs[fresh2 as usize] =
        &mut (*h).mb.mb_transform_size as *mut *mut int8_t as *mut *mut uint8_t;
    prealloc_size += (i_mb_count as usize).wrapping_mul(::core::mem::size_of::<int8_t>() as usize)
        as int64_t
        + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
        & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
    (*h).mb.slice_table = prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut int32_t;
    let fresh3 = prealloc_idx;
    prealloc_idx = prealloc_idx + 1;
    preallocs[fresh3 as usize] = &mut (*h).mb.slice_table as *mut *mut int32_t as *mut *mut uint8_t;
    prealloc_size += (i_mb_count as usize).wrapping_mul(::core::mem::size_of::<int32_t>() as usize)
        as int64_t
        + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
        & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
    (*h).mb.intra4x4_pred_mode =
        prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut [int8_t; 8];
    let fresh4 = prealloc_idx;
    prealloc_idx = prealloc_idx + 1;
    preallocs[fresh4 as usize] =
        &mut (*h).mb.intra4x4_pred_mode as *mut *mut [int8_t; 8] as *mut *mut uint8_t;
    prealloc_size += ((i_mb_count * 8 as ::core::ffi::c_int) as usize)
        .wrapping_mul(::core::mem::size_of::<int8_t>() as usize) as int64_t
        + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
        & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
    (*h).mb.non_zero_count =
        prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut [uint8_t; 48];
    let fresh5 = prealloc_idx;
    prealloc_idx = prealloc_idx + 1;
    preallocs[fresh5 as usize] =
        &mut (*h).mb.non_zero_count as *mut *mut [uint8_t; 48] as *mut *mut uint8_t;
    prealloc_size += ((i_mb_count * 48 as ::core::ffi::c_int) as usize)
        .wrapping_mul(::core::mem::size_of::<uint8_t>() as usize) as int64_t
        + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
        & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
    if (*h).param.b_cabac != 0 {
        (*h).mb.skipbp = prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut int8_t;
        let fresh6 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[fresh6 as usize] = &mut (*h).mb.skipbp as *mut *mut int8_t as *mut *mut uint8_t;
        prealloc_size += (i_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<int8_t>() as usize)
            as int64_t
            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
        (*h).mb.chroma_pred_mode =
            prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut int8_t;
        let fresh7 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[fresh7 as usize] =
            &mut (*h).mb.chroma_pred_mode as *mut *mut int8_t as *mut *mut uint8_t;
        prealloc_size += (i_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<int8_t>() as usize)
            as int64_t
            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
        (*h).mb.mvd[0 as ::core::ffi::c_int as usize] =
            prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut [[uint8_t; 2]; 8];
        let fresh8 = prealloc_idx;
        prealloc_idx = prealloc_idx + 1;
        preallocs[fresh8 as usize] = &mut *(*h)
            .mb
            .mvd
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut *mut [[uint8_t; 2]; 8]
            as *mut *mut uint8_t;
        prealloc_size += (i_mb_count as usize)
            .wrapping_mul(::core::mem::size_of::<[[uint8_t; 2]; 8]>() as usize)
            as int64_t
            + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
            & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
        if (*h).param.i_bframe != 0 {
            (*h).mb.mvd[1 as ::core::ffi::c_int as usize] =
                prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut [[uint8_t; 2]; 8];
            let fresh9 = prealloc_idx;
            prealloc_idx = prealloc_idx + 1;
            preallocs[fresh9 as usize] = &mut *(*h)
                .mb
                .mvd
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize)
                as *mut *mut [[uint8_t; 2]; 8]
                as *mut *mut uint8_t;
            prealloc_size += (i_mb_count as usize)
                .wrapping_mul(::core::mem::size_of::<[[uint8_t; 2]; 8]>() as usize)
                as int64_t
                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
        }
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        let mut i_refs: ::core::ffi::c_int = (if (16 as ::core::ffi::c_int)
            < (if i != 0 {
                1 as ::core::ffi::c_int + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
            } else {
                (*h).param.i_frame_reference
            }) {
            16 as ::core::ffi::c_int
        } else {
            (if i != 0 {
                1 as ::core::ffi::c_int + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
            } else {
                (*h).param.i_frame_reference
            })
        }) << (*h).param.b_interlaced;
        if (*h).param.analyse.i_weighted_pred == X264_WEIGHTP_SMART {
            i_refs = if (16 as ::core::ffi::c_int)
                < i_refs
                    + 1 as ::core::ffi::c_int
                    + (10 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) as ::core::ffi::c_int
            {
                16 as ::core::ffi::c_int
            } else {
                i_refs
                    + 1 as ::core::ffi::c_int
                    + (10 as ::core::ffi::c_int == 8 as ::core::ffi::c_int) as ::core::ffi::c_int
            };
        }
        let mut j: ::core::ffi::c_int = (i == 0) as ::core::ffi::c_int;
        while j < i_refs {
            (*h).mb.mvr[i as usize][j as usize] =
                prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut [int16_t; 2];
            let fresh10 = prealloc_idx;
            prealloc_idx = prealloc_idx + 1;
            preallocs[fresh10 as usize] = &mut *(*(*h).mb.mvr.as_mut_ptr().offset(i as isize))
                .as_mut_ptr()
                .offset(j as isize)
                as *mut *mut [int16_t; 2]
                as *mut *mut uint8_t;
            prealloc_size += ((2 as ::core::ffi::c_int * (i_mb_count + 1 as ::core::ffi::c_int))
                as usize)
                .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                as int64_t
                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
            j += 1;
        }
        i += 1;
    }
    if (*h).param.analyse.i_weighted_pred != 0 {
        let mut i_padv: ::core::ffi::c_int = PADV << (*h).param.b_interlaced;
        let mut luma_plane_size: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut numweightbuf: ::core::ffi::c_int = 0;
        if (*h).param.analyse.i_weighted_pred == X264_WEIGHTP_FAKE {
            if (*h).param.i_sync_lookahead == 0 || h == (*h).thread[(*h).param.i_threads as usize] {
                luma_plane_size = (*(*h).fdec).i_stride_lowres
                    * ((*h).mb.i_mb_height * 8 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * i_padv);
                numweightbuf = 1 as ::core::ffi::c_int;
            } else {
                numweightbuf = 0 as ::core::ffi::c_int;
            }
        } else {
            luma_plane_size = (*(*h).fdec).i_stride[0 as ::core::ffi::c_int as usize]
                * ((*h).mb.i_mb_height
                    * ((16 as ::core::ffi::c_int)
                        << ((*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                            == CHROMA_422 as ::core::ffi::c_int)
                            as ::core::ffi::c_int)
                    + 2 as ::core::ffi::c_int * i_padv);
            if (*h).param.analyse.i_weighted_pred == X264_WEIGHTP_SMART {
                numweightbuf = 1 as ::core::ffi::c_int
                    + (BIT_DEPTH == 8 as ::core::ffi::c_int) as ::core::ffi::c_int;
            } else {
                numweightbuf = 1 as ::core::ffi::c_int;
            }
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < numweightbuf {
            (*h).mb.p_weight_buf[i_0 as usize] =
                prealloc_size as intptr_t as *mut ::core::ffi::c_void as *mut pixel;
            let fresh11 = prealloc_idx;
            prealloc_idx = prealloc_idx + 1;
            preallocs[fresh11 as usize] =
                &mut *(*h).mb.p_weight_buf.as_mut_ptr().offset(i_0 as isize) as *mut *mut pixel
                    as *mut *mut uint8_t;
            prealloc_size += (luma_plane_size
                * ::core::mem::size_of::<pixel>() as ::core::ffi::c_int)
                as int64_t
                + (64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t
                & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as int64_t;
            i_0 += 1;
        }
    }
    (*h).mb.base = x264_malloc(prealloc_size) as *mut uint8_t;
    if (*h).mb.base.is_null() {
        return -(1 as ::core::ffi::c_int);
    } else {
        loop {
            let fresh12 = prealloc_idx;
            prealloc_idx = prealloc_idx - 1;
            if !(fresh12 != 0) {
                break;
            }
            *preallocs[prealloc_idx as usize] = (*preallocs[prealloc_idx as usize] as intptr_t
                + (*h).mb.base as intptr_t)
                as *mut uint8_t;
        }
        memset(
            (*h).mb.slice_table as *mut ::core::ffi::c_void,
            -(1 as ::core::ffi::c_int),
            (i_mb_count as size_t).wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        );
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1 < 2 as ::core::ffi::c_int {
            let mut i_refs_0: ::core::ffi::c_int = (if (16 as ::core::ffi::c_int)
                < (if i_1 != 0 {
                    1 as ::core::ffi::c_int
                        + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
                } else {
                    (*h).param.i_frame_reference
                }) {
                16 as ::core::ffi::c_int
            } else {
                (if i_1 != 0 {
                    1 as ::core::ffi::c_int
                        + ((*h).param.i_bframe_pyramid != 0) as ::core::ffi::c_int
                } else {
                    (*h).param.i_frame_reference
                })
            }) << (*h).param.b_interlaced;
            if (*h).param.analyse.i_weighted_pred == X264_WEIGHTP_SMART {
                i_refs_0 = if (16 as ::core::ffi::c_int)
                    < i_refs_0
                        + 1 as ::core::ffi::c_int
                        + (10 as ::core::ffi::c_int == 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                {
                    16 as ::core::ffi::c_int
                } else {
                    i_refs_0
                        + 1 as ::core::ffi::c_int
                        + (10 as ::core::ffi::c_int == 8 as ::core::ffi::c_int)
                            as ::core::ffi::c_int
                };
            }
            let mut j_0: ::core::ffi::c_int = (i_1 == 0) as ::core::ffi::c_int;
            while j_0 < i_refs_0 {
                (*((*(*(*(*h).mb.mvr.as_mut_ptr().offset(i_1 as isize))
                    .as_mut_ptr()
                    .offset(j_0 as isize))
                .offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = 0 as uint32_t;
                (*h).mb.mvr[i_1 as usize][j_0 as usize] =
                    (*h).mb.mvr[i_1 as usize][j_0 as usize].offset(1);
                j_0 += 1;
            }
            i_1 += 1;
        }
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
#[c2rust::src_loc = "348:1"]
pub unsafe extern "C" fn x264_10_macroblock_cache_free(mut h: *mut x264_t) {
    x264_free((*h).mb.base as *mut ::core::ffi::c_void);
}
#[no_mangle]
#[c2rust::src_loc = "353:1"]
pub unsafe extern "C" fn x264_10_macroblock_thread_allocate(
    mut h: *mut x264_t,
    mut b_lookahead: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut scratch_size: ::core::ffi::c_int = 0;
    let mut buf_mbtree: ::core::ffi::c_int = 0;
    let mut buf_lookahead_threads: ::core::ffi::c_int = 0;
    let mut buf_mbtree2: ::core::ffi::c_int = 0;
    let mut current_block: u64;
    if b_lookahead == 0 {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        's_5: loop {
            if !(i
                < (if (*h).param.b_interlaced != 0 {
                    5 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                }))
            {
                current_block = 8515828400728868193;
                break;
            }
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j
                < (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                    == CHROMA_444 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                })
            {
                (*h).intra_border_backup[i as usize][j as usize] = x264_malloc(
                    (((*(*h).sps.as_mut_ptr()).i_mb_width * 16 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int)
                        * ::core::mem::size_of::<pixel>() as ::core::ffi::c_int)
                        as int64_t,
                ) as *mut pixel;
                if (*h).intra_border_backup[i as usize][j as usize].is_null() {
                    current_block = 11409641321532490549;
                    break 's_5;
                }
                (*h).intra_border_backup[i as usize][j as usize] = (*h).intra_border_backup
                    [i as usize][j as usize]
                    .offset(16 as ::core::ffi::c_int as isize);
                j += 1;
            }
            i += 1;
        }
        match current_block {
            11409641321532490549 => {}
            _ => {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                loop {
                    if !(i_0 <= (*h).param.b_interlaced) {
                        current_block = 5783071609795492627;
                        break;
                    }
                    if (*h).param.b_sliced_threads != 0 {
                        if h == (*h).thread[0 as ::core::ffi::c_int as usize] && i_0 == 0 {
                            (*h).deblock_strength[0 as ::core::ffi::c_int as usize] = x264_malloc(
                                (::core::mem::size_of::<[[[uint8_t; 4]; 8]; 2]>() as usize)
                                    .wrapping_mul((*h).mb.i_mb_count as usize)
                                    as int64_t,
                            )
                                as *mut [[[uint8_t; 4]; 8]; 2];
                            if (*h).deblock_strength[0 as ::core::ffi::c_int as usize].is_null() {
                                current_block = 11409641321532490549;
                                break;
                            }
                        } else {
                            (*h).deblock_strength[i_0 as usize] =
                                (*(*h).thread[0 as ::core::ffi::c_int as usize]).deblock_strength
                                    [0 as ::core::ffi::c_int as usize];
                        }
                    } else {
                        (*h).deblock_strength[i_0 as usize] = x264_malloc(
                            (::core::mem::size_of::<[[[uint8_t; 4]; 8]; 2]>() as usize)
                                .wrapping_mul((*h).mb.i_mb_width as usize)
                                as int64_t,
                        )
                            as *mut [[[uint8_t; 4]; 8]; 2];
                        if (*h).deblock_strength[i_0 as usize].is_null() {
                            current_block = 11409641321532490549;
                            break;
                        }
                    }
                    (*h).deblock_strength[1 as ::core::ffi::c_int as usize] =
                        (*h).deblock_strength[i_0 as usize];
                    i_0 += 1;
                }
            }
        }
    } else {
        current_block = 5783071609795492627;
    }
    match current_block {
        5783071609795492627 => {
            scratch_size = 0 as ::core::ffi::c_int;
            if b_lookahead == 0 {
                let mut buf_hpel: ::core::ffi::c_int =
                    (((*(*(*h).thread[0 as ::core::ffi::c_int as usize]).fdec).i_width
                        [0 as ::core::ffi::c_int as usize]
                        + 48 as ::core::ffi::c_int
                        + 32 as ::core::ffi::c_int) as usize)
                        .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                        as ::core::ffi::c_int;
                let mut buf_ssim: ::core::ffi::c_int = (((*h).param.analyse.b_ssim
                    * 8 as ::core::ffi::c_int
                    * ((*h).param.i_width / 4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int))
                    as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    as ::core::ffi::c_int;
                let mut me_range: ::core::ffi::c_int =
                    if (*h).param.analyse.i_me_range < (*h).param.analyse.i_mv_range {
                        (*h).param.analyse.i_me_range
                    } else {
                        (*h).param.analyse.i_mv_range
                    };
                let mut buf_tesa: ::core::ffi::c_int = (((*h).param.analyse.i_me_method
                    >= X264_ME_ESA)
                    as ::core::ffi::c_int
                    as usize)
                    .wrapping_mul(
                        ((me_range * 2 as ::core::ffi::c_int + 24 as ::core::ffi::c_int) as usize)
                            .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                            .wrapping_add(
                                (((me_range + 4 as ::core::ffi::c_int)
                                    * (me_range + 1 as ::core::ffi::c_int)
                                    * 4 as ::core::ffi::c_int)
                                    as usize)
                                    .wrapping_mul(::core::mem::size_of::<mvsad_t>() as usize),
                            ),
                    ) as ::core::ffi::c_int;
                scratch_size = if buf_hpel
                    > (if buf_ssim > buf_tesa {
                        buf_ssim
                    } else {
                        buf_tesa
                    }) {
                    buf_hpel
                } else if buf_ssim > buf_tesa {
                    buf_ssim
                } else {
                    buf_tesa
                };
            }
            buf_mbtree = ((*h).param.rc.b_mb_tree as usize).wrapping_mul(
                ((*h).mb.i_mb_width as usize)
                    .wrapping_mul(::core::mem::size_of::<int16_t>() as usize)
                    .wrapping_add((64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize)
                    & !(64 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize,
            ) as ::core::ffi::c_int;
            scratch_size = if scratch_size > buf_mbtree {
                scratch_size
            } else {
                buf_mbtree
            };
            if scratch_size != 0 {
                (*h).scratch_buffer = x264_malloc(scratch_size as int64_t);
                if (*h).scratch_buffer.is_null() {
                    current_block = 11409641321532490549;
                } else {
                    current_block = 2891135413264362348;
                }
            } else {
                (*h).scratch_buffer = NULL;
                current_block = 2891135413264362348;
            }
            match current_block {
                11409641321532490549 => {}
                _ => {
                    buf_lookahead_threads =
                        (((*h).mb.i_mb_height
                            + (4 as ::core::ffi::c_int + 32 as ::core::ffi::c_int)
                                * (*h).param.i_lookahead_threads) as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                            .wrapping_mul(2 as usize) as ::core::ffi::c_int;
                    buf_mbtree2 = buf_mbtree * 12 as ::core::ffi::c_int;
                    scratch_size = if buf_lookahead_threads > buf_mbtree2 {
                        buf_lookahead_threads
                    } else {
                        buf_mbtree2
                    };
                    (*h).scratch_buffer2 = x264_malloc(scratch_size as int64_t);
                    if !(*h).scratch_buffer2.is_null() {
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "408:1"]
pub unsafe extern "C" fn x264_10_macroblock_thread_free(
    mut h: *mut x264_t,
    mut b_lookahead: ::core::ffi::c_int,
) {
    if b_lookahead == 0 {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i <= (*h).param.b_interlaced {
            if (*h).param.b_sliced_threads == 0
                || h == (*h).thread[0 as ::core::ffi::c_int as usize] && i == 0
            {
                x264_free((*h).deblock_strength[i as usize] as *mut ::core::ffi::c_void);
            }
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0
            < (if (*h).param.b_interlaced != 0 {
                5 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            })
        {
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j
                < (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                    == CHROMA_444 as ::core::ffi::c_int
                {
                    3 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                })
            {
                x264_free(
                    (*h).intra_border_backup[i_0 as usize][j as usize]
                        .offset(-(16 as ::core::ffi::c_int as isize))
                        as *mut ::core::ffi::c_void,
                );
                j += 1;
            }
            i_0 += 1;
        }
    }
    x264_free((*h).scratch_buffer);
    x264_free((*h).scratch_buffer2);
}
#[no_mangle]
#[c2rust::src_loc = "423:1"]
pub unsafe extern "C" fn x264_10_macroblock_slice_init(mut h: *mut x264_t) {
    (*h).mb.mv[0 as ::core::ffi::c_int as usize] =
        (*(*h).fdec).mv[0 as ::core::ffi::c_int as usize];
    (*h).mb.mv[1 as ::core::ffi::c_int as usize] =
        (*(*h).fdec).mv[1 as ::core::ffi::c_int as usize];
    (*h).mb.mvr[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] =
        (*(*h).fdec).mv16x16;
    (*h).mb.ref_0[0 as ::core::ffi::c_int as usize] =
        (*(*h).fdec).ref_0[0 as ::core::ffi::c_int as usize];
    (*h).mb.ref_0[1 as ::core::ffi::c_int as usize] =
        (*(*h).fdec).ref_0[1 as ::core::ffi::c_int as usize];
    (*h).mb.type_0 = (*(*h).fdec).mb_type;
    (*h).mb.partition = (*(*h).fdec).mb_partition;
    (*h).mb.field = (*(*h).fdec).field;
    (*(*h).fdec).i_ref[0 as ::core::ffi::c_int as usize] =
        (*h).i_ref[0 as ::core::ffi::c_int as usize];
    (*(*h).fdec).i_ref[1 as ::core::ffi::c_int as usize] =
        (*h).i_ref[1 as ::core::ffi::c_int as usize];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
        (*(*h).fdec).ref_poc[0 as ::core::ffi::c_int as usize][i as usize] =
            (*(*h).fref[0 as ::core::ffi::c_int as usize][i as usize]).i_poc;
        i += 1;
    }
    if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*h).i_ref[1 as ::core::ffi::c_int as usize] {
            (*(*h).fdec).ref_poc[1 as ::core::ffi::c_int as usize][i_0 as usize] =
                (*(*h).fref[1 as ::core::ffi::c_int as usize][i_0 as usize]).i_poc;
            i_0 += 1;
        }
        (*h).mb.map_col_to_list0[(-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int) as usize] =
            -(1 as ::core::ffi::c_int) as int8_t;
        (*h).mb.map_col_to_list0[(-(2 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int) as usize] =
            -(2 as ::core::ffi::c_int) as int8_t;
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_1
            < (*(*h).fref[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize]).i_ref
                [0 as ::core::ffi::c_int as usize]
        {
            let mut poc: ::core::ffi::c_int = (*(*h).fref[1 as ::core::ffi::c_int as usize]
                [0 as ::core::ffi::c_int as usize])
                .ref_poc[0 as ::core::ffi::c_int as usize][i_1 as usize];
            (*h).mb.map_col_to_list0[(i_1 + 2 as ::core::ffi::c_int) as usize] =
                -(2 as ::core::ffi::c_int) as int8_t;
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < (*h).i_ref[0 as ::core::ffi::c_int as usize] {
                if (*(*h).fref[0 as ::core::ffi::c_int as usize][j as usize]).i_poc == poc {
                    (*h).mb.map_col_to_list0[(i_1 + 2 as ::core::ffi::c_int) as usize] =
                        j as int8_t;
                    break;
                } else {
                    j += 1;
                }
            }
            i_1 += 1;
        }
    } else if (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int {
        if (*h).sh.i_disable_deblocking_filter_idc != 1 as ::core::ffi::c_int
            && (*h).param.analyse.i_weighted_pred == X264_WEIGHTP_SMART
        {
            (*h).mb.deblock_ref_table
                [(-(2 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int) as usize] =
                -(2 as ::core::ffi::c_int) as int8_t;
            (*h).mb.deblock_ref_table
                [(-(1 as ::core::ffi::c_int) + 2 as ::core::ffi::c_int) as usize] =
                -(1 as ::core::ffi::c_int) as int8_t;
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2 < (*h).i_ref[0 as ::core::ffi::c_int as usize] << (*h).sh.b_mbaff {
                if (*h).mb.b_interlaced == 0 {
                    (*h).mb.deblock_ref_table[(i_2 + 2 as ::core::ffi::c_int) as usize] =
                        ((*(*h).fref[0 as ::core::ffi::c_int as usize][i_2 as usize]).i_frame_num
                            & 63 as ::core::ffi::c_int) as int8_t;
                } else {
                    (*h).mb.deblock_ref_table[(i_2 + 2 as ::core::ffi::c_int) as usize] =
                        ((((*(*h).fref[0 as ::core::ffi::c_int as usize]
                            [(i_2 >> 1 as ::core::ffi::c_int) as usize])
                            .i_frame_num
                            & 63 as ::core::ffi::c_int)
                            << 1 as ::core::ffi::c_int)
                            + (i_2 & 1 as ::core::ffi::c_int)) as int8_t;
                }
                i_2 += 1;
            }
        }
    }
    memset(
        (*h).mb.cache.ref_0.as_mut_ptr() as *mut ::core::ffi::c_void,
        -(2 as ::core::ffi::c_int),
        ::core::mem::size_of::<[[int8_t; 40]; 2]>() as size_t,
    );
    if (*h).i_ref[0 as ::core::ffi::c_int as usize] > 0 as ::core::ffi::c_int {
        let mut field: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while field <= (*h).sh.b_mbaff {
            let mut curpoc: ::core::ffi::c_int =
                (*(*h).fdec).i_poc + (*(*h).fdec).i_delta_poc[field as usize];
            let mut refpoc: ::core::ffi::c_int = (*(*h).fref[0 as ::core::ffi::c_int as usize]
                [0 as ::core::ffi::c_int as usize])
                .i_poc
                + (*(*h).fref[0 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize])
                    .i_delta_poc[field as usize];
            let mut delta: ::core::ffi::c_int = curpoc - refpoc;
            (*(*h).fdec).inv_ref_poc[field as usize] =
                ((256 as ::core::ffi::c_int + delta / 2 as ::core::ffi::c_int) / delta) as int16_t;
            field += 1;
        }
    }
    (*h).mb.i_neighbour4[14 as ::core::ffi::c_int as usize] = (MB_LEFT as ::core::ffi::c_int
        | MB_TOP as ::core::ffi::c_int
        | MB_TOPLEFT as ::core::ffi::c_int
        | MB_TOPRIGHT as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    (*h).mb.i_neighbour4[12 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour4[14 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour4[9 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour4[12 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour4[6 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour4[9 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour8[3 as ::core::ffi::c_int as usize] = (MB_LEFT as ::core::ffi::c_int
        | MB_TOP as ::core::ffi::c_int
        | MB_TOPLEFT as ::core::ffi::c_int)
        as ::core::ffi::c_uint;
    (*h).mb.i_neighbour4[15 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour8[3 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour4[13 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour4[15 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour4[11 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour4[13 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour4[7 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour4[11 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour4[3 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour4[7 as ::core::ffi::c_int as usize];
}
#[no_mangle]
#[c2rust::src_loc = "501:1"]
pub unsafe extern "C" fn x264_10_macroblock_thread_init(mut h: *mut x264_t) {
    (*h).mb.i_me_method = (*h).param.analyse.i_me_method;
    (*h).mb.i_subpel_refine = (*h).param.analyse.i_subpel_refine;
    if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int
        && ((*h).mb.i_subpel_refine == 6 as ::core::ffi::c_int
            || (*h).mb.i_subpel_refine == 8 as ::core::ffi::c_int)
    {
        (*h).mb.i_subpel_refine -= 1;
    }
    (*h).mb.b_chroma_me = ((*h).param.analyse.b_chroma_me != 0
        && ((*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int
            && (*h).mb.i_subpel_refine >= 5 as ::core::ffi::c_int
            || (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int
                && (*h).mb.i_subpel_refine >= 9 as ::core::ffi::c_int))
        as ::core::ffi::c_int;
    (*h).mb.b_dct_decimate = ((*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int
        || (*h).param.analyse.b_dct_decimate != 0
            && (*h).sh.i_type != SLICE_TYPE_I as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    (*h).mb.i_mb_prev_xy = -(1 as ::core::ffi::c_int);
    (*h).mb.pic.p_fenc[0 as ::core::ffi::c_int as usize] = (*h).mb.pic.fenc_buf.as_mut_ptr();
    (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize] = (*h)
        .mb
        .pic
        .fdec_buf
        .as_mut_ptr()
        .offset((2 as ::core::ffi::c_int * FDEC_STRIDE) as isize);
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        (*h).mb.pic.p_fenc[1 as ::core::ffi::c_int as usize] = (*h)
            .mb
            .pic
            .fenc_buf
            .as_mut_ptr()
            .offset((16 as ::core::ffi::c_int * FENC_STRIDE) as isize);
        (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize] = (*h)
            .mb
            .pic
            .fdec_buf
            .as_mut_ptr()
            .offset((20 as ::core::ffi::c_int * FDEC_STRIDE) as isize);
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
            (*h).mb.pic.p_fenc[2 as ::core::ffi::c_int as usize] = (*h)
                .mb
                .pic
                .fenc_buf
                .as_mut_ptr()
                .offset((32 as ::core::ffi::c_int * FENC_STRIDE) as isize);
            (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize] = (*h)
                .mb
                .pic
                .fdec_buf
                .as_mut_ptr()
                .offset((38 as ::core::ffi::c_int * FDEC_STRIDE) as isize);
        } else {
            (*h).mb.pic.p_fenc[2 as ::core::ffi::c_int as usize] = (*h)
                .mb
                .pic
                .fenc_buf
                .as_mut_ptr()
                .offset((16 as ::core::ffi::c_int * FENC_STRIDE) as isize)
                .offset(8 as ::core::ffi::c_int as isize);
            (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize] = (*h)
                .mb
                .pic
                .fdec_buf
                .as_mut_ptr()
                .offset((20 as ::core::ffi::c_int * FDEC_STRIDE) as isize)
                .offset(16 as ::core::ffi::c_int as isize);
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "551:1"]
pub unsafe extern "C" fn x264_10_prefetch_fenc(
    mut h: *mut x264_t,
    mut fenc: *mut x264_frame_t,
    mut i_mb_x: ::core::ffi::c_int,
    mut i_mb_y: ::core::ffi::c_int,
) {
    let mut stride_y: ::core::ffi::c_int = (*fenc).i_stride[0 as ::core::ffi::c_int as usize];
    let mut stride_uv: ::core::ffi::c_int = (*fenc).i_stride[1 as ::core::ffi::c_int as usize];
    let mut off_y: ::core::ffi::c_int =
        16 as ::core::ffi::c_int * i_mb_x + 16 as ::core::ffi::c_int * i_mb_y * stride_y;
    let mut off_uv: ::core::ffi::c_int = 16 as ::core::ffi::c_int * i_mb_x
        + (16 as ::core::ffi::c_int * i_mb_y * stride_uv >> (*h).mb.chroma_v_shift);
    (*h).mc.prefetch_fenc.expect("non-null function pointer")(
        (*fenc).plane[0 as ::core::ffi::c_int as usize].offset(off_y as isize),
        stride_y as intptr_t,
        if !(*fenc).plane[1 as ::core::ffi::c_int as usize].is_null() {
            (*fenc).plane[1 as ::core::ffi::c_int as usize].offset(off_uv as isize)
        } else {
            0 as *mut pixel
        },
        stride_uv as intptr_t,
        i_mb_x,
    );
}
#[no_mangle]
#[inline(never)]
#[c2rust::src_loc = "561:1"]
pub unsafe extern "C" fn x264_10_copy_column8(mut dst: *mut pixel, mut src: *mut pixel) {
    let mut i: ::core::ffi::c_int = -(4 as ::core::ffi::c_int);
    while i < 4 as ::core::ffi::c_int {
        *dst.offset((i * FDEC_STRIDE) as isize) = *src.offset((i * FDEC_STRIDE) as isize);
        i += 1;
    }
}
#[inline(always)]
#[c2rust::src_loc = "568:1"]
unsafe extern "C" fn macroblock_load_pic_pointers(
    mut h: *mut x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut i: ::core::ffi::c_int,
    mut b_chroma: ::core::ffi::c_int,
    mut b_mbaff: ::core::ffi::c_int,
) {
    let mut mb_interlaced: ::core::ffi::c_int =
        (b_mbaff != 0 && (*h).mb.b_interlaced != 0) as ::core::ffi::c_int;
    let mut height: ::core::ffi::c_int = if b_chroma != 0 {
        16 as ::core::ffi::c_int >> (*h).mb.chroma_v_shift
    } else {
        16 as ::core::ffi::c_int
    };
    let mut i_stride: ::core::ffi::c_int = (*(*h).fdec).i_stride[i as usize];
    let mut i_stride2: ::core::ffi::c_int = i_stride << mb_interlaced;
    let mut i_pix_offset: ::core::ffi::c_int = if mb_interlaced != 0 {
        16 as ::core::ffi::c_int * mb_x
            + height * (mb_y & !(1 as ::core::ffi::c_int)) * i_stride
            + (mb_y & 1 as ::core::ffi::c_int) * i_stride
    } else {
        16 as ::core::ffi::c_int * mb_x + height * mb_y * i_stride
    };
    let mut plane_fdec: *mut pixel = &mut *(*(*(*h).fdec).plane.as_mut_ptr().offset(i as isize))
        .offset(i_pix_offset as isize) as *mut pixel;
    let mut fdec_idx: ::core::ffi::c_int = if b_mbaff != 0 {
        if mb_interlaced != 0 {
            3 as ::core::ffi::c_int + (mb_y & 1 as ::core::ffi::c_int)
        } else if mb_y & 1 as ::core::ffi::c_int != 0 {
            2 as ::core::ffi::c_int
        } else {
            4 as ::core::ffi::c_int
        }
    } else {
        (mb_y & 1 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
    };
    let mut intra_fdec: *mut pixel = &mut *(*(*(*h)
        .intra_border_backup
        .as_mut_ptr()
        .offset(fdec_idx as isize))
    .as_mut_ptr()
    .offset(i as isize))
    .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
        as *mut pixel;
    let mut ref_pix_offset: [::core::ffi::c_int; 2] = [i_pix_offset, i_pix_offset];
    if mb_interlaced != 0 {
        ref_pix_offset[1 as ::core::ffi::c_int as usize] += (1 as ::core::ffi::c_int
            - 2 as ::core::ffi::c_int * (mb_y & 1 as ::core::ffi::c_int))
            * i_stride;
    }
    (*h).mb.pic.i_stride[i as usize] = i_stride2;
    (*h).mb.pic.p_fenc_plane[i as usize] =
        &mut *(*(*(*h).fenc).plane.as_mut_ptr().offset(i as isize)).offset(i_pix_offset as isize)
            as *mut pixel;
    if b_chroma != 0 {
        (*h).mc
            .load_deinterleave_chroma_fenc
            .expect("non-null function pointer")(
            (*h).mb.pic.p_fenc[1 as ::core::ffi::c_int as usize],
            (*h).mb.pic.p_fenc_plane[1 as ::core::ffi::c_int as usize],
            i_stride2 as intptr_t,
            height,
        );
        memcpy(
            (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(-(FDEC_STRIDE as isize))
                as *mut ::core::ffi::c_void,
            intra_fdec as *const ::core::ffi::c_void,
            (8 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
        memcpy(
            (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(-(FDEC_STRIDE as isize))
                as *mut ::core::ffi::c_void,
            intra_fdec.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (8 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
        *(*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
            .offset((-FDEC_STRIDE - 1 as ::core::ffi::c_int) as isize) =
            *intra_fdec.offset((-(1 as ::core::ffi::c_int) - 8 as ::core::ffi::c_int) as isize);
        *(*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
            .offset((-FDEC_STRIDE - 1 as ::core::ffi::c_int) as isize) =
            *intra_fdec.offset(-(1 as ::core::ffi::c_int) as isize);
    } else {
        (*h).mc.copy[PIXEL_16x16 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            (*h).mb.pic.p_fenc[i as usize],
            FENC_STRIDE as intptr_t,
            (*h).mb.pic.p_fenc_plane[i as usize],
            i_stride2 as intptr_t,
            16 as ::core::ffi::c_int,
        );
        memcpy(
            (*h).mb.pic.p_fdec[i as usize].offset(-(FDEC_STRIDE as isize))
                as *mut ::core::ffi::c_void,
            intra_fdec as *const ::core::ffi::c_void,
            (24 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
        *(*h).mb.pic.p_fdec[i as usize].offset((-FDEC_STRIDE - 1 as ::core::ffi::c_int) as isize) =
            *intra_fdec.offset(-(1 as ::core::ffi::c_int) as isize);
    }
    if b_mbaff != 0 || (*h).mb.b_reencode_mb != 0 {
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < height {
            if b_chroma != 0 {
                *(*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                    .offset((-(1 as ::core::ffi::c_int) + j * FDEC_STRIDE) as isize) =
                    *plane_fdec.offset((-(2 as ::core::ffi::c_int) + j * i_stride2) as isize);
                *(*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                    .offset((-(1 as ::core::ffi::c_int) + j * FDEC_STRIDE) as isize) =
                    *plane_fdec.offset((-(1 as ::core::ffi::c_int) + j * i_stride2) as isize);
            } else {
                *(*h).mb.pic.p_fdec[i as usize]
                    .offset((-(1 as ::core::ffi::c_int) + j * FDEC_STRIDE) as isize) =
                    *plane_fdec.offset((-(1 as ::core::ffi::c_int) + j * i_stride2) as isize);
            }
            j += 1;
        }
    }
    let mut plane_src: *mut pixel = 0 as *mut pixel;
    let mut filtered_src: *mut *mut pixel = 0 as *mut *mut pixel;
    let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j_0 < (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] {
        if mb_interlaced != 0 {
            plane_src = (*(*h).fref[0 as ::core::ffi::c_int as usize]
                [(j_0 >> 1 as ::core::ffi::c_int) as usize])
                .plane_fld[i as usize];
            filtered_src = (*(**(*(*h)
                .fref
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset((j_0 >> 1 as ::core::ffi::c_int) as isize))
            .filtered_fld
            .as_mut_ptr()
            .offset(i as isize))
            .as_mut_ptr();
        } else {
            plane_src =
                (*(*h).fref[0 as ::core::ffi::c_int as usize][j_0 as usize]).plane[i as usize];
            filtered_src = (*(**(*(*h)
                .fref
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(j_0 as isize))
            .filtered
            .as_mut_ptr()
            .offset(i as isize))
            .as_mut_ptr();
        }
        (*h).mb.pic.p_fref[0 as ::core::ffi::c_int as usize][j_0 as usize]
            [(i * 4 as ::core::ffi::c_int) as usize] =
            plane_src.offset(ref_pix_offset[(j_0 & 1 as ::core::ffi::c_int) as usize] as isize);
        if b_chroma == 0 {
            if (*h).param.analyse.i_subpel_refine != 0 {
                let mut k: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while k < 4 as ::core::ffi::c_int {
                    (*h).mb.pic.p_fref[0 as ::core::ffi::c_int as usize][j_0 as usize]
                        [(i * 4 as ::core::ffi::c_int + k) as usize] = (*filtered_src
                        .offset(k as isize))
                    .offset(ref_pix_offset[(j_0 & 1 as ::core::ffi::c_int) as usize] as isize);
                    k += 1;
                }
            }
            if i == 0 {
                if !(*h).sh.weight[j_0 as usize][0 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()
                {
                    (*h).mb.pic.p_fref_w[j_0 as usize] = &mut *(*(*(*h).fenc)
                        .weighted
                        .as_mut_ptr()
                        .offset((j_0 >> mb_interlaced) as isize))
                    .offset(
                        *ref_pix_offset
                            .as_mut_ptr()
                            .offset((j_0 & 1 as ::core::ffi::c_int) as isize)
                            as isize,
                    ) as *mut pixel;
                } else {
                    (*h).mb.pic.p_fref_w[j_0 as usize] = (*h).mb.pic.p_fref
                        [0 as ::core::ffi::c_int as usize][j_0 as usize]
                        [0 as ::core::ffi::c_int as usize];
                }
            }
        }
        j_0 += 1;
    }
    if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
        let mut j_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j_1 < (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] {
            if mb_interlaced != 0 {
                plane_src = (*(*h).fref[1 as ::core::ffi::c_int as usize]
                    [(j_1 >> 1 as ::core::ffi::c_int) as usize])
                    .plane_fld[i as usize];
                filtered_src = (*(**(*(*h)
                    .fref
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset((j_1 >> 1 as ::core::ffi::c_int) as isize))
                .filtered_fld
                .as_mut_ptr()
                .offset(i as isize))
                .as_mut_ptr();
            } else {
                plane_src =
                    (*(*h).fref[1 as ::core::ffi::c_int as usize][j_1 as usize]).plane[i as usize];
                filtered_src = (*(**(*(*h)
                    .fref
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(j_1 as isize))
                .filtered
                .as_mut_ptr()
                .offset(i as isize))
                .as_mut_ptr();
            }
            (*h).mb.pic.p_fref[1 as ::core::ffi::c_int as usize][j_1 as usize]
                [(i * 4 as ::core::ffi::c_int) as usize] =
                plane_src.offset(ref_pix_offset[(j_1 & 1 as ::core::ffi::c_int) as usize] as isize);
            if b_chroma == 0 && (*h).param.analyse.i_subpel_refine != 0 {
                let mut k_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while k_0 < 4 as ::core::ffi::c_int {
                    (*h).mb.pic.p_fref[1 as ::core::ffi::c_int as usize][j_1 as usize]
                        [(i * 4 as ::core::ffi::c_int + k_0) as usize] = (*filtered_src
                        .offset(k_0 as isize))
                    .offset(ref_pix_offset[(j_1 & 1 as ::core::ffi::c_int) as usize] as isize);
                    k_0 += 1;
                }
            }
            j_1 += 1;
        }
    }
}
#[c2rust::src_loc = "662:32"]
static mut left_indices: [x264_left_table_t; 4] = [
    {
        let mut init = x264_left_table_t {
            intra: [
                4 as ::core::ffi::c_int as uint8_t,
                4 as ::core::ffi::c_int as uint8_t,
                5 as ::core::ffi::c_int as uint8_t,
                5 as ::core::ffi::c_int as uint8_t,
            ],
            nnz: [
                3 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
                7 as ::core::ffi::c_int as uint8_t,
                7 as ::core::ffi::c_int as uint8_t,
            ],
            nnz_chroma: [
                (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
                (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
                (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
                (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
            ],
            mv: [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
            ref_0: [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = x264_left_table_t {
            intra: [
                6 as ::core::ffi::c_int as uint8_t,
                6 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
            ],
            nnz: [
                11 as ::core::ffi::c_int as uint8_t,
                11 as ::core::ffi::c_int as uint8_t,
                15 as ::core::ffi::c_int as uint8_t,
                15 as ::core::ffi::c_int as uint8_t,
            ],
            nnz_chroma: [
                (16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as uint8_t,
                (16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as uint8_t,
                (32 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as uint8_t,
                (32 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as uint8_t,
            ],
            mv: [
                2 as ::core::ffi::c_int as uint8_t,
                2 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
            ],
            ref_0: [
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = x264_left_table_t {
            intra: [
                4 as ::core::ffi::c_int as uint8_t,
                6 as ::core::ffi::c_int as uint8_t,
                4 as ::core::ffi::c_int as uint8_t,
                6 as ::core::ffi::c_int as uint8_t,
            ],
            nnz: [
                3 as ::core::ffi::c_int as uint8_t,
                11 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
                11 as ::core::ffi::c_int as uint8_t,
            ],
            nnz_chroma: [
                (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
                (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
                (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
                (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
            ],
            mv: [
                0 as ::core::ffi::c_int as uint8_t,
                2 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
                2 as ::core::ffi::c_int as uint8_t,
            ],
            ref_0: [
                0 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = x264_left_table_t {
            intra: [
                4 as ::core::ffi::c_int as uint8_t,
                5 as ::core::ffi::c_int as uint8_t,
                6 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
            ],
            nnz: [
                3 as ::core::ffi::c_int as uint8_t,
                7 as ::core::ffi::c_int as uint8_t,
                11 as ::core::ffi::c_int as uint8_t,
                15 as ::core::ffi::c_int as uint8_t,
            ],
            nnz_chroma: [
                (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
                (16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as uint8_t,
                (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t,
                (32 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as uint8_t,
            ],
            mv: [
                0 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
                2 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
            ],
            ref_0: [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
            ],
        };
        init
    },
];
#[inline(always)]
#[c2rust::src_loc = "673:1"]
unsafe extern "C" fn macroblock_cache_load_neighbours(
    mut h: *mut x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut b_interlaced: ::core::ffi::c_int,
) {
    let mb_interlaced: ::core::ffi::c_int =
        (b_interlaced != 0 && (*h).mb.b_interlaced != 0) as ::core::ffi::c_int;
    let mut top_y: ::core::ffi::c_int = mb_y - ((1 as ::core::ffi::c_int) << mb_interlaced);
    let mut top: ::core::ffi::c_int = top_y * (*h).mb.i_mb_stride + mb_x;
    (*h).mb.i_mb_x = mb_x;
    (*h).mb.i_mb_y = mb_y;
    (*h).mb.i_mb_xy = mb_y * (*h).mb.i_mb_stride + mb_x;
    (*h).mb.i_b8_xy = 2 as ::core::ffi::c_int * (mb_y * (*h).mb.i_b8_stride + mb_x);
    (*h).mb.i_b4_xy = 4 as ::core::ffi::c_int * (mb_y * (*h).mb.i_b4_stride + mb_x);
    (*h).mb.left_b8[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
    (*h).mb.left_b8[0 as ::core::ffi::c_int as usize] =
        (*h).mb.left_b8[1 as ::core::ffi::c_int as usize];
    (*h).mb.left_b4[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
    (*h).mb.left_b4[0 as ::core::ffi::c_int as usize] =
        (*h).mb.left_b4[1 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour = 0 as ::core::ffi::c_uint;
    (*h).mb.i_neighbour_intra = 0 as ::core::ffi::c_uint;
    (*h).mb.i_neighbour_frame = 0 as ::core::ffi::c_uint;
    (*h).mb.i_mb_top_xy = -(1 as ::core::ffi::c_int);
    (*h).mb.i_mb_top_y = -(1 as ::core::ffi::c_int);
    (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
    (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] =
        (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize];
    (*h).mb.i_mb_topleft_xy = -(1 as ::core::ffi::c_int);
    (*h).mb.i_mb_topright_xy = -(1 as ::core::ffi::c_int);
    (*h).mb.i_mb_type_top = -(1 as ::core::ffi::c_int);
    (*h).mb.i_mb_type_left[1 as ::core::ffi::c_int as usize] = -(1 as ::core::ffi::c_int);
    (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize] =
        (*h).mb.i_mb_type_left[1 as ::core::ffi::c_int as usize];
    (*h).mb.i_mb_type_topleft = -(1 as ::core::ffi::c_int);
    (*h).mb.i_mb_type_topright = -(1 as ::core::ffi::c_int);
    (*h).mb.left_index_table = &*left_indices
        .as_ptr()
        .offset(3 as ::core::ffi::c_int as isize)
        as *const x264_left_table_t;
    (*h).mb.topleft_partition = 0 as ::core::ffi::c_int;
    let mut topleft_y: ::core::ffi::c_int = top_y;
    let mut topright_y: ::core::ffi::c_int = top_y;
    let mut left: [::core::ffi::c_int; 2] = [0; 2];
    left[1 as ::core::ffi::c_int as usize] = (*h).mb.i_mb_xy - 1 as ::core::ffi::c_int;
    left[0 as ::core::ffi::c_int as usize] = left[1 as ::core::ffi::c_int as usize];
    (*h).mb.left_b8[1 as ::core::ffi::c_int as usize] = (*h).mb.i_b8_xy - 2 as ::core::ffi::c_int;
    (*h).mb.left_b8[0 as ::core::ffi::c_int as usize] =
        (*h).mb.left_b8[1 as ::core::ffi::c_int as usize];
    (*h).mb.left_b4[1 as ::core::ffi::c_int as usize] = (*h).mb.i_b4_xy - 4 as ::core::ffi::c_int;
    (*h).mb.left_b4[0 as ::core::ffi::c_int as usize] =
        (*h).mb.left_b4[1 as ::core::ffi::c_int as usize];
    if b_interlaced != 0 {
        (*h).mb.i_mb_top_mbpair_xy =
            (*h).mb.i_mb_xy - 2 as ::core::ffi::c_int * (*h).mb.i_mb_stride;
        (*h).mb.i_mb_topleft_y = -(1 as ::core::ffi::c_int);
        (*h).mb.i_mb_topright_y = -(1 as ::core::ffi::c_int);
        if mb_y & 1 as ::core::ffi::c_int != 0 {
            if mb_x != 0
                && mb_interlaced
                    != *(*h)
                        .mb
                        .field
                        .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
            {
                left[1 as ::core::ffi::c_int as usize] =
                    (*h).mb.i_mb_xy - 1 as ::core::ffi::c_int - (*h).mb.i_mb_stride;
                left[0 as ::core::ffi::c_int as usize] = left[1 as ::core::ffi::c_int as usize];
                (*h).mb.left_b8[1 as ::core::ffi::c_int as usize] = (*h).mb.i_b8_xy
                    - 2 as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int * (*h).mb.i_b8_stride;
                (*h).mb.left_b8[0 as ::core::ffi::c_int as usize] =
                    (*h).mb.left_b8[1 as ::core::ffi::c_int as usize];
                (*h).mb.left_b4[1 as ::core::ffi::c_int as usize] = (*h).mb.i_b4_xy
                    - 4 as ::core::ffi::c_int
                    - 4 as ::core::ffi::c_int * (*h).mb.i_b4_stride;
                (*h).mb.left_b4[0 as ::core::ffi::c_int as usize] =
                    (*h).mb.left_b4[1 as ::core::ffi::c_int as usize];
                if mb_interlaced != 0 {
                    (*h).mb.left_index_table = &*left_indices
                        .as_ptr()
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const x264_left_table_t;
                    left[1 as ::core::ffi::c_int as usize] += (*h).mb.i_mb_stride;
                    (*h).mb.left_b8[1 as ::core::ffi::c_int as usize] +=
                        2 as ::core::ffi::c_int * (*h).mb.i_b8_stride;
                    (*h).mb.left_b4[1 as ::core::ffi::c_int as usize] +=
                        4 as ::core::ffi::c_int * (*h).mb.i_b4_stride;
                } else {
                    (*h).mb.left_index_table = &*left_indices
                        .as_ptr()
                        .offset(1 as ::core::ffi::c_int as isize)
                        as *const x264_left_table_t;
                    topleft_y += 1;
                    (*h).mb.topleft_partition = 1 as ::core::ffi::c_int;
                }
            }
            if mb_interlaced == 0 {
                topright_y = -(1 as ::core::ffi::c_int);
            }
        } else {
            if mb_interlaced != 0 && top >= 0 as ::core::ffi::c_int {
                if *(*h).mb.field.offset(top as isize) == 0 {
                    top += (*h).mb.i_mb_stride;
                    top_y += 1;
                }
                if mb_x != 0 {
                    topleft_y += (*(*h).mb.field.offset(
                        ((*h).mb.i_mb_stride * topleft_y + mb_x - 1 as ::core::ffi::c_int) as isize,
                    ) == 0) as ::core::ffi::c_int;
                }
                if mb_x < (*h).mb.i_mb_width - 1 as ::core::ffi::c_int {
                    topright_y += (*(*h).mb.field.offset(
                        ((*h).mb.i_mb_stride * topright_y + mb_x + 1 as ::core::ffi::c_int)
                            as isize,
                    ) == 0) as ::core::ffi::c_int;
                }
            }
            if mb_x != 0
                && mb_interlaced
                    != *(*h)
                        .mb
                        .field
                        .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
            {
                if mb_interlaced != 0 {
                    (*h).mb.left_index_table = &*left_indices
                        .as_ptr()
                        .offset(2 as ::core::ffi::c_int as isize)
                        as *const x264_left_table_t;
                    left[1 as ::core::ffi::c_int as usize] += (*h).mb.i_mb_stride;
                    (*h).mb.left_b8[1 as ::core::ffi::c_int as usize] +=
                        2 as ::core::ffi::c_int * (*h).mb.i_b8_stride;
                    (*h).mb.left_b4[1 as ::core::ffi::c_int as usize] +=
                        4 as ::core::ffi::c_int * (*h).mb.i_b4_stride;
                } else {
                    (*h).mb.left_index_table = &*left_indices
                        .as_ptr()
                        .offset(0 as ::core::ffi::c_int as isize)
                        as *const x264_left_table_t;
                }
            }
        }
    }
    if mb_x > 0 as ::core::ffi::c_int {
        (*h).mb.i_neighbour_frame |= MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint;
        (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] =
            left[0 as ::core::ffi::c_int as usize];
        (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] =
            left[1 as ::core::ffi::c_int as usize];
        (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize] = *(*h)
            .mb
            .type_0
            .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
            as ::core::ffi::c_int;
        (*h).mb.i_mb_type_left[1 as ::core::ffi::c_int as usize] = *(*h)
            .mb
            .type_0
            .offset((*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] as isize)
            as ::core::ffi::c_int;
        if *(*h)
            .mb
            .slice_table
            .offset(left[0 as ::core::ffi::c_int as usize] as isize)
            == (*h).sh.i_first_mb as int32_t
        {
            (*h).mb.i_neighbour |= MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint;
            if (*h).param.b_constrained_intra == 0
                || ((*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                    == I_4x4 as ::core::ffi::c_int
                    || (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                        == I_8x8 as ::core::ffi::c_int
                    || (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                        == I_16x16 as ::core::ffi::c_int
                    || (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                        == I_PCM as ::core::ffi::c_int)
            {
                (*h).mb.i_neighbour_intra |= MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint;
            }
        }
    }
    if (*h).i_threadslice_start >> mb_interlaced != mb_y >> mb_interlaced {
        if top >= 0 as ::core::ffi::c_int {
            (*h).mb.i_neighbour_frame |= MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint;
            (*h).mb.i_mb_top_xy = top;
            (*h).mb.i_mb_top_y = top_y;
            (*h).mb.i_mb_type_top =
                *(*h).mb.type_0.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int;
            if *(*h).mb.slice_table.offset(top as isize) == (*h).sh.i_first_mb as int32_t {
                (*h).mb.i_neighbour |= MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint;
                if (*h).param.b_constrained_intra == 0
                    || ((*h).mb.i_mb_type_top == I_4x4 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_top == I_8x8 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_top == I_16x16 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_top == I_PCM as ::core::ffi::c_int)
                {
                    (*h).mb.i_neighbour_intra |=
                        MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint;
                }
                &mut *(*h).mb.cbp.offset(top as isize) as *mut int16_t;
                &mut *(*(*h).mb.non_zero_count.offset(top as isize))
                    .as_mut_ptr()
                    .offset(12 as ::core::ffi::c_int as isize) as *mut uint8_t;
                &mut *(*h).mb.mb_transform_size.offset(top as isize) as *mut int8_t;
                if (*h).param.b_cabac != 0 {
                    &mut *(*h).mb.skipbp.offset(top as isize) as *mut int8_t;
                }
            }
        }
        if mb_x > 0 as ::core::ffi::c_int && topleft_y >= 0 as ::core::ffi::c_int {
            (*h).mb.i_neighbour_frame |= MB_TOPLEFT as ::core::ffi::c_int as ::core::ffi::c_uint;
            (*h).mb.i_mb_topleft_xy =
                (*h).mb.i_mb_stride * topleft_y + mb_x - 1 as ::core::ffi::c_int;
            (*h).mb.i_mb_topleft_y = topleft_y;
            (*h).mb.i_mb_type_topleft =
                *(*h).mb.type_0.offset((*h).mb.i_mb_topleft_xy as isize) as ::core::ffi::c_int;
            if *(*h).mb.slice_table.offset((*h).mb.i_mb_topleft_xy as isize)
                == (*h).sh.i_first_mb as int32_t
            {
                (*h).mb.i_neighbour |= MB_TOPLEFT as ::core::ffi::c_int as ::core::ffi::c_uint;
                if (*h).param.b_constrained_intra == 0
                    || ((*h).mb.i_mb_type_topleft == I_4x4 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_topleft == I_8x8 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_topleft == I_16x16 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_topleft == I_PCM as ::core::ffi::c_int)
                {
                    (*h).mb.i_neighbour_intra |=
                        MB_TOPLEFT as ::core::ffi::c_int as ::core::ffi::c_uint;
                }
            }
        }
        if mb_x < (*h).mb.i_mb_width - 1 as ::core::ffi::c_int
            && topright_y >= 0 as ::core::ffi::c_int
        {
            (*h).mb.i_neighbour_frame |= MB_TOPRIGHT as ::core::ffi::c_int as ::core::ffi::c_uint;
            (*h).mb.i_mb_topright_xy =
                (*h).mb.i_mb_stride * topright_y + mb_x + 1 as ::core::ffi::c_int;
            (*h).mb.i_mb_topright_y = topright_y;
            (*h).mb.i_mb_type_topright =
                *(*h).mb.type_0.offset((*h).mb.i_mb_topright_xy as isize) as ::core::ffi::c_int;
            if *(*h)
                .mb
                .slice_table
                .offset((*h).mb.i_mb_topright_xy as isize)
                == (*h).sh.i_first_mb as int32_t
            {
                (*h).mb.i_neighbour |= MB_TOPRIGHT as ::core::ffi::c_int as ::core::ffi::c_uint;
                if (*h).param.b_constrained_intra == 0
                    || ((*h).mb.i_mb_type_topright == I_4x4 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_topright == I_8x8 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_topright == I_16x16 as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_topright == I_PCM as ::core::ffi::c_int)
                {
                    (*h).mb.i_neighbour_intra |=
                        MB_TOPRIGHT as ::core::ffi::c_int as ::core::ffi::c_uint;
                }
            }
        }
    }
}
#[c2rust::src_loc = "848:9"]
pub const LTOP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[c2rust::src_loc = "850:12"]
pub const LBOT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline(always)]
#[c2rust::src_loc = "855:1"]
unsafe extern "C" fn macroblock_cache_load(
    mut h: *mut x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut b_mbaff: ::core::ffi::c_int,
) {
    macroblock_cache_load_neighbours(h, mb_x, mb_y, b_mbaff);
    let mut left: *mut ::core::ffi::c_int = (*h).mb.i_mb_left_xy.as_mut_ptr();
    let mut top: ::core::ffi::c_int = (*h).mb.i_mb_top_xy;
    let mut top_y: ::core::ffi::c_int = (*h).mb.i_mb_top_y;
    let mut s8x8: ::core::ffi::c_int = (*h).mb.i_b8_stride;
    let mut s4x4: ::core::ffi::c_int = (*h).mb.i_b4_stride;
    let mut top_8x8: ::core::ffi::c_int =
        (2 as ::core::ffi::c_int * top_y + 1 as ::core::ffi::c_int) * s8x8
            + 2 as ::core::ffi::c_int * mb_x;
    let mut top_4x4: ::core::ffi::c_int =
        (4 as ::core::ffi::c_int * top_y + 3 as ::core::ffi::c_int) * s4x4
            + 4 as ::core::ffi::c_int * mb_x;
    let mut lists: ::core::ffi::c_int =
        (1 as ::core::ffi::c_int) << (*h).sh.i_type & 3 as ::core::ffi::c_int;
    let mut i4x4: *mut [int8_t; 8] = (*h).mb.intra4x4_pred_mode;
    let mut nnz: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
    let mut cbp: *mut int16_t = (*h).mb.cbp;
    let mut left_index_table: *const x264_left_table_t = (*h).mb.left_index_table;
    (*h).mb.cache.deblock_strength = (*(*(*h)
        .deblock_strength
        .as_mut_ptr()
        .offset((mb_y & 1 as ::core::ffi::c_int) as isize))
    .offset(
        (if (*h).param.b_sliced_threads != 0 {
            (*h).mb.i_mb_xy
        } else {
            mb_x
        }) as isize,
    ))
    .as_mut_ptr() as *mut [[uint8_t; 4]; 8];
    if (*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        (*h).mb.cache.i_cbp_top = *cbp.offset(top as isize) as ::core::ffi::c_int;
        (*(&mut *(*h).mb.cache.intra4x4_pred_mode.as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = (*(&mut *(*i4x4.offset(top as isize))
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize) as *mut int8_t
            as *mut x264_union32_t))
            .i;
        (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*nnz.offset(top as isize))
            .as_mut_ptr()
            .offset(12 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut x264_union32_t))
            .i;
        (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            (*x264_scan8
                .as_ptr()
                .offset(16 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*nnz.offset(top as isize)).as_mut_ptr().offset(
            (16 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                + (16 as ::core::ffi::c_int >> (*h).mb.chroma_v_shift)) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            (*x264_scan8
                .as_ptr()
                .offset(32 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*nnz.offset(top as isize)).as_mut_ptr().offset(
            (32 as ::core::ffi::c_int - 4 as ::core::ffi::c_int
                + (16 as ::core::ffi::c_int >> (*h).mb.chroma_v_shift)) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        let mut l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while l < lists {
            &mut *(*(*h).mb.mv.as_mut_ptr().offset(l as isize))
                .offset((top_4x4 - 1 as ::core::ffi::c_int) as isize)
                as *mut [int16_t; 2];
            &mut *(*(*h).mb.mv.as_mut_ptr().offset(l as isize))
                .offset((top_4x4 + 4 as ::core::ffi::c_int) as isize)
                as *mut [int16_t; 2];
            &mut *(*(*h).mb.ref_0.as_mut_ptr().offset(l as isize))
                .offset((top_8x8 - 1 as ::core::ffi::c_int) as isize) as *mut int8_t;
            if (*h).param.b_cabac != 0 {
                &mut *(*(*h).mb.mvd.as_mut_ptr().offset(l as isize)).offset(top as isize)
                    as *mut [[uint8_t; 2]; 8];
            }
            l += 1;
        }
    } else {
        (*h).mb.cache.i_cbp_top = -(1 as ::core::ffi::c_int);
        (*(&mut *(*h).mb.cache.intra4x4_pred_mode.as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0x80808080 as ::core::ffi::c_uint as uint32_t;
        (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            (*x264_scan8
                .as_ptr()
                .offset(16 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0x80808080 as ::core::ffi::c_uint as uint32_t;
        (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            (*x264_scan8
                .as_ptr()
                .offset(32 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0x80808080 as ::core::ffi::c_uint as uint32_t;
    }
    if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
        let mut ltop: ::core::ffi::c_int = *left.offset(LTOP as isize);
        let mut lbot: ::core::ffi::c_int = if b_mbaff != 0 {
            *left.offset(LBOT as isize)
        } else {
            ltop
        };
        if b_mbaff != 0 {
            let top_luma: int16_t = (*cbp.offset(ltop as isize) as ::core::ffi::c_int
                >> ((*left_index_table).mv[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    & !(1 as ::core::ffi::c_int))
                & 2 as ::core::ffi::c_int) as int16_t;
            let bot_luma: int16_t = (*cbp.offset(lbot as isize) as ::core::ffi::c_int
                >> ((*left_index_table).mv[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    & !(1 as ::core::ffi::c_int))
                & 2 as ::core::ffi::c_int) as int16_t;
            (*h).mb.cache.i_cbp_left = *cbp.offset(ltop as isize) as ::core::ffi::c_int
                & 0xfff0 as ::core::ffi::c_int
                | (bot_luma as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                | top_luma as ::core::ffi::c_int;
        } else {
            (*h).mb.cache.i_cbp_left = *cbp.offset(ltop as isize) as ::core::ffi::c_int;
        }
        (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*i4x4.offset(ltop as isize))
            [(*left_index_table).intra[0 as ::core::ffi::c_int as usize] as usize];
        (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[2 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*i4x4.offset(ltop as isize))
            [(*left_index_table).intra[1 as ::core::ffi::c_int as usize] as usize];
        (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[8 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*i4x4.offset(lbot as isize))
            [(*left_index_table).intra[2 as ::core::ffi::c_int as usize] as usize];
        (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[10 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*i4x4.offset(lbot as isize))
            [(*left_index_table).intra[3 as ::core::ffi::c_int as usize] as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(ltop as isize))
            [(*left_index_table).nnz[0 as ::core::ffi::c_int as usize] as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8[2 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(ltop as isize))
            [(*left_index_table).nnz[1 as ::core::ffi::c_int as usize] as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8[8 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(lbot as isize))
            [(*left_index_table).nnz[2 as ::core::ffi::c_int as usize] as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8[10 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(lbot as isize))
            [(*left_index_table).nnz[3 as ::core::ffi::c_int as usize] as usize];
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as ::core::ffi::c_int {
            let mut offset: ::core::ffi::c_int =
                (4 as ::core::ffi::c_int >> (*h).mb.chroma_h_shift) - 4 as ::core::ffi::c_int;
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(ltop as isize))
                [((*left_index_table).nnz[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(ltop as isize))
                [((*left_index_table).nnz[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(lbot as isize))
                [((*left_index_table).nnz[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(lbot as isize))
                [((*left_index_table).nnz[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 16 as ::core::ffi::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(ltop as isize))
                [((*left_index_table).nnz[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(ltop as isize))
                [((*left_index_table).nnz[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(lbot as isize))
                [((*left_index_table).nnz[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int
                    + offset) as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(lbot as isize))
                [((*left_index_table).nnz[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 32 as ::core::ffi::c_int
                    + offset) as usize];
        } else {
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(ltop as isize))
                [(*left_index_table).nnz_chroma[0 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(lbot as isize))
                [(*left_index_table).nnz_chroma[1 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(ltop as isize))
                [(*left_index_table).nnz_chroma[2 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz.offset(lbot as isize))
                [(*left_index_table).nnz_chroma[3 as ::core::ffi::c_int as usize] as usize];
        }
    } else {
        (*h).mb.cache.i_cbp_left = -(1 as ::core::ffi::c_int);
        (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[10 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = -(1 as ::core::ffi::c_int) as int8_t;
        (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[8 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] =
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize];
        (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[2 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] =
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize];
        (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] =
            (*h).mb.cache.intra4x4_pred_mode[(x264_scan8[2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8
            [(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = 0x80 as uint8_t;
        (*h).mb.cache.non_zero_count[(x264_scan8
            [(32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
            [(32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int)
            as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8
            [(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
            [(32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int)
            as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8
            [(16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
            [(16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int)
            as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8[10 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
            [(16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int)
            as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8[8 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] =
            (*h).mb.cache.non_zero_count[(x264_scan8[10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8[2 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] =
            (*h).mb.cache.non_zero_count[(x264_scan8[8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize];
        (*h).mb.cache.non_zero_count[(x264_scan8[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] =
            (*h).mb.cache.non_zero_count[(x264_scan8[2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize];
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as ::core::ffi::c_int {
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = 0x80 as uint8_t;
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [(32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*h).mb.cache.non_zero_count[(x264_scan8
                [(16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int)
                as usize];
        }
    }
    if (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0 {
        (*h).mb.cache.i_neighbour_transform_size =
            ((*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                && *(*h)
                    .mb
                    .mb_transform_size
                    .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize)
                    as ::core::ffi::c_int
                    != 0) as ::core::ffi::c_int
                + ((*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                    && *(*h).mb.mb_transform_size.offset(top as isize) as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int;
    }
    if b_mbaff != 0 {
        (*h).mb.pic.i_fref[0 as ::core::ffi::c_int as usize] =
            (*h).i_ref[0 as ::core::ffi::c_int as usize] << (*h).mb.b_interlaced;
        (*h).mb.pic.i_fref[1 as ::core::ffi::c_int as usize] =
            (*h).i_ref[1 as ::core::ffi::c_int as usize] << (*h).mb.b_interlaced;
    }
    if b_mbaff == 0 {
        x264_10_copy_column8(
            (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize]
                .offset(-(1 as ::core::ffi::c_int as isize))
                .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
            (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize]
                .offset(15 as ::core::ffi::c_int as isize)
                .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
        );
        x264_10_copy_column8(
            (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize]
                .offset(-(1 as ::core::ffi::c_int as isize))
                .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
            (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize]
                .offset(15 as ::core::ffi::c_int as isize)
                .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
        );
        macroblock_load_pic_pointers(
            h,
            mb_x,
            mb_y,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
            x264_10_copy_column8(
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                    .offset(15 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
            );
            x264_10_copy_column8(
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                    .offset(15 as ::core::ffi::c_int as isize)
                    .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
            );
            x264_10_copy_column8(
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                    .offset(15 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
            );
            x264_10_copy_column8(
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                    .offset(15 as ::core::ffi::c_int as isize)
                    .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
            );
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                2 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            x264_10_copy_column8(
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                    .offset(7 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
            );
            x264_10_copy_column8(
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                    .offset(-(1 as ::core::ffi::c_int as isize))
                    .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                    .offset(7 as ::core::ffi::c_int as isize)
                    .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
            );
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_422 as ::core::ffi::c_int {
                x264_10_copy_column8(
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                        .offset(7 as ::core::ffi::c_int as isize)
                        .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                );
                x264_10_copy_column8(
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                        .offset(-(1 as ::core::ffi::c_int as isize))
                        .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                        .offset(7 as ::core::ffi::c_int as isize)
                        .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                );
            }
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
    } else {
        macroblock_load_pic_pointers(
            h,
            mb_x,
            mb_y,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                2 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            macroblock_load_pic_pointers(
                h,
                mb_x,
                mb_y,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
        }
    }
    if !(*(*h).fdec).integral.is_null() {
        let mut offset_0: ::core::ffi::c_int = 16 as ::core::ffi::c_int
            * (mb_x + mb_y * (*(*h).fdec).i_stride[0 as ::core::ffi::c_int as usize]);
        let mut list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while list < 2 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*h).mb.pic.i_fref[list as usize] {
                (*h).mb.pic.p_integral[list as usize][i as usize] =
                    &mut *(**(*(*h).fref.as_mut_ptr().offset(list as isize))
                        .as_mut_ptr()
                        .offset(i as isize))
                    .integral
                    .offset(offset_0 as isize) as *mut uint16_t;
                i += 1;
            }
            list += 1;
        }
    }
    x264_10_prefetch_fenc(h, (*h).fenc, mb_x, mb_y);
    let mut l_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while l_0 < lists {
        let mut mv: *mut [int16_t; 2] = (*h).mb.mv[l_0 as usize];
        let mut ref_0: *mut int8_t = (*h).mb.ref_0[l_0 as usize];
        let mut i8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
        if (*h).mb.i_neighbour & MB_TOPLEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
            let mut ir: ::core::ffi::c_int = if b_mbaff != 0 {
                2 as ::core::ffi::c_int
                    * (s8x8 * (*h).mb.i_mb_topleft_y + mb_x - 1 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int
                    + s8x8
            } else {
                top_8x8 - 1 as ::core::ffi::c_int
            };
            let mut iv: ::core::ffi::c_int = if b_mbaff != 0 {
                4 as ::core::ffi::c_int
                    * (s4x4 * (*h).mb.i_mb_topleft_y + mb_x - 1 as ::core::ffi::c_int)
                    + 3 as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int * s4x4
            } else {
                top_4x4 - 1 as ::core::ffi::c_int
            };
            if b_mbaff != 0 && (*h).mb.topleft_partition != 0 {
                iv -= 2 as ::core::ffi::c_int * s4x4;
                ir -= s8x8;
            }
            (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = *ref_0.offset(ir as isize);
            (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                .as_mut_ptr()
                .offset(i8 as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*((*mv.offset(iv as isize)).as_mut_ptr() as *mut x264_union32_t)).i;
        } else {
            (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = -(2 as ::core::ffi::c_int) as int8_t;
            (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                .as_mut_ptr()
                .offset(i8 as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = 0 as uint32_t;
        }
        i8 = x264_scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            - 8 as ::core::ffi::c_int;
        if (*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
            (*h).mb.cache.ref_0[l_0 as usize][(i8 + 1 as ::core::ffi::c_int) as usize] =
                *ref_0.offset((top_8x8 + 0 as ::core::ffi::c_int) as isize);
            (*h).mb.cache.ref_0[l_0 as usize][(i8 + 0 as ::core::ffi::c_int) as usize] =
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 1 as ::core::ffi::c_int) as usize];
            (*h).mb.cache.ref_0[l_0 as usize][(i8 + 3 as ::core::ffi::c_int) as usize] =
                *ref_0.offset((top_8x8 + 1 as ::core::ffi::c_int) as isize);
            (*h).mb.cache.ref_0[l_0 as usize][(i8 + 2 as ::core::ffi::c_int) as usize] =
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + 3 as ::core::ffi::c_int) as usize];
            (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                .as_mut_ptr()
                .offset(i8 as isize))
            .as_mut_ptr() as *mut x264_union128_sse_t))
                .i = (*((*mv.offset(top_4x4 as isize)).as_mut_ptr() as *mut x264_union128_sse_t)).i;
        } else {
            (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                .as_mut_ptr()
                .offset(i8 as isize))
            .as_mut_ptr() as *mut x264_union128_sse_t))
                .i = M128_ZERO;
            (*(&mut *(*(*h).mb.cache.ref_0.as_mut_ptr().offset(l_0 as isize))
                .as_mut_ptr()
                .offset(i8 as isize) as *mut int8_t as *mut x264_union32_t))
                .i = (-(2 as ::core::ffi::c_int) as uint8_t as ::core::ffi::c_uint)
                .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
        }
        i8 = x264_scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
        if (*h).mb.i_neighbour & MB_TOPRIGHT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
            let mut ir_0: ::core::ffi::c_int = if b_mbaff != 0 {
                2 as ::core::ffi::c_int
                    * (s8x8 * (*h).mb.i_mb_topright_y + (mb_x + 1 as ::core::ffi::c_int))
                    + s8x8
            } else {
                top_8x8 + 2 as ::core::ffi::c_int
            };
            let mut iv_0: ::core::ffi::c_int = if b_mbaff != 0 {
                4 as ::core::ffi::c_int
                    * (s4x4 * (*h).mb.i_mb_topright_y + (mb_x + 1 as ::core::ffi::c_int))
                    + 3 as ::core::ffi::c_int * s4x4
            } else {
                top_4x4 + 4 as ::core::ffi::c_int
            };
            (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = *ref_0.offset(ir_0 as isize);
            (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                .as_mut_ptr()
                .offset(i8 as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*((*mv.offset(iv_0 as isize)).as_mut_ptr() as *mut x264_union32_t)).i;
        } else {
            (*h).mb.cache.ref_0[l_0 as usize][i8 as usize] = -(2 as ::core::ffi::c_int) as int8_t;
        }
        i8 = x264_scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
            if b_mbaff != 0 {
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[LTOP as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8
                                * (*left_index_table).ref_0[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) as isize,
                    );
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[LTOP as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8
                                * (*left_index_table).ref_0[1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) as isize,
                    );
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[LBOT as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8
                                * (*left_index_table).ref_0[2 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) as isize,
                    );
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[LBOT as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8
                                * (*left_index_table).ref_0[3 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) as isize,
                    );
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset((i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4
                            * *(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset((i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4
                            * *(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset((i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(1 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4
                            * *(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset((i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(1 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4
                            * *(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            } else {
                let ir_1: ::core::ffi::c_int = (*h).mb.i_b8_xy - 1 as ::core::ffi::c_int;
                let iv_1: ::core::ffi::c_int = (*h).mb.i_b4_xy - 1 as ::core::ffi::c_int;
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                    *ref_0.offset((ir_1 + 0 as ::core::ffi::c_int * s8x8) as isize);
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                    (*h).mb.cache.ref_0[l_0 as usize]
                        [(i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                    *ref_0.offset((ir_1 + 1 as ::core::ffi::c_int * s8x8) as isize);
                (*h).mb.cache.ref_0[l_0 as usize]
                    [(i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                    (*h).mb.cache.ref_0[l_0 as usize]
                        [(i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset((i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 0 as ::core::ffi::c_int * s4x4) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset((i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 1 as ::core::ffi::c_int * s4x4) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset((i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 2 as ::core::ffi::c_int * s4x4) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset((i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset((iv_1 + 3 as ::core::ffi::c_int * s4x4) as isize))
                    .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            }
        } else {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 4 as ::core::ffi::c_int {
                (*h).mb.cache.ref_0[l_0 as usize][(i8 + i_0 * 8 as ::core::ffi::c_int) as usize] =
                    -(2 as ::core::ffi::c_int) as int8_t;
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset((i8 + i_0 * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = 0 as uint32_t;
                i_0 += 1;
            }
        }
        if b_mbaff != 0
            && (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        {
            if (*h).mb.b_interlaced != 0
                && *(*h)
                    .mb
                    .field
                    .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                    == 0
            {
                (*h).mb.cache.topright_ref[l_0 as usize][0 as ::core::ffi::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8 * 0 as ::core::ffi::c_int) as isize,
                    );
                (*h).mb.cache.topright_ref[l_0 as usize][1 as ::core::ffi::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8 * 1 as ::core::ffi::c_int) as isize,
                    );
                (*h).mb.cache.topright_ref[l_0 as usize][2 as ::core::ffi::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[1 as ::core::ffi::c_int as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8 * 0 as ::core::ffi::c_int) as isize,
                    );
                (*((*(*(*h).mb.cache.topright_mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4
                            * (*(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.topright_mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4
                            * (*(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.topright_mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(2 as ::core::ffi::c_int as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(1 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4
                            * (*(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            } else if (*h).mb.b_interlaced == 0
                && *(*h)
                    .mb
                    .field
                    .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    != 0
            {
                (*h).mb.cache.topright_ref[l_0 as usize][0 as ::core::ffi::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8 * 2 as ::core::ffi::c_int
                            + s8x8
                                * (*left_index_table).ref_0[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) as isize,
                    );
                (*h).mb.cache.topright_ref[l_0 as usize][1 as ::core::ffi::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8 * 2 as ::core::ffi::c_int
                            + s8x8
                                * (*left_index_table).ref_0[1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) as isize,
                    );
                (*h).mb.cache.topright_ref[l_0 as usize][2 as ::core::ffi::c_int as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8 * 2 as ::core::ffi::c_int
                            + s8x8
                                * (*left_index_table).ref_0[2 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) as isize,
                    );
                (*((*(*(*h).mb.cache.topright_mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4 * 4 as ::core::ffi::c_int
                        + s4x4
                            * *(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.topright_mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4 * 4 as ::core::ffi::c_int
                        + s4x4
                            * *(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.topright_mv.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(2 as ::core::ffi::c_int as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4 * 4 as ::core::ffi::c_int
                        + s4x4
                            * *(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            }
        }
        if (*h).param.b_cabac != 0 {
            let mut mvd: *mut [[uint8_t; 2]; 8] = (*h).mb.mvd[l_0 as usize];
            if (*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
                (*((*(*(*h).mb.cache.mvd.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as isize,
                    ))
                .as_mut_ptr() as *mut x264_union64_t))
                    .i = (*((*(*mvd.offset(top as isize))
                    .as_mut_ptr()
                    .offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr() as *mut x264_union64_t))
                    .i;
            } else {
                (*((*(*(*h).mb.cache.mvd.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - 8 as ::core::ffi::c_int) as isize,
                    ))
                .as_mut_ptr() as *mut x264_union64_t))
                    .i = 0 as uint64_t;
            }
            if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                && (b_mbaff == 0
                    || (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int)
            {
                (*((*(*(*h).mb.cache.mvd.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as isize,
                    ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*((*(*mvd
                    .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize))
                .as_mut_ptr()
                .offset(
                    *(*left_index_table)
                        .intra
                        .as_ptr()
                        .offset(0 as ::core::ffi::c_int as isize) as isize,
                ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*((*(*(*h).mb.cache.mvd.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(2 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as isize,
                    ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*((*(*mvd
                    .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize))
                .as_mut_ptr()
                .offset(
                    *(*left_index_table)
                        .intra
                        .as_ptr()
                        .offset(1 as ::core::ffi::c_int as isize) as isize,
                ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            } else {
                (*((*(*(*h).mb.cache.mvd.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as isize,
                    ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as uint16_t;
                (*((*(*(*h).mb.cache.mvd.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as isize,
                    ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as uint16_t;
            }
            if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                && (b_mbaff == 0
                    || (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int)
            {
                (*((*(*(*h).mb.cache.mvd.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(8 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as isize,
                    ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*((*(*mvd
                    .offset(*left.offset(1 as ::core::ffi::c_int as isize) as isize))
                .as_mut_ptr()
                .offset(
                    *(*left_index_table)
                        .intra
                        .as_ptr()
                        .offset(2 as ::core::ffi::c_int as isize) as isize,
                ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*((*(*(*h).mb.cache.mvd.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset(10 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int) as isize,
                    ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i = (*((*(*mvd
                    .offset(*left.offset(1 as ::core::ffi::c_int as isize) as isize))
                .as_mut_ptr()
                .offset(
                    *(*left_index_table)
                        .intra
                        .as_ptr()
                        .offset(3 as ::core::ffi::c_int as isize) as isize,
                ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            } else {
                (*((*(*(*h).mb.cache.mvd.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as isize,
                    ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as uint16_t;
                (*((*(*(*h).mb.cache.mvd.as_mut_ptr().offset(l_0 as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as isize,
                    ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i = 0 as uint16_t;
            }
        }
        if b_mbaff != 0 {
            if (*h).mb.b_interlaced != 0 {
                if (*h).mb.i_mb_topleft_xy >= 0 as ::core::ffi::c_int
                    && *(*h).mb.field.offset((*h).mb.i_mb_topleft_xy as isize) == 0
                {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                }
                if top >= 0 as ::core::ffi::c_int && *(*h).mb.field.offset(top as isize) == 0 {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                }
                if (*h).mb.i_mb_topright_xy >= 0 as ::core::ffi::c_int
                    && *(*h).mb.field.offset((*h).mb.i_mb_topright_xy as isize) == 0
                {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                }
                if *left.offset(0 as ::core::ffi::c_int as isize) >= 0 as ::core::ffi::c_int
                    && *(*h)
                        .mb
                        .field
                        .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize)
                        == 0
                {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            (((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                / 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize]
                            [0 as ::core::ffi::c_int as usize] = (((*h).mb.cache.topright_ref
                            [l_0 as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int)
                            << 1 as ::core::ffi::c_int)
                            as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][0 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                            [l_0 as usize][0 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            / 2 as ::core::ffi::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][0 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize]
                            [1 as ::core::ffi::c_int as usize] = (((*h).mb.cache.topright_ref
                            [l_0 as usize][1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int)
                            << 1 as ::core::ffi::c_int)
                            as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][1 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                            [l_0 as usize][1 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            / 2 as ::core::ffi::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][1 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][1 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][2 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize]
                            [2 as ::core::ffi::c_int as usize] = (((*h).mb.cache.topright_ref
                            [l_0 as usize][2 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int)
                            << 1 as ::core::ffi::c_int)
                            as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][2 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                            [l_0 as usize][2 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            / 2 as ::core::ffi::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][2 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mvd[l_0 as usize][2 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as uint8_t;
                    }
                }
            } else {
                if (*h).mb.i_mb_topleft_xy >= 0 as ::core::ffi::c_int
                    && *(*h).mb.field.offset((*h).mb.i_mb_topleft_xy as isize) as ::core::ffi::c_int
                        != 0
                {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                }
                if top >= 0 as ::core::ffi::c_int
                    && *(*h).mb.field.offset(top as isize) as ::core::ffi::c_int != 0
                {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                }
                if (*h).mb.i_mb_topright_xy >= 0 as ::core::ffi::c_int
                    && *(*h).mb.field.offset((*h).mb.i_mb_topright_xy as isize)
                        as ::core::ffi::c_int
                        != 0
                {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        + 4 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + 4 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                }
                if *left.offset(0 as ::core::ffi::c_int as isize) >= 0 as ::core::ffi::c_int
                    && *(*h)
                        .mb
                        .field
                        .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize)
                        as ::core::ffi::c_int
                        != 0
                {
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                        [0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int
                        + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                        as usize] as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize] =
                            ((*h).mb.cache.ref_0[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize] as ::core::ffi::c_int
                                >> 1 as ::core::ffi::c_int) as int8_t;
                        (*h).mb.cache.mv[l_0 as usize][(x264_scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            ((*h).mb.cache.mv[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                * 2 as ::core::ffi::c_int) as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                            [0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][(x264_scan8
                                [0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize]
                            [0 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_ref
                            [l_0 as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                            as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][0 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                            [l_0 as usize][0 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][0 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][0 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize]
                            [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_ref
                            [l_0 as usize][1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                            as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][1 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                            [l_0 as usize][1 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][1 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][1 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                    if (*h).mb.cache.topright_ref[l_0 as usize][2 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int
                        >= 0 as ::core::ffi::c_int
                    {
                        (*h).mb.cache.topright_ref[l_0 as usize]
                            [2 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_ref
                            [l_0 as usize][2 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int)
                            as int8_t;
                        (*h).mb.cache.topright_mv[l_0 as usize][2 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] = ((*h).mb.cache.topright_mv
                            [l_0 as usize][2 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            * 2 as ::core::ffi::c_int)
                            as int16_t;
                        (*h).mb.cache.mvd[l_0 as usize][2 as ::core::ffi::c_int as usize]
                            [1 as ::core::ffi::c_int as usize] =
                            (((*h).mb.cache.mvd[l_0 as usize][2 as ::core::ffi::c_int as usize]
                                [1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int)
                                << 1 as ::core::ffi::c_int) as uint8_t;
                    }
                }
            }
        }
        l_0 += 1;
    }
    if b_mbaff != 0 && mb_x == 0 as ::core::ffi::c_int && mb_y & 1 as ::core::ffi::c_int == 0 {
        if (*h).mb.i_mb_top_xy >= (*h).sh.i_first_mb {
            (*h).mb.field_decoding_flag =
                *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int;
        } else {
            (*h).mb.field_decoding_flag = 0 as ::core::ffi::c_int;
        }
    }
    (*h).mb.b_allow_skip = 1 as ::core::ffi::c_int;
    if b_mbaff != 0 {
        if (*h).mb.b_interlaced != (*h).mb.field_decoding_flag
            && mb_y & 1 as ::core::ffi::c_int != 0
            && (*(*h)
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
                    == B_SKIP as ::core::ffi::c_int)
        {
            (*h).mb.b_allow_skip = 0 as ::core::ffi::c_int;
        }
    }
    if (*h).param.b_cabac != 0 {
        if b_mbaff != 0 {
            let mut left_xy: ::core::ffi::c_int = 0;
            let mut top_xy: ::core::ffi::c_int = 0;
            let mut mb_xy: ::core::ffi::c_int =
                mb_x + (mb_y & !(1 as ::core::ffi::c_int)) * (*h).mb.i_mb_stride;
            left_xy = mb_xy - 1 as ::core::ffi::c_int;
            if mb_y & 1 as ::core::ffi::c_int != 0
                && mb_x > 0 as ::core::ffi::c_int
                && (*h).mb.field_decoding_flag
                    == *(*h).mb.field.offset(left_xy as isize) as ::core::ffi::c_int
            {
                left_xy += (*h).mb.i_mb_stride;
            }
            if (*h).mb.field_decoding_flag != 0 {
                top_xy = mb_xy - (*h).mb.i_mb_stride;
                if mb_y & 1 as ::core::ffi::c_int == 0
                    && top_xy >= 0 as ::core::ffi::c_int
                    && *(*h).mb.slice_table.offset(top_xy as isize) == (*h).sh.i_first_mb as int32_t
                    && *(*h).mb.field.offset(top_xy as isize) as ::core::ffi::c_int != 0
                {
                    top_xy -= (*h).mb.i_mb_stride;
                }
            } else {
                top_xy = mb_x + (mb_y - 1 as ::core::ffi::c_int) * (*h).mb.i_mb_stride;
            }
            (*h).mb.cache.i_neighbour_skip = (mb_x > 0 as ::core::ffi::c_int
                && *(*h).mb.slice_table.offset(left_xy as isize) == (*h).sh.i_first_mb as int32_t
                && !(*(*h).mb.type_0.offset(left_xy as isize) as ::core::ffi::c_int
                    == P_SKIP as ::core::ffi::c_int
                    || *(*h).mb.type_0.offset(left_xy as isize) as ::core::ffi::c_int
                        == B_SKIP as ::core::ffi::c_int))
                as ::core::ffi::c_int
                + (top_xy >= 0 as ::core::ffi::c_int
                    && *(*h).mb.slice_table.offset(top_xy as isize)
                        == (*h).sh.i_first_mb as int32_t
                    && !(*(*h).mb.type_0.offset(top_xy as isize) as ::core::ffi::c_int
                        == P_SKIP as ::core::ffi::c_int
                        || *(*h).mb.type_0.offset(top_xy as isize) as ::core::ffi::c_int
                            == B_SKIP as ::core::ffi::c_int))
                    as ::core::ffi::c_int;
        } else {
            (*h).mb.cache.i_neighbour_skip =
                ((*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
                    && !((*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                        == P_SKIP as ::core::ffi::c_int
                        || (*h).mb.i_mb_type_left[0 as ::core::ffi::c_int as usize]
                            == B_SKIP as ::core::ffi::c_int)) as ::core::ffi::c_int
                    + ((*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
                        != 0
                        && !((*h).mb.i_mb_type_top == P_SKIP as ::core::ffi::c_int
                            || (*h).mb.i_mb_type_top == B_SKIP as ::core::ffi::c_int))
                        as ::core::ffi::c_int;
        }
    }
    if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
        (*h).mb.bipred_weight = (*(*(*h)
            .mb
            .bipred_weight_buf
            .as_mut_ptr()
            .offset((*h).mb.b_interlaced as isize))
        .as_mut_ptr()
        .offset(((*h).mb.b_interlaced & (mb_y & 1 as ::core::ffi::c_int)) as isize))
        .as_mut_ptr() as *mut [int8_t; 4];
        (*h).mb.dist_scale_factor = (*(*(*h)
            .mb
            .dist_scale_factor_buf
            .as_mut_ptr()
            .offset((*h).mb.b_interlaced as isize))
        .as_mut_ptr()
        .offset(((*h).mb.b_interlaced & (mb_y & 1 as ::core::ffi::c_int)) as isize))
        .as_mut_ptr() as *mut [int16_t; 4];
        if (*h).param.b_cabac != 0 {
            let mut skipbp: uint8_t = 0;
            x264_macroblock_cache_skip(
                h,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            if b_mbaff != 0 {
                skipbp = (if (*h).mb.i_neighbour
                    & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    *(*h).mb.skipbp.offset(*left.offset(LTOP as isize) as isize)
                        as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t;
                (*h).mb.cache.skip[(x264_scan8[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as usize] = (skipbp as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int
                        + ((*left_index_table).mv[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            & !(1 as ::core::ffi::c_int))
                    & 1 as ::core::ffi::c_int)
                    as int8_t;
                skipbp = (if (*h).mb.i_neighbour
                    & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    *(*h).mb.skipbp.offset(*left.offset(LBOT as isize) as isize)
                        as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t;
                (*h).mb.cache.skip[(x264_scan8[8 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as usize] = (skipbp as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int
                        + ((*left_index_table).mv[2 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            & !(1 as ::core::ffi::c_int))
                    & 1 as ::core::ffi::c_int)
                    as int8_t;
            } else {
                skipbp = (if (*h).mb.i_neighbour
                    & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint
                    != 0
                {
                    *(*h)
                        .mb
                        .skipbp
                        .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize)
                        as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint8_t;
                (*h).mb.cache.skip[(x264_scan8[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as usize] =
                    (skipbp as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int) as int8_t;
                (*h).mb.cache.skip[(x264_scan8[8 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as usize] =
                    (skipbp as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int) as int8_t;
            }
            skipbp = (if (*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
            {
                *(*h).mb.skipbp.offset(top as isize) as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint8_t;
            (*h).mb.cache.skip[(x264_scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as usize] =
                (skipbp as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int) as int8_t;
            (*h).mb.cache.skip[(x264_scan8[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as usize] =
                (skipbp as ::core::ffi::c_int & 0x8 as ::core::ffi::c_int) as int8_t;
        }
    }
    if (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int {
        x264_10_mb_predict_mv_pskip(h, (*h).mb.cache.pskip_mv.as_mut_ptr());
    }
    (*h).mb.i_neighbour8[0 as ::core::ffi::c_int as usize] = (*h).mb.i_neighbour_intra
        & (MB_TOP as ::core::ffi::c_int
            | MB_LEFT as ::core::ffi::c_int
            | MB_TOPLEFT as ::core::ffi::c_int) as ::core::ffi::c_uint
        | (if (*h).mb.i_neighbour_intra & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
            MB_TOPRIGHT as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint;
    (*h).mb.i_neighbour4[0 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour8[0 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour4[1 as ::core::ffi::c_int as usize] = (MB_LEFT as ::core::ffi::c_int
        | (if (*h).mb.i_neighbour_intra & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
            MB_TOP as ::core::ffi::c_int
                | MB_TOPLEFT as ::core::ffi::c_int
                | MB_TOPRIGHT as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })) as ::core::ffi::c_uint;
    (*h).mb.i_neighbour4[4 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour4[1 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour8[2 as ::core::ffi::c_int as usize] = (MB_TOP as ::core::ffi::c_int
        | MB_TOPRIGHT as ::core::ffi::c_int
        | (if (*h).mb.i_neighbour_intra & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        {
            MB_LEFT as ::core::ffi::c_int | MB_TOPLEFT as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        })) as ::core::ffi::c_uint;
    (*h).mb.i_neighbour4[10 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour8[2 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour4[8 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour4[10 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour4[2 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour4[8 as ::core::ffi::c_int as usize];
    (*h).mb.i_neighbour8[1 as ::core::ffi::c_int as usize] = MB_LEFT as ::core::ffi::c_int
        as ::core::ffi::c_uint
        | (*h).mb.i_neighbour_intra & MB_TOPRIGHT as ::core::ffi::c_int as ::core::ffi::c_uint
        | (if (*h).mb.i_neighbour_intra & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
            MB_TOP as ::core::ffi::c_int | MB_TOPLEFT as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint;
    (*h).mb.i_neighbour4[5 as ::core::ffi::c_int as usize] =
        (*h).mb.i_neighbour8[1 as ::core::ffi::c_int as usize];
}
#[no_mangle]
#[c2rust::src_loc = "1354:1"]
pub unsafe extern "C" fn x264_10_macroblock_cache_load_progressive(
    mut h: *mut x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) {
    macroblock_cache_load(h, mb_x, mb_y, 0 as ::core::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "1359:1"]
pub unsafe extern "C" fn x264_10_macroblock_cache_load_interlaced(
    mut h: *mut x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) {
    macroblock_cache_load(h, mb_x, mb_y, 1 as ::core::ffi::c_int);
}
#[c2rust::src_loc = "1364:1"]
unsafe extern "C" fn macroblock_deblock_strength_mbaff(
    mut h: *mut x264_t,
    mut bs: *mut [[uint8_t; 4]; 8],
) {
    if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        && *(*h)
            .mb
            .field
            .offset((*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] as isize)
            as ::core::ffi::c_int
            != (*h).mb.b_interlaced
    {
        static mut offset: [[[uint8_t; 8]; 2]; 2] = [
            [
                [
                    0 as ::core::ffi::c_int as uint8_t,
                    0 as ::core::ffi::c_int as uint8_t,
                    0 as ::core::ffi::c_int as uint8_t,
                    0 as ::core::ffi::c_int as uint8_t,
                    1 as ::core::ffi::c_int as uint8_t,
                    1 as ::core::ffi::c_int as uint8_t,
                    1 as ::core::ffi::c_int as uint8_t,
                    1 as ::core::ffi::c_int as uint8_t,
                ],
                [
                    2 as ::core::ffi::c_int as uint8_t,
                    2 as ::core::ffi::c_int as uint8_t,
                    2 as ::core::ffi::c_int as uint8_t,
                    2 as ::core::ffi::c_int as uint8_t,
                    3 as ::core::ffi::c_int as uint8_t,
                    3 as ::core::ffi::c_int as uint8_t,
                    3 as ::core::ffi::c_int as uint8_t,
                    3 as ::core::ffi::c_int as uint8_t,
                ],
            ],
            [
                [
                    0 as ::core::ffi::c_int as uint8_t,
                    1 as ::core::ffi::c_int as uint8_t,
                    2 as ::core::ffi::c_int as uint8_t,
                    3 as ::core::ffi::c_int as uint8_t,
                    0 as ::core::ffi::c_int as uint8_t,
                    1 as ::core::ffi::c_int as uint8_t,
                    2 as ::core::ffi::c_int as uint8_t,
                    3 as ::core::ffi::c_int as uint8_t,
                ],
                [
                    0 as ::core::ffi::c_int as uint8_t,
                    1 as ::core::ffi::c_int as uint8_t,
                    2 as ::core::ffi::c_int as uint8_t,
                    3 as ::core::ffi::c_int as uint8_t,
                    0 as ::core::ffi::c_int as uint8_t,
                    1 as ::core::ffi::c_int as uint8_t,
                    2 as ::core::ffi::c_int as uint8_t,
                    3 as ::core::ffi::c_int as uint8_t,
                ],
            ],
        ];
        let mut tmpbs: [uint8_t; 8] = [0; 8];
        let mut off: *const uint8_t = (*(*offset.as_ptr().offset((*h).mb.b_interlaced as isize))
            .as_ptr()
            .offset(((*h).mb.i_mb_y & 1 as ::core::ffi::c_int) as isize))
        .as_ptr();
        let mut nnz: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            let mut left: ::core::ffi::c_int = (*h).mb.i_mb_left_xy[(if (*h).mb.b_interlaced != 0 {
                i >> 2 as ::core::ffi::c_int
            } else {
                i & 1 as ::core::ffi::c_int
            }) as usize];
            let mut nnz_this: ::core::ffi::c_int = (*h).mb.cache.non_zero_count[(x264_scan8
                [0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int * (i >> 1 as ::core::ffi::c_int))
                as usize] as ::core::ffi::c_int;
            let mut nnz_left: ::core::ffi::c_int = (*nnz.offset(left as isize))[(3
                as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int * *off.offset(i as isize) as ::core::ffi::c_int)
                as usize] as ::core::ffi::c_int;
            if (*h).param.b_cabac == 0 && (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0 {
                let mut j: ::core::ffi::c_int =
                    *off.offset(i as isize) as ::core::ffi::c_int & !(1 as ::core::ffi::c_int);
                if *(*h).mb.mb_transform_size.offset(left as isize) != 0 {
                    nnz_left = ((*(&mut *(*nnz.offset(left as isize))
                        .as_mut_ptr()
                        .offset((2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * j) as isize)
                        as *mut uint8_t as *mut x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*(&mut *(*nnz.offset(left as isize)).as_mut_ptr().offset(
                            (2 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int * (1 as ::core::ffi::c_int + j))
                                as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                            .i as ::core::ffi::c_int
                        != 0) as ::core::ffi::c_int;
                }
            }
            tmpbs[i as usize] = (if nnz_left != 0 || nnz_this != 0 {
                2 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            }) as uint8_t;
            i += 1;
        }
        if (*h).mb.b_interlaced != 0 {
            (*((*(*bs.offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*(&mut *tmpbs.as_mut_ptr().offset(0 as ::core::ffi::c_int as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i;
            (*((*(*bs.offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(4 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = (*(&mut *tmpbs.as_mut_ptr().offset(4 as ::core::ffi::c_int as isize)
                as *mut uint8_t as *mut x264_union32_t))
                .i;
        } else {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 4 as ::core::ffi::c_int {
                (*bs.offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                    [i_0 as usize] = tmpbs[(2 as ::core::ffi::c_int * i_0) as usize];
                i_0 += 1;
            }
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1 < 4 as ::core::ffi::c_int {
                (*bs.offset(0 as ::core::ffi::c_int as isize))[4 as ::core::ffi::c_int as usize]
                    [i_1 as usize] =
                    tmpbs[(1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i_1) as usize];
                i_1 += 1;
            }
        }
    }
    if (*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        && (*h).mb.b_interlaced
            != *(*h).mb.field.offset((*h).mb.i_mb_top_xy as isize) as ::core::ffi::c_int
    {
        if (*h).mb.i_mb_y & 1 as ::core::ffi::c_int == 0 && (*h).mb.b_interlaced == 0 {
            let mut mbn_xy: ::core::ffi::c_int =
                (*h).mb.i_mb_xy - 2 as ::core::ffi::c_int * (*h).mb.i_mb_stride;
            let mut nnz_cur: *mut uint8_t = &mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as isize)
                as *mut uint8_t;
            let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j_0 < 2 as ::core::ffi::c_int {
                let mut nnz_0: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
                let mut nnz_top: [uint8_t; 4] = [0; 4];
                (*(nnz_top.as_mut_ptr() as *mut x264_union32_t)).i =
                    (*(&mut *(*nnz_0.offset(mbn_xy as isize))
                        .as_mut_ptr()
                        .offset((3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
                        as *mut uint8_t as *mut x264_union32_t))
                        .i;
                if (*h).param.b_cabac == 0
                    && (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0
                    && *(*h).mb.mb_transform_size.offset(mbn_xy as isize) as ::core::ffi::c_int != 0
                {
                    nnz_top[1 as ::core::ffi::c_int as usize] =
                        ((*(&mut *(*nnz_0.offset(mbn_xy as isize))
                            .as_mut_ptr()
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut uint8_t as *mut x264_union16_t))
                            .i as ::core::ffi::c_int
                            != 0
                            || (*(&mut *(*nnz_0.offset(mbn_xy as isize))
                                .as_mut_ptr()
                                .offset(12 as ::core::ffi::c_int as isize)
                                as *mut uint8_t
                                as *mut x264_union16_t))
                                .i as ::core::ffi::c_int
                                != 0) as ::core::ffi::c_int as uint8_t;
                    nnz_top[0 as ::core::ffi::c_int as usize] =
                        nnz_top[1 as ::core::ffi::c_int as usize];
                    nnz_top[3 as ::core::ffi::c_int as usize] =
                        ((*(&mut *(*nnz_0.offset(mbn_xy as isize))
                            .as_mut_ptr()
                            .offset(10 as ::core::ffi::c_int as isize)
                            as *mut uint8_t as *mut x264_union16_t))
                            .i as ::core::ffi::c_int
                            != 0
                            || (*(&mut *(*nnz_0.offset(mbn_xy as isize))
                                .as_mut_ptr()
                                .offset(14 as ::core::ffi::c_int as isize)
                                as *mut uint8_t
                                as *mut x264_union16_t))
                                .i as ::core::ffi::c_int
                                != 0) as ::core::ffi::c_int as uint8_t;
                    nnz_top[2 as ::core::ffi::c_int as usize] =
                        nnz_top[3 as ::core::ffi::c_int as usize];
                }
                let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_2 < 4 as ::core::ffi::c_int {
                    (*bs.offset(1 as ::core::ffi::c_int as isize))
                        [(4 as ::core::ffi::c_int * j_0) as usize][i_2 as usize] =
                        (if *nnz_cur.offset(i_2 as isize) as ::core::ffi::c_int != 0
                            || nnz_top[i_2 as usize] as ::core::ffi::c_int != 0
                        {
                            2 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        }) as uint8_t;
                    i_2 += 1;
                }
                j_0 += 1;
                mbn_xy += (*h).mb.i_mb_stride;
            }
        } else {
            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_3 < 4 as ::core::ffi::c_int {
                (*bs.offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                    [i_3 as usize] = (if (*bs.offset(1 as ::core::ffi::c_int as isize))
                    [0 as ::core::ffi::c_int as usize][i_3 as usize]
                    as ::core::ffi::c_int
                    > 1 as ::core::ffi::c_int
                {
                    (*bs.offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
                        [i_3 as usize] as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                }) as uint8_t;
                i_3 += 1;
            }
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1438:1"]
pub unsafe extern "C" fn x264_10_macroblock_deblock_strength(mut h: *mut x264_t) {
    let mut bs: *mut [[uint8_t; 4]; 8] = (*h).mb.cache.deblock_strength;
    if (*h).mb.i_type == I_4x4 as ::core::ffi::c_int
        || (*h).mb.i_type == I_8x8 as ::core::ffi::c_int
        || (*h).mb.i_type == I_16x16 as ::core::ffi::c_int
        || (*h).mb.i_type == I_PCM as ::core::ffi::c_int
    {
        (*((*(*bs.offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
        .as_mut_ptr() as *mut x264_union32_t))
            .i = 0x3030303 as uint32_t;
        (*((*(*bs.offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(2 as ::core::ffi::c_int as isize))
        .as_mut_ptr() as *mut x264_union64_t))
            .i = 0x303030303030303 as uint64_t;
        (*((*(*bs.offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
        .as_mut_ptr() as *mut x264_union32_t))
            .i = 0x3030303 as uint32_t;
        (*((*(*bs.offset(1 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(2 as ::core::ffi::c_int as isize))
        .as_mut_ptr() as *mut x264_union64_t))
            .i = 0x303030303030303 as uint64_t;
        return;
    }
    if (*h).mb.b_transform_8x8 != 0
        && !((*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int)
    {
        let mut cbp_mask: ::core::ffi::c_int = 0xf as ::core::ffi::c_int >> (*h).mb.chroma_v_shift;
        if (*h).mb.i_cbp_luma & cbp_mask == cbp_mask {
            (*((*(*bs.offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = 0x2020202 as uint32_t;
            (*((*(*bs.offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(2 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = 0x2020202 as uint32_t;
            (*((*(*bs.offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(4 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = 0x2020202 as uint32_t;
            (*((*(*bs.offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union64_t))
                .i = 0x202020202020202 as uint64_t;
            (*((*(*bs.offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(2 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union64_t))
                .i = 0x202020202020202 as uint64_t;
            (*((*(*bs.offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(4 as ::core::ffi::c_int as isize))
            .as_mut_ptr() as *mut x264_union32_t))
                .i = 0x2020202 as uint32_t;
            return;
        }
    }
    let mut neighbour_changed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*h).sh.i_disable_deblocking_filter_idc != 2 as ::core::ffi::c_int {
        neighbour_changed =
            ((*h).mb.i_neighbour_frame & !(*h).mb.i_neighbour) as ::core::ffi::c_int;
        (*h).mb.i_neighbour = (*h).mb.i_neighbour_frame;
    }
    if (*h).sh.b_mbaff != 0
        && (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        && *(*h)
            .mb
            .field
            .offset(((*h).mb.i_mb_xy - 1 as ::core::ffi::c_int) as isize)
            as ::core::ffi::c_int
            != (*h).mb.b_interlaced
    {
        (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] =
            (*h).mb.i_mb_xy - 1 as ::core::ffi::c_int;
        (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] =
            (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize];
        if (*h).mb.i_mb_y & 1 as ::core::ffi::c_int != 0 {
            (*h).mb.i_mb_left_xy[0 as ::core::ffi::c_int as usize] -= (*h).mb.i_mb_stride;
        } else {
            (*h).mb.i_mb_left_xy[1 as ::core::ffi::c_int as usize] += (*h).mb.i_mb_stride;
        }
    }
    if neighbour_changed != 0 {
        let mut top_y: ::core::ffi::c_int = (*h).mb.i_mb_top_y;
        let mut top_8x8: ::core::ffi::c_int =
            (2 as ::core::ffi::c_int * top_y + 1 as ::core::ffi::c_int) * (*h).mb.i_b8_stride
                + 2 as ::core::ffi::c_int * (*h).mb.i_mb_x;
        let mut top_4x4: ::core::ffi::c_int =
            (4 as ::core::ffi::c_int * top_y + 3 as ::core::ffi::c_int) * (*h).mb.i_b4_stride
                + 4 as ::core::ffi::c_int * (*h).mb.i_mb_x;
        let mut s8x8: ::core::ffi::c_int = (*h).mb.i_b8_stride;
        let mut s4x4: ::core::ffi::c_int = (*h).mb.i_b4_stride;
        let mut nnz: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
        let mut left_index_table: *const x264_left_table_t = if (*h).sh.b_mbaff != 0 {
            (*h).mb.left_index_table
        } else {
            &*left_indices
                .as_ptr()
                .offset(3 as ::core::ffi::c_int as isize) as *const x264_left_table_t
        };
        if neighbour_changed & MB_TOP as ::core::ffi::c_int != 0 {
            (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
                .i = (*(&mut *(*nnz.offset((*h).mb.i_mb_top_xy as isize))
                .as_mut_ptr()
                .offset(12 as ::core::ffi::c_int as isize) as *mut uint8_t
                as *mut x264_union32_t))
                .i;
        }
        if neighbour_changed & MB_LEFT as ::core::ffi::c_int != 0 {
            let mut left: *mut ::core::ffi::c_int = (*h).mb.i_mb_left_xy.as_mut_ptr();
            (*h).mb.cache.non_zero_count[(x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz
                .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize))
                [(*left_index_table).nnz[0 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[2 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz
                .offset(*left.offset(0 as ::core::ffi::c_int as isize) as isize))
                [(*left_index_table).nnz[1 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[8 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz
                .offset(*left.offset(1 as ::core::ffi::c_int as isize) as isize))
                [(*left_index_table).nnz[2 as ::core::ffi::c_int as usize] as usize];
            (*h).mb.cache.non_zero_count[(x264_scan8[10 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] = (*nnz
                .offset(*left.offset(1 as ::core::ffi::c_int as isize) as isize))
                [(*left_index_table).nnz[3 as ::core::ffi::c_int as usize] as usize];
        }
        let mut l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while l <= ((*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int) as ::core::ffi::c_int {
            let mut mv: *mut [int16_t; 2] = (*h).mb.mv[l as usize];
            let mut ref_0: *mut int8_t = (*h).mb.ref_0[l as usize];
            let mut i8: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int;
            if neighbour_changed & MB_TOP as ::core::ffi::c_int != 0 {
                (*h).mb.cache.ref_0[l as usize][(i8 + 1 as ::core::ffi::c_int) as usize] =
                    *ref_0.offset((top_8x8 + 0 as ::core::ffi::c_int) as isize);
                (*h).mb.cache.ref_0[l as usize][(i8 + 0 as ::core::ffi::c_int) as usize] =
                    (*h).mb.cache.ref_0[l as usize][(i8 + 1 as ::core::ffi::c_int) as usize];
                (*h).mb.cache.ref_0[l as usize][(i8 + 3 as ::core::ffi::c_int) as usize] =
                    *ref_0.offset((top_8x8 + 1 as ::core::ffi::c_int) as isize);
                (*h).mb.cache.ref_0[l as usize][(i8 + 2 as ::core::ffi::c_int) as usize] =
                    (*h).mb.cache.ref_0[l as usize][(i8 + 3 as ::core::ffi::c_int) as usize];
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                    .as_mut_ptr()
                    .offset(i8 as isize))
                .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i =
                    (*((*mv.offset(top_4x4 as isize)).as_mut_ptr() as *mut x264_union128_sse_t)).i;
            }
            i8 = x264_scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int;
            if neighbour_changed & MB_LEFT as ::core::ffi::c_int != 0 {
                (*h).mb.cache.ref_0[l as usize]
                    [(i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[0 as ::core::ffi::c_int as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8
                                * (*left_index_table).ref_0[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) as isize,
                    );
                (*h).mb.cache.ref_0[l as usize]
                    [(i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                    (*h).mb.cache.ref_0[l as usize]
                        [(i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
                (*h).mb.cache.ref_0[l as usize]
                    [(i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] = *ref_0
                    .offset(
                        ((*h).mb.left_b8[1 as ::core::ffi::c_int as usize]
                            + 1 as ::core::ffi::c_int
                            + s8x8
                                * (*left_index_table).ref_0[2 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) as isize,
                    );
                (*h).mb.cache.ref_0[l as usize]
                    [(i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
                    (*h).mb.cache.ref_0[l as usize]
                        [(i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                    .as_mut_ptr()
                    .offset((i8 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4
                            * *(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                    .as_mut_ptr()
                    .offset((i8 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4
                            * *(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                    .as_mut_ptr()
                    .offset((i8 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(1 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4
                            * *(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(2 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
                (*((*(*(*h).mb.cache.mv.as_mut_ptr().offset(l as isize))
                    .as_mut_ptr()
                    .offset((i8 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i = (*((*mv.offset(
                    (*(*h)
                        .mb
                        .left_b4
                        .as_mut_ptr()
                        .offset(1 as ::core::ffi::c_int as isize)
                        + 3 as ::core::ffi::c_int
                        + s4x4
                            * *(*left_index_table)
                                .mv
                                .as_ptr()
                                .offset(3 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                    .i;
            }
            l += 1;
        }
    }
    if (*h).param.analyse.i_weighted_pred == X264_WEIGHTP_SMART
        && (*h).sh.i_type == SLICE_TYPE_P as ::core::ffi::c_int
    {
        let mut i8_0: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int
            - 8 as ::core::ffi::c_int;
        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [(i8_0 + 1 as ::core::ffi::c_int) as usize] =
            (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 0 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize];
        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [(i8_0 + 0 as ::core::ffi::c_int) as usize] = (*h).mb.cache.ref_0
            [0 as ::core::ffi::c_int as usize][(i8_0 + 1 as ::core::ffi::c_int) as usize];
        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [(i8_0 + 3 as ::core::ffi::c_int) as usize] =
            (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 2 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize];
        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [(i8_0 + 2 as ::core::ffi::c_int) as usize] = (*h).mb.cache.ref_0
            [0 as ::core::ffi::c_int as usize][(i8_0 + 3 as ::core::ffi::c_int) as usize];
        i8_0 = x264_scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int;
        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [(i8_0 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
            (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize];
        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [(i8_0 + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
            (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [(i8_0 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
            (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize]
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize];
        (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
            [(i8_0 + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize] =
            (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [(i8_0 + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as usize];
        let mut ref0: ::core::ffi::c_int =
            (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
        let mut ref1: ::core::ffi::c_int =
            (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [x264_scan8[4 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
        let mut ref2: ::core::ffi::c_int =
            (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [x264_scan8[8 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
        let mut ref3: ::core::ffi::c_int =
            (*h).mb.deblock_ref_table[((*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                [x264_scan8[12 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
        let mut reftop: uint32_t =
            pack16to32(ref0 as uint8_t as uint32_t, ref1 as uint8_t as uint32_t)
                .wrapping_mul(0x101 as uint32_t);
        let mut refbot: uint32_t =
            pack16to32(ref2 as uint8_t as uint32_t, ref3 as uint8_t as uint32_t)
                .wrapping_mul(0x101 as uint32_t);
        (*(&mut *(*(*h)
            .mb
            .cache
            .ref_0
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = reftop;
        (*(&mut *(*(*h)
            .mb
            .cache
            .ref_0
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = reftop;
        (*(&mut *(*(*h)
            .mb
            .cache
            .ref_0
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = refbot;
        (*(&mut *(*(*h)
            .mb
            .cache
            .ref_0
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i = refbot;
    }
    if (*h).param.b_cabac == 0 && (*(*h).pps.as_mut_ptr()).b_transform_8x8_mode != 0 {
        let mut nnz_0: *mut [uint8_t; 48] = (*h).mb.non_zero_count;
        let mut top: ::core::ffi::c_int = (*h).mb.i_mb_top_xy;
        let mut left_0: *mut ::core::ffi::c_int = (*h).mb.i_mb_left_xy.as_mut_ptr();
        if (*h).mb.i_neighbour & MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint != 0
            && *(*h).mb.mb_transform_size.offset(top as isize) as ::core::ffi::c_int != 0
        {
            let mut i8_1: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int;
            let mut nnz_top0: ::core::ffi::c_int = (*(&mut *(*nnz_0.offset(top as isize))
                .as_mut_ptr()
                .offset(8 as ::core::ffi::c_int as isize)
                as *mut uint8_t
                as *mut x264_union16_t))
                .i as ::core::ffi::c_int
                | (*(&mut *(*nnz_0.offset(top as isize))
                    .as_mut_ptr()
                    .offset(12 as ::core::ffi::c_int as isize) as *mut uint8_t
                    as *mut x264_union16_t))
                    .i as ::core::ffi::c_int;
            let mut nnz_top1: ::core::ffi::c_int = (*(&mut *(*nnz_0.offset(top as isize))
                .as_mut_ptr()
                .offset(10 as ::core::ffi::c_int as isize)
                as *mut uint8_t
                as *mut x264_union16_t))
                .i as ::core::ffi::c_int
                | (*(&mut *(*nnz_0.offset(top as isize))
                    .as_mut_ptr()
                    .offset(14 as ::core::ffi::c_int as isize) as *mut uint8_t
                    as *mut x264_union16_t))
                    .i as ::core::ffi::c_int;
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset((i8_1 + 0 as ::core::ffi::c_int) as isize) as *mut uint8_t
                as *mut x264_union16_t))
                .i = (if nnz_top0 != 0 {
                0x101 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint16_t;
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset((i8_1 + 2 as ::core::ffi::c_int) as isize) as *mut uint8_t
                as *mut x264_union16_t))
                .i = (if nnz_top1 != 0 {
                0x101 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint16_t;
        }
        if (*h).mb.i_neighbour & MB_LEFT as ::core::ffi::c_int as ::core::ffi::c_uint != 0 {
            let mut i8_2: ::core::ffi::c_int = x264_scan8[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int;
            if *(*h)
                .mb
                .mb_transform_size
                .offset(*left_0.offset(0 as ::core::ffi::c_int as isize) as isize)
                != 0
            {
                let mut nnz_left0: ::core::ffi::c_int =
                    (*(&mut *(*nnz_0
                        .offset(*left_0.offset(0 as ::core::ffi::c_int as isize) as isize))
                    .as_mut_ptr()
                    .offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t
                        as *mut x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*(&mut *(*nnz_0
                            .offset(*left_0.offset(0 as ::core::ffi::c_int as isize) as isize))
                        .as_mut_ptr()
                        .offset(6 as ::core::ffi::c_int as isize)
                            as *mut uint8_t as *mut x264_union16_t))
                            .i as ::core::ffi::c_int;
                (*h).mb.cache.non_zero_count
                    [(i8_2 + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as usize] =
                    (nnz_left0 != 0) as ::core::ffi::c_int as uint8_t;
                (*h).mb.cache.non_zero_count
                    [(i8_2 + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as usize] =
                    (nnz_left0 != 0) as ::core::ffi::c_int as uint8_t;
            }
            if *(*h)
                .mb
                .mb_transform_size
                .offset(*left_0.offset(1 as ::core::ffi::c_int as isize) as isize)
                != 0
            {
                let mut nnz_left1: ::core::ffi::c_int =
                    (*(&mut *(*nnz_0
                        .offset(*left_0.offset(1 as ::core::ffi::c_int as isize) as isize))
                    .as_mut_ptr()
                    .offset(10 as ::core::ffi::c_int as isize)
                        as *mut uint8_t as *mut x264_union16_t))
                        .i as ::core::ffi::c_int
                        | (*(&mut *(*nnz_0
                            .offset(*left_0.offset(1 as ::core::ffi::c_int as isize) as isize))
                        .as_mut_ptr()
                        .offset(14 as ::core::ffi::c_int as isize)
                            as *mut uint8_t as *mut x264_union16_t))
                            .i as ::core::ffi::c_int;
                (*h).mb.cache.non_zero_count
                    [(i8_2 + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as usize] =
                    (nnz_left1 != 0) as ::core::ffi::c_int as uint8_t;
                (*h).mb.cache.non_zero_count
                    [(i8_2 + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as usize] =
                    (nnz_left1 != 0) as ::core::ffi::c_int as uint8_t;
            }
        }
        if (*h).mb.b_transform_8x8 != 0 {
            let mut nnz0: ::core::ffi::c_int =
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as isize)
                    as *mut uint8_t as *mut x264_union16_t))
                    .i as ::core::ffi::c_int
                    | (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        *x264_scan8.as_ptr().offset(2 as ::core::ffi::c_int as isize) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                        .i as ::core::ffi::c_int;
            let mut nnz1: ::core::ffi::c_int =
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(*x264_scan8.as_ptr().offset(4 as ::core::ffi::c_int as isize) as isize)
                    as *mut uint8_t as *mut x264_union16_t))
                    .i as ::core::ffi::c_int
                    | (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        *x264_scan8.as_ptr().offset(6 as ::core::ffi::c_int as isize) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                        .i as ::core::ffi::c_int;
            let mut nnz2: ::core::ffi::c_int = (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(8 as ::core::ffi::c_int as isize) as isize)
                as *mut uint8_t
                as *mut x264_union16_t))
                .i as ::core::ffi::c_int
                | (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                    *x264_scan8
                        .as_ptr()
                        .offset(10 as ::core::ffi::c_int as isize) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                    .i as ::core::ffi::c_int;
            let mut nnz3: ::core::ffi::c_int =
                (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                    *x264_scan8
                        .as_ptr()
                        .offset(12 as ::core::ffi::c_int as isize) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                    .i as ::core::ffi::c_int
                    | (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                        *x264_scan8
                            .as_ptr()
                            .offset(14 as ::core::ffi::c_int as isize)
                            as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                        .i as ::core::ffi::c_int;
            let mut nnztop: uint32_t = pack16to32(
                (nnz0 != 0) as ::core::ffi::c_int as uint32_t,
                (nnz1 != 0) as ::core::ffi::c_int as uint32_t,
            )
            .wrapping_mul(0x101 as uint32_t);
            let mut nnzbot: uint32_t = pack16to32(
                (nnz2 != 0) as ::core::ffi::c_int as uint32_t,
                (nnz3 != 0) as ::core::ffi::c_int as uint32_t,
            )
            .wrapping_mul(0x101 as uint32_t);
            (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnztop;
            (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnztop;
            (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnzbot;
            (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
                (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
                .i = nnzbot;
        }
    }
    (*h).loopf
        .deblock_strength
        .expect("non-null function pointer")(
        (*h).mb.cache.non_zero_count.as_mut_ptr(),
        (*h).mb.cache.ref_0.as_mut_ptr(),
        (*h).mb.cache.mv.as_mut_ptr(),
        bs as *mut [[uint8_t; 4]; 8],
        4 as ::core::ffi::c_int >> (*h).mb.b_interlaced,
        ((*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int) as ::core::ffi::c_int,
    );
    if (*h).sh.b_mbaff != 0 {
        macroblock_deblock_strength_mbaff(h, bs);
    }
}
#[inline(always)]
#[c2rust::src_loc = "1624:1"]
unsafe extern "C" fn macroblock_store_pic(
    mut h: *mut x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut i: ::core::ffi::c_int,
    mut b_chroma: ::core::ffi::c_int,
    mut b_mbaff: ::core::ffi::c_int,
) {
    let mut height: ::core::ffi::c_int = if b_chroma != 0 {
        16 as ::core::ffi::c_int >> (*h).mb.chroma_v_shift
    } else {
        16 as ::core::ffi::c_int
    };
    let mut i_stride: ::core::ffi::c_int = (*(*h).fdec).i_stride[i as usize];
    let mut i_stride2: ::core::ffi::c_int =
        i_stride << (b_mbaff != 0 && (*h).mb.b_interlaced != 0) as ::core::ffi::c_int;
    let mut i_pix_offset: ::core::ffi::c_int = if b_mbaff != 0 && (*h).mb.b_interlaced != 0 {
        16 as ::core::ffi::c_int * mb_x
            + height * (mb_y & !(1 as ::core::ffi::c_int)) * i_stride
            + (mb_y & 1 as ::core::ffi::c_int) * i_stride
    } else {
        16 as ::core::ffi::c_int * mb_x + height * mb_y * i_stride
    };
    if b_chroma != 0 {
        (*h).mc
            .store_interleave_chroma
            .expect("non-null function pointer")(
            &mut *(*(*(*h).fdec)
                .plane
                .as_mut_ptr()
                .offset(1 as ::core::ffi::c_int as isize))
            .offset(i_pix_offset as isize),
            i_stride2 as intptr_t,
            (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
            (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize],
            height,
        );
    } else {
        (*h).mc.copy[PIXEL_16x16 as ::core::ffi::c_int as usize]
            .expect("non-null function pointer")(
            &mut *(*(*(*h).fdec).plane.as_mut_ptr().offset(i as isize))
                .offset(i_pix_offset as isize),
            i_stride2 as intptr_t,
            (*h).mb.pic.p_fdec[i as usize],
            FDEC_STRIDE as intptr_t,
            16 as ::core::ffi::c_int,
        );
    };
}
#[inline(always)]
#[c2rust::src_loc = "1638:1"]
unsafe extern "C" fn macroblock_backup_intra(
    mut h: *mut x264_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut b_mbaff: ::core::ffi::c_int,
) {
    let mut backup_dst: ::core::ffi::c_int = if b_mbaff == 0 {
        mb_y & 1 as ::core::ffi::c_int
    } else if mb_y & 1 as ::core::ffi::c_int != 0 {
        1 as ::core::ffi::c_int
    } else if (*h).mb.b_interlaced != 0 {
        0 as ::core::ffi::c_int
    } else {
        2 as ::core::ffi::c_int
    };
    memcpy(
        &mut *(*(*(*h)
            .intra_border_backup
            .as_mut_ptr()
            .offset(backup_dst as isize))
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
        .offset((mb_x * 16 as ::core::ffi::c_int) as isize) as *mut pixel
            as *mut ::core::ffi::c_void,
        (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize]
            .offset((FDEC_STRIDE * 15 as ::core::ffi::c_int) as isize)
            as *const ::core::ffi::c_void,
        (16 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
    );
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
        memcpy(
            &mut *(*(*(*h)
                .intra_border_backup
                .as_mut_ptr()
                .offset(backup_dst as isize))
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
            .offset((mb_x * 16 as ::core::ffi::c_int) as isize) as *mut pixel
                as *mut ::core::ffi::c_void,
            (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                .offset((FDEC_STRIDE * 15 as ::core::ffi::c_int) as isize)
                as *const ::core::ffi::c_void,
            (16 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
        memcpy(
            &mut *(*(*(*h)
                .intra_border_backup
                .as_mut_ptr()
                .offset(backup_dst as isize))
            .as_mut_ptr()
            .offset(2 as ::core::ffi::c_int as isize))
            .offset((mb_x * 16 as ::core::ffi::c_int) as isize) as *mut pixel
                as *mut ::core::ffi::c_void,
            (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                .offset((FDEC_STRIDE * 15 as ::core::ffi::c_int) as isize)
                as *const ::core::ffi::c_void,
            (16 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        let mut backup_src: ::core::ffi::c_int =
            (15 as ::core::ffi::c_int >> (*h).mb.chroma_v_shift) * FDEC_STRIDE;
        memcpy(
            &mut *(*(*(*h)
                .intra_border_backup
                .as_mut_ptr()
                .offset(backup_dst as isize))
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
            .offset((mb_x * 16 as ::core::ffi::c_int) as isize) as *mut pixel
                as *mut ::core::ffi::c_void,
            (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize].offset(backup_src as isize)
                as *const ::core::ffi::c_void,
            (8 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
        memcpy(
            &mut *(*(*(*h)
                .intra_border_backup
                .as_mut_ptr()
                .offset(backup_dst as isize))
            .as_mut_ptr()
            .offset(1 as ::core::ffi::c_int as isize))
            .offset((mb_x * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                as *mut pixel as *mut ::core::ffi::c_void,
            (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize].offset(backup_src as isize)
                as *const ::core::ffi::c_void,
            (8 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
    }
    if b_mbaff != 0 {
        if mb_y & 1 as ::core::ffi::c_int != 0 {
            let mut backup_src_0: ::core::ffi::c_int = (if (*h).mb.b_interlaced != 0 {
                7 as ::core::ffi::c_int
            } else {
                14 as ::core::ffi::c_int
            }) * FDEC_STRIDE;
            backup_dst = if (*h).mb.b_interlaced != 0 {
                2 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            memcpy(
                &mut *(*(*(*h)
                    .intra_border_backup
                    .as_mut_ptr()
                    .offset(backup_dst as isize))
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
                .offset((mb_x * 16 as ::core::ffi::c_int) as isize) as *mut pixel
                    as *mut ::core::ffi::c_void,
                (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize].offset(backup_src_0 as isize)
                    as *const ::core::ffi::c_void,
                (16 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
            );
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
                memcpy(
                    &mut *(*(*(*h)
                        .intra_border_backup
                        .as_mut_ptr()
                        .offset(backup_dst as isize))
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                    .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                        as *mut pixel as *mut ::core::ffi::c_void,
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                        .offset(backup_src_0 as isize)
                        as *const ::core::ffi::c_void,
                    (16 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
                );
                memcpy(
                    &mut *(*(*(*h)
                        .intra_border_backup
                        .as_mut_ptr()
                        .offset(backup_dst as isize))
                    .as_mut_ptr()
                    .offset(2 as ::core::ffi::c_int as isize))
                    .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                        as *mut pixel as *mut ::core::ffi::c_void,
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                        .offset(backup_src_0 as isize)
                        as *const ::core::ffi::c_void,
                    (16 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
                );
            } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
                if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as ::core::ffi::c_int
                {
                    backup_src_0 = (if (*h).mb.b_interlaced != 0 {
                        3 as ::core::ffi::c_int
                    } else {
                        6 as ::core::ffi::c_int
                    }) * FDEC_STRIDE;
                }
                memcpy(
                    &mut *(*(*(*h)
                        .intra_border_backup
                        .as_mut_ptr()
                        .offset(backup_dst as isize))
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                    .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                        as *mut pixel as *mut ::core::ffi::c_void,
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                        .offset(backup_src_0 as isize)
                        as *const ::core::ffi::c_void,
                    (8 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
                );
                memcpy(
                    &mut *(*(*(*h)
                        .intra_border_backup
                        .as_mut_ptr()
                        .offset(backup_dst as isize))
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                    .offset((mb_x * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                        as *mut pixel as *mut ::core::ffi::c_void,
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                        .offset(backup_src_0 as isize)
                        as *const ::core::ffi::c_void,
                    (8 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
                );
            }
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1680:1"]
pub unsafe extern "C" fn x264_10_macroblock_cache_save(mut h: *mut x264_t) {
    let i_mb_xy: ::core::ffi::c_int = (*h).mb.i_mb_xy;
    let i_mb_type: ::core::ffi::c_int =
        x264_mb_type_fix[(*h).mb.i_type as usize] as ::core::ffi::c_int;
    let s8x8: ::core::ffi::c_int = (*h).mb.i_b8_stride;
    let s4x4: ::core::ffi::c_int = (*h).mb.i_b4_stride;
    let i_mb_4x4: ::core::ffi::c_int = (*h).mb.i_b4_xy;
    let i_mb_8x8: ::core::ffi::c_int = (*h).mb.i_b8_xy;
    let mut i4x4: *mut int8_t = (*(*h).mb.intra4x4_pred_mode.offset(i_mb_xy as isize)).as_mut_ptr();
    let mut nnz: *mut uint8_t = (*(*h).mb.non_zero_count.offset(i_mb_xy as isize)).as_mut_ptr();
    if (*h).sh.b_mbaff != 0 {
        macroblock_backup_intra(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 1 as ::core::ffi::c_int);
        macroblock_store_pic(
            h,
            (*h).mb.i_mb_x,
            (*h).mb.i_mb_y,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                2 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            );
        }
    } else {
        macroblock_backup_intra(h, (*h).mb.i_mb_x, (*h).mb.i_mb_y, 0 as ::core::ffi::c_int);
        macroblock_store_pic(
            h,
            (*h).mb.i_mb_x,
            (*h).mb.i_mb_y,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                2 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
            macroblock_store_pic(
                h,
                (*h).mb.i_mb_x,
                (*h).mb.i_mb_y,
                1 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
    }
    x264_10_prefetch_fenc(h, (*h).fdec, (*h).mb.i_mb_x, (*h).mb.i_mb_y);
    *(*h).mb.type_0.offset(i_mb_xy as isize) = i_mb_type as int8_t;
    *(*h).mb.slice_table.offset(i_mb_xy as isize) = (*h).sh.i_first_mb as int32_t;
    *(*h).mb.partition.offset(i_mb_xy as isize) = (if i_mb_type == I_4x4 as ::core::ffi::c_int
        || i_mb_type == I_8x8 as ::core::ffi::c_int
        || i_mb_type == I_16x16 as ::core::ffi::c_int
        || i_mb_type == I_PCM as ::core::ffi::c_int
    {
        D_16x16 as ::core::ffi::c_int
    } else {
        (*h).mb.i_partition
    }) as uint8_t;
    (*h).mb.i_mb_prev_xy = i_mb_xy;
    if i_mb_type == I_4x4 as ::core::ffi::c_int {
        (*(&mut *i4x4.offset(0 as ::core::ffi::c_int as isize) as *mut int8_t
            as *mut x264_union32_t))
            .i = (*(&mut *(*h).mb.cache.intra4x4_pred_mode.as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset(10 as ::core::ffi::c_int as isize) as isize,
        ) as *mut int8_t as *mut x264_union32_t))
            .i;
        (*(&mut *i4x4.offset(4 as ::core::ffi::c_int as isize) as *mut int8_t
            as *mut x264_union32_t))
            .i = pack8to32(
            (*h).mb.cache.intra4x4_pred_mode[x264_scan8[5 as ::core::ffi::c_int as usize] as usize]
                as uint32_t,
            (*h).mb.cache.intra4x4_pred_mode[x264_scan8[7 as ::core::ffi::c_int as usize] as usize]
                as uint32_t,
            (*h).mb.cache.intra4x4_pred_mode[x264_scan8[13 as ::core::ffi::c_int as usize] as usize]
                as uint32_t,
            0 as uint32_t,
        );
    } else if (*h).param.b_constrained_intra == 0
        || (i_mb_type == I_4x4 as ::core::ffi::c_int
            || i_mb_type == I_8x8 as ::core::ffi::c_int
            || i_mb_type == I_16x16 as ::core::ffi::c_int
            || i_mb_type == I_PCM as ::core::ffi::c_int)
    {
        (*(i4x4 as *mut x264_union64_t)).i = (I_PRED_4x4_DC as ::core::ffi::c_int
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x101010101010101 as ::core::ffi::c_ulonglong)
            as uint64_t;
    } else {
        (*(i4x4 as *mut x264_union64_t)).i = (-(1 as ::core::ffi::c_int) as uint8_t
            as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x101010101010101 as ::core::ffi::c_ulonglong)
            as uint64_t;
    }
    if i_mb_type == I_PCM as ::core::ffi::c_int {
        *(*h).mb.qp.offset(i_mb_xy as isize) = 0 as int8_t;
        (*h).mb.i_last_dqp = 0 as ::core::ffi::c_int;
        (*h).mb.i_cbp_chroma =
            if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else {
                2 as ::core::ffi::c_int
            };
        (*h).mb.i_cbp_luma = 0xf as ::core::ffi::c_int;
        *(*h).mb.cbp.offset(i_mb_xy as isize) = ((*h).mb.i_cbp_chroma << 4 as ::core::ffi::c_int
            | (*h).mb.i_cbp_luma
            | 0x1700 as ::core::ffi::c_int)
            as int16_t;
        (*h).mb.b_transform_8x8 = 0 as ::core::ffi::c_int;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 48 as ::core::ffi::c_int {
            (*h).mb.cache.non_zero_count[x264_scan8[i as usize] as usize] =
                (if (*h).param.b_cabac != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    16 as ::core::ffi::c_int
                }) as uint8_t;
            i += 1;
        }
    } else {
        if (*h).mb.i_type != I_16x16 as ::core::ffi::c_int
            && (*h).mb.i_cbp_luma == 0 as ::core::ffi::c_int
            && (*h).mb.i_cbp_chroma == 0 as ::core::ffi::c_int
        {
            (*h).mb.i_qp = (*h).mb.i_last_qp;
        }
        *(*h).mb.qp.offset(i_mb_xy as isize) = (*h).mb.i_qp as int8_t;
        (*h).mb.i_last_dqp = (*h).mb.i_qp - (*h).mb.i_last_qp;
        (*h).mb.i_last_qp = (*h).mb.i_qp;
    }
    (*(&mut *nnz.offset(
        (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset(
        (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(2 as ::core::ffi::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset(
        (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(8 as ::core::ffi::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset(
        (0 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset(10 as ::core::ffi::c_int as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset(
        (16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset(
        (16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset(
        (32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    (*(&mut *nnz.offset(
        (32 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
        *x264_scan8
            .as_ptr()
            .offset((32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize) as isize,
    ) as *mut uint8_t as *mut x264_union32_t))
        .i;
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as ::core::ffi::c_int {
        (*(&mut *nnz.offset(
            (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *nnz.offset(
            (16 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *nnz.offset(
            (32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
        (*(&mut *nnz.offset(
            (32 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i = (*(&mut *(*h).mb.cache.non_zero_count.as_mut_ptr().offset(
            *x264_scan8
                .as_ptr()
                .offset((32 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
            .i;
    }
    if (*h).mb.i_cbp_luma == 0 as ::core::ffi::c_int
        && (*h).mb.i_type != I_8x8 as ::core::ffi::c_int
    {
        (*h).mb.b_transform_8x8 = 0 as ::core::ffi::c_int;
    }
    *(*h).mb.mb_transform_size.offset(i_mb_xy as isize) = (*h).mb.b_transform_8x8 as int8_t;
    if (*h).sh.i_type != SLICE_TYPE_I as ::core::ffi::c_int {
        let mut mv0: *mut [int16_t; 2] = &mut *(*(*h)
            .mb
            .mv
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .offset(i_mb_4x4 as isize) as *mut [int16_t; 2];
        let mut ref0: *mut int8_t = &mut *(*(*h)
            .mb
            .ref_0
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .offset(i_mb_8x8 as isize) as *mut int8_t;
        if !(i_mb_type == I_4x4 as ::core::ffi::c_int
            || i_mb_type == I_8x8 as ::core::ffi::c_int
            || i_mb_type == I_16x16 as ::core::ffi::c_int
            || i_mb_type == I_PCM as ::core::ffi::c_int)
        {
            *ref0.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * s8x8) as isize) =
                (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[0 as ::core::ffi::c_int as usize] as usize];
            *ref0.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * s8x8) as isize) =
                (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[4 as ::core::ffi::c_int as usize] as usize];
            *ref0.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * s8x8) as isize) =
                (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[8 as ::core::ffi::c_int as usize] as usize];
            *ref0.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * s8x8) as isize) =
                (*h).mb.cache.ref_0[0 as ::core::ffi::c_int as usize]
                    [x264_scan8[12 as ::core::ffi::c_int as usize] as usize];
            (*(&mut *mv0.offset((0 as ::core::ffi::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*((*(*(*h)
                .mb
                .cache
                .mv
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
            ))
            .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            (*(&mut *mv0.offset((1 as ::core::ffi::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*((*(*(*h)
                .mb
                .cache
                .mv
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize,
            ))
            .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            (*(&mut *mv0.offset((2 as ::core::ffi::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*((*(*(*h)
                .mb
                .cache
                .mv
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
            ))
            .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            (*(&mut *mv0.offset((3 as ::core::ffi::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = (*((*(*(*h)
                .mb
                .cache
                .mv
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
            ))
            .as_mut_ptr() as *mut x264_union128_sse_t))
                .i;
            if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
                let mut mv1: *mut [int16_t; 2] = &mut *(*(*h)
                    .mb
                    .mv
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(i_mb_4x4 as isize)
                    as *mut [int16_t; 2];
                let mut ref1: *mut int8_t = &mut *(*(*h)
                    .mb
                    .ref_0
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(i_mb_8x8 as isize)
                    as *mut int8_t;
                *ref1.offset((0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                        [x264_scan8[0 as ::core::ffi::c_int as usize] as usize];
                *ref1.offset((1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                        [x264_scan8[4 as ::core::ffi::c_int as usize] as usize];
                *ref1.offset((0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                        [x264_scan8[8 as ::core::ffi::c_int as usize] as usize];
                *ref1.offset((1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * s8x8) as isize) =
                    (*h).mb.cache.ref_0[1 as ::core::ffi::c_int as usize]
                        [x264_scan8[12 as ::core::ffi::c_int as usize] as usize];
                (*(&mut *mv1.offset((0 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = (*((*(*(*h)
                    .mb
                    .cache
                    .mv
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
                (*(&mut *mv1.offset((1 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = (*((*(*(*h)
                    .mb
                    .cache
                    .mv
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
                (*(&mut *mv1.offset((2 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = (*((*(*(*h)
                    .mb
                    .cache
                    .mv
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
                (*(&mut *mv1.offset((3 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = (*((*(*(*h)
                    .mb
                    .cache
                    .mv
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize,
                    ))
                    .as_mut_ptr() as *mut x264_union128_sse_t))
                    .i;
            }
        } else {
            (*(&mut *ref0.offset((0 as ::core::ffi::c_int * s8x8) as isize) as *mut int8_t
                as *mut x264_union16_t))
                .i = (-(1 as ::core::ffi::c_int) as uint8_t as ::core::ffi::c_int
                * 0x101 as ::core::ffi::c_int) as uint16_t;
            (*(&mut *ref0.offset((1 as ::core::ffi::c_int * s8x8) as isize) as *mut int8_t
                as *mut x264_union16_t))
                .i = (-(1 as ::core::ffi::c_int) as uint8_t as ::core::ffi::c_int
                * 0x101 as ::core::ffi::c_int) as uint16_t;
            (*(&mut *mv0.offset((0 as ::core::ffi::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = M128_ZERO;
            (*(&mut *mv0.offset((1 as ::core::ffi::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = M128_ZERO;
            (*(&mut *mv0.offset((2 as ::core::ffi::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = M128_ZERO;
            (*(&mut *mv0.offset((3 as ::core::ffi::c_int * s4x4) as isize) as *mut [int16_t; 2]
                as *mut x264_union128_sse_t))
                .i = M128_ZERO;
            if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
                let mut mv1_0: *mut [int16_t; 2] = &mut *(*(*h)
                    .mb
                    .mv
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(i_mb_4x4 as isize)
                    as *mut [int16_t; 2];
                let mut ref1_0: *mut int8_t = &mut *(*(*h)
                    .mb
                    .ref_0
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(i_mb_8x8 as isize)
                    as *mut int8_t;
                (*(&mut *ref1_0.offset((0 as ::core::ffi::c_int * s8x8) as isize) as *mut int8_t
                    as *mut x264_union16_t))
                    .i = (-(1 as ::core::ffi::c_int) as uint8_t as ::core::ffi::c_int
                    * 0x101 as ::core::ffi::c_int) as uint16_t;
                (*(&mut *ref1_0.offset((1 as ::core::ffi::c_int * s8x8) as isize) as *mut int8_t
                    as *mut x264_union16_t))
                    .i = (-(1 as ::core::ffi::c_int) as uint8_t as ::core::ffi::c_int
                    * 0x101 as ::core::ffi::c_int) as uint16_t;
                (*(&mut *mv1_0.offset((0 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = M128_ZERO;
                (*(&mut *mv1_0.offset((1 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = M128_ZERO;
                (*(&mut *mv1_0.offset((2 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = M128_ZERO;
                (*(&mut *mv1_0.offset((3 as ::core::ffi::c_int * s4x4) as isize)
                    as *mut [int16_t; 2] as *mut x264_union128_sse_t))
                    .i = M128_ZERO;
            }
        }
    }
    if (*h).param.b_cabac != 0 {
        let mut mvd0: *mut [uint8_t; 2] = (*(*(*h)
            .mb
            .mvd
            .as_mut_ptr()
            .offset(0 as ::core::ffi::c_int as isize))
        .offset(i_mb_xy as isize))
        .as_mut_ptr() as *mut [uint8_t; 2];
        if (i_mb_type == I_4x4 as ::core::ffi::c_int
            || i_mb_type == I_8x8 as ::core::ffi::c_int
            || i_mb_type == I_16x16 as ::core::ffi::c_int
            || i_mb_type == I_PCM as ::core::ffi::c_int)
            && i_mb_type != I_PCM as ::core::ffi::c_int
        {
            *(*h).mb.chroma_pred_mode.offset(i_mb_xy as isize) =
                x264_mb_chroma_pred_mode_fix[(*h).mb.i_chroma_pred_mode as usize] as int8_t;
        } else {
            *(*h).mb.chroma_pred_mode.offset(i_mb_xy as isize) =
                I_PRED_CHROMA_DC as ::core::ffi::c_int as int8_t;
        }
        if 0x3ff30 as ::core::ffi::c_int >> i_mb_type & 1 as ::core::ffi::c_int != 0 {
            (*((*mvd0.offset(0 as ::core::ffi::c_int as isize)).as_mut_ptr()
                as *mut x264_union64_t))
                .i = (*((*(*(*h)
                .mb
                .cache
                .mvd
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset(10 as ::core::ffi::c_int as isize) as isize,
            ))
            .as_mut_ptr() as *mut x264_union64_t))
                .i;
            (*((*mvd0.offset(4 as ::core::ffi::c_int as isize)).as_mut_ptr()
                as *mut x264_union16_t))
                .i = (*((*(*(*h)
                .mb
                .cache
                .mvd
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(5 as ::core::ffi::c_int as isize) as isize))
            .as_mut_ptr() as *mut x264_union16_t))
                .i;
            (*((*mvd0.offset(5 as ::core::ffi::c_int as isize)).as_mut_ptr()
                as *mut x264_union16_t))
                .i = (*((*(*(*h)
                .mb
                .cache
                .mvd
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(*x264_scan8.as_ptr().offset(7 as ::core::ffi::c_int as isize) as isize))
            .as_mut_ptr() as *mut x264_union16_t))
                .i;
            (*((*mvd0.offset(6 as ::core::ffi::c_int as isize)).as_mut_ptr()
                as *mut x264_union16_t))
                .i = (*((*(*(*h)
                .mb
                .cache
                .mvd
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset(13 as ::core::ffi::c_int as isize) as isize,
            ))
            .as_mut_ptr() as *mut x264_union16_t))
                .i;
            if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
                let mut mvd1: *mut [uint8_t; 2] = (*(*(*h)
                    .mb
                    .mvd
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(i_mb_xy as isize))
                .as_mut_ptr()
                    as *mut [uint8_t; 2];
                (*((*mvd1.offset(0 as ::core::ffi::c_int as isize)).as_mut_ptr()
                    as *mut x264_union64_t))
                    .i = (*((*(*(*h)
                    .mb
                    .cache
                    .mvd
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    *x264_scan8
                        .as_ptr()
                        .offset(10 as ::core::ffi::c_int as isize) as isize,
                ))
                .as_mut_ptr() as *mut x264_union64_t))
                    .i;
                (*((*mvd1.offset(4 as ::core::ffi::c_int as isize)).as_mut_ptr()
                    as *mut x264_union16_t))
                    .i = (*((*(*(*h)
                    .mb
                    .cache
                    .mvd
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(5 as ::core::ffi::c_int as isize) as isize))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*((*mvd1.offset(5 as ::core::ffi::c_int as isize)).as_mut_ptr()
                    as *mut x264_union16_t))
                    .i = (*((*(*(*h)
                    .mb
                    .cache
                    .mvd
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(*x264_scan8.as_ptr().offset(7 as ::core::ffi::c_int as isize) as isize))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i;
                (*((*mvd1.offset(6 as ::core::ffi::c_int as isize)).as_mut_ptr()
                    as *mut x264_union16_t))
                    .i = (*((*(*(*h)
                    .mb
                    .cache
                    .mvd
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    *x264_scan8
                        .as_ptr()
                        .offset(13 as ::core::ffi::c_int as isize) as isize,
                ))
                .as_mut_ptr() as *mut x264_union16_t))
                    .i;
            }
        } else {
            (*((*mvd0.offset(0 as ::core::ffi::c_int as isize)).as_mut_ptr()
                as *mut x264_union128_sse_t))
                .i = M128_ZERO;
            if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
                let mut mvd1_0: *mut [uint8_t; 2] = (*(*(*h)
                    .mb
                    .mvd
                    .as_mut_ptr()
                    .offset(1 as ::core::ffi::c_int as isize))
                .offset(i_mb_xy as isize))
                .as_mut_ptr()
                    as *mut [uint8_t; 2];
                (*((*mvd1_0.offset(0 as ::core::ffi::c_int as isize)).as_mut_ptr()
                    as *mut x264_union128_sse_t))
                    .i = M128_ZERO;
            }
        }
        if (*h).sh.i_type == SLICE_TYPE_B as ::core::ffi::c_int {
            if i_mb_type == B_SKIP as ::core::ffi::c_int
                || i_mb_type == B_DIRECT as ::core::ffi::c_int
            {
                *(*h).mb.skipbp.offset(i_mb_xy as isize) = 0xf as int8_t;
            } else if i_mb_type == B_8x8 as ::core::ffi::c_int {
                let mut skipbp: ::core::ffi::c_int = (((*h).mb.i_sub_partition
                    [0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    == D_DIRECT_8x8 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int;
                skipbp |= (((*h).mb.i_sub_partition[1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    == D_DIRECT_8x8 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int;
                skipbp |= (((*h).mb.i_sub_partition[2 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    == D_DIRECT_8x8 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int;
                skipbp |= (((*h).mb.i_sub_partition[3 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                    == D_DIRECT_8x8 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int;
                *(*h).mb.skipbp.offset(i_mb_xy as isize) = skipbp as int8_t;
            } else {
                *(*h).mb.skipbp.offset(i_mb_xy as isize) = 0 as int8_t;
            }
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "1883:1"]
pub unsafe extern "C" fn x264_10_macroblock_bipred_init(mut h: *mut x264_t) {
    let mut mbfield: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while mbfield <= (*h).sh.b_mbaff {
        let mut field: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while field <= (*h).sh.b_mbaff {
            let mut i_ref0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_ref0 < (*h).i_ref[0 as ::core::ffi::c_int as usize] << mbfield {
                let mut l0: *mut x264_frame_t =
                    (*h).fref[0 as ::core::ffi::c_int as usize][(i_ref0 >> mbfield) as usize];
                let mut poc0: ::core::ffi::c_int = (*l0).i_poc
                    + mbfield
                        * (*l0).i_delta_poc[(field ^ i_ref0 & 1 as ::core::ffi::c_int) as usize];
                let mut i_ref1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_ref1 < (*h).i_ref[1 as ::core::ffi::c_int as usize] << mbfield {
                    let mut l1: *mut x264_frame_t =
                        (*h).fref[1 as ::core::ffi::c_int as usize][(i_ref1 >> mbfield) as usize];
                    let mut cur_poc: ::core::ffi::c_int =
                        (*(*h).fdec).i_poc + mbfield * (*(*h).fdec).i_delta_poc[field as usize];
                    let mut poc1: ::core::ffi::c_int = (*l1).i_poc
                        + mbfield
                            * (*l1).i_delta_poc
                                [(field ^ i_ref1 & 1 as ::core::ffi::c_int) as usize];
                    let mut td: ::core::ffi::c_int = x264_clip3(
                        poc1 - poc0,
                        -(128 as ::core::ffi::c_int),
                        127 as ::core::ffi::c_int,
                    );
                    if td == 0 as ::core::ffi::c_int {
                        (*h).mb.dist_scale_factor_buf[mbfield as usize][field as usize]
                            [i_ref0 as usize][i_ref1 as usize] = 256 as int16_t;
                        (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                            [i_ref0 as usize][i_ref1 as usize] = 32 as int8_t;
                    } else {
                        let mut tb: ::core::ffi::c_int = x264_clip3(
                            cur_poc - poc0,
                            -(128 as ::core::ffi::c_int),
                            127 as ::core::ffi::c_int,
                        );
                        let mut tx: ::core::ffi::c_int = (16384 as ::core::ffi::c_int
                            + (abs(td) >> 1 as ::core::ffi::c_int))
                            / td;
                        let mut dist_scale_factor: ::core::ffi::c_int = x264_clip3(
                            tb * tx + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int,
                            -(1024 as ::core::ffi::c_int),
                            1023 as ::core::ffi::c_int,
                        );
                        (*h).mb.dist_scale_factor_buf[mbfield as usize][field as usize]
                            [i_ref0 as usize][i_ref1 as usize] = dist_scale_factor as int16_t;
                        dist_scale_factor >>= 2 as ::core::ffi::c_int;
                        if (*h).param.analyse.b_weighted_bipred != 0
                            && dist_scale_factor >= -(64 as ::core::ffi::c_int)
                            && dist_scale_factor <= 128 as ::core::ffi::c_int
                        {
                            (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                                [i_ref0 as usize][i_ref1 as usize] =
                                (64 as ::core::ffi::c_int - dist_scale_factor) as int8_t;
                            if dist_scale_factor >= -(63 as ::core::ffi::c_int)
                                && dist_scale_factor <= 127 as ::core::ffi::c_int
                            {
                            } else {
                                __assert_fail(
                                    b"dist_scale_factor >= -63 && dist_scale_factor <= 127\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    b"common/macroblock.c\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    1918 as ::core::ffi::c_uint,
                                    ::core::mem::transmute::<[u8; 46], [::core::ffi::c_char; 46]>(
                                        *b"void x264_10_macroblock_bipred_init(x264_t *)\0",
                                    )
                                    .as_ptr(),
                                );
                            }
                            'c_40597: {
                                if dist_scale_factor >= -(63 as ::core::ffi::c_int)
                                    && dist_scale_factor <= 127 as ::core::ffi::c_int
                                {
                                } else {
                                    __assert_fail(
                                        b"dist_scale_factor >= -63 && dist_scale_factor <= 127\0"
                                            as *const u8 as *const ::core::ffi::c_char,
                                        b"common/macroblock.c\0" as *const u8
                                            as *const ::core::ffi::c_char,
                                        1918 as ::core::ffi::c_uint,
                                        ::core::mem::transmute::<
                                            [u8; 46],
                                            [::core::ffi::c_char; 46],
                                        >(*b"void x264_10_macroblock_bipred_init(x264_t *)\0")
                                            .as_ptr(),
                                    );
                                }
                            };
                        } else {
                            (*h).mb.bipred_weight_buf[mbfield as usize][field as usize]
                                [i_ref0 as usize][i_ref1 as usize] = 32 as int8_t;
                        }
                    }
                    i_ref1 += 1;
                }
                i_ref0 += 1;
            }
            field += 1;
        }
        mbfield += 1;
    }
}
