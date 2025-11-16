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
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_size_t.h:26"]
pub mod __stddef_size_t_h {
    #[c2rust::src_loc = "18:1"]
    pub type size_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/types.h:26"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:26"]
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:26"]
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
#[c2rust::header_src = "/usr/include/stdint.h:26"]
pub mod stdint_h {
    #[c2rust::src_loc = "76:1"]
    pub type intptr_t = isize;
    #[c2rust::src_loc = "79:1"]
    pub type uintptr_t = usize;
}
#[c2rust::header_src = "/usr/include/bits/atomic_wide_counter.h:26"]
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
#[c2rust::header_src = "/usr/include/bits/thread-shared-types.h:26"]
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
#[c2rust::header_src = "/usr/include/bits/struct_mutex.h:26"]
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
#[c2rust::header_src = "/usr/include/bits/pthreadtypes.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/common.h:26"]
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
    #[c2rust::src_loc = "60:9"]
    pub const QP_MAX: ::core::ffi::c_int = QP_MAX_SPEC + 18 as ::core::ffi::c_int;
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
        #[c2rust::src_loc = "138:1"]
        pub fn x264_10_log(
            h: *mut x264_t,
            i_level: ::core::ffi::c_int,
            psz_fmt: *const ::core::ffi::c_char,
            ...
        );
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/frame.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/x264.h:26"]
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
    #[c2rust::src_loc = "213:9"]
    pub const X264_CQM_CUSTOM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
    #[c2rust::src_loc = "289:9"]
    pub const X264_LOG_ERROR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    use super::common_h::x264_t;
    use super::internal::__va_list_tag;
    use super::stdint_intn_h::int64_t;
    use super::stdint_uintn_h::{uint32_t, uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/mc.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/bitstream.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/cabac.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/quant.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/dct.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/pixel.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/predict.h:26"]
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
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/set.h:26"]
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
    use super::stdint_uintn_h::{uint32_t, uint8_t};
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/threadpool.h:26"]
pub mod threadpool_h {
    extern "C" {
        #[c2rust::src_loc = "29:16"]
        pub type x264_threadpool_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/base.h:26"]
pub mod base_h {
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
    use super::stdint_intn_h::int64_t;
    extern "C" {
        #[c2rust::src_loc = "279:10"]
        pub fn x264_malloc(_: int64_t) -> *mut ::core::ffi::c_void;
        #[c2rust::src_loc = "280:10"]
        pub fn x264_free(_: *mut ::core::ffi::c_void);
        #[c2rust::src_loc = "283:10"]
        pub fn x264_slurp_file(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    }
}
#[c2rust::header_src = "/usr/include/string.h:26"]
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
        #[c2rust::src_loc = "246:1"]
        pub fn strchr(
            __s: *const ::core::ffi::c_char,
            __c: ::core::ffi::c_int,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "293:1"]
        pub fn strcspn(
            __s: *const ::core::ffi::c_char,
            __reject: *const ::core::ffi::c_char,
        ) -> ::core::ffi::c_ulong;
        #[c2rust::src_loc = "323:1"]
        pub fn strpbrk(
            __s: *const ::core::ffi::c_char,
            __accept: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "350:1"]
        pub fn strstr(
            __haystack: *const ::core::ffi::c_char,
            __needle: *const ::core::ffi::c_char,
        ) -> *mut ::core::ffi::c_char;
        #[c2rust::src_loc = "407:1"]
        pub fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    }
}
#[c2rust::header_src = "/home/nwplayer123/Hacks/hex264/x264/common/tables.h:26"]
pub mod tables_h {
    use super::stdint_uintn_h::uint8_t;
    extern "C" {
        #[c2rust::src_loc = "52:22"]
        pub static x264_cqm_jvt4i: [uint8_t; 16];
        #[c2rust::src_loc = "53:22"]
        pub static x264_cqm_jvt4p: [uint8_t; 16];
        #[c2rust::src_loc = "54:22"]
        pub static x264_cqm_jvt8i: [uint8_t; 64];
        #[c2rust::src_loc = "55:22"]
        pub static x264_cqm_jvt8p: [uint8_t; 64];
    }
}
#[c2rust::header_src = "/usr/include/stdio.h:26"]
pub mod stdio_h {
    extern "C" {
        #[c2rust::src_loc = "450:1"]
        pub fn sscanf(
            __s: *const ::core::ffi::c_char,
            __format: *const ::core::ffi::c_char,
            ...
        ) -> ::core::ffi::c_int;
    }
}
#[c2rust::header_src = "/usr/include/bits/mathcalls.h:26"]
pub mod mathcalls_h {
    extern "C" {
        #[c2rust::src_loc = "177:1"]
        pub fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double)
            -> ::core::ffi::c_double;
    }
}
#[c2rust::header_src = "/usr/lib/clang/21/include/__stddef_null.h:26"]
pub mod __stddef_null_h {
    #[c2rust::src_loc = "26:9"]
    pub const NULL: *mut ::core::ffi::c_void = 0 as *mut ::core::ffi::c_void;
}
pub use self::__stddef_null_h::NULL;
pub use self::__stddef_size_t_h::size_t;
pub use self::atomic_wide_counter_h::{C2RustUnnamed, __atomic_wide_counter};
pub use self::base_h::{
    chroma_format_e, profile_e, x264_free, x264_malloc, x264_slurp_file, CHROMA_400, CHROMA_420,
    CHROMA_422, CHROMA_444, PROFILE_BASELINE, PROFILE_HIGH, PROFILE_HIGH10, PROFILE_HIGH422,
    PROFILE_HIGH444_PREDICTIVE, PROFILE_MAIN,
};
pub use self::bitstream_h::{bs_s, bs_t, x264_bitstream_function_t, x264_run_level_t};
pub use self::cabac_h::x264_cabac_t;
pub use self::common_h::{
    dctcoef, pixel, udctcoef, x264_10_log, x264_frame_stat_t, x264_left_table_t, x264_lookahead_t,
    x264_ratecontrol_t, x264_slice_header_t, x264_t, C2RustUnnamed_10, C2RustUnnamed_11,
    C2RustUnnamed_12, C2RustUnnamed_13, C2RustUnnamed_17, C2RustUnnamed_18, C2RustUnnamed_6,
    C2RustUnnamed_7, C2RustUnnamed_8, C2RustUnnamed_9, QP_BD_OFFSET, QP_MAX, QP_MAX_SPEC,
};
pub use self::dct_h::{x264_dct_function_t, x264_zigzag_function_t};
pub use self::frame_h::{
    x264_deblock_function_t, x264_deblock_inter_t, x264_deblock_intra_t, x264_frame, x264_frame_t,
    x264_sync_frame_list_t,
};
pub use self::internal::{__va_list_tag, BIT_DEPTH};
use self::mathcalls_h::pow;
pub use self::mc_h::{weight_fn_t, x264_mc_functions_t, x264_weight_t};
pub use self::pixel_h::{
    x264_pixel_cmp_t, x264_pixel_cmp_x3_t, x264_pixel_cmp_x4_t, x264_pixel_function_t,
};
pub use self::predict_h::{x264_predict8x8_t, x264_predict_8x8_filter_t, x264_predict_t};
pub use self::pthreadtypes_h::{pthread_cond_t, pthread_mutex_t, pthread_t};
pub use self::quant_h::x264_quant_function_t;
pub use self::set_h::{
    cqm4_e, cqm8_e, x264_pps_t, x264_sps_t, C2RustUnnamed_14, C2RustUnnamed_15, C2RustUnnamed_16,
    CQM_4IC, CQM_4IY, CQM_4PC, CQM_4PY, CQM_8IC, CQM_8IY, CQM_8PC, CQM_8PY,
};
pub use self::stdint_h::{intptr_t, uintptr_t};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t, int8_t};
pub use self::stdint_uintn_h::{uint16_t, uint32_t, uint64_t, uint8_t};
use self::stdio_h::sscanf;
use self::string_h::{memcmp, memcpy, memset, strchr, strcspn, strlen, strpbrk, strstr};
pub use self::struct_mutex_h::__pthread_mutex_s;
use self::tables_h::{x264_cqm_jvt4i, x264_cqm_jvt4p, x264_cqm_jvt8i, x264_cqm_jvt8p};
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
    C2RustUnnamed_5, X264_CQM_CUSTOM, X264_LOG_ERROR,
};
#[c2rust::src_loc = "31:22"]
static mut dequant4_scale: [[uint8_t; 3]; 6] = [
    [
        10 as ::core::ffi::c_int as uint8_t,
        13 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
    ],
    [
        11 as ::core::ffi::c_int as uint8_t,
        14 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
    ],
    [
        13 as ::core::ffi::c_int as uint8_t,
        16 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
    ],
    [
        14 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
    ],
    [
        16 as ::core::ffi::c_int as uint8_t,
        20 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
    ],
    [
        18 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        29 as ::core::ffi::c_int as uint8_t,
    ],
];
#[c2rust::src_loc = "40:23"]
static mut quant4_scale: [[uint16_t; 3]; 6] = [
    [
        13107 as ::core::ffi::c_int as uint16_t,
        8066 as ::core::ffi::c_int as uint16_t,
        5243 as ::core::ffi::c_int as uint16_t,
    ],
    [
        11916 as ::core::ffi::c_int as uint16_t,
        7490 as ::core::ffi::c_int as uint16_t,
        4660 as ::core::ffi::c_int as uint16_t,
    ],
    [
        10082 as ::core::ffi::c_int as uint16_t,
        6554 as ::core::ffi::c_int as uint16_t,
        4194 as ::core::ffi::c_int as uint16_t,
    ],
    [
        9362 as ::core::ffi::c_int as uint16_t,
        5825 as ::core::ffi::c_int as uint16_t,
        3647 as ::core::ffi::c_int as uint16_t,
    ],
    [
        8192 as ::core::ffi::c_int as uint16_t,
        5243 as ::core::ffi::c_int as uint16_t,
        3355 as ::core::ffi::c_int as uint16_t,
    ],
    [
        7282 as ::core::ffi::c_int as uint16_t,
        4559 as ::core::ffi::c_int as uint16_t,
        2893 as ::core::ffi::c_int as uint16_t,
    ],
];
#[c2rust::src_loc = "50:22"]
static mut quant8_scan: [uint8_t; 16] = [
    0 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
];
#[c2rust::src_loc = "54:22"]
static mut dequant8_scale: [[uint8_t; 6]; 6] = [
    [
        20 as ::core::ffi::c_int as uint8_t,
        18 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
    ],
    [
        22 as ::core::ffi::c_int as uint8_t,
        19 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        21 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
    ],
    [
        26 as ::core::ffi::c_int as uint8_t,
        23 as ::core::ffi::c_int as uint8_t,
        42 as ::core::ffi::c_int as uint8_t,
        24 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
        31 as ::core::ffi::c_int as uint8_t,
    ],
    [
        28 as ::core::ffi::c_int as uint8_t,
        25 as ::core::ffi::c_int as uint8_t,
        45 as ::core::ffi::c_int as uint8_t,
        26 as ::core::ffi::c_int as uint8_t,
        35 as ::core::ffi::c_int as uint8_t,
        33 as ::core::ffi::c_int as uint8_t,
    ],
    [
        32 as ::core::ffi::c_int as uint8_t,
        28 as ::core::ffi::c_int as uint8_t,
        51 as ::core::ffi::c_int as uint8_t,
        30 as ::core::ffi::c_int as uint8_t,
        40 as ::core::ffi::c_int as uint8_t,
        38 as ::core::ffi::c_int as uint8_t,
    ],
    [
        36 as ::core::ffi::c_int as uint8_t,
        32 as ::core::ffi::c_int as uint8_t,
        58 as ::core::ffi::c_int as uint8_t,
        34 as ::core::ffi::c_int as uint8_t,
        46 as ::core::ffi::c_int as uint8_t,
        43 as ::core::ffi::c_int as uint8_t,
    ],
];
#[c2rust::src_loc = "63:23"]
static mut quant8_scale: [[uint16_t; 6]; 6] = [
    [
        13107 as ::core::ffi::c_int as uint16_t,
        11428 as ::core::ffi::c_int as uint16_t,
        20972 as ::core::ffi::c_int as uint16_t,
        12222 as ::core::ffi::c_int as uint16_t,
        16777 as ::core::ffi::c_int as uint16_t,
        15481 as ::core::ffi::c_int as uint16_t,
    ],
    [
        11916 as ::core::ffi::c_int as uint16_t,
        10826 as ::core::ffi::c_int as uint16_t,
        19174 as ::core::ffi::c_int as uint16_t,
        11058 as ::core::ffi::c_int as uint16_t,
        14980 as ::core::ffi::c_int as uint16_t,
        14290 as ::core::ffi::c_int as uint16_t,
    ],
    [
        10082 as ::core::ffi::c_int as uint16_t,
        8943 as ::core::ffi::c_int as uint16_t,
        15978 as ::core::ffi::c_int as uint16_t,
        9675 as ::core::ffi::c_int as uint16_t,
        12710 as ::core::ffi::c_int as uint16_t,
        11985 as ::core::ffi::c_int as uint16_t,
    ],
    [
        9362 as ::core::ffi::c_int as uint16_t,
        8228 as ::core::ffi::c_int as uint16_t,
        14913 as ::core::ffi::c_int as uint16_t,
        8931 as ::core::ffi::c_int as uint16_t,
        11984 as ::core::ffi::c_int as uint16_t,
        11259 as ::core::ffi::c_int as uint16_t,
    ],
    [
        8192 as ::core::ffi::c_int as uint16_t,
        7346 as ::core::ffi::c_int as uint16_t,
        13159 as ::core::ffi::c_int as uint16_t,
        7740 as ::core::ffi::c_int as uint16_t,
        10486 as ::core::ffi::c_int as uint16_t,
        9777 as ::core::ffi::c_int as uint16_t,
    ],
    [
        7282 as ::core::ffi::c_int as uint16_t,
        6428 as ::core::ffi::c_int as uint16_t,
        11570 as ::core::ffi::c_int as uint16_t,
        6830 as ::core::ffi::c_int as uint16_t,
        9118 as ::core::ffi::c_int as uint16_t,
        8640 as ::core::ffi::c_int as uint16_t,
    ],
];
#[no_mangle]
#[c2rust::src_loc = "73:1"]
pub unsafe extern "C" fn x264_10_cqm_init(mut h: *mut x264_t) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut def_quant4: [[::core::ffi::c_int; 16]; 6] = [[0; 16]; 6];
    let mut def_quant8: [[::core::ffi::c_int; 64]; 6] = [[0; 64]; 6];
    let mut def_dequant4: [[::core::ffi::c_int; 16]; 6] = [[0; 16]; 6];
    let mut def_dequant8: [[::core::ffi::c_int; 64]; 6] = [[0; 64]; 6];
    let mut quant4_mf: [[[::core::ffi::c_int; 16]; 6]; 4] = [[[0; 16]; 6]; 4];
    let mut quant8_mf: [[[::core::ffi::c_int; 64]; 6]; 4] = [[[0; 64]; 6]; 4];
    let mut deadzone: [::core::ffi::c_int; 4] = [
        32 as ::core::ffi::c_int
            - (*h).param.analyse.i_luma_deadzone[1 as ::core::ffi::c_int as usize],
        32 as ::core::ffi::c_int
            - (*h).param.analyse.i_luma_deadzone[0 as ::core::ffi::c_int as usize],
        32 as ::core::ffi::c_int - 11 as ::core::ffi::c_int,
        32 as ::core::ffi::c_int - 21 as ::core::ffi::c_int,
    ];
    let mut max_qp_err: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut max_chroma_qp_err: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut min_qp_err: ::core::ffi::c_int = QP_MAX + 1 as ::core::ffi::c_int;
    let mut num_8x8_lists: ::core::ffi::c_int =
        if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
            4 as ::core::ffi::c_int
        } else if (*h).param.analyse.b_transform_8x8 != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        if !(i < 4 as ::core::ffi::c_int) {
            current_block = 5529461102203738653;
            break;
        }
        let mut size: ::core::ffi::c_int = 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
        let mut start: ::core::ffi::c_int = if 4 as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
            4 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        while j < i {
            if memcmp(
                (*(*h).sps.as_mut_ptr()).scaling_list[(i + start) as usize]
                    as *const ::core::ffi::c_void,
                (*(*h).sps.as_mut_ptr()).scaling_list[(j + start) as usize]
                    as *const ::core::ffi::c_void,
                (size as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) == 0
            {
                break;
            }
            j += 1;
        }
        if j < i {
            (*h).quant4_mf[i as usize] = (*h).quant4_mf[j as usize];
            (*h).dequant4_mf[i as usize] = (*h).dequant4_mf[j as usize];
            (*h).unquant4_mf[i as usize] = (*h).unquant4_mf[j as usize];
        } else {
            (*h).quant4_mf[i as usize] = x264_malloc(
                (((51 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int
                        * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int)
                    * size) as usize)
                    .wrapping_mul(::core::mem::size_of::<udctcoef>() as usize)
                    as int64_t,
            ) as *mut [udctcoef; 16];
            if (*h).quant4_mf[i as usize].is_null() {
                current_block = 4491644631990352080;
                break;
            }
            (*h).dequant4_mf[i as usize] = x264_malloc(
                ((6 as ::core::ffi::c_int * size) as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    as int64_t,
            ) as *mut [::core::ffi::c_int; 16];
            if (*h).dequant4_mf[i as usize].is_null() {
                current_block = 4491644631990352080;
                break;
            }
            (*h).unquant4_mf[i as usize] = x264_malloc(
                (((51 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int
                        * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int)
                    * size) as usize)
                    .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                    as int64_t,
            ) as *mut [::core::ffi::c_int; 16];
            if (*h).unquant4_mf[i as usize].is_null() {
                current_block = 4491644631990352080;
                break;
            }
        }
        j = 0 as ::core::ffi::c_int;
        while j < i {
            if deadzone[j as usize] == deadzone[i as usize]
                && memcmp(
                    (*(*h).sps.as_mut_ptr()).scaling_list[(i + start) as usize]
                        as *const ::core::ffi::c_void,
                    (*(*h).sps.as_mut_ptr()).scaling_list[(j + start) as usize]
                        as *const ::core::ffi::c_void,
                    (size as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                ) == 0
            {
                break;
            }
            j += 1;
        }
        if j < i {
            (*h).quant4_bias[i as usize] = (*h).quant4_bias[j as usize];
            (*h).quant4_bias0[i as usize] = (*h).quant4_bias0[j as usize];
        } else {
            (*h).quant4_bias[i as usize] = x264_malloc(
                (((51 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int
                        * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int)
                    * size) as usize)
                    .wrapping_mul(::core::mem::size_of::<udctcoef>() as usize)
                    as int64_t,
            ) as *mut [udctcoef; 16];
            if (*h).quant4_bias[i as usize].is_null() {
                current_block = 4491644631990352080;
                break;
            }
            (*h).quant4_bias0[i as usize] = x264_malloc(
                (((51 as ::core::ffi::c_int
                    + 6 as ::core::ffi::c_int
                        * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                    + 1 as ::core::ffi::c_int)
                    * size) as usize)
                    .wrapping_mul(::core::mem::size_of::<udctcoef>() as usize)
                    as int64_t,
            ) as *mut [udctcoef; 16];
            if (*h).quant4_bias0[i as usize].is_null() {
                current_block = 4491644631990352080;
                break;
            }
        }
        i += 1;
    }
    match current_block {
        5529461102203738653 => {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            loop {
                if !(i_0 < num_8x8_lists) {
                    current_block = 7419121793134201633;
                    break;
                }
                let mut size_0: ::core::ffi::c_int =
                    8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
                let mut start_0: ::core::ffi::c_int =
                    if 8 as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                        4 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                let mut j_0: ::core::ffi::c_int = 0;
                j_0 = 0 as ::core::ffi::c_int;
                while j_0 < i_0 {
                    if memcmp(
                        (*(*h).sps.as_mut_ptr()).scaling_list[(i_0 + start_0) as usize]
                            as *const ::core::ffi::c_void,
                        (*(*h).sps.as_mut_ptr()).scaling_list[(j_0 + start_0) as usize]
                            as *const ::core::ffi::c_void,
                        (size_0 as size_t)
                            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                    ) == 0
                    {
                        break;
                    }
                    j_0 += 1;
                }
                if j_0 < i_0 {
                    (*h).quant8_mf[i_0 as usize] = (*h).quant8_mf[j_0 as usize];
                    (*h).dequant8_mf[i_0 as usize] = (*h).dequant8_mf[j_0 as usize];
                    (*h).unquant8_mf[i_0 as usize] = (*h).unquant8_mf[j_0 as usize];
                } else {
                    (*h).quant8_mf[i_0 as usize] = x264_malloc(
                        (((51 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int
                                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_int)
                            * size_0) as usize)
                            .wrapping_mul(::core::mem::size_of::<udctcoef>() as usize)
                            as int64_t,
                    ) as *mut [udctcoef; 64];
                    if (*h).quant8_mf[i_0 as usize].is_null() {
                        current_block = 4491644631990352080;
                        break;
                    }
                    (*h).dequant8_mf[i_0 as usize] = x264_malloc(
                        ((6 as ::core::ffi::c_int * size_0) as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                            as int64_t,
                    )
                        as *mut [::core::ffi::c_int; 64];
                    if (*h).dequant8_mf[i_0 as usize].is_null() {
                        current_block = 4491644631990352080;
                        break;
                    }
                    (*h).unquant8_mf[i_0 as usize] = x264_malloc(
                        (((51 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int
                                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_int)
                            * size_0) as usize)
                            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                            as int64_t,
                    )
                        as *mut [::core::ffi::c_int; 64];
                    if (*h).unquant8_mf[i_0 as usize].is_null() {
                        current_block = 4491644631990352080;
                        break;
                    }
                }
                j_0 = 0 as ::core::ffi::c_int;
                while j_0 < i_0 {
                    if deadzone[j_0 as usize] == deadzone[i_0 as usize]
                        && memcmp(
                            (*(*h).sps.as_mut_ptr()).scaling_list[(i_0 + start_0) as usize]
                                as *const ::core::ffi::c_void,
                            (*(*h).sps.as_mut_ptr()).scaling_list[(j_0 + start_0) as usize]
                                as *const ::core::ffi::c_void,
                            (size_0 as size_t)
                                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
                        ) == 0
                    {
                        break;
                    }
                    j_0 += 1;
                }
                if j_0 < i_0 {
                    (*h).quant8_bias[i_0 as usize] = (*h).quant8_bias[j_0 as usize];
                    (*h).quant8_bias0[i_0 as usize] = (*h).quant8_bias0[j_0 as usize];
                } else {
                    (*h).quant8_bias[i_0 as usize] = x264_malloc(
                        (((51 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int
                                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_int)
                            * size_0) as usize)
                            .wrapping_mul(::core::mem::size_of::<udctcoef>() as usize)
                            as int64_t,
                    ) as *mut [udctcoef; 64];
                    if (*h).quant8_bias[i_0 as usize].is_null() {
                        current_block = 4491644631990352080;
                        break;
                    }
                    (*h).quant8_bias0[i_0 as usize] = x264_malloc(
                        (((51 as ::core::ffi::c_int
                            + 6 as ::core::ffi::c_int
                                * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                            + 1 as ::core::ffi::c_int)
                            * size_0) as usize)
                            .wrapping_mul(::core::mem::size_of::<udctcoef>() as usize)
                            as int64_t,
                    ) as *mut [udctcoef; 64];
                    if (*h).quant8_bias0[i_0 as usize].is_null() {
                        current_block = 4491644631990352080;
                        break;
                    }
                }
                i_0 += 1;
            }
            match current_block {
                4491644631990352080 => {}
                _ => {
                    let mut q: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while q < 6 as ::core::ffi::c_int {
                        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_1 < 16 as ::core::ffi::c_int {
                            let mut j_1: ::core::ffi::c_int = (i_1 & 1 as ::core::ffi::c_int)
                                + (i_1 >> 2 as ::core::ffi::c_int & 1 as ::core::ffi::c_int);
                            def_dequant4[q as usize][i_1 as usize] =
                                dequant4_scale[q as usize][j_1 as usize] as ::core::ffi::c_int;
                            def_quant4[q as usize][i_1 as usize] =
                                quant4_scale[q as usize][j_1 as usize] as ::core::ffi::c_int;
                            i_1 += 1;
                        }
                        let mut i_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_2 < 64 as ::core::ffi::c_int {
                            let mut j_2: ::core::ffi::c_int =
                                quant8_scan[(i_2 >> 1 as ::core::ffi::c_int
                                    & 12 as ::core::ffi::c_int
                                    | i_2 & 3 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int;
                            def_dequant8[q as usize][i_2 as usize] =
                                dequant8_scale[q as usize][j_2 as usize] as ::core::ffi::c_int;
                            def_quant8[q as usize][i_2 as usize] =
                                quant8_scale[q as usize][j_2 as usize] as ::core::ffi::c_int;
                            i_2 += 1;
                        }
                        q += 1;
                    }
                    let mut q_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while q_0 < 6 as ::core::ffi::c_int {
                        let mut i_list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_list < 4 as ::core::ffi::c_int {
                            let mut i_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_3 < 16 as ::core::ffi::c_int {
                                (*(*h).dequant4_mf[i_list as usize].offset(q_0 as isize))
                                    [i_3 as usize] = def_dequant4[q_0 as usize][i_3 as usize]
                                    * *(*(*h).sps.as_mut_ptr()).scaling_list[i_list as usize]
                                        .offset(i_3 as isize)
                                        as ::core::ffi::c_int;
                                quant4_mf[i_list as usize][q_0 as usize][i_3 as usize] = (def_quant4
                                    [q_0 as usize][i_3 as usize]
                                    * 16 as ::core::ffi::c_int
                                    + (*(*(*h).sps.as_mut_ptr()).scaling_list[i_list as usize]
                                        .offset(i_3 as isize)
                                        as ::core::ffi::c_int
                                        >> 1 as ::core::ffi::c_int))
                                    / *(*(*h).sps.as_mut_ptr()).scaling_list[i_list as usize]
                                        .offset(i_3 as isize)
                                        as ::core::ffi::c_int;
                                i_3 += 1;
                            }
                            i_list += 1;
                        }
                        let mut i_list_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_list_0 < num_8x8_lists {
                            let mut i_4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_4 < 64 as ::core::ffi::c_int {
                                (*(*h).dequant8_mf[i_list_0 as usize].offset(q_0 as isize))
                                    [i_4 as usize] = def_dequant8[q_0 as usize][i_4 as usize]
                                    * *(*(*h).sps.as_mut_ptr()).scaling_list
                                        [(4 as ::core::ffi::c_int + i_list_0) as usize]
                                        .offset(i_4 as isize)
                                        as ::core::ffi::c_int;
                                quant8_mf[i_list_0 as usize][q_0 as usize][i_4 as usize] =
                                    (def_quant8[q_0 as usize][i_4 as usize]
                                        * 16 as ::core::ffi::c_int
                                        + (*(*(*h).sps.as_mut_ptr()).scaling_list
                                            [(4 as ::core::ffi::c_int + i_list_0) as usize]
                                            .offset(i_4 as isize)
                                            as ::core::ffi::c_int
                                            >> 1 as ::core::ffi::c_int))
                                        / *(*(*h).sps.as_mut_ptr()).scaling_list
                                            [(4 as ::core::ffi::c_int + i_list_0) as usize]
                                            .offset(i_4 as isize)
                                            as ::core::ffi::c_int;
                                i_4 += 1;
                            }
                            i_list_0 += 1;
                        }
                        q_0 += 1;
                    }
                    let mut q_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while q_1 <= QP_MAX_SPEC {
                        let mut j_3: ::core::ffi::c_int = 0;
                        let mut i_list_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i_list_1 < 4 as ::core::ffi::c_int {
                            let mut i_5: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_5 < 16 as ::core::ffi::c_int {
                                (*(*h).unquant4_mf[i_list_1 as usize].offset(q_1 as isize))
                                    [i_5 as usize] = ((1 as ::core::ffi::c_ulonglong)
                                    << q_1 / 6 as ::core::ffi::c_int
                                        + 15 as ::core::ffi::c_int
                                        + 8 as ::core::ffi::c_int)
                                    .wrapping_div(
                                        quant4_mf[i_list_1 as usize]
                                            [(q_1 % 6 as ::core::ffi::c_int) as usize]
                                            [i_5 as usize]
                                            as ::core::ffi::c_ulonglong,
                                    )
                                    as ::core::ffi::c_int;
                                j_3 = if q_1 / 6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    <= 0 as ::core::ffi::c_int
                                {
                                    quant4_mf[i_list_1 as usize]
                                        [(q_1 % 6 as ::core::ffi::c_int) as usize]
                                        [i_5 as usize]
                                        << -(q_1 / 6 as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int)
                                } else {
                                    quant4_mf[i_list_1 as usize]
                                        [(q_1 % 6 as ::core::ffi::c_int) as usize]
                                        [i_5 as usize]
                                        + ((1 as ::core::ffi::c_int)
                                            << q_1 / 6 as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int)
                                        >> q_1 / 6 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                };
                                (*(*h).quant4_mf[i_list_1 as usize].offset(q_1 as isize))
                                    [i_5 as usize] = j_3 as uint16_t as udctcoef;
                                if j_3 == 0 {
                                    min_qp_err = if min_qp_err < q_1 { min_qp_err } else { q_1 };
                                } else {
                                    (*(*h).quant4_bias[i_list_1 as usize].offset(q_1 as isize))
                                        [i_5 as usize] = (if ((deadzone[i_list_1 as usize]
                                        << 10 as ::core::ffi::c_int)
                                        + (j_3 >> 1 as ::core::ffi::c_int))
                                        / j_3
                                        < ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                            / j_3
                                    {
                                        ((deadzone[i_list_1 as usize] << 10 as ::core::ffi::c_int)
                                            + (j_3 >> 1 as ::core::ffi::c_int))
                                            / j_3
                                    } else {
                                        ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                            / j_3
                                    })
                                        as udctcoef;
                                    (*(*h).quant4_bias0[i_list_1 as usize].offset(q_1 as isize))
                                        [i_5 as usize] = (((1 as ::core::ffi::c_int)
                                        << 15 as ::core::ffi::c_int)
                                        / j_3)
                                        as udctcoef;
                                    if j_3
                                        > (if (0xffff as ::core::ffi::c_int)
                                            < ((1 as ::core::ffi::c_int)
                                                << 25 as ::core::ffi::c_int
                                                    - 10 as ::core::ffi::c_int)
                                                - 1 as ::core::ffi::c_int
                                        {
                                            0xffff as ::core::ffi::c_int
                                        } else {
                                            ((1 as ::core::ffi::c_int)
                                                << 25 as ::core::ffi::c_int
                                                    - 10 as ::core::ffi::c_int)
                                                - 1 as ::core::ffi::c_int
                                        })
                                        && q_1 > max_qp_err
                                        && (i_list_1 == CQM_4IY as ::core::ffi::c_int
                                            || i_list_1 == CQM_4PY as ::core::ffi::c_int)
                                    {
                                        max_qp_err = q_1;
                                    }
                                    if j_3
                                        > (if (0xffff as ::core::ffi::c_int)
                                            < ((1 as ::core::ffi::c_int)
                                                << 25 as ::core::ffi::c_int
                                                    - 10 as ::core::ffi::c_int)
                                                - 1 as ::core::ffi::c_int
                                        {
                                            0xffff as ::core::ffi::c_int
                                        } else {
                                            ((1 as ::core::ffi::c_int)
                                                << 25 as ::core::ffi::c_int
                                                    - 10 as ::core::ffi::c_int)
                                                - 1 as ::core::ffi::c_int
                                        })
                                        && q_1 > max_chroma_qp_err
                                        && (i_list_1 == CQM_4IC as ::core::ffi::c_int
                                            || i_list_1 == CQM_4PC as ::core::ffi::c_int)
                                    {
                                        max_chroma_qp_err = q_1;
                                    }
                                }
                                i_5 += 1;
                            }
                            i_list_1 += 1;
                        }
                        if (*h).param.analyse.b_transform_8x8 != 0 {
                            let mut i_list_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while i_list_2 < num_8x8_lists {
                                let mut i_6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while i_6 < 64 as ::core::ffi::c_int {
                                    (*(*h).unquant8_mf[i_list_2 as usize].offset(q_1 as isize))
                                        [i_6 as usize] = ((1 as ::core::ffi::c_ulonglong)
                                        << q_1 / 6 as ::core::ffi::c_int
                                            + 16 as ::core::ffi::c_int
                                            + 8 as ::core::ffi::c_int)
                                        .wrapping_div(
                                            quant8_mf[i_list_2 as usize]
                                                [(q_1 % 6 as ::core::ffi::c_int) as usize]
                                                [i_6 as usize]
                                                as ::core::ffi::c_ulonglong,
                                        )
                                        as ::core::ffi::c_int;
                                    j_3 = if q_1 / 6 as ::core::ffi::c_int
                                        <= 0 as ::core::ffi::c_int
                                    {
                                        quant8_mf[i_list_2 as usize]
                                            [(q_1 % 6 as ::core::ffi::c_int) as usize]
                                            [i_6 as usize]
                                            << -(q_1 / 6 as ::core::ffi::c_int)
                                    } else {
                                        quant8_mf[i_list_2 as usize]
                                            [(q_1 % 6 as ::core::ffi::c_int) as usize]
                                            [i_6 as usize]
                                            + ((1 as ::core::ffi::c_int)
                                                << q_1 / 6 as ::core::ffi::c_int
                                                    - 1 as ::core::ffi::c_int)
                                            >> q_1 / 6 as ::core::ffi::c_int
                                    };
                                    (*(*h).quant8_mf[i_list_2 as usize].offset(q_1 as isize))
                                        [i_6 as usize] = j_3 as uint16_t as udctcoef;
                                    if j_3 == 0 {
                                        min_qp_err =
                                            if min_qp_err < q_1 { min_qp_err } else { q_1 };
                                    } else {
                                        (*(*h).quant8_bias[i_list_2 as usize]
                                            .offset(q_1 as isize))
                                            [i_6 as usize] = (if ((deadzone[i_list_2 as usize]
                                            << 10 as ::core::ffi::c_int)
                                            + (j_3 >> 1 as ::core::ffi::c_int))
                                            / j_3
                                            < ((1 as ::core::ffi::c_int)
                                                << 15 as ::core::ffi::c_int)
                                                / j_3
                                        {
                                            ((deadzone[i_list_2 as usize]
                                                << 10 as ::core::ffi::c_int)
                                                + (j_3 >> 1 as ::core::ffi::c_int))
                                                / j_3
                                        } else {
                                            ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                                / j_3
                                        })
                                            as udctcoef;
                                        (*(*h).quant8_bias0[i_list_2 as usize]
                                            .offset(q_1 as isize))
                                            [i_6 as usize] = (((1 as ::core::ffi::c_int)
                                            << 15 as ::core::ffi::c_int)
                                            / j_3)
                                            as udctcoef;
                                        if j_3
                                            > (if (0xffff as ::core::ffi::c_int)
                                                < ((1 as ::core::ffi::c_int)
                                                    << 25 as ::core::ffi::c_int
                                                        - 10 as ::core::ffi::c_int)
                                                    - 1 as ::core::ffi::c_int
                                            {
                                                0xffff as ::core::ffi::c_int
                                            } else {
                                                ((1 as ::core::ffi::c_int)
                                                    << 25 as ::core::ffi::c_int
                                                        - 10 as ::core::ffi::c_int)
                                                    - 1 as ::core::ffi::c_int
                                            })
                                            && q_1 > max_qp_err
                                            && (i_list_2 == CQM_8IY as ::core::ffi::c_int
                                                || i_list_2 == CQM_8PY as ::core::ffi::c_int)
                                        {
                                            max_qp_err = q_1;
                                        }
                                        if j_3
                                            > (if (0xffff as ::core::ffi::c_int)
                                                < ((1 as ::core::ffi::c_int)
                                                    << 25 as ::core::ffi::c_int
                                                        - 10 as ::core::ffi::c_int)
                                                    - 1 as ::core::ffi::c_int
                                            {
                                                0xffff as ::core::ffi::c_int
                                            } else {
                                                ((1 as ::core::ffi::c_int)
                                                    << 25 as ::core::ffi::c_int
                                                        - 10 as ::core::ffi::c_int)
                                                    - 1 as ::core::ffi::c_int
                                            })
                                            && q_1 > max_chroma_qp_err
                                            && (i_list_2 == CQM_8IC as ::core::ffi::c_int
                                                || i_list_2 == CQM_8PC as ::core::ffi::c_int)
                                        {
                                            max_chroma_qp_err = q_1;
                                        }
                                    }
                                    i_6 += 1;
                                }
                                i_list_2 += 1;
                            }
                        }
                        q_1 += 1;
                    }
                    (*h).nr_offset_emergency = x264_malloc(
                        (::core::mem::size_of::<[[udctcoef; 64]; 4]>() as usize).wrapping_mul(
                            (51 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int
                                    * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                + 18 as ::core::ffi::c_int
                                - (51 as ::core::ffi::c_int
                                    + 6 as ::core::ffi::c_int
                                        * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)))
                                as usize,
                        ) as int64_t,
                    ) as *mut [[udctcoef; 64]; 4];
                    if !(*h).nr_offset_emergency.is_null() {
                        let mut q_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while q_2 < QP_MAX - QP_MAX_SPEC {
                            let mut cat: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while cat
                                < 3 as ::core::ffi::c_int
                                    + ((*(*h).sps.as_mut_ptr()).i_chroma_format_idc
                                        == CHROMA_444 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                            {
                                let mut dct8x8: ::core::ffi::c_int = cat & 1 as ::core::ffi::c_int;
                                if !((*h).param.analyse.b_transform_8x8 == 0 && dct8x8 != 0) {
                                    let mut size_1: ::core::ffi::c_int = if dct8x8 != 0 {
                                        64 as ::core::ffi::c_int
                                    } else {
                                        16 as ::core::ffi::c_int
                                    };
                                    let mut nr_offset: *mut udctcoef =
                                        (*(*(*h).nr_offset_emergency.offset(q_2 as isize))
                                            .as_mut_ptr()
                                            .offset(cat as isize))
                                        .as_mut_ptr();
                                    let mut dc_threshold: ::core::ffi::c_int =
                                        (QP_MAX - QP_MAX_SPEC) * 2 as ::core::ffi::c_int
                                            / 3 as ::core::ffi::c_int;
                                    let mut luma_threshold: ::core::ffi::c_int =
                                        (QP_MAX - QP_MAX_SPEC) * 2 as ::core::ffi::c_int
                                            / 3 as ::core::ffi::c_int;
                                    let mut chroma_threshold: ::core::ffi::c_int =
                                        0 as ::core::ffi::c_int;
                                    let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                    while i_7 < size_1 {
                                        let mut max: ::core::ffi::c_int = ((1
                                            as ::core::ffi::c_int)
                                            << 7 as ::core::ffi::c_int + BIT_DEPTH)
                                            - 1 as ::core::ffi::c_int;
                                        if q_2 == QP_MAX - QP_MAX_SPEC - 1 as ::core::ffi::c_int {
                                            *nr_offset.offset(i_7 as isize) = max as udctcoef;
                                        } else {
                                            let mut thresh: ::core::ffi::c_int =
                                                if i_7 == 0 as ::core::ffi::c_int {
                                                    dc_threshold
                                                } else if cat >= 2 as ::core::ffi::c_int {
                                                    chroma_threshold
                                                } else {
                                                    luma_threshold
                                                };
                                            if q_2 < thresh {
                                                *nr_offset.offset(i_7 as isize) = 0 as udctcoef;
                                            } else {
                                                let mut pos: ::core::ffi::c_double = (q_2 - thresh
                                                    + 1 as ::core::ffi::c_int)
                                                    as ::core::ffi::c_double
                                                    / (QP_MAX - QP_MAX_SPEC - thresh)
                                                        as ::core::ffi::c_double;
                                                let mut start_1: ::core::ffi::c_double = (if dct8x8
                                                    != 0
                                                {
                                                    (*(*h).unquant8_mf
                                                        [CQM_8PY as ::core::ffi::c_int as usize]
                                                        .offset(QP_MAX_SPEC as isize))
                                                        [i_7 as usize]
                                                } else {
                                                    (*(*h).unquant4_mf
                                                        [CQM_4PY as ::core::ffi::c_int as usize]
                                                        .offset(QP_MAX_SPEC as isize))
                                                        [i_7 as usize]
                                                })
                                                    as ::core::ffi::c_double;
                                                let mut bias: ::core::ffi::c_double = (pow(
                                                    2 as ::core::ffi::c_int
                                                        as ::core::ffi::c_double,
                                                    pos * (QP_MAX - QP_MAX_SPEC)
                                                        as ::core::ffi::c_double
                                                        / 10.0f64,
                                                )
                                                    * 0.003f64
                                                    - 0.003f64)
                                                    * start_1;
                                                *nr_offset.offset(i_7 as isize) = (if bias + 0.5f64
                                                    < max as ::core::ffi::c_double
                                                {
                                                    bias + 0.5f64
                                                } else {
                                                    max as ::core::ffi::c_double
                                                })
                                                    as udctcoef;
                                            }
                                        }
                                        i_7 += 1;
                                    }
                                }
                                cat += 1;
                            }
                            q_2 += 1;
                        }
                        if (*h).mb.b_lossless == 0 {
                            while *(*h).chroma_qp_table.offset(
                                (if (*h).param.rc.i_qp_min
                                    < 51 as ::core::ffi::c_int
                                        + 6 as ::core::ffi::c_int
                                            * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                {
                                    (*h).param.rc.i_qp_min
                                } else {
                                    51 as ::core::ffi::c_int
                                        + 6 as ::core::ffi::c_int
                                            * (10 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                }) as isize,
                            ) as ::core::ffi::c_int
                                <= max_chroma_qp_err
                            {
                                (*h).param.rc.i_qp_min += 1;
                            }
                            if min_qp_err <= (*h).param.rc.i_qp_max {
                                (*h).param.rc.i_qp_max = min_qp_err - 1 as ::core::ffi::c_int;
                            }
                            if max_qp_err >= (*h).param.rc.i_qp_min {
                                (*h).param.rc.i_qp_min = max_qp_err + 1 as ::core::ffi::c_int;
                            }
                            if (*h).param.b_cabac == 0
                                && (*(*h).sps.as_mut_ptr()).i_profile_idc
                                    < PROFILE_HIGH as ::core::ffi::c_int
                            {
                                while *(*h).chroma_qp_table.offset(
                                    (if (*h).param.rc.i_qp_max
                                        < 51 as ::core::ffi::c_int
                                            + 6 as ::core::ffi::c_int
                                                * (10 as ::core::ffi::c_int
                                                    - 8 as ::core::ffi::c_int)
                                    {
                                        (*h).param.rc.i_qp_max
                                    } else {
                                        51 as ::core::ffi::c_int
                                            + 6 as ::core::ffi::c_int
                                                * (10 as ::core::ffi::c_int
                                                    - 8 as ::core::ffi::c_int)
                                    }) as isize,
                                ) as ::core::ffi::c_int
                                    <= 12 as ::core::ffi::c_int
                                    || (*h).param.rc.i_qp_max <= 12 as ::core::ffi::c_int
                                {
                                    (*h).param.rc.i_qp_max += 1;
                                }
                            }
                            if (*h).param.rc.i_qp_min > (*h).param.rc.i_qp_max {
                                x264_10_log(
                                    h,
                                    X264_LOG_ERROR,
                                    b"Impossible QP constraints for CQM (min=%d, max=%d)\n\0"
                                        as *const u8
                                        as *const ::core::ffi::c_char,
                                    (*h).param.rc.i_qp_min,
                                    (*h).param.rc.i_qp_max,
                                );
                                return -(1 as ::core::ffi::c_int);
                            }
                        }
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    x264_10_cqm_delete(h);
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
#[c2rust::src_loc = "300:1"]
pub unsafe extern "C" fn x264_10_cqm_delete(mut h: *mut x264_t) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        while j < i {
            if (*h).quant4_mf[i as usize] == (*h).quant4_mf[j as usize] {
                break;
            }
            j += 1;
        }
        if j == i {
            x264_free((*h).quant4_mf[i as usize] as *mut ::core::ffi::c_void);
            x264_free((*h).dequant4_mf[i as usize] as *mut ::core::ffi::c_void);
            x264_free((*h).unquant4_mf[i as usize] as *mut ::core::ffi::c_void);
        }
        j = 0 as ::core::ffi::c_int;
        while j < i {
            if (*h).quant4_bias[i as usize] == (*h).quant4_bias[j as usize] {
                break;
            }
            j += 1;
        }
        if j == i {
            x264_free((*h).quant4_bias[i as usize] as *mut ::core::ffi::c_void);
            x264_free((*h).quant4_bias0[i as usize] as *mut ::core::ffi::c_void);
        }
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0
        < (if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
            4 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int
        })
    {
        let mut j_0: ::core::ffi::c_int = 0;
        j_0 = 0 as ::core::ffi::c_int;
        while j_0 < i_0 {
            if (*h).quant8_mf[i_0 as usize] == (*h).quant8_mf[j_0 as usize] {
                break;
            }
            j_0 += 1;
        }
        if j_0 == i_0 {
            x264_free((*h).quant8_mf[i_0 as usize] as *mut ::core::ffi::c_void);
            x264_free((*h).dequant8_mf[i_0 as usize] as *mut ::core::ffi::c_void);
            x264_free((*h).unquant8_mf[i_0 as usize] as *mut ::core::ffi::c_void);
        }
        j_0 = 0 as ::core::ffi::c_int;
        while j_0 < i_0 {
            if (*h).quant8_bias[i_0 as usize] == (*h).quant8_bias[j_0 as usize] {
                break;
            }
            j_0 += 1;
        }
        if j_0 == i_0 {
            x264_free((*h).quant8_bias[i_0 as usize] as *mut ::core::ffi::c_void);
            x264_free((*h).quant8_bias0[i_0 as usize] as *mut ::core::ffi::c_void);
        }
        i_0 += 1;
    }
    x264_free((*h).nr_offset_emergency as *mut ::core::ffi::c_void);
}
#[c2rust::src_loc = "307:1"]
unsafe extern "C" fn cqm_parse_jmlist(
    mut h: *mut x264_t,
    mut buf: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut cqm: *mut uint8_t,
    mut jvt: *const uint8_t,
    mut length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = strstr(buf, name);
    if p.is_null() {
        memset(
            cqm as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            length as size_t,
        );
        return 0 as ::core::ffi::c_int;
    }
    p = p.offset(strlen(name) as isize);
    if *p as ::core::ffi::c_int == 'U' as i32 || *p as ::core::ffi::c_int == 'V' as i32 {
        p = p.offset(1);
    }
    let mut nextvar: *mut ::core::ffi::c_char =
        strstr(p, b"INT\0" as *const u8 as *const ::core::ffi::c_char);
    i = 0 as ::core::ffi::c_int;
    while i < length
        && {
            p = strpbrk(p, b" \t\n,\0" as *const u8 as *const ::core::ffi::c_char);
            !p.is_null()
        }
        && {
            p = strpbrk(
                p,
                b"0123456789\0" as *const u8 as *const ::core::ffi::c_char,
            );
            !p.is_null()
        }
    {
        let mut coef: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        sscanf(
            p,
            b"%d\0" as *const u8 as *const ::core::ffi::c_char,
            &mut coef as *mut ::core::ffi::c_int,
        );
        if i == 0 as ::core::ffi::c_int && coef == 0 as ::core::ffi::c_int {
            memcpy(
                cqm as *mut ::core::ffi::c_void,
                jvt as *const ::core::ffi::c_void,
                length as size_t,
            );
            return 0 as ::core::ffi::c_int;
        }
        if coef < 1 as ::core::ffi::c_int || coef > 255 as ::core::ffi::c_int {
            x264_10_log(
                h,
                X264_LOG_ERROR,
                b"bad coefficient in list '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
                name,
            );
            return -(1 as ::core::ffi::c_int);
        }
        *cqm.offset(i as isize) = coef as uint8_t;
        i += 1;
    }
    if !nextvar.is_null() && p > nextvar || i != length {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"not enough coefficients in list '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            name,
        );
        return -(1 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
#[c2rust::src_loc = "351:1"]
pub unsafe extern "C" fn x264_10_cqm_parse_file(
    mut h: *mut x264_t,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = 0 as *mut ::core::ffi::c_char;
    let mut b_error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*h).param.i_cqm_preset = X264_CQM_CUSTOM;
    let mut buf: *mut ::core::ffi::c_char = x264_slurp_file(filename);
    if buf.is_null() {
        x264_10_log(
            h,
            X264_LOG_ERROR,
            b"can't open file '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            filename,
        );
        return -(1 as ::core::ffi::c_int);
    }
    loop {
        p = strchr(buf, '#' as i32);
        if p.is_null() {
            break;
        }
        memset(
            p as *mut ::core::ffi::c_void,
            ' ' as i32,
            strcspn(p, b"\n\0" as *const u8 as *const ::core::ffi::c_char) as size_t,
        );
    }
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTRA4X4_LUMA\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).param.cqm_4iy.as_mut_ptr(),
        x264_cqm_jvt4i.as_ptr(),
        16 as ::core::ffi::c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTER4X4_LUMA\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).param.cqm_4py.as_mut_ptr(),
        x264_cqm_jvt4p.as_ptr(),
        16 as ::core::ffi::c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTRA4X4_CHROMA\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).param.cqm_4ic.as_mut_ptr(),
        x264_cqm_jvt4i.as_ptr(),
        16 as ::core::ffi::c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTER4X4_CHROMA\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).param.cqm_4pc.as_mut_ptr(),
        x264_cqm_jvt4p.as_ptr(),
        16 as ::core::ffi::c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTRA8X8_LUMA\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).param.cqm_8iy.as_mut_ptr(),
        x264_cqm_jvt8i.as_ptr(),
        64 as ::core::ffi::c_int,
    );
    b_error |= cqm_parse_jmlist(
        h,
        buf,
        b"INTER8X8_LUMA\0" as *const u8 as *const ::core::ffi::c_char,
        (*h).param.cqm_8py.as_mut_ptr(),
        x264_cqm_jvt8p.as_ptr(),
        64 as ::core::ffi::c_int,
    );
    if (*(*h).sps.as_mut_ptr()).i_chroma_format_idc == CHROMA_444 as ::core::ffi::c_int {
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTRA8X8_CHROMA\0" as *const u8 as *const ::core::ffi::c_char,
            (*h).param.cqm_8ic.as_mut_ptr(),
            x264_cqm_jvt8i.as_ptr(),
            64 as ::core::ffi::c_int,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTER8X8_CHROMA\0" as *const u8 as *const ::core::ffi::c_char,
            (*h).param.cqm_8pc.as_mut_ptr(),
            x264_cqm_jvt8p.as_ptr(),
            64 as ::core::ffi::c_int,
        );
    }
    x264_free(buf as *mut ::core::ffi::c_void);
    return b_error;
}
