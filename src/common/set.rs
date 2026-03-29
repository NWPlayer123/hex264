// =============== BEGIN set_h ================
pub type cqm4_e = ::core::ffi::c_uint;

pub const CQM_4IY: crate::src::common::set::cqm4_e = 0;

pub const CQM_4PY: crate::src::common::set::cqm4_e = 1;

pub const CQM_4IC: crate::src::common::set::cqm4_e = 2;

pub const CQM_4PC: crate::src::common::set::cqm4_e = 3;

pub type cqm8_e = ::core::ffi::c_uint;

pub const CQM_8IY: crate::src::common::set::cqm8_e = 0;

pub const CQM_8PY: crate::src::common::set::cqm8_e = 1;

pub const CQM_8IC: crate::src::common::set::cqm8_e = 2;

pub const CQM_8PC: crate::src::common::set::cqm8_e = 3;
#[derive(Copy, Clone)]
#[repr(C)]

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
    pub crop: crate::src::common::set::C2Rust_Unnamed_26,
    pub b_vui: ::core::ffi::c_int,
    pub vui: crate::src::common::set::C2Rust_Unnamed_24,
    pub b_qpprime_y_zero_transform_bypass: ::core::ffi::c_int,
    pub i_chroma_format_idc: ::core::ffi::c_int,
    pub b_avcintra_hd: ::core::ffi::c_int,
    pub b_avcintra_4k: ::core::ffi::c_int,
    pub i_cqm_preset: ::core::ffi::c_int,
    pub scaling_list: [*const crate::stdlib::uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_26 {
    pub i_left: ::core::ffi::c_int,
    pub i_right: ::core::ffi::c_int,
    pub i_top: ::core::ffi::c_int,
    pub i_bottom: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]

pub struct C2Rust_Unnamed_24 {
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
    pub i_num_units_in_tick: crate::stdlib::uint32_t,
    pub i_time_scale: crate::stdlib::uint32_t,
    pub b_fixed_frame_rate: ::core::ffi::c_int,
    pub b_nal_hrd_parameters_present: ::core::ffi::c_int,
    pub b_vcl_hrd_parameters_present: ::core::ffi::c_int,
    pub hrd: crate::src::common::set::C2Rust_Unnamed_25,
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

pub struct C2Rust_Unnamed_25 {
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
pub use crate::__stddef_null_h::NULL;
pub use crate::__stddef_size_t_h::size_t;

pub use crate::src::common::base::chroma_format_e;
pub use crate::src::common::base::profile_e;
pub use crate::src::common::base::x264_free;
pub use crate::src::common::base::x264_malloc;
pub use crate::src::common::base::x264_slurp_file;
pub use crate::src::common::base::CHROMA_400;
pub use crate::src::common::base::CHROMA_420;
pub use crate::src::common::base::CHROMA_422;
pub use crate::src::common::base::CHROMA_444;
pub use crate::src::common::base::PROFILE_BASELINE;
pub use crate::src::common::base::PROFILE_HIGH;
pub use crate::src::common::base::PROFILE_HIGH10;
pub use crate::src::common::base::PROFILE_HIGH422;
pub use crate::src::common::base::PROFILE_HIGH444_PREDICTIVE;
pub use crate::src::common::base::PROFILE_MAIN;
pub use crate::src::common::bitstream::bs_s;
pub use crate::src::common::bitstream::bs_t;
pub use crate::src::common::bitstream::x264_bitstream_function_t;
pub use crate::src::common::bitstream::x264_run_level_t;
pub use crate::src::common::cabac::x264_cabac_t;
pub use crate::stdlib::C2Rust_Unnamed_7;
pub use crate::stdlib::__atomic_wide_counter;

pub use crate::internal::__va_list_tag;
pub use crate::internal::BIT_DEPTH;
pub use crate::src::common::common::dctcoef;
pub use crate::src::common::common::pixel;
pub use crate::src::common::common::udctcoef;
pub use crate::src::common::common::x264_8_log;
pub use crate::src::common::common::x264_frame_stat_t;
pub use crate::src::common::common::x264_left_table_t;
pub use crate::src::common::common::x264_lookahead_t;
pub use crate::src::common::common::x264_ratecontrol_t;
pub use crate::src::common::common::x264_slice_header_t;
pub use crate::src::common::common::x264_t;
pub use crate::src::common::common::C2Rust_Unnamed_10;
pub use crate::src::common::common::C2Rust_Unnamed_11;
pub use crate::src::common::common::C2Rust_Unnamed_12;
pub use crate::src::common::common::C2Rust_Unnamed_13;
pub use crate::src::common::common::C2Rust_Unnamed_14;
pub use crate::src::common::common::C2Rust_Unnamed_15;
pub use crate::src::common::common::C2Rust_Unnamed_16;
pub use crate::src::common::common::C2Rust_Unnamed_17;
pub use crate::src::common::common::C2Rust_Unnamed_8;
pub use crate::src::common::common::C2Rust_Unnamed_9;
pub use crate::src::common::common::QP_BD_OFFSET;
pub use crate::src::common::common::QP_MAX;
pub use crate::src::common::common::QP_MAX_SPEC;
pub use crate::src::common::dct::x264_dct_function_t;
pub use crate::src::common::dct::x264_zigzag_function_t;
pub use crate::src::common::frame::x264_deblock_function_t;
pub use crate::src::common::frame::x264_deblock_inter_t;
pub use crate::src::common::frame::x264_deblock_intra_t;
pub use crate::src::common::frame::x264_frame;
pub use crate::src::common::frame::x264_frame_t;
pub use crate::src::common::frame::x264_sync_frame_list_t;
use crate::stdlib::pow;

pub use crate::src::common::mc::weight_fn_t;
pub use crate::src::common::mc::x264_mc_functions_t_10;
pub use crate::src::common::mc::x264_weight_t;
pub use crate::src::common::pixel::x264_pixel_cmp_t;
pub use crate::src::common::pixel::x264_pixel_cmp_x3_t;
pub use crate::src::common::pixel::x264_pixel_cmp_x4_t;
pub use crate::src::common::pixel::x264_pixel_function_t;
pub use crate::src::common::predict::x264_predict8x8_t;
pub use crate::src::common::predict::x264_predict_8x8_filter_t;
pub use crate::src::common::predict::x264_predict_t;
pub use crate::src::common::quant::x264_quant_function_t;
pub use crate::stdlib::pthread_cond_t;
pub use crate::stdlib::pthread_mutex_t;
pub use crate::stdlib::pthread_t;

pub use crate::stdlib::__pthread_mutex_s;
pub use crate::stdlib::int16_t;
pub use crate::stdlib::int32_t;
pub use crate::stdlib::int64_t;
pub use crate::stdlib::int8_t;
pub use crate::stdlib::intptr_t;
use crate::stdlib::memcmp;
use crate::stdlib::memcpy;
use crate::stdlib::memset;
use crate::stdlib::sscanf;
use crate::stdlib::strchr;
use crate::stdlib::strcspn;
use crate::stdlib::strlen;
use crate::stdlib::strpbrk;
use crate::stdlib::strstr;
pub use crate::stdlib::uint16_t;
pub use crate::stdlib::uint32_t;
pub use crate::stdlib::uint64_t;
pub use crate::stdlib::uint8_t;
pub use crate::stdlib::uintptr_t;

use crate::src::common::tables::x264_cqm_jvt4i;
use crate::src::common::tables::x264_cqm_jvt4p;
use crate::src::common::tables::x264_cqm_jvt8i;
use crate::src::common::tables::x264_cqm_jvt8p;
use crate::src::common::threadpool::x264_threadpool_t;
pub use crate::stdlib::__pthread_cond_s;
pub use crate::stdlib::__pthread_internal_list;
pub use crate::stdlib::__pthread_list_t;

pub use crate::stdlib::__int16_t;
pub use crate::stdlib::__int32_t;
pub use crate::stdlib::__int64_t;
pub use crate::stdlib::__int8_t;
pub use crate::stdlib::__uint16_t;
pub use crate::stdlib::__uint32_t;
pub use crate::stdlib::__uint64_t;
pub use crate::stdlib::__uint8_t;
pub use crate::x264_h::x264_hrd_t;
pub use crate::x264_h::x264_nal_t;
pub use crate::x264_h::x264_param_t;
pub use crate::x264_h::x264_sei_payload_t;
pub use crate::x264_h::x264_sei_t;
pub use crate::x264_h::x264_zone_t;
pub use crate::x264_h::C2Rust_Unnamed_0;
pub use crate::x264_h::C2Rust_Unnamed_1;
pub use crate::x264_h::C2Rust_Unnamed_2;
pub use crate::x264_h::C2Rust_Unnamed_3;
pub use crate::x264_h::C2Rust_Unnamed_4;
pub use crate::x264_h::C2Rust_Unnamed_5;
pub use crate::x264_h::X264_CQM_CUSTOM;
pub use crate::x264_h::X264_LOG_ERROR_1;

static mut dequant4_scale: [[crate::stdlib::uint8_t; 3]; 6] = [
    [
        10 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        11 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        13 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        20 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        14 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        23 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        16 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        20 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        25 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        23 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        29 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
];

static mut quant4_scale: [[crate::stdlib::uint16_t; 3]; 6] = [
    [
        13107 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        8066 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        5243 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
    [
        11916 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        7490 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        4660 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
    [
        10082 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        6554 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        4194 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
    [
        9362 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        5825 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        3647 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
    [
        8192 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        5243 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        3355 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
    [
        7282 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        4559 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        2893 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
];

static mut quant8_scan: [crate::stdlib::uint8_t; 16] = [
    0 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    4 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    2 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    3 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    5 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    1 as ::core::ffi::c_int as crate::stdlib::uint8_t,
];

static mut dequant8_scale: [[crate::stdlib::uint8_t; 6]; 6] = [
    [
        20 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        18 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        32 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        19 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        25 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        24 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        22 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        19 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        35 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        21 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        28 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        26 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        26 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        23 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        42 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        24 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        33 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        31 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        28 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        25 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        45 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        26 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        35 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        33 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        32 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        28 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        51 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        30 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        40 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        38 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
    [
        36 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        32 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        58 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        34 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        46 as ::core::ffi::c_int as crate::stdlib::uint8_t,
        43 as ::core::ffi::c_int as crate::stdlib::uint8_t,
    ],
];

static mut quant8_scale: [[crate::stdlib::uint16_t; 6]; 6] = [
    [
        13107 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        11428 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        20972 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        12222 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        16777 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        15481 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
    [
        11916 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        10826 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        19174 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        11058 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        14980 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        14290 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
    [
        10082 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        8943 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        15978 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        9675 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        12710 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        11985 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
    [
        9362 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        8228 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        14913 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        8931 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        11984 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        11259 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
    [
        8192 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        7346 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        13159 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        7740 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        10486 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        9777 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
    [
        7282 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        6428 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        11570 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        6830 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        9118 as ::core::ffi::c_int as crate::stdlib::uint16_t,
        8640 as ::core::ffi::c_int as crate::stdlib::uint16_t,
    ],
];
#[no_mangle]

pub unsafe extern "C" fn x264_8_cqm_init(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
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
        let mut min_qp_err: ::core::ffi::c_int =
            crate::src::common::common::QP_MAX + 1 as ::core::ffi::c_int;
        let mut num_8x8_lists: ::core::ffi::c_int = if (*(&raw mut (*h).sps
            as *mut crate::src::common::set::x264_sps_t))
            .i_chroma_format_idc
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            4 as ::core::ffi::c_int
        } else if (*h).param.analyse.b_transform_8x8 != 0 {
            2 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            if !(i < 4 as ::core::ffi::c_int) {
                c2rust_current_block = 5529461102203738653;
                break;
            }
            let mut size: ::core::ffi::c_int = 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int;
            let mut start: ::core::ffi::c_int =
                if 4 as ::core::ffi::c_int == 8 as ::core::ffi::c_int {
                    4 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                };
            let mut j: ::core::ffi::c_int = 0;
            j = 0 as ::core::ffi::c_int;
            while j < i {
                if crate::stdlib::memcmp(
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).scaling_list
                        [(i + start) as usize] as *const ::core::ffi::c_void,
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).scaling_list
                        [(j + start) as usize] as *const ::core::ffi::c_void,
                    (size as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>()
                            as crate::__stddef_size_t_h::size_t),
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
                (*h).quant4_mf[i as usize] = crate::src::common::base::x264_malloc(
                    (((51 as ::core::ffi::c_int
                        + 6 as ::core::ffi::c_int
                            * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int)
                        * size) as usize)
                        .wrapping_mul(
                            ::core::mem::size_of::<crate::src::common::common::udctcoef>() as usize,
                        ) as crate::stdlib::int64_t,
                )
                    as *mut [crate::src::common::common::udctcoef; 16];
                if (*h).quant4_mf[i as usize].is_null() {
                    c2rust_current_block = 16190416820810941155;
                    break;
                }
                (*h).dequant4_mf[i as usize] = crate::src::common::base::x264_malloc(
                    ((6 as ::core::ffi::c_int * size) as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        as crate::stdlib::int64_t,
                ) as *mut [::core::ffi::c_int; 16];
                if (*h).dequant4_mf[i as usize].is_null() {
                    c2rust_current_block = 16190416820810941155;
                    break;
                }
                (*h).unquant4_mf[i as usize] = crate::src::common::base::x264_malloc(
                    (((51 as ::core::ffi::c_int
                        + 6 as ::core::ffi::c_int
                            * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int)
                        * size) as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                        as crate::stdlib::int64_t,
                ) as *mut [::core::ffi::c_int; 16];
                if (*h).unquant4_mf[i as usize].is_null() {
                    c2rust_current_block = 16190416820810941155;
                    break;
                }
            }
            j = 0 as ::core::ffi::c_int;
            while j < i {
                if deadzone[j as usize] == deadzone[i as usize]
                    && crate::stdlib::memcmp(
                        (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .scaling_list[(i + start) as usize]
                            as *const ::core::ffi::c_void,
                        (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                            .scaling_list[(j + start) as usize]
                            as *const ::core::ffi::c_void,
                        (size as crate::__stddef_size_t_h::size_t)
                            .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>()
                                as crate::__stddef_size_t_h::size_t),
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
                (*h).quant4_bias[i as usize] = crate::src::common::base::x264_malloc(
                    (((51 as ::core::ffi::c_int
                        + 6 as ::core::ffi::c_int
                            * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int)
                        * size) as usize)
                        .wrapping_mul(
                            ::core::mem::size_of::<crate::src::common::common::udctcoef>() as usize,
                        ) as crate::stdlib::int64_t,
                )
                    as *mut [crate::src::common::common::udctcoef; 16];
                if (*h).quant4_bias[i as usize].is_null() {
                    c2rust_current_block = 16190416820810941155;
                    break;
                }
                (*h).quant4_bias0[i as usize] = crate::src::common::base::x264_malloc(
                    (((51 as ::core::ffi::c_int
                        + 6 as ::core::ffi::c_int
                            * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                        + 1 as ::core::ffi::c_int)
                        * size) as usize)
                        .wrapping_mul(
                            ::core::mem::size_of::<crate::src::common::common::udctcoef>() as usize,
                        ) as crate::stdlib::int64_t,
                )
                    as *mut [crate::src::common::common::udctcoef; 16];
                if (*h).quant4_bias0[i as usize].is_null() {
                    c2rust_current_block = 16190416820810941155;
                    break;
                }
            }
            i += 1;
        }
        match c2rust_current_block {
            5529461102203738653 => {
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                loop {
                    if !(i_0 < num_8x8_lists) {
                        c2rust_current_block = 7419121793134201633;
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
                        if crate::stdlib::memcmp(
                            (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                .scaling_list[(i_0 + start_0) as usize]
                                as *const ::core::ffi::c_void,
                            (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                .scaling_list[(j_0 + start_0) as usize]
                                as *const ::core::ffi::c_void,
                            (size_0 as crate::__stddef_size_t_h::size_t)
                                .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>()
                                    as crate::__stddef_size_t_h::size_t),
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
                        (*h).quant8_mf[i_0 as usize] = crate::src::common::base::x264_malloc(
                            (((51 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int
                                    * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                + 1 as ::core::ffi::c_int)
                                * size_0) as usize)
                                .wrapping_mul(::core::mem::size_of::<
                                    crate::src::common::common::udctcoef,
                                >() as usize) as crate::stdlib::int64_t,
                        )
                            as *mut [crate::src::common::common::udctcoef; 64];
                        if (*h).quant8_mf[i_0 as usize].is_null() {
                            c2rust_current_block = 16190416820810941155;
                            break;
                        }
                        (*h).dequant8_mf[i_0 as usize] = crate::src::common::base::x264_malloc(
                            ((6 as ::core::ffi::c_int * size_0) as usize)
                                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                as crate::stdlib::int64_t,
                        )
                            as *mut [::core::ffi::c_int; 64];
                        if (*h).dequant8_mf[i_0 as usize].is_null() {
                            c2rust_current_block = 16190416820810941155;
                            break;
                        }
                        (*h).unquant8_mf[i_0 as usize] = crate::src::common::base::x264_malloc(
                            (((51 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int
                                    * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                + 1 as ::core::ffi::c_int)
                                * size_0) as usize)
                                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as usize)
                                as crate::stdlib::int64_t,
                        )
                            as *mut [::core::ffi::c_int; 64];
                        if (*h).unquant8_mf[i_0 as usize].is_null() {
                            c2rust_current_block = 16190416820810941155;
                            break;
                        }
                    }
                    j_0 = 0 as ::core::ffi::c_int;
                    while j_0 < i_0 {
                        if deadzone[j_0 as usize] == deadzone[i_0 as usize]
                            && crate::stdlib::memcmp(
                                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                    .scaling_list[(i_0 + start_0) as usize]
                                    as *const ::core::ffi::c_void,
                                (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                    .scaling_list[(j_0 + start_0) as usize]
                                    as *const ::core::ffi::c_void,
                                (size_0 as crate::__stddef_size_t_h::size_t)
                                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>()
                                        as crate::__stddef_size_t_h::size_t),
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
                        (*h).quant8_bias[i_0 as usize] = crate::src::common::base::x264_malloc(
                            (((51 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int
                                    * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                + 1 as ::core::ffi::c_int)
                                * size_0) as usize)
                                .wrapping_mul(::core::mem::size_of::<
                                    crate::src::common::common::udctcoef,
                                >() as usize) as crate::stdlib::int64_t,
                        )
                            as *mut [crate::src::common::common::udctcoef; 64];
                        if (*h).quant8_bias[i_0 as usize].is_null() {
                            c2rust_current_block = 16190416820810941155;
                            break;
                        }
                        (*h).quant8_bias0[i_0 as usize] = crate::src::common::base::x264_malloc(
                            (((51 as ::core::ffi::c_int
                                + 6 as ::core::ffi::c_int
                                    * (8 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                + 1 as ::core::ffi::c_int)
                                * size_0) as usize)
                                .wrapping_mul(::core::mem::size_of::<
                                    crate::src::common::common::udctcoef,
                                >() as usize) as crate::stdlib::int64_t,
                        )
                            as *mut [crate::src::common::common::udctcoef; 64];
                        if (*h).quant8_bias0[i_0 as usize].is_null() {
                            c2rust_current_block = 16190416820810941155;
                            break;
                        }
                    }
                    i_0 += 1;
                }
                match c2rust_current_block {
                    16190416820810941155 => {}
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
                                let mut j_2: ::core::ffi::c_int = quant8_scan[(i_2
                                    >> 1 as ::core::ffi::c_int
                                    & 12 as ::core::ffi::c_int
                                    | i_2 & 3 as ::core::ffi::c_int)
                                    as usize]
                                    as ::core::ffi::c_int;
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
                                        * *(*(&raw mut (*h).sps
                                            as *mut crate::src::common::set::x264_sps_t))
                                            .scaling_list
                                            [i_list as usize]
                                            .offset(i_3 as isize)
                                            as ::core::ffi::c_int;
                                    quant4_mf[i_list as usize][q_0 as usize][i_3 as usize] =
                                        (def_quant4[q_0 as usize][i_3 as usize]
                                            * 16 as ::core::ffi::c_int
                                            + (*(*(&raw mut (*h).sps
                                                as *mut crate::src::common::set::x264_sps_t))
                                                .scaling_list
                                                [i_list as usize]
                                                .offset(i_3 as isize)
                                                as ::core::ffi::c_int
                                                >> 1 as ::core::ffi::c_int))
                                            / *(*(&raw mut (*h).sps
                                                as *mut crate::src::common::set::x264_sps_t))
                                                .scaling_list
                                                [i_list as usize]
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
                                        * *(*(&raw mut (*h).sps
                                            as *mut crate::src::common::set::x264_sps_t))
                                            .scaling_list
                                            [(4 as ::core::ffi::c_int + i_list_0) as usize]
                                            .offset(i_4 as isize)
                                            as ::core::ffi::c_int;
                                    quant8_mf[i_list_0 as usize][q_0 as usize][i_4 as usize] =
                                        (def_quant8[q_0 as usize][i_4 as usize]
                                            * 16 as ::core::ffi::c_int
                                            + (*(*(&raw mut (*h).sps
                                                as *mut crate::src::common::set::x264_sps_t))
                                                .scaling_list
                                                [(4 as ::core::ffi::c_int + i_list_0) as usize]
                                                .offset(i_4 as isize)
                                                as ::core::ffi::c_int
                                                >> 1 as ::core::ffi::c_int))
                                            / *(*(&raw mut (*h).sps
                                                as *mut crate::src::common::set::x264_sps_t))
                                                .scaling_list
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
                        while q_1 <= crate::src::common::common::QP_MAX_SPEC {
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
                                            >> q_1 / 6 as ::core::ffi::c_int
                                                - 1 as ::core::ffi::c_int
                                    };
                                    (*(*h).quant4_mf[i_list_1 as usize].offset(q_1 as isize))
                                        [i_5 as usize] = j_3 as crate::stdlib::uint16_t
                                        as crate::src::common::common::udctcoef;
                                    if j_3 == 0 {
                                        min_qp_err =
                                            if min_qp_err < q_1 { min_qp_err } else { q_1 };
                                    } else {
                                        (*(*h).quant4_bias[i_list_1 as usize]
                                            .offset(q_1 as isize))
                                            [i_5 as usize] = (if ((deadzone[i_list_1 as usize]
                                            << 10 as ::core::ffi::c_int)
                                            + (j_3 >> 1 as ::core::ffi::c_int))
                                            / j_3
                                            < ((1 as ::core::ffi::c_int)
                                                << 15 as ::core::ffi::c_int)
                                                / j_3
                                        {
                                            ((deadzone[i_list_1 as usize]
                                                << 10 as ::core::ffi::c_int)
                                                + (j_3 >> 1 as ::core::ffi::c_int))
                                                / j_3
                                        } else {
                                            ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                                                / j_3
                                        })
                                            as crate::src::common::common::udctcoef;
                                        (*(*h).quant4_bias0[i_list_1 as usize]
                                            .offset(q_1 as isize))
                                            [i_5 as usize] = (((1 as ::core::ffi::c_int)
                                            << 15 as ::core::ffi::c_int)
                                            / j_3)
                                            as crate::src::common::common::udctcoef;
                                        if j_3
                                            > (if (0xffff as ::core::ffi::c_int)
                                                < ((1 as ::core::ffi::c_int)
                                                    << 25 as ::core::ffi::c_int
                                                        - 8 as ::core::ffi::c_int)
                                                    - 1 as ::core::ffi::c_int
                                            {
                                                0xffff as ::core::ffi::c_int
                                            } else {
                                                ((1 as ::core::ffi::c_int)
                                                    << 25 as ::core::ffi::c_int
                                                        - 8 as ::core::ffi::c_int)
                                                    - 1 as ::core::ffi::c_int
                                            })
                                            && q_1 > max_qp_err
                                            && (i_list_1
                                                == crate::src::common::set::CQM_4IY
                                                    as ::core::ffi::c_int
                                                || i_list_1
                                                    == crate::src::common::set::CQM_4PY
                                                        as ::core::ffi::c_int)
                                        {
                                            max_qp_err = q_1;
                                        }
                                        if j_3
                                            > (if (0xffff as ::core::ffi::c_int)
                                                < ((1 as ::core::ffi::c_int)
                                                    << 25 as ::core::ffi::c_int
                                                        - 8 as ::core::ffi::c_int)
                                                    - 1 as ::core::ffi::c_int
                                            {
                                                0xffff as ::core::ffi::c_int
                                            } else {
                                                ((1 as ::core::ffi::c_int)
                                                    << 25 as ::core::ffi::c_int
                                                        - 8 as ::core::ffi::c_int)
                                                    - 1 as ::core::ffi::c_int
                                            })
                                            && q_1 > max_chroma_qp_err
                                            && (i_list_1
                                                == crate::src::common::set::CQM_4IC
                                                    as ::core::ffi::c_int
                                                || i_list_1
                                                    == crate::src::common::set::CQM_4PC
                                                        as ::core::ffi::c_int)
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
                                        (*(*h).unquant8_mf[i_list_2 as usize]
                                            .offset(q_1 as isize))
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
                                            [i_6 as usize] = j_3 as crate::stdlib::uint16_t
                                            as crate::src::common::common::udctcoef;
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
                                                ((1 as ::core::ffi::c_int)
                                                    << 15 as ::core::ffi::c_int)
                                                    / j_3
                                            })
                                                as crate::src::common::common::udctcoef;
                                            (*(*h).quant8_bias0[i_list_2 as usize]
                                                .offset(q_1 as isize))
                                                [i_6 as usize] = (((1 as ::core::ffi::c_int)
                                                << 15 as ::core::ffi::c_int)
                                                / j_3)
                                                as crate::src::common::common::udctcoef;
                                            if j_3
                                                > (if (0xffff as ::core::ffi::c_int)
                                                    < ((1 as ::core::ffi::c_int)
                                                        << 25 as ::core::ffi::c_int
                                                            - 8 as ::core::ffi::c_int)
                                                        - 1 as ::core::ffi::c_int
                                                {
                                                    0xffff as ::core::ffi::c_int
                                                } else {
                                                    ((1 as ::core::ffi::c_int)
                                                        << 25 as ::core::ffi::c_int
                                                            - 8 as ::core::ffi::c_int)
                                                        - 1 as ::core::ffi::c_int
                                                })
                                                && q_1 > max_qp_err
                                                && (i_list_2
                                                    == crate::src::common::set::CQM_8IY
                                                        as ::core::ffi::c_int
                                                    || i_list_2
                                                        == crate::src::common::set::CQM_8PY
                                                            as ::core::ffi::c_int)
                                            {
                                                max_qp_err = q_1;
                                            }
                                            if j_3
                                                > (if (0xffff as ::core::ffi::c_int)
                                                    < ((1 as ::core::ffi::c_int)
                                                        << 25 as ::core::ffi::c_int
                                                            - 8 as ::core::ffi::c_int)
                                                        - 1 as ::core::ffi::c_int
                                                {
                                                    0xffff as ::core::ffi::c_int
                                                } else {
                                                    ((1 as ::core::ffi::c_int)
                                                        << 25 as ::core::ffi::c_int
                                                            - 8 as ::core::ffi::c_int)
                                                        - 1 as ::core::ffi::c_int
                                                })
                                                && q_1 > max_chroma_qp_err
                                                && (i_list_2
                                                    == crate::src::common::set::CQM_8IC
                                                        as ::core::ffi::c_int
                                                    || i_list_2
                                                        == crate::src::common::set::CQM_8PC
                                                            as ::core::ffi::c_int)
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
                        (*h).nr_offset_emergency =
                            crate::src::common::base::x264_malloc(
                                (::core::mem::size_of::<
                                    [[crate::src::common::common::udctcoef; 64]; 4],
                                >() as usize)
                                    .wrapping_mul(
                                        (51 as ::core::ffi::c_int
                                            + 6 as ::core::ffi::c_int
                                                * (8 as ::core::ffi::c_int
                                                    - 8 as ::core::ffi::c_int)
                                            + 18 as ::core::ffi::c_int
                                            - (51 as ::core::ffi::c_int
                                                + 6 as ::core::ffi::c_int
                                                    * (8 as ::core::ffi::c_int
                                                        - 8 as ::core::ffi::c_int)))
                                            as usize,
                                    ) as crate::stdlib::int64_t,
                            )
                                as *mut [[crate::src::common::common::udctcoef; 64]; 4];
                        if !(*h).nr_offset_emergency.is_null() {
                            let mut q_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                            while q_2
                                < crate::src::common::common::QP_MAX
                                    - crate::src::common::common::QP_MAX_SPEC
                            {
                                let mut cat: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                while cat
                                    < 3 as ::core::ffi::c_int
                                        + (crate::src::common::base::CHROMA_444
                                            as ::core::ffi::c_int
                                            == crate::src::common::base::CHROMA_444
                                                as ::core::ffi::c_int)
                                            as ::core::ffi::c_int
                                {
                                    let mut dct8x8: ::core::ffi::c_int =
                                        cat & 1 as ::core::ffi::c_int;
                                    if !((*h).param.analyse.b_transform_8x8 == 0 && dct8x8 != 0) {
                                        let mut size_1: ::core::ffi::c_int = if dct8x8 != 0 {
                                            64 as ::core::ffi::c_int
                                        } else {
                                            16 as ::core::ffi::c_int
                                        };
                                        let mut nr_offset: *mut crate::src::common::common::udctcoef = &raw mut *(&raw mut *(*h)
                                            .nr_offset_emergency
                                            .offset(q_2 as isize)
                                            as *mut [crate::src::common::common::udctcoef; 64])
                                            .offset(cat as isize)
                                            as *mut crate::src::common::common::udctcoef;
                                        let mut dc_threshold: ::core::ffi::c_int =
                                            (crate::src::common::common::QP_MAX
                                                - crate::src::common::common::QP_MAX_SPEC)
                                                * 2 as ::core::ffi::c_int
                                                / 3 as ::core::ffi::c_int;
                                        let mut luma_threshold: ::core::ffi::c_int =
                                            (crate::src::common::common::QP_MAX
                                                - crate::src::common::common::QP_MAX_SPEC)
                                                * 2 as ::core::ffi::c_int
                                                / 3 as ::core::ffi::c_int;
                                        let mut chroma_threshold: ::core::ffi::c_int =
                                            0 as ::core::ffi::c_int;
                                        let mut i_7: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                        while i_7 < size_1 {
                                            let mut max: ::core::ffi::c_int = ((1
                                                as ::core::ffi::c_int)
                                                << 7 as ::core::ffi::c_int
                                                    + crate::internal::BIT_DEPTH)
                                                - 1 as ::core::ffi::c_int;
                                            if q_2
                                                == crate::src::common::common::QP_MAX
                                                    - crate::src::common::common::QP_MAX_SPEC
                                                    - 1 as ::core::ffi::c_int
                                            {
                                                *nr_offset.offset(i_7 as isize) =
                                                    max as crate::src::common::common::udctcoef;
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
                                                    *nr_offset.offset(i_7 as isize) =
                                                        0 as crate::src::common::common::udctcoef;
                                                } else {
                                                    let mut pos: ::core::ffi::c_double =
                                                        (q_2 - thresh + 1 as ::core::ffi::c_int)
                                                            as ::core::ffi::c_double
                                                            / (crate::src::common::common::QP_MAX - crate::src::common::common::QP_MAX_SPEC - thresh)
                                                                as ::core::ffi::c_double;
                                                    let mut start_1: ::core::ffi::c_double =
                                                        (if dct8x8 != 0 {
                                                            (*(*h).unquant8_mf[crate::src::common::set::CQM_8PY
                                                                as ::core::ffi::c_int
                                                                as usize]
                                                                .offset(crate::src::common::common::QP_MAX_SPEC as isize))
                                                                [i_7 as usize]
                                                        } else {
                                                            (*(*h).unquant4_mf[crate::src::common::set::CQM_4PY
                                                                as ::core::ffi::c_int
                                                                as usize]
                                                                .offset(crate::src::common::common::QP_MAX_SPEC as isize))
                                                                [i_7 as usize]
                                                        })
                                                            as ::core::ffi::c_double;
                                                    let mut bias: ::core::ffi::c_double = (crate::stdlib::pow(
                                                        2 as ::core::ffi::c_int
                                                            as ::core::ffi::c_double,
                                                        pos * (crate::src::common::common::QP_MAX - crate::src::common::common::QP_MAX_SPEC)
                                                            as ::core::ffi::c_double
                                                            / 10.0f64,
                                                    )
                                                        * 0.003f64
                                                        - 0.003f64)
                                                        * start_1;
                                                    *nr_offset.offset(i_7 as isize) = (if bias
                                                        + 0.5f64
                                                        < max as ::core::ffi::c_double
                                                    {
                                                        bias + 0.5f64
                                                    } else {
                                                        max as ::core::ffi::c_double
                                                    })
                                                        as crate::src::common::common::udctcoef;
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
                                                * (8 as ::core::ffi::c_int
                                                    - 8 as ::core::ffi::c_int)
                                    {
                                        (*h).param.rc.i_qp_min
                                    } else {
                                        51 as ::core::ffi::c_int
                                            + 6 as ::core::ffi::c_int
                                                * (8 as ::core::ffi::c_int
                                                    - 8 as ::core::ffi::c_int)
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
                                    && (*(&raw mut (*h).sps
                                        as *mut crate::src::common::set::x264_sps_t))
                                        .i_profile_idc
                                        < crate::src::common::base::PROFILE_HIGH
                                            as ::core::ffi::c_int
                                {
                                    while *(*h).chroma_qp_table.offset(
                                        (if (*h).param.rc.i_qp_max
                                            < 51 as ::core::ffi::c_int
                                                + 6 as ::core::ffi::c_int
                                                    * (8 as ::core::ffi::c_int
                                                        - 8 as ::core::ffi::c_int)
                                        {
                                            (*h).param.rc.i_qp_max
                                        } else {
                                            51 as ::core::ffi::c_int
                                                + 6 as ::core::ffi::c_int
                                                    * (8 as ::core::ffi::c_int
                                                        - 8 as ::core::ffi::c_int)
                                        }) as isize,
                                    )
                                        as ::core::ffi::c_int
                                        <= 12 as ::core::ffi::c_int
                                        || (*h).param.rc.i_qp_max <= 12 as ::core::ffi::c_int
                                    {
                                        (*h).param.rc.i_qp_max += 1;
                                    }
                                }
                                if (*h).param.rc.i_qp_min > (*h).param.rc.i_qp_max {
                                    crate::src::common::common::x264_8_log(
                                        h as *mut crate::src::common::common::x264_t,
                                        crate::x264_h::X264_LOG_ERROR_1,
                                        b"Impossible QP constraints for CQM (min=%d, max=%d)\n\0"
                                            .as_ptr()
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
        x264_8_cqm_delete(h);
        return -(1 as ::core::ffi::c_int);
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_cqm_delete(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
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
                crate::src::common::base::x264_free(
                    (*h).quant4_mf[i as usize] as *mut ::core::ffi::c_void,
                );
                crate::src::common::base::x264_free(
                    (*h).dequant4_mf[i as usize] as *mut ::core::ffi::c_void,
                );
                crate::src::common::base::x264_free(
                    (*h).unquant4_mf[i as usize] as *mut ::core::ffi::c_void,
                );
            }
            j = 0 as ::core::ffi::c_int;
            while j < i {
                if (*h).quant4_bias[i as usize] == (*h).quant4_bias[j as usize] {
                    break;
                }
                j += 1;
            }
            if j == i {
                crate::src::common::base::x264_free(
                    (*h).quant4_bias[i as usize] as *mut ::core::ffi::c_void,
                );
                crate::src::common::base::x264_free(
                    (*h).quant4_bias0[i as usize] as *mut ::core::ffi::c_void,
                );
            }
            i += 1;
        }
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0
            < (if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
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
                crate::src::common::base::x264_free(
                    (*h).quant8_mf[i_0 as usize] as *mut ::core::ffi::c_void,
                );
                crate::src::common::base::x264_free(
                    (*h).dequant8_mf[i_0 as usize] as *mut ::core::ffi::c_void,
                );
                crate::src::common::base::x264_free(
                    (*h).unquant8_mf[i_0 as usize] as *mut ::core::ffi::c_void,
                );
            }
            j_0 = 0 as ::core::ffi::c_int;
            while j_0 < i_0 {
                if (*h).quant8_bias[i_0 as usize] == (*h).quant8_bias[j_0 as usize] {
                    break;
                }
                j_0 += 1;
            }
            if j_0 == i_0 {
                crate::src::common::base::x264_free(
                    (*h).quant8_bias[i_0 as usize] as *mut ::core::ffi::c_void,
                );
                crate::src::common::base::x264_free(
                    (*h).quant8_bias0[i_0 as usize] as *mut ::core::ffi::c_void,
                );
            }
            i_0 += 1;
        }
        crate::src::common::base::x264_free((*h).nr_offset_emergency as *mut ::core::ffi::c_void);
    }
}

unsafe extern "C" fn cqm_parse_jmlist(
    mut h: *mut crate::src::common::common::x264_t,
    mut buf: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut cqm: *mut crate::stdlib::uint8_t,
    mut jvt: *const crate::stdlib::uint8_t,
    mut length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i: ::core::ffi::c_int = 0;
        let mut p: *mut ::core::ffi::c_char = crate::stdlib::strstr(buf, name);
        if p.is_null() {
            crate::stdlib::memset(
                cqm as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                length as crate::__stddef_size_t_h::size_t,
            );
            return 0 as ::core::ffi::c_int;
        }
        p = p.offset(crate::stdlib::strlen(name) as isize);
        if *p as ::core::ffi::c_int == 'U' as i32 || *p as ::core::ffi::c_int == 'V' as i32 {
            p = p.offset(1);
        }
        let mut nextvar: *mut ::core::ffi::c_char =
            crate::stdlib::strstr(p, b"INT\0".as_ptr() as *const ::core::ffi::c_char);
        i = 0 as ::core::ffi::c_int;
        while i < length
            && {
                p = crate::stdlib::strpbrk(p, b" \t\n,\0".as_ptr() as *const ::core::ffi::c_char);
                !p.is_null()
            }
            && {
                p = crate::stdlib::strpbrk(
                    p,
                    b"0123456789\0".as_ptr() as *const ::core::ffi::c_char,
                );
                !p.is_null()
            }
        {
            let mut coef: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            crate::stdlib::sscanf(
                p,
                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut coef,
            );
            if i == 0 as ::core::ffi::c_int && coef == 0 as ::core::ffi::c_int {
                crate::stdlib::memcpy(
                    cqm as *mut ::core::ffi::c_void,
                    jvt as *const ::core::ffi::c_void,
                    length as crate::__stddef_size_t_h::size_t,
                );
                return 0 as ::core::ffi::c_int;
            }
            if coef < 1 as ::core::ffi::c_int || coef > 255 as ::core::ffi::c_int {
                crate::src::common::common::x264_8_log(
                    h as *mut crate::src::common::common::x264_t,
                    crate::x264_h::X264_LOG_ERROR_1,
                    b"bad coefficient in list '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                    name,
                );
                return -(1 as ::core::ffi::c_int);
            }
            *cqm.offset(i as isize) = coef as crate::stdlib::uint8_t;
            i += 1;
        }
        if !nextvar.is_null() && p > nextvar || i != length {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"not enough coefficients in list '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                name,
            );
            return -(1 as ::core::ffi::c_int);
        }
        return 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]

pub unsafe extern "C" fn x264_8_cqm_parse_file(
    mut h: *mut crate::src::common::common::x264_t,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut b_error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        (*h).param.i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
        let mut buf: *mut ::core::ffi::c_char = crate::src::common::base::x264_slurp_file(filename);
        if buf.is_null() {
            crate::src::common::common::x264_8_log(
                h as *mut crate::src::common::common::x264_t,
                crate::x264_h::X264_LOG_ERROR_1,
                b"can't open file '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
                filename,
            );
            return -(1 as ::core::ffi::c_int);
        }
        loop {
            p = crate::stdlib::strchr(buf, '#' as i32);
            if p.is_null() {
                break;
            }
            crate::stdlib::memset(
                p as *mut ::core::ffi::c_void,
                ' ' as i32,
                crate::stdlib::strcspn(p, b"\n\0".as_ptr() as *const ::core::ffi::c_char)
                    as crate::__stddef_size_t_h::size_t,
            );
        }
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTRA4X4_LUMA\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*h).param.cqm_4iy as *mut crate::stdlib::uint8_t,
            &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTER4X4_LUMA\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*h).param.cqm_4py as *mut crate::stdlib::uint8_t,
            &raw const crate::src::common::tables::x264_cqm_jvt4p as *const crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTRA4X4_CHROMA\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*h).param.cqm_4ic as *mut crate::stdlib::uint8_t,
            &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTER4X4_CHROMA\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*h).param.cqm_4pc as *mut crate::stdlib::uint8_t,
            &raw const crate::src::common::tables::x264_cqm_jvt4p as *const crate::stdlib::uint8_t,
            16 as ::core::ffi::c_int,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTRA8X8_LUMA\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*h).param.cqm_8iy as *mut crate::stdlib::uint8_t,
            &raw const crate::src::common::tables::x264_cqm_jvt8i as *const crate::stdlib::uint8_t,
            64 as ::core::ffi::c_int,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTER8X8_LUMA\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*h).param.cqm_8py as *mut crate::stdlib::uint8_t,
            &raw const crate::src::common::tables::x264_cqm_jvt8p as *const crate::stdlib::uint8_t,
            64 as ::core::ffi::c_int,
        );
        if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            b_error |= cqm_parse_jmlist(
                h,
                buf,
                b"INTRA8X8_CHROMA\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*h).param.cqm_8ic as *mut crate::stdlib::uint8_t,
                &raw const crate::src::common::tables::x264_cqm_jvt8i
                    as *const crate::stdlib::uint8_t,
                64 as ::core::ffi::c_int,
            );
            b_error |= cqm_parse_jmlist(
                h,
                buf,
                b"INTER8X8_CHROMA\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*h).param.cqm_8pc as *mut crate::stdlib::uint8_t,
                &raw const crate::src::common::tables::x264_cqm_jvt8p
                    as *const crate::stdlib::uint8_t,
                64 as ::core::ffi::c_int,
            );
        }
        crate::src::common::base::x264_free(buf as *mut ::core::ffi::c_void);
        return b_error;
    }
}
