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
    use super::types_h::{__int8_t, __int16_t, __int32_t, __int64_t};
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
    use super::types_h::{__uint8_t, __uint16_t, __uint32_t, __uint64_t};
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
    #[c2rust::src_loc = "111:9"]
    pub const SIZEOF_PIXEL: ::core::ffi::c_int = ::core::mem::size_of::<pixel>()
        as ::core::ffi::c_int;
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/bitstream.h:29"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/dct.h:29"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/predict.h:29"]
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
    #[c2rust::src_loc = "95:1"]
    pub type intra8x8_pred_e = ::core::ffi::c_uint;
    #[c2rust::src_loc = "109:5"]
    pub const I_PRED_8x8_DC_128: intra8x8_pred_e = 11;
    #[c2rust::src_loc = "108:5"]
    pub const I_PRED_8x8_DC_TOP: intra8x8_pred_e = 10;
    #[c2rust::src_loc = "107:5"]
    pub const I_PRED_8x8_DC_LEFT: intra8x8_pred_e = 9;
    #[c2rust::src_loc = "105:5"]
    pub const I_PRED_8x8_HU: intra8x8_pred_e = 8;
    #[c2rust::src_loc = "104:5"]
    pub const I_PRED_8x8_VL: intra8x8_pred_e = 7;
    #[c2rust::src_loc = "103:5"]
    pub const I_PRED_8x8_HD: intra8x8_pred_e = 6;
    #[c2rust::src_loc = "102:5"]
    pub const I_PRED_8x8_VR: intra8x8_pred_e = 5;
    #[c2rust::src_loc = "101:5"]
    pub const I_PRED_8x8_DDR: intra8x8_pred_e = 4;
    #[c2rust::src_loc = "100:5"]
    pub const I_PRED_8x8_DDL: intra8x8_pred_e = 3;
    #[c2rust::src_loc = "99:5"]
    pub const I_PRED_8x8_DC: intra8x8_pred_e = 2;
    #[c2rust::src_loc = "98:5"]
    pub const I_PRED_8x8_H: intra8x8_pred_e = 1;
    #[c2rust::src_loc = "97:5"]
    pub const I_PRED_8x8_V: intra8x8_pred_e = 0;
    use super::common_h::pixel;
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
    #[c2rust::src_loc = "177:9"]
    pub const LUMA_DC: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
    #[c2rust::src_loc = "178:9"]
    pub const CHROMA_DC: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
    #[c2rust::src_loc = "180:22"]
    pub static mut x264_scan8: [uint8_t; 51] = [
        (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (4 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (5 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (6 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (7 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
        (0 as ::core::ffi::c_int + 10 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
            as uint8_t,
    ];
    #[inline(always)]
    #[c2rust::src_loc = "206:1"]
    pub unsafe extern "C" fn x264_clip3(
        mut v: ::core::ffi::c_int,
        mut i_min: ::core::ffi::c_int,
        mut i_max: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        return if v < i_min { i_min } else if v > i_max { i_max } else { v };
    }
    use super::stdint_uintn_h::{uint16_t, uint8_t, uint32_t, uint64_t};
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
    #[c2rust::src_loc = "43:22"]
    pub static mut x264_pred_i4x4_neighbors: [uint8_t; 12] = [
        MB_TOP as ::core::ffi::c_int as uint8_t,
        MB_LEFT as ::core::ffi::c_int as uint8_t,
        (MB_LEFT as ::core::ffi::c_int | MB_TOP as ::core::ffi::c_int) as uint8_t,
        (MB_TOP as ::core::ffi::c_int | MB_TOPRIGHT as ::core::ffi::c_int) as uint8_t,
        (MB_LEFT as ::core::ffi::c_int | MB_TOPLEFT as ::core::ffi::c_int
            | MB_TOP as ::core::ffi::c_int) as uint8_t,
        (MB_LEFT as ::core::ffi::c_int | MB_TOPLEFT as ::core::ffi::c_int
            | MB_TOP as ::core::ffi::c_int) as uint8_t,
        (MB_LEFT as ::core::ffi::c_int | MB_TOPLEFT as ::core::ffi::c_int
            | MB_TOP as ::core::ffi::c_int) as uint8_t,
        (MB_TOP as ::core::ffi::c_int | MB_TOPRIGHT as ::core::ffi::c_int) as uint8_t,
        MB_LEFT as ::core::ffi::c_int as uint8_t,
        MB_LEFT as ::core::ffi::c_int as uint8_t,
        MB_TOP as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
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
    #[c2rust::src_loc = "217:22"]
    pub static mut block_idx_xy_1d: [uint8_t; 16] = [
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
    ];
    #[c2rust::src_loc = "221:22"]
    pub static mut block_idx_yx_1d: [uint8_t; 16] = [
        0 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        5 as ::core::ffi::c_int as uint8_t,
        8 as ::core::ffi::c_int as uint8_t,
        12 as ::core::ffi::c_int as uint8_t,
        9 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        6 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        7 as ::core::ffi::c_int as uint8_t,
        10 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        11 as ::core::ffi::c_int as uint8_t,
        15 as ::core::ffi::c_int as uint8_t,
    ];
    #[c2rust::src_loc = "225:22"]
    pub static mut block_idx_xy_fenc: [uint8_t; 16] = [
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE)
            as uint8_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FENC_STRIDE) as uint8_t,
    ];
    #[c2rust::src_loc = "236:23"]
    pub static mut block_idx_xy_fdec: [uint16_t; 16] = [
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
        (3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int
            + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int * FDEC_STRIDE)
            as uint16_t,
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
    use super::stdint_uintn_h::{uint8_t, uint16_t};
    use super::common_h::{FENC_STRIDE, FDEC_STRIDE, pixel, x264_t};
    extern "C" {
        #[c2rust::src_loc = "333:1"]
        pub fn x264_10_copy_column8(dst: *mut pixel, src: *mut pixel);
        #[c2rust::src_loc = "367:1"]
        pub fn x264_10_mb_mc(h: *mut x264_t);
        #[c2rust::src_loc = "369:1"]
        pub fn x264_10_mb_mc_8x8(h: *mut x264_t, i8: ::core::ffi::c_int);
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/osdep.h:29"]
pub mod osdep_h {
    #[c2rust::src_loc = "452:9"]
    pub const WORD_SIZE: uint64_t = ::core::mem::size_of::<*mut ::core::ffi::c_void>()
        as uint64_t;
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
    use super::stdint_uintn_h::{uint64_t, uint32_t, uint8_t};
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
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/tables.h:29"]
pub mod tables_h {
    use super::stdint_uintn_h::uint32_t;
    extern "C" {
        #[c2rust::src_loc = "44:23"]
        pub static x264_lambda2_tab: [::core::ffi::c_int; 82];
        #[c2rust::src_loc = "75:23"]
        pub static x264_dct4_weight2_tab: [uint32_t; 16];
        #[c2rust::src_loc = "76:23"]
        pub static x264_dct8_weight2_tab: [uint32_t; 64];
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/encoder/macroblock.h:29"]
pub mod encoder_macroblock_h {
    #[inline(always)]
    #[c2rust::src_loc = "90:1"]
    pub unsafe extern "C" fn x264_quant_4x4(
        mut h: *mut x264_t,
        mut dct: *mut dctcoef,
        mut i_qp: ::core::ffi::c_int,
        mut ctx_block_cat: ::core::ffi::c_int,
        mut b_intra: ::core::ffi::c_int,
        mut p: ::core::ffi::c_int,
        mut idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut i_quant_cat: ::core::ffi::c_int = if b_intra != 0 {
            if p != 0 {
                CQM_4IC as ::core::ffi::c_int
            } else {
                CQM_4IY as ::core::ffi::c_int
            }
        } else if p != 0 {
            CQM_4PC as ::core::ffi::c_int
        } else {
            CQM_4PY as ::core::ffi::c_int
        };
        if (*h).mb.b_noise_reduction != 0 {
            (*h)
                .quantf
                .denoise_dct
                .expect(
                    "non-null function pointer",
                )(
                dct as *mut dctcoef,
                (*(*h)
                    .nr_residual_sum
                    .offset(
                        (0 as ::core::ffi::c_int
                            + (p != 0) as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .as_mut_ptr(),
                (*(*h)
                    .nr_offset
                    .offset(
                        (0 as ::core::ffi::c_int
                            + (p != 0) as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .as_mut_ptr(),
                16 as ::core::ffi::c_int,
            );
        }
        if (*h).mb.b_trellis != 0 {
            return x264_10_quant_4x4_trellis(
                h,
                dct as *mut dctcoef,
                i_quant_cat,
                i_qp,
                ctx_block_cat,
                b_intra,
                (p != 0) as ::core::ffi::c_int,
                idx + p * 16 as ::core::ffi::c_int,
            )
        } else {
            return (*h)
                .quantf
                .quant_4x4
                .expect(
                    "non-null function pointer",
                )(
                dct,
                (*(*(*h).quant4_mf.as_mut_ptr().offset(i_quant_cat as isize))
                    .offset(i_qp as isize))
                    .as_mut_ptr(),
                (*(*(*h).quant4_bias.as_mut_ptr().offset(i_quant_cat as isize))
                    .offset(i_qp as isize))
                    .as_mut_ptr(),
            )
        };
    }
    #[inline(always)]
    #[c2rust::src_loc = "101:1"]
    pub unsafe extern "C" fn x264_quant_8x8(
        mut h: *mut x264_t,
        mut dct: *mut dctcoef,
        mut i_qp: ::core::ffi::c_int,
        mut ctx_block_cat: ::core::ffi::c_int,
        mut b_intra: ::core::ffi::c_int,
        mut p: ::core::ffi::c_int,
        mut idx: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int {
        let mut i_quant_cat: ::core::ffi::c_int = if b_intra != 0 {
            if p != 0 {
                CQM_8IC as ::core::ffi::c_int
            } else {
                CQM_8IY as ::core::ffi::c_int
            }
        } else if p != 0 {
            CQM_8PC as ::core::ffi::c_int
        } else {
            CQM_8PY as ::core::ffi::c_int
        };
        if (*h).mb.b_noise_reduction != 0 {
            (*h)
                .quantf
                .denoise_dct
                .expect(
                    "non-null function pointer",
                )(
                dct as *mut dctcoef,
                (*(*h)
                    .nr_residual_sum
                    .offset(
                        (1 as ::core::ffi::c_int
                            + (p != 0) as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .as_mut_ptr(),
                (*(*h)
                    .nr_offset
                    .offset(
                        (1 as ::core::ffi::c_int
                            + (p != 0) as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as isize,
                    ))
                    .as_mut_ptr(),
                64 as ::core::ffi::c_int,
            );
        }
        if (*h).mb.b_trellis != 0 {
            return x264_10_quant_8x8_trellis(
                h,
                dct as *mut dctcoef,
                i_quant_cat,
                i_qp,
                ctx_block_cat,
                b_intra,
                (p != 0) as ::core::ffi::c_int,
                idx + p * 4 as ::core::ffi::c_int,
            )
        } else {
            return (*h)
                .quantf
                .quant_8x8
                .expect(
                    "non-null function pointer",
                )(
                dct,
                (*(*(*h).quant8_mf.as_mut_ptr().offset(i_quant_cat as isize))
                    .offset(i_qp as isize))
                    .as_mut_ptr(),
                (*(*(*h).quant8_bias.as_mut_ptr().offset(i_quant_cat as isize))
                    .offset(i_qp as isize))
                    .as_mut_ptr(),
            )
        };
    }
    #[inline(always)]
    #[c2rust::src_loc = "132:1"]
    pub unsafe extern "C" fn x264_mb_encode_i4x4(
        mut h: *mut x264_t,
        mut p: ::core::ffi::c_int,
        mut idx: ::core::ffi::c_int,
        mut i_qp: ::core::ffi::c_int,
        mut i_mode: ::core::ffi::c_int,
        mut b_predict: ::core::ffi::c_int,
    ) {
        let mut nz: ::core::ffi::c_int = 0;
        let mut p_src: *mut pixel = &mut *(*(*h)
            .mb
            .pic
            .p_fenc
            .as_mut_ptr()
            .offset(p as isize))
            .offset(*block_idx_xy_fenc.as_ptr().offset(idx as isize) as isize)
            as *mut pixel;
        let mut p_dst: *mut pixel = &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(p as isize))
            .offset(*block_idx_xy_fdec.as_ptr().offset(idx as isize) as isize)
            as *mut pixel;
        let mut dct4x4: [dctcoef; 16] = [0; 16];
        if b_predict != 0 {
            if (*h).mb.b_lossless != 0 {
                x264_10_predict_lossless_4x4(h, p_dst, p, idx, i_mode);
            } else {
                (*h)
                    .predict_4x4[i_mode as usize]
                    .expect("non-null function pointer")(p_dst);
            }
        }
        if (*h).mb.b_lossless != 0 {
            nz = (*h)
                .zigzagf
                .sub_4x4
                .expect(
                    "non-null function pointer",
                )(
                (*(*h)
                    .dct
                    .luma4x4
                    .as_mut_ptr()
                    .offset((p * 16 as ::core::ffi::c_int + idx) as isize))
                    .as_mut_ptr(),
                p_src,
                p_dst,
            );
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(p * 16 as ::core::ffi::c_int + idx) as usize]
                as usize] = nz as uint8_t;
            (*h).mb.i_cbp_luma |= nz << (idx >> 2 as ::core::ffi::c_int);
            return;
        }
        (*h)
            .dctf
            .sub4x4_dct
            .expect("non-null function pointer")(dct4x4.as_mut_ptr(), p_src, p_dst);
        nz = x264_quant_4x4(
            h,
            dct4x4.as_mut_ptr(),
            i_qp,
            ctx_cat_plane[DCT_LUMA_4x4 as ::core::ffi::c_int as usize][p as usize]
                as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            p,
            idx,
        );
        (*h)
            .mb
            .cache
            .non_zero_count[x264_scan8[(p * 16 as ::core::ffi::c_int + idx) as usize]
            as usize] = nz as uint8_t;
        if nz != 0 {
            (*h).mb.i_cbp_luma
                |= (1 as ::core::ffi::c_int) << (idx >> 2 as ::core::ffi::c_int);
            (*h)
                .zigzagf
                .scan_4x4
                .expect(
                    "non-null function pointer",
                )(
                (*(*h)
                    .dct
                    .luma4x4
                    .as_mut_ptr()
                    .offset((p * 16 as ::core::ffi::c_int + idx) as isize))
                    .as_mut_ptr(),
                dct4x4.as_mut_ptr(),
            );
            (*h)
                .quantf
                .dequant_4x4
                .expect(
                    "non-null function pointer",
                )(
                dct4x4.as_mut_ptr(),
                (*h)
                    .dequant4_mf[(if p != 0 {
                    CQM_4IC as ::core::ffi::c_int
                } else {
                    CQM_4IY as ::core::ffi::c_int
                }) as usize],
                i_qp,
            );
            (*h)
                .dctf
                .add4x4_idct
                .expect("non-null function pointer")(p_dst, dct4x4.as_mut_ptr());
        }
    }
    #[inline(always)]
    #[c2rust::src_loc = "168:1"]
    pub unsafe extern "C" fn x264_mb_encode_i8x8(
        mut h: *mut x264_t,
        mut p: ::core::ffi::c_int,
        mut idx: ::core::ffi::c_int,
        mut i_qp: ::core::ffi::c_int,
        mut i_mode: ::core::ffi::c_int,
        mut edge: *mut pixel,
        mut b_predict: ::core::ffi::c_int,
    ) {
        let mut x: ::core::ffi::c_int = idx & 1 as ::core::ffi::c_int;
        let mut y: ::core::ffi::c_int = idx >> 1 as ::core::ffi::c_int;
        let mut nz: ::core::ffi::c_int = 0;
        let mut p_src: *mut pixel = &mut *(*(*h)
            .mb
            .pic
            .p_fenc
            .as_mut_ptr()
            .offset(p as isize))
            .offset(
                (8 as ::core::ffi::c_int * x + 8 as ::core::ffi::c_int * y * FENC_STRIDE)
                    as isize,
            ) as *mut pixel;
        let mut p_dst: *mut pixel = &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(p as isize))
            .offset(
                (8 as ::core::ffi::c_int * x + 8 as ::core::ffi::c_int * y * FDEC_STRIDE)
                    as isize,
            ) as *mut pixel;
        let mut dct8x8: [dctcoef; 64] = [0; 64];
        let mut edge_buf: [pixel; 36] = [0; 36];
        if b_predict != 0 {
            if edge.is_null() {
                (*h)
                    .predict_8x8_filter
                    .expect(
                        "non-null function pointer",
                    )(
                    p_dst,
                    edge_buf.as_mut_ptr(),
                    (*h).mb.i_neighbour8[idx as usize] as ::core::ffi::c_int,
                    x264_pred_i4x4_neighbors[i_mode as usize] as ::core::ffi::c_int,
                );
                edge = edge_buf.as_mut_ptr();
            }
            if (*h).mb.b_lossless != 0 {
                x264_10_predict_lossless_8x8(
                    h,
                    p_dst,
                    p,
                    idx,
                    i_mode,
                    edge as *mut pixel,
                );
            } else {
                (*h)
                    .predict_8x8[i_mode as usize]
                    .expect("non-null function pointer")(p_dst, edge as *mut pixel);
            }
        }
        if (*h).mb.b_lossless != 0 {
            nz = (*h)
                .zigzagf
                .sub_8x8
                .expect(
                    "non-null function pointer",
                )(
                (*(*h)
                    .dct
                    .luma8x8
                    .as_mut_ptr()
                    .offset((p * 4 as ::core::ffi::c_int + idx) as isize))
                    .as_mut_ptr(),
                p_src,
                p_dst,
            );
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset(
                            (p * 16 as ::core::ffi::c_int
                                + idx * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                .i = (nz * 0x101 as ::core::ffi::c_int) as uint16_t;
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset(
                            (p * 16 as ::core::ffi::c_int
                                + idx * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                .i = (nz * 0x101 as ::core::ffi::c_int) as uint16_t;
            (*h).mb.i_cbp_luma |= nz << idx;
            return;
        }
        (*h)
            .dctf
            .sub8x8_dct8
            .expect("non-null function pointer")(dct8x8.as_mut_ptr(), p_src, p_dst);
        nz = x264_quant_8x8(
            h,
            dct8x8.as_mut_ptr(),
            i_qp,
            ctx_cat_plane[DCT_LUMA_8x8 as ::core::ffi::c_int as usize][p as usize]
                as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            p,
            idx,
        );
        if nz != 0 {
            (*h).mb.i_cbp_luma |= (1 as ::core::ffi::c_int) << idx;
            (*h)
                .zigzagf
                .scan_8x8
                .expect(
                    "non-null function pointer",
                )(
                (*(*h)
                    .dct
                    .luma8x8
                    .as_mut_ptr()
                    .offset((p * 4 as ::core::ffi::c_int + idx) as isize))
                    .as_mut_ptr(),
                dct8x8.as_mut_ptr(),
            );
            (*h)
                .quantf
                .dequant_8x8
                .expect(
                    "non-null function pointer",
                )(
                dct8x8.as_mut_ptr(),
                (*h)
                    .dequant8_mf[(if p != 0 {
                    CQM_8IC as ::core::ffi::c_int
                } else {
                    CQM_8IY as ::core::ffi::c_int
                }) as usize],
                i_qp,
            );
            (*h)
                .dctf
                .add8x8_idct8
                .expect("non-null function pointer")(p_dst, dct8x8.as_mut_ptr());
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset(
                            (p * 16 as ::core::ffi::c_int
                                + idx * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                .i = (1 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int) as uint16_t;
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset(
                            (p * 16 as ::core::ffi::c_int
                                + idx * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                .i = (1 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int) as uint16_t;
        } else {
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset(
                            (p * 16 as ::core::ffi::c_int
                                + idx * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int) as uint16_t;
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    (*x264_scan8
                        .as_ptr()
                        .offset(
                            (p * 16 as ::core::ffi::c_int
                                + idx * 4 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                ) as *mut uint8_t as *mut x264_union16_t))
                .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int) as uint16_t;
        };
    }
    use super::common_h::{x264_t, dctcoef, pixel, FENC_STRIDE, FDEC_STRIDE};
    use super::set_h::{
        CQM_8IC, CQM_8IY, CQM_8PC, CQM_8PY, CQM_4IC, CQM_4IY, CQM_4PC, CQM_4PY,
    };
    use super::macroblock_h::{
        block_idx_xy_fenc, block_idx_xy_fdec, DCT_LUMA_4x4, ctx_cat_plane,
        x264_pred_i4x4_neighbors, DCT_LUMA_8x8,
    };
    use super::{x264_10_predict_lossless_4x4, x264_10_predict_lossless_8x8};
    use super::stdint_uintn_h::{uint8_t, uint16_t};
    use super::base_h::{x264_scan8, x264_union16_t};
    extern "C" {
        #[c2rust::src_loc = "76:1"]
        pub fn x264_10_quant_luma_dc_trellis(
            h: *mut x264_t,
            dct: *mut dctcoef,
            i_quant_cat: ::core::ffi::c_int,
            i_qp: ::core::ffi::c_int,
            ctx_block_cat: ::core::ffi::c_int,
            b_intra: ::core::ffi::c_int,
            idx: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "79:1"]
        pub fn x264_10_quant_chroma_dc_trellis(
            h: *mut x264_t,
            dct: *mut dctcoef,
            i_qp: ::core::ffi::c_int,
            b_intra: ::core::ffi::c_int,
            idx: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "81:1"]
        pub fn x264_10_quant_4x4_trellis(
            h: *mut x264_t,
            dct: *mut dctcoef,
            i_quant_cat: ::core::ffi::c_int,
            i_qp: ::core::ffi::c_int,
            ctx_block_cat: ::core::ffi::c_int,
            b_intra: ::core::ffi::c_int,
            b_chroma: ::core::ffi::c_int,
            idx: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
        #[c2rust::src_loc = "84:1"]
        pub fn x264_10_quant_8x8_trellis(
            h: *mut x264_t,
            dct: *mut dctcoef,
            i_quant_cat: ::core::ffi::c_int,
            i_qp: ::core::ffi::c_int,
            ctx_block_cat: ::core::ffi::c_int,
            b_intra: ::core::ffi::c_int,
            b_chroma: ::core::ffi::c_int,
            idx: ::core::ffi::c_int,
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:29"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
pub use self::internal::__va_list_tag;
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
    C2RustUnnamed_12, C2RustUnnamed_13, C2RustUnnamed_17, C2RustUnnamed_18, SIZEOF_PIXEL,
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
    x264_predict_8x8_filter_t, x264_predict_t, x264_predict8x8_t, intra_chroma_pred_e,
    I_PRED_CHROMA_DC_128, I_PRED_CHROMA_DC_TOP, I_PRED_CHROMA_DC_LEFT, I_PRED_CHROMA_P,
    I_PRED_CHROMA_V, I_PRED_CHROMA_H, I_PRED_CHROMA_DC, intra16x16_pred_e,
    I_PRED_16x16_DC_128, I_PRED_16x16_DC_TOP, I_PRED_16x16_DC_LEFT, I_PRED_16x16_P,
    I_PRED_16x16_DC, I_PRED_16x16_H, I_PRED_16x16_V, intra4x4_pred_e, I_PRED_4x4_DC_128,
    I_PRED_4x4_DC_TOP, I_PRED_4x4_DC_LEFT, I_PRED_4x4_HU, I_PRED_4x4_VL, I_PRED_4x4_HD,
    I_PRED_4x4_VR, I_PRED_4x4_DDR, I_PRED_4x4_DDL, I_PRED_4x4_DC, I_PRED_4x4_H,
    I_PRED_4x4_V, intra8x8_pred_e, I_PRED_8x8_DC_128, I_PRED_8x8_DC_TOP,
    I_PRED_8x8_DC_LEFT, I_PRED_8x8_HU, I_PRED_8x8_VL, I_PRED_8x8_HD, I_PRED_8x8_VR,
    I_PRED_8x8_DDR, I_PRED_8x8_DDL, I_PRED_8x8_DC, I_PRED_8x8_H, I_PRED_8x8_V,
};
pub use self::set_h::{
    x264_pps_t, x264_sps_t, C2RustUnnamed_14, C2RustUnnamed_15, C2RustUnnamed_16, cqm4_e,
    CQM_4PC, CQM_4IC, CQM_4PY, CQM_4IY, cqm8_e, CQM_8PC, CQM_8IC, CQM_8PY, CQM_8IY,
};
use self::threadpool_h::x264_threadpool_t;
pub use self::base_h::{
    x264_union16_t, x264_union32_t, x264_union64_t, chroma_format_e, CHROMA_444,
    CHROMA_422, CHROMA_420, CHROMA_400, LUMA_DC, CHROMA_DC, x264_scan8, x264_clip3,
};
pub use self::macroblock_h::{
    macroblock_position_e, ALL_NEIGHBORS, MB_PRIVATE, MB_TOPLEFT, MB_TOPRIGHT, MB_TOP,
    MB_LEFT, mb_class_e, X264_MBTYPE_MAX, B_SKIP, B_8x8, B_BI_BI, B_BI_L1, B_BI_L0,
    B_L1_BI, B_L1_L1, B_L1_L0, B_L0_BI, B_L0_L1, B_L0_L0, B_DIRECT, P_SKIP, P_8x8, P_L0,
    I_PCM, I_16x16, I_8x8, I_4x4, mb_partition_e, X264_PARTTYPE_MAX, D_16x16, D_8x16,
    D_16x8, D_8x8, D_DIRECT_8x8, D_BI_8x8, D_BI_4x8, D_BI_8x4, D_BI_4x4, D_L1_8x8,
    D_L1_4x8, D_L1_8x4, D_L1_4x4, D_L0_8x8, D_L0_4x8, D_L0_8x4, D_L0_4x4,
    cabac_ctx_block_cat_e, DCT_CHROMAV_8x8, DCT_CHROMAV_4x4, DCT_CHROMAV_AC,
    DCT_CHROMAV_DC, DCT_CHROMAU_8x8, DCT_CHROMAU_4x4, DCT_CHROMAU_AC, DCT_CHROMAU_DC,
    DCT_LUMA_8x8, DCT_CHROMA_AC, DCT_CHROMA_DC, DCT_LUMA_4x4, DCT_LUMA_AC, DCT_LUMA_DC,
    x264_pred_i4x4_neighbors, block_idx_x, block_idx_y, block_idx_xy_1d, block_idx_yx_1d,
    block_idx_xy_fenc, block_idx_xy_fdec, ctx_cat_plane, x264_10_copy_column8,
    x264_10_mb_mc, x264_10_mb_mc_8x8,
};
pub use self::osdep_h::{WORD_SIZE, x264_ctz_4bit};
use self::string_h::memcpy;
use self::tables_h::{x264_lambda2_tab, x264_dct4_weight2_tab, x264_dct8_weight2_tab};
pub use self::encoder_macroblock_h::{
    x264_quant_4x4, x264_quant_8x8, x264_mb_encode_i4x4, x264_mb_encode_i8x8,
    x264_10_quant_luma_dc_trellis, x264_10_quant_chroma_dc_trellis,
    x264_10_quant_4x4_trellis, x264_10_quant_8x8_trellis,
};
pub use self::__stddef_null_h::NULL;
#[inline]
#[c2rust::src_loc = "35:1"]
unsafe extern "C" fn zigzag_scan_2x2_dc(mut level: *mut dctcoef, mut dct: *mut dctcoef) {
    *level.offset(0 as ::core::ffi::c_int as isize) = *dct
        .offset(
            (0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as isize,
        );
    *level.offset(1 as ::core::ffi::c_int as isize) = *dct
        .offset(
            (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 0 as ::core::ffi::c_int)
                as isize,
        );
    *level.offset(2 as ::core::ffi::c_int as isize) = *dct
        .offset(
            (0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as isize,
        );
    *level.offset(3 as ::core::ffi::c_int as isize) = *dct
        .offset(
            (1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as isize,
        );
}
#[inline]
#[c2rust::src_loc = "44:1"]
unsafe extern "C" fn zigzag_scan_2x4_dc(mut level: *mut dctcoef, mut dct: *mut dctcoef) {
    *level.offset(0 as ::core::ffi::c_int as isize) = *dct
        .offset(0 as ::core::ffi::c_int as isize);
    *level.offset(1 as ::core::ffi::c_int as isize) = *dct
        .offset(2 as ::core::ffi::c_int as isize);
    *level.offset(2 as ::core::ffi::c_int as isize) = *dct
        .offset(1 as ::core::ffi::c_int as isize);
    *level.offset(3 as ::core::ffi::c_int as isize) = *dct
        .offset(4 as ::core::ffi::c_int as isize);
    *level.offset(4 as ::core::ffi::c_int as isize) = *dct
        .offset(6 as ::core::ffi::c_int as isize);
    *level.offset(5 as ::core::ffi::c_int as isize) = *dct
        .offset(3 as ::core::ffi::c_int as isize);
    *level.offset(6 as ::core::ffi::c_int as isize) = *dct
        .offset(5 as ::core::ffi::c_int as isize);
    *level.offset(7 as ::core::ffi::c_int as isize) = *dct
        .offset(7 as ::core::ffi::c_int as isize);
}
#[inline]
#[c2rust::src_loc = "63:1"]
unsafe extern "C" fn idct_dequant_2x2_dc(
    mut dct: *mut dctcoef,
    mut dct4x4: *mut [dctcoef; 16],
    mut dequant_mf: *mut [::core::ffi::c_int; 16],
    mut i_qp: ::core::ffi::c_int,
) {
    let mut d0: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
        + *dct.offset(1 as ::core::ffi::c_int as isize);
    let mut d1: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
        + *dct.offset(3 as ::core::ffi::c_int as isize);
    let mut d2: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
        - *dct.offset(1 as ::core::ffi::c_int as isize);
    let mut d3: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
        - *dct.offset(3 as ::core::ffi::c_int as isize);
    let mut dmf: ::core::ffi::c_int = (*dequant_mf
        .offset(
            (i_qp % 6 as ::core::ffi::c_int) as isize,
        ))[0 as ::core::ffi::c_int as usize] << i_qp / 6 as ::core::ffi::c_int;
    (*dct4x4
        .offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] = ((d0
        + d1) * dmf >> 5 as ::core::ffi::c_int) as dctcoef;
    (*dct4x4
        .offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] = ((d0
        - d1) * dmf >> 5 as ::core::ffi::c_int) as dctcoef;
    (*dct4x4
        .offset(2 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] = ((d2
        + d3) * dmf >> 5 as ::core::ffi::c_int) as dctcoef;
    (*dct4x4
        .offset(3 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] = ((d2
        - d3) * dmf >> 5 as ::core::ffi::c_int) as dctcoef;
}
#[inline]
#[c2rust::src_loc = "72:1"]
unsafe extern "C" fn idct_dequant_2x2_dconly(
    mut dct: *mut dctcoef,
    mut dequant_mf: *mut [::core::ffi::c_int; 16],
    mut i_qp: ::core::ffi::c_int,
) {
    let mut d0: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
        + *dct.offset(1 as ::core::ffi::c_int as isize);
    let mut d1: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
        + *dct.offset(3 as ::core::ffi::c_int as isize);
    let mut d2: ::core::ffi::c_int = *dct.offset(0 as ::core::ffi::c_int as isize)
        - *dct.offset(1 as ::core::ffi::c_int as isize);
    let mut d3: ::core::ffi::c_int = *dct.offset(2 as ::core::ffi::c_int as isize)
        - *dct.offset(3 as ::core::ffi::c_int as isize);
    let mut dmf: ::core::ffi::c_int = (*dequant_mf
        .offset(
            (i_qp % 6 as ::core::ffi::c_int) as isize,
        ))[0 as ::core::ffi::c_int as usize] << i_qp / 6 as ::core::ffi::c_int;
    *dct.offset(0 as ::core::ffi::c_int as isize) = ((d0 + d1) * dmf
        >> 5 as ::core::ffi::c_int) as dctcoef;
    *dct.offset(1 as ::core::ffi::c_int as isize) = ((d0 - d1) * dmf
        >> 5 as ::core::ffi::c_int) as dctcoef;
    *dct.offset(2 as ::core::ffi::c_int as isize) = ((d2 + d3) * dmf
        >> 5 as ::core::ffi::c_int) as dctcoef;
    *dct.offset(3 as ::core::ffi::c_int as isize) = ((d2 - d3) * dmf
        >> 5 as ::core::ffi::c_int) as dctcoef;
}
#[inline]
#[c2rust::src_loc = "82:1"]
unsafe extern "C" fn dct2x2dc(mut d: *mut dctcoef, mut dct4x4: *mut [dctcoef; 16]) {
    let mut d0: ::core::ffi::c_int = (*dct4x4
        .offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
        + (*dct4x4
            .offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize];
    let mut d1: ::core::ffi::c_int = (*dct4x4
        .offset(2 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
        + (*dct4x4
            .offset(3 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize];
    let mut d2: ::core::ffi::c_int = (*dct4x4
        .offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
        - (*dct4x4
            .offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize];
    let mut d3: ::core::ffi::c_int = (*dct4x4
        .offset(2 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize]
        - (*dct4x4
            .offset(3 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize];
    *d.offset(0 as ::core::ffi::c_int as isize) = (d0 + d1) as dctcoef;
    *d.offset(2 as ::core::ffi::c_int as isize) = (d2 + d3) as dctcoef;
    *d.offset(1 as ::core::ffi::c_int as isize) = (d0 - d1) as dctcoef;
    *d.offset(3 as ::core::ffi::c_int as isize) = (d2 - d3) as dctcoef;
    (*dct4x4
        .offset(0 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] = 0
        as ::core::ffi::c_int as dctcoef;
    (*dct4x4
        .offset(1 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] = 0
        as ::core::ffi::c_int as dctcoef;
    (*dct4x4
        .offset(2 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] = 0
        as ::core::ffi::c_int as dctcoef;
    (*dct4x4
        .offset(3 as ::core::ffi::c_int as isize))[0 as ::core::ffi::c_int as usize] = 0
        as ::core::ffi::c_int as dctcoef;
}
#[inline(always)]
#[c2rust::src_loc = "98:1"]
unsafe extern "C" fn array_non_zero(
    mut v: *mut dctcoef,
    mut i_count: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if WORD_SIZE == 8 as uint64_t {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < i_count {
            if (*(&mut *v.offset(i as isize) as *mut dctcoef as *mut x264_union64_t)).i
                != 0
            {
                return 1 as ::core::ffi::c_int;
            }
            i = (i as ::core::ffi::c_ulong)
                .wrapping_add(
                    (8 as usize).wrapping_div(::core::mem::size_of::<dctcoef>() as usize)
                        as ::core::ffi::c_ulong,
                ) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
    } else {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < i_count {
            if (*(&mut *v.offset(i_0 as isize) as *mut dctcoef as *mut x264_union32_t)).i
                != 0
            {
                return 1 as ::core::ffi::c_int;
            }
            i_0 = (i_0 as ::core::ffi::c_ulong)
                .wrapping_add(
                    (4 as usize).wrapping_div(::core::mem::size_of::<dctcoef>() as usize)
                        as ::core::ffi::c_ulong,
                ) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[c2rust::src_loc = "126:1"]
unsafe extern "C" fn mb_encode_i16x16(
    mut h: *mut x264_t,
    mut p: ::core::ffi::c_int,
    mut i_qp: ::core::ffi::c_int,
) {
    let mut p_src: *mut pixel = (*h).mb.pic.p_fenc[p as usize];
    let mut p_dst: *mut pixel = (*h).mb.pic.p_fdec[p as usize];
    let mut dct4x4: [[dctcoef; 16]; 16] = [[0; 16]; 16];
    let mut dct_dc4x4: [dctcoef; 16] = [0; 16];
    let mut nz: ::core::ffi::c_int = 0;
    let mut block_cbp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut decimate_score: ::core::ffi::c_int = if (*h).mb.b_dct_decimate != 0 {
        0 as ::core::ffi::c_int
    } else {
        9 as ::core::ffi::c_int
    };
    let mut i_quant_cat: ::core::ffi::c_int = if p != 0 {
        CQM_4IC as ::core::ffi::c_int
    } else {
        CQM_4IY as ::core::ffi::c_int
    };
    let mut i_mode: ::core::ffi::c_int = (*h).mb.i_intra16x16_pred_mode;
    if (*h).mb.b_lossless != 0 {
        x264_10_predict_lossless_16x16(h, p, i_mode);
    } else {
        (*h)
            .predict_16x16[i_mode as usize]
            .expect("non-null function pointer")((*h).mb.pic.p_fdec[p as usize]);
    }
    if (*h).mb.b_lossless != 0 {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            let mut oe: ::core::ffi::c_int = block_idx_xy_fenc[i as usize]
                as ::core::ffi::c_int;
            let mut od: ::core::ffi::c_int = block_idx_xy_fdec[i as usize]
                as ::core::ffi::c_int;
            nz = (*h)
                .zigzagf
                .sub_4x4ac
                .expect(
                    "non-null function pointer",
                )(
                (*(*h)
                    .dct
                    .luma4x4
                    .as_mut_ptr()
                    .offset((16 as ::core::ffi::c_int * p + i) as isize))
                    .as_mut_ptr(),
                p_src.offset(oe as isize),
                p_dst.offset(od as isize),
                &mut *dct_dc4x4
                    .as_mut_ptr()
                    .offset(*block_idx_yx_1d.as_ptr().offset(i as isize) as isize),
            );
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(16 as ::core::ffi::c_int * p + i) as usize]
                as usize] = nz as uint8_t;
            block_cbp |= nz;
            i += 1;
        }
        (*h).mb.i_cbp_luma |= block_cbp * 0xf as ::core::ffi::c_int;
        (*h).mb.cache.non_zero_count[x264_scan8[(LUMA_DC + p) as usize] as usize] = array_non_zero(
            dct_dc4x4.as_mut_ptr(),
            16 as ::core::ffi::c_int,
        ) as uint8_t;
        (*h)
            .zigzagf
            .scan_4x4
            .expect(
                "non-null function pointer",
            )(
            (*(*h).dct.luma16x16_dc.as_mut_ptr().offset(p as isize)).as_mut_ptr(),
            dct_dc4x4.as_mut_ptr(),
        );
        return;
    }
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset((16 as ::core::ffi::c_int * p) as isize)
                as ::core::ffi::c_int
                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset((16 as ::core::ffi::c_int * p) as isize)
                as ::core::ffi::c_int
                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset((16 as ::core::ffi::c_int * p) as isize)
                as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(
            (*x264_scan8.as_ptr().offset((16 as ::core::ffi::c_int * p) as isize)
                as ::core::ffi::c_int
                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    (*h)
        .dctf
        .sub16x16_dct
        .expect("non-null function pointer")(dct4x4.as_mut_ptr(), p_src, p_dst);
    if (*h).mb.b_noise_reduction != 0 {
        let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while idx < 16 as ::core::ffi::c_int {
            (*h)
                .quantf
                .denoise_dct
                .expect(
                    "non-null function pointer",
                )(
                (*dct4x4.as_mut_ptr().offset(idx as isize)).as_mut_ptr(),
                (*(*h).nr_residual_sum.offset(0 as ::core::ffi::c_int as isize))
                    .as_mut_ptr(),
                (*(*h).nr_offset.offset(0 as ::core::ffi::c_int as isize)).as_mut_ptr(),
                16 as ::core::ffi::c_int,
            );
            idx += 1;
        }
    }
    let mut idx_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while idx_0 < 16 as ::core::ffi::c_int {
        dct_dc4x4[block_idx_xy_1d[idx_0 as usize] as usize] = dct4x4[idx_0
            as usize][0 as ::core::ffi::c_int as usize];
        dct4x4[idx_0 as usize][0 as ::core::ffi::c_int as usize] = 0
            as ::core::ffi::c_int as dctcoef;
        idx_0 += 1;
    }
    if (*h).mb.b_trellis != 0 {
        let mut idx_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while idx_1 < 16 as ::core::ffi::c_int {
            if x264_10_quant_4x4_trellis(
                h,
                (*dct4x4.as_mut_ptr().offset(idx_1 as isize)).as_mut_ptr(),
                i_quant_cat,
                i_qp,
                ctx_cat_plane[DCT_LUMA_AC as ::core::ffi::c_int as usize][p as usize]
                    as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
                (p != 0) as ::core::ffi::c_int,
                idx_1,
            ) != 0
            {
                block_cbp = 0xf as ::core::ffi::c_int;
                (*h)
                    .zigzagf
                    .scan_4x4
                    .expect(
                        "non-null function pointer",
                    )(
                    (*(*h)
                        .dct
                        .luma4x4
                        .as_mut_ptr()
                        .offset((16 as ::core::ffi::c_int * p + idx_1) as isize))
                        .as_mut_ptr(),
                    (*dct4x4.as_mut_ptr().offset(idx_1 as isize)).as_mut_ptr(),
                );
                (*h)
                    .quantf
                    .dequant_4x4
                    .expect(
                        "non-null function pointer",
                    )(
                    (*dct4x4.as_mut_ptr().offset(idx_1 as isize)).as_mut_ptr(),
                    (*h).dequant4_mf[i_quant_cat as usize],
                    i_qp,
                );
                if decimate_score < 6 as ::core::ffi::c_int {
                    decimate_score
                        += (*h)
                            .quantf
                            .decimate_score15
                            .expect(
                                "non-null function pointer",
                            )(
                            (*(*h)
                                .dct
                                .luma4x4
                                .as_mut_ptr()
                                .offset((16 as ::core::ffi::c_int * p + idx_1) as isize))
                                .as_mut_ptr(),
                        );
                }
                (*h)
                    .mb
                    .cache
                    .non_zero_count[x264_scan8[(16 as ::core::ffi::c_int * p + idx_1)
                    as usize] as usize] = 1 as uint8_t;
            }
            idx_1 += 1;
        }
    } else {
        let mut i8x8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i8x8 < 4 as ::core::ffi::c_int {
            nz = (*h)
                .quantf
                .quant_4x4x4
                .expect(
                    "non-null function pointer",
                )(
                &mut *dct4x4
                    .as_mut_ptr()
                    .offset((i8x8 * 4 as ::core::ffi::c_int) as isize),
                (*(*(*h).quant4_mf.as_mut_ptr().offset(i_quant_cat as isize))
                    .offset(i_qp as isize))
                    .as_mut_ptr(),
                (*(*(*h).quant4_bias.as_mut_ptr().offset(i_quant_cat as isize))
                    .offset(i_qp as isize))
                    .as_mut_ptr(),
            );
            if nz != 0 {
                block_cbp = 0xf as ::core::ffi::c_int;
                let mut idx_2: ::core::ffi::c_int = i8x8 * 4 as ::core::ffi::c_int;
                let mut msk: ::core::ffi::c_int = nz;
                let mut skip: ::core::ffi::c_int = 0;
                while msk != 0
                    && {
                        skip = x264_ctz_4bit(msk as uint32_t);
                        idx_2 += skip;
                        msk >>= skip + 1 as ::core::ffi::c_int;
                        1 as ::core::ffi::c_int != 0
                    }
                {
                    (*h)
                        .zigzagf
                        .scan_4x4
                        .expect(
                            "non-null function pointer",
                        )(
                        (*(*h)
                            .dct
                            .luma4x4
                            .as_mut_ptr()
                            .offset((16 as ::core::ffi::c_int * p + idx_2) as isize))
                            .as_mut_ptr(),
                        (*dct4x4.as_mut_ptr().offset(idx_2 as isize)).as_mut_ptr(),
                    );
                    (*h)
                        .quantf
                        .dequant_4x4
                        .expect(
                            "non-null function pointer",
                        )(
                        (*dct4x4.as_mut_ptr().offset(idx_2 as isize)).as_mut_ptr(),
                        (*h).dequant4_mf[i_quant_cat as usize],
                        i_qp,
                    );
                    if decimate_score < 6 as ::core::ffi::c_int {
                        decimate_score
                            += (*h)
                                .quantf
                                .decimate_score15
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*(*h)
                                    .dct
                                    .luma4x4
                                    .as_mut_ptr()
                                    .offset((16 as ::core::ffi::c_int * p + idx_2) as isize))
                                    .as_mut_ptr(),
                            );
                    }
                    (*h)
                        .mb
                        .cache
                        .non_zero_count[x264_scan8[(16 as ::core::ffi::c_int * p + idx_2)
                        as usize] as usize] = 1 as uint8_t;
                    idx_2 += 1;
                }
            }
            i8x8 += 1;
        }
    }
    if decimate_score < 6 as ::core::ffi::c_int {
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset((16 as ::core::ffi::c_int * p) as isize)
                    as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as uint32_t;
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset((16 as ::core::ffi::c_int * p) as isize)
                    as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as uint32_t;
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset((16 as ::core::ffi::c_int * p) as isize)
                    as ::core::ffi::c_int
                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as uint32_t;
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                (*x264_scan8.as_ptr().offset((16 as ::core::ffi::c_int * p) as isize)
                    as ::core::ffi::c_int
                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as uint32_t;
        block_cbp = 0 as ::core::ffi::c_int;
    } else {
        (*h).mb.i_cbp_luma |= block_cbp;
    }
    (*h).dctf.dct4x4dc.expect("non-null function pointer")(dct_dc4x4.as_mut_ptr());
    if (*h).mb.b_trellis != 0 {
        nz = x264_10_quant_luma_dc_trellis(
            h,
            dct_dc4x4.as_mut_ptr(),
            i_quant_cat,
            i_qp,
            ctx_cat_plane[DCT_LUMA_DC as ::core::ffi::c_int as usize][p as usize]
                as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            LUMA_DC + p,
        );
    } else {
        nz = (*h)
            .quantf
            .quant_4x4_dc
            .expect(
                "non-null function pointer",
            )(
            dct_dc4x4.as_mut_ptr(),
            ((*(*h)
                .quant4_mf[i_quant_cat as usize]
                .offset(i_qp as isize))[0 as ::core::ffi::c_int as usize]
                >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
            ((*(*h)
                .quant4_bias[i_quant_cat as usize]
                .offset(i_qp as isize))[0 as ::core::ffi::c_int as usize]
                << 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
    }
    (*h).mb.cache.non_zero_count[x264_scan8[(LUMA_DC + p) as usize] as usize] = nz
        as uint8_t;
    if nz != 0 {
        (*h)
            .zigzagf
            .scan_4x4
            .expect(
                "non-null function pointer",
            )(
            (*(*h).dct.luma16x16_dc.as_mut_ptr().offset(p as isize)).as_mut_ptr(),
            dct_dc4x4.as_mut_ptr(),
        );
        (*h).dctf.idct4x4dc.expect("non-null function pointer")(dct_dc4x4.as_mut_ptr());
        (*h)
            .quantf
            .dequant_4x4_dc
            .expect(
                "non-null function pointer",
            )(dct_dc4x4.as_mut_ptr(), (*h).dequant4_mf[i_quant_cat as usize], i_qp);
        if block_cbp != 0 {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 16 as ::core::ffi::c_int {
                dct4x4[i_0 as usize][0 as ::core::ffi::c_int as usize] = dct_dc4x4[block_idx_xy_1d[i_0
                    as usize] as usize];
                i_0 += 1;
            }
        }
    }
    if block_cbp != 0 {
        (*h)
            .dctf
            .add16x16_idct
            .expect("non-null function pointer")(p_dst, dct4x4.as_mut_ptr());
    } else if nz != 0 {
        (*h)
            .dctf
            .add16x16_idct_dc
            .expect("non-null function pointer")(p_dst, dct_dc4x4.as_mut_ptr());
    }
}
#[inline(always)]
#[c2rust::src_loc = "245:1"]
unsafe extern "C" fn mb_optimize_chroma_dc(
    mut h: *mut x264_t,
    mut dct_dc: *mut dctcoef,
    mut dequant_mf: *mut [::core::ffi::c_int; 16],
    mut i_qp: ::core::ffi::c_int,
    mut chroma422: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dmf: ::core::ffi::c_int = (*dequant_mf
        .offset(
            (i_qp % 6 as ::core::ffi::c_int) as isize,
        ))[0 as ::core::ffi::c_int as usize] << i_qp / 6 as ::core::ffi::c_int;
    if dmf > 32 as ::core::ffi::c_int * 64 as ::core::ffi::c_int {
        return 1 as ::core::ffi::c_int;
    }
    if chroma422 != 0 {
        return (*h)
            .quantf
            .optimize_chroma_2x4_dc
            .expect("non-null function pointer")(dct_dc as *mut dctcoef, dmf)
    } else {
        return (*h)
            .quantf
            .optimize_chroma_2x2_dc
            .expect("non-null function pointer")(dct_dc as *mut dctcoef, dmf)
    };
}
#[inline(always)]
#[c2rust::src_loc = "259:1"]
unsafe extern "C" fn mb_encode_chroma_internal(
    mut h: *mut x264_t,
    mut b_inter: ::core::ffi::c_int,
    mut i_qp: ::core::ffi::c_int,
    mut chroma422: ::core::ffi::c_int,
) {
    let mut nz: ::core::ffi::c_int = 0;
    let mut nz_dc: ::core::ffi::c_int = 0;
    let mut b_decimate: ::core::ffi::c_int = (b_inter != 0
        && (*h).mb.b_dct_decimate != 0) as ::core::ffi::c_int;
    let mut dequant_mf: *mut [::core::ffi::c_int; 16] = (*h)
        .dequant4_mf[(CQM_4IC as ::core::ffi::c_int + b_inter) as usize];
    let mut dct_dc: [dctcoef; 8] = [0; 8];
    (*h).mb.i_cbp_chroma = 0 as ::core::ffi::c_int;
    let ref mut fresh2 = *(*h).nr_count.offset(2 as ::core::ffi::c_int as isize);
    *fresh2 = (*fresh2)
        .wrapping_add(((*h).mb.b_noise_reduction * 4 as ::core::ffi::c_int) as uint32_t);
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(16 as ::core::ffi::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as uint16_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(18 as ::core::ffi::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as uint16_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(32 as ::core::ffi::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as uint16_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(34 as ::core::ffi::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union16_t))
        .i = 0 as uint16_t;
    if chroma422 != 0 {
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                *x264_scan8.as_ptr().offset(24 as ::core::ffi::c_int as isize) as isize,
            ) as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as uint16_t;
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                *x264_scan8.as_ptr().offset(26 as ::core::ffi::c_int as isize) as isize,
            ) as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as uint16_t;
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                *x264_scan8.as_ptr().offset(40 as ::core::ffi::c_int as isize) as isize,
            ) as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as uint16_t;
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                *x264_scan8.as_ptr().offset(42 as ::core::ffi::c_int as isize) as isize,
            ) as *mut uint8_t as *mut x264_union16_t))
            .i = 0 as uint16_t;
    }
    if b_decimate != 0
        && i_qp
            >= (if (*h).mb.b_trellis != 0 {
                12 as ::core::ffi::c_int
            } else {
                18 as ::core::ffi::c_int
            }) && (*h).mb.b_noise_reduction == 0
    {
        let mut thresh: ::core::ffi::c_int = if chroma422 != 0 {
            x264_lambda2_tab[i_qp as usize] + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int
        } else {
            x264_lambda2_tab[i_qp as usize] + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int
        };
        let mut ssd: [::core::ffi::c_int; 2] = [0; 2];
        let mut chromapix: ::core::ffi::c_int = if chroma422 != 0 {
            PIXEL_8x16 as ::core::ffi::c_int
        } else {
            PIXEL_8x8 as ::core::ffi::c_int
        };
        if (*h)
            .pixf
            .var2[chromapix as usize]
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fenc[1 as ::core::ffi::c_int as usize],
            (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
            ssd.as_mut_ptr(),
        ) < thresh * 4 as ::core::ffi::c_int
        {
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(CHROMA_DC + 0 as ::core::ffi::c_int)
                as usize] as usize] = 0 as uint8_t;
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(CHROMA_DC + 1 as ::core::ffi::c_int)
                as usize] as usize] = 0 as uint8_t;
            let mut ch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while ch < 2 as ::core::ffi::c_int {
                if ssd[ch as usize] > thresh {
                    let mut p_src: *mut pixel = (*h)
                        .mb
                        .pic
                        .p_fenc[(1 as ::core::ffi::c_int + ch) as usize];
                    let mut p_dst: *mut pixel = (*h)
                        .mb
                        .pic
                        .p_fdec[(1 as ::core::ffi::c_int + ch) as usize];
                    if chroma422 != 0 {
                        (*h)
                            .dctf
                            .sub8x16_dct_dc
                            .expect(
                                "non-null function pointer",
                            )(dct_dc.as_mut_ptr(), p_src, p_dst);
                    } else {
                        (*h)
                            .dctf
                            .sub8x8_dct_dc
                            .expect(
                                "non-null function pointer",
                            )(dct_dc.as_mut_ptr(), p_src, p_dst);
                    }
                    if (*h).mb.b_trellis != 0 {
                        nz_dc = x264_10_quant_chroma_dc_trellis(
                            h,
                            dct_dc.as_mut_ptr(),
                            i_qp + 3 as ::core::ffi::c_int * chroma422,
                            (b_inter == 0) as ::core::ffi::c_int,
                            CHROMA_DC + ch,
                        );
                    } else {
                        nz_dc = 0 as ::core::ffi::c_int;
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i <= chroma422 {
                            nz_dc
                                |= (*h)
                                    .quantf
                                    .quant_2x2_dc
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    &mut *dct_dc
                                        .as_mut_ptr()
                                        .offset((4 as ::core::ffi::c_int * i) as isize),
                                    ((*(*h)
                                        .quant4_mf[(CQM_4IC as ::core::ffi::c_int + b_inter)
                                            as usize]
                                        .offset(
                                            (i_qp + 3 as ::core::ffi::c_int * chroma422) as isize,
                                        ))[0 as ::core::ffi::c_int as usize]
                                        >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                    ((*(*h)
                                        .quant4_bias[(CQM_4IC as ::core::ffi::c_int + b_inter)
                                            as usize]
                                        .offset(
                                            (i_qp + 3 as ::core::ffi::c_int * chroma422) as isize,
                                        ))[0 as ::core::ffi::c_int as usize]
                                        << 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                                );
                            i += 1;
                        }
                    }
                    if nz_dc != 0 {
                        if !(mb_optimize_chroma_dc(
                            h,
                            dct_dc.as_mut_ptr(),
                            dequant_mf as *mut [::core::ffi::c_int; 16],
                            i_qp + 3 as ::core::ffi::c_int * chroma422,
                            chroma422,
                        ) == 0)
                        {
                            (*h)
                                .mb
                                .cache
                                .non_zero_count[x264_scan8[(CHROMA_DC + ch) as usize]
                                as usize] = 1 as uint8_t;
                            if chroma422 != 0 {
                                zigzag_scan_2x4_dc(
                                    (*(*h).dct.chroma_dc.as_mut_ptr().offset(ch as isize))
                                        .as_mut_ptr(),
                                    dct_dc.as_mut_ptr(),
                                );
                                (*h)
                                    .quantf
                                    .idct_dequant_2x4_dconly
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dct_dc.as_mut_ptr(),
                                    dequant_mf as *mut [::core::ffi::c_int; 16],
                                    i_qp + 3 as ::core::ffi::c_int,
                                );
                            } else {
                                zigzag_scan_2x2_dc(
                                    (*(*h).dct.chroma_dc.as_mut_ptr().offset(ch as isize))
                                        .as_mut_ptr(),
                                    dct_dc.as_mut_ptr(),
                                );
                                idct_dequant_2x2_dconly(
                                    dct_dc.as_mut_ptr(),
                                    dequant_mf as *mut [::core::ffi::c_int; 16],
                                    i_qp,
                                );
                            }
                            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_0 <= chroma422 {
                                (*h)
                                    .dctf
                                    .add8x8_idct_dc
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    p_dst
                                        .offset(
                                            (8 as ::core::ffi::c_int * i_0 * FDEC_STRIDE) as isize,
                                        ),
                                    &mut *dct_dc
                                        .as_mut_ptr()
                                        .offset((4 as ::core::ffi::c_int * i_0) as isize),
                                );
                                i_0 += 1;
                            }
                            (*h).mb.i_cbp_chroma = 1 as ::core::ffi::c_int;
                        }
                    }
                }
                ch += 1;
            }
            return;
        }
    }
    let mut ch_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while ch_0 < 2 as ::core::ffi::c_int {
        let mut p_src_0: *mut pixel = (*h)
            .mb
            .pic
            .p_fenc[(1 as ::core::ffi::c_int + ch_0) as usize];
        let mut p_dst_0: *mut pixel = (*h)
            .mb
            .pic
            .p_fdec[(1 as ::core::ffi::c_int + ch_0) as usize];
        let mut i_decimate_score: ::core::ffi::c_int = if b_decimate != 0 {
            0 as ::core::ffi::c_int
        } else {
            7 as ::core::ffi::c_int
        };
        let mut nz_ac: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut dct4x4: [[dctcoef; 16]; 8] = [[0; 16]; 8];
        if (*h).mb.b_lossless != 0 {
            static mut chroma422_scan: [uint8_t; 8] = [
                0 as ::core::ffi::c_int as uint8_t,
                2 as ::core::ffi::c_int as uint8_t,
                1 as ::core::ffi::c_int as uint8_t,
                5 as ::core::ffi::c_int as uint8_t,
                3 as ::core::ffi::c_int as uint8_t,
                6 as ::core::ffi::c_int as uint8_t,
                4 as ::core::ffi::c_int as uint8_t,
                7 as ::core::ffi::c_int as uint8_t,
            ];
            let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_1
                < (if chroma422 != 0 {
                    8 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                })
            {
                let mut oe: ::core::ffi::c_int = 4 as ::core::ffi::c_int
                    * (i_1 & 1 as ::core::ffi::c_int)
                    + 4 as ::core::ffi::c_int * (i_1 >> 1 as ::core::ffi::c_int)
                        * FENC_STRIDE;
                let mut od: ::core::ffi::c_int = 4 as ::core::ffi::c_int
                    * (i_1 & 1 as ::core::ffi::c_int)
                    + 4 as ::core::ffi::c_int * (i_1 >> 1 as ::core::ffi::c_int)
                        * FDEC_STRIDE;
                nz = (*h)
                    .zigzagf
                    .sub_4x4ac
                    .expect(
                        "non-null function pointer",
                    )(
                    (*(*h)
                        .dct
                        .luma4x4
                        .as_mut_ptr()
                        .offset(
                            (16 as ::core::ffi::c_int + i_1
                                + (if chroma422 != 0 {
                                    i_1 & 4 as ::core::ffi::c_int
                                } else {
                                    0 as ::core::ffi::c_int
                                }) + ch_0 * 16 as ::core::ffi::c_int) as isize,
                        ))
                        .as_mut_ptr(),
                    p_src_0.offset(oe as isize),
                    p_dst_0.offset(od as isize),
                    &mut *(*(*h).dct.chroma_dc.as_mut_ptr().offset(ch_0 as isize))
                        .as_mut_ptr()
                        .offset(
                            (if chroma422 != 0 {
                                *chroma422_scan.as_ptr().offset(i_1 as isize)
                                    as ::core::ffi::c_int
                            } else {
                                i_1
                            }) as isize,
                        ),
                );
                (*h)
                    .mb
                    .cache
                    .non_zero_count[x264_scan8[(16 as ::core::ffi::c_int + i_1
                    + (if chroma422 != 0 {
                        i_1 & 4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) + ch_0 * 16 as ::core::ffi::c_int) as usize] as usize] = nz
                    as uint8_t;
                (*h).mb.i_cbp_chroma |= nz;
                i_1 += 1;
            }
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(CHROMA_DC + ch_0) as usize] as usize] = array_non_zero(
                (*(*h).dct.chroma_dc.as_mut_ptr().offset(ch_0 as isize)).as_mut_ptr(),
                if chroma422 != 0 {
                    8 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                },
            ) as uint8_t;
        } else {
            let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_2 <= chroma422 {
                (*h)
                    .dctf
                    .sub8x8_dct
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut *dct4x4
                        .as_mut_ptr()
                        .offset((4 as ::core::ffi::c_int * i_2) as isize),
                    p_src_0
                        .offset((8 as ::core::ffi::c_int * i_2 * FENC_STRIDE) as isize),
                    p_dst_0
                        .offset((8 as ::core::ffi::c_int * i_2 * FDEC_STRIDE) as isize),
                );
                i_2 += 1;
            }
            if (*h).mb.b_noise_reduction != 0 {
                let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_3
                    < (if chroma422 != 0 {
                        8 as ::core::ffi::c_int
                    } else {
                        4 as ::core::ffi::c_int
                    })
                {
                    (*h)
                        .quantf
                        .denoise_dct
                        .expect(
                            "non-null function pointer",
                        )(
                        (*dct4x4.as_mut_ptr().offset(i_3 as isize)).as_mut_ptr(),
                        (*(*h).nr_residual_sum.offset(2 as ::core::ffi::c_int as isize))
                            .as_mut_ptr(),
                        (*(*h).nr_offset.offset(2 as ::core::ffi::c_int as isize))
                            .as_mut_ptr(),
                        16 as ::core::ffi::c_int,
                    );
                    i_3 += 1;
                }
            }
            if chroma422 != 0 {
                (*h)
                    .dctf
                    .dct2x4dc
                    .expect(
                        "non-null function pointer",
                    )(dct_dc.as_mut_ptr(), dct4x4.as_mut_ptr());
            } else {
                dct2x2dc(dct_dc.as_mut_ptr(), dct4x4.as_mut_ptr());
            }
            let mut i8x8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i8x8
                < (if chroma422 != 0 {
                    2 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                })
            {
                if (*h).mb.b_trellis != 0 {
                    let mut i4x4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i4x4 < 4 as ::core::ffi::c_int {
                        if x264_10_quant_4x4_trellis(
                            h,
                            (*dct4x4
                                .as_mut_ptr()
                                .offset((i8x8 * 4 as ::core::ffi::c_int + i4x4) as isize))
                                .as_mut_ptr(),
                            CQM_4IC as ::core::ffi::c_int + b_inter,
                            i_qp,
                            DCT_CHROMA_AC as ::core::ffi::c_int,
                            (b_inter == 0) as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        ) != 0
                        {
                            let mut idx: ::core::ffi::c_int = 16 as ::core::ffi::c_int
                                + ch_0 * 16 as ::core::ffi::c_int
                                + i8x8 * 8 as ::core::ffi::c_int + i4x4;
                            (*h)
                                .zigzagf
                                .scan_4x4
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*(*h).dct.luma4x4.as_mut_ptr().offset(idx as isize))
                                    .as_mut_ptr(),
                                (*dct4x4
                                    .as_mut_ptr()
                                    .offset((i8x8 * 4 as ::core::ffi::c_int + i4x4) as isize))
                                    .as_mut_ptr(),
                            );
                            (*h)
                                .quantf
                                .dequant_4x4
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*dct4x4
                                    .as_mut_ptr()
                                    .offset((i8x8 * 4 as ::core::ffi::c_int + i4x4) as isize))
                                    .as_mut_ptr(),
                                dequant_mf as *mut [::core::ffi::c_int; 16],
                                i_qp,
                            );
                            if i_decimate_score < 7 as ::core::ffi::c_int {
                                i_decimate_score
                                    += (*h)
                                        .quantf
                                        .decimate_score15
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        (*(*h).dct.luma4x4.as_mut_ptr().offset(idx as isize))
                                            .as_mut_ptr(),
                                    );
                            }
                            (*h)
                                .mb
                                .cache
                                .non_zero_count[x264_scan8[idx as usize] as usize] = 1
                                as uint8_t;
                            nz_ac = 1 as ::core::ffi::c_int;
                        }
                        i4x4 += 1;
                    }
                } else {
                    nz = (*h)
                        .quantf
                        .quant_4x4x4
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut *dct4x4
                            .as_mut_ptr()
                            .offset((i8x8 * 4 as ::core::ffi::c_int) as isize),
                        (*(*(*h)
                            .quant4_mf
                            .as_mut_ptr()
                            .offset((CQM_4IC as ::core::ffi::c_int + b_inter) as isize))
                            .offset(i_qp as isize))
                            .as_mut_ptr(),
                        (*(*(*h)
                            .quant4_bias
                            .as_mut_ptr()
                            .offset((CQM_4IC as ::core::ffi::c_int + b_inter) as isize))
                            .offset(i_qp as isize))
                            .as_mut_ptr(),
                    );
                    nz_ac |= nz;
                    let mut i4x4_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut msk: ::core::ffi::c_int = nz;
                    let mut skip: ::core::ffi::c_int = 0;
                    while msk != 0
                        && {
                            skip = x264_ctz_4bit(msk as uint32_t);
                            i4x4_0 += skip;
                            msk >>= skip + 1 as ::core::ffi::c_int;
                            1 as ::core::ffi::c_int != 0
                        }
                    {
                        let mut idx_0: ::core::ffi::c_int = 16 as ::core::ffi::c_int
                            + ch_0 * 16 as ::core::ffi::c_int
                            + i8x8 * 8 as ::core::ffi::c_int + i4x4_0;
                        (*h)
                            .zigzagf
                            .scan_4x4
                            .expect(
                                "non-null function pointer",
                            )(
                            (*(*h).dct.luma4x4.as_mut_ptr().offset(idx_0 as isize))
                                .as_mut_ptr(),
                            (*dct4x4
                                .as_mut_ptr()
                                .offset((i8x8 * 4 as ::core::ffi::c_int + i4x4_0) as isize))
                                .as_mut_ptr(),
                        );
                        (*h)
                            .quantf
                            .dequant_4x4
                            .expect(
                                "non-null function pointer",
                            )(
                            (*dct4x4
                                .as_mut_ptr()
                                .offset((i8x8 * 4 as ::core::ffi::c_int + i4x4_0) as isize))
                                .as_mut_ptr(),
                            dequant_mf as *mut [::core::ffi::c_int; 16],
                            i_qp,
                        );
                        if i_decimate_score < 7 as ::core::ffi::c_int {
                            i_decimate_score
                                += (*h)
                                    .quantf
                                    .decimate_score15
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*(*h).dct.luma4x4.as_mut_ptr().offset(idx_0 as isize))
                                        .as_mut_ptr(),
                                );
                        }
                        (*h)
                            .mb
                            .cache
                            .non_zero_count[x264_scan8[idx_0 as usize] as usize] = 1
                            as uint8_t;
                        i4x4_0 += 1;
                    }
                }
                i8x8 += 1;
            }
            if (*h).mb.b_trellis != 0 {
                nz_dc = x264_10_quant_chroma_dc_trellis(
                    h,
                    dct_dc.as_mut_ptr(),
                    i_qp + 3 as ::core::ffi::c_int * chroma422,
                    (b_inter == 0) as ::core::ffi::c_int,
                    CHROMA_DC + ch_0,
                );
            } else {
                nz_dc = 0 as ::core::ffi::c_int;
                let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_4 <= chroma422 {
                    nz_dc
                        |= (*h)
                            .quantf
                            .quant_2x2_dc
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *dct_dc
                                .as_mut_ptr()
                                .offset((4 as ::core::ffi::c_int * i_4) as isize),
                            ((*(*h)
                                .quant4_mf[(CQM_4IC as ::core::ffi::c_int + b_inter)
                                    as usize]
                                .offset(
                                    (i_qp + 3 as ::core::ffi::c_int * chroma422) as isize,
                                ))[0 as ::core::ffi::c_int as usize]
                                >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                            ((*(*h)
                                .quant4_bias[(CQM_4IC as ::core::ffi::c_int + b_inter)
                                    as usize]
                                .offset(
                                    (i_qp + 3 as ::core::ffi::c_int * chroma422) as isize,
                                ))[0 as ::core::ffi::c_int as usize]
                                << 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                        );
                    i_4 += 1;
                }
            }
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(CHROMA_DC + ch_0) as usize] as usize] = nz_dc
                as uint8_t;
            if i_decimate_score < 7 as ::core::ffi::c_int || nz_ac == 0 {
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        *x264_scan8
                            .as_ptr()
                            .offset(
                                (16 as ::core::ffi::c_int + 16 as ::core::ffi::c_int * ch_0)
                                    as isize,
                            ) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = 0 as uint16_t;
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        *x264_scan8
                            .as_ptr()
                            .offset(
                                (18 as ::core::ffi::c_int + 16 as ::core::ffi::c_int * ch_0)
                                    as isize,
                            ) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = 0 as uint16_t;
                if chroma422 != 0 {
                    (*(&mut *(*h)
                        .mb
                        .cache
                        .non_zero_count
                        .as_mut_ptr()
                        .offset(
                            *x264_scan8
                                .as_ptr()
                                .offset(
                                    (24 as ::core::ffi::c_int + 16 as ::core::ffi::c_int * ch_0)
                                        as isize,
                                ) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                        .i = 0 as uint16_t;
                    (*(&mut *(*h)
                        .mb
                        .cache
                        .non_zero_count
                        .as_mut_ptr()
                        .offset(
                            *x264_scan8
                                .as_ptr()
                                .offset(
                                    (26 as ::core::ffi::c_int + 16 as ::core::ffi::c_int * ch_0)
                                        as isize,
                                ) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                        .i = 0 as uint16_t;
                }
                if !(nz_dc == 0) {
                    if mb_optimize_chroma_dc(
                        h,
                        dct_dc.as_mut_ptr(),
                        dequant_mf as *mut [::core::ffi::c_int; 16],
                        i_qp + 3 as ::core::ffi::c_int * chroma422,
                        chroma422,
                    ) == 0
                    {
                        (*h)
                            .mb
                            .cache
                            .non_zero_count[x264_scan8[(CHROMA_DC + ch_0) as usize]
                            as usize] = 0 as uint8_t;
                    } else {
                        if chroma422 != 0 {
                            zigzag_scan_2x4_dc(
                                (*(*h).dct.chroma_dc.as_mut_ptr().offset(ch_0 as isize))
                                    .as_mut_ptr(),
                                dct_dc.as_mut_ptr(),
                            );
                            (*h)
                                .quantf
                                .idct_dequant_2x4_dconly
                                .expect(
                                    "non-null function pointer",
                                )(
                                dct_dc.as_mut_ptr(),
                                dequant_mf as *mut [::core::ffi::c_int; 16],
                                i_qp + 3 as ::core::ffi::c_int,
                            );
                        } else {
                            zigzag_scan_2x2_dc(
                                (*(*h).dct.chroma_dc.as_mut_ptr().offset(ch_0 as isize))
                                    .as_mut_ptr(),
                                dct_dc.as_mut_ptr(),
                            );
                            idct_dequant_2x2_dconly(
                                dct_dc.as_mut_ptr(),
                                dequant_mf as *mut [::core::ffi::c_int; 16],
                                i_qp,
                            );
                        }
                        let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_5 <= chroma422 {
                            (*h)
                                .dctf
                                .add8x8_idct_dc
                                .expect(
                                    "non-null function pointer",
                                )(
                                p_dst_0
                                    .offset(
                                        (8 as ::core::ffi::c_int * i_5 * FDEC_STRIDE) as isize,
                                    ),
                                &mut *dct_dc
                                    .as_mut_ptr()
                                    .offset((4 as ::core::ffi::c_int * i_5) as isize),
                            );
                            i_5 += 1;
                        }
                    }
                }
            } else {
                (*h).mb.i_cbp_chroma = 1 as ::core::ffi::c_int;
                if nz_dc != 0 {
                    if chroma422 != 0 {
                        zigzag_scan_2x4_dc(
                            (*(*h).dct.chroma_dc.as_mut_ptr().offset(ch_0 as isize))
                                .as_mut_ptr(),
                            dct_dc.as_mut_ptr(),
                        );
                        (*h)
                            .quantf
                            .idct_dequant_2x4_dc
                            .expect(
                                "non-null function pointer",
                            )(
                            dct_dc.as_mut_ptr(),
                            dct4x4.as_mut_ptr(),
                            dequant_mf as *mut [::core::ffi::c_int; 16],
                            i_qp + 3 as ::core::ffi::c_int,
                        );
                    } else {
                        zigzag_scan_2x2_dc(
                            (*(*h).dct.chroma_dc.as_mut_ptr().offset(ch_0 as isize))
                                .as_mut_ptr(),
                            dct_dc.as_mut_ptr(),
                        );
                        idct_dequant_2x2_dc(
                            dct_dc.as_mut_ptr(),
                            dct4x4.as_mut_ptr(),
                            dequant_mf as *mut [::core::ffi::c_int; 16],
                            i_qp,
                        );
                    }
                }
                let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_6 <= chroma422 {
                    (*h)
                        .dctf
                        .add8x8_idct
                        .expect(
                            "non-null function pointer",
                        )(
                        p_dst_0
                            .offset(
                                (8 as ::core::ffi::c_int * i_6 * FDEC_STRIDE) as isize,
                            ),
                        &mut *dct4x4
                            .as_mut_ptr()
                            .offset((4 as ::core::ffi::c_int * i_6) as isize),
                    );
                    i_6 += 1;
                }
            }
        }
        ch_0 += 1;
    }
    (*h).mb.i_cbp_chroma
        += (*h)
            .mb
            .cache
            .non_zero_count[x264_scan8[(CHROMA_DC + 0 as ::core::ffi::c_int) as usize]
            as usize] as ::core::ffi::c_int
            | (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(CHROMA_DC + 1 as ::core::ffi::c_int)
                as usize] as usize] as ::core::ffi::c_int | (*h).mb.i_cbp_chroma;
}
#[no_mangle]
#[c2rust::src_loc = "492:1"]
pub unsafe extern "C" fn x264_10_mb_encode_chroma(
    mut h: *mut x264_t,
    mut b_inter: ::core::ffi::c_int,
    mut i_qp: ::core::ffi::c_int,
) {
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as ::core::ffi::c_int {
        mb_encode_chroma_internal(h, b_inter, i_qp, 0 as ::core::ffi::c_int);
    } else {
        mb_encode_chroma_internal(h, b_inter, i_qp, 1 as ::core::ffi::c_int);
    };
}
#[c2rust::src_loc = "500:1"]
unsafe extern "C" fn macroblock_encode_skip(mut h: *mut x264_t) {
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(2 as ::core::ffi::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(8 as ::core::ffi::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(*x264_scan8.as_ptr().offset(10 as ::core::ffi::c_int as isize) as isize)
        as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(
            *x264_scan8
                .as_ptr()
                .offset((16 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(
            *x264_scan8
                .as_ptr()
                .offset((16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(
            *x264_scan8
                .as_ptr()
                .offset((32 as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    (*(&mut *(*h)
        .mb
        .cache
        .non_zero_count
        .as_mut_ptr()
        .offset(
            *x264_scan8
                .as_ptr()
                .offset((32 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as isize)
                as isize,
        ) as *mut uint8_t as *mut x264_union32_t))
        .i = 0 as uint32_t;
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc >= CHROMA_422 as ::core::ffi::c_int {
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset(
                        (16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                    ) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as uint32_t;
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset(
                        (16 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                    ) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as uint32_t;
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset(
                        (32 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                    ) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as uint32_t;
        (*(&mut *(*h)
            .mb
            .cache
            .non_zero_count
            .as_mut_ptr()
            .offset(
                *x264_scan8
                    .as_ptr()
                    .offset(
                        (32 as ::core::ffi::c_int + 10 as ::core::ffi::c_int) as isize,
                    ) as isize,
            ) as *mut uint8_t as *mut x264_union32_t))
            .i = 0 as uint32_t;
    }
    (*h).mb.i_cbp_luma = 0 as ::core::ffi::c_int;
    (*h).mb.i_cbp_chroma = 0 as ::core::ffi::c_int;
    *(*h).mb.cbp.offset((*h).mb.i_mb_xy as isize) = 0 as int16_t;
}
#[no_mangle]
#[c2rust::src_loc = "526:1"]
pub unsafe extern "C" fn x264_10_predict_lossless_chroma(
    mut h: *mut x264_t,
    mut i_mode: ::core::ffi::c_int,
) {
    let mut height: ::core::ffi::c_int = 16 as ::core::ffi::c_int
        >> (*h).mb.chroma_v_shift;
    if i_mode == I_PRED_CHROMA_V as ::core::ffi::c_int {
        (*h)
            .mc
            .copy[PIXEL_8x8 as ::core::ffi::c_int as usize]
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
            FDEC_STRIDE as intptr_t,
            (*h)
                .mb
                .pic
                .p_fenc[1 as ::core::ffi::c_int as usize]
                .offset(-(FENC_STRIDE as isize)),
            FENC_STRIDE as intptr_t,
            height,
        );
        (*h)
            .mc
            .copy[PIXEL_8x8 as ::core::ffi::c_int as usize]
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize],
            FDEC_STRIDE as intptr_t,
            (*h)
                .mb
                .pic
                .p_fenc[2 as ::core::ffi::c_int as usize]
                .offset(-(FENC_STRIDE as isize)),
            FENC_STRIDE as intptr_t,
            height,
        );
        memcpy(
            (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]
                as *mut ::core::ffi::c_void,
            (*h)
                .mb
                .pic
                .p_fdec[1 as ::core::ffi::c_int as usize]
                .offset(-(FDEC_STRIDE as isize)) as *const ::core::ffi::c_void,
            (8 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
        memcpy(
            (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]
                as *mut ::core::ffi::c_void,
            (*h)
                .mb
                .pic
                .p_fdec[2 as ::core::ffi::c_int as usize]
                .offset(-(FDEC_STRIDE as isize)) as *const ::core::ffi::c_void,
            (8 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
    } else if i_mode == I_PRED_CHROMA_H as ::core::ffi::c_int {
        (*h)
            .mc
            .copy[PIXEL_8x8 as ::core::ffi::c_int as usize]
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
            FDEC_STRIDE as intptr_t,
            (*h)
                .mb
                .pic
                .p_fenc[1 as ::core::ffi::c_int as usize]
                .offset(-(1 as ::core::ffi::c_int as isize)),
            FENC_STRIDE as intptr_t,
            height,
        );
        (*h)
            .mc
            .copy[PIXEL_8x8 as ::core::ffi::c_int as usize]
            .expect(
                "non-null function pointer",
            )(
            (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize],
            FDEC_STRIDE as intptr_t,
            (*h)
                .mb
                .pic
                .p_fenc[2 as ::core::ffi::c_int as usize]
                .offset(-(1 as ::core::ffi::c_int as isize)),
            FENC_STRIDE as intptr_t,
            height,
        );
        x264_10_copy_column8(
            (*h)
                .mb
                .pic
                .p_fdec[1 as ::core::ffi::c_int as usize]
                .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
            (*h)
                .mb
                .pic
                .p_fdec[1 as ::core::ffi::c_int as usize]
                .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
        );
        x264_10_copy_column8(
            (*h)
                .mb
                .pic
                .p_fdec[2 as ::core::ffi::c_int as usize]
                .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
            (*h)
                .mb
                .pic
                .p_fdec[2 as ::core::ffi::c_int as usize]
                .offset((4 as ::core::ffi::c_int * FDEC_STRIDE) as isize)
                .offset(-(1 as ::core::ffi::c_int as isize)),
        );
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
            == CHROMA_422 as ::core::ffi::c_int
        {
            x264_10_copy_column8(
                (*h)
                    .mb
                    .pic
                    .p_fdec[1 as ::core::ffi::c_int as usize]
                    .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                (*h)
                    .mb
                    .pic
                    .p_fdec[1 as ::core::ffi::c_int as usize]
                    .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)),
            );
            x264_10_copy_column8(
                (*h)
                    .mb
                    .pic
                    .p_fdec[2 as ::core::ffi::c_int as usize]
                    .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize),
                (*h)
                    .mb
                    .pic
                    .p_fdec[2 as ::core::ffi::c_int as usize]
                    .offset((12 as ::core::ffi::c_int * FDEC_STRIDE) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize)),
            );
        }
    } else {
        (*h)
            .predict_chroma[i_mode as usize]
            .expect(
                "non-null function pointer",
            )((*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]);
        (*h)
            .predict_chroma[i_mode as usize]
            .expect(
                "non-null function pointer",
            )((*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]);
    };
}
#[no_mangle]
#[c2rust::src_loc = "555:1"]
pub unsafe extern "C" fn x264_10_predict_lossless_4x4(
    mut h: *mut x264_t,
    mut p_dst: *mut pixel,
    mut p: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut i_mode: ::core::ffi::c_int,
) {
    let mut stride: ::core::ffi::c_int = (*(*h).fenc).i_stride[p as usize]
        << (*h).mb.b_interlaced;
    let mut p_src: *mut pixel = (*h)
        .mb
        .pic
        .p_fenc_plane[p as usize]
        .offset(
            (block_idx_x[idx as usize] as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as isize,
        )
        .offset(
            (block_idx_y[idx as usize] as ::core::ffi::c_int * 4 as ::core::ffi::c_int
                * stride) as isize,
        );
    if i_mode == I_PRED_4x4_V as ::core::ffi::c_int {
        (*h)
            .mc
            .copy[PIXEL_4x4 as ::core::ffi::c_int as usize]
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            FDEC_STRIDE as intptr_t,
            p_src.offset(-(stride as isize)),
            stride as intptr_t,
            4 as ::core::ffi::c_int,
        );
        memcpy(
            p_dst as *mut ::core::ffi::c_void,
            p_dst.offset(-(FDEC_STRIDE as isize)) as *const ::core::ffi::c_void,
            (4 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
    } else if i_mode == I_PRED_4x4_H as ::core::ffi::c_int {
        (*h)
            .mc
            .copy[PIXEL_4x4 as ::core::ffi::c_int as usize]
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            FDEC_STRIDE as intptr_t,
            p_src.offset(-(1 as ::core::ffi::c_int as isize)),
            stride as intptr_t,
            4 as ::core::ffi::c_int,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            *p_dst.offset((i * FDEC_STRIDE) as isize) = *p_dst
                .offset((i * FDEC_STRIDE - 1 as ::core::ffi::c_int) as isize);
            i += 1;
        }
    } else {
        (*h).predict_4x4[i_mode as usize].expect("non-null function pointer")(p_dst);
    };
}
#[no_mangle]
#[c2rust::src_loc = "575:1"]
pub unsafe extern "C" fn x264_10_predict_lossless_8x8(
    mut h: *mut x264_t,
    mut p_dst: *mut pixel,
    mut p: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut i_mode: ::core::ffi::c_int,
    mut edge: *mut pixel,
) {
    let mut stride: ::core::ffi::c_int = (*(*h).fenc).i_stride[p as usize]
        << (*h).mb.b_interlaced;
    let mut p_src: *mut pixel = (*h)
        .mb
        .pic
        .p_fenc_plane[p as usize]
        .offset(((idx & 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int) as isize)
        .offset(
            ((idx >> 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int * stride)
                as isize,
        );
    if i_mode == I_PRED_8x8_V as ::core::ffi::c_int {
        (*h)
            .mc
            .copy[PIXEL_8x8 as ::core::ffi::c_int as usize]
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            FDEC_STRIDE as intptr_t,
            p_src.offset(-(stride as isize)),
            stride as intptr_t,
            8 as ::core::ffi::c_int,
        );
        memcpy(
            p_dst as *mut ::core::ffi::c_void,
            &mut *edge.offset(16 as ::core::ffi::c_int as isize) as *mut pixel
                as *const ::core::ffi::c_void,
            (8 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
    } else if i_mode == I_PRED_8x8_H as ::core::ffi::c_int {
        (*h)
            .mc
            .copy[PIXEL_8x8 as ::core::ffi::c_int as usize]
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            FDEC_STRIDE as intptr_t,
            p_src.offset(-(1 as ::core::ffi::c_int as isize)),
            stride as intptr_t,
            8 as ::core::ffi::c_int,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            *p_dst.offset((i * FDEC_STRIDE) as isize) = *edge
                .offset((14 as ::core::ffi::c_int - i) as isize);
            i += 1;
        }
    } else {
        (*h)
            .predict_8x8[i_mode as usize]
            .expect("non-null function pointer")(p_dst, edge);
    };
}
#[no_mangle]
#[c2rust::src_loc = "595:1"]
pub unsafe extern "C" fn x264_10_predict_lossless_16x16(
    mut h: *mut x264_t,
    mut p: ::core::ffi::c_int,
    mut i_mode: ::core::ffi::c_int,
) {
    let mut stride: ::core::ffi::c_int = (*(*h).fenc).i_stride[p as usize]
        << (*h).mb.b_interlaced;
    let mut p_dst: *mut pixel = (*h).mb.pic.p_fdec[p as usize];
    if i_mode == I_PRED_16x16_V as ::core::ffi::c_int {
        (*h)
            .mc
            .copy[PIXEL_16x16 as ::core::ffi::c_int as usize]
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            FDEC_STRIDE as intptr_t,
            (*h).mb.pic.p_fenc_plane[p as usize].offset(-(stride as isize)),
            stride as intptr_t,
            16 as ::core::ffi::c_int,
        );
        memcpy(
            p_dst as *mut ::core::ffi::c_void,
            p_dst.offset(-(FDEC_STRIDE as isize)) as *const ::core::ffi::c_void,
            (16 as ::core::ffi::c_int * SIZEOF_PIXEL) as size_t,
        );
    } else if i_mode == I_PRED_16x16_H as ::core::ffi::c_int {
        (*h)
            .mc
            .copy_16x16_unaligned
            .expect(
                "non-null function pointer",
            )(
            p_dst,
            FDEC_STRIDE as intptr_t,
            (*h)
                .mb
                .pic
                .p_fenc_plane[p as usize]
                .offset(-(1 as ::core::ffi::c_int as isize)),
            stride as intptr_t,
            16 as ::core::ffi::c_int,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 16 as ::core::ffi::c_int {
            *p_dst.offset((i * FDEC_STRIDE) as isize) = *p_dst
                .offset((i * FDEC_STRIDE - 1 as ::core::ffi::c_int) as isize);
            i += 1;
        }
    } else {
        (*h).predict_16x16[i_mode as usize].expect("non-null function pointer")(p_dst);
    };
}
#[inline(always)]
#[c2rust::src_loc = "618:1"]
unsafe extern "C" fn macroblock_encode_internal(
    mut h: *mut x264_t,
    mut plane_count: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    let mut i_qp: ::core::ffi::c_int = (*h).mb.i_qp;
    let mut b_decimate: ::core::ffi::c_int = (*h).mb.b_dct_decimate;
    let mut b_force_no_skip: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nz: ::core::ffi::c_int = 0;
    (*h).mb.i_cbp_luma = 0 as ::core::ffi::c_int;
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while p < plane_count {
        (*h).mb.cache.non_zero_count[x264_scan8[(LUMA_DC + p) as usize] as usize] = 0
            as uint8_t;
        p += 1;
    }
    if (*h).mb.i_type == I_PCM as ::core::ffi::c_int {
        let mut p_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while p_0 < plane_count {
            (*h)
                .mc
                .copy[PIXEL_16x16 as ::core::ffi::c_int as usize]
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[p_0 as usize],
                FDEC_STRIDE as intptr_t,
                (*h).mb.pic.p_fenc[p_0 as usize],
                FENC_STRIDE as intptr_t,
                16 as ::core::ffi::c_int,
            );
            p_0 += 1;
        }
        if chroma != 0 {
            let mut height: ::core::ffi::c_int = 16 as ::core::ffi::c_int
                >> (*h).mb.chroma_v_shift;
            (*h)
                .mc
                .copy[PIXEL_8x8 as ::core::ffi::c_int as usize]
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
                FDEC_STRIDE as intptr_t,
                (*h).mb.pic.p_fenc[1 as ::core::ffi::c_int as usize],
                FENC_STRIDE as intptr_t,
                height,
            );
            (*h)
                .mc
                .copy[PIXEL_8x8 as ::core::ffi::c_int as usize]
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize],
                FDEC_STRIDE as intptr_t,
                (*h).mb.pic.p_fenc[2 as ::core::ffi::c_int as usize],
                FENC_STRIDE as intptr_t,
                height,
            );
        }
        return;
    }
    if (*h).mb.b_allow_skip == 0 {
        b_force_no_skip = 1 as ::core::ffi::c_int;
        if (*h).mb.i_type == P_SKIP as ::core::ffi::c_int
            || (*h).mb.i_type == B_SKIP as ::core::ffi::c_int
        {
            if (*h).mb.i_type == P_SKIP as ::core::ffi::c_int {
                (*h).mb.i_type = P_L0 as ::core::ffi::c_int;
            } else if (*h).mb.i_type == B_SKIP as ::core::ffi::c_int {
                (*h).mb.i_type = B_DIRECT as ::core::ffi::c_int;
            }
        }
    }
    if (*h).mb.i_type == P_SKIP as ::core::ffi::c_int {
        if (*h).mb.b_skip_mc == 0 {
            let mut mvx: ::core::ffi::c_int = x264_clip3(
                (*h)
                    .mb
                    .cache
                    .mv[0 as ::core::ffi::c_int
                    as usize][x264_scan8[0 as ::core::ffi::c_int as usize]
                    as usize][0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                (*h).mb.mv_min[0 as ::core::ffi::c_int as usize],
                (*h).mb.mv_max[0 as ::core::ffi::c_int as usize],
            );
            let mut mvy: ::core::ffi::c_int = x264_clip3(
                (*h)
                    .mb
                    .cache
                    .mv[0 as ::core::ffi::c_int
                    as usize][x264_scan8[0 as ::core::ffi::c_int as usize]
                    as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                (*h).mb.mv_min[1 as ::core::ffi::c_int as usize],
                (*h).mb.mv_max[1 as ::core::ffi::c_int as usize],
            );
            let mut p_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p_1 < plane_count {
                (*h)
                    .mc
                    .mc_luma
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).mb.pic.p_fdec[p_1 as usize],
                    FDEC_STRIDE as intptr_t,
                    &mut *(*(*(*h)
                        .mb
                        .pic
                        .p_fref
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset((p_1 * 4 as ::core::ffi::c_int) as isize),
                    (*h).mb.pic.i_stride[p_1 as usize] as intptr_t,
                    mvx,
                    mvy,
                    16 as ::core::ffi::c_int,
                    16 as ::core::ffi::c_int,
                    &mut *(*(*h)
                        .sh
                        .weight
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset(p_1 as isize),
                );
                p_1 += 1;
            }
            if chroma != 0 {
                let mut v_shift: ::core::ffi::c_int = (*h).mb.chroma_v_shift;
                let mut height_0: ::core::ffi::c_int = 16 as ::core::ffi::c_int
                    >> v_shift;
                if mvx | mvy != 0 {
                    (*h)
                        .mc
                        .mc_chroma
                        .expect(
                            "non-null function pointer",
                        )(
                        (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
                        (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize],
                        FDEC_STRIDE as intptr_t,
                        (*h)
                            .mb
                            .pic
                            .p_fref[0 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int
                            as usize][4 as ::core::ffi::c_int as usize],
                        (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize]
                            as intptr_t,
                        mvx,
                        2 as ::core::ffi::c_int * mvy >> v_shift,
                        8 as ::core::ffi::c_int,
                        height_0,
                    );
                } else {
                    (*h)
                        .mc
                        .load_deinterleave_chroma_fdec
                        .expect(
                            "non-null function pointer",
                        )(
                        (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
                        (*h)
                            .mb
                            .pic
                            .p_fref[0 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int
                            as usize][4 as ::core::ffi::c_int as usize],
                        (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize]
                            as intptr_t,
                        height_0,
                    );
                }
                if !(*h)
                    .sh
                    .weight[0 as ::core::ffi::c_int
                        as usize][1 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()
                {
                    (*(*h)
                        .sh
                        .weight[0 as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int as usize]
                        .weightfn
                        .offset(
                            (8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as isize,
                        ))
                        .expect(
                            "non-null function pointer",
                        )(
                        (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
                        FDEC_STRIDE as intptr_t,
                        (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
                        FDEC_STRIDE as intptr_t,
                        &mut *(*(*h)
                            .sh
                            .weight
                            .as_mut_ptr()
                            .offset(0 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(1 as ::core::ffi::c_int as isize),
                        height_0,
                    );
                }
                if !(*h)
                    .sh
                    .weight[0 as ::core::ffi::c_int
                        as usize][2 as ::core::ffi::c_int as usize]
                    .weightfn
                    .is_null()
                {
                    (*(*h)
                        .sh
                        .weight[0 as ::core::ffi::c_int
                            as usize][2 as ::core::ffi::c_int as usize]
                        .weightfn
                        .offset(
                            (8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as isize,
                        ))
                        .expect(
                            "non-null function pointer",
                        )(
                        (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize],
                        FDEC_STRIDE as intptr_t,
                        (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize],
                        FDEC_STRIDE as intptr_t,
                        &mut *(*(*h)
                            .sh
                            .weight
                            .as_mut_ptr()
                            .offset(0 as ::core::ffi::c_int as isize))
                            .as_mut_ptr()
                            .offset(2 as ::core::ffi::c_int as isize),
                        height_0,
                    );
                }
            }
        }
        macroblock_encode_skip(h);
        return;
    }
    if (*h).mb.i_type == B_SKIP as ::core::ffi::c_int {
        if (*h).mb.b_skip_mc == 0 {
            x264_10_mb_mc(h);
        }
        macroblock_encode_skip(h);
        return;
    }
    if (*h).mb.i_type == I_16x16 as ::core::ffi::c_int {
        (*h).mb.b_transform_8x8 = 0 as ::core::ffi::c_int;
        let mut p_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while p_2 < plane_count {
            mb_encode_i16x16(h, p_2, i_qp);
            p_2 += 1;
            i_qp = (*h).mb.i_chroma_qp;
        }
    } else if (*h).mb.i_type == I_8x8 as ::core::ffi::c_int {
        (*h).mb.b_transform_8x8 = 1 as ::core::ffi::c_int;
        if (*h).mb.i_skip_intra != 0 {
            (*h)
                .mc
                .copy[PIXEL_16x16 as ::core::ffi::c_int as usize]
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize],
                FDEC_STRIDE as intptr_t,
                (*h).mb.pic.i8x8_fdec_buf.as_mut_ptr(),
                16 as intptr_t,
                16 as ::core::ffi::c_int,
            );
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    *x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                        as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[0 as ::core::ffi::c_int as usize];
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    *x264_scan8.as_ptr().offset(2 as ::core::ffi::c_int as isize)
                        as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[1 as ::core::ffi::c_int as usize];
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    *x264_scan8.as_ptr().offset(8 as ::core::ffi::c_int as isize)
                        as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[2 as ::core::ffi::c_int as usize];
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    *x264_scan8.as_ptr().offset(10 as ::core::ffi::c_int as isize)
                        as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i8x8_nnz_buf[3 as ::core::ffi::c_int as usize];
            (*h).mb.i_cbp_luma = (*h).mb.pic.i8x8_cbp;
            if (*h).mb.i_skip_intra == 2 as ::core::ffi::c_int {
                (*h)
                    .mc
                    .memcpy_aligned
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).dct.luma8x8.as_mut_ptr() as *mut ::core::ffi::c_void,
                    (*h).mb.pic.i8x8_dct_buf.as_mut_ptr() as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[[dctcoef; 64]; 3]>() as size_t,
                );
            }
        }
        let mut p_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while p_3 < plane_count {
            let mut i: ::core::ffi::c_int = if p_3 == 0 as ::core::ffi::c_int
                && (*h).mb.i_skip_intra != 0
            {
                3 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            while i < 4 as ::core::ffi::c_int {
                let mut i_mode: ::core::ffi::c_int = (*h)
                    .mb
                    .cache
                    .intra4x4_pred_mode[x264_scan8[(4 as ::core::ffi::c_int * i)
                    as usize] as usize] as ::core::ffi::c_int;
                x264_mb_encode_i8x8(
                    h,
                    p_3,
                    i,
                    i_qp,
                    i_mode,
                    0 as *mut pixel,
                    1 as ::core::ffi::c_int,
                );
                i += 1;
            }
            p_3 += 1;
            i_qp = (*h).mb.i_chroma_qp;
        }
    } else if (*h).mb.i_type == I_4x4 as ::core::ffi::c_int {
        (*h).mb.b_transform_8x8 = 0 as ::core::ffi::c_int;
        if (*h).mb.i_skip_intra != 0 {
            (*h)
                .mc
                .copy[PIXEL_16x16 as ::core::ffi::c_int as usize]
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[0 as ::core::ffi::c_int as usize],
                FDEC_STRIDE as intptr_t,
                (*h).mb.pic.i4x4_fdec_buf.as_mut_ptr(),
                16 as intptr_t,
                16 as ::core::ffi::c_int,
            );
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    *x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                        as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[0 as ::core::ffi::c_int as usize];
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    *x264_scan8.as_ptr().offset(2 as ::core::ffi::c_int as isize)
                        as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[1 as ::core::ffi::c_int as usize];
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    *x264_scan8.as_ptr().offset(8 as ::core::ffi::c_int as isize)
                        as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[2 as ::core::ffi::c_int as usize];
            (*(&mut *(*h)
                .mb
                .cache
                .non_zero_count
                .as_mut_ptr()
                .offset(
                    *x264_scan8.as_ptr().offset(10 as ::core::ffi::c_int as isize)
                        as isize,
                ) as *mut uint8_t as *mut x264_union32_t))
                .i = (*h).mb.pic.i4x4_nnz_buf[3 as ::core::ffi::c_int as usize];
            (*h).mb.i_cbp_luma = (*h).mb.pic.i4x4_cbp;
            if (*h).mb.i_skip_intra == 2 as ::core::ffi::c_int {
                (*h)
                    .mc
                    .memcpy_aligned
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).dct.luma4x4.as_mut_ptr() as *mut ::core::ffi::c_void,
                    (*h).mb.pic.i4x4_dct_buf.as_mut_ptr() as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[[dctcoef; 16]; 15]>() as size_t,
                );
            }
        }
        let mut p_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while p_4 < plane_count {
            let mut i_0: ::core::ffi::c_int = if p_4 == 0 as ::core::ffi::c_int
                && (*h).mb.i_skip_intra != 0
            {
                15 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            };
            while i_0 < 16 as ::core::ffi::c_int {
                let mut p_dst: *mut pixel = &mut *(*(*h)
                    .mb
                    .pic
                    .p_fdec
                    .as_mut_ptr()
                    .offset(p_4 as isize))
                    .offset(*block_idx_xy_fdec.as_ptr().offset(i_0 as isize) as isize)
                    as *mut pixel;
                let mut i_mode_0: ::core::ffi::c_int = (*h)
                    .mb
                    .cache
                    .intra4x4_pred_mode[x264_scan8[i_0 as usize] as usize]
                    as ::core::ffi::c_int;
                if (*h).mb.i_neighbour4[i_0 as usize]
                    & (MB_TOPRIGHT as ::core::ffi::c_int | MB_TOP as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                    == MB_TOP as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    (*(&mut *p_dst
                        .offset(
                            (4 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as isize,
                        ) as *mut pixel as *mut x264_union64_t))
                        .i = (*p_dst
                        .offset(
                            (3 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as isize,
                        ) as ::core::ffi::c_ulonglong)
                        .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong)
                        as uint64_t;
                }
                x264_mb_encode_i4x4(
                    h,
                    p_4,
                    i_0,
                    i_qp,
                    i_mode_0,
                    1 as ::core::ffi::c_int,
                );
                i_0 += 1;
            }
            p_4 += 1;
            i_qp = (*h).mb.i_chroma_qp;
        }
    } else {
        let mut i_decimate_mb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*h).mb.b_skip_mc == 0 {
            x264_10_mb_mc(h);
        }
        if (*h).mb.b_lossless != 0 {
            if (*h).mb.b_transform_8x8 != 0 {
                let mut p_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while p_5 < plane_count {
                    let mut i8x8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i8x8 < 4 as ::core::ffi::c_int {
                        let mut x: ::core::ffi::c_int = i8x8 & 1 as ::core::ffi::c_int;
                        let mut y: ::core::ffi::c_int = i8x8 >> 1 as ::core::ffi::c_int;
                        nz = (*h)
                            .zigzagf
                            .sub_8x8
                            .expect(
                                "non-null function pointer",
                            )(
                            (*(*h)
                                .dct
                                .luma8x8
                                .as_mut_ptr()
                                .offset((p_5 * 4 as ::core::ffi::c_int + i8x8) as isize))
                                .as_mut_ptr(),
                            (*h)
                                .mb
                                .pic
                                .p_fenc[p_5 as usize]
                                .offset((8 as ::core::ffi::c_int * x) as isize)
                                .offset(
                                    (8 as ::core::ffi::c_int * y * FENC_STRIDE) as isize,
                                ),
                            (*h)
                                .mb
                                .pic
                                .p_fdec[p_5 as usize]
                                .offset((8 as ::core::ffi::c_int * x) as isize)
                                .offset(
                                    (8 as ::core::ffi::c_int * y * FDEC_STRIDE) as isize,
                                ),
                        );
                        (*(&mut *(*h)
                            .mb
                            .cache
                            .non_zero_count
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_5 * 16 as ::core::ffi::c_int
                                            + i8x8 * 4 as ::core::ffi::c_int) as isize,
                                    ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (nz * 0x101 as ::core::ffi::c_int) as uint16_t;
                        (*(&mut *(*h)
                            .mb
                            .cache
                            .non_zero_count
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_5 * 16 as ::core::ffi::c_int
                                            + i8x8 * 4 as ::core::ffi::c_int) as isize,
                                    ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (nz * 0x101 as ::core::ffi::c_int) as uint16_t;
                        (*h).mb.i_cbp_luma |= nz << i8x8;
                        i8x8 += 1;
                    }
                    p_5 += 1;
                }
            } else {
                let mut p_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while p_6 < plane_count {
                    let mut i4x4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i4x4 < 16 as ::core::ffi::c_int {
                        nz = (*h)
                            .zigzagf
                            .sub_4x4
                            .expect(
                                "non-null function pointer",
                            )(
                            (*(*h)
                                .dct
                                .luma4x4
                                .as_mut_ptr()
                                .offset((p_6 * 16 as ::core::ffi::c_int + i4x4) as isize))
                                .as_mut_ptr(),
                            (*h)
                                .mb
                                .pic
                                .p_fenc[p_6 as usize]
                                .offset(
                                    block_idx_xy_fenc[i4x4 as usize] as ::core::ffi::c_int
                                        as isize,
                                ),
                            (*h)
                                .mb
                                .pic
                                .p_fdec[p_6 as usize]
                                .offset(
                                    block_idx_xy_fdec[i4x4 as usize] as ::core::ffi::c_int
                                        as isize,
                                ),
                        );
                        (*h)
                            .mb
                            .cache
                            .non_zero_count[x264_scan8[(p_6 * 16 as ::core::ffi::c_int
                            + i4x4) as usize] as usize] = nz as uint8_t;
                        (*h).mb.i_cbp_luma |= nz << (i4x4 >> 2 as ::core::ffi::c_int);
                        i4x4 += 1;
                    }
                    p_6 += 1;
                }
            }
        } else if (*h).mb.b_transform_8x8 != 0 {
            let mut dct8x8: [[dctcoef; 64]; 4] = [[0; 64]; 4];
            b_decimate
                &= ((*h).mb.b_trellis == 0 || (*h).param.b_cabac == 0)
                    as ::core::ffi::c_int;
            let mut p_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p_7 < plane_count {
                let mut quant_cat: ::core::ffi::c_int = if p_7 != 0 {
                    CQM_8PC as ::core::ffi::c_int
                } else {
                    CQM_8PY as ::core::ffi::c_int
                };
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * p_7) as isize)
                            as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as uint32_t;
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * p_7) as isize)
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as uint32_t;
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * p_7) as isize)
                            as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as uint32_t;
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * p_7) as isize)
                            as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as uint32_t;
                (*h)
                    .dctf
                    .sub16x16_dct8
                    .expect(
                        "non-null function pointer",
                    )(
                    dct8x8.as_mut_ptr(),
                    (*h).mb.pic.p_fenc[p_7 as usize],
                    (*h).mb.pic.p_fdec[p_7 as usize],
                );
                let ref mut fresh0 = *(*h)
                    .nr_count
                    .offset(
                        (1 as ::core::ffi::c_int
                            + (p_7 != 0) as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                            as isize,
                    );
                *fresh0 = (*fresh0)
                    .wrapping_add(
                        ((*h).mb.b_noise_reduction * 4 as ::core::ffi::c_int) as uint32_t,
                    );
                let mut plane_cbp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while idx < 4 as ::core::ffi::c_int {
                    nz = x264_quant_8x8(
                        h,
                        (*dct8x8.as_mut_ptr().offset(idx as isize)).as_mut_ptr(),
                        i_qp,
                        ctx_cat_plane[DCT_LUMA_8x8 as ::core::ffi::c_int
                            as usize][p_7 as usize] as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        p_7,
                        idx,
                    );
                    if nz != 0 {
                        (*h)
                            .zigzagf
                            .scan_8x8
                            .expect(
                                "non-null function pointer",
                            )(
                            (*(*h)
                                .dct
                                .luma8x8
                                .as_mut_ptr()
                                .offset((p_7 * 4 as ::core::ffi::c_int + idx) as isize))
                                .as_mut_ptr(),
                            (*dct8x8.as_mut_ptr().offset(idx as isize)).as_mut_ptr(),
                        );
                        if b_decimate != 0 {
                            let mut i_decimate_8x8: ::core::ffi::c_int = (*h)
                                .quantf
                                .decimate_score64
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*(*h)
                                    .dct
                                    .luma8x8
                                    .as_mut_ptr()
                                    .offset((p_7 * 4 as ::core::ffi::c_int + idx) as isize))
                                    .as_mut_ptr(),
                            );
                            i_decimate_mb += i_decimate_8x8;
                            if i_decimate_8x8 >= 4 as ::core::ffi::c_int {
                                plane_cbp |= (1 as ::core::ffi::c_int) << idx;
                            }
                        } else {
                            plane_cbp |= (1 as ::core::ffi::c_int) << idx;
                        }
                    }
                    idx += 1;
                }
                if i_decimate_mb >= 6 as ::core::ffi::c_int || b_decimate == 0 {
                    (*h).mb.i_cbp_luma |= plane_cbp;
                    let mut idx_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut msk: ::core::ffi::c_int = plane_cbp;
                    let mut skip: ::core::ffi::c_int = 0;
                    while msk != 0
                        && {
                            skip = x264_ctz_4bit(msk as uint32_t);
                            idx_0 += skip;
                            msk >>= skip + 1 as ::core::ffi::c_int;
                            1 as ::core::ffi::c_int != 0
                        }
                    {
                        (*h)
                            .quantf
                            .dequant_8x8
                            .expect(
                                "non-null function pointer",
                            )(
                            (*dct8x8.as_mut_ptr().offset(idx_0 as isize)).as_mut_ptr(),
                            (*h).dequant8_mf[quant_cat as usize],
                            i_qp,
                        );
                        (*h)
                            .dctf
                            .add8x8_idct8
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *(*(*h).mb.pic.p_fdec.as_mut_ptr().offset(p_7 as isize))
                                .offset(
                                    (8 as ::core::ffi::c_int * (idx_0 & 1 as ::core::ffi::c_int)
                                        + 8 as ::core::ffi::c_int
                                            * (idx_0 >> 1 as ::core::ffi::c_int) * FDEC_STRIDE) as isize,
                                ),
                            (*dct8x8.as_mut_ptr().offset(idx_0 as isize)).as_mut_ptr(),
                        );
                        (*(&mut *(*h)
                            .mb
                            .cache
                            .non_zero_count
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_7 * 16 as ::core::ffi::c_int
                                            + idx_0 * 4 as ::core::ffi::c_int) as isize,
                                    ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                            as uint16_t;
                        (*(&mut *(*h)
                            .mb
                            .cache
                            .non_zero_count
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_7 * 16 as ::core::ffi::c_int
                                            + idx_0 * 4 as ::core::ffi::c_int) as isize,
                                    ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                            as uint16_t;
                        idx_0 += 1;
                    }
                }
                p_7 += 1;
                i_qp = (*h).mb.i_chroma_qp;
            }
        } else {
            let mut dct4x4: [[dctcoef; 16]; 16] = [[0; 16]; 16];
            let mut p_8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p_8 < plane_count {
                let mut quant_cat_0: ::core::ffi::c_int = if p_8 != 0 {
                    CQM_4PC as ::core::ffi::c_int
                } else {
                    CQM_4PY as ::core::ffi::c_int
                };
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * p_8) as isize)
                            as ::core::ffi::c_int
                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as uint32_t;
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * p_8) as isize)
                            as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as uint32_t;
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * p_8) as isize)
                            as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as uint32_t;
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset((16 as ::core::ffi::c_int * p_8) as isize)
                            as ::core::ffi::c_int
                            + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union32_t))
                    .i = 0 as uint32_t;
                (*h)
                    .dctf
                    .sub16x16_dct
                    .expect(
                        "non-null function pointer",
                    )(
                    dct4x4.as_mut_ptr(),
                    (*h).mb.pic.p_fenc[p_8 as usize],
                    (*h).mb.pic.p_fdec[p_8 as usize],
                );
                if (*h).mb.b_noise_reduction != 0 {
                    let ref mut fresh1 = *(*h)
                        .nr_count
                        .offset(
                            (0 as ::core::ffi::c_int
                                + (p_8 != 0) as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int) as isize,
                        );
                    *fresh1 = (*fresh1).wrapping_add(16 as uint32_t);
                    let mut idx_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while idx_1 < 16 as ::core::ffi::c_int {
                        (*h)
                            .quantf
                            .denoise_dct
                            .expect(
                                "non-null function pointer",
                            )(
                            (*dct4x4.as_mut_ptr().offset(idx_1 as isize)).as_mut_ptr(),
                            (*(*h)
                                .nr_residual_sum
                                .offset(
                                    (0 as ::core::ffi::c_int
                                        + (p_8 != 0) as ::core::ffi::c_int
                                            * 2 as ::core::ffi::c_int) as isize,
                                ))
                                .as_mut_ptr(),
                            (*(*h)
                                .nr_offset
                                .offset(
                                    (0 as ::core::ffi::c_int
                                        + (p_8 != 0) as ::core::ffi::c_int
                                            * 2 as ::core::ffi::c_int) as isize,
                                ))
                                .as_mut_ptr(),
                            16 as ::core::ffi::c_int,
                        );
                        idx_1 += 1;
                    }
                }
                let mut plane_cbp_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut i8x8_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i8x8_0 < 4 as ::core::ffi::c_int {
                    let mut i_decimate_8x8_0: ::core::ffi::c_int = if b_decimate != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        6 as ::core::ffi::c_int
                    };
                    let mut nnz8x8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if (*h).mb.b_trellis != 0 {
                        let mut i4x4_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i4x4_0 < 4 as ::core::ffi::c_int {
                            let mut idx_2: ::core::ffi::c_int = i8x8_0
                                * 4 as ::core::ffi::c_int + i4x4_0;
                            if x264_10_quant_4x4_trellis(
                                h,
                                (*dct4x4.as_mut_ptr().offset(idx_2 as isize)).as_mut_ptr(),
                                quant_cat_0,
                                i_qp,
                                ctx_cat_plane[DCT_LUMA_4x4 as ::core::ffi::c_int
                                    as usize][p_8 as usize] as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                (p_8 != 0) as ::core::ffi::c_int,
                                p_8 * 16 as ::core::ffi::c_int + idx_2,
                            ) != 0
                            {
                                (*h)
                                    .zigzagf
                                    .scan_4x4
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*(*h)
                                        .dct
                                        .luma4x4
                                        .as_mut_ptr()
                                        .offset((p_8 * 16 as ::core::ffi::c_int + idx_2) as isize))
                                        .as_mut_ptr(),
                                    (*dct4x4.as_mut_ptr().offset(idx_2 as isize)).as_mut_ptr(),
                                );
                                (*h)
                                    .quantf
                                    .dequant_4x4
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*dct4x4.as_mut_ptr().offset(idx_2 as isize)).as_mut_ptr(),
                                    (*h).dequant4_mf[quant_cat_0 as usize],
                                    i_qp,
                                );
                                if i_decimate_8x8_0 < 6 as ::core::ffi::c_int {
                                    i_decimate_8x8_0
                                        += (*h)
                                            .quantf
                                            .decimate_score16
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            (*(*h)
                                                .dct
                                                .luma4x4
                                                .as_mut_ptr()
                                                .offset((p_8 * 16 as ::core::ffi::c_int + idx_2) as isize))
                                                .as_mut_ptr(),
                                        );
                                }
                                (*h)
                                    .mb
                                    .cache
                                    .non_zero_count[x264_scan8[(p_8 * 16 as ::core::ffi::c_int
                                    + idx_2) as usize] as usize] = 1 as uint8_t;
                                nnz8x8 = 1 as ::core::ffi::c_int;
                            }
                            i4x4_0 += 1;
                        }
                    } else {
                        nz = (*h)
                            .quantf
                            .quant_4x4x4
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *dct4x4
                                .as_mut_ptr()
                                .offset((i8x8_0 * 4 as ::core::ffi::c_int) as isize),
                            (*(*(*h).quant4_mf.as_mut_ptr().offset(quant_cat_0 as isize))
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                            (*(*(*h)
                                .quant4_bias
                                .as_mut_ptr()
                                .offset(quant_cat_0 as isize))
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                        );
                        nnz8x8 = nz;
                        if nz != 0 {
                            let mut idx_3: ::core::ffi::c_int = i8x8_0
                                * 4 as ::core::ffi::c_int;
                            let mut msk_0: ::core::ffi::c_int = nz;
                            let mut skip_0: ::core::ffi::c_int = 0;
                            while msk_0 != 0
                                && {
                                    skip_0 = x264_ctz_4bit(msk_0 as uint32_t);
                                    idx_3 += skip_0;
                                    msk_0 >>= skip_0 + 1 as ::core::ffi::c_int;
                                    1 as ::core::ffi::c_int != 0
                                }
                            {
                                (*h)
                                    .zigzagf
                                    .scan_4x4
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*(*h)
                                        .dct
                                        .luma4x4
                                        .as_mut_ptr()
                                        .offset((p_8 * 16 as ::core::ffi::c_int + idx_3) as isize))
                                        .as_mut_ptr(),
                                    (*dct4x4.as_mut_ptr().offset(idx_3 as isize)).as_mut_ptr(),
                                );
                                (*h)
                                    .quantf
                                    .dequant_4x4
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*dct4x4.as_mut_ptr().offset(idx_3 as isize)).as_mut_ptr(),
                                    (*h).dequant4_mf[quant_cat_0 as usize],
                                    i_qp,
                                );
                                if i_decimate_8x8_0 < 6 as ::core::ffi::c_int {
                                    i_decimate_8x8_0
                                        += (*h)
                                            .quantf
                                            .decimate_score16
                                            .expect(
                                                "non-null function pointer",
                                            )(
                                            (*(*h)
                                                .dct
                                                .luma4x4
                                                .as_mut_ptr()
                                                .offset((p_8 * 16 as ::core::ffi::c_int + idx_3) as isize))
                                                .as_mut_ptr(),
                                        );
                                }
                                (*h)
                                    .mb
                                    .cache
                                    .non_zero_count[x264_scan8[(p_8 * 16 as ::core::ffi::c_int
                                    + idx_3) as usize] as usize] = 1 as uint8_t;
                                idx_3 += 1;
                            }
                        }
                    }
                    if nnz8x8 != 0 {
                        i_decimate_mb += i_decimate_8x8_0;
                        if i_decimate_8x8_0 < 4 as ::core::ffi::c_int {
                            (*(&mut *(*h)
                                .mb
                                .cache
                                .non_zero_count
                                .as_mut_ptr()
                                .offset(
                                    (*x264_scan8
                                        .as_ptr()
                                        .offset(
                                            (p_8 * 16 as ::core::ffi::c_int
                                                + i8x8_0 * 4 as ::core::ffi::c_int) as isize,
                                        ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                                ) as *mut uint8_t as *mut x264_union16_t))
                                .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                                as uint16_t;
                            (*(&mut *(*h)
                                .mb
                                .cache
                                .non_zero_count
                                .as_mut_ptr()
                                .offset(
                                    (*x264_scan8
                                        .as_ptr()
                                        .offset(
                                            (p_8 * 16 as ::core::ffi::c_int
                                                + i8x8_0 * 4 as ::core::ffi::c_int) as isize,
                                        ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                                ) as *mut uint8_t as *mut x264_union16_t))
                                .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                                as uint16_t;
                        } else {
                            plane_cbp_0 |= (1 as ::core::ffi::c_int) << i8x8_0;
                        }
                    }
                    i8x8_0 += 1;
                }
                if i_decimate_mb < 6 as ::core::ffi::c_int {
                    plane_cbp_0 = 0 as ::core::ffi::c_int;
                    (*(&mut *(*h)
                        .mb
                        .cache
                        .non_zero_count
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((16 as ::core::ffi::c_int * p_8) as isize)
                                as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as uint32_t;
                    (*(&mut *(*h)
                        .mb
                        .cache
                        .non_zero_count
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((16 as ::core::ffi::c_int * p_8) as isize)
                                as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as uint32_t;
                    (*(&mut *(*h)
                        .mb
                        .cache
                        .non_zero_count
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((16 as ::core::ffi::c_int * p_8) as isize)
                                as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as uint32_t;
                    (*(&mut *(*h)
                        .mb
                        .cache
                        .non_zero_count
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset((16 as ::core::ffi::c_int * p_8) as isize)
                                as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut uint8_t as *mut x264_union32_t))
                        .i = 0 as uint32_t;
                } else {
                    (*h).mb.i_cbp_luma |= plane_cbp_0;
                    let mut i8x8_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut msk_1: ::core::ffi::c_int = plane_cbp_0;
                    let mut skip_1: ::core::ffi::c_int = 0;
                    while msk_1 != 0
                        && {
                            skip_1 = x264_ctz_4bit(msk_1 as uint32_t);
                            i8x8_1 += skip_1;
                            msk_1 >>= skip_1 + 1 as ::core::ffi::c_int;
                            1 as ::core::ffi::c_int != 0
                        }
                    {
                        (*h)
                            .dctf
                            .add8x8_idct
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *(*(*h).mb.pic.p_fdec.as_mut_ptr().offset(p_8 as isize))
                                .offset(
                                    ((i8x8_1 & 1 as ::core::ffi::c_int)
                                        * 8 as ::core::ffi::c_int
                                        + (i8x8_1 >> 1 as ::core::ffi::c_int)
                                            * 8 as ::core::ffi::c_int * FDEC_STRIDE) as isize,
                                ),
                            &mut *dct4x4
                                .as_mut_ptr()
                                .offset((i8x8_1 * 4 as ::core::ffi::c_int) as isize),
                        );
                        i8x8_1 += 1;
                    }
                }
                p_8 += 1;
                i_qp = (*h).mb.i_chroma_qp;
            }
        }
    }
    if chroma != 0 {
        if (*h).mb.i_type == I_4x4 as ::core::ffi::c_int
            || (*h).mb.i_type == I_8x8 as ::core::ffi::c_int
            || (*h).mb.i_type == I_16x16 as ::core::ffi::c_int
            || (*h).mb.i_type == I_PCM as ::core::ffi::c_int
        {
            let mut i_mode_1: ::core::ffi::c_int = (*h).mb.i_chroma_pred_mode;
            if (*h).mb.b_lossless != 0 {
                x264_10_predict_lossless_chroma(h, i_mode_1);
            } else {
                (*h)
                    .predict_chroma[i_mode_1 as usize]
                    .expect(
                        "non-null function pointer",
                    )((*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize]);
                (*h)
                    .predict_chroma[i_mode_1 as usize]
                    .expect(
                        "non-null function pointer",
                    )((*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize]);
            }
        }
        x264_10_mb_encode_chroma(
            h,
            !((*h).mb.i_type == I_4x4 as ::core::ffi::c_int
                || (*h).mb.i_type == I_8x8 as ::core::ffi::c_int
                || (*h).mb.i_type == I_16x16 as ::core::ffi::c_int
                || (*h).mb.i_type == I_PCM as ::core::ffi::c_int) as ::core::ffi::c_int,
            (*h).mb.i_chroma_qp,
        );
    } else {
        (*h).mb.i_cbp_chroma = 0 as ::core::ffi::c_int;
    }
    let mut cbp: ::core::ffi::c_int = (*h).mb.i_cbp_chroma << 4 as ::core::ffi::c_int
        | (*h).mb.i_cbp_luma;
    if (*h).param.b_cabac != 0 {
        cbp
            |= ((*h).mb.cache.non_zero_count[x264_scan8[LUMA_DC as usize] as usize]
                as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | ((*h)
                    .mb
                    .cache
                    .non_zero_count[x264_scan8[(CHROMA_DC + 0 as ::core::ffi::c_int)
                    as usize] as usize] as ::core::ffi::c_int) << 9 as ::core::ffi::c_int
                | ((*h)
                    .mb
                    .cache
                    .non_zero_count[x264_scan8[(CHROMA_DC + 1 as ::core::ffi::c_int)
                    as usize] as usize] as ::core::ffi::c_int)
                    << 10 as ::core::ffi::c_int;
    }
    *(*h).mb.cbp.offset((*h).mb.i_mb_xy as isize) = cbp as int16_t;
    if b_force_no_skip == 0 {
        if (*h).mb.i_type == P_L0 as ::core::ffi::c_int
            && (*h).mb.i_partition == D_16x16 as ::core::ffi::c_int
            && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
            && (*((*(*(*h)
                .mb
                .cache
                .mv
                .as_mut_ptr()
                .offset(0 as ::core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(
                    *x264_scan8.as_ptr().offset(0 as ::core::ffi::c_int as isize)
                        as isize,
                ))
                .as_mut_ptr() as *mut x264_union32_t))
                .i == (*((*h).mb.cache.pskip_mv.as_mut_ptr() as *mut x264_union32_t)).i
            && (*h)
                .mb
                .cache
                .ref_0[0 as ::core::ffi::c_int
                as usize][x264_scan8[0 as ::core::ffi::c_int as usize] as usize]
                as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*h).mb.i_type = P_SKIP as ::core::ffi::c_int;
        }
        if (*h).mb.i_type == B_DIRECT as ::core::ffi::c_int
            && (*h).mb.i_cbp_luma | (*h).mb.i_cbp_chroma == 0
        {
            (*h).mb.i_type = B_SKIP as ::core::ffi::c_int;
        }
    }
}
#[no_mangle]
#[c2rust::src_loc = "974:1"]
pub unsafe extern "C" fn x264_10_macroblock_encode(mut h: *mut x264_t) {
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
        macroblock_encode_internal(h, 3 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc != 0 {
        macroblock_encode_internal(h, 1 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
    } else {
        macroblock_encode_internal(h, 1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    };
}
#[inline(always)]
#[c2rust::src_loc = "988:1"]
unsafe extern "C" fn macroblock_probe_skip_internal(
    mut h: *mut x264_t,
    mut b_bidir: ::core::ffi::c_int,
    mut plane_count: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dct4x4: [[dctcoef; 16]; 8] = [[0; 16]; 8];
    let mut dctscan: [dctcoef; 16] = [0; 16];
    let mut mvp: [int16_t; 2] = [0; 2];
    let mut i_qp: ::core::ffi::c_int = (*h).mb.i_qp;
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while p < plane_count {
        let mut quant_cat: ::core::ffi::c_int = if p != 0 {
            CQM_4PC as ::core::ffi::c_int
        } else {
            CQM_4PY as ::core::ffi::c_int
        };
        if b_bidir == 0 {
            mvp[0 as ::core::ffi::c_int as usize] = x264_clip3(
                (*h).mb.cache.pskip_mv[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int,
                (*h).mb.mv_min[0 as ::core::ffi::c_int as usize],
                (*h).mb.mv_max[0 as ::core::ffi::c_int as usize],
            ) as int16_t;
            mvp[1 as ::core::ffi::c_int as usize] = x264_clip3(
                (*h).mb.cache.pskip_mv[1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int,
                (*h).mb.mv_min[1 as ::core::ffi::c_int as usize],
                (*h).mb.mv_max[1 as ::core::ffi::c_int as usize],
            ) as int16_t;
            (*h)
                .mc
                .mc_luma
                .expect(
                    "non-null function pointer",
                )(
                (*h).mb.pic.p_fdec[p as usize],
                FDEC_STRIDE as intptr_t,
                &mut *(*(*(*h)
                    .mb
                    .pic
                    .p_fref
                    .as_mut_ptr()
                    .offset(0 as ::core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(0 as ::core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset((p * 4 as ::core::ffi::c_int) as isize),
                (*h).mb.pic.i_stride[p as usize] as intptr_t,
                mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                16 as ::core::ffi::c_int,
                &mut *(*(*h)
                    .sh
                    .weight
                    .as_mut_ptr()
                    .offset(0 as ::core::ffi::c_int as isize))
                    .as_mut_ptr()
                    .offset(p as isize),
            );
        }
        let mut i8x8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut i_decimate_mb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i8x8 < 4 as ::core::ffi::c_int {
            let mut fenc_offset: ::core::ffi::c_int = (i8x8 & 1 as ::core::ffi::c_int)
                * 8 as ::core::ffi::c_int
                + (i8x8 >> 1 as ::core::ffi::c_int) * FENC_STRIDE
                    * 8 as ::core::ffi::c_int;
            let mut fdec_offset: ::core::ffi::c_int = (i8x8 & 1 as ::core::ffi::c_int)
                * 8 as ::core::ffi::c_int
                + (i8x8 >> 1 as ::core::ffi::c_int) * FDEC_STRIDE
                    * 8 as ::core::ffi::c_int;
            (*h)
                .dctf
                .sub8x8_dct
                .expect(
                    "non-null function pointer",
                )(
                dct4x4.as_mut_ptr(),
                (*h).mb.pic.p_fenc[p as usize].offset(fenc_offset as isize),
                (*h).mb.pic.p_fdec[p as usize].offset(fdec_offset as isize),
            );
            if (*h).mb.b_noise_reduction != 0 {
                let mut i4x4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i4x4 < 4 as ::core::ffi::c_int {
                    (*h)
                        .quantf
                        .denoise_dct
                        .expect(
                            "non-null function pointer",
                        )(
                        (*dct4x4.as_mut_ptr().offset(i4x4 as isize)).as_mut_ptr(),
                        (*(*h)
                            .nr_residual_sum
                            .offset(
                                (0 as ::core::ffi::c_int
                                    + (p != 0) as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .as_mut_ptr(),
                        (*(*h)
                            .nr_offset
                            .offset(
                                (0 as ::core::ffi::c_int
                                    + (p != 0) as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as isize,
                            ))
                            .as_mut_ptr(),
                        16 as ::core::ffi::c_int,
                    );
                    i4x4 += 1;
                }
            }
            let mut nz: ::core::ffi::c_int = (*h)
                .quantf
                .quant_4x4x4
                .expect(
                    "non-null function pointer",
                )(
                dct4x4.as_mut_ptr(),
                (*(*(*h).quant4_mf.as_mut_ptr().offset(quant_cat as isize))
                    .offset(i_qp as isize))
                    .as_mut_ptr(),
                (*(*(*h).quant4_bias.as_mut_ptr().offset(quant_cat as isize))
                    .offset(i_qp as isize))
                    .as_mut_ptr(),
            );
            let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut msk: ::core::ffi::c_int = nz;
            let mut skip: ::core::ffi::c_int = 0;
            while msk != 0
                && {
                    skip = x264_ctz_4bit(msk as uint32_t);
                    idx += skip;
                    msk >>= skip + 1 as ::core::ffi::c_int;
                    1 as ::core::ffi::c_int != 0
                }
            {
                (*h)
                    .zigzagf
                    .scan_4x4
                    .expect(
                        "non-null function pointer",
                    )(
                    dctscan.as_mut_ptr(),
                    (*dct4x4.as_mut_ptr().offset(idx as isize)).as_mut_ptr(),
                );
                i_decimate_mb
                    += (*h)
                        .quantf
                        .decimate_score16
                        .expect("non-null function pointer")(dctscan.as_mut_ptr());
                if i_decimate_mb >= 6 as ::core::ffi::c_int {
                    return 0 as ::core::ffi::c_int;
                }
                idx += 1;
            }
            i8x8 += 1;
        }
        p += 1;
        i_qp = (*h).mb.i_chroma_qp;
    }
    if chroma == CHROMA_420 as ::core::ffi::c_int
        || chroma == CHROMA_422 as ::core::ffi::c_int
    {
        i_qp = (*h).mb.i_chroma_qp;
        let mut chroma422: ::core::ffi::c_int = (chroma
            == CHROMA_422 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let mut thresh: ::core::ffi::c_int = if chroma422 != 0 {
            x264_lambda2_tab[i_qp as usize] + 16 as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int
        } else {
            x264_lambda2_tab[i_qp as usize] + 32 as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int
        };
        let mut ssd: ::core::ffi::c_int = 0;
        let mut dct_dc: [dctcoef; 8] = [0; 8];
        if b_bidir == 0 {
            if (*(mvp.as_mut_ptr() as *mut x264_union32_t)).i != 0 {
                (*h)
                    .mc
                    .mc_chroma
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
                    (*h).mb.pic.p_fdec[2 as ::core::ffi::c_int as usize],
                    FDEC_STRIDE as intptr_t,
                    (*h)
                        .mb
                        .pic
                        .p_fref[0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int
                        as usize][4 as ::core::ffi::c_int as usize],
                    (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    mvp[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    mvp[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        * ((1 as ::core::ffi::c_int) << chroma422),
                    8 as ::core::ffi::c_int,
                    if chroma422 != 0 {
                        16 as ::core::ffi::c_int
                    } else {
                        8 as ::core::ffi::c_int
                    },
                );
            } else {
                (*h)
                    .mc
                    .load_deinterleave_chroma_fdec
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).mb.pic.p_fdec[1 as ::core::ffi::c_int as usize],
                    (*h)
                        .mb
                        .pic
                        .p_fref[0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int
                        as usize][4 as ::core::ffi::c_int as usize],
                    (*h).mb.pic.i_stride[1 as ::core::ffi::c_int as usize] as intptr_t,
                    if chroma422 != 0 {
                        16 as ::core::ffi::c_int
                    } else {
                        8 as ::core::ffi::c_int
                    },
                );
            }
        }
        let mut ch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while ch < 2 as ::core::ffi::c_int {
            let mut p_src: *mut pixel = (*h)
                .mb
                .pic
                .p_fenc[(1 as ::core::ffi::c_int + ch) as usize];
            let mut p_dst: *mut pixel = (*h)
                .mb
                .pic
                .p_fdec[(1 as ::core::ffi::c_int + ch) as usize];
            if b_bidir == 0
                && !(*h)
                    .sh
                    .weight[0 as ::core::ffi::c_int
                        as usize][(1 as ::core::ffi::c_int + ch) as usize]
                    .weightfn
                    .is_null()
            {
                (*(*h)
                    .sh
                    .weight[0 as ::core::ffi::c_int
                        as usize][(1 as ::core::ffi::c_int + ch) as usize]
                    .weightfn
                    .offset(
                        (8 as ::core::ffi::c_int >> 2 as ::core::ffi::c_int) as isize,
                    ))
                    .expect(
                        "non-null function pointer",
                    )(
                    (*h).mb.pic.p_fdec[(1 as ::core::ffi::c_int + ch) as usize],
                    FDEC_STRIDE as intptr_t,
                    (*h).mb.pic.p_fdec[(1 as ::core::ffi::c_int + ch) as usize],
                    FDEC_STRIDE as intptr_t,
                    &mut *(*(*h)
                        .sh
                        .weight
                        .as_mut_ptr()
                        .offset(0 as ::core::ffi::c_int as isize))
                        .as_mut_ptr()
                        .offset((1 as ::core::ffi::c_int + ch) as isize),
                    if chroma422 != 0 {
                        16 as ::core::ffi::c_int
                    } else {
                        8 as ::core::ffi::c_int
                    },
                );
            }
            ssd = (*h)
                .pixf
                .ssd[(if chroma422 != 0 {
                    PIXEL_8x16 as ::core::ffi::c_int
                } else {
                    PIXEL_8x8 as ::core::ffi::c_int
                }) as usize]
                .expect(
                    "non-null function pointer",
                )(p_dst, FDEC_STRIDE as intptr_t, p_src, FENC_STRIDE as intptr_t);
            if !(ssd < thresh) {
                if (*h).mb.b_noise_reduction != 0 {
                    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i <= chroma422 {
                        (*h)
                            .dctf
                            .sub8x8_dct
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *dct4x4
                                .as_mut_ptr()
                                .offset((4 as ::core::ffi::c_int * i) as isize),
                            p_src
                                .offset(
                                    (8 as ::core::ffi::c_int * i * FENC_STRIDE) as isize,
                                ),
                            p_dst
                                .offset(
                                    (8 as ::core::ffi::c_int * i * FDEC_STRIDE) as isize,
                                ),
                        );
                        i += 1;
                    }
                    let mut i4x4_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i4x4_0
                        < (if chroma422 != 0 {
                            8 as ::core::ffi::c_int
                        } else {
                            4 as ::core::ffi::c_int
                        })
                    {
                        (*h)
                            .quantf
                            .denoise_dct
                            .expect(
                                "non-null function pointer",
                            )(
                            (*dct4x4.as_mut_ptr().offset(i4x4_0 as isize)).as_mut_ptr(),
                            (*(*h)
                                .nr_residual_sum
                                .offset(2 as ::core::ffi::c_int as isize))
                                .as_mut_ptr(),
                            (*(*h).nr_offset.offset(2 as ::core::ffi::c_int as isize))
                                .as_mut_ptr(),
                            16 as ::core::ffi::c_int,
                        );
                        dct_dc[i4x4_0 as usize] = dct4x4[i4x4_0
                            as usize][0 as ::core::ffi::c_int as usize];
                        dct4x4[i4x4_0 as usize][0 as ::core::ffi::c_int as usize] = 0
                            as ::core::ffi::c_int as dctcoef;
                        i4x4_0 += 1;
                    }
                } else if chroma422 != 0 {
                    (*h)
                        .dctf
                        .sub8x16_dct_dc
                        .expect(
                            "non-null function pointer",
                        )(dct_dc.as_mut_ptr(), p_src, p_dst);
                } else {
                    (*h)
                        .dctf
                        .sub8x8_dct_dc
                        .expect(
                            "non-null function pointer",
                        )(dct_dc.as_mut_ptr(), p_src, p_dst);
                }
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 <= chroma422 {
                    if (*h)
                        .quantf
                        .quant_2x2_dc
                        .expect(
                            "non-null function pointer",
                        )(
                        &mut *dct_dc
                            .as_mut_ptr()
                            .offset((4 as ::core::ffi::c_int * i_0) as isize),
                        ((*(*h)
                            .quant4_mf[CQM_4PC as ::core::ffi::c_int as usize]
                            .offset(
                                (i_qp + 3 as ::core::ffi::c_int * chroma422) as isize,
                            ))[0 as ::core::ffi::c_int as usize]
                            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                        ((*(*h)
                            .quant4_bias[CQM_4PC as ::core::ffi::c_int as usize]
                            .offset(
                                (i_qp + 3 as ::core::ffi::c_int * chroma422) as isize,
                            ))[0 as ::core::ffi::c_int as usize]
                            << 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
                    ) != 0
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    i_0 += 1;
                }
                if !(ssd < thresh * 4 as ::core::ffi::c_int) {
                    if (*h).mb.b_noise_reduction == 0 {
                        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_1 <= chroma422 {
                            (*h)
                                .dctf
                                .sub8x8_dct
                                .expect(
                                    "non-null function pointer",
                                )(
                                &mut *dct4x4
                                    .as_mut_ptr()
                                    .offset((4 as ::core::ffi::c_int * i_1) as isize),
                                p_src
                                    .offset(
                                        (8 as ::core::ffi::c_int * i_1 * FENC_STRIDE) as isize,
                                    ),
                                p_dst
                                    .offset(
                                        (8 as ::core::ffi::c_int * i_1 * FDEC_STRIDE) as isize,
                                    ),
                            );
                            dct4x4[(i_1 * 4 as ::core::ffi::c_int
                                + 0 as ::core::ffi::c_int)
                                as usize][0 as ::core::ffi::c_int as usize] = 0
                                as ::core::ffi::c_int as dctcoef;
                            dct4x4[(i_1 * 4 as ::core::ffi::c_int
                                + 1 as ::core::ffi::c_int)
                                as usize][0 as ::core::ffi::c_int as usize] = 0
                                as ::core::ffi::c_int as dctcoef;
                            dct4x4[(i_1 * 4 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int)
                                as usize][0 as ::core::ffi::c_int as usize] = 0
                                as ::core::ffi::c_int as dctcoef;
                            dct4x4[(i_1 * 4 as ::core::ffi::c_int
                                + 3 as ::core::ffi::c_int)
                                as usize][0 as ::core::ffi::c_int as usize] = 0
                                as ::core::ffi::c_int as dctcoef;
                            i_1 += 1;
                        }
                    }
                    let mut i8x8_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut i_decimate_mb_0: ::core::ffi::c_int = 0
                        as ::core::ffi::c_int;
                    while i8x8_0
                        < (if chroma422 != 0 {
                            2 as ::core::ffi::c_int
                        } else {
                            1 as ::core::ffi::c_int
                        })
                    {
                        let mut nz_0: ::core::ffi::c_int = (*h)
                            .quantf
                            .quant_4x4x4
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut *dct4x4
                                .as_mut_ptr()
                                .offset((i8x8_0 * 4 as ::core::ffi::c_int) as isize),
                            (*(*(*h)
                                .quant4_mf
                                .as_mut_ptr()
                                .offset(CQM_4PC as ::core::ffi::c_int as isize))
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                            (*(*(*h)
                                .quant4_bias
                                .as_mut_ptr()
                                .offset(CQM_4PC as ::core::ffi::c_int as isize))
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                        );
                        let mut idx_0: ::core::ffi::c_int = i8x8_0
                            * 4 as ::core::ffi::c_int;
                        let mut msk_0: ::core::ffi::c_int = nz_0;
                        let mut skip_0: ::core::ffi::c_int = 0;
                        while msk_0 != 0
                            && {
                                skip_0 = x264_ctz_4bit(msk_0 as uint32_t);
                                idx_0 += skip_0;
                                msk_0 >>= skip_0 + 1 as ::core::ffi::c_int;
                                1 as ::core::ffi::c_int != 0
                            }
                        {
                            (*h)
                                .zigzagf
                                .scan_4x4
                                .expect(
                                    "non-null function pointer",
                                )(
                                dctscan.as_mut_ptr(),
                                (*dct4x4.as_mut_ptr().offset(idx_0 as isize)).as_mut_ptr(),
                            );
                            i_decimate_mb_0
                                += (*h)
                                    .quantf
                                    .decimate_score15
                                    .expect("non-null function pointer")(dctscan.as_mut_ptr());
                            if i_decimate_mb_0 >= 7 as ::core::ffi::c_int {
                                return 0 as ::core::ffi::c_int;
                            }
                            idx_0 += 1;
                        }
                        i8x8_0 += 1;
                    }
                }
            }
            ch += 1;
        }
    }
    (*h).mb.b_skip_mc = 1 as ::core::ffi::c_int;
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "1129:1"]
pub unsafe extern "C" fn x264_10_macroblock_probe_skip(
    mut h: *mut x264_t,
    mut b_bidir: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as ::core::ffi::c_int {
        return macroblock_probe_skip_internal(
            h,
            b_bidir,
            1 as ::core::ffi::c_int,
            CHROMA_420 as ::core::ffi::c_int,
        )
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
        == CHROMA_422 as ::core::ffi::c_int
    {
        return macroblock_probe_skip_internal(
            h,
            b_bidir,
            1 as ::core::ffi::c_int,
            CHROMA_422 as ::core::ffi::c_int,
        )
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
        == CHROMA_444 as ::core::ffi::c_int
    {
        return macroblock_probe_skip_internal(
            h,
            b_bidir,
            3 as ::core::ffi::c_int,
            CHROMA_444 as ::core::ffi::c_int,
        )
    } else {
        return macroblock_probe_skip_internal(
            h,
            b_bidir,
            1 as ::core::ffi::c_int,
            CHROMA_400 as ::core::ffi::c_int,
        )
    };
}
#[no_mangle]
#[c2rust::src_loc = "1146:1"]
pub unsafe extern "C" fn x264_10_noise_reduction_update(mut h: *mut x264_t) {
    (*h).nr_offset = (*h).nr_offset_denoise.as_mut_ptr() as *mut [udctcoef; 64];
    (*h).nr_residual_sum = (*(*h)
        .nr_residual_sum_buf
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
        .as_mut_ptr() as *mut [uint32_t; 64];
    (*h).nr_count = (*(*h)
        .nr_count_buf
        .as_mut_ptr()
        .offset(0 as ::core::ffi::c_int as isize))
        .as_mut_ptr();
    let mut cat: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while cat
        < 3 as ::core::ffi::c_int
            + ((*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                == CHROMA_444 as ::core::ffi::c_int) as ::core::ffi::c_int
    {
        let mut dct8x8: ::core::ffi::c_int = cat & 1 as ::core::ffi::c_int;
        let mut size: ::core::ffi::c_int = if dct8x8 != 0 {
            64 as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        };
        let mut weight: *const uint32_t = if dct8x8 != 0 {
            x264_dct8_weight2_tab.as_ptr()
        } else {
            x264_dct4_weight2_tab.as_ptr()
        };
        if *(*h).nr_count.offset(cat as isize)
            > (if dct8x8 != 0 {
                (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            } else {
                (1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int
            }) as uint32_t
        {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < size {
                (*(*h).nr_residual_sum.offset(cat as isize))[i as usize]
                    >>= 1 as ::core::ffi::c_int;
                i += 1;
            }
            *(*h).nr_count.offset(cat as isize) >>= 1 as ::core::ffi::c_int;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < size {
            (*(*h).nr_offset.offset(cat as isize))[i_0 as usize] = ((*h)
                .param
                .analyse
                .i_noise_reduction as uint64_t)
                .wrapping_mul(*(*h).nr_count.offset(cat as isize) as uint64_t)
                .wrapping_add(
                    (*(*h).nr_residual_sum.offset(cat as isize))[i_0 as usize]
                        .wrapping_div(2 as uint32_t) as uint64_t,
                )
                .wrapping_div(
                    ((*(*h).nr_residual_sum.offset(cat as isize))[i_0 as usize]
                        as uint64_t)
                        .wrapping_mul(*weight.offset(i_0 as isize) as uint64_t)
                        .wrapping_div(256 as uint64_t)
                        .wrapping_add(1 as uint64_t),
                ) as udctcoef;
            i_0 += 1;
        }
        (*(*h).nr_offset.offset(cat as isize))[0 as ::core::ffi::c_int as usize] = 0
            as udctcoef;
        cat += 1;
    }
}
#[inline(always)]
#[c2rust::src_loc = "1179:1"]
unsafe extern "C" fn macroblock_encode_p8x8_internal(
    mut h: *mut x264_t,
    mut i8: ::core::ffi::c_int,
    mut plane_count: ::core::ffi::c_int,
    mut chroma: ::core::ffi::c_int,
) {
    let mut b_decimate: ::core::ffi::c_int = (*h).mb.b_dct_decimate;
    let mut i_qp: ::core::ffi::c_int = (*h).mb.i_qp;
    let mut x: ::core::ffi::c_int = i8 & 1 as ::core::ffi::c_int;
    let mut y: ::core::ffi::c_int = i8 >> 1 as ::core::ffi::c_int;
    let mut nz: ::core::ffi::c_int = 0;
    let mut chroma422: ::core::ffi::c_int = (chroma == CHROMA_422 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
    (*h).mb.i_cbp_chroma = 0 as ::core::ffi::c_int;
    (*h).mb.i_cbp_luma &= !((1 as ::core::ffi::c_int) << i8);
    if (*h).mb.b_skip_mc == 0 {
        x264_10_mb_mc_8x8(h, i8);
    }
    if (*h).mb.b_lossless != 0 {
        let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while p < plane_count {
            let mut p_fenc: *mut pixel = (*h)
                .mb
                .pic
                .p_fenc[p as usize]
                .offset((8 as ::core::ffi::c_int * x) as isize)
                .offset((8 as ::core::ffi::c_int * y * FENC_STRIDE) as isize);
            let mut p_fdec: *mut pixel = (*h)
                .mb
                .pic
                .p_fdec[p as usize]
                .offset((8 as ::core::ffi::c_int * x) as isize)
                .offset((8 as ::core::ffi::c_int * y * FDEC_STRIDE) as isize);
            let mut nnz8x8: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if (*h).mb.b_transform_8x8 != 0 {
                nnz8x8 = (*h)
                    .zigzagf
                    .sub_8x8
                    .expect(
                        "non-null function pointer",
                    )(
                    (*(*h)
                        .dct
                        .luma8x8
                        .as_mut_ptr()
                        .offset((4 as ::core::ffi::c_int * p + i8) as isize))
                        .as_mut_ptr(),
                    p_fenc,
                    p_fdec,
                );
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset(
                                (p * 16 as ::core::ffi::c_int
                                    + i8 * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (nnz8x8 * 0x101 as ::core::ffi::c_int) as uint16_t;
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset(
                                (p * 16 as ::core::ffi::c_int
                                    + i8 * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (nnz8x8 * 0x101 as ::core::ffi::c_int) as uint16_t;
            } else {
                let mut i4: ::core::ffi::c_int = i8 * 4 as ::core::ffi::c_int;
                while i4 < i8 * 4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int {
                    nz = (*h)
                        .zigzagf
                        .sub_4x4
                        .expect(
                            "non-null function pointer",
                        )(
                        (*(*h)
                            .dct
                            .luma4x4
                            .as_mut_ptr()
                            .offset((16 as ::core::ffi::c_int * p + i4) as isize))
                            .as_mut_ptr(),
                        (*h)
                            .mb
                            .pic
                            .p_fenc[p as usize]
                            .offset(
                                block_idx_xy_fenc[i4 as usize] as ::core::ffi::c_int
                                    as isize,
                            ),
                        (*h)
                            .mb
                            .pic
                            .p_fdec[p as usize]
                            .offset(
                                block_idx_xy_fdec[i4 as usize] as ::core::ffi::c_int
                                    as isize,
                            ),
                    );
                    (*h)
                        .mb
                        .cache
                        .non_zero_count[x264_scan8[(16 as ::core::ffi::c_int * p + i4)
                        as usize] as usize] = nz as uint8_t;
                    nnz8x8 |= nz;
                    i4 += 1;
                }
            }
            (*h).mb.i_cbp_luma |= nnz8x8 << i8;
            p += 1;
        }
        if chroma == CHROMA_420 as ::core::ffi::c_int
            || chroma == CHROMA_422 as ::core::ffi::c_int
        {
            let mut ch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while ch < 2 as ::core::ffi::c_int {
                let mut dc: dctcoef = 0;
                let mut p_fenc_0: *mut pixel = (*h)
                    .mb
                    .pic
                    .p_fenc[(1 as ::core::ffi::c_int + ch) as usize]
                    .offset((4 as ::core::ffi::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as ::core::ffi::c_int
                        } else {
                            4 as ::core::ffi::c_int
                        }) * y * FENC_STRIDE) as isize,
                    );
                let mut p_fdec_0: *mut pixel = (*h)
                    .mb
                    .pic
                    .p_fdec[(1 as ::core::ffi::c_int + ch) as usize]
                    .offset((4 as ::core::ffi::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as ::core::ffi::c_int
                        } else {
                            4 as ::core::ffi::c_int
                        }) * y * FDEC_STRIDE) as isize,
                    );
                let mut i4x4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i4x4 <= chroma422 {
                    let mut offset: ::core::ffi::c_int = if chroma422 != 0 {
                        8 as ::core::ffi::c_int * y + 2 as ::core::ffi::c_int * i4x4 + x
                    } else {
                        i8
                    };
                    nz = (*h)
                        .zigzagf
                        .sub_4x4ac
                        .expect(
                            "non-null function pointer",
                        )(
                        (*(*h)
                            .dct
                            .luma4x4
                            .as_mut_ptr()
                            .offset(
                                (16 as ::core::ffi::c_int + offset
                                    + ch * 16 as ::core::ffi::c_int) as isize,
                            ))
                            .as_mut_ptr(),
                        p_fenc_0
                            .offset(
                                (4 as ::core::ffi::c_int * i4x4 * FENC_STRIDE) as isize,
                            ),
                        p_fdec_0
                            .offset(
                                (4 as ::core::ffi::c_int * i4x4 * FDEC_STRIDE) as isize,
                            ),
                        &mut dc,
                    );
                    (*h)
                        .mb
                        .cache
                        .non_zero_count[x264_scan8[(16 as ::core::ffi::c_int + offset
                        + ch * 16 as ::core::ffi::c_int) as usize] as usize] = nz
                        as uint8_t;
                    i4x4 += 1;
                }
                ch += 1;
            }
            (*h).mb.i_cbp_chroma = 0x2 as ::core::ffi::c_int;
        }
    } else {
        if (*h).mb.b_transform_8x8 != 0 {
            let mut p_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p_0 < plane_count {
                let mut quant_cat: ::core::ffi::c_int = if p_0 != 0 {
                    CQM_8PC as ::core::ffi::c_int
                } else {
                    CQM_8PY as ::core::ffi::c_int
                };
                let mut p_fenc_1: *mut pixel = (*h)
                    .mb
                    .pic
                    .p_fenc[p_0 as usize]
                    .offset((8 as ::core::ffi::c_int * x) as isize)
                    .offset((8 as ::core::ffi::c_int * y * FENC_STRIDE) as isize);
                let mut p_fdec_1: *mut pixel = (*h)
                    .mb
                    .pic
                    .p_fdec[p_0 as usize]
                    .offset((8 as ::core::ffi::c_int * x) as isize)
                    .offset((8 as ::core::ffi::c_int * y * FDEC_STRIDE) as isize);
                let mut dct8x8: [dctcoef; 64] = [0; 64];
                (*h)
                    .dctf
                    .sub8x8_dct8
                    .expect(
                        "non-null function pointer",
                    )(dct8x8.as_mut_ptr(), p_fenc_1, p_fdec_1);
                let mut nnz8x8_0: ::core::ffi::c_int = x264_quant_8x8(
                    h,
                    dct8x8.as_mut_ptr(),
                    i_qp,
                    ctx_cat_plane[DCT_LUMA_8x8 as ::core::ffi::c_int
                        as usize][p_0 as usize] as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    p_0,
                    i8,
                );
                if nnz8x8_0 != 0 {
                    (*h)
                        .zigzagf
                        .scan_8x8
                        .expect(
                            "non-null function pointer",
                        )(
                        (*(*h)
                            .dct
                            .luma8x8
                            .as_mut_ptr()
                            .offset((4 as ::core::ffi::c_int * p_0 + i8) as isize))
                            .as_mut_ptr(),
                        dct8x8.as_mut_ptr(),
                    );
                    if b_decimate != 0 && (*h).mb.b_trellis == 0 {
                        nnz8x8_0 = (4 as ::core::ffi::c_int
                            <= (*h)
                                .quantf
                                .decimate_score64
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*(*h)
                                    .dct
                                    .luma8x8
                                    .as_mut_ptr()
                                    .offset((4 as ::core::ffi::c_int * p_0 + i8) as isize))
                                    .as_mut_ptr(),
                            )) as ::core::ffi::c_int;
                    }
                    if nnz8x8_0 != 0 {
                        (*h)
                            .quantf
                            .dequant_8x8
                            .expect(
                                "non-null function pointer",
                            )(
                            dct8x8.as_mut_ptr(),
                            (*h).dequant8_mf[quant_cat as usize],
                            i_qp,
                        );
                        (*h)
                            .dctf
                            .add8x8_idct8
                            .expect(
                                "non-null function pointer",
                            )(p_fdec_1, dct8x8.as_mut_ptr());
                        (*(&mut *(*h)
                            .mb
                            .cache
                            .non_zero_count
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_0 * 16 as ::core::ffi::c_int
                                            + i8 * 4 as ::core::ffi::c_int) as isize,
                                    ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                            as uint16_t;
                        (*(&mut *(*h)
                            .mb
                            .cache
                            .non_zero_count
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_0 * 16 as ::core::ffi::c_int
                                            + i8 * 4 as ::core::ffi::c_int) as isize,
                                    ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (1 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                            as uint16_t;
                        (*h).mb.i_cbp_luma |= (1 as ::core::ffi::c_int) << i8;
                    } else {
                        (*(&mut *(*h)
                            .mb
                            .cache
                            .non_zero_count
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_0 * 16 as ::core::ffi::c_int
                                            + i8 * 4 as ::core::ffi::c_int) as isize,
                                    ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                            as uint16_t;
                        (*(&mut *(*h)
                            .mb
                            .cache
                            .non_zero_count
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_0 * 16 as ::core::ffi::c_int
                                            + i8 * 4 as ::core::ffi::c_int) as isize,
                                    ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                            as uint16_t;
                    }
                } else {
                    (*(&mut *(*h)
                        .mb
                        .cache
                        .non_zero_count
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset(
                                    (p_0 * 16 as ::core::ffi::c_int
                                        + i8 * 4 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                        .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                        as uint16_t;
                    (*(&mut *(*h)
                        .mb
                        .cache
                        .non_zero_count
                        .as_mut_ptr()
                        .offset(
                            (*x264_scan8
                                .as_ptr()
                                .offset(
                                    (p_0 * 16 as ::core::ffi::c_int
                                        + i8 * 4 as ::core::ffi::c_int) as isize,
                                ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                        ) as *mut uint8_t as *mut x264_union16_t))
                        .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                        as uint16_t;
                }
                p_0 += 1;
                i_qp = (*h).mb.i_chroma_qp;
            }
        } else {
            let mut p_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while p_1 < plane_count {
                let mut quant_cat_0: ::core::ffi::c_int = if p_1 != 0 {
                    CQM_4PC as ::core::ffi::c_int
                } else {
                    CQM_4PY as ::core::ffi::c_int
                };
                let mut p_fenc_2: *mut pixel = (*h)
                    .mb
                    .pic
                    .p_fenc[p_1 as usize]
                    .offset((8 as ::core::ffi::c_int * x) as isize)
                    .offset((8 as ::core::ffi::c_int * y * FENC_STRIDE) as isize);
                let mut p_fdec_2: *mut pixel = (*h)
                    .mb
                    .pic
                    .p_fdec[p_1 as usize]
                    .offset((8 as ::core::ffi::c_int * x) as isize)
                    .offset((8 as ::core::ffi::c_int * y * FDEC_STRIDE) as isize);
                let mut i_decimate_8x8: ::core::ffi::c_int = if b_decimate != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    4 as ::core::ffi::c_int
                };
                let mut dct4x4: [[dctcoef; 16]; 4] = [[0; 16]; 4];
                let mut nnz8x8_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                (*h)
                    .dctf
                    .sub8x8_dct
                    .expect(
                        "non-null function pointer",
                    )(dct4x4.as_mut_ptr(), p_fenc_2, p_fdec_2);
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset(
                                (p_1 * 16 as ::core::ffi::c_int
                                    + i8 * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                    as uint16_t;
                (*(&mut *(*h)
                    .mb
                    .cache
                    .non_zero_count
                    .as_mut_ptr()
                    .offset(
                        (*x264_scan8
                            .as_ptr()
                            .offset(
                                (p_1 * 16 as ::core::ffi::c_int
                                    + i8 * 4 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut x264_union16_t))
                    .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                    as uint16_t;
                if (*h).mb.b_noise_reduction != 0 {
                    let mut idx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while idx < 4 as ::core::ffi::c_int {
                        (*h)
                            .quantf
                            .denoise_dct
                            .expect(
                                "non-null function pointer",
                            )(
                            (*dct4x4.as_mut_ptr().offset(idx as isize)).as_mut_ptr(),
                            (*(*h)
                                .nr_residual_sum
                                .offset(
                                    (0 as ::core::ffi::c_int
                                        + (p_1 != 0) as ::core::ffi::c_int
                                            * 2 as ::core::ffi::c_int) as isize,
                                ))
                                .as_mut_ptr(),
                            (*(*h)
                                .nr_offset
                                .offset(
                                    (0 as ::core::ffi::c_int
                                        + (p_1 != 0) as ::core::ffi::c_int
                                            * 2 as ::core::ffi::c_int) as isize,
                                ))
                                .as_mut_ptr(),
                            16 as ::core::ffi::c_int,
                        );
                        idx += 1;
                    }
                }
                if (*h).mb.b_trellis != 0 {
                    let mut i4x4_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i4x4_0 < 4 as ::core::ffi::c_int {
                        if x264_10_quant_4x4_trellis(
                            h,
                            (*dct4x4.as_mut_ptr().offset(i4x4_0 as isize)).as_mut_ptr(),
                            quant_cat_0,
                            i_qp,
                            ctx_cat_plane[DCT_LUMA_4x4 as ::core::ffi::c_int
                                as usize][p_1 as usize] as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            (p_1 != 0) as ::core::ffi::c_int,
                            i8 * 4 as ::core::ffi::c_int + i4x4_0
                                + p_1 * 16 as ::core::ffi::c_int,
                        ) != 0
                        {
                            (*h)
                                .zigzagf
                                .scan_4x4
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*(*h)
                                    .dct
                                    .luma4x4
                                    .as_mut_ptr()
                                    .offset(
                                        (p_1 * 16 as ::core::ffi::c_int
                                            + i8 * 4 as ::core::ffi::c_int + i4x4_0) as isize,
                                    ))
                                    .as_mut_ptr(),
                                (*dct4x4.as_mut_ptr().offset(i4x4_0 as isize)).as_mut_ptr(),
                            );
                            (*h)
                                .quantf
                                .dequant_4x4
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*dct4x4.as_mut_ptr().offset(i4x4_0 as isize)).as_mut_ptr(),
                                (*h).dequant4_mf[quant_cat_0 as usize],
                                i_qp,
                            );
                            if i_decimate_8x8 < 4 as ::core::ffi::c_int {
                                i_decimate_8x8
                                    += (*h)
                                        .quantf
                                        .decimate_score16
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        (*(*h)
                                            .dct
                                            .luma4x4
                                            .as_mut_ptr()
                                            .offset(
                                                (p_1 * 16 as ::core::ffi::c_int
                                                    + i8 * 4 as ::core::ffi::c_int + i4x4_0) as isize,
                                            ))
                                            .as_mut_ptr(),
                                    );
                            }
                            (*h)
                                .mb
                                .cache
                                .non_zero_count[x264_scan8[(p_1 * 16 as ::core::ffi::c_int
                                + i8 * 4 as ::core::ffi::c_int + i4x4_0) as usize]
                                as usize] = 1 as uint8_t;
                            nnz8x8_1 = 1 as ::core::ffi::c_int;
                        }
                        i4x4_0 += 1;
                    }
                } else {
                    nz = (*h)
                        .quantf
                        .quant_4x4x4
                        .expect(
                            "non-null function pointer",
                        )(
                        dct4x4.as_mut_ptr(),
                        (*(*(*h).quant4_mf.as_mut_ptr().offset(quant_cat_0 as isize))
                            .offset(i_qp as isize))
                            .as_mut_ptr(),
                        (*(*(*h).quant4_bias.as_mut_ptr().offset(quant_cat_0 as isize))
                            .offset(i_qp as isize))
                            .as_mut_ptr(),
                    );
                    nnz8x8_1 = nz;
                    if nz != 0 {
                        let mut i4x4_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        let mut msk: ::core::ffi::c_int = nz;
                        let mut skip: ::core::ffi::c_int = 0;
                        while msk != 0
                            && {
                                skip = x264_ctz_4bit(msk as uint32_t);
                                i4x4_1 += skip;
                                msk >>= skip + 1 as ::core::ffi::c_int;
                                1 as ::core::ffi::c_int != 0
                            }
                        {
                            (*h)
                                .zigzagf
                                .scan_4x4
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*(*h)
                                    .dct
                                    .luma4x4
                                    .as_mut_ptr()
                                    .offset(
                                        (p_1 * 16 as ::core::ffi::c_int
                                            + i8 * 4 as ::core::ffi::c_int + i4x4_1) as isize,
                                    ))
                                    .as_mut_ptr(),
                                (*dct4x4.as_mut_ptr().offset(i4x4_1 as isize)).as_mut_ptr(),
                            );
                            (*h)
                                .quantf
                                .dequant_4x4
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*dct4x4.as_mut_ptr().offset(i4x4_1 as isize)).as_mut_ptr(),
                                (*h).dequant4_mf[quant_cat_0 as usize],
                                i_qp,
                            );
                            if i_decimate_8x8 < 4 as ::core::ffi::c_int {
                                i_decimate_8x8
                                    += (*h)
                                        .quantf
                                        .decimate_score16
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        (*(*h)
                                            .dct
                                            .luma4x4
                                            .as_mut_ptr()
                                            .offset(
                                                (p_1 * 16 as ::core::ffi::c_int
                                                    + i8 * 4 as ::core::ffi::c_int + i4x4_1) as isize,
                                            ))
                                            .as_mut_ptr(),
                                    );
                            }
                            (*h)
                                .mb
                                .cache
                                .non_zero_count[x264_scan8[(p_1 * 16 as ::core::ffi::c_int
                                + i8 * 4 as ::core::ffi::c_int + i4x4_1) as usize]
                                as usize] = 1 as uint8_t;
                            i4x4_1 += 1;
                        }
                    }
                }
                if nnz8x8_1 != 0 {
                    if i_decimate_8x8 < 4 as ::core::ffi::c_int {
                        (*(&mut *(*h)
                            .mb
                            .cache
                            .non_zero_count
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_1 * 16 as ::core::ffi::c_int
                                            + i8 * 4 as ::core::ffi::c_int) as isize,
                                    ) as ::core::ffi::c_int + 0 as ::core::ffi::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                            as uint16_t;
                        (*(&mut *(*h)
                            .mb
                            .cache
                            .non_zero_count
                            .as_mut_ptr()
                            .offset(
                                (*x264_scan8
                                    .as_ptr()
                                    .offset(
                                        (p_1 * 16 as ::core::ffi::c_int
                                            + i8 * 4 as ::core::ffi::c_int) as isize,
                                    ) as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as isize,
                            ) as *mut uint8_t as *mut x264_union16_t))
                            .i = (0 as ::core::ffi::c_int * 0x101 as ::core::ffi::c_int)
                            as uint16_t;
                    } else {
                        (*h)
                            .dctf
                            .add8x8_idct
                            .expect(
                                "non-null function pointer",
                            )(p_fdec_2, dct4x4.as_mut_ptr());
                        (*h).mb.i_cbp_luma |= (1 as ::core::ffi::c_int) << i8;
                    }
                }
                p_1 += 1;
                i_qp = (*h).mb.i_chroma_qp;
            }
        }
        if chroma == CHROMA_420 as ::core::ffi::c_int
            || chroma == CHROMA_422 as ::core::ffi::c_int
        {
            i_qp = (*h).mb.i_chroma_qp;
            let mut ch_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while ch_0 < 2 as ::core::ffi::c_int {
                let mut dct4x4_0: [[dctcoef; 16]; 2] = [[0; 16]; 2];
                let mut p_fenc_3: *mut pixel = (*h)
                    .mb
                    .pic
                    .p_fenc[(1 as ::core::ffi::c_int + ch_0) as usize]
                    .offset((4 as ::core::ffi::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as ::core::ffi::c_int
                        } else {
                            4 as ::core::ffi::c_int
                        }) * y * FENC_STRIDE) as isize,
                    );
                let mut p_fdec_3: *mut pixel = (*h)
                    .mb
                    .pic
                    .p_fdec[(1 as ::core::ffi::c_int + ch_0) as usize]
                    .offset((4 as ::core::ffi::c_int * x) as isize)
                    .offset(
                        ((if chroma422 != 0 {
                            8 as ::core::ffi::c_int
                        } else {
                            4 as ::core::ffi::c_int
                        }) * y * FDEC_STRIDE) as isize,
                    );
                let mut i4x4_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i4x4_2 <= chroma422 {
                    (*h)
                        .dctf
                        .sub4x4_dct
                        .expect(
                            "non-null function pointer",
                        )(
                        (*dct4x4_0.as_mut_ptr().offset(i4x4_2 as isize)).as_mut_ptr(),
                        p_fenc_3
                            .offset(
                                (4 as ::core::ffi::c_int * i4x4_2 * FENC_STRIDE) as isize,
                            ),
                        p_fdec_3
                            .offset(
                                (4 as ::core::ffi::c_int * i4x4_2 * FDEC_STRIDE) as isize,
                            ),
                    );
                    if (*h).mb.b_noise_reduction != 0 {
                        (*h)
                            .quantf
                            .denoise_dct
                            .expect(
                                "non-null function pointer",
                            )(
                            (*dct4x4_0.as_mut_ptr().offset(i4x4_2 as isize))
                                .as_mut_ptr(),
                            (*(*h)
                                .nr_residual_sum
                                .offset(2 as ::core::ffi::c_int as isize))
                                .as_mut_ptr(),
                            (*(*h).nr_offset.offset(2 as ::core::ffi::c_int as isize))
                                .as_mut_ptr(),
                            16 as ::core::ffi::c_int,
                        );
                    }
                    dct4x4_0[i4x4_2 as usize][0 as ::core::ffi::c_int as usize] = 0
                        as ::core::ffi::c_int as dctcoef;
                    if (*h).mb.b_trellis != 0 {
                        nz = x264_10_quant_4x4_trellis(
                            h,
                            (*dct4x4_0.as_mut_ptr().offset(i4x4_2 as isize))
                                .as_mut_ptr(),
                            CQM_4PC as ::core::ffi::c_int,
                            i_qp,
                            DCT_CHROMA_AC as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        );
                    } else {
                        nz = (*h)
                            .quantf
                            .quant_4x4
                            .expect(
                                "non-null function pointer",
                            )(
                            (*dct4x4_0.as_mut_ptr().offset(i4x4_2 as isize))
                                .as_mut_ptr(),
                            (*(*(*h)
                                .quant4_mf
                                .as_mut_ptr()
                                .offset(CQM_4PC as ::core::ffi::c_int as isize))
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                            (*(*(*h)
                                .quant4_bias
                                .as_mut_ptr()
                                .offset(CQM_4PC as ::core::ffi::c_int as isize))
                                .offset(i_qp as isize))
                                .as_mut_ptr(),
                        );
                    }
                    let mut offset_0: ::core::ffi::c_int = if chroma422 != 0 {
                        (5 as ::core::ffi::c_int * i8 & 0x9 as ::core::ffi::c_int)
                            + 2 as ::core::ffi::c_int * i4x4_2
                    } else {
                        i8
                    };
                    (*h)
                        .mb
                        .cache
                        .non_zero_count[x264_scan8[(16 as ::core::ffi::c_int + offset_0
                        + ch_0 * 16 as ::core::ffi::c_int) as usize] as usize] = nz
                        as uint8_t;
                    if nz != 0 {
                        (*h)
                            .zigzagf
                            .scan_4x4
                            .expect(
                                "non-null function pointer",
                            )(
                            (*(*h)
                                .dct
                                .luma4x4
                                .as_mut_ptr()
                                .offset(
                                    (16 as ::core::ffi::c_int + offset_0
                                        + ch_0 * 16 as ::core::ffi::c_int) as isize,
                                ))
                                .as_mut_ptr(),
                            (*dct4x4_0.as_mut_ptr().offset(i4x4_2 as isize)).as_mut_ptr(),
                        );
                        (*h)
                            .quantf
                            .dequant_4x4
                            .expect(
                                "non-null function pointer",
                            )(
                            (*dct4x4_0.as_mut_ptr().offset(i4x4_2 as isize))
                                .as_mut_ptr(),
                            (*h).dequant4_mf[CQM_4PC as ::core::ffi::c_int as usize],
                            i_qp,
                        );
                        (*h)
                            .dctf
                            .add4x4_idct
                            .expect(
                                "non-null function pointer",
                            )(
                            p_fdec_3
                                .offset(
                                    (4 as ::core::ffi::c_int * i4x4_2 * FDEC_STRIDE) as isize,
                                ),
                            (*dct4x4_0.as_mut_ptr().offset(i4x4_2 as isize)).as_mut_ptr(),
                        );
                    }
                    i4x4_2 += 1;
                }
                ch_0 += 1;
            }
            (*h).mb.i_cbp_chroma = 0x2 as ::core::ffi::c_int;
        }
    };
}
#[no_mangle]
#[c2rust::src_loc = "1370:1"]
pub unsafe extern "C" fn x264_10_macroblock_encode_p8x8(
    mut h: *mut x264_t,
    mut i8: ::core::ffi::c_int,
) {
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_420 as ::core::ffi::c_int {
        macroblock_encode_p8x8_internal(
            h,
            i8,
            1 as ::core::ffi::c_int,
            CHROMA_420 as ::core::ffi::c_int,
        );
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
        == CHROMA_422 as ::core::ffi::c_int
    {
        macroblock_encode_p8x8_internal(
            h,
            i8,
            1 as ::core::ffi::c_int,
            CHROMA_422 as ::core::ffi::c_int,
        );
    } else if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc
        == CHROMA_444 as ::core::ffi::c_int
    {
        macroblock_encode_p8x8_internal(
            h,
            i8,
            3 as ::core::ffi::c_int,
            CHROMA_444 as ::core::ffi::c_int,
        );
    } else {
        macroblock_encode_p8x8_internal(
            h,
            i8,
            1 as ::core::ffi::c_int,
            CHROMA_400 as ::core::ffi::c_int,
        );
    };
}
#[inline(always)]
#[c2rust::src_loc = "1385:1"]
unsafe extern "C" fn macroblock_encode_p4x4_internal(
    mut h: *mut x264_t,
    mut i4: ::core::ffi::c_int,
    mut plane_count: ::core::ffi::c_int,
) {
    let mut i_qp: ::core::ffi::c_int = (*h).mb.i_qp;
    let mut p: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while p < plane_count {
        let mut quant_cat: ::core::ffi::c_int = if p != 0 {
            CQM_4PC as ::core::ffi::c_int
        } else {
            CQM_4PY as ::core::ffi::c_int
        };
        let mut p_fenc: *mut pixel = &mut *(*(*h)
            .mb
            .pic
            .p_fenc
            .as_mut_ptr()
            .offset(p as isize))
            .offset(*block_idx_xy_fenc.as_ptr().offset(i4 as isize) as isize)
            as *mut pixel;
        let mut p_fdec: *mut pixel = &mut *(*(*h)
            .mb
            .pic
            .p_fdec
            .as_mut_ptr()
            .offset(p as isize))
            .offset(*block_idx_xy_fdec.as_ptr().offset(i4 as isize) as isize)
            as *mut pixel;
        let mut nz: ::core::ffi::c_int = 0;
        if (*h).mb.b_lossless != 0 {
            nz = (*h)
                .zigzagf
                .sub_4x4
                .expect(
                    "non-null function pointer",
                )(
                (*(*h)
                    .dct
                    .luma4x4
                    .as_mut_ptr()
                    .offset((p * 16 as ::core::ffi::c_int + i4) as isize))
                    .as_mut_ptr(),
                p_fenc,
                p_fdec,
            );
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(p * 16 as ::core::ffi::c_int + i4) as usize]
                as usize] = nz as uint8_t;
        } else {
            let mut dct4x4: [dctcoef; 16] = [0; 16];
            (*h)
                .dctf
                .sub4x4_dct
                .expect(
                    "non-null function pointer",
                )(dct4x4.as_mut_ptr(), p_fenc, p_fdec);
            nz = x264_quant_4x4(
                h,
                dct4x4.as_mut_ptr(),
                i_qp,
                ctx_cat_plane[DCT_LUMA_4x4 as ::core::ffi::c_int as usize][p as usize]
                    as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                p,
                i4,
            );
            (*h)
                .mb
                .cache
                .non_zero_count[x264_scan8[(p * 16 as ::core::ffi::c_int + i4) as usize]
                as usize] = nz as uint8_t;
            if nz != 0 {
                (*h)
                    .zigzagf
                    .scan_4x4
                    .expect(
                        "non-null function pointer",
                    )(
                    (*(*h)
                        .dct
                        .luma4x4
                        .as_mut_ptr()
                        .offset((p * 16 as ::core::ffi::c_int + i4) as isize))
                        .as_mut_ptr(),
                    dct4x4.as_mut_ptr(),
                );
                (*h)
                    .quantf
                    .dequant_4x4
                    .expect(
                        "non-null function pointer",
                    )(dct4x4.as_mut_ptr(), (*h).dequant4_mf[quant_cat as usize], i_qp);
                (*h)
                    .dctf
                    .add4x4_idct
                    .expect("non-null function pointer")(p_fdec, dct4x4.as_mut_ptr());
            }
        }
        p += 1;
        i_qp = (*h).mb.i_chroma_qp;
    }
}
#[no_mangle]
#[c2rust::src_loc = "1419:1"]
pub unsafe extern "C" fn x264_10_macroblock_encode_p4x4(
    mut h: *mut x264_t,
    mut i8: ::core::ffi::c_int,
) {
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
        macroblock_encode_p4x4_internal(h, i8, 3 as ::core::ffi::c_int);
    } else {
        macroblock_encode_p4x4_internal(h, i8, 1 as ::core::ffi::c_int);
    };
}
