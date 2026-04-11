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
    pub constraint_set0: bool,
    pub constraint_set1: bool,
    pub constraint_set2: bool,
    pub constraint_set3: bool,
    pub i_log2_max_frame_num: ::core::ffi::c_int,
    pub i_poc_type: ::core::ffi::c_int,
    pub i_log2_max_poc_lsb: ::core::ffi::c_int,
    pub i_num_ref_frames: ::core::ffi::c_int,
    pub gaps_in_frame_num_value_allowed: bool,
    pub i_mb_width: ::core::ffi::c_int,
    pub i_mb_height: ::core::ffi::c_int,
    pub frame_mbs_only: bool,
    pub mb_adaptive_frame_field: bool,
    pub direct8x8_inference: bool,
    pub has_crop: bool,
    pub crop: crate::src::common::set::C2Rust_Unnamed_19,
    pub has_vui: bool,
    pub vui: crate::src::common::set::C2Rust_Unnamed_20,
    pub qpprime_y_zero_transform_bypass: bool,
    pub i_chroma_format_idc: ::core::ffi::c_int,
    pub avcintra_hd: bool,
    pub avcintra_4k: bool,
    pub i_cqm_preset: ::core::ffi::c_int,
    pub scaling_list: [*const crate::stdlib::uint8_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub i_left: ::core::ffi::c_int,
    pub i_right: ::core::ffi::c_int,
    pub i_top: ::core::ffi::c_int,
    pub i_bottom: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub aspect_ratio_info_present: bool,
    pub i_sar_width: ::core::ffi::c_int,
    pub i_sar_height: ::core::ffi::c_int,
    pub overscan_info_present: bool,
    pub overscan_info: bool,
    pub signal_type_present: bool,
    pub i_vidformat: ::core::ffi::c_int,
    pub fullrange: bool,
    pub color_description_present: bool,
    pub i_colorprim: ::core::ffi::c_int,
    pub i_transfer: ::core::ffi::c_int,
    pub i_colmatrix: ::core::ffi::c_int,
    pub chroma_loc_info_present: bool,
    pub i_chroma_loc_top: ::core::ffi::c_int,
    pub i_chroma_loc_bottom: ::core::ffi::c_int,
    pub timing_info_present: bool,
    pub i_num_units_in_tick: crate::stdlib::uint32_t,
    pub i_time_scale: crate::stdlib::uint32_t,
    pub fixed_frame_rate: bool,
    pub nal_hrd_parameters_present: bool,
    pub vcl_hrd_parameters_present: bool,
    pub hrd: crate::src::common::set::C2Rust_Unnamed_21,
    pub pic_struct_present: bool,
    pub bitstream_restriction: bool,
    pub motion_vectors_over_pic_boundaries: bool,
    pub i_max_bytes_per_pic_denom: ::core::ffi::c_int,
    pub i_max_bits_per_mb_denom: ::core::ffi::c_int,
    pub i_log2_max_mv_length_horizontal: ::core::ffi::c_int,
    pub i_log2_max_mv_length_vertical: ::core::ffi::c_int,
    pub i_num_reorder_frames: ::core::ffi::c_int,
    pub i_max_dec_frame_buffering: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub i_cpb_cnt: ::core::ffi::c_int,
    pub i_bit_rate_scale: ::core::ffi::c_int,
    pub i_cpb_size_scale: ::core::ffi::c_int,
    pub i_bit_rate_value: ::core::ffi::c_int,
    pub i_cpb_size_value: ::core::ffi::c_int,
    pub i_bit_rate_unscaled: ::core::ffi::c_int,
    pub i_cpb_size_unscaled: ::core::ffi::c_int,
    pub cbr_hrd: bool,
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
    pub cabac: bool,
    pub pic_order: bool,
    pub i_num_slice_groups: ::core::ffi::c_int,
    pub i_num_ref_idx_l0_default_active: ::core::ffi::c_int,
    pub i_num_ref_idx_l1_default_active: ::core::ffi::c_int,
    pub weighted_pred: bool,
    pub weighted_bipred: ::core::ffi::c_int,
    pub i_pic_init_qp: ::core::ffi::c_int,
    pub i_pic_init_qs: ::core::ffi::c_int,
    pub i_chroma_qp_index_offset: ::core::ffi::c_int,
    pub deblocking_filter_control: bool,
    pub constrained_intra_pred: bool,
    pub redundant_pic_cnt: bool,
    pub transform_8x8_mode: bool,
}
static mut dequant4_scale: [[crate::stdlib::uint8_t; 3]; 6] = [
    [10u8, 13u8, 16u8],
    [11u8, 14u8, 18u8],
    [13u8, 16u8, 20u8],
    [14u8, 18u8, 23u8],
    [16u8, 20u8, 25u8],
    [18u8, 23u8, 29u8],
];
static mut quant4_scale: [[crate::stdlib::uint16_t; 3]; 6] = [
    [13107u16, 8066u16, 5243u16],
    [11916u16, 7490u16, 4660u16],
    [10082u16, 6554u16, 4194u16],
    [9362u16, 5825u16, 3647u16],
    [8192u16, 5243u16, 3355u16],
    [7282u16, 4559u16, 2893u16],
];
static mut quant8_scan: [crate::stdlib::uint8_t; 16] = [
    0u8, 3u8, 4u8, 3u8, 3u8, 1u8, 5u8, 1u8, 4u8, 5u8, 2u8, 5u8, 3u8, 1u8, 5u8, 1u8,
];
static mut dequant8_scale: [[crate::stdlib::uint8_t; 6]; 6] = [
    [20u8, 18u8, 32u8, 19u8, 25u8, 24u8],
    [22u8, 19u8, 35u8, 21u8, 28u8, 26u8],
    [26u8, 23u8, 42u8, 24u8, 33u8, 31u8],
    [28u8, 25u8, 45u8, 26u8, 35u8, 33u8],
    [32u8, 28u8, 51u8, 30u8, 40u8, 38u8],
    [36u8, 32u8, 58u8, 34u8, 46u8, 43u8],
];
static mut quant8_scale: [[crate::stdlib::uint16_t; 6]; 6] = [
    [13107u16, 11428u16, 20972u16, 12222u16, 16777u16, 15481u16],
    [11916u16, 10826u16, 19174u16, 11058u16, 14980u16, 14290u16],
    [10082u16, 8943u16, 15978u16, 9675u16, 12710u16, 11985u16],
    [9362u16, 8228u16, 14913u16, 8931u16, 11984u16, 11259u16],
    [8192u16, 7346u16, 13159u16, 7740u16, 10486u16, 9777u16],
    [7282u16, 6428u16, 11570u16, 6830u16, 9118u16, 8640u16],
];
pub unsafe extern "C" fn x264_8_cqm_init(
    mut h: *mut crate::src::common::common::x264_t,
) -> ::core::ffi::c_int {
    unsafe {
        let mut c2rust_current_block: u64;
        let mut deadzone = [
            32i32 - (*h).param.analyse.i_luma_deadzone[1usize],
            32i32 - (*h).param.analyse.i_luma_deadzone[0usize],
            32i32 - 11i32,
            32i32 - 21i32,
        ];
        let mut max_qp_err = -(1i32);
        let mut max_chroma_qp_err = -(1i32);
        let mut min_qp_err = crate::src::common::common::QP_MAX + 1i32;
        let mut num_8x8_lists = if (*(&raw mut (*h).sps
            as *mut crate::src::common::set::x264_sps_t))
            .i_chroma_format_idc
            == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
        {
            4i32
        } else if (*h).param.analyse.transform_8x8 {
            2i32
        } else {
            0i32
        };
        loop {
            let mut i = 0i32;
            let mut j = 0i32;
            if !(i < 4i32) {
                c2rust_current_block = 5529461102203738653;
                break;
            }
            let mut size = 4i32 * 4i32;
            let mut start = if 4i32 == 8i32 { 4i32 } else { 0i32 };
            while j < i {
                if crate::stdlib::memcmp(
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).scaling_list
                        [(i + start) as usize] as *const ::core::ffi::c_void,
                    (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t)).scaling_list
                        [(j + start) as usize] as *const ::core::ffi::c_void,
                    (size as crate::__stddef_size_t_h::size_t)
                        .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>()),
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
                    (((51i32 + 6i32 * (8i32 - 8i32) + 1i32) * size) as usize).wrapping_mul(
                        ::core::mem::size_of::<crate::src::common::common::udctcoef>(),
                    ) as crate::stdlib::int64_t,
                )
                    as *mut [crate::src::common::common::udctcoef; 16];
                if (*h).quant4_mf[i as usize].is_null() {
                    c2rust_current_block = 16190416820810941155;
                    break;
                }
                (*h).dequant4_mf[i as usize] = crate::src::common::base::x264_malloc(
                    ((6i32 * size) as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>())
                        as crate::stdlib::int64_t,
                ) as *mut [::core::ffi::c_int; 16];
                if (*h).dequant4_mf[i as usize].is_null() {
                    c2rust_current_block = 16190416820810941155;
                    break;
                }
                (*h).unquant4_mf[i as usize] = crate::src::common::base::x264_malloc(
                    (((51i32 + 6i32 * (8i32 - 8i32) + 1i32) * size) as usize)
                        .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>())
                        as crate::stdlib::int64_t,
                ) as *mut [::core::ffi::c_int; 16];
                if (*h).unquant4_mf[i as usize].is_null() {
                    c2rust_current_block = 16190416820810941155;
                    break;
                }
            }
            j = 0i32;
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
                            .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>()),
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
                    (((51i32 + 6i32 * (8i32 - 8i32) + 1i32) * size) as usize).wrapping_mul(
                        ::core::mem::size_of::<crate::src::common::common::udctcoef>(),
                    ) as crate::stdlib::int64_t,
                )
                    as *mut [crate::src::common::common::udctcoef; 16];
                if (*h).quant4_bias[i as usize].is_null() {
                    c2rust_current_block = 16190416820810941155;
                    break;
                }
                (*h).quant4_bias0[i as usize] = crate::src::common::base::x264_malloc(
                    (((51i32 + 6i32 * (8i32 - 8i32) + 1i32) * size) as usize).wrapping_mul(
                        ::core::mem::size_of::<crate::src::common::common::udctcoef>(),
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
                loop {
                    let mut i_0 = 0i32;
                    let mut j_0 = 0i32;
                    if !(i_0 < num_8x8_lists) {
                        c2rust_current_block = 7419121793134201633;
                        break;
                    }
                    let mut size_0 = 8i32 * 8i32;
                    let mut start_0 = if 8i32 == 8i32 { 4i32 } else { 0i32 };
                    while j_0 < i_0 {
                        if crate::stdlib::memcmp(
                            (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                .scaling_list[(i_0 + start_0) as usize]
                                as *const ::core::ffi::c_void,
                            (*(&raw mut (*h).sps as *mut crate::src::common::set::x264_sps_t))
                                .scaling_list[(j_0 + start_0) as usize]
                                as *const ::core::ffi::c_void,
                            (size_0 as crate::__stddef_size_t_h::size_t)
                                .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>()),
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
                            (((51i32 + 6i32 * (8i32 - 8i32) + 1i32) * size_0) as usize)
                                .wrapping_mul(::core::mem::size_of::<
                                    crate::src::common::common::udctcoef,
                                >()) as crate::stdlib::int64_t,
                        )
                            as *mut [crate::src::common::common::udctcoef; 64];
                        if (*h).quant8_mf[i_0 as usize].is_null() {
                            c2rust_current_block = 16190416820810941155;
                            break;
                        }
                        (*h).dequant8_mf[i_0 as usize] = crate::src::common::base::x264_malloc(
                            ((6i32 * size_0) as usize)
                                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>())
                                as crate::stdlib::int64_t,
                        )
                            as *mut [::core::ffi::c_int; 64];
                        if (*h).dequant8_mf[i_0 as usize].is_null() {
                            c2rust_current_block = 16190416820810941155;
                            break;
                        }
                        (*h).unquant8_mf[i_0 as usize] = crate::src::common::base::x264_malloc(
                            (((51i32 + 6i32 * (8i32 - 8i32) + 1i32) * size_0) as usize)
                                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>())
                                as crate::stdlib::int64_t,
                        )
                            as *mut [::core::ffi::c_int; 64];
                        if (*h).unquant8_mf[i_0 as usize].is_null() {
                            c2rust_current_block = 16190416820810941155;
                            break;
                        }
                    }
                    j_0 = 0i32;
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
                                    .wrapping_mul(::core::mem::size_of::<crate::stdlib::uint8_t>()),
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
                            (((51i32 + 6i32 * (8i32 - 8i32) + 1i32) * size_0) as usize)
                                .wrapping_mul(::core::mem::size_of::<
                                    crate::src::common::common::udctcoef,
                                >()) as crate::stdlib::int64_t,
                        )
                            as *mut [crate::src::common::common::udctcoef; 64];
                        if (*h).quant8_bias[i_0 as usize].is_null() {
                            c2rust_current_block = 16190416820810941155;
                            break;
                        }
                        (*h).quant8_bias0[i_0 as usize] = crate::src::common::base::x264_malloc(
                            (((51i32 + 6i32 * (8i32 - 8i32) + 1i32) * size_0) as usize)
                                .wrapping_mul(::core::mem::size_of::<
                                    crate::src::common::common::udctcoef,
                                >()) as crate::stdlib::int64_t,
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
                        let mut def_quant4 = [[0; 16]; 6];
                        let mut def_quant8 = [[0; 64]; 6];
                        let mut def_dequant4 = [[0; 16]; 6];
                        let mut def_dequant8 = [[0; 64]; 6];
                        let mut quant4_mf = [[[0; 16]; 6]; 4];
                        let mut quant8_mf = [[[0; 64]; 6]; 4];
                        let mut q = 0i32;
                        let mut q_0 = 0i32;
                        let mut q_1 = 0i32;
                        while q < 6i32 {
                            let mut i_1 = 0i32;
                            let mut i_2 = 0i32;
                            while i_1 < 16i32 {
                                let mut j_1 = (i_1 & 1i32) + (i_1 >> 2i32 & 1i32);
                                def_dequant4[q as usize][i_1 as usize] =
                                    dequant4_scale[q as usize][j_1 as usize] as ::core::ffi::c_int;
                                def_quant4[q as usize][i_1 as usize] =
                                    quant4_scale[q as usize][j_1 as usize] as ::core::ffi::c_int;
                                i_1 += 1;
                            }
                            while i_2 < 64i32 {
                                let mut j_2 = quant8_scan
                                    [(i_2 >> 1i32 & 12i32 | i_2 & 3i32) as usize]
                                    as ::core::ffi::c_int;
                                def_dequant8[q as usize][i_2 as usize] =
                                    dequant8_scale[q as usize][j_2 as usize] as ::core::ffi::c_int;
                                def_quant8[q as usize][i_2 as usize] =
                                    quant8_scale[q as usize][j_2 as usize] as ::core::ffi::c_int;
                                i_2 += 1;
                            }
                            q += 1;
                        }
                        while q_0 < 6i32 {
                            let mut i_list = 0i32;
                            let mut i_list_0 = 0i32;
                            while i_list < 4i32 {
                                let mut i_3 = 0i32;
                                while i_3 < 16i32 {
                                    (*(*h).dequant4_mf[i_list as usize].offset(q_0 as isize))
                                        [i_3 as usize] = def_dequant4[q_0 as usize][i_3 as usize]
                                        * *(*(&raw mut (*h).sps
                                            as *mut crate::src::common::set::x264_sps_t))
                                            .scaling_list
                                            [i_list as usize]
                                            .offset(i_3 as isize)
                                            as ::core::ffi::c_int;
                                    quant4_mf[i_list as usize][q_0 as usize][i_3 as usize] =
                                        (def_quant4[q_0 as usize][i_3 as usize] * 16i32
                                            + (*(*(&raw mut (*h).sps
                                                as *mut crate::src::common::set::x264_sps_t))
                                                .scaling_list
                                                [i_list as usize]
                                                .offset(i_3 as isize)
                                                as ::core::ffi::c_int
                                                >> 1i32))
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
                            while i_list_0 < num_8x8_lists {
                                let mut i_4 = 0i32;
                                while i_4 < 64i32 {
                                    (*(*h).dequant8_mf[i_list_0 as usize].offset(q_0 as isize))
                                        [i_4 as usize] = def_dequant8[q_0 as usize][i_4 as usize]
                                        * *(*(&raw mut (*h).sps
                                            as *mut crate::src::common::set::x264_sps_t))
                                            .scaling_list
                                            [(4i32 + i_list_0) as usize]
                                            .offset(i_4 as isize)
                                            as ::core::ffi::c_int;
                                    quant8_mf[i_list_0 as usize][q_0 as usize][i_4 as usize] =
                                        (def_quant8[q_0 as usize][i_4 as usize] * 16i32
                                            + (*(*(&raw mut (*h).sps
                                                as *mut crate::src::common::set::x264_sps_t))
                                                .scaling_list
                                                [(4i32 + i_list_0) as usize]
                                                .offset(i_4 as isize)
                                                as ::core::ffi::c_int
                                                >> 1i32))
                                            / *(*(&raw mut (*h).sps
                                                as *mut crate::src::common::set::x264_sps_t))
                                                .scaling_list
                                                [(4i32 + i_list_0) as usize]
                                                .offset(i_4 as isize)
                                                as ::core::ffi::c_int;
                                    i_4 += 1;
                                }
                                i_list_0 += 1;
                            }
                            q_0 += 1;
                        }
                        while q_1 <= crate::src::common::common::QP_MAX_SPEC {
                            let mut j_3 = 0;
                            let mut i_list_1 = 0i32;
                            while i_list_1 < 4i32 {
                                let mut i_5 = 0i32;
                                while i_5 < 16i32 {
                                    (*(*h).unquant4_mf[i_list_1 as usize].offset(q_1 as isize))
                                        [i_5 as usize] = ((1u64) << q_1 / 6i32 + 15i32 + 8i32)
                                        .wrapping_div(
                                            quant4_mf[i_list_1 as usize][(q_1 % 6i32) as usize]
                                                [i_5 as usize]
                                                as ::core::ffi::c_ulonglong,
                                        )
                                        as ::core::ffi::c_int;
                                    j_3 = if q_1 / 6i32 - 1i32 <= 0i32 {
                                        quant4_mf[i_list_1 as usize][(q_1 % 6i32) as usize]
                                            [i_5 as usize]
                                            << -(q_1 / 6i32 - 1i32)
                                    } else {
                                        quant4_mf[i_list_1 as usize][(q_1 % 6i32) as usize]
                                            [i_5 as usize]
                                            + ((1i32) << q_1 / 6i32 - 1i32 - 1i32)
                                            >> q_1 / 6i32 - 1i32
                                    };
                                    (*(*h).quant4_mf[i_list_1 as usize].offset(q_1 as isize))
                                        [i_5 as usize] =
                                        j_3 as crate::src::common::common::udctcoef;
                                    if j_3 == 0 {
                                        min_qp_err =
                                            if min_qp_err < q_1 { min_qp_err } else { q_1 };
                                    } else {
                                        (*(*h).quant4_bias[i_list_1 as usize]
                                            .offset(q_1 as isize))
                                            [i_5 as usize] = (if ((deadzone[i_list_1 as usize]
                                            << 10i32)
                                            + (j_3 >> 1i32))
                                            / j_3
                                            < ((1i32) << 15i32) / j_3
                                        {
                                            ((deadzone[i_list_1 as usize] << 10i32) + (j_3 >> 1i32))
                                                / j_3
                                        } else {
                                            ((1i32) << 15i32) / j_3
                                        })
                                            as crate::src::common::common::udctcoef;
                                        (*(*h).quant4_bias0[i_list_1 as usize]
                                            .offset(q_1 as isize))
                                            [i_5 as usize] = (((1i32) << 15i32) / j_3)
                                            as crate::src::common::common::udctcoef;
                                        if j_3
                                            > (if (0xffffi32) < ((1i32) << 25i32 - 8i32) - 1i32 {
                                                0xffffi32
                                            } else {
                                                ((1i32) << 25i32 - 8i32) - 1i32
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
                                            > (if (0xffffi32) < ((1i32) << 25i32 - 8i32) - 1i32 {
                                                0xffffi32
                                            } else {
                                                ((1i32) << 25i32 - 8i32) - 1i32
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
                            if (*h).param.analyse.transform_8x8 {
                                let mut i_list_2 = 0i32;
                                while i_list_2 < num_8x8_lists {
                                    let mut i_6 = 0i32;
                                    while i_6 < 64i32 {
                                        (*(*h).unquant8_mf[i_list_2 as usize]
                                            .offset(q_1 as isize))
                                            [i_6 as usize] = ((1u64) << q_1 / 6i32 + 16i32 + 8i32)
                                            .wrapping_div(
                                                quant8_mf[i_list_2 as usize][(q_1 % 6i32) as usize]
                                                    [i_6 as usize]
                                                    as ::core::ffi::c_ulonglong,
                                            )
                                            as ::core::ffi::c_int;
                                        j_3 = if q_1 / 6i32 <= 0i32 {
                                            quant8_mf[i_list_2 as usize][(q_1 % 6i32) as usize]
                                                [i_6 as usize]
                                                << -(q_1 / 6i32)
                                        } else {
                                            quant8_mf[i_list_2 as usize][(q_1 % 6i32) as usize]
                                                [i_6 as usize]
                                                + ((1i32) << q_1 / 6i32 - 1i32)
                                                >> q_1 / 6i32
                                        };
                                        (*(*h).quant8_mf[i_list_2 as usize].offset(q_1 as isize))
                                            [i_6 as usize] =
                                            j_3 as crate::src::common::common::udctcoef;
                                        if j_3 == 0 {
                                            min_qp_err =
                                                if min_qp_err < q_1 { min_qp_err } else { q_1 };
                                        } else {
                                            (*(*h).quant8_bias[i_list_2 as usize]
                                                .offset(q_1 as isize))
                                                [i_6 as usize] = (if ((deadzone[i_list_2 as usize]
                                                << 10i32)
                                                + (j_3 >> 1i32))
                                                / j_3
                                                < ((1i32) << 15i32) / j_3
                                            {
                                                ((deadzone[i_list_2 as usize] << 10i32)
                                                    + (j_3 >> 1i32))
                                                    / j_3
                                            } else {
                                                ((1i32) << 15i32) / j_3
                                            })
                                                as crate::src::common::common::udctcoef;
                                            (*(*h).quant8_bias0[i_list_2 as usize]
                                                .offset(q_1 as isize))
                                                [i_6 as usize] = (((1i32) << 15i32) / j_3)
                                                as crate::src::common::common::udctcoef;
                                            if j_3
                                                > (if (0xffffi32) < ((1i32) << 25i32 - 8i32) - 1i32
                                                {
                                                    0xffffi32
                                                } else {
                                                    ((1i32) << 25i32 - 8i32) - 1i32
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
                                                > (if (0xffffi32) < ((1i32) << 25i32 - 8i32) - 1i32
                                                {
                                                    0xffffi32
                                                } else {
                                                    ((1i32) << 25i32 - 8i32) - 1i32
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
                                >())
                                .wrapping_mul(
                                    (51i32 + 6i32 * (8i32 - 8i32) + 18i32
                                        - (51i32 + 6i32 * (8i32 - 8i32)))
                                        as usize,
                                ) as crate::stdlib::int64_t,
                            )
                                as *mut [[crate::src::common::common::udctcoef; 64]; 4];
                        if !(*h).nr_offset_emergency.is_null() {
                            let mut q_2 = 0i32;
                            while q_2
                                < crate::src::common::common::QP_MAX
                                    - crate::src::common::common::QP_MAX_SPEC
                            {
                                let mut cat = 0i32;
                                while cat
                                    < 3i32
                                        + (crate::src::common::base::CHROMA_444
                                            as ::core::ffi::c_int
                                            == crate::src::common::base::CHROMA_444
                                                as ::core::ffi::c_int)
                                            as ::core::ffi::c_int
                                {
                                    let mut dct8x8 = cat & 1i32;
                                    if !(!(*h).param.analyse.transform_8x8 && dct8x8 != 0) {
                                        let mut i_7 = 0i32;
                                        let mut size_1 = if dct8x8 != 0 { 64i32 } else { 16i32 };
                                        let mut nr_offset = &raw mut *(&raw mut *(*h)
                                            .nr_offset_emergency
                                            .offset(q_2 as isize)
                                            as *mut [crate::src::common::common::udctcoef; 64])
                                            .offset(cat as isize)
                                            as *mut crate::src::common::common::udctcoef;
                                        let mut dc_threshold = (crate::src::common::common::QP_MAX
                                            - crate::src::common::common::QP_MAX_SPEC)
                                            * 2i32
                                            / 3i32;
                                        let mut luma_threshold = (crate::src::common::common::QP_MAX
                                            - crate::src::common::common::QP_MAX_SPEC)
                                            * 2i32
                                            / 3i32;
                                        while i_7 < size_1 {
                                            let mut max = ((1i32)
                                                << 7i32 + crate::internal::BIT_DEPTH)
                                                - 1i32;
                                            if q_2
                                                == crate::src::common::common::QP_MAX
                                                    - crate::src::common::common::QP_MAX_SPEC
                                                    - 1i32
                                            {
                                                *nr_offset.offset(i_7 as isize) =
                                                    max as crate::src::common::common::udctcoef;
                                            } else {
                                                let mut thresh = if i_7 == 0i32 {
                                                    dc_threshold
                                                } else if cat >= 2i32 {
                                                    let mut chroma_threshold = 0i32;
                                                    chroma_threshold
                                                } else {
                                                    luma_threshold
                                                };
                                                if q_2 < thresh {
                                                    *nr_offset.offset(i_7 as isize) = 0u16;
                                                } else {
                                                    let mut pos =
                                                        (q_2 - thresh + 1i32)
                                                            as ::core::ffi::c_double
                                                            / (crate::src::common::common::QP_MAX - crate::src::common::common::QP_MAX_SPEC - thresh)
                                                                as ::core::ffi::c_double;
                                                    let mut start_1 = (if dct8x8 != 0 {
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
                                                    let mut bias =
     (crate::stdlib::pow(
                                                        2f64,
                                                        pos * (crate::src::common::common::QP_MAX - crate::src::common::common::QP_MAX_SPEC)
                                                            as ::core::ffi::c_double
                                                            / 10.0,
                                                    )
                                                        * 0.003
                                                        - 0.003)
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
                            if !(*h).mb.lossless {
                                while *(*h).chroma_qp_table.offset(
                                    (if (*h).param.rc.i_qp_min < 51i32 + 6i32 * (8i32 - 8i32) {
                                        (*h).param.rc.i_qp_min
                                    } else {
                                        51i32 + 6i32 * (8i32 - 8i32)
                                    }) as isize,
                                ) as ::core::ffi::c_int
                                    <= max_chroma_qp_err
                                {
                                    (*h).param.rc.i_qp_min += 1;
                                }
                                if min_qp_err <= (*h).param.rc.i_qp_max {
                                    (*h).param.rc.i_qp_max = min_qp_err - 1i32;
                                }
                                if max_qp_err >= (*h).param.rc.i_qp_min {
                                    (*h).param.rc.i_qp_min = max_qp_err + 1i32;
                                }
                                if !(*h).param.cabac
                                    && (*(&raw mut (*h).sps
                                        as *mut crate::src::common::set::x264_sps_t))
                                        .i_profile_idc
                                        < crate::src::common::base::PROFILE_HIGH
                                            as ::core::ffi::c_int
                                {
                                    while *(*h).chroma_qp_table.offset(
                                        (if (*h).param.rc.i_qp_max < 51i32 + 6i32 * (8i32 - 8i32) {
                                            (*h).param.rc.i_qp_max
                                        } else {
                                            51i32 + 6i32 * (8i32 - 8i32)
                                        }) as isize,
                                    )
                                        as ::core::ffi::c_int
                                        <= 12i32
                                        || (*h).param.rc.i_qp_max <= 12i32
                                    {
                                        (*h).param.rc.i_qp_max += 1;
                                    }
                                }
                                if (*h).param.rc.i_qp_min > (*h).param.rc.i_qp_max {
                                    log::error!(
                                        "Impossible QP constraints for CQM (min={}, max={})",
                                        (*h).param.rc.i_qp_min,
                                        (*h).param.rc.i_qp_max
                                    );
                                    return -(1i32);
                                }
                            }
                            return 0i32;
                        }
                    }
                }
            }
            _ => {}
        }
        x264_8_cqm_delete(h);
        return -(1i32);
    }
}
pub unsafe extern "C" fn x264_8_cqm_delete(mut h: *mut crate::src::common::common::x264_t) {
    unsafe {
        let mut i = 0i32;
        let mut i_0 = 0i32;
        while i < 4i32 {
            let mut j = 0i32;
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
            j = 0i32;
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
        while i_0
            < (if crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
                == crate::src::common::base::CHROMA_444 as ::core::ffi::c_int
            {
                4i32
            } else {
                2i32
            })
        {
            let mut j_0 = 0i32;
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
            j_0 = 0i32;
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
    _h: *mut crate::src::common::common::x264_t,
    mut buf: *const ::core::ffi::c_char,
    mut name: *const ::core::ffi::c_char,
    mut cqm: *mut crate::stdlib::uint8_t,
    mut jvt: *const crate::stdlib::uint8_t,
    mut length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    unsafe {
        let mut i = 0i32;
        let mut p = crate::stdlib::strstr(buf, name);
        if p.is_null() {
            crate::stdlib::memset(
                cqm as *mut ::core::ffi::c_void,
                16i32,
                length as crate::__stddef_size_t_h::size_t,
            );
            return 0i32;
        }
        p = p.offset(crate::stdlib::strlen(name) as isize);
        if *p as ::core::ffi::c_int == 'U' as i32 || *p as ::core::ffi::c_int == 'V' as i32 {
            p = p.offset(1);
        }
        let mut nextvar = crate::stdlib::strstr(p, b"INT\0".as_ptr() as *const ::core::ffi::c_char);
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
            let mut coef = -(1i32);
            crate::stdlib::sscanf(
                p,
                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut coef,
            );
            if i == 0i32 && coef == 0i32 {
                crate::stdlib::memcpy(
                    cqm as *mut ::core::ffi::c_void,
                    jvt as *const ::core::ffi::c_void,
                    length as crate::__stddef_size_t_h::size_t,
                );
                return 0i32;
            }
            if coef < 1i32 || coef > 255i32 {
                log::error!(
                    "bad coefficient in list '{}'",
                    std::ffi::CStr::from_ptr(name).to_string_lossy()
                );
                return -(1i32);
            }
            *cqm.offset(i as isize) = coef as crate::stdlib::uint8_t;
            i += 1;
        }
        if !nextvar.is_null() && p > nextvar || i != length {
            log::error!(
                "not enough coefficients in list '{}'",
                std::ffi::CStr::from_ptr(name).to_string_lossy()
            );
            return -(1i32);
        }
        return 0i32;
    }
}
pub unsafe extern "C" fn x264_8_cqm_parse_file(
    mut h: *mut crate::src::common::common::x264_t,
    mut filename: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    unsafe {
        let mut b_error = 0i32;
        (*h).param.i_cqm_preset = crate::x264_h::X264_CQM_CUSTOM;
        let mut buf = crate::src::common::base::x264_slurp_file(filename);
        if buf.is_null() {
            log::error!(
                "can't open file '{}'",
                std::ffi::CStr::from_ptr(filename).to_string_lossy()
            );
            return -(1i32);
        }
        loop {
            let mut p = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
            16i32,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTER4X4_LUMA\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*h).param.cqm_4py as *mut crate::stdlib::uint8_t,
            &raw const crate::src::common::tables::x264_cqm_jvt4p as *const crate::stdlib::uint8_t,
            16i32,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTRA4X4_CHROMA\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*h).param.cqm_4ic as *mut crate::stdlib::uint8_t,
            &raw const crate::src::common::tables::x264_cqm_jvt4i as *const crate::stdlib::uint8_t,
            16i32,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTER4X4_CHROMA\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*h).param.cqm_4pc as *mut crate::stdlib::uint8_t,
            &raw const crate::src::common::tables::x264_cqm_jvt4p as *const crate::stdlib::uint8_t,
            16i32,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTRA8X8_LUMA\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*h).param.cqm_8iy as *mut crate::stdlib::uint8_t,
            &raw const crate::src::common::tables::x264_cqm_jvt8i as *const crate::stdlib::uint8_t,
            64i32,
        );
        b_error |= cqm_parse_jmlist(
            h,
            buf,
            b"INTER8X8_LUMA\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*h).param.cqm_8py as *mut crate::stdlib::uint8_t,
            &raw const crate::src::common::tables::x264_cqm_jvt8p as *const crate::stdlib::uint8_t,
            64i32,
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
                64i32,
            );
            b_error |= cqm_parse_jmlist(
                h,
                buf,
                b"INTER8X8_CHROMA\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut (*h).param.cqm_8pc as *mut crate::stdlib::uint8_t,
                &raw const crate::src::common::tables::x264_cqm_jvt8p
                    as *const crate::stdlib::uint8_t,
                64i32,
            );
        }
        crate::src::common::base::x264_free(buf as *mut ::core::ffi::c_void);
        return b_error;
    }
}
